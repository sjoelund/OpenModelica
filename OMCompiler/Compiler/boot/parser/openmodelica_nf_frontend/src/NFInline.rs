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
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE::InlineType;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;

pub fn inlineCallExp(mut callExp: Arc<Expression::NFExpression>, mut forceInline: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut shouldInline: bool = false;
            shouldInline = (match Call::inlineType(call.clone()) {
        DAE::InlineType::BUILTIN_EARLY_INLINE { .. } => true,
        DAE::InlineType::EARLY_INLINE { .. } if (Flags::isSet(Flags::INLINE_FUNCTIONS.clone())?) => true,
        DAE::InlineType::NORM_INLINE { .. } => forceInline.clone() || Flags::getConfigBool(Flags::FRONTEND_INLINE.clone())?,
        _ => forceInline.clone(),
    });
            if (shouldInline.clone()) {inlineCall(callExp.clone(), forceInline.clone())?} else {callExp.clone()}
        },
        _ => {
            callExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn inlineCall(mut callExp: Arc<Expression::NFExpression>, mut forceInline: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut call: Arc<Call::NFCall>;
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut stmt: Arc<Statement::NFStatement>;
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let __pa0 = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    call = __pa0.clone();
    exp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_CALL { arguments: args, r#fn, .. } if (!(InstNode::isEmpty(r#fn.node.clone())) && InstNode::isNamed(InstNode::parentScope(r#fn.node.clone(), false)?, (literal!("'constructor'")).clone())) => {
            let mut args = (*args).clone();
            body = Function::getBody(r#fn.clone());
            if !(body.clone().is_empty() && r#fn.locals.clone().is_empty()) {
                exp = callExp.clone();
                return Ok(exp.clone());
            }
            binding = Component::getBinding(InstNode::component(listHead(r#fn.outputs.clone())?)?);
            if Binding::hasExp(binding.clone()) {
                exp = Binding::getExp(binding.clone())?;
                let true = (Expression::isRecord(exp.clone())) else { bail!("pattern mismatch") };
            } else {
                exp = Class::makeRecordExp(listHead(r#fn.outputs.clone())?, r#fn.node.clone(), true)?;
            }
            for mut i in &*r#fn.inputs.clone() {
                let mut i = i.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                arg = __pa0.clone();
                args = __pa1.clone();
                arg = inlineCallExp(arg.clone(), forceInline.clone())?;
                exp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = i.clone(); let __pe_b2 = arg.clone(); move |__pe_a0| replaceCrefNode(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            }
            exp.clone()
        },
        Deref @ Call::TYPED_CALL { arguments: args, r#fn: r#fn @ Deref @ Function::FUNCTION { locals, outputs, inputs, .. }, .. } if (Function::hasSingleOrEmptyBody(r#fn.clone())) => {
            let mut args = (*args).clone();
            body = Function::getBody(r#fn.clone());
            body = removeDeadCode(body.clone())?;
            if (body.clone().len() as i32) > 1 || (outputs.clone().len() as i32) != 1 || !(locals.clone().is_empty()) {
                exp = callExp.clone();
                return Ok(exp.clone());
            }
            if body.clone().is_empty() {
                stmt = makeOutputStatement(listHead(outputs.clone())?)?;
            } else {
                stmt = convertToAssignment(listHead(body.clone())?)?;
            }
            if !(Statement::isAssignment(stmt.clone())) {
                exp = callExp.clone();
                return Ok(exp.clone());
            }
            Error::assertion((inputs.clone().len() as i32) == (args.clone().len() as i32), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInline.inlineCall")); __mm_s.push_str(&*literal!(" got wrong number of arguments for ")); __mm_s.push_str(&*AbsynUtil::pathString(Function::name(r#fn.clone()), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            match '__try0: {
                for mut i in &*inputs.clone() {
                    let mut i = i.clone();
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(args.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                        _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    arg = __pa1.clone();
                    args = __pa2.clone();
                    arg = unwrap_break_err!(inlineCallExp(arg.clone(), forceInline.clone()), '__try0);
                    stmt = Statement::mapExp(stmt.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b1 = i.clone(); let __pe_b2 = arg.clone(); move |__pe_a0| replaceCrefNode(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }));
                }
                exp = getOutputExp(stmt.clone(), listHead(outputs.clone())?, call.clone());
                exp = unwrap_break_err!(Expression::map(exp.clone(), Arc::new({ let __pe_b1 = forceInline.clone(); move |__pe_a0| inlineCallExp(__pe_a0, __pe_b1.clone()) })), '__try0);
                Ok::<_, anyhow::Error>((exp.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp = __try0_o0;
                }
                Err(_) => {
                    exp = callExp.clone();
                }
            }
            exp.clone()
        },
        _ => callExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn replaceCrefNode(mut exp: Arc<Expression::NFExpression>, mut node: Arc<InstNode::InstNode>, mut value: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut repl_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (InstNode::refEqual(ComponentRef::node(ComponentRef::firstNonScope(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))?, node.clone())) => replaceCrefNode2(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), node.clone(), value.clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty = Expression::typeOf(exp.clone());
    repl_ty = Type::mapDims(ty.clone(), Arc::new({ let __pe_b1 = node.clone(); let __pe_b2 = value.clone(); move |__pe_a0| replaceDimExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    if !(referenceEq(&ty.clone(),&repl_ty.clone())) {
        exp = Expression::setType(repl_ty.clone(), exp.clone())?;
    }
    Ok(exp)
}

fn replaceCrefNode2(mut cref: Arc<ComponentRef::NFComponentRef>, mut node: Arc<InstNode::InstNode>, mut value: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut value: Arc<Expression::NFExpression> = value;
    if !(InstNode::refEqual(node.clone(), ComponentRef::node(cref.clone())?)) {
        value = replaceCrefNode2(ComponentRef::rest(cref.clone())?, node.clone(), value.clone())?;
        value = Expression::recordElement((InstNode::name(ComponentRef::node(cref.clone())?)?).clone(), value.clone())?;
    }
    value = Expression::applySubscripts(ComponentRef::getSubscripts(cref.clone()), value.clone(), false)?;
    Ok(value)
}

fn replaceDimExp(mut dim: Arc<Dimension::NFDimension>, mut node: Arc<InstNode::InstNode>, mut value: Arc<Expression::NFExpression>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = dim;
    dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp = Expression::map(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone(), Arc::new({ let __pe_b1 = node.clone(); let __pe_b2 = value.clone(); move |__pe_a0| replaceCrefNode(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            Dimension::fromExp(exp.clone(), var_field!((*dim).var, Dimension::NFDimension::EXP).clone())?
        },
        _ => {
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

fn removeDeadCode(mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = body;
    if (body.clone().len() as i32) > 1 && Statement::isReturn((body.clone()).get(2)?) {
        body = list![listHead(body.clone())?];
    }
    Ok(body)
}

fn convertToAssignment(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> {
    let mut outStmt: Arc<Statement::NFStatement>;
    outStmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::IF { .. } => convertIfToAssignment(stmt.clone())?,
        _ => stmt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStmt)
}

fn convertIfToAssignment(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement> = stmt;
    let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut if_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut output_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut s: Arc<Statement::NFStatement>;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::IF { source: __pa0, branches: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    source = __pa0.clone();
    branches = __pa1.clone();
    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(branches.clone().reverse()) {
        Deref @ metamodelica::List::Cons { head: (__pa2, __pa3), tail: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cond = __pa2.clone();
    body = __pa3.clone();
    branches = __pa4.clone();
    if !(branches.clone().is_empty()) && !(Expression::isTrue(cond.clone())) {
        return Ok(stmt.clone());
    }
    if (body.clone().len() as i32) != 1 {
        return Ok(stmt.clone());
    }
    s = convertToAssignment(listHead(body.clone())?)?;
    if !(Statement::isAssignment(s.clone())) {
        return Ok(stmt.clone());
    }
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Statement::ASSIGNMENT { rhs: __pa5, lhs: __pa6, .. } => (__pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    if_exp = __pa5.clone();
    output_exp = __pa6.clone();
    for mut b in &*branches.clone() {
        let mut b = b.clone();
        let (__pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(branches.clone()) {
            Deref @ metamodelica::List::Cons { head: (__pa7, __pa8), tail: __pa9 } => (__pa7.clone(), __pa8.clone(), __pa9.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa7.clone();
        body = __pa8.clone();
        branches = __pa9.clone();
        if (body.clone().len() as i32) != 1 {
            return Ok(stmt.clone());
        }
        s = convertToAssignment(listHead(body.clone())?)?;
        if !(Statement::isAssignment(s.clone())) {
            return Ok(stmt.clone());
        }
        let (__pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(s.clone()) {
            Deref @ Statement::ASSIGNMENT { ty: __pa10, rhs: __pa11, lhs: __pa12, .. } => (__pa10.clone(), __pa11.clone(), __pa12.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa10.clone();
        rhs = __pa11.clone();
        lhs = __pa12.clone();
        if !(Expression::isEqual(lhs.clone(), output_exp.clone())?) {
            return Ok(stmt.clone());
        }
        if_exp = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: rhs.clone(), falseBranch: if_exp.clone() });
    }
    stmt = Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: output_exp.clone(), rhs: if_exp.clone(), ty: ty.clone(), source: source.clone() });
    Ok(stmt)
}

fn makeOutputStatement(mut outputNode: Arc<InstNode::InstNode>) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement>;
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut cref_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut binding_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    binding = Component::getImplicitBinding(InstNode::component(outputNode.clone())?, InstNode::instanceParent(outputNode.clone()));
    if Binding::isBound(binding.clone()) {
        cref_exp = Expression::fromCref(ComponentRef::fromNode(outputNode.clone(), Arc::new(crate::NFType::UNKNOWN), metamodelica::nil(), ComponentRef::Origin::CREF.clone()), false)?;
        binding_exp = Binding::getExp(binding.clone())?;
        stmt = Statement::makeAssignment(cref_exp.clone(), binding_exp.clone(), Arc::new(crate::NFType::UNKNOWN), DAE::emptyElementSource().clone());
    } else {
        stmt = Arc::new(Statement::NFStatement::FAILURE { body: metamodelica::nil(), source: DAE::emptyElementSource().clone() });
    }
    Ok(stmt)
}

fn getOutputExp(mut stmt: Arc<Statement::NFStatement>, mut outputNode: Arc<InstNode::InstNode>, mut call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { lhs: Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { restCref: rest_cr, subscripts: Deref @ metamodelica::List::Nil, node: cr_node, .. }, .. }, .. } if (InstNode::refEqual(outputNode.clone(), cr_node.clone()) && !(ComponentRef::isFromCref(rest_cr.clone()))) => {
            var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone()
        },
        _ => {
            Arc::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

