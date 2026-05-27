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

use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use openmodelica_util::Error;
use openmodelica_util::Util;

pub fn isStructuralComponent(mut component: Arc<Component::NFComponent>, mut compAttrs: Arc<Attributes::NFAttributes>, mut compBinding: Arc<Binding::NFBinding>, mut compNode: Arc<InstNode::InstNode>, mut compEval: bool, mut parentEval: bool, mut context: i32) -> Result<bool> {
    let mut isStructural: bool = false;
    let mut is_fixed: bool = false;
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    if compAttrs.variability.clone() != Variability::PARAMETER.clone() {
        isStructural = false;
    } else if compEval.clone() || parentEval.clone() {
        binding = if (Binding::isBound(compBinding.clone())) {compBinding.clone()} else {Component::getTypeAttributeBinding(component.clone(), (literal!("start")).clone())};
        if !(Component::isFixed(component.clone())?) {
            isStructural = false;
        } else if Component::isExternalObject(component.clone())? {
            isStructural = false;
        } else if !(Binding::isBound(binding.clone()) || InstNode::hasBinding(compNode.clone())?) {
            if !(parentEval.clone()) && !(InstContext::inRelaxed(context.clone())) {
                Error::addSourceMessage(Error::UNBOUND_PARAMETER_EVALUATE_TRUE.clone(), list![(InstNode::name(compNode.clone())?).clone()], InstNode::info(compNode.clone())?)?;
            }
            isStructural = false;
        } else if isBindingNotFixed(binding.clone(), false, 4)? {
            isStructural = false;
        } else {
            isStructural = true;
        }
    } else {
        isStructural = false;
    }
    Ok(isStructural)
}

pub fn isBindingNotFixed(mut binding: Arc<Binding::NFBinding>, mut requireFinal: bool, mut maxDepth: i32) -> Result<bool> {
    let mut isNotFixed: bool = false;
    if maxDepth.clone() == 0 {
        isNotFixed = true;
        return Ok(isNotFixed);
    }
    if Binding::hasExp(binding.clone()) {
        isNotFixed = isExpressionNotFixed(Binding::getExp(binding.clone())?, requireFinal.clone(), maxDepth.clone())?;
    } else {
        isNotFixed = true;
    }
    Ok(isNotFixed)
}

pub fn isComponentBindingNotFixed(mut component: Arc<Component::NFComponent>, mut node: Arc<InstNode::InstNode>, mut requireFinal: bool, mut maxDepth: i32, mut isRecord: bool) -> Result<bool> {
    let mut isNotFixed: bool = false;
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    binding = Component::getBinding(component.clone());
    if Binding::isUnbound(binding.clone()) {
        if isRecord.clone() || InstNode::isRecord(node.clone()) {
            isNotFixed = false;
        } else {
            parent = InstNode::parent(node.clone());
            if InstNode::isComponent(parent.clone()) && InstNode::isRecord(parent.clone()) {
                isNotFixed = isComponentBindingNotFixed(InstNode::component(parent.clone())?, parent.clone(), requireFinal.clone(), maxDepth.clone(), true)?;
            } else {
                binding = Component::getTypeAttributeBinding(component.clone(), (literal!("start")).clone());
                isNotFixed = isBindingNotFixed(binding.clone(), requireFinal.clone(), maxDepth.clone())?;
            }
        }
    } else {
        isNotFixed = isBindingNotFixed(binding.clone(), requireFinal.clone(), maxDepth.clone())?;
    }
    Ok(isNotFixed)
}

pub fn isExpressionNotFixed(mut exp: Arc<Expression::NFExpression>, mut requireFinal: bool, mut maxDepth: i32) -> Result<bool> {
    let mut isNotFixed: bool = false;
    isNotFixed = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()) && !(ComponentRef::isIterator(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))) => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
            let mut var: Variability = Variability::CONSTANT;
            node = ComponentRef::node(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?;
            if InstNode::isComponent(node.clone()) {
                c = InstNode::component(node.clone())?;
                var = Component::variability(c.clone());
                if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
                    isNotFixed = false;
                } else if var.clone() == Variability::PARAMETER.clone() && (!(requireFinal.clone()) || Component::isFinal(c.clone())?) && !(Component::isExternalObject(c.clone())?) && Component::isFixed(c.clone())? {
                    isNotFixed = isComponentBindingNotFixed(c.clone(), node.clone(), requireFinal.clone(), maxDepth.clone() - 1, false)?;
                } else {
                    isNotFixed = true;
                }
            } else {
                isNotFixed = true;
            }
            isNotFixed.clone() || Expression::containsShallow(exp.clone(), Arc::new({ let __pe_b1 = requireFinal.clone(); let __pe_b2 = maxDepth.clone(); move |__pe_a0| isExpressionNotFixed(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?
        },
        Deref @ Expression::SIZE { .. } => {
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                isNotFixed = isExpressionNotFixed(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?, requireFinal.clone(), maxDepth.clone())?;
            } else {
                isNotFixed = false;
            }
            isNotFixed.clone()
        },
        Deref @ Expression::CALL { .. } => {
            if Call::isImpure(var_field!((*exp).call, Expression::NFExpression::CALL).clone())? || Call::isExternal(var_field!((*exp).call, Expression::NFExpression::CALL).clone())? {
                isNotFixed = true;
            } else {
                isNotFixed = Expression::containsShallow(exp.clone(), Arc::new({ let __pe_b1 = requireFinal.clone(); let __pe_b2 = maxDepth.clone(); move |__pe_a0| isExpressionNotFixed(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            }
            isNotFixed.clone()
        },
        _ => {
            Expression::containsShallow(exp.clone(), Arc::new({ let __pe_b1 = requireFinal.clone(); let __pe_b2 = maxDepth.clone(); move |__pe_a0| isExpressionNotFixed(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isNotFixed)
}

pub fn markDimension(mut dimension: Arc<Dimension::NFDimension>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(dimension.clone()) {
        Deref @ Dimension::UNTYPED { .. } => {
            markExp(var_field!((*dimension).dimension, Dimension::NFDimension::UNTYPED).clone())?;
            ()
        },
        Deref @ Dimension::EXP { .. } => {
            markExp(var_field!((*dimension).exp, Dimension::NFDimension::EXP).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn markExp(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    use crate::NFComponentRef::Origin;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF { .. }, node, .. }, .. } => {
            let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
            if InstNode::isComponent(node.clone()) {
                comp = InstNode::component(node.clone())?;
                if Component::variability(comp.clone()) == Variability::PARAMETER.clone() {
                    markComponent(comp.clone(), node.clone())?;
                }
            }
            Expression::applyShallow(exp.clone(), (std::sync::Arc::new(markExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
            ()
        },
        Deref @ Expression::SIZE { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            markSubscriptsInExp(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone())?;
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
                markExp(e.clone())?;
            }
            ()
        },
        _ => {
            Expression::applyShallow(exp.clone(), (std::sync::Arc::new(markExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn markSubscriptsInExp(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            ComponentRef::applySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(markSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<()> + 'static>), false);
            ()
        },
        _ => {
            Expression::applyShallow(exp.clone(), (std::sync::Arc::new(markSubscriptsInExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn markComponent(mut component: Arc<Component::NFComponent>, mut node: Arc<InstNode::InstNode>) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Option<Arc<Expression::NFExpression>> = None;
    comp = Component::setVariability(Variability::STRUCTURAL_PARAMETER.clone(), component.clone());
    comp = Component::setFinal(comp.clone(), true);
    InstNode::updateComponent(comp.clone(), node.clone())?;
    binding = Binding::getExpOpt(Component::getBinding(comp.clone()));
    if isSome(binding.clone()) {
        markExp(Util::getOption(binding.clone())?)?;
    }
    Ok(())
}

pub fn markExpSize(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    Expression::apply(exp.clone(), (std::sync::Arc::new(markExpSize_traverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
    Ok(())
}

pub fn markExpSize_traverser(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::UNTYPED_ARRAY_CONSTRUCTOR { iters, .. } } => {
            for mut iter in &*iters.clone() {
                let mut iter = iter.clone();
                markExp(Util::tuple22(iter.clone()))?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn markSubscripts(mut exp: Arc<Expression::NFExpression>) -> () {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            ComponentRef::applySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(markSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<()> + 'static>), false);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn markSubscript(mut sub: Arc<Subscript::NFSubscript>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::UNTYPED { .. } => {
            markExp(var_field!((*sub).exp, Subscript::NFSubscript::UNTYPED).clone())?;
            ()
        },
        Deref @ Subscript::INDEX { .. } => {
            markExp(var_field!((*sub).index, Subscript::NFSubscript::INDEX).clone())?;
            ()
        },
        Deref @ Subscript::SLICE { .. } => {
            markExp(var_field!((*sub).slice, Subscript::NFSubscript::SLICE).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

