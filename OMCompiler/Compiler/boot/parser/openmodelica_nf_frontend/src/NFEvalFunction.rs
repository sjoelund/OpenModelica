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

use crate::FFI;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunctionExt as EvalFunctionExt;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRecord as Record;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Autoconf;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FlowControl {
    NEXT = 1,
    CONTINUE = 2,
    BREAK = 3,
    RETURN = 4,
    ASSERTION = 5,
}
impl PartialOrd for FlowControl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for FlowControl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub type ArgumentMap = Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<Expression::NFExpression>>>;

pub const STATEMENT_CONTEXT: i32 = intBitOr(InstContext::FUNCTION, InstContext::ALGORITHM);

pub const IF_COND_CONTEXT: i32 = intBitOr(STATEMENT_CONTEXT, intBitOr(InstContext::IF, InstContext::CONDITION));

pub fn evaluate(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Function::isExternal(r#fn.clone()) {
        result = evaluateExternal(r#fn.clone(), args.clone(), target.clone())?;
    } else if Function::isPartialDerivative(r#fn.clone()) {
        bail!("fail");
    } else {
        result = evaluateNormal(r#fn.clone(), args.clone(), target.context.clone())?;
    }
    Ok(result)
}

pub fn evaluateNormal(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fn_body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut bindings: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = metamodelica::nil();
    let mut arg_map: ArgumentMap;
    let mut call_count: i32 = 0;
    let mut limit: i32 = 0;
    let mut call_counter: Pointer::Pointer<i32> = r#fn.callCounter.clone();
    let mut ctrl: FlowControl = FlowControl::NEXT;
    let mut body_context: i32 = 0;
    call_count = Pointer::access(call_counter.clone()) + 1;
    limit = Flags::getConfigInt(Flags::EVAL_RECURSION_LIMIT.clone())?;
    if call_count.clone() > limit.clone() {
        Pointer::update(call_counter.clone(), 0);
        Error::addSourceMessage(Error::EVAL_RECURSION_LIMIT_REACHED.clone(), list![ArcStr::from(::std::format!("{}", limit.clone())), (AbsynUtil::pathString(Function::name(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone()], InstNode::info(r#fn.node.clone())?)?;
        bail!("fail");
    }
    Pointer::update(call_counter.clone(), call_count.clone());
    body_context = InstContext::clearScopeFlags(context.clone());
    match '__try0: {
        fn_body = Function::getBody(r#fn.clone());
        arg_map = unwrap_break_err!(createArgumentMap(r#fn.inputs.clone(), r#fn.outputs.clone(), r#fn.locals.clone(), args.clone(), true, true), '__try0);
        fn_body = applyReplacements(arg_map.clone(), fn_body.clone());
        fn_body = optimizeBody(fn_body.clone());
        ctrl = unwrap_break_err!(evaluateStatements(fn_body.clone(), body_context.clone()), '__try0);
        if ctrl.clone() != FlowControl::ASSERTION.clone() {
            result = unwrap_break_err!(createResult(arg_map.clone(), r#fn.outputs.clone()), '__try0);
        } else {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        Ok::<_, anyhow::Error>((arg_map.clone(), ctrl.clone(), fn_body.clone(), result.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            arg_map = __try0_o0;
            ctrl = __try0_o1;
            fn_body = __try0_o2;
            result = __try0_o3;
        }
        Err(_) => {
            Pointer::update(call_counter.clone(), call_count.clone() - 1);
            bail!("fail");
        }
    }
    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(Function::name(r#fn.clone()), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" => ")); ArcStr::from(__mm_s) }).clone());
        println!("{}", (Expression::toString(result.clone())?).clone());
        println!("{}", (literal!("\nArguments:\n")).clone());
        println!("{}", (UnorderedMap::toString(arg_map.clone(), (std::sync::Arc::new(InstNode::name) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?).clone());
        println!("{}", (literal!("\n")).clone());
    }
    Pointer::update(call_counter.clone(), call_count.clone() - 1);
    Ok(result)
}

pub fn evaluateExternal(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut name: ArcStr = arcstr::literal!("");
    let mut lang: ArcStr = arcstr::literal!("");
    let mut output_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ann: Option<Arc<SCode::Annotation>> = None;
    let mut ext_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Class::getSections(InstNode::getClass(r#fn.node.clone())?)?) {
        Deref @ Sections::EXTERNAL { ann: __pa0, language: __pa1, outputRef: __pa2, args: __pa3, name: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ann = __pa0.clone();
    lang = __pa1.clone();
    output_ref = __pa2.clone();
    ext_args = __pa3.clone();
    name = __pa4.clone();
    result = 'mc: {
        let __mc_input = lang.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "builtin" => {
                    Ok(Ceval::evalBuiltinCall(r#fn.clone(), args.clone(), Ceval::noTarget().clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "FORTRAN 77" => {
                    Ok(evaluateExternal2((name.clone()).clone(), r#fn.clone(), args.clone(), ext_args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((!(InstContext::inInstanceAPI(target.context.clone())))) { bail!("guard") }
                    Ok(callExternalFunction((name.clone()).clone(), r#fn.clone(), args.clone(), ext_args.clone(), output_ref.clone(), ann.clone(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Ceval::EvalTarget::hasInfo(target.clone()) {
                        Error::addSourceMessage(Error::FAILED_TO_EVALUATE_FUNCTION.clone(), list![(AbsynUtil::pathString(r#fn.path.clone(), (literal!(".")).clone(), true, false)?).clone()], Ceval::EvalTarget::getInfo(target.clone()))?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(result)
}

pub fn evaluateRecordConstructor(mut r#fn: Arc<Function::Function>, mut ty: Arc<Type::NFType>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut evaluate: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg_map: ArgumentMap;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut out_ty: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    arg_map = createArgumentMap(r#fn.inputs.clone(), metamodelica::nil(), r#fn.locals.clone(), args.clone(), false, true)?;
    let __pa0 = ::match_deref::match_deref! { match &(r#fn.returnType.clone()) {
        Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    out_ty = __pa0.clone();
    let __range1 = ClassTree::getComponents(Class::classTree(InstNode::getClass(out_ty.clone())?)?)?.borrow().iter().cloned().collect::<Vec<_>>();
    for mut c in __range1 {
        expl = cons(UnorderedMap::getOrFail(c.clone(), arg_map.clone()), expl.clone());
    }
    result = Expression::makeRecord(Function::name(r#fn.clone()), ty.clone(), expl.clone().reverse());
    if evaluate.clone() {
        result = Ceval::evalExp(result.clone(), Ceval::noTarget().clone())?;
    }
    Ok(result)
}

fn createArgumentMap(mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut mutableParams: bool, mut buildArrayBinding: bool) -> Result<ArgumentMap> {
    let mut map: ArgumentMap;
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rest_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = args.clone();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
    map = UnorderedMap::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 1);
    for mut i in &*inputs.clone() {
        let mut i = i.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        UnorderedMap::add(i.clone(), arg.clone(), map.clone())?;
        if Expression::isFunctionPointer(arg.clone()) {
            for mut r#fn in &*Function::getCachedFuncs(i.clone())? {
                let mut r#fn = r#fn.clone();
                UnorderedMap::add(r#fn.node.clone(), arg.clone(), map.clone())?;
            }
        }
    }
    if mutableParams.clone() {
        List::fold(outputs.clone(), Arc::new({ let __pe_b2 = buildArrayBinding.clone(); move |__pe_a0, __pe_a1| addMutableArgument(__pe_a0, __pe_a1, __pe_b2.clone()) }), map.clone());
        List::fold(locals.clone(), Arc::new({ let __pe_b2 = buildArrayBinding.clone(); move |__pe_a0, __pe_a1| addMutableArgument(__pe_a0, __pe_a1, __pe_b2.clone()) }), map.clone());
    } else {
        List::fold(outputs.clone(), Arc::new({ let __pe_b2 = buildArrayBinding.clone(); move |__pe_a0, __pe_a1| addImmutableArgument(__pe_a0, __pe_a1, __pe_b2.clone()) }), map.clone());
        List::fold(locals.clone(), Arc::new({ let __pe_b2 = buildArrayBinding.clone(); move |__pe_a0, __pe_a1| addImmutableArgument(__pe_a0, __pe_a1, __pe_b2.clone()) }), map.clone());
    }
    UnorderedMap::apply(map.clone(), Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| applyBindingReplacement(__pe_a0, __pe_b1.clone()) }));
    UnorderedMap::apply(map.clone(), (std::sync::Arc::new(evaluateReplacement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>));
    Ok(map)
}

fn addMutableArgument(mut node: Arc<InstNode::InstNode>, mut map: ArgumentMap, mut buildArrayBinding: bool) -> Result<ArgumentMap> {
    let mut map: ArgumentMap = map;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = getBindingExp(node.clone(), map.clone(), true, buildArrayBinding.clone())?;
    exp = Expression::makeMutable(exp.clone());
    UnorderedMap::add(node.clone(), exp.clone(), map.clone())?;
    Ok(map)
}

fn addImmutableArgument(mut node: Arc<InstNode::InstNode>, mut map: ArgumentMap, mut buildArrayBinding: bool) -> Result<ArgumentMap> {
    let mut map: ArgumentMap = map;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = getBindingExp(node.clone(), map.clone(), false, buildArrayBinding.clone())?;
    UnorderedMap::add(node.clone(), exp.clone(), map.clone())?;
    Ok(map)
}

fn getBindingExp(mut node: Arc<InstNode::InstNode>, mut map: ArgumentMap, mut mutableParams: bool, mut buildArrayBinding: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut bindingExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    comp = InstNode::component(node.clone())?;
    binding = Component::getBinding(comp.clone());
    if Binding::isBound(binding.clone()) {
        bindingExp = Binding::getExp(binding.clone())?;
        bindingExp = Expression::map(bindingExp.clone(), (std::sync::Arc::new(fnptr!(Expression::clone, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    } else {
        bindingExp = buildBinding(node.clone(), map.clone(), mutableParams.clone(), buildArrayBinding.clone())?;
    }
    Ok(bindingExp)
}

fn buildBinding(mut node: Arc<InstNode::InstNode>, mut map: ArgumentMap, mut mutableParams: bool, mut buildArrayBinding: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = InstNode::getType(node.clone())?;
    ty = Type::mapDims(ty.clone(), Arc::new({ let __pe_b0 = map.clone(); move |__pe_a1| applyReplacementsDim(__pe_b0.clone(), __pe_a1) }));
    result = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { .. } if (buildArrayBinding.clone()) => if (Type::hasKnownSize(ty.clone())) {Expression::fillType(ty.clone(), Arc::new(Expression::NFExpression::EMPTY { ty: Type::arrayElementType(ty.clone()) }))?} else {Expression::makeEmptyArray(ty.clone())},
        Deref @ Type::COMPLEX { .. } => buildRecordBinding(node.clone(), map.clone(), mutableParams.clone())?,
        _ => Arc::new(Expression::NFExpression::EMPTY { ty: ty.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn applyReplacementsDim(mut map: ArgumentMap, mut dim: Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = dim;
    dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp = Expression::map(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone(), Arc::new({ let __pe_b0 = map.clone(); move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1) }))?;
            exp = Ceval::evalExp(exp.clone(), Ceval::noTarget().clone())?;
            Dimension::fromExp(exp.clone(), Variability::CONSTANT.clone())?
        },
        _ => {
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn buildRecordBinding(mut recordNode: Arc<InstNode::InstNode>, mut map: ArgumentMap, mut mutableParams: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cls_node: Arc<InstNode::InstNode> = InstNode::classScope(recordNode.clone());
    let mut cls: Arc<Class::NFClass> = InstNode::getClass(cls_node.clone())?;
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut bindings: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut local_map: ArgumentMap;
    result = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: comps, .. }, .. } => {
            bindings = metamodelica::nil();
            local_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 1);
            let __range0 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                exp = getBindingExp(comp.clone(), map.clone(), mutableParams.clone(), true)?;
                if mutableParams.clone() {
                    exp = Expression::makeMutable(exp.clone());
                }
                UnorderedMap::add(comp.clone(), exp.clone(), local_map.clone())?;
            }
            UnorderedMap::apply(local_map.clone(), Arc::new({ let __pe_b1 = local_map.clone(); move |__pe_a0| applyBindingReplacement(__pe_a0, __pe_b1.clone()) }));
            bindings = UnorderedMap::valueList(local_map.clone());
            Expression::makeRecord(InstNode::fullPath(cls_node.clone(), false), var_field!((*cls).ty, Class::NFClass::INSTANCED_CLASS).clone(), bindings.clone())
        },
        Deref @ Class::TYPED_DERIVED { .. } => buildRecordBinding(var_field!((*cls).baseClass, Class::NFClass::TYPED_DERIVED).clone(), map.clone(), mutableParams.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn applyBindingReplacement(mut exp: Arc<Expression::NFExpression>, mut map: ArgumentMap) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = Expression::map(exp.clone(), Arc::new({ let __pe_b0 = map.clone(); move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1) }))?;
    Ok(outExp)
}

fn applyReplacements(mut map: ArgumentMap, mut fnBody: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Arc<metamodelica::List<Arc<Statement::NFStatement>>> {
    let mut fnBody: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = fnBody;
    fnBody = Statement::mapExpList(fnBody.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b0 = map.clone(); move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1) }); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }));
    fnBody
}

fn applyReplacements2(mut map: ArgumentMap, mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => applyReplacementCref(map.clone(), var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), exp.clone())?,
        Deref @ Expression::CALL { .. } => applyReplacementCall(map.clone(), var_field!((*exp).call, Expression::NFExpression::CALL).clone(), exp.clone())?,
        Deref @ Expression::UNBOX { .. } => var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn applyReplacementCref(mut map: ArgumentMap, mut cref: Arc<ComponentRef::NFComponentRef>, mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref_parts: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut repl_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    cref_parts = ComponentRef::toListReverse(cref.clone(), true, metamodelica::nil());
    if cref_parts.clone().is_empty() {
        outExp = exp.clone();
    } else {
        parent = ComponentRef::node(listHead(cref_parts.clone())?)?;
        repl_exp = UnorderedMap::get(parent.clone(), map.clone());
        if isSome(repl_exp.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(repl_exp.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            outExp = __pa0.clone();
        } else {
            outExp = exp.clone();
            return Ok(outExp);
        }
        outExp = Expression::applySubscripts(ComponentRef::getSubscripts(listHead(cref_parts.clone())?), outExp.clone(), false)?;
        cref_parts = listRest(cref_parts.clone())?;
        if !(cref_parts.clone().is_empty()) {
            if '__try1: {
                for mut cr in &*cref_parts.clone() {
                    let mut cr = cr.clone();
                    node = unwrap_break_err!(ComponentRef::node(cr.clone()), '__try1);
                    outExp = Expression::makeImmutable(outExp.clone());
                    outExp = unwrap_break_err!(Expression::recordElement((InstNode::name(node.clone())?).clone(), outExp.clone()), '__try1);
                    outExp = unwrap_break_err!(Expression::applySubscripts(ComponentRef::getSubscripts(cr.clone()), outExp.clone(), false), '__try1);
                }
                Ok::<(), anyhow::Error>(())
            }.is_err() {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.applyReplacementCref")); __mm_s.push_str(&*literal!(" could not find replacement for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
        }
        outExp = Expression::map(outExp.clone(), Arc::new({ let __pe_b0 = map.clone(); move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1) }))?;
    }
    Ok(outExp)
}

fn applyReplacementCall(mut map: ArgumentMap, mut call: Arc<Call::NFCall>, mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut repl_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut repl_oexp: Option<Arc<Expression::NFExpression>> = None;
    let mut repl_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    outExp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_CALL { .. } => {
            repl_oexp = UnorderedMap::get(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).node.clone(), map.clone());
            if isSome(repl_oexp.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(repl_oexp.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                repl_exp = __pa0.clone();
                outExp = (::match_deref::match_deref! { match &(repl_exp.clone()) {
        Deref @ Expression::CREF { ty: Deref @ Type::FUNCTION { r#fn, .. }, .. } => {
            assign_variant_field!(call => Call::NFCall::TYPED_CALL;
                arguments = mergeFunctionApplicationArgs(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone(), var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone(), r#fn.clone(), metamodelica::nil(), metamodelica::nil())?,
                r#fn = r#fn.clone()
            );
            Arc::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. } => {
            r#fn = listHead(Function::getCachedFuncs(ComponentRef::node(var_field!((*repl_exp).r#fn, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?)?)?;
            assign_variant_field!(call => Call::NFCall::TYPED_CALL;
                arguments = mergeFunctionApplicationArgs(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone(), var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone(), r#fn.clone(), var_field!((*repl_exp).args, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), var_field!((*repl_exp).argNames, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?,
                r#fn = r#fn.clone()
            );
            Arc::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
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

fn evaluateReplacement(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::MUTABLE { .. } => {
            Expression::applyMutable(exp.clone(), (std::sync::Arc::new(evaluateReplacement2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn evaluateReplacement2(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::MUTABLE { .. } => {
            Expression::applyMutable(exp.clone(), (std::sync::Arc::new(evaluateReplacement2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            exp.clone()
        },
        Deref @ Expression::RECORD { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, Expression::NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = evaluateReplacement2(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp.clone()
        },
        _ => if (Expression::contains(exp.clone(), (std::sync::Arc::new(fnptr!(Expression::isEmpty, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) {exp.clone()} else {Ceval::evalExp(exp.clone(), Ceval::noTarget().clone())?},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn mergeFunctionApplicationArgs(mut oldFn: Arc<Function::Function>, mut oldArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut newFn: Arc<Function::Function>, mut newArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut argNames: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arg_map: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Expression::NFExpression>>>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    arg_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    for mut s in &*newFn.slots.clone() {
        let mut s = s.clone();
        if isSome(s.default.clone()) {
            UnorderedMap::add((InstNode::name(s.node.clone())?).clone(), Expression::unbox(Util::getOption(s.default.clone())?), arg_map.clone())?;
        }
    }
    args = oldArgs.clone();
    for mut i in &*oldFn.inputs.clone() {
        let mut i = i.clone();
        UnorderedMap::add((InstNode::name(i.clone())?).clone(), Expression::unbox(listHead(args.clone())?), arg_map.clone())?;
        args = listRest(args.clone())?;
    }
    args = newArgs.clone();
    for mut n in &*argNames.clone() {
        let mut n = n.clone();
        UnorderedMap::add((n.clone()).clone(), Expression::unbox(listHead(args.clone())?), arg_map.clone())?;
        args = listRest(args.clone())?;
    }
    for mut i in &*newFn.inputs.clone() {
        let mut i = i.clone();
        outArgs = cons(UnorderedMap::getOrFail((InstNode::name(i.clone())?).clone(), arg_map.clone()), outArgs.clone());
    }
    outArgs = outArgs.clone().reverse();
    Ok(outArgs)
}

fn optimizeBody(mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Arc<metamodelica::List<Arc<Statement::NFStatement>>> {
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = body;
    body = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (body.clone()).into_iter().cloned() {
            let __x = Statement::map(s.clone(), (std::sync::Arc::new(optimizeStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> + 'static>));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    body
}

fn optimizeStatement(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::FOR { .. } => {
            let mut iter_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            iter_exp = Expression::makeMutable(Arc::new(Expression::NFExpression::EMPTY { ty: InstNode::getType(var_field!((*stmt).iterator, Statement::NFStatement::FOR).clone())? }));
            assign_variant_field!(stmt => Statement::NFStatement::FOR;
                body = Statement::replaceIteratorList(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), var_field!((*stmt).iterator, Statement::NFStatement::FOR).clone(), iter_exp.clone()),
                iterator = Arc::new(InstNode::InstNode::ITERATOR_NODE { exp: iter_exp.clone() })
            );
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

fn createResult(mut map: ArgumentMap, mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut types: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if (outputs.clone().len() as i32) == 1 {
        exp = Ceval::evalExp(UnorderedMap::getOrFail(listHead(outputs.clone())?, map.clone()), Ceval::noTarget().clone())?;
        assertAssignedOutput(listHead(outputs.clone())?, exp.clone())?;
    } else {
        expl = metamodelica::nil();
        types = metamodelica::nil();
        for mut o in &*outputs.clone() {
            let mut o = o.clone();
            e = Ceval::evalExp(UnorderedMap::getOrFail(o.clone(), map.clone()), Ceval::noTarget().clone())?;
            assertAssignedOutput(o.clone(), e.clone())?;
            expl = cons(e.clone(), expl.clone());
        }
        expl = expl.clone().reverse();
        types = {
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = Expression::typeOf(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        exp = Arc::new(Expression::NFExpression::TUPLE { ty: Arc::new(Type::NFType::TUPLE { types: types.clone(), names: None }), elements: expl.clone() });
    }
    Ok(exp)
}

fn assertAssignedOutput(mut outputNode: Arc<InstNode::InstNode>, mut value: Arc<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(value.clone()) {
        Deref @ Expression::EMPTY { .. } => {
            Error::addSourceMessageAsError(Error::UNASSIGNED_FUNCTION_OUTPUT.clone(), list![(InstNode::name(outputNode.clone())?).clone()], InstNode::info(outputNode.clone())?)?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn evaluateStatements(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    for mut s in &*stmts.clone() {
        let mut s = s.clone();
        ctrl = evaluateStatement(s.clone(), context.clone())?;
        if ctrl.clone() != FlowControl::NEXT.clone() {
            if ctrl.clone() == FlowControl::CONTINUE.clone() {
                ctrl = FlowControl::NEXT.clone();
            }
            break;
        }
    }
    Ok(ctrl)
}

fn evaluateStatement(mut stmt: Arc<Statement::NFStatement>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT;
    ctrl = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => evaluateAssignment(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), var_field!((*stmt).source, Statement::NFStatement::ASSIGNMENT).clone(), context.clone())?,
        Deref @ Statement::FOR { .. } => evaluateFor(var_field!((*stmt).iterator, Statement::NFStatement::FOR).clone(), var_field!((*stmt).range, Statement::NFStatement::FOR).clone(), var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), var_field!((*stmt).source, Statement::NFStatement::FOR).clone(), context.clone())?,
        Deref @ Statement::IF { .. } => evaluateIf(var_field!((*stmt).branches, Statement::NFStatement::IF).clone(), var_field!((*stmt).source, Statement::NFStatement::IF).clone(), context.clone())?,
        Deref @ Statement::ASSERT { .. } => evaluateAssert(var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), stmt.clone(), var_field!((*stmt).source, Statement::NFStatement::ASSERT).clone(), context.clone())?,
        Deref @ Statement::NORETCALL { .. } => evaluateNoRetCall(var_field!((*stmt).exp, Statement::NFStatement::NORETCALL).clone(), var_field!((*stmt).source, Statement::NFStatement::NORETCALL).clone(), context.clone())?,
        Deref @ Statement::WHILE { .. } => evaluateWhile(var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), var_field!((*stmt).body, Statement::NFStatement::WHILE).clone(), var_field!((*stmt).source, Statement::NFStatement::WHILE).clone(), context.clone())?,
        Deref @ Statement::RETURN { .. } => FlowControl::RETURN.clone(),
        Deref @ Statement::BREAK { .. } => FlowControl::BREAK.clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.evaluateStatement")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*anyString(stmt.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ctrl)
}

fn evaluateAssignment(mut lhsExp: Arc<Expression::NFExpression>, mut rhsExp: Arc<Expression::NFExpression>, mut source: Arc<DAE::ElementSource>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    assignVariable(lhsExp.clone(), Ceval::evalExp(rhsExp.clone(), evalTargetFromSource(source.clone(), STATEMENT_CONTEXT.clone(), context.clone()))?)?;
    Ok(ctrl)
}

pub fn assignVariable(mut variable: Arc<Expression::NFExpression>, mut value: Arc<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((variable.clone(), value.clone())) {
        (Deref @ Expression::MUTABLE { exp: var_ptr }, _) => {
            Mutable::update(var_ptr.clone(), assignExp(Mutable::access(var_ptr.clone()), value.clone())?);
            ()
        },
        (Deref @ Expression::TUPLE { .. }, Deref @ Expression::TUPLE { elements: vals, .. }) => {
            let mut var: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut vals = (*vals).clone();
            for mut var in &*var_field!((*variable).elements, Expression::NFExpression::TUPLE).clone() {
                let mut var = var.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(vals.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                val = __pa0.clone();
                vals = __pa1.clone();
                assignVariable(var.clone(), val.clone())?;
            }
            ()
        },
        (Deref @ Expression::SUBSCRIPTED_EXP { exp: Deref @ Expression::MUTABLE { exp: var_ptr }, .. }, _) => {
            assignSubscriptedVariable(var_ptr.clone(), var_field!((*variable).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), value.clone())?;
            ()
        },
        (Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. }, _) => {
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.assignVariable")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*Expression::toString(variable.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*Expression::toString(value.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn assignSubscriptedVariable(mut variable: Mutable::Mutable<Arc<Expression::NFExpression>>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut value: Arc<Expression::NFExpression>) -> Result<()> {
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subscripts.clone()).into_iter().cloned() {
            let __x = Subscript::eval(s.clone(), Ceval::noTarget().clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Mutable::update(variable.clone(), assignArrayElement(Mutable::access(variable.clone()), subs.clone(), value.clone())?);
    Ok(())
}

fn assignArrayElement(mut arrayExp: Arc<Expression::NFExpression>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut value: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut sub: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rest_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut idx: i32 = 0;
    let mut subs: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut vals: metamodelica::Array<Arc<Expression::NFExpression>>;
    result = (::match_deref::match_deref! { match &((arrayExp.clone(), subscripts.clone())) {
        (Deref @ Expression::ARRAY { .. }, Deref @ metamodelica::List::Cons { head: Deref @ Subscript::INDEX { index: sub }, tail: rest_subs }) if (Expression::isScalarLiteral(sub.clone())) => {
            idx = Expression::toInteger(sub.clone())?;
            if rest_subs.clone().is_empty() {
                {let _arr = var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = value.clone(); _arr};
            } else {
                {let _arr = var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(); let _val = assignArrayElement(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone().borrow()[(idx.clone()-1) as usize].clone(), rest_subs.clone(), value.clone())?; _arr.borrow_mut()[(idx.clone()-1) as usize] = _val; _arr};
            }
            arrayExp.clone()
        },
        (Deref @ Expression::ARRAY { .. }, Deref @ metamodelica::List::Cons { head: Deref @ Subscript::SLICE { slice: sub }, tail: rest_subs }) => {
            let mut sub = (*sub).clone();
            subs = Expression::arrayElements(sub.clone())?;
            vals = Expression::arrayElements(value.clone())?;
            if (subs.clone().borrow().len() as i32) > (vals.clone().borrow().len() as i32) {
                bail!("fail");
            }
            if rest_subs.clone().is_empty() {
                let __range0 = 1..=(subs.clone().borrow().len() as i32);
                for mut i in __range0 {
                    sub = subs.clone().borrow()[(i.clone()-1) as usize].clone();
                    val = vals.clone().borrow()[(i.clone()-1) as usize].clone();
                    idx = Expression::toInteger(sub.clone())?;
                    {let _arr = var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = val.clone(); _arr};
                }
            } else {
                let __range1 = 1..=(subs.clone().borrow().len() as i32);
                for mut i in __range1 {
                    sub = subs.clone().borrow()[(i.clone()-1) as usize].clone();
                    val = vals.clone().borrow()[(i.clone()-1) as usize].clone();
                    idx = Expression::toInteger(sub.clone())?;
                    {let _arr = var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(); let _val = assignArrayElement(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone().borrow()[(idx.clone()-1) as usize].clone(), rest_subs.clone(), val.clone())?; _arr.borrow_mut()[(idx.clone()-1) as usize] = _val; _arr};
                }
            }
            arrayExp.clone()
        },
        (Deref @ Expression::ARRAY { .. }, Deref @ metamodelica::List::Cons { head: Deref @ Subscript::WHOLE, tail: rest_subs }) => {
            if rest_subs.clone().is_empty() {
                assign_variant_field!(arrayExp => Expression::NFExpression::ARRAY; elements = metamodelica::arrayFromVec(Expression::arrayElements(value.clone())?.borrow().clone()));
            } else {
                assign_variant_field!(arrayExp => Expression::NFExpression::ARRAY; elements = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for (e, v) in (var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter().cloned().zip((Expression::arrayElements(value.clone())?).borrow().iter().cloned()) {
            let __x = assignArrayElement(e.clone(), rest_subs.clone(), v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()));
            }
            arrayExp.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.assignArrayElement")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(arrayExp.clone())?); __mm_s.push_str(&*Subscript::toStringList(subscripts.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(value.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn assignExp(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ Expression::RECORD { .. } => assignRecord(lhs.clone(), rhs.clone())?,
        _ => rhs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn assignRecord(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(rhs.clone()) {
        Deref @ Expression::RECORD { .. } => {
            let mut elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let __pa0 = ::match_deref::match_deref! { match &(lhs.clone()) {
                Deref @ Expression::RECORD { elements: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elems = __pa0.clone();
            for mut v in &*var_field!((*rhs).elements, Expression::NFExpression::RECORD).clone() {
                let mut v = v.clone();
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(elems.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa1.clone();
                elems = __pa2.clone();
                assignVariable(e.clone(), v.clone())?;
            }
            lhs.clone()
        },
        Deref @ Expression::CREF { .. } => {
            let mut elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
            let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let __pa0 = ::match_deref::match_deref! { match &(lhs.clone()) {
                Deref @ Expression::RECORD { elements: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elems = __pa0.clone();
            cls_tree = Class::classTree(InstNode::getClass(ComponentRef::node(var_field!((*rhs).cref, Expression::NFExpression::CREF).clone())?)?)?;
            comps = ClassTree::getComponents(cls_tree.clone())?;
            let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range1 {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(elems.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa2.clone();
                elems = __pa3.clone();
                ty = InstNode::getType(c.clone())?;
                val = Arc::new(Expression::NFExpression::CREF { ty: Type::liftArrayLeftList(ty.clone(), Type::arrayDims(var_field!((*rhs).ty, Expression::NFExpression::CREF).clone())), cref: ComponentRef::prefixCref(c.clone(), ty.clone(), metamodelica::nil(), var_field!((*rhs).cref, Expression::NFExpression::CREF).clone()) });
                assignVariable(e.clone(), val.clone())?;
            }
            lhs.clone()
        },
        _ => {
            rhs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evaluateFor(mut iterator: Arc<InstNode::InstNode>, mut range: Option<Arc<Expression::NFExpression>>, mut forBody: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut source: Arc<DAE::ElementSource>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    let mut range_iter: Arc<RangeIterator::NFRangeIterator>;
    let mut iter_exp: Mutable::Mutable<Arc<Expression::NFExpression>>;
    let mut range_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = forBody.clone();
    let mut i: i32 = 0;
    let mut limit: i32 = Flags::getConfigInt(Flags::EVAL_LOOP_LIMIT.clone())?;
    range_exp = Ceval::evalExp(Util::getOption(range.clone())?, evalTargetFromSource(source.clone(), STATEMENT_CONTEXT.clone(), context.clone()))?;
    range_iter = RangeIterator::fromExp(range_exp.clone())?;
    if RangeIterator::hasNext(range_iter.clone())? {
        let __pa0 = ::match_deref::match_deref! { match &(iterator.clone()) {
            Deref @ InstNode::ITERATOR_NODE { exp: Deref @ Expression::MUTABLE { exp: __pa0 } } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        iter_exp = __pa0.clone();
        while RangeIterator::hasNext(range_iter.clone())? {
            (range_iter, value) = RangeIterator::next(range_iter.clone())?;
            Mutable::update(iter_exp.clone(), value.clone());
            ctrl = evaluateStatements(body.clone(), context.clone())?;
            if ctrl.clone() != FlowControl::NEXT.clone() {
                if ctrl.clone() == FlowControl::BREAK.clone() {
                    ctrl = FlowControl::NEXT.clone();
                }
                break;
            }
            i = i.clone() + 1;
            if i.clone() > limit.clone() {
                Error::addSourceMessage(Error::EVAL_LOOP_LIMIT_REACHED.clone(), list![ArcStr::from(::std::format!("{}", limit.clone()))], ElementSource::getInfo(source.clone()))?;
                bail!("fail");
            }
        }
    }
    Ok(ctrl)
}

fn evaluateIf(mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>, mut source: Arc<DAE::ElementSource>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT;
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        (cond, body) = branch.clone();
        if Expression::isTrue(Ceval::evalExp(cond.clone(), evalTargetFromSource(source.clone(), IF_COND_CONTEXT.clone(), context.clone()))?) {
            ctrl = evaluateStatements(body.clone(), context.clone())?;
            return Ok(ctrl);
        }
    }
    ctrl = FlowControl::NEXT.clone();
    Ok(ctrl)
}

fn evaluateAssert(mut condition: Arc<Expression::NFExpression>, mut assertStmt: Arc<Statement::NFStatement>, mut source: Arc<DAE::ElementSource>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut msg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lvl: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut target: Arc<EvalTarget::EvalTarget> = evalTargetFromSource(source.clone(), STATEMENT_CONTEXT.clone(), context.clone());
    if Expression::isFalse(Ceval::evalExp(condition.clone(), target.clone())?) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(assertStmt.clone()) {
            Deref @ Statement::ASSERT { level: __pa0, message: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        lvl = __pa0.clone();
        msg = __pa1.clone();
        msg = Ceval::evalExp(msg.clone(), target.clone())?;
        lvl = Ceval::evalExp(lvl.clone(), target.clone())?;
        let () = (::match_deref::match_deref! { match &((msg.clone(), lvl.clone())) {
        (Deref @ Expression::STRING { .. }, Deref @ Expression::ENUM_LITERAL { name: Deref @ "warning", .. }) => {
            Error::addSourceMessage(Error::ASSERT_TRIGGERED_WARNING.clone(), list![(var_field!((*msg).value, Expression::NFExpression::STRING).clone()).clone()], Ceval::EvalTarget::getInfo(target.clone()))?;
            ()
        },
        (Deref @ Expression::STRING { .. }, Deref @ Expression::ENUM_LITERAL { name: Deref @ "error", .. }) => {
            Error::addSourceMessage(Error::ASSERT_TRIGGERED_ERROR.clone(), list![(var_field!((*msg).value, Expression::NFExpression::STRING).clone()).clone()], Ceval::EvalTarget::getInfo(target.clone()))?;
            ctrl = FlowControl::ASSERTION.clone();
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.evaluateAssert")); __mm_s.push_str(&*literal!(" failed to evaluate assert(false, ")); __mm_s.push_str(&*Expression::toString(msg.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(lvl.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(ctrl)
}

fn evaluateNoRetCall(mut callExp: Arc<Expression::NFExpression>, mut source: Arc<DAE::ElementSource>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    Ceval::evalExp(callExp.clone(), evalTargetFromSource(source.clone(), STATEMENT_CONTEXT.clone(), context.clone()))?;
    Ok(ctrl)
}

fn evaluateWhile(mut condition: Arc<Expression::NFExpression>, mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut source: Arc<DAE::ElementSource>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    let mut i: i32 = 0;
    let mut limit: i32 = Flags::getConfigInt(Flags::EVAL_LOOP_LIMIT.clone())?;
    let mut target: Arc<EvalTarget::EvalTarget> = evalTargetFromSource(source.clone(), STATEMENT_CONTEXT.clone(), context.clone());
    while Expression::isTrue(Ceval::evalExp(condition.clone(), target.clone())?) {
        ctrl = evaluateStatements(body.clone(), context.clone())?;
        if ctrl.clone() != FlowControl::NEXT.clone() {
            if ctrl.clone() == FlowControl::BREAK.clone() {
                ctrl = FlowControl::NEXT.clone();
            }
            break;
        }
        i = i.clone() + 1;
        if i.clone() > limit.clone() {
            Error::addSourceMessage(Error::EVAL_LOOP_LIMIT_REACHED.clone(), list![ArcStr::from(::std::format!("{}", limit.clone()))], ElementSource::getInfo(source.clone()))?;
            bail!("fail");
        }
    }
    Ok(ctrl)
}

fn evalTargetFromSource(mut source: Arc<DAE::ElementSource>, mut context: i32, mut currentContext: i32) -> Arc<EvalTarget::EvalTarget> {
    let mut target: Arc<EvalTarget::EvalTarget> = Ceval::EvalTarget::new(ElementSource::getInfo(source.clone()), InstContext::set(context.clone(), currentContext.clone()), None);
    target
}

fn evaluateExternal2(mut name: ArcStr, mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut extArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut map: ArgumentMap;
    let mut ext_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    map = createArgumentMap(r#fn.inputs.clone(), r#fn.outputs.clone(), r#fn.locals.clone(), args.clone(), true, true)?;
    ext_args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (extArgs.clone()).into_iter().cloned() {
            let __x = Expression::map(e.clone(), Arc::new({ let __pe_b0 = map.clone(); move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    evaluateExternal3((name.clone()).clone(), ext_args.clone())?;
    result = createResult(map.clone(), r#fn.outputs.clone())?;
    Ok(result)
}

fn evaluateExternal3(mut name: ArcStr, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "dgeev" => {
            EvalFunctionExt::Lapack_dgeev(args.clone())?;
            ()
        },
        Deref @ "dgegv" => {
            EvalFunctionExt::Lapack_dgegv(args.clone())?;
            ()
        },
        Deref @ "dgels" => {
            EvalFunctionExt::Lapack_dgels(args.clone())?;
            ()
        },
        Deref @ "dgelsx" => {
            EvalFunctionExt::Lapack_dgelsx(args.clone())?;
            ()
        },
        Deref @ "dgelsy" => {
            EvalFunctionExt::Lapack_dgelsy(args.clone())?;
            ()
        },
        Deref @ "dgesv" => {
            EvalFunctionExt::Lapack_dgesv(args.clone())?;
            ()
        },
        Deref @ "dgglse" => {
            EvalFunctionExt::Lapack_dgglse(args.clone())?;
            ()
        },
        Deref @ "dgtsv" => {
            EvalFunctionExt::Lapack_dgtsv(args.clone())?;
            ()
        },
        Deref @ "dgbsv" => {
            EvalFunctionExt::Lapack_dgtsv(args.clone())?;
            ()
        },
        Deref @ "dgesvd" => {
            EvalFunctionExt::Lapack_dgesvd(args.clone())?;
            ()
        },
        Deref @ "dgetrf" => {
            EvalFunctionExt::Lapack_dgetrf(args.clone())?;
            ()
        },
        Deref @ "dgetrs" => {
            EvalFunctionExt::Lapack_dgetrs(args.clone())?;
            ()
        },
        Deref @ "dgetri" => {
            EvalFunctionExt::Lapack_dgetri(args.clone())?;
            ()
        },
        Deref @ "dgeqpf" => {
            EvalFunctionExt::Lapack_dgeqpf(args.clone())?;
            ()
        },
        Deref @ "dorgqr" => {
            EvalFunctionExt::Lapack_dorgqr(args.clone())?;
            ()
        },
        Deref @ "dhseqr" => {
            EvalFunctionExt::Lapack_dhseqr(args.clone())?;
            ()
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn callExternalFunction(mut extName: ArcStr, mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut extArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut outputRef: Arc<ComponentRef::NFComponentRef>, mut extAnnotation: Option<Arc<SCode::Annotation>>, mut debug: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut pkg_name: ArcStr = arcstr::literal!("");
    let mut mapped_args: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut specs: metamodelica::Array<FFI::ArgSpec>;
    let mut ret_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut output_vals: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut fn_handle: i32 = 0;
    info = InstNode::info(r#fn.node.clone())?;
    checkExtReturnValue(outputRef.clone(), info.clone())?;
    pkg_name = (InstNode::name(InstNode::libraryScope(r#fn.node.clone())?)?).clone();
    fn_handle = loadLibraryFunction((pkg_name.clone()).clone(), (extName.clone()).clone(), extAnnotation.clone(), debug.clone(), info.clone())?;
    match '__try0: {
        (mapped_args, specs) = unwrap_break_err!(mapExternalArgs(r#fn.clone(), args.clone(), extArgs.clone()), '__try0);
        ret_ty = if (ComponentRef::isCref(outputRef.clone())) {unwrap_break_err!(ComponentRef::nodeType(outputRef.clone()), '__try0)} else {Arc::new(crate::NFType::NORETCALL)};
        (res, output_vals) = unwrap_break_err!(FFI::callFunction(fn_handle.clone(), mapped_args.clone(), specs.clone(), ret_ty.clone()), '__try0);
        unwrap_break_err!(freeLibraryFunction(fn_handle.clone(), debug.clone()), '__try0);
        Ok::<_, anyhow::Error>((mapped_args.clone(), output_vals.clone(), res.clone(), ret_ty.clone(), specs.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            mapped_args = __try0_o0;
            output_vals = __try0_o1;
            res = __try0_o2;
            ret_ty = __try0_o3;
            specs = __try0_o4;
        }
        Err(_) => {
            freeLibraryFunction(fn_handle.clone(), debug.clone())?;
            bail!("fail");
        }
    }
    if output_vals.clone().is_empty() {
        result = res.clone();
    } else {
        result = makeExternalResult(cons(res.clone(), output_vals.clone()), outputRef.clone(), extArgs.clone(), r#fn.outputs.clone())?;
    }
    Ok(result)
}

fn lookupLibraryInCache(mut libName: ArcStr) -> i32 {
    let mut libHandle: i32 = 0;
    let mut cache: Arc<metamodelica::List<(ArcStr, i32)>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    cache = openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| __root.borrow().clone());
    for mut l in &*cache.clone() {
        let mut l = l.clone();
        (name, libHandle) = l.clone();
        if name.clone() == libName.clone() {
            return libHandle;
        }
    }
    libHandle = -1;
    libHandle
}

fn cacheLibrary(mut libName: ArcStr, mut libHandle: i32) -> () {
    let mut cache: Arc<metamodelica::List<(ArcStr, i32)>> = metamodelica::nil();
    cache = openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| __root.borrow().clone());
    cache = cons((libName.clone(), libHandle.clone()), cache.clone());
    openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
    ()
}

pub fn clearLibraryCache() -> Result<()> {
    let mut cache: Arc<metamodelica::List<(ArcStr, i32)>> = metamodelica::nil();
    let mut lib_handle: i32 = 0;
    cache = openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| __root.borrow().clone());
    for mut v in &*cache.clone() {
        let mut v = v.clone();
        (_, lib_handle) = v.clone();
        System::freeLibrary(lib_handle.clone(), false)?;
    }
    openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| *__root.borrow_mut() = metamodelica::nil());
    Ok(())
}

fn loadLibraryFunction(mut libName: ArcStr, mut fnName: ArcStr, mut extAnnotation: Option<Arc<SCode::Annotation>>, mut debug: bool, mut info: SourceInfo) -> Result<i32> {
    let mut fnHandle: i32 = 0;
    let mut lib_handle: i32 = 0;
    let mut ann: Arc<SCode::Annotation> = Arc::new(<SCode::Annotation as ::std::default::Default>::default());
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut dirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut paths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut found: bool = false;
    let mut installLibDir: ArcStr = arcstr::literal!("");
    if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
        installLibDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/bin")); ArcStr::from(__mm_s) }).clone();
    } else {
        installLibDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); ArcStr::from(__mm_s) }).clone();
    }
    if isSome(extAnnotation.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(extAnnotation.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        ann = __pa0.clone();
        libs = parseExternalAnnotation((literal!("Library")).clone(), ann.clone())?;
        dirs = parseExternalAnnotation((literal!("LibraryDirectory")).clone(), ann.clone())?;
    }
    dirs = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*libName.clone()); __mm_s.push_str(&*literal!("/Resources/Library")); ArcStr::from(__mm_s) }).clone(), dirs.clone());
    libs = List::unique(libs.clone());
    dirs = List::unique(dirs.clone());
    for mut lib in &*libs.clone() {
        let mut lib = lib.clone();
        if !(stringEmpty((lib.clone()).clone())) {
            libs2 = cons((lib.clone()).clone(), libs2.clone());
            libs2 = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*lib.clone()); ArcStr::from(__mm_s) }).clone(), libs2.clone());
        }
    }
    libs = libs2.clone();
    for mut lib in &*libs.clone() {
        let mut lib = lib.clone();
        paths = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*installLibDir.clone()); __mm_s.push_str(&*literal!("/ffi/")); __mm_s.push_str(&*lib.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) }).clone(), paths.clone());
    }
    for mut lib in &*libs.clone() {
        let mut lib = lib.clone();
        if stringEmpty((lib.clone()).clone()) {
            paths = cons((literal!("")).clone(), paths.clone());
            continue;
        }
        lib = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*lib.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) }).clone();
        for mut dir in &*dirs.clone() {
            let mut dir = dir.clone();
            paths = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*lib.clone()); ArcStr::from(__mm_s) }).clone(), paths.clone());
            paths = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::modelicaPlatform()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*lib.clone()); ArcStr::from(__mm_s) }).clone(), paths.clone());
            if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
                paths = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::openModelicaPlatform()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*lib.clone()); ArcStr::from(__mm_s) }).clone(), cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::openModelicaPlatformAlternative()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*lib.clone()); ArcStr::from(__mm_s) }).clone(), paths.clone()));
            }
        }
        paths = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*installLibDir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*lib.clone()); ArcStr::from(__mm_s) }).clone(), paths.clone());
    }
    if libs.clone().is_empty() {
        paths = cons((literal!("")).clone(), paths.clone());
    }
    ErrorExt::setCheckpoint((literal!("NFEvalFunction.loadLibraryFunction")).clone());
    for mut path in &*paths.clone() {
        let mut path = path.clone();
        match '__try1: {
            if !(stringEmpty((path.clone()).clone())) {
                path = uriToFilename((path.clone()).clone());
            }
            lib_handle = lookupLibraryInCache((path.clone()).clone());
            if lib_handle.clone() == -1 {
                lib_handle = unwrap_break_err!(System::loadLibrary((path.clone()).clone(), false, debug.clone()), '__try1);
                cacheLibrary((path.clone()).clone(), lib_handle.clone());
            }
            fnHandle = unwrap_break_err!(System::lookupFunction(lib_handle.clone(), (fnName.clone()).clone()), '__try1);
            found = true;
            Ok::<_, anyhow::Error>((fnHandle.clone(), found.clone(), lib_handle.clone()))
        } {
            Ok((__try1_o0, __try1_o1, __try1_o2)) => {
                fnHandle = __try1_o0;
                found = __try1_o1;
                lib_handle = __try1_o2;
            }
            Err(_) => {
                bail!("try/else: outputs not set in else branch");
            }
        }
        if found.clone() {
            break;
        }
    }
    ErrorExt::rollBack((literal!("NFEvalFunction.loadLibraryFunction")).clone());
    if !(found.clone()) {
        paths = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
            if !(!(stringEmpty((p.clone()).clone()))) { continue; }
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*Testsuite::friendly(uriToFilename((p.clone()).clone()))?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        Error::addSourceMessage(Error::EXTERNAL_FUNCTION_NOT_FOUND.clone(), list![(fnName.clone()).clone(), stringDelimitList(paths.clone(), (literal!("\n")).clone())], info.clone())?;
        bail!("fail");
    }
    Ok(fnHandle)
}

fn parseExternalAnnotation(mut name: ArcStr, mut ann: Arc<SCode::Annotation>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut mods: Arc<metamodelica::List<Arc<SCode::Mod>>> = metamodelica::nil();
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    mods = SCodeUtil::lookupAnnotations(ann.clone(), (name.clone()).clone())?;
    for mut m in &*mods.clone() {
        let mut m = m.clone();
        strl = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ SCode::Mod::MOD { binding: Some(exp), .. } => parseExternalAnnotationExp(exp.clone(), strl.clone()),
        _ => strl.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(strl)
}

fn parseExternalAnnotationExp(mut exp: Arc<Absyn::Exp>, mut strl: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut strl: Arc<metamodelica::List<ArcStr>> = strl;
    strl = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::STRING { .. } => cons((var_field!((*exp).value, Absyn::Exp::STRING).clone()).clone(), strl.clone()),
        Deref @ Absyn::Exp::ARRAY { .. } => List::fold(var_field!((*exp).arrayExp, Absyn::Exp::ARRAY).clone(), (std::sync::Arc::new(fnptr!(parseExternalAnnotationExp, Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), strl.clone()),
        _ => strl.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    strl
}

fn freeLibraryFunction(mut fnHandle: i32, mut debug: bool) -> Result<()> {
    System::freeFunction(fnHandle.clone(), debug.clone())?;
    Ok(())
}

fn mapExternalArgs(mut r#fn: Arc<Function::Function>, mut inputArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut extArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(metamodelica::Array<Arc<Expression::NFExpression>>, metamodelica::Array<FFI::ArgSpec>)> {
    let mut mappedArgs: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut argSpecs: metamodelica::Array<FFI::ArgSpec>;
    let mut arg_map: ArgumentMap;
    let mut marg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg_spec: FFI::ArgSpec = FFI::ArgSpec::INPUT;
    let mut args_len: i32 = 0;
    let mut i: i32 = 1;
    let mut input_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    input_args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (inputArgs.clone()).into_iter().cloned() {
            let __x = makeExternalArg(arg.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    arg_map = createArgumentMap(r#fn.inputs.clone(), r#fn.outputs.clone(), r#fn.locals.clone(), input_args.clone(), false, false)?;
    args_len = (extArgs.clone().len() as i32);
    mappedArgs = metamodelica::arrayCreate(args_len.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }));
    argSpecs = metamodelica::arrayCreate(args_len.clone(), FFI::ArgSpec::INPUT.clone());
    for mut ext_arg in &*extArgs.clone() {
        let mut ext_arg = ext_arg.clone();
        (marg, arg_spec) = mapExternalArg(ext_arg.clone(), arg_map.clone(), r#fn.clone())?;
        {
            let __cell0 = marg.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(mappedArgs.clone().clone(), i.clone(), __cell0); }
        }
        {
            let __cell1 = arg_spec.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(argSpecs.clone().clone(), i.clone(), __cell1); }
        }
        i = i.clone() + 1;
    }
    Ok((mappedArgs, argSpecs))
}

fn makeExternalArg(mut arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut extArg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    extArg = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::FILENAME { .. } => Arc::new(Expression::NFExpression::STRING { value: (var_field!((*arg).filename, Expression::NFExpression::FILENAME).clone()).clone() }),
        _ => arg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    extArg
}

fn mapExternalArg(mut extArg: Arc<Expression::NFExpression>, mut argMap: ArgumentMap, mut r#fn: Arc<Function::Function>) -> Result<(Arc<Expression::NFExpression>, FFI::ArgSpec)> {
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut spec: FFI::ArgSpec = FFI::ArgSpec::INPUT;
    let mut cr_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    arg = applyBindingReplacement(extArg.clone(), argMap.clone())?;
    arg = Ceval::evalExp(arg.clone(), Ceval::noTarget().clone())?;
    spec = (::match_deref::match_deref! { match &(extArg.clone()) {
        Deref @ Expression::CREF { .. } => {
            cr_node = ComponentRef::node(ComponentRef::last(var_field!((*extArg).cref, Expression::NFExpression::CREF).clone()))?;
            if InstNode::isProtected(cr_node.clone()) {
                spec = FFI::ArgSpec::LOCAL.clone();
            } else if InstNode::isOutput(cr_node.clone()) {
                spec = FFI::ArgSpec::OUTPUT.clone();
            } else {
                spec = FFI::ArgSpec::INPUT.clone();
            }
            spec.clone()
        },
        _ => FFI::ArgSpec::INPUT.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((arg, spec))
}

fn makeExternalResult(mut values: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut outputRef: Arc<ComponentRef::NFComponentRef>, mut extArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg_map: ArgumentMap;
    let mut val: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut vals: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ret_vals: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ret_val: Option<Arc<Expression::NFExpression>> = None;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    arg_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 1);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(values.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    val = __pa0.clone();
    vals = __pa1.clone();
    if ComponentRef::isCref(outputRef.clone()) {
        UnorderedMap::addUnique(ComponentRef::node(outputRef.clone())?, val.clone(), arg_map.clone())?;
    }
    for mut ext_arg in &*extArgs.clone() {
        let mut ext_arg = ext_arg.clone();
        let () = (::match_deref::match_deref! { match &(ext_arg.clone()) {
        Deref @ Expression::CREF { .. } if (InstNode::isOutput(ComponentRef::node(ComponentRef::last(var_field!((*ext_arg).cref, Expression::NFExpression::CREF).clone()))?)) => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(vals.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            val = __pa0.clone();
            vals = __pa1.clone();
            UnorderedMap::addUnique(ComponentRef::node(var_field!((*ext_arg).cref, Expression::NFExpression::CREF).clone())?, val.clone(), arg_map.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    ret_vals = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut o in (outputs.clone()).into_iter().cloned() {
            let __x = getExternalOutputResult(o.clone(), arg_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outExp = Expression::makeTuple(ret_vals.clone())?;
    Ok(outExp)
}

fn getExternalOutputResult(mut outputNode: Arc<InstNode::InstNode>, mut map: ArgumentMap) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut oexp: Option<Arc<Expression::NFExpression>> = None;
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    oexp = UnorderedMap::get(outputNode.clone(), map.clone());
    if isSome(oexp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(oexp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
    } else if InstNode::isRecord(outputNode.clone()) {
        cls_node = InstNode::classScope(outputNode.clone());
        comps = ClassTree::getComponents(Class::classTree(InstNode::getClass(cls_node.clone())?)?)?;
        expl = metamodelica::nil();
        let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range1 {
            expl = cons(getExternalOutputResult(c.clone(), map.clone())?, expl.clone());
        }
        exp = Expression::makeRecord(InstNode::fullPath(cls_node.clone(), false), InstNode::getType(cls_node.clone())?, expl.clone().reverse());
    } else {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.getExternalOutputResult")); __mm_s.push_str(&*literal!(" failed to find return value for output ")); __mm_s.push_str(&*InstNode::name(outputNode.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    Ok(exp)
}

fn checkExtReturnValue(mut cref: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<()> {
    if ComponentRef::isCref(cref.clone()) && Type::isRecord(ComponentRef::nodeType(cref.clone())?) {
        Error::addSourceMessage(Error::UNSUPPORTED_LANGUAGE_FEATURE.clone(), list![(literal!("\"record return value in external function\"")).clone(), (literal!("Pass the record as an output parameter")).clone()], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

