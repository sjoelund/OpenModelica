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

use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFClockKind;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use crate::NFTyping::TypingError;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub mod EvalTarget {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct EvalTarget {
        pub info: SourceInfo,
        pub context: i32,
        pub extra: Option<Arc<EvalTargetData>>,
    }

    pub type EVAL_TARGET = EvalTarget;

    pub fn new(mut info: SourceInfo, mut context: i32, mut extra: Option<Arc<EvalTargetData>>) -> Arc<EvalTarget> {
        let mut target: Arc<EvalTarget> = Arc::new(EvalTarget { info: info.clone(), context: context.clone(), extra: extra.clone() });
        target
    }

    pub fn hasInfo(mut target: Arc<EvalTarget>) -> bool {
        let mut res: bool = !(stringEmpty(target.info.fileName.clone()));
        res
    }

    pub fn getInfo(mut target: Arc<EvalTarget>) -> SourceInfo {
        let mut info: SourceInfo = target.info.clone();
        info
    }

}

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<EvalTarget::EvalTarget>
// Expr: Constructor { name: 'NFCeval.EvalTarget.EVAL_TARGET', args: [Var { name: 'Absyn.dummyInfo', segments: [CrefSegment { name: 'Absyn', subscripts: [] }, CrefSegment { name: 'dummyInfo', subscripts: [] }], ty: AliasTo('SourceInfo') }, Var { name: 'NFInstContext.NO_CONTEXT', segments: [CrefSegment { name: 'NFInstContext', subscripts: [] }, CrefSegment { name: 'NO_CONTEXT', subscripts: [] }], ty: I32 }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFCeval.EvalTarget'), field_names: ['info', 'context', 'extra'] }
pub fn noTarget() -> Arc<EvalTarget::EvalTarget> { todo!("non-Sync, non-const-emittable constant noTarget — extend codegen") }

#[derive(Clone, Debug, PartialEq)]
pub struct EvalTargetData {
    pub component: Arc<InstNode::InstNode>,
    pub index: i32,
    pub exp: Arc<Expression::NFExpression>,
}

pub type DIMENSION_DATA = EvalTargetData;


pub fn tryEvalExpResizable(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint((literal!("NFCeval.tryEvalExpResizable")).clone());
    match '__try0: {
        exp = unwrap_break_err!(evalExp(exp.clone(), target.clone()), '__try0);
        ErrorExt::delCheckpoint((literal!("NFCeval.tryEvalExpResizable")).clone());
        Ok::<_, anyhow::Error>((exp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            exp = __try0_o0;
        }
        Err(_) => {
            exp = tryEvalExpPartial(exp.clone(), target.clone());
            if Expression::contains(exp.clone(), Arc::new(Expression::isResizableCref))? {
                ErrorExt::rollBack((literal!("NFCeval.tryEvalExpResizable")).clone());
            } else {
                ErrorExt::delCheckpoint((literal!("NFCeval.tryEvalExpResizable")).clone());
                bail!("fail");
            }
        }
    }
    Ok(exp)
}

pub fn tryEvalExp(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint((literal!("NFCeval.tryEvalExp")).clone());
    match '__try0: {
        exp = unwrap_break_err!(evalExp(exp.clone(), target.clone()), '__try0);
        Ok::<_, anyhow::Error>((exp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            exp = __try0_o0;
        }
        Err(_) => {
            panic!("try/else: outputs not set in else branch");
        }
    }
    ErrorExt::rollBack((literal!("NFCeval.tryEvalExp")).clone());
    exp
}

pub fn evalExp(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            evalCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), exp.clone(), target.clone(), true, true)?
        },
        Deref @ Expression::TYPENAME { .. } => {
            evalTypename(var_field!((*exp).ty, Expression::NFExpression::TYPENAME).clone(), exp.clone(), target.clone())?
        },
        Deref @ Expression::ARRAY { .. } => {
            if (var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone()) {exp.clone()} else {Expression::makeArrayCheckLiteral(var_field!((*exp).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) })))}
        },
        Deref @ Expression::RANGE { .. } => {
            evalRange(exp.clone(), target.clone())?
        },
        Deref @ Expression::TUPLE { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::TUPLE; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = evalExp(e.clone(), target.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp.clone()
        },
        Deref @ Expression::RECORD { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, Expression::NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = evalExp(e.clone(), target.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp.clone()
        },
        Deref @ Expression::CALL { .. } => {
            evalCall(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), target.clone())?
        },
        Deref @ Expression::SIZE { .. } => {
            evalSize(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone(), var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone(), target.clone())?
        },
        Deref @ Expression::BINARY { .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = evalExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), target.clone())?;
            exp2 = evalExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), target.clone())?;
            evalBinaryOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), exp2.clone(), target.clone())?
        },
        Deref @ Expression::UNARY { .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = evalExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), target.clone())?;
            evalUnaryOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::UNARY).clone())?
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = evalExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), target.clone())?;
            if Expression::isSplitSubscriptedExp(exp1.clone()) {
                exp2 = evalExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), target.clone())?;
            } else {
                exp2 = var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone();
            }
            evalLogicBinaryOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::LBINARY).clone(), exp2.clone(), target.clone())?
        },
        Deref @ Expression::LUNARY { .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = evalExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), target.clone())?;
            evalLogicUnaryOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::LUNARY).clone())?
        },
        Deref @ Expression::RELATION { .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = evalExp(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), target.clone())?;
            exp2 = evalExp(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), target.clone())?;
            evalRelationOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::RELATION).clone(), exp2.clone())?
        },
        Deref @ Expression::IF { .. } => {
            evalIfExp(exp.clone(), target.clone())?
        },
        Deref @ Expression::CAST { .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = evalExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), target.clone())?;
            evalCast(exp1.clone(), var_field!((*exp).ty, Expression::NFExpression::CAST).clone())?
        },
        Deref @ Expression::BOX { .. } => {
            evalExp(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), target.clone())?
        },
        Deref @ Expression::UNBOX { .. } => {
            evalExp(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), target.clone())?
        },
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            evalSubscriptedExp(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), target.clone())?
        },
        Deref @ Expression::TUPLE_ELEMENT { .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = evalExp(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone(), target.clone())?;
            Expression::tupleElement(exp1.clone(), var_field!((*exp).ty, Expression::NFExpression::TUPLE_ELEMENT).clone(), var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone())?
        },
        Deref @ Expression::RECORD_ELEMENT { .. } => {
            evalRecordElement(exp.clone(), target.clone())?
        },
        Deref @ Expression::MUTABLE { .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = evalExp(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone()), target.clone())?;
            exp1.clone()
        },
        Deref @ Expression::INSTANCE_NAME { .. } => {
            evalGetInstanceName(var_field!((*exp).scope, Expression::NFExpression::INSTANCE_NAME).clone())?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn tryEvalExpPartial(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint((literal!("NFCeval.tryEvalExpPartial")).clone());
    match '__try0: {
        (exp, _) = unwrap_break_err!(evalExpPartial(exp.clone(), target.clone(), true), '__try0);
        Ok::<_, anyhow::Error>((exp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            exp = __try0_o0;
        }
        Err(_) => {
            panic!("try/else: outputs not set in else branch");
        }
    }
    ErrorExt::rollBack((literal!("NFCeval.tryEvalExpPartial")).clone());
    exp
}

pub fn evalExpPartialDefault(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    (exp, _) = evalExpPartial(exp.clone(), noTarget().clone(), true)?;
    Ok(exp)
}

pub fn evalExpPartial(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>, mut evaluated: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outEvaluated: bool = false;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eval1: bool = false;
    let mut eval2: bool = false;
    (e, outEvaluated) = Expression::mapFoldShallow(exp.clone(), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0, __pe_a2| evalExpPartial(__pe_a0, __pe_b1.clone(), __pe_a2) }), true)?;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::CREF { .. } => {
            if ComponentRef::isIterator(var_field!((*e).cref, Expression::NFExpression::CREF).clone()) {
                outExp = e.clone();
                outEvaluated = false;
            } else {
                outExp = evalCref(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), e.clone(), target.clone(), false, true)?;
                outEvaluated = Expression::isLiteral(outExp.clone());
            }
            outExp.clone()
        },
        Deref @ Expression::MUTABLE { .. } => {
            outEvaluated = false;
            e.clone()
        },
        _ => if (outEvaluated.clone()) {evalExp(e.clone(), target.clone())?} else {e.clone()},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEvaluated = evaluated.clone() && outEvaluated.clone();
    Ok((outExp, outEvaluated))
}

pub fn evalCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut defaultExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>, mut evalSubscripts: bool, mut liftExp: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut evaled: bool = false;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    exp = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { node: c @ Deref @ InstNode::COMPONENT_NODE { .. }, .. } if (!(ComponentRef::isIterator(cref.clone())) && ComponentRef::nodeVariability(cref.clone())? < Variability::NON_STRUCTURAL_PARAMETER.clone()) => evalComponentBinding(c.clone(), cref.clone(), defaultExp.clone(), target.clone(), evalSubscripts.clone(), liftExp.clone())?,
        _ => defaultExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalComponentBinding(mut node: Arc<InstNode::InstNode>, mut cref: Arc<ComponentRef::NFComponentRef>, mut defaultExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>, mut evalSubscripts: bool, mut liftExp: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp_context: i32 = 0;
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut evaluated: bool = false;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let mut start_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut cref_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut exp_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dim_diff: i32 = 0;
    let mut errors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    exp_context = InstContext::nodeContext(node.clone(), target.context.clone());
    Typing::typeComponentBinding(node.clone(), exp_context.clone(), false)?;
    comp = InstNode::component(node.clone())?;
    binding = Component::getBinding(comp.clone());
    if Binding::isUnbound(binding.clone()) {
        binding = makeComponentBinding(comp.clone(), node.clone(), Expression::toCref(defaultExp.clone())?, target.clone())?;
        if Binding::isUnbound(binding.clone()) {
            start_exp = evalComponentStartBinding(node.clone(), comp.clone(), cref.clone(), target.clone(), evalSubscripts.clone())?;
            if isSome(start_exp.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(start_exp.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
                return Ok(exp);
            }
        }
    }
    (exp, evaluated) = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { .. } => {
            exp = (match Mutable::access(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone()) {
        Binding::EvalState::NOT_EVALUATED => {
            Mutable::update(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(), Binding::EvalState::EVALUATING.clone());
            ErrorExt::setCheckpoint((literal!("NFCeval.evalComponentBinding")).clone());
            match '__try0: {
                exp = unwrap_break_err!(evalExp(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), target.clone()), '__try0);
                ErrorExt::delCheckpoint((literal!("NFCeval.evalComponentBinding")).clone());
                Ok::<_, anyhow::Error>((exp.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp = __try0_o0;
                }
                Err(_) => {
                    Mutable::update(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(), Binding::EvalState::NOT_EVALUATED.clone());
                    errors = ErrorExt::popCheckPoint((literal!("NFCeval.evalComponentBinding")).clone());
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), var_field!((*binding).info, Binding::NFBinding::TYPED_BINDING).clone())?;
                    ErrorExt::pushMessages(errors.clone());
                    bail!("fail");
                }
            }
            assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = exp.clone());
            comp = Component::setBinding(binding.clone(), comp.clone())?;
            InstNode::updateComponent(comp.clone(), node.clone())?;
            Mutable::update(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(), Binding::EvalState::EVALUATED.clone());
            exp.clone()
        },
        Binding::EvalState::EVALUATED => var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(),
        _ => {
            Error::addSourceMessage(Error::CIRCULAR_PARAM.clone(), list![(InstNode::name(node.clone())?).clone(), (Prefixes::variabilityString(Component::variability(comp.clone()))?).clone()], InstNode::info(node.clone())?)?;
            bail!("fail")
        },
    });
            (exp.clone(), true)
        },
        Deref @ Binding::CEVAL_BINDING { .. } => (var_field!((*binding).bindingExp, Binding::NFBinding::CEVAL_BINDING).clone(), true),
        Deref @ Binding::UNBOUND => {
            printUnboundError(comp.clone(), target.clone(), defaultExp.clone())?;
            (defaultExp.clone(), false)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalComponentBinding")); __mm_s.push_str(&*literal!(" failed on untyped binding")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if evaluated.clone() {
        exp = subscriptBinding(exp.clone(), cref.clone(), evalSubscripts.clone())?;
    }
    if liftExp.clone() && !(Expression::contains(exp.clone(), Arc::new(fnptr!(Expression::isSplitSubscriptedExp, Arc<Expression::NFExpression>)))?) {
        exp_ty = Expression::typeOf(exp.clone());
        cref_ty = Expression::typeOf(defaultExp.clone());
        dim_diff = Type::dimensionDiff(cref_ty.clone(), exp_ty.clone());
        if dim_diff.clone() > 0 {
            (exp, _) = Expression::liftArrayList(List::firstN(Type::arrayDims(cref_ty.clone()), dim_diff.clone())?, exp.clone())?;
        }
    }
    Ok(exp)
}

pub fn subscriptBinding(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut evalSubscripts: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    subs = ComponentRef::getSubscripts(cref.clone());
    if evalSubscripts.clone() {
        subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Subscript::eval(s.clone(), noTarget().clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    subs = List::trimToLength(subs.clone(), Expression::dimensionCount(exp.clone())?)?;
    exp = Expression::applySubscripts(subs.clone(), exp.clone(), false)?;
    (exp, _) = subscriptBinding2(exp.clone(), cref.clone(), evalSubscripts.clone(), None)?;
    Ok(exp)
}

pub fn subscriptBinding2(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut evalSubscripts: bool, mut subMap: Option<Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>>) -> Result<(Arc<Expression::NFExpression>, Option<Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>>)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut subMap: Option<Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>> = subMap;
    let mut sub_map: Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut cref_parts: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (exp, subMap) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { subscripts: subs, .. } => {
            let mut subs = (*subs).clone();
            if isSome(subMap.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(subMap.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                sub_map = __pa0.clone();
            } else {
                cref_parts = ComponentRef::toListReverse(cref.clone(), isFlatCref(cref.clone()), metamodelica::nil());
                sub_map = UnorderedMap::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>), fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>), Util::nextPrime((cref_parts.clone().len() as i32)));
                for mut cr in &*cref_parts.clone() {
                    let mut cr = cr.clone();
                    UnorderedMap::addUnique(ComponentRef::node(cr.clone())?, ComponentRef::getSubscripts(cr.clone()), sub_map.clone())?;
                }
                subMap = Some(sub_map.clone());
            }
            subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = subscriptBinding3(s.clone(), sub_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if evalSubscripts.clone() {
                subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Subscript::eval(s.clone(), noTarget().clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            }
            (e, subMap) = subscriptBinding2(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), cref.clone(), evalSubscripts.clone(), subMap.clone())?;
            e = Expression::applySubscripts(subs.clone(), e.clone(), false)?;
            (e.clone(), subMap.clone())
        },
        Deref @ Expression::ARRAY { literal: true, .. } => (exp.clone(), subMap.clone()),
        _ => Expression::mapFoldShallow(exp.clone(), Arc::new({ let __pe_b1 = cref.clone(); let __pe_b2 = evalSubscripts.clone(); move |__pe_a0, __pe_a3| subscriptBinding2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), subMap.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, subMap))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isFlatCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> bool {
    let mut flat: bool = false;
    flat = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::SCOPE, .. } if (Type::isArray(var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone())) => !(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone().is_empty()),
        Deref @ ComponentRef::CREF { .. } => isFlatCref(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    flat
}

pub fn subscriptBinding3(mut subscript: Arc<Subscript::NFSubscript>, mut subMap: Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>) -> Result<Arc<Subscript::NFSubscript>> {
    let mut outSubscript: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut osubs: Option<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>> = None;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    outSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            osubs = UnorderedMap::get(var_field!((*subscript).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), subMap.clone());
            if isSome(osubs.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(osubs.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                subs = __pa0.clone();
                if var_field!((*subscript).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone() > (subs.clone().len() as i32) {
                    outSubscript = Arc::new(crate::NFSubscript::WHOLE);
                } else {
                    outSubscript = (subs.clone()).get(var_field!((*subscript).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone())?;
                }
            } else {
                outSubscript = subscript.clone();
            }
            outSubscript.clone()
        },
        _ => subscript.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

pub fn evalComponentStartBinding(mut node: Arc<InstNode::InstNode>, mut comp: Arc<Component::NFComponent>, mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<EvalTarget::EvalTarget>, mut evalSubscripts: bool) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut outExp: Option<Arc<Expression::NFExpression>> = None;
    let mut var: Variability = Variability::CONSTANT;
    let mut start_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut start_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut pcount: i32 = 0;
    var = Component::variability(comp.clone());
    if var.clone() != Variability::PARAMETER.clone() && var.clone() != Variability::STRUCTURAL_PARAMETER.clone() || !(Component::isFixed(comp.clone())?) {
        return Ok(outExp);
    }
    if let Ok(__iflet0) = Class::lookupElement((literal!("start")).clone(), InstNode::getClass(node.clone())?) {
        start_node = __iflet0.0;
    } else {
        return Ok(outExp);
    }
    start_comp = InstNode::component(start_node.clone())?;
    if !(Component::isTypeAttribute(start_comp.clone())) {
        return Ok(outExp);
    }
    binding = Component::getBinding(start_comp.clone());
    outExp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { .. } => {
            exp = evalExp(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), target.clone())?;
            if !(referenceEq(&exp.clone(),&var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone())) {
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = exp.clone());
                start_comp = Component::setBinding(binding.clone(), start_comp.clone())?;
                InstNode::updateComponent(start_comp.clone(), start_node.clone())?;
            }
            Some(exp.clone())
        },
        _ => outExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn makeComponentBinding(mut component: Arc<Component::NFComponent>, mut node: Arc<InstNode::InstNode>, mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut rec_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    binding = 'mc: {
        let __mc_input = component.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    exp = makeRecordFieldBindingFromParent(cref.clone(), target.clone())?;
                    Ok(if (Expression::isEmpty(exp.clone())) {Binding::EMPTY_BINDING().clone()} else {Arc::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Component::COMPONENT { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_node, .. }, .. }, .. } => {
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    let mut binding: Arc<Binding::NFBinding> = binding.clone();
                    exp = makeRecordBindingExp(var_field!((*component).classInst, Component::NFComponent::COMPONENT).clone(), rec_node.clone(), var_field!((*component).ty, Component::NFComponent::COMPONENT).clone(), cref.clone(), target.clone())?;
                    binding = Arc::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() });
                    if !(ComponentRef::hasSubscripts(cref.clone())) {
                        InstNode::updateComponent(Component::setBinding(binding.clone(), component.clone())?, node.clone())?;
                    }
                    Ok(binding.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Component::COMPONENT { ty: Deref @ Type::ARRAY { elementType: ty @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_node, .. }, .. }, .. }, .. } => {
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    let mut binding: Arc<Binding::NFBinding> = binding.clone();
                    exp = Expression::mapCrefScalars(Expression::fromCref(cref.clone(), false)?, Arc::new({ let __pe_b0 = var_field!((*component).classInst, Component::NFComponent::COMPONENT).clone(); let __pe_b1 = rec_node.clone(); let __pe_b2 = ty.clone(); let __pe_b4 = target.clone(); move |__pe_a3| makeRecordBindingExp(__pe_b0.clone(), __pe_b1.clone(), __pe_b2.clone(), __pe_a3, __pe_b4.clone()) }))?;
                    binding = Arc::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() });
                    if !(ComponentRef::hasSubscripts(cref.clone())) {
                        InstNode::updateComponent(Component::setBinding(binding.clone(), component.clone())?, node.clone())?;
                    }
                    Ok(binding.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Binding::EMPTY_BINDING().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(binding)
}

pub fn makeRecordFieldBindingFromParent(mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut parent_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exp_context: i32 = 0;
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    parent_cr = ComponentRef::rest(cref.clone())?;
    parent = ComponentRef::node(parent_cr.clone())?;
    exp_context = InstContext::nodeContext(parent.clone(), target.context.clone());
    comp = InstNode::component(parent.clone())?;
    binding = Component::getBinding(comp.clone());
    subs = ComponentRef::getSubscripts(parent_cr.clone());
    if Binding::hasExp(binding.clone()) {
        if !(Binding::isTyped(binding.clone())) {
            binding = Typing::typeBinding(binding.clone(), InstContext::set(exp_context.clone(), InstContext::BINDING.clone()))?;
            comp = Component::setBinding(binding.clone(), comp.clone())?;
            InstNode::updateComponent(comp.clone(), parent.clone())?;
        }
        exp = Binding::getExp(binding.clone())?;
        exp = Expression::applySubscripts(subs.clone(), exp.clone(), false)?;
        exp = Expression::recordElement((ComponentRef::firstName(cref.clone(), false)?).clone(), exp.clone())?;
        exp = evalExp(exp.clone(), target.clone())?;
        exp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = ComponentRef::nodesIncludingSplitSubs(cref.clone(), metamodelica::nil())?; move |__pe_a0| Expression::expandNonListedSplitIndices(__pe_a0, __pe_b1.clone()) }))?;
    } else {
        exp = makeRecordFieldBindingFromParent(parent_cr.clone(), target.clone())?;
        exp = Expression::applySubscripts(subs.clone(), exp.clone(), false)?;
        exp = Expression::recordElement((ComponentRef::firstName(cref.clone(), false)?).clone(), exp.clone())?;
    }
    Ok(exp)
}

pub fn makeRecordBindingExp(mut typeNode: Arc<InstNode::InstNode>, mut recordNode: Arc<InstNode::InstNode>, mut recordType: Arc<Type::NFType>, mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut c: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    tree = Class::classTree(InstNode::getClass(typeNode.clone())?)?;
    comps = ClassTree::getComponents(tree.clone())?;
    args = metamodelica::nil();
    ErrorExt::setCheckpoint((literal!("NFCeval.makeRecordBindingExp")).clone());
    let __range0 = (1..=(comps.clone().borrow().len() as i32)).rev();
    for mut i in __range0 {
        c = comps.borrow()[(i.clone()-1) as usize].clone();
        ty = InstNode::getType(c.clone())?;
        cr = Arc::new(ComponentRef::NFComponentRef::CREF { node: c.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: cref.clone() });
        arg = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: cr.clone() });
        if Component::variability(InstNode::component(c.clone())?) <= Variability::PARAMETER.clone() {
            if '__try1: {
                arg = unwrap_break_err!(evalExp(arg.clone(), target.clone()), '__try1);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
        args = cons(arg.clone(), args.clone());
    }
    ErrorExt::rollBack((literal!("NFCeval.makeRecordBindingExp")).clone());
    exp = Expression::makeRecord(InstNode::fullPath(recordNode.clone(), false), recordType.clone(), args.clone());
    Ok(exp)
}

pub fn evalTypename(mut ty: Arc<Type::NFType>, mut originExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = if (InstContext::inIterationRange(target.context.clone())) {ExpandExp::expandTypename(ty.clone())?} else {originExp.clone()};
    Ok(exp)
}

pub fn evalRange(mut rangeExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut start_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stop_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut step_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut max_prop_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut max_prop_count: i32 = 0;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(rangeExp.clone()) {
        Deref @ Expression::RANGE { stop: __pa0, step: __pa1, start: __pa2, ty: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stop_exp = __pa0.clone();
    step_exp = __pa1.clone();
    start_exp = __pa2.clone();
    ty = __pa3.clone();
    start_exp = evalExp(start_exp.clone(), target.clone())?;
    step_exp = Util::applyOption(step_exp.clone(), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }));
    stop_exp = evalExp(stop_exp.clone(), target.clone())?;
    if InstContext::inIterationRange(target.context.clone()) {
        ty = TypeCheck::getRangeType(start_exp.clone(), step_exp.clone(), stop_exp.clone(), Type::arrayElementType(ty.clone()), EvalTarget::getInfo(target.clone()))?;
        result = Arc::new(Expression::NFExpression::RANGE { ty: ty.clone(), start: start_exp.clone(), step: step_exp.clone(), stop: stop_exp.clone() });
    } else {
        result = Arc::new(Expression::NFExpression::RANGE { ty: ty.clone(), start: start_exp.clone(), step: step_exp.clone(), stop: stop_exp.clone() });
        result = Expression::mapSplitExpressions(result.clone(), Arc::new(evalRangeExp))?;
    }
    Ok(result)
}

pub fn evalRangeExp(mut rangeExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut start: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut step: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stop: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut opt_step: Option<Arc<Expression::NFExpression>> = None;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut literals: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut istep: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(SimplifyExp::simplify(Expression::map(rangeExp.clone(), Arc::new(Expression::replaceResizableParameter))?, false)?) {
        Deref @ Expression::RANGE { stop: __pa0, step: __pa1, start: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stop = __pa0.clone();
    opt_step = __pa1.clone();
    start = __pa2.clone();
    if isSome(opt_step.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(opt_step.clone()) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        step = __pa3.clone();
        (ty, expl) = (::match_deref::match_deref! { match &((start.clone(), step.clone(), stop.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { value: istep }, Deref @ Expression::INTEGER { .. }) => {
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (({let __s=var_field!((*start).value, Expression::NFExpression::INTEGER).clone(); let __e=var_field!((*stop).value, Expression::NFExpression::INTEGER).clone(); let __step=istep.clone(); if __step>0 {__s..=__e} else {__e..=__s}}).step_by((if istep.clone()>0 {istep.clone()} else {-(istep.clone())}) as usize)).into_iter() {
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (Arc::new(crate::NFType::INTEGER), expl.clone())
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            expl = evalRangeReal(var_field!((*start).value, Expression::NFExpression::REAL).clone(), var_field!((*step).value, Expression::NFExpression::REAL).clone(), var_field!((*stop).value, Expression::NFExpression::REAL).clone());
            (Arc::new(crate::NFType::REAL), expl.clone())
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalRangeExp")).clone(), list![start.clone(), step.clone(), stop.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        (ty, expl) = (::match_deref::match_deref! { match &((start.clone(), stop.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (var_field!((*start).value, Expression::NFExpression::INTEGER).clone()..=var_field!((*stop).value, Expression::NFExpression::INTEGER).clone()).into_iter() {
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (Arc::new(crate::NFType::INTEGER), expl.clone())
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            expl = evalRangeReal(var_field!((*start).value, Expression::NFExpression::REAL).clone(), metamodelica::OrderedFloat(1.0_f64), var_field!((*stop).value, Expression::NFExpression::REAL).clone());
            (Arc::new(crate::NFType::REAL), expl.clone())
        },
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => {
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut b in (({let __bs = var_field!((*start).value, Expression::NFExpression::BOOLEAN).clone(); let __be = var_field!((*stop).value, Expression::NFExpression::BOOLEAN).clone(); if !__bs && !__be { vec![false] } else if !__bs && __be { vec![false, true] } else if __bs && __be { vec![true] } else { Vec::<bool>::new() }})).into_iter() {
            let __x = Arc::new(Expression::NFExpression::BOOLEAN { value: b.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (Arc::new(crate::NFType::BOOLEAN), expl.clone())
        },
        (Deref @ Expression::ENUM_LITERAL { ty: ty @ Deref @ Type::ENUMERATION { .. }, .. }, Deref @ Expression::ENUM_LITERAL { .. }) => {
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (var_field!((*start).index, Expression::NFExpression::ENUM_LITERAL).clone()..=var_field!((*stop).index, Expression::NFExpression::ENUM_LITERAL).clone()).into_iter() {
            let __x = Arc::new(Expression::NFExpression::ENUM_LITERAL { ty: ty.clone(), name: ((var_field!((**ty).literals, Type::NFType::ENUMERATION).clone()).get(i.clone())?).clone(), index: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (ty.clone(), expl.clone())
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalRangeExp")).clone(), list![start.clone(), stop.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    exp = Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![Dimension::fromInteger((expl.clone().len() as i32), Prefixes::Variability::CONSTANT.clone())] }), metamodelica::arrayFromVec(expl.clone().into_iter().cloned().collect()), true);
    Ok(exp)
}

pub fn evalRangeReal(mut start: metamodelica::Real, mut step: metamodelica::Real, mut stop: metamodelica::Real) -> Arc<metamodelica::List<Arc<Expression::NFExpression>>> {
    let mut result: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut steps: i32 = 0;
    steps = Util::realRangeSize(start.clone(), step.clone(), stop.clone());
    if steps.clone() == 0 {
        result = metamodelica::nil();
    } else if steps.clone() == 1 {
        result = list![Arc::new(Expression::NFExpression::REAL { value: start.clone() })];
    } else {
        result = list![Arc::new(Expression::NFExpression::REAL { value: stop.clone() })];
        for mut i in (1..=steps.clone() - 2).rev() {
            result = cons(Arc::new(Expression::NFExpression::REAL { value: start.clone() + metamodelica::OrderedFloat((i.clone()) as f64) * step.clone() }), result.clone());
        }
        result = cons(Arc::new(Expression::NFExpression::REAL { value: start.clone() }), result.clone());
    }
    result
}

pub fn printFailedEvalError(mut name: ArcStr, mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<()> {
    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" failed to evaluate ‘")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("‘")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
    Ok(())
}

pub fn evalBinaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalBinaryExp(__pe_a0, __pe_b1.clone()) }))?;
    Ok(exp)
}

pub fn evalBinaryExp(mut binaryExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(binaryExp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e2 = __pa0.clone();
    op = __pa1.clone();
    e1 = __pa2.clone();
    result = evalBinaryOp_dispatch(e1.clone(), op.clone(), e2.clone(), target.clone())?;
    Ok(result)
}

pub fn evalBinaryOp_dispatch(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (match op.op.clone() {
        Operator::Op::ADD => evalBinaryAdd(exp1.clone(), exp2.clone())?,
        Operator::Op::SUB => evalBinarySub(exp1.clone(), exp2.clone())?,
        Operator::Op::MUL => evalBinaryMul(exp1.clone(), exp2.clone())?,
        Operator::Op::DIV => evalBinaryDiv(exp1.clone(), exp2.clone(), target.clone())?,
        Operator::Op::POW => evalBinaryPow(exp1.clone(), exp2.clone(), target.clone())?,
        Operator::Op::ADD_EW => evalBinaryAdd(exp1.clone(), exp2.clone())?,
        Operator::Op::SUB_EW => evalBinarySub(exp1.clone(), exp2.clone())?,
        Operator::Op::MUL_EW => evalBinaryMul(exp1.clone(), exp2.clone())?,
        Operator::Op::ADD_SCALAR_ARRAY => evalBinaryScalarArray(exp1.clone(), exp2.clone(), Arc::new(evalBinaryAdd)),
        Operator::Op::ADD_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(exp1.clone(), exp2.clone(), Arc::new(evalBinaryAdd)),
        Operator::Op::SUB_SCALAR_ARRAY { .. } => evalBinaryScalarArray(exp1.clone(), exp2.clone(), Arc::new(evalBinarySub)),
        Operator::Op::SUB_ARRAY_SCALAR => evalBinaryArrayScalar(exp1.clone(), exp2.clone(), Arc::new(evalBinarySub)),
        Operator::Op::MUL_SCALAR_ARRAY => evalBinaryScalarArray(exp1.clone(), exp2.clone(), Arc::new(evalBinaryMul)),
        Operator::Op::MUL_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(exp1.clone(), exp2.clone(), Arc::new(evalBinaryMul)),
        Operator::Op::MUL_VECTOR_MATRIX => evalBinaryMulVectorMatrix(exp1.clone(), exp2.clone())?,
        Operator::Op::MUL_MATRIX_VECTOR => evalBinaryMulMatrixVector(exp1.clone(), exp2.clone())?,
        Operator::Op::SCALAR_PRODUCT => evalBinaryScalarProduct(exp1.clone(), exp2.clone())?,
        Operator::Op::MATRIX_PRODUCT => evalBinaryMatrixProduct(exp1.clone(), exp2.clone())?,
        Operator::Op::DIV_SCALAR_ARRAY { .. } => evalBinaryScalarArray(exp1.clone(), exp2.clone(), Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, __pe_a1, __pe_b2.clone()) })),
        Operator::Op::DIV_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(exp1.clone(), exp2.clone(), Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, __pe_a1, __pe_b2.clone()) })),
        Operator::Op::POW_SCALAR_ARRAY { .. } => evalBinaryScalarArray(exp1.clone(), exp2.clone(), Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, __pe_b2.clone()) })),
        Operator::Op::POW_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(exp1.clone(), exp2.clone(), Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, __pe_b2.clone()) })),
        Operator::Op::POW_MATRIX => evalBinaryPowMatrix(exp1.clone(), exp2.clone())?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalBinaryOp_dispatch")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(exp)
}

pub fn evalBinaryAdd(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() + metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) + var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() + var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() + var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => Arc::new(Expression::NFExpression::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).value, Expression::NFExpression::STRING).clone()); __mm_s.push_str(&*var_field!((*exp2).value, Expression::NFExpression::STRING).clone()); ArcStr::from(__mm_s) }).clone() }),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => Arc::new(Expression::NFExpression::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).value, Expression::NFExpression::STRING).clone()); __mm_s.push_str(&*var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()); ArcStr::from(__mm_s) }).clone() }),
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => Arc::new(Expression::NFExpression::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()); __mm_s.push_str(&*var_field!((*exp2).value, Expression::NFExpression::STRING).clone()); ArcStr::from(__mm_s) }).clone() }),
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => Arc::new(Expression::NFExpression::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()); __mm_s.push_str(&*var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()); ArcStr::from(__mm_s) }).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if ((var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32) == (var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new(evalBinaryAdd))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinaryAdd(__pe_a0, __pe_b1.clone()) })), var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinaryAdd(__pe_b0.clone(), __pe_a1) })), var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => exp2.clone(),
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeAdd(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryAdd")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBinarySub(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() - metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) - var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() - var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() - var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if ((var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32) == (var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new(evalBinarySub))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinarySub(__pe_a0, __pe_b1.clone()) })), var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinarySub(__pe_b0.clone(), __pe_a1) })), var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => evalBinarySub(Expression::makeZero(Expression::typeOf(exp2.clone()))?, exp2.clone())?,
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeSub(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinarySub")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalMultaryAddSub(mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut inv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut operator_ty: Arc<Type::NFType>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::EMPTY { ty: operator_ty.clone() });
    let mut isNeutral: bool = false;
    for mut arg in &*arguments.clone() {
        let mut arg = arg.clone();
        exp = evalBinaryAdd(exp.clone(), arg.clone())?;
    }
    for mut arg in &*inv_arguments.clone() {
        let mut arg = arg.clone();
        exp = evalBinarySub(exp.clone(), arg.clone())?;
    }
    isNeutral = Expression::isEmpty(exp.clone()) || Expression::isZero(exp.clone());
    Ok((exp, isNeutral))
}

pub fn evalBinaryMul(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() * metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) * var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() * var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() * var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if ((var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32) == (var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new(evalBinaryMul))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinaryMul(__pe_a0, __pe_b1.clone()) })), var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinaryMul(__pe_b0.clone(), __pe_a1) })), var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => exp2.clone(),
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeMul(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryMul")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBinaryDiv(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (_, _) if (Expression::isZero(exp2.clone())) => {
            if EvalTarget::hasInfo(target.clone()) {
                Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![(Expression::toString(exp1.clone())?).clone(), (Expression::toString(exp2.clone())?).clone()], EvalTarget::getInfo(target.clone()))?;
                bail!("fail");
            } else {
                exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeDiv(Arc::new(crate::NFType::REAL)), exp2: exp2.clone() });
            }
            exp.clone()
        },
        (_, Deref @ Expression::INTEGER { value: 1 }) => exp1.clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() / metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) / var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (intMod(var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone(), var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) == 0) {Arc::new(Expression::NFExpression::INTEGER { value: intDiv(var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone(), var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) })} else {Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() / var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) })},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() / var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if ((var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32) == (var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, __pe_a1, __pe_b2.clone()) }))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = exp2.clone(); let __pe_b2 = target.clone(); move |__pe_a0| evalBinaryDiv(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) })), var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b0 = exp1.clone(); let __pe_b2 = target.clone(); move |__pe_a1| evalBinaryDiv(__pe_b0.clone(), __pe_a1, __pe_b2.clone()) })), var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => evalBinaryDiv(Expression::makeOne(Expression::typeOf(exp2.clone()))?, exp2.clone(), target.clone())?,
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeDiv(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryDiv")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalMultaryMulDiv(mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut inv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut operator_ty: Arc<Type::NFType>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::EMPTY { ty: operator_ty.clone() });
    let mut isNeutral: bool = false;
    for mut arg in &*arguments.clone() {
        let mut arg = arg.clone();
        exp = evalBinaryMul(exp.clone(), arg.clone())?;
    }
    for mut arg in &*inv_arguments.clone() {
        let mut arg = arg.clone();
        exp = evalBinaryDiv(exp.clone(), arg.clone(), noTarget().clone())?;
    }
    isNeutral = Expression::isEmpty(exp.clone()) || Expression::isOne(exp.clone());
    Ok((exp, isNeutral))
}

pub fn evalBinaryPow(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < metamodelica::OrderedFloat((0) as f64) && metamodelica::OrderedFloat((((var_field!((*exp2).value, Expression::NFExpression::REAL).clone()).0 as i32)) as f64) != var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) => {
            if EvalTarget::hasInfo(target.clone()) {
                Error::addSourceMessage(Error::INVALID_NEGATIVE_POW.clone(), list![(Expression::toString(exp1.clone())?).clone(), (Expression::toString(exp2.clone())?).clone()], EvalTarget::getInfo(target.clone()))?;
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makePow(Arc::new(crate::NFType::REAL)), exp2: exp2.clone() })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*exp1).value, Expression::NFExpression::REAL).clone()).powf(var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if ((var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32) == (var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, __pe_b2.clone()) }))?, true),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makePow(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryPow")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBinaryScalarArray(mut scalarExp: Arc<Expression::NFExpression>, mut arrayExp: Arc<Expression::NFExpression>, mut opFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<Expression::NFExpression> {
    pub type FuncT = fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ Expression::ARRAY { .. } => Expression::makeArray(var_field!((*arrayExp).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b0 = scalarExp.clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = opFunc.clone(); move |__pe_a1| Ok(evalBinaryScalarArray(__pe_b0.clone(), __pe_a1, __pe_b2.clone())) })), true),
        _ => opFunc(scalarExp.clone(), arrayExp.clone()).unwrap(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn evalBinaryArrayScalar(mut arrayExp: Arc<Expression::NFExpression>, mut scalarExp: Arc<Expression::NFExpression>, mut opFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<Expression::NFExpression> {
    pub type FuncT = fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ Expression::ARRAY { .. } => Expression::makeArray(var_field!((*arrayExp).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = scalarExp.clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = opFunc.clone(); move |__pe_a0| Ok(evalBinaryArrayScalar(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) })), true),
        _ => opFunc(arrayExp.clone(), scalarExp.clone()).unwrap(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn evalBinaryMulVectorMatrix(mut vectorExp: Arc<Expression::NFExpression>, mut matrixExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut m: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    exp = (::match_deref::match_deref! { match &(Expression::transposeArray(matrixExp.clone())?) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: ty, dimensions: Deref @ metamodelica::List::Cons { head: m, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: arr, .. } => {
            let mut arr = (*arr).clone();
            arr = Array::map(arr.clone(), Arc::new({ let __pe_b0 = vectorExp.clone(); move |__pe_a1| evalBinaryScalarProduct(__pe_b0.clone(), __pe_a1) }));
            Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![m.clone()] }), arr.clone(), true)
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: vectorExp.clone(), operator: Operator::makeMul(Arc::new(crate::NFType::UNKNOWN)), exp2: matrixExp.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryMulVectorMatrix")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBinaryMulMatrixVector(mut matrixExp: Arc<Expression::NFExpression>, mut vectorExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    exp = (::match_deref::match_deref! { match &(matrixExp.clone()) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: ty, dimensions: Deref @ metamodelica::List::Cons { head: n, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: arr, .. } => {
            let mut arr = (*arr).clone();
            arr = Array::map(arr.clone(), Arc::new({ let __pe_b1 = vectorExp.clone(); move |__pe_a0| evalBinaryScalarProduct(__pe_a0, __pe_b1.clone()) }));
            Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![n.clone()] }), arr.clone(), true)
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: matrixExp.clone(), operator: Operator::makeMul(Arc::new(crate::NFType::UNKNOWN)), exp2: vectorExp.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryMulMatrixVector")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBinaryScalarProduct(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: elem_ty, .. }, .. }, Deref @ Expression::ARRAY { .. }) if ((var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32) == (var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)) => {
            exp = Expression::makeZero(elem_ty.clone())?;
            let __range0 = 1..=(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32);
            for mut i in __range0 {
                exp = evalBinaryAdd(exp.clone(), evalBinaryMul(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow()[(i.clone()-1) as usize].clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow()[(i.clone()-1) as usize].clone())?)?;
            }
            exp.clone()
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeMul(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryScalarProduct")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBinaryMatrixProduct(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl1: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut expl2: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut elem_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mat_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut n: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut p: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut arr1: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut arr2: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    e2 = Expression::transposeArray(exp2.clone())?;
    exp = (::match_deref::match_deref! { match &((exp1.clone(), e2.clone())) {
        (Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: elem_ty, dimensions: Deref @ metamodelica::List::Cons { head: n, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: arr1, .. }, Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: _, dimensions: Deref @ metamodelica::List::Cons { head: p, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: arr2, .. }) => {
            mat_ty = Arc::new(Type::NFType::ARRAY { elementType: elem_ty.clone(), dimensions: list![n.clone(), p.clone()] });
            if arr2.clone().borrow().is_empty() {
                exp = Expression::makeZero(mat_ty.clone())?;
            } else {
                row_ty = Arc::new(Type::NFType::ARRAY { elementType: elem_ty.clone(), dimensions: list![p.clone()] });
                arr = metamodelica::arrayCreate((arr1.clone().borrow().len() as i32), exp1.clone());
                let __range0 = 1..=(arr1.clone().borrow().len() as i32);
                for mut i in __range0 {
                    unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), Expression::makeArray(row_ty.clone(), Array::map(arr2.clone(), Arc::new({ let __pe_b0 = arr1.clone().borrow()[(i.clone()-1) as usize].clone(); move |__pe_a1| evalBinaryScalarProduct(__pe_b0.clone(), __pe_a1) })), true)) };
                }
                exp = Expression::makeArray(mat_ty.clone(), arr.clone(), true);
            }
            exp.clone()
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeMul(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryMatrixProduct")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBinaryPowMatrix(mut matrixExp: Arc<Expression::NFExpression>, mut nExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: i32 = 0;
    exp = (::match_deref::match_deref! { match &(nExp.clone()) {
        Deref @ Expression::INTEGER { value: 0 } => {
            n = Dimension::size(listHead(Type::arrayDims(Expression::typeOf(matrixExp.clone())))?, false)?;
            Expression::makeIdentityMatrix(n.clone(), Arc::new(crate::NFType::REAL))?
        },
        Deref @ Expression::INTEGER { value: n } => evalBinaryPowMatrix2(matrixExp.clone(), n.clone())?,
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: matrixExp.clone(), operator: Operator::makePow(Arc::new(crate::NFType::UNKNOWN)), exp2: nExp.clone() });
            printFailedEvalError((literal!("NFCeval.evalBinaryPowMatrix")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBinaryPowMatrix2(mut matrix: Arc<Expression::NFExpression>, mut n: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (match n.clone() {
        1 => matrix.clone(),
        2 => evalBinaryMatrixProduct(matrix.clone(), matrix.clone())?,
        _ if (intMod(n.clone(), 2) == 0) => {
            exp = evalBinaryPowMatrix2(matrix.clone(), intDiv(n.clone(), 2))?;
            evalBinaryMatrixProduct(exp.clone(), exp.clone())?
        },
        _ => {
            exp = evalBinaryPowMatrix2(matrix.clone(), n.clone() - 1)?;
            evalBinaryMatrixProduct(matrix.clone(), exp.clone())?
        },
    });
    Ok(exp)
}

pub fn evalUnaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (match op.op.clone() {
        Operator::Op::UMINUS if (Expression::isZero(exp1.clone())) => exp1.clone(),
        Operator::Op::UMINUS => Expression::mapSplitExpressions(exp1.clone(), Arc::new(evalUnaryMinus))?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalUnaryOp")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::UNARY { operator: op.clone(), exp: exp1.clone() }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(exp)
}

pub fn evalUnaryMinus(mut exp1: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::INTEGER { .. } => Arc::new(Expression::NFExpression::INTEGER { value: -(var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) }),
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: -(var_field!((*exp1).value, Expression::NFExpression::REAL).clone()) }),
        Deref @ Expression::ARRAY { .. } => {
            assign_variant_field!(exp1 => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), Arc::new(evalUnaryMinus)));
            exp1.clone()
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::UNARY { operator: Operator::makeUMinus(Arc::new(crate::NFType::UNKNOWN)), exp: exp1.clone() });
            printFailedEvalError((literal!("NFCeval.evalUnaryMinus")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalLogicBinaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalLogicBinaryExp(__pe_a0, __pe_b1.clone()) }))?;
    Ok(exp)
}

pub fn evalLogicBinaryExp(mut binaryExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(binaryExp.clone()) {
        Deref @ Expression::LBINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e2 = __pa0.clone();
    op = __pa1.clone();
    e1 = __pa2.clone();
    result = evalLogicBinaryOp_dispatch(e1.clone(), op.clone(), e2.clone(), target.clone())?;
    Ok(result)
}

pub fn evalLogicBinaryOp_dispatch(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (match op.op.clone() {
        Operator::Op::AND => evalLogicBinaryAnd(evalExp(exp1.clone(), target.clone())?, exp2.clone(), target.clone())?,
        Operator::Op::OR => evalLogicBinaryOr(evalExp(exp1.clone(), target.clone())?, exp2.clone(), target.clone())?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalLogicBinaryOp_dispatch")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(exp)
}

pub fn evalLogicBinaryAnd(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = 'mc: {
        let __mc_input = exp1.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BOOLEAN { .. } => {
                    Ok(if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone()) {evalExp(exp2.clone(), target.clone())?} else {exp1.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::ARRAY { .. } => {
                    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
                    let __pa0 = ::match_deref::match_deref! { match &(evalExp(exp2.clone(), target.clone())?) {
                        Deref @ Expression::ARRAY { elements: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    arr = __pa0.clone();
                    arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), arr.clone(), Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalLogicBinaryAnd(__pe_a0, __pe_a1, __pe_b2.clone()) }))?;
                    Ok(Expression::makeArray(Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Arc::new(crate::NFType::BOOLEAN)), arr.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    exp = Arc::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: Operator::makeAnd(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
                    printFailedEvalError((literal!("NFCeval.evalLogicBinaryAnd")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(exp)
}

pub fn evalLogicBinaryOr(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::BOOLEAN { .. } => {
            if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone()) {exp1.clone()} else {evalExp(exp2.clone(), target.clone())?}
        },
        Deref @ Expression::ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(evalExp(exp2.clone(), target.clone())?) {
                Deref @ Expression::ARRAY { elements: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            arr = __pa0.clone();
            arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), arr.clone(), Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalLogicBinaryOr(__pe_a0, __pe_a1, __pe_b2.clone()) }))?;
            Expression::makeArray(Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Arc::new(crate::NFType::BOOLEAN)), arr.clone(), true)
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: Operator::makeOr(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone() });
            printFailedEvalError((literal!("NFCeval.evalLogicBinaryOr")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalLogicUnaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (match op.op.clone() {
        Operator::Op::NOT => Expression::mapSplitExpressions(exp1.clone(), Arc::new(evalLogicUnaryNot))?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalLogicUnaryOp")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::LUNARY { operator: op.clone(), exp: exp1.clone() }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(exp)
}

pub fn evalLogicUnaryNot(mut exp1: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::BOOLEAN { .. } => Arc::new(Expression::NFExpression::BOOLEAN { value: !(var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone()) }),
        Deref @ Expression::ARRAY { .. } => Expression::mapArrayElements(exp1.clone(), Arc::new(evalLogicUnaryNot)),
        _ => {
            exp = Arc::new(Expression::NFExpression::LUNARY { operator: Operator::makeNot(Arc::new(crate::NFType::UNKNOWN)), exp: exp1.clone() });
            printFailedEvalError((literal!("NFCeval.evalLogicUnaryNot")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalRelationOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone(), index: -1 }), Arc::new(evalRelationExp))?;
    Ok(exp)
}

pub fn evalRelationExp(mut relationExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(relationExp.clone()) {
        Deref @ Expression::RELATION { exp2: __pa0, operator: __pa1, exp1: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e2 = __pa0.clone();
    op = __pa1.clone();
    e1 = __pa2.clone();
    result = evalRelationOp_dispatch(e1.clone(), op.clone(), e2.clone())?;
    Ok(result)
}

pub fn evalRelationOp_dispatch(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut res: bool = false;
    res = (match op.op.clone() {
        Operator::Op::LESS => evalRelationLess(exp1.clone(), exp2.clone())?,
        Operator::Op::LESSEQ => evalRelationLessEq(exp1.clone(), exp2.clone())?,
        Operator::Op::GREATER => evalRelationGreater(exp1.clone(), exp2.clone())?,
        Operator::Op::GREATEREQ => evalRelationGreaterEq(exp1.clone(), exp2.clone())?,
        Operator::Op::EQUAL => evalRelationEqual(exp1.clone(), exp2.clone())?,
        Operator::Op::NEQUAL => evalRelationNotEqual(exp1.clone(), exp2.clone())?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalRelationOp_dispatch")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone(), index: -1 }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    exp = Arc::new(Expression::NFExpression::BOOLEAN { value: res.clone() });
    Ok(exp)
}

pub fn evalRelationLess(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() < var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) < 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) < 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) < 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) < 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() < var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError((literal!("NFCeval.evalRelationLess")).clone(), Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: Operator::makeLess(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone(), index: -1 }), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn evalRelationLessEq(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() <= var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() <= var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() <= var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) <= 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) <= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) <= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) <= 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() <= var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError((literal!("NFCeval.evalRelationLessEq")).clone(), Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: Operator::makeLessEq(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone(), index: -1 }), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn evalRelationGreater(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() > var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() > var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() > var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) > 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) > 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) > 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) > 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() > var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError((literal!("NFCeval.evalRelationGreater")).clone(), Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: Operator::makeGreater(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone(), index: -1 }), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn evalRelationGreaterEq(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() >= var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() >= var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() >= var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) >= 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) >= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) >= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) >= 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() >= var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError((literal!("NFCeval.evalRelationGreaterEq")).clone(), Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: Operator::makeGreaterEq(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone(), index: -1 }), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn evalRelationEqual(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() == var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() == var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() == var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) == 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) == 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) == 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) == 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() == var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError((literal!("NFCeval.evalRelationEqual")).clone(), Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: Operator::makeEqual(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone(), index: -1 }), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn evalRelationNotEqual(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() != var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() != var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() != var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) != 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) != 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) != 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) != 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() != var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError((literal!("NFCeval.evalRelationNotEqual")).clone(), Arc::new(Expression::NFExpression::RELATION { exp1: exp1.clone(), operator: Operator::makeNotEqual(Arc::new(crate::NFType::UNKNOWN)), exp2: exp2.clone(), index: -1 }), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn evalIfExp(mut ifExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut btrue: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut bfalse: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ifExp.clone()) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    cond = __pa1.clone();
    btrue = __pa2.clone();
    bfalse = __pa3.clone();
    result = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: evalExp(cond.clone(), target.clone())?, trueBranch: btrue.clone(), falseBranch: bfalse.clone() });
    result = Expression::mapSplitExpressions(result.clone(), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalIfExp2(__pe_a0, __pe_b1.clone()) }))?;
    Ok(result)
}

pub fn evalIfExp2(mut ifExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ifExp.clone()) {
        Deref @ Expression::IF { falseBranch: __pa0, trueBranch: __pa1, condition: __pa2, ty: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    fb = __pa0.clone();
    tb = __pa1.clone();
    cond = __pa2.clone();
    ty = __pa3.clone();
    result = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Expression::BOOLEAN { .. } => {
            if Type::isConditionalArray(ty.clone()) && !(Type::isMatchedBranch(var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone(), ty.clone())?) {
                (tb, fb) = Util::swap(var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone(), fb.clone(), tb.clone());
                Error::addSourceMessage(Error::ARRAY_DIMENSION_MISMATCH.clone(), list![(Expression::toString(tb.clone())?).clone(), (Type::toString(Expression::typeOf(tb.clone()))?).clone(), (Dimension::toStringList(Type::arrayDims(Expression::typeOf(fb.clone())), false)?).clone()], EvalTarget::getInfo(target.clone()))?;
                bail!("fail");
            }
            evalExp(if (var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone()) {tb.clone()} else {fb.clone()}, target.clone())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalIfExp2")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(ifExp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalCast(mut castExp: Arc<Expression::NFExpression>, mut castTy: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = Expression::typeCast(castExp.clone(), castTy.clone())?;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CAST { .. } => {
            exp = Arc::new(Expression::NFExpression::CAST { ty: castTy.clone(), exp: castExp.clone() });
            printFailedEvalError((literal!("NFCeval.evalCast")).clone(), exp.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalCall(mut call: Arc<Call::NFCall>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c: Arc<Call::NFCall> = call.clone();
    exp = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Call::TYPED_CALL { .. } => {
            assign_variant_field!(c => Call::NFCall::TYPED_CALL; arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*c).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = evalExp(arg.clone(), target.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if (Function::isBuiltin(var_field!((*c).r#fn, Call::NFCall::TYPED_CALL).clone())) {Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::CALL { call: c.clone() }), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalBuiltinCallExp(__pe_a0, __pe_b1.clone()) }))?} else {Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::CALL { call: c.clone() }), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalNormalCallExp(__pe_a0, __pe_b1.clone()) }))?}
        },
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => {
            assign_variant_field!(c => Call::NFCall::TYPED_ARRAY_CONSTRUCTOR;
                exp = evalExpPartial(var_field!((*c).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), noTarget().clone(), true)?.0,
                iters = Call::mapIteratorsExpShallow(var_field!((*c).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), Arc::new(evalExpPartialDefault))
            );
            Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::CALL { call: c.clone() }), Arc::new(evalArrayConstructor))?
        },
        Deref @ Call::TYPED_REDUCTION { .. } => {
            assign_variant_field!(c => Call::NFCall::TYPED_REDUCTION;
                exp = evalExpPartial(var_field!((*c).exp, Call::NFCall::TYPED_REDUCTION).clone(), noTarget().clone(), true)?.0,
                iters = Call::mapIteratorsExpShallow(var_field!((*c).iters, Call::NFCall::TYPED_REDUCTION).clone(), Arc::new(evalExpPartialDefault))
            );
            Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::CALL { call: c.clone() }), Arc::new(evalReduction))?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalCall")); __mm_s.push_str(&*literal!(" got untyped call")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn evalBuiltinCallExp(mut callExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: __pa0, r#fn: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    args = __pa0.clone();
    r#fn = __pa1.clone();
    result = evalBuiltinCall(r#fn.clone(), args.clone(), target.clone())?;
    Ok(result)
}

pub fn evalBuiltinCall(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fn_path: Arc<Absyn::Path> = Function::nameConsiderBuiltin(r#fn.clone())?;
    result = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(fn_path.clone())?) {
        Deref @ "abs" => evalBuiltinAbs(listHead(args.clone())?)?,
        Deref @ "acos" => evalBuiltinAcos(listHead(args.clone())?, target.clone())?,
        Deref @ "array" => evalBuiltinArray(args.clone())?,
        Deref @ "asin" => evalBuiltinAsin(listHead(args.clone())?, target.clone())?,
        Deref @ "atan2" => evalBuiltinAtan2(args.clone())?,
        Deref @ "atan" => evalBuiltinAtan(listHead(args.clone())?)?,
        Deref @ "cat" => evalBuiltinCat(listHead(args.clone())?, listRest(args.clone())?, target.clone())?,
        Deref @ "ceil" => evalBuiltinCeil(listHead(args.clone())?)?,
        Deref @ "cosh" => evalBuiltinCosh(listHead(args.clone())?)?,
        Deref @ "cos" => evalBuiltinCos(listHead(args.clone())?)?,
        Deref @ "der" => evalBuiltinDer(listHead(args.clone())?)?,
        Deref @ "diagonal" => evalBuiltinDiagonal(Expression::unbox(listHead(args.clone())?))?,
        Deref @ "div" => evalBuiltinDiv(args.clone(), target.clone())?,
        Deref @ "exp" => evalBuiltinExp(listHead(args.clone())?)?,
        Deref @ "fill" => evalBuiltinFill(args.clone())?,
        Deref @ "floor" => evalBuiltinFloor(listHead(args.clone())?)?,
        Deref @ "identity" => evalBuiltinIdentity(listHead(args.clone())?)?,
        Deref @ "integer" => evalBuiltinInteger(listHead(args.clone())?)?,
        Deref @ "Integer" => evalBuiltinIntegerEnum(listHead(args.clone())?)?,
        Deref @ "log10" => evalBuiltinLog10(listHead(args.clone())?, target.clone())?,
        Deref @ "log" => evalBuiltinLog(listHead(args.clone())?, target.clone())?,
        Deref @ "matrix" => evalBuiltinMatrix(listHead(args.clone())?)?,
        Deref @ "max" => evalBuiltinMax(args.clone(), r#fn.clone())?,
        Deref @ "min" => evalBuiltinMin(args.clone(), r#fn.clone())?,
        Deref @ "mod" => evalBuiltinMod(args.clone(), target.clone())?,
        Deref @ "noEvent" => listHead(args.clone())?,
        Deref @ "ones" => evalBuiltinOnes(args.clone())?,
        Deref @ "pre" => listHead(args.clone())?,
        Deref @ "product" => evalBuiltinProduct(listHead(args.clone())?)?,
        Deref @ "promote" => evalBuiltinPromote((args.clone()).get(1)?, (args.clone()).get(2)?)?,
        Deref @ "rem" => evalBuiltinRem(args.clone(), target.clone())?,
        Deref @ "scalar" => evalBuiltinScalar(listHead(args.clone())?)?,
        Deref @ "sign" => evalBuiltinSign(listHead(args.clone())?)?,
        Deref @ "sinh" => evalBuiltinSinh(listHead(args.clone())?)?,
        Deref @ "sin" => evalBuiltinSin(listHead(args.clone())?)?,
        Deref @ "skew" => evalBuiltinSkew(listHead(args.clone())?)?,
        Deref @ "smooth" => (args.clone()).get(2)?,
        Deref @ "sqrt" => evalBuiltinSqrt(listHead(args.clone())?)?,
        Deref @ "String" => evalBuiltinString(args.clone())?,
        Deref @ "sum" => evalBuiltinSum(listHead(args.clone())?)?,
        Deref @ "symmetric" => evalBuiltinSymmetric(listHead(args.clone())?)?,
        Deref @ "tanh" => evalBuiltinTanh(listHead(args.clone())?)?,
        Deref @ "tan" => evalBuiltinTan(listHead(args.clone())?)?,
        Deref @ "transpose" => evalBuiltinTranspose(listHead(args.clone())?)?,
        Deref @ "vector" => evalBuiltinVector(listHead(args.clone())?),
        Deref @ "zeros" => evalBuiltinZeros(args.clone())?,
        Deref @ "OpenModelica_uriToFilename" => evalUriToFilename(r#fn.clone(), listHead(args.clone())?, target.clone())?,
        Deref @ "intBitAnd" => evalIntBitAnd(args.clone())?,
        Deref @ "intBitOr" => evalIntBitOr(args.clone())?,
        Deref @ "intBitXor" => evalIntBitXor(args.clone())?,
        Deref @ "intBitLShift" => evalIntBitLShift(args.clone())?,
        Deref @ "intBitRShift" => evalIntBitRShift(args.clone())?,
        Deref @ "intMaxLit" => Arc::new(Expression::NFExpression::INTEGER { value: System::intMaxLit() }),
        Deref @ "inferredClock" => evalInferredClock(args.clone())?,
        Deref @ "rationalClock" => evalRationalClock(args.clone())?,
        Deref @ "realClock" => evalRealClock(args.clone())?,
        Deref @ "booleanClock" => evalBooleanClock(args.clone())?,
        Deref @ "solverClock" => evalSolverClock(args.clone())?,
        Deref @ "$OMC$PositiveMax" => evalPositiveMax((args.clone()).get(1)?, (args.clone()).get(2)?)?,
        Deref @ "$OMC$inStreamDiv" => listHead(args.clone())?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalBuiltinCall")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*AbsynUtil::pathString(fn_path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalNormalCallExp(mut callExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: __pa0, r#fn: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    args = __pa0.clone();
    r#fn = __pa1.clone();
    result = evalNormalCall(r#fn.clone(), args.clone(), target.clone());
    Ok(result)
}

pub fn evalNormalCall(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = EvalFunction::evaluate(r#fn.clone(), args.clone(), target.clone()).unwrap();
    result
}

pub fn evalBuiltinAbs(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::INTEGER { .. } => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*arg).value, Expression::NFExpression::INTEGER).clone().abs() }),
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: var_field!((*arg).value, Expression::NFExpression::REAL).clone().abs() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinAbs")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinAcos(mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { value: x } => {
            if x.clone() < metamodelica::OrderedFloat(-1.0_f64) || x.clone() > metamodelica::OrderedFloat(1.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", x.clone())), (literal!("acos")).clone(), (literal!("-1 <= x <= 1")).clone()], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: (x.clone()).acos() })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinAcos")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinArray(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = Expression::typeOf(listHead(args.clone())?);
    ty = Type::liftArrayLeft(ty.clone(), Dimension::fromInteger((args.clone().len() as i32), Prefixes::Variability::CONSTANT.clone()));
    result = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(args.clone().into_iter().cloned().collect()), true);
    Ok(result)
}

pub fn evalBuiltinAsin(mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { value: x } => {
            if x.clone() < metamodelica::OrderedFloat(-1.0_f64) || x.clone() > metamodelica::OrderedFloat(1.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", x.clone())), (literal!("asin")).clone(), (literal!("-1 <= x <= 1")).clone()], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: (x.clone()).asin() })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinAsin")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinAtan2(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: y }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: x }, tail: Deref @ metamodelica::List::Nil } } => Arc::new(Expression::NFExpression::REAL { value: (y.clone()).atan2(x.clone()) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinAtan2")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinAtan(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).atan() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinAtan")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinCat(mut argN: Arc<Expression::NFExpression>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: i32 = 0;
    let mut nd: i32 = 0;
    let mut sz: i32 = 0;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut es: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(argN.clone()) {
        Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    ty = Expression::typeOf(listHead(args.clone())?);
    nd = Type::dimensionCount(ty.clone());
    if n.clone() > nd.clone() || n.clone() < 1 {
        if EvalTarget::hasInfo(target.clone()) {
            Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", n.clone())), (literal!("cat")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1 <= x <= ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", nd.clone()))); ArcStr::from(__mm_s) }).clone()], EvalTarget::getInfo(target.clone()))?;
        }
        bail!("fail");
    }
    es = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (args.clone()).into_iter().cloned() {
            if !(!(Expression::isEmptyArray(e.clone()))) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    sz = (es.clone().len() as i32);
    if sz.clone() == 0 {
        result = listHead(args.clone())?;
    } else if sz.clone() == 1 {
        result = listHead(es.clone())?;
    } else {
        (es, dims) = ExpressionBasics::evalCat(n.clone(), es.clone(), Arc::new(Expression::arrayElementList), Arc::new(Expression::toString))?;
        result = Expression::arrayFromList(es.clone(), Expression::typeOf(listHead(es.clone())?), {
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = Dimension::fromInteger(d.clone(), Prefixes::Variability::CONSTANT.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
    }
    Ok(result)
}

pub fn evalBuiltinCeil(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).ceil() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinCeil")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinCosh(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).cosh() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinCosh")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinCos(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).cos() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinCos")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinDer(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = Expression::fillType(Expression::typeOf(arg.clone()), Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }))?;
    Ok(result)
}

pub fn evalBuiltinDiagonal(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut elem_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut zero: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut row: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut rows: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut n: i32 = 0;
    let mut i: i32 = 1;
    let mut e_lit: bool = false;
    let mut arg_lit: bool = true;
    let mut arr_zero: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut arr_row: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut arr_rows: metamodelica::Array<Arc<Expression::NFExpression>>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ARRAY { .. } if (var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone().borrow().is_empty()) => arg.clone(),
        Deref @ Expression::ARRAY { .. } => {
            n = (var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32);
            elem_ty = Type::unliftArray(var_field!((*arg).ty, Expression::NFExpression::ARRAY).clone())?;
            row_ty = Type::liftArrayLeft(elem_ty.clone(), Dimension::fromInteger(n.clone(), Prefixes::Variability::CONSTANT.clone()));
            zero = Expression::makeZero(elem_ty.clone())?;
            arr_zero = arrayCreate(n.clone(), zero.clone());
            arr_rows = metamodelica::arrayCreate(n.clone(), zero.clone());
            for mut i in 1..=n.clone() {
                arr_row = metamodelica::arrayFromVec(arr_zero.clone().borrow().clone());
                exp = var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone().borrow()[(i.clone()-1) as usize].clone();
                e_lit = Expression::isLiteral(exp.clone());
                arg_lit = arg_lit.clone() && e_lit.clone();
                {let _arr = arr_row.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = exp.clone(); _arr};
                exp = Expression::makeArray(row_ty.clone(), arr_row.clone(), e_lit.clone());
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr_rows.clone(), i.clone(), exp.clone()) };
            }
            Expression::makeArray(Type::liftArrayLeft(row_ty.clone(), Dimension::fromInteger(n.clone(), Prefixes::Variability::CONSTANT.clone())), arr_rows.clone(), arg_lit.clone())
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinDiagonal")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinDiv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rx: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut ry: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: ix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: iy }, tail: Deref @ metamodelica::List::Nil } } => {
            if iy.clone() == 0 {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![ArcStr::from(::std::format!("{}", ix.clone())), ArcStr::from(::std::format!("{}", iy.clone()))], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::INTEGER { value: intDiv(ix.clone(), iy.clone()) })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: rx }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: ry }, tail: Deref @ metamodelica::List::Nil } } => {
            let mut rx = (*rx).clone();
            if ry.clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![ArcStr::from(::std::format!("{}", rx.clone())), ArcStr::from(::std::format!("{}", ry.clone()))], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            rx = rx.clone() / ry.clone();
            Arc::new(Expression::NFExpression::REAL { value: if (rx.clone() < metamodelica::OrderedFloat(0.0_f64)) {(rx.clone()).ceil()} else {(rx.clone()).floor()} })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinDiv")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinExp(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).exp() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinExp")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinFill(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fill_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut dims: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        fill_exp = __pa1.clone();
        dims = __pa2.clone();
        result = unwrap_break_err!(Expression::fillArgs(fill_exp.clone(), dims.clone()), '__try0);
        Ok::<_, anyhow::Error>((dims.clone(), fill_exp.clone(), result.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            dims = __try0_o0;
            fill_exp = __try0_o1;
            result = __try0_o2;
        }
        Err(_) => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinFill")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok(result)
}

fn evalBuiltinFloor(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).floor() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinFloor")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinIdentity(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::INTEGER { .. } => Expression::makeIdentityMatrix(var_field!((*arg).value, Expression::NFExpression::INTEGER).clone(), Arc::new(crate::NFType::INTEGER))?,
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinIdentity")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinInteger(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::INTEGER { .. } => arg.clone(),
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::INTEGER { value: ((var_field!((*arg).value, Expression::NFExpression::REAL).clone()).0 as i32) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinInteger")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinIntegerEnum(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ENUM_LITERAL { .. } => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*arg).index, Expression::NFExpression::ENUM_LITERAL).clone() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinIntegerEnum")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinLog10(mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { value: x } => {
            if x.clone() <= metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", x.clone())), (literal!("log10")).clone(), (literal!("x > 0")).clone()], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: (x.clone()).log10() })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinLog10")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinLog(mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { value: x } => {
            if x.clone() <= metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", x.clone())), (literal!("log")).clone(), (literal!("x > 0")).clone()], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: (x.clone()).ln() })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinLog")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMatrix(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ARRAY { ty, .. } => {
            let mut dim_count: i32 = 0;
            let mut dim1: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            let mut dim2: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            let mut ty = (*ty).clone();
            dim_count = Type::dimensionCount(ty.clone());
            if dim_count.clone() < 2 {
                (result, _) = Expression::promote(arg.clone(), ty.clone(), 2)?;
            } else if dim_count.clone() == 2 {
                result = arg.clone();
            } else {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Type::arrayDims(ty.clone())) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                dim1 = __pa0.clone();
                dim2 = __pa1.clone();
                ty = Type::liftArrayLeft(Type::arrayElementType(ty.clone()), dim2.clone());
                arr = Array::map(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = ty.clone(); move |__pe_a0| evalBuiltinMatrix2(__pe_a0, __pe_b1.clone()) }));
                ty = Type::liftArrayLeft(ty.clone(), dim1.clone());
                result = Expression::makeArray(ty.clone(), arr.clone(), false);
            }
            result.clone()
        },
        _ => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            ty = Expression::typeOf(arg.clone());
            if Type::isScalar(ty.clone()) {
                (result, _) = Expression::promote(arg.clone(), ty.clone(), 2)?;
            } else {
                printWrongArgsError((literal!("NFCeval.evalBuiltinMatrix")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
                bail!("fail");
            }
            result.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMatrix2(mut arg: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ARRAY { .. } => Expression::makeArray(ty.clone(), Array::map(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), Arc::new(Expression::toScalar)), var_field!((*arg).literal, Expression::NFExpression::ARRAY).clone()),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinMatrix2")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMax(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut r#fn: Arc<Function::Function>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => evalBuiltinMax2(e1.clone(), e2.clone())?,
        Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } if (Expression::isArray(e1.clone())) => {
            ty = Expression::typeOf(e1.clone());
            result = Expression::fold(e1.clone(), Arc::new(evalBuiltinMax2), Arc::new(Expression::NFExpression::EMPTY { ty: ty.clone() }))?;
            if Expression::isEmpty(result.clone()) {
                result = Expression::makeMinValue(Type::arrayElementType(ty.clone()))?;
            }
            result.clone()
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinMax")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinMax2(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() < var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) {exp2.clone()} else {exp1.clone()},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) {exp2.clone()} else {exp1.clone()},
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone()) {exp2.clone()} else {exp1.clone()},
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => if (var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() < var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone()) {exp2.clone()} else {exp1.clone()},
        (Deref @ Expression::ARRAY { .. }, _) => exp2.clone(),
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinMax2")).clone(), list![exp1.clone(), exp2.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalPositiveMax(mut flow_exp: Arc<Expression::NFExpression>, mut eps: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = if (Expression::isNonPositive(flow_exp.clone())) {Expression::makeZero(Expression::typeOf(flow_exp.clone()))?} else {evalBuiltinMax2(flow_exp.clone(), eps.clone())?};
    Ok(result)
}

fn evalBuiltinMin(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut r#fn: Arc<Function::Function>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => evalBuiltinMin2(e1.clone(), e2.clone())?,
        Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } if (Expression::isArray(e1.clone())) => {
            ty = Expression::typeOf(e1.clone());
            result = Expression::fold(e1.clone(), Arc::new(evalBuiltinMin2), Arc::new(Expression::NFExpression::EMPTY { ty: ty.clone() }))?;
            if Expression::isEmpty(result.clone()) {
                result = Expression::makeMaxValue(Type::arrayElementType(ty.clone()))?;
            }
            result.clone()
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinMin")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalBuiltinMin2(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() > var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) {exp2.clone()} else {exp1.clone()},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() > var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) {exp2.clone()} else {exp1.clone()},
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() > var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone()) {exp2.clone()} else {exp1.clone()},
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => if (var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() > var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone()) {exp2.clone()} else {exp1.clone()},
        (Deref @ Expression::ARRAY { .. }, _) => exp2.clone(),
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinMin2")).clone(), list![exp1.clone(), exp2.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMod(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x = __pa0.clone();
    y = __pa1.clone();
    result = (::match_deref::match_deref! { match &((x.clone(), y.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::INTEGER).clone() == 0 {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::MODULO_BY_ZERO.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::INTEGER).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::INTEGER).clone()))], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::INTEGER { value: intMod(var_field!((*x).value, Expression::NFExpression::INTEGER).clone(), var_field!((*y).value, Expression::NFExpression::INTEGER).clone()) })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::MODULO_BY_ZERO.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::REAL).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::REAL).clone()))], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: realMod(var_field!((*x).value, Expression::NFExpression::REAL).clone(), var_field!((*y).value, Expression::NFExpression::REAL).clone()) })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinMod")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinOnes(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = evalBuiltinFill(cons(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), args.clone()))?;
    Ok(result)
}

fn evalBuiltinProduct(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        _ if (Expression::isArray(arg.clone())) => (::match_deref::match_deref! { match &(Type::arrayElementType(Expression::typeOf(arg.clone()))) {
        Deref @ Type::INTEGER => Arc::new(Expression::NFExpression::INTEGER { value: Expression::fold(arg.clone(), Arc::new(evalBuiltinProductInt), 1)? }),
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::REAL { value: Expression::fold(arg.clone(), Arc::new(evalBuiltinProductReal), metamodelica::OrderedFloat(1.0_f64))? }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinProduct")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinProduct")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinProductInt(mut exp: Arc<Expression::NFExpression>, mut result: i32) -> Result<i32> {
    let mut result: i32 = result;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => result.clone() * var_field!((*exp).value, Expression::NFExpression::INTEGER).clone(),
        Deref @ Expression::ARRAY { .. } => result.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinProductReal(mut exp: Arc<Expression::NFExpression>, mut result: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut result: metamodelica::Real = result;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { .. } => result.clone() * var_field!((*exp).value, Expression::NFExpression::REAL).clone(),
        Deref @ Expression::ARRAY { .. } => result.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinPromote(mut arg: Arc<Expression::NFExpression>, mut argN: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: i32 = 0;
    if Expression::isInteger(argN.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(argN.clone()) {
            Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        n = __pa0.clone();
        (result, _) = Expression::promote(arg.clone(), Expression::typeOf(arg.clone()), n.clone())?;
    } else {
        printWrongArgsError((literal!("NFCeval.evalBuiltinPromote")).clone(), list![arg.clone(), argN.clone()], metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok(result)
}

fn evalBuiltinRem(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x = __pa0.clone();
    y = __pa1.clone();
    result = (::match_deref::match_deref! { match &((x.clone(), y.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::INTEGER).clone() == 0 {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::REM_ARG_ZERO.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::INTEGER).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::INTEGER).clone()))], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*x).value, Expression::NFExpression::INTEGER).clone() - var_field!((*x).value, Expression::NFExpression::INTEGER).clone() / var_field!((*y).value, Expression::NFExpression::INTEGER).clone() * var_field!((*y).value, Expression::NFExpression::INTEGER).clone() })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::REM_ARG_ZERO.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::REAL).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::REAL).clone()))], EvalTarget::getInfo(target.clone()))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: var_field!((*x).value, Expression::NFExpression::REAL).clone() - var_field!((*x).value, Expression::NFExpression::REAL).clone() / var_field!((*y).value, Expression::NFExpression::REAL).clone() * var_field!((*y).value, Expression::NFExpression::REAL).clone() })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinRem")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinScalar(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = arg.clone();
    while Expression::isArray(result.clone()) {
        result = Expression::arrayScalarElement(result.clone())?;
    }
    Ok(result)
}

fn evalBuiltinSign(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::INTEGER { value: if (var_field!((*arg).value, Expression::NFExpression::REAL).clone() > metamodelica::OrderedFloat((0) as f64)) {1} else {if (var_field!((*arg).value, Expression::NFExpression::REAL).clone() < metamodelica::OrderedFloat((0) as f64)) {-1} else {0}} }),
        Deref @ Expression::INTEGER { .. } => Arc::new(Expression::NFExpression::INTEGER { value: if (var_field!((*arg).value, Expression::NFExpression::INTEGER).clone() > 0) {1} else {if (var_field!((*arg).value, Expression::NFExpression::INTEGER).clone() < 0) {-1} else {0}} }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinSign")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSinh(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).sinh() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinSinh")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSin(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).sin() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinSin")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSkew(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut zero: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut literal: bool = false;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ARRAY { literal, ty, .. } => {
            let mut ty = (*ty).clone();
            x1 = var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone().borrow()[(1-1) as usize].clone();
            x2 = var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone().borrow()[(2-1) as usize].clone();
            x3 = var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone().borrow()[(3-1) as usize].clone();
            zero = Expression::makeZero(Type::arrayElementType(ty.clone()))?;
            y1 = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![zero.clone(), Expression::negate(x3.clone()), x2.clone()].into_iter().cloned().collect()), literal.clone());
            y2 = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![x3.clone(), zero.clone(), Expression::negate(x1.clone())].into_iter().cloned().collect()), literal.clone());
            y3 = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![Expression::negate(x2.clone()), x1.clone(), zero.clone()].into_iter().cloned().collect()), literal.clone());
            ty = Type::liftArrayLeft(ty.clone(), Dimension::fromInteger(3, Prefixes::Variability::CONSTANT.clone()));
            Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![y1.clone(), y2.clone(), y3.clone()].into_iter().cloned().collect()), literal.clone())
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinSkew")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSqrt(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).sqrt() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinSqrt")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinString(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: min_len }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::BOOLEAN { value: left_justified }, tail: Deref @ metamodelica::List::Nil } } } => {
            let mut str_len: i32 = 0;
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ((::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::INTEGER { .. } => intString(var_field!((**arg).value, Expression::NFExpression::INTEGER).clone()),
        Deref @ Expression::BOOLEAN { .. } => boolString(var_field!((**arg).value, Expression::NFExpression::BOOLEAN).clone()),
        Deref @ Expression::ENUM_LITERAL { .. } => var_field!((**arg).name, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinString")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
            str_len = ((r#str.clone()).clone().len() as i32);
            if str_len.clone() < min_len.clone() {
                if left_justified.clone() {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*stringAppendList(List::fill((literal!(" ")).clone(), min_len.clone() - str_len.clone()))); ArcStr::from(__mm_s) }).clone();
                } else {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringAppendList(List::fill((literal!(" ")).clone(), min_len.clone() - str_len.clone()))); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                }
            }
            Arc::new(Expression::NFExpression::STRING { value: (r#str.clone()).clone() })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: r }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: significant_digits }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: min_len }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::BOOLEAN { value: left_justified }, tail: Deref @ metamodelica::List::Nil } } } } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut format: ArcStr = arcstr::literal!("");
            format = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*if (left_justified.clone()) {literal!("-")} else {literal!("")}); __mm_s.push_str(&*intString(min_len.clone())); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*intString(significant_digits.clone())); __mm_s.push_str(&*literal!("g")); ArcStr::from(__mm_s) }).clone();
            r#str = (System::sprintff((format.clone()).clone(), r.clone())?).clone();
            Arc::new(Expression::NFExpression::STRING { value: (r#str.clone()).clone() })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: r }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::STRING { value: format }, tail: Deref @ metamodelica::List::Nil } } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (System::sprintff(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*format.clone()); ArcStr::from(__mm_s) }).clone(), r.clone())?).clone();
            Arc::new(Expression::NFExpression::STRING { value: (r#str.clone()).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn evalBuiltinSum(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        _ if (Expression::isArray(arg.clone())) => (::match_deref::match_deref! { match &(Type::arrayElementType(Expression::typeOf(arg.clone()))) {
        Deref @ Type::INTEGER => Arc::new(Expression::NFExpression::INTEGER { value: Expression::fold(arg.clone(), Arc::new(evalBuiltinSumInt), 0)? }),
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::REAL { value: Expression::fold(arg.clone(), Arc::new(evalBuiltinSumReal), metamodelica::OrderedFloat(0.0_f64))? }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinSum")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinSum")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSumInt(mut exp: Arc<Expression::NFExpression>, mut result: i32) -> Result<i32> {
    let mut result: i32 = result;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => result.clone() + var_field!((*exp).value, Expression::NFExpression::INTEGER).clone(),
        Deref @ Expression::ARRAY { .. } => result.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSumReal(mut exp: Arc<Expression::NFExpression>, mut result: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut result: metamodelica::Real = result;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { .. } => result.clone() + var_field!((*exp).value, Expression::NFExpression::REAL).clone(),
        Deref @ Expression::ARRAY { .. } => result.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSymmetric(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mat: metamodelica::Array<metamodelica::Array<Arc<Expression::NFExpression>>>;
    let mut n: i32 = 0;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut accum: metamodelica::Array<Arc<Expression::NFExpression>>;
    ty = Expression::typeOf(arg.clone());
    if Expression::isArray(arg.clone()) && Type::isSquareMatrix(ty.clone())? {
        mat = Array::map(Expression::arrayElements(arg.clone())?, Arc::new(Expression::arrayElements));
        n = (mat.clone().borrow().len() as i32);
        row_ty = Type::unliftArray(Expression::typeOf(arg.clone()))?;
        accum = metamodelica::arrayCreate(n.clone(), arg.clone());
        for mut i in 1..=n.clone() {
            arr = metamodelica::arrayCreate(n.clone(), arg.clone());
            for mut j in 1..=n.clone() {
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), j.clone(), if (i.clone() > j.clone()) {mat.borrow()[(j.clone()-1) as usize].clone().borrow()[(i.clone()-1) as usize].clone()} else {mat.borrow()[(i.clone()-1) as usize].clone().borrow()[(j.clone()-1) as usize].clone()}) };
            }
            unsafe { metamodelica::Dangerous::arrayInitSlot(accum.clone(), i.clone(), Expression::makeArray(row_ty.clone(), arr.clone(), true)) };
        }
        result = Expression::makeArray(ty.clone(), accum.clone(), true);
    } else {
        printWrongArgsError((literal!("NFCeval.evalBuiltinSymmetric")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok(result)
}

fn evalBuiltinTanh(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).tanh() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinTanh")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinTan(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).tan() }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBuiltinTan")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinTranspose(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut dim1: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim2: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rest_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arrl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Expression::NFExpression>>>>> = metamodelica::nil();
    let mut literal: bool = false;
    ty = Expression::typeOf(arg.clone());
    if Expression::isArray(arg.clone()) && Type::dimensionCount(ty.clone()) >= 2 {
        result = Expression::transposeArray(arg.clone())?;
    } else {
        printWrongArgsError((literal!("NFCeval.evalBuiltinTranspose")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok(result)
}

fn evalBuiltinVector(mut arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    expl = Expression::arrayScalarElements(arg.clone());
    result = Expression::makeExpArray(metamodelica::arrayFromVec(expl.clone().into_iter().cloned().collect()), Type::arrayElementType(Expression::typeOf(arg.clone())), true);
    result
}

fn evalBuiltinZeros(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = evalBuiltinFill(cons(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), args.clone()))?;
    Ok(result)
}

fn evalUriToFilename(mut r#fn: Arc<Function::Function>, mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::STRING { .. } => Arc::new(Expression::NFExpression::FILENAME { filename: uriToFilename((var_field!((*arg).value, Expression::NFExpression::STRING).clone()).clone()) }),
        Deref @ Expression::FILENAME { .. } => Arc::new(Expression::NFExpression::FILENAME { filename: uriToFilename((var_field!((*arg).filename, Expression::NFExpression::FILENAME).clone()).clone()) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalUriToFilename")).clone(), list![arg.clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitAnd(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } => Arc::new(Expression::NFExpression::INTEGER { value: intBitAnd(i1.clone(), i2.clone()) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalIntBitAnd")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitOr(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } => Arc::new(Expression::NFExpression::INTEGER { value: intBitOr(i1.clone(), i2.clone()) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalIntBitOr")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitXor(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } => Arc::new(Expression::NFExpression::INTEGER { value: intBitXor(i1.clone(), i2.clone()) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalIntBitXor")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitLShift(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } => Arc::new(Expression::NFExpression::INTEGER { value: intBitLShift(i1.clone(), i2.clone()) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalIntBitLShift")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitRShift(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } => Arc::new(Expression::NFExpression::INTEGER { value: intBitRShift(i1.clone(), i2.clone()) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalIntBitRShift")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalInferredClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Nil => Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) }),
        _ => {
            printWrongArgsError((literal!("NFCeval.evalInferredClock")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalRationalClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: interval @ Deref @ Expression::INTEGER { .. }, tail: Deref @ metamodelica::List::Cons { head: resolution @ Deref @ Expression::INTEGER { .. }, tail: Deref @ metamodelica::List::Nil } } => {
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::RATIONAL_CLOCK { intervalCounter: interval.clone(), resolution: resolution.clone() }) })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalRationalClock")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalRealClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: interval @ Deref @ Expression::REAL { .. }, tail: Deref @ metamodelica::List::Nil } => {
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::REAL_CLOCK { interval: interval.clone() }) })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalRealClock")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBooleanClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: condition @ Deref @ Expression::BOOLEAN { .. }, tail: Deref @ metamodelica::List::Cons { head: interval @ Deref @ Expression::REAL { .. }, tail: Deref @ metamodelica::List::Nil } } => {
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::EVENT_CLOCK { condition: condition.clone(), startInterval: interval.clone() }) })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalBooleanClock")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalSolverClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: c @ Deref @ Expression::CLKCONST { .. }, tail: Deref @ metamodelica::List::Cons { head: solver @ Deref @ Expression::STRING { .. }, tail: Deref @ metamodelica::List::Nil } } => {
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::SOLVER_CLOCK { c: c.clone(), solverMethod: solver.clone() }) })
        },
        _ => {
            printWrongArgsError((literal!("NFCeval.evalSolverClock")).clone(), args.clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn evalGetInstanceName(mut scope: Arc<InstNode::InstNode>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = Arc::new(Expression::NFExpression::STRING { value: (AbsynUtil::pathString(InstNode::rootPath(scope.clone(), false), (literal!(".")).clone(), true, false)?).clone() });
    Ok(result)
}

fn evalArrayConstructor(mut callExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut iter_exps: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { iters: __pa0, exp: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    iters = __pa0.clone();
    exp = __pa1.clone();
    (exp, ranges, iter_exps) = Expression::createIterationRanges(exp.clone(), iters.clone())?;
    result = evalArrayConstructor2(exp.clone(), ranges.clone(), iter_exps.clone())?;
    Ok(result)
}

fn evalArrayConstructor2(mut exp: Arc<Expression::NFExpression>, mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut iterators: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ranges_rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut iter: Mutable::Mutable<Arc<Expression::NFExpression>>;
    let mut iters_rest: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>> = metamodelica::nil();
    let mut range_iter: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
    let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if ranges.clone().is_empty() {
        result = evalExp(exp.clone(), noTarget().clone())?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ranges.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        range = __pa0.clone();
        ranges_rest = __pa1.clone();
        range = evalExp(range.clone(), noTarget().clone())?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(iterators.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        iter = __pa2.clone();
        iters_rest = __pa3.clone();
        range_iter = ExpressionIterator::fromExp(range.clone(), false, false)?;
        while ExpressionIterator::hasNext(range_iter.clone())? {
            (range_iter, value) = ExpressionIterator::next(range_iter.clone())?;
            Mutable::update(iter.clone(), value.clone());
            expl = cons(evalArrayConstructor2(exp.clone(), ranges_rest.clone(), iters_rest.clone())?, expl.clone());
        }
        arr = metamodelica::arrayFromVec(expl.clone().reverse().into_iter().cloned().collect());
        ty = if (arr.clone().borrow().is_empty()) {Type::liftArrayLeftList(Expression::typeOf(exp.clone()), List::mapFlat(ranges_rest.clone(), Arc::new(fnptr!(Expression::dimensions, Arc<Expression::NFExpression>))))} else {Expression::typeOf(listHead(expl.clone())?)};
        ty = Type::liftArrayLeft(ty.clone(), Dimension::fromInteger((arr.clone().borrow().len() as i32), Prefixes::Variability::CONSTANT.clone()));
        result = Expression::makeArray(ty.clone(), arr.clone(), true);
    }
    Ok(result)
}

type ReductionFn = fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

fn evalReduction(mut callExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    fn reductionFn(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>, mut r#fn: ReductionFn) -> Arc<Expression::NFExpression> {
        let mut result: Arc<Expression::NFExpression> = r#fn(exp1.clone(), evalExp(exp2.clone(), target.clone()).unwrap()).unwrap();
        result
    }

    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function>;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut default_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut red_fn: ReductionFn;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { iters: __pa0, exp: __pa1, r#fn: __pa2, .. } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    iters = __pa0.clone();
    exp = __pa1.clone();
    r#fn = __pa2.clone();
    ty = Expression::typeOf(exp.clone());
    (red_fn, default_exp) = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::name(r#fn.clone()), (literal!(".")).clone(), true, false)?) {
        Deref @ "sum" => (evalBinaryAdd, Expression::makeZero(ty.clone())?),
        Deref @ "product" => (evalBinaryMul, Expression::makeOne(ty.clone())?),
        Deref @ "min" => (evalBuiltinMin2, Expression::makeMaxValue(ty.clone())?),
        Deref @ "max" => (evalBuiltinMax2, Expression::makeMinValue(ty.clone())?),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalReduction")); __mm_s.push_str(&*literal!(" got unknown reduction function ")); __mm_s.push_str(&*AbsynUtil::pathString(Function::name(r#fn.clone()), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result = Expression::foldReduction(exp.clone(), iters.clone(), default_exp.clone(), Arc::new({ let __pe_b1 = noTarget().clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }), Arc::new(red_fn))?;
    Ok(result)
}

fn evalSize(mut exp: Arc<Expression::NFExpression>, mut optIndex: Option<Arc<Expression::NFExpression>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index: i32 = 0;
    let mut ty_err: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut info: SourceInfo;
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    info = EvalTarget::getInfo(target.clone());
    if isSome(optIndex.clone()) {
        index_exp = evalExp(Util::getOption(optIndex.clone())?, target.clone())?;
        index = Expression::toInteger(index_exp.clone())?;
        (dim, _, ty_err) = Typing::typeExpDim(exp.clone(), index.clone(), InstContext::CLASS.clone(), info.clone())?;
        Typing::checkSizeTypingError(ty_err.clone(), exp.clone(), index.clone(), info.clone())?;
        outExp = Dimension::sizeExp(dim.clone())?;
        outExp = evalExp(outExp.clone(), target.clone())?;
    } else {
        (outExp, ty, _, _) = Typing::typeExp(exp.clone(), InstContext::CLASS.clone(), info.clone(), false)?;
        arr = Array::mapList(Type::arrayDims(ty.clone()), Arc::new(Dimension::sizeExp))?;
        Array::mapNoCopy(arr.clone(), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }));
        dim = Dimension::fromInteger((arr.clone().borrow().len() as i32), Variability::PARAMETER.clone());
        outExp = Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: list![dim.clone()] }), arr.clone(), false);
    }
    Ok(outExp)
}

fn evalSubscriptedExp(mut exp: Arc<Expression::NFExpression>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RANGE { .. } => Arc::new(Expression::NFExpression::RANGE { ty: var_field!((*exp).ty, Expression::NFExpression::RANGE).clone(), start: evalExp(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), target.clone())?, step: Util::applyOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone(), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) })), stop: evalExp(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), target.clone())? }),
        _ => evalExp(exp.clone(), target.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subscripts.clone()).into_iter().cloned() {
            let __x = Subscript::mapShallowExp(s.clone(), Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    result = Expression::applySubscripts(subs.clone(), result.clone(), false)?;
    Ok(result)
}

fn evalRecordElement(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD_ELEMENT { index: __pa0, recordExp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    index = __pa0.clone();
    e = __pa1.clone();
    e = evalExp(e.clone(), target.clone())?;
    match '__try2: {
        result = unwrap_break_err!(Expression::mapSplitExpressions(e.clone(), Arc::new({ let __pe_b0 = index.clone(); move |__pe_a1| Expression::nthRecordElement(__pe_b0.clone(), __pe_a1) })), '__try2);
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try2_o0,)) => {
            result = __try2_o0;
        }
        Err(_) => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalRecordElement")); __mm_s.push_str(&*literal!(" could not evaluate ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(result)
}

fn evalRecordElement2(mut exp: Arc<Expression::NFExpression>, mut index: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD { .. } => (var_field!((*exp).elements, Expression::NFExpression::RECORD).clone()).get(index.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn printUnboundError(mut component: Arc<Component::NFComponent>, mut target: Arc<EvalTarget::EvalTarget>, mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    let mut extra: Arc<EvalTargetData>;
    if !(EvalTarget::hasInfo(target.clone())) {
        return Ok(());
    }
    let () = (::match_deref::match_deref! { match &(target.extra.clone()) {
        Some(extra @ Deref @ EvalTargetData { .. }) => {
            Error::addSourceMessage(Error::STRUCTURAL_PARAMETER_OR_CONSTANT_WITH_NO_BINDING.clone(), list![(Expression::toString(extra.exp.clone())?).clone(), (InstNode::name(extra.component.clone())?).clone()], target.info.clone())?;
            bail!("fail")
        },
        _ if (InstContext::inCondition(target.context.clone())) => {
            Error::addSourceMessage(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone(), list![(Expression::toString(exp.clone())?).clone()], target.info.clone())?;
            bail!("fail")
        },
        _ => {
            if listMember(Component::variability(component.clone()), list![Variability::STRUCTURAL_PARAMETER.clone(), Variability::PARAMETER.clone()]) && Util::getOptionOrDefault(Component::getEvaluateAnnotation(component.clone())?, false) {
                if Component::isFixed(component.clone())? {
                    Error::addMultiSourceMessage(Error::UNBOUND_PARAMETER_EVALUATE_TRUE.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("(fixed = true)")); ArcStr::from(__mm_s) }).clone()], list![InstNode::info(ComponentRef::node(Expression::toCref(exp.clone())?)?)?, EvalTarget::getInfo(target.clone())])?;
                }
            } else {
                Error::addMultiSourceMessage(Error::UNBOUND_CONSTANT.clone(), list![(Expression::toString(exp.clone())?).clone()], list![InstNode::info(ComponentRef::node(Expression::toCref(exp.clone())?)?)?, EvalTarget::getInfo(target.clone())])?;
                bail!("fail");
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printWrongArgsError(mut evalFunc: ArcStr, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut info: SourceInfo) -> Result<()> {
    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*evalFunc.clone()); __mm_s.push_str(&*literal!(" got invalid arguments ")); __mm_s.push_str(&*List::toString(args.clone(), Arc::new(Expression::toString), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), info.clone())?;
    Ok(())
}

