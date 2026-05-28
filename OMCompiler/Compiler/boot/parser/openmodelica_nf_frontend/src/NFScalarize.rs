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

use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableAttributes;
use crate::NFBinding as Binding;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub mod AttributeIterator {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AttributeIterator {
        pub name: ArcStr,
        pub iterator: Mutable::Mutable<Arc<ExpressionIterator::NFExpressionIterator>>,
    }

    impl Default for AttributeIterator {
        fn default() -> Self {
            Self {
                name: Default::default(),
                iterator: Default::default(),
            }
        }
    }

    pub type ATTRIBUTE_ITERATOR = AttributeIterator;

    pub fn create(mut attribute: (ArcStr, Arc<Binding::NFBinding>)) -> Result<Arc<AttributeIterator>> {
        let mut iter: Arc<AttributeIterator> = Arc::new(<AttributeIterator as ::std::default::Default>::default());
        let mut name: ArcStr = arcstr::literal!("");
        let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        (name, binding) = attribute.clone();
        iter = Arc::new(AttributeIterator { name: (name.clone()).clone(), iterator: Mutable::create(ExpressionIterator::fromBinding(binding.clone())?) });
        Ok(iter)
    }

    pub fn nextBinding(mut iter: Arc<AttributeIterator>) -> Result<(ArcStr, Arc<Binding::NFBinding>)> {
        let mut binding: (ArcStr, Arc<Binding::NFBinding>);
        let mut it: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        (it, exp) = ExpressionIterator::next(Mutable::access(iter.iterator.clone()))?;
        Mutable::update(iter.iterator.clone(), it.clone());
        binding = (iter.name.clone(), Binding::makeFlat(exp.clone(), Variability::PARAMETER.clone(), Binding::Source::BINDING.clone()));
        Ok(binding)
    }

}

pub fn scalarize(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = scalarizeVariables(flatModel.variables.clone(), false)?,
        flatModel.equations = Equation::mapExpList(flatModel.equations.clone(), (std::sync::Arc::new(expandComplexCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)),
        flatModel.equations = scalarizeEquations(flatModel.equations.clone(), false)?,
        flatModel.initialEquations = Equation::mapExpList(flatModel.initialEquations.clone(), (std::sync::Arc::new(expandComplexCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)),
        flatModel.initialEquations = scalarizeEquations(flatModel.initialEquations.clone(), false)?,
        flatModel.algorithms = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (flatModel.algorithms.clone()).into_iter().cloned() {
            let __x = scalarizeAlgorithm(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.initialAlgorithms = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (flatModel.initialAlgorithms.clone()).into_iter().cloned() {
            let __x = scalarizeAlgorithm(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
    );
    execStat((literal!("NFScalarize.scalarize")).clone())?;
    Ok(flatModel)
}

pub fn scalarizeVariables(mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut forceScalarize: bool) -> Result<Arc<metamodelica::List<Arc<Variable::NFVariable>>>> {
    let mut outVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    for mut v in &*vars.clone() {
        let mut v = v.clone();
        outVars = scalarizeVariable(v.clone(), outVars.clone(), forceScalarize.clone())?;
    }
    outVars = outVars.clone().reverse();
    Ok(outVars)
}

pub fn scalarizeVariable(mut var: Arc<Variable::NFVariable>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut forceScalarize: bool) -> Result<Arc<metamodelica::List<Arc<Variable::NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut elem_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut vis: Visibility = Visibility::PUBLIC;
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut ty_attr: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut binding_iter: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty_attr_iters: Arc<metamodelica::List<Arc<AttributeIterator::AttributeIterator>>> = metamodelica::nil();
    let mut backend_attributes: Arc<metamodelica::List<Arc<BackendInfo::BackendInfo>>> = metamodelica::nil();
    let mut bind_var: Variability = Variability::CONSTANT;
    let mut binfo: Arc<BackendInfo::BackendInfo> = Arc::new(<BackendInfo::BackendInfo as ::std::default::Default>::default());
    let mut bind_src: Binding::Source = Binding::Source::BINDING;
    let mut has_binding: bool = false;
    assign_field!(var.binding = Binding::mapExp(var.binding.clone(), (std::sync::Arc::new(expandComplexCref_traverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?);
    if Type::isArray(var.ty.clone()) && Type::hasKnownSize(var.ty.clone()) {
        if '__try0: {
            let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(var.clone()) {
                Deref @ Variable::VARIABLE { name: __pa1, ty: __pa2, binding: __pa3, visibility: __pa4, attributes: __pa5, typeAttributes: __pa6, children: _, comment: __pa7, info: __pa8, backendinfo: __pa9 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            name = __pa1.clone();
            ty = __pa2.clone();
            binding = __pa3.clone();
            vis = __pa4.clone();
            attr = __pa5.clone();
            ty_attr = __pa6.clone();
            cmt = __pa7.clone();
            info = __pa8.clone();
            binfo = __pa9.clone();
            crefs = unwrap_break_err!(ComponentRef::scalarize(name.clone(), false), '__try0);
            if crefs.clone().is_empty() {
                return Ok(vars);
            }
            has_binding = Binding::isBound(binding.clone());
            bind_src = Binding::source(binding.clone());
            if has_binding.clone() {
                binding_iter = unwrap_break_err!(ExpressionIterator::fromExp(Binding::getTypedExp(binding.clone())?, false, false), '__try0);
                bind_var = unwrap_break_err!(Binding::variability(binding.clone()), '__try0);
                if !(forceScalarize.clone()) && unwrap_break_err!(ExpressionIterator::isSubscriptedArrayCall(binding_iter.clone(), true), '__try0) && !(unwrap_break_err!(Flags::getConfigBool(Flags::BUILDING_FMU.clone()), '__try0)) && !(variableHasForcedScalarAttribute(var.clone())) {
                    vars = cons(var.clone(), vars.clone());
                    return Ok(vars);
                }
            } else {
                bind_var = Variability::CONSTANT.clone();
            }
            elem_ty = Type::arrayElementType(ty.clone());
            ty_attr_iters = {
        let mut __acc: Arc<metamodelica::List<Arc<AttributeIterator::AttributeIterator>>> = metamodelica::nil();
        for mut a in (ty_attr.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(AttributeIterator::create(a.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            backend_attributes = unwrap_break_err!(BackendInfo::scalarize(binfo.clone(), (crefs.clone().len() as i32)), '__try0);
            for mut cr in &*crefs.clone() {
                let mut cr = cr.clone();
                if has_binding.clone() {
                    (binding_iter, exp) = unwrap_break_err!(ExpressionIterator::next(binding_iter.clone()), '__try0);
                    binding = Binding::makeFlat(exp.clone(), bind_var.clone(), bind_src.clone());
                }
                ty_attr = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut i in (ty_attr_iters.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(AttributeIterator::nextBinding(i.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                let (__pa10, __pa11) = ::match_deref::match_deref! { match &(backend_attributes.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                binfo = __pa10.clone();
                backend_attributes = __pa11.clone();
                vars = cons(Arc::new(Variable::NFVariable { name: cr.clone(), ty: elem_ty.clone(), binding: binding.clone(), visibility: vis.clone(), attributes: attr.clone(), typeAttributes: ty_attr.clone(), children: metamodelica::nil(), comment: cmt.clone(), info: info.clone(), backendinfo: binfo.clone() }), vars.clone());
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFScalarize.scalarizeVariable")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*Variable::toString(var.clone(), (literal!("")).clone(), true)?); ArcStr::from(__mm_s) }).clone(), var.info.clone())?;
        }
    } else {
        vars = cons(var.clone(), vars.clone());
    }
    Ok(vars)
}

pub fn scalarizeBackendVariable(mut var: Arc<Variable::NFVariable>, mut indices: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<Variable::NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut binding_iter: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut bind_var: Variability = Variability::CONSTANT;
    let mut bind_src: Binding::Source = Binding::Source::BINDING;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut elem_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut binfo: Arc<BackendInfo::BackendInfo> = Arc::new(<BackendInfo::BackendInfo as ::std::default::Default>::default());
    let mut backend_attributes: Arc<metamodelica::List<Arc<BackendInfo::BackendInfo>>> = metamodelica::nil();
    if '__try0: {
        crefs = unwrap_break_err!(ComponentRef::scalarizeAll(ComponentRef::stripSubscriptsAll(var.name.clone()), false), '__try0).reverse();
        elem_ty = Type::arrayElementType(var.ty.clone());
        backend_attributes = unwrap_break_err!(BackendInfo::scalarize(var.backendinfo.clone(), (crefs.clone().len() as i32)), '__try0);
        if Binding::isBound(var.binding.clone()) {
            binding_iter = unwrap_break_err!(ExpressionIterator::fromExp(Binding::getTypedExp(var.binding.clone())?, true, false), '__try0);
            bind_var = unwrap_break_err!(Binding::variability(var.binding.clone()), '__try0);
            bind_src = Binding::source(var.binding.clone());
            vars = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut cr in (crefs.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(cr.clone()) {
        _ => {
            (binding_iter, exp) = unwrap_break_err!(ExpressionIterator::next(binding_iter.clone()), '__try0);
            binding = Binding::makeFlat(exp.clone(), bind_var.clone(), bind_src.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(backend_attributes.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            binfo = __pa0.clone();
            backend_attributes = __pa1.clone();
            Arc::new(Variable::NFVariable { name: cr.clone(), ty: elem_ty.clone(), binding: binding.clone(), visibility: var.visibility.clone(), attributes: var.attributes.clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: var.comment.clone(), info: var.info.clone(), backendinfo: binfo.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        } else {
            vars = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut cr in (crefs.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(cr.clone()) {
        _ => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(backend_attributes.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            binfo = __pa0.clone();
            backend_attributes = __pa1.clone();
            Arc::new(Variable::NFVariable { name: cr.clone(), ty: elem_ty.clone(), binding: var.binding.clone(), visibility: var.visibility.clone(), attributes: var.attributes.clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: var.comment.clone(), info: var.info.clone(), backendinfo: binfo.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        }
        if !(indices.clone().is_empty() || (indices.clone().len() as i32) == (vars.clone().len() as i32)) {
            vars = unwrap_break_err!(List::keepPositions(vars.clone(), indices.clone(), true), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFScalarize.scalarizeBackendVariable")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Variable::toString(var.clone(), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    Ok(vars)
}

pub fn scalarizeComplexVariable(mut var: Arc<Variable::NFVariable>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<metamodelica::List<Arc<Variable::NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    vars = (::match_deref::match_deref! { match &(var.backendinfo.attributes.clone()) {
        attr @ Deref @ VariableAttributes::VAR_ATTR_RECORD { .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut index: i32 = 0;
            let mut elem_var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            for mut tpl in &*UnorderedMap::toList(var_field!((**attr).indexMap, VariableAttributes::VariableAttributes::VAR_ATTR_RECORD).clone()) {
                let mut tpl = tpl.clone();
                (name, index) = tpl.clone();
                elem_var = var.clone();
                assign_field!(
                    elem_var.name = ComponentRef::prepend(elem_var.name.clone(), ComponentRef::rename((name.clone()).clone(), elem_var.name.clone())?)?,
                    elem_var.backendinfo = BackendInfo::setAttributes(elem_var.backendinfo.clone(), var_field!((**attr).childrenAttr, VariableAttributes::VariableAttributes::VAR_ATTR_RECORD).borrow()[(index.clone()-1) as usize].clone(), var.backendinfo.annotations.clone()),
                    elem_var.ty = VariableAttributes::elemType(var_field!((**attr).childrenAttr, VariableAttributes::VariableAttributes::VAR_ATTR_RECORD).borrow()[(index.clone()-1) as usize].clone())?,
                    elem_var.name = ComponentRef::setNodeType(elem_var.ty.clone(), elem_var.name.clone())
                );
                vars = cons(elem_var.clone(), vars.clone());
            }
            vars.clone().reverse()
        },
        _ => {
            list![var.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vars)
}

pub fn expandComplexCref(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), (std::sync::Arc::new(expandComplexCref_traverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

pub fn expandComplexCref_traverser(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { ty: Deref @ Type::ARRAY { .. }, .. } => {
            if ComponentRef::isComplexArray(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())? {
                (exp, _) = ExpandExp::expand(exp.clone(), false, false)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn scalarizeEquations(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut forceScalarize: bool) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        equations = scalarizeEquation(eq.clone(), equations.clone(), forceScalarize.clone())?;
    }
    equations = equations.clone().reverse();
    Ok(equations)
}

pub fn scalarizeEquation(mut eq: Arc<Equation::NFEquation>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut forceScalarize: bool) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    equations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { source: src, ty, rhs, lhs, .. } if (Type::isArray(ty.clone())) => {
            let mut lhs_iter: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
            let mut rhs_iter: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
            let mut scalarize: bool = false;
            let mut ty = (*ty).clone();
            let mut rhs = (*rhs).clone();
            let mut lhs = (*lhs).clone();
            if forceScalarize.clone() || var_field!((*eq).scalarizeMode, Equation::NFEquation::EQUALITY).clone() == Equation::ScalarizeMode::SCALARIZE.clone() {
                scalarize = true;
            } else if var_field!((*eq).scalarizeMode, Equation::NFEquation::EQUALITY).clone() == Equation::ScalarizeMode::DONT_SCALARIZE.clone() {
                scalarize = false;
            } else if Expression::hasArrayCall(lhs.clone())? || Expression::hasArrayCall(rhs.clone())? {
                scalarize = false;
            } else {
                scalarize = true;
            }
            if scalarize.clone() {
                lhs_iter = ExpressionIterator::fromExp(lhs.clone(), false, false)?;
                rhs_iter = ExpressionIterator::fromExp(rhs.clone(), false, false)?;
                ty = Type::arrayElementType(ty.clone());
                while ExpressionIterator::hasNext(lhs_iter.clone())? {
                    if !(ExpressionIterator::hasNext(rhs_iter.clone())?) {
                        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFScalarize.scalarizeEquation")); __mm_s.push_str(&*literal!(" could not expand rhs ")); __mm_s.push_str(&*Expression::toString(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone())?); ArcStr::from(__mm_s) }).clone(), ElementSource::getInfo(src.clone()))?;
                    }
                    (lhs_iter, lhs) = ExpressionIterator::next(lhs_iter.clone())?;
                    (rhs_iter, rhs) = ExpressionIterator::next(rhs_iter.clone())?;
                    equations = cons(Equation::makeEquality(lhs.clone(), rhs.clone(), ty.clone(), src.clone(), var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), Equation::ScalarizeMode::NO_PREFERENCE.clone()), equations.clone());
                }
            } else {
                equations = cons(eq.clone(), equations.clone());
            }
            equations.clone()
        },
        Deref @ Equation::CONNECT { .. } => {
            equations.clone()
        },
        Deref @ Equation::IF { .. } => {
            scalarizeIfEquation(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), var_field!((*eq).scope, Equation::NFEquation::IF).clone(), var_field!((*eq).source, Equation::NFEquation::IF).clone(), equations.clone())?
        },
        Deref @ Equation::WHEN { .. } => {
            scalarizeWhenEquation(var_field!((*eq).branches, Equation::NFEquation::WHEN).clone(), var_field!((*eq).scope, Equation::NFEquation::WHEN).clone(), var_field!((*eq).source, Equation::NFEquation::WHEN).clone(), equations.clone())?
        },
        _ => {
            cons(eq.clone(), equations.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub fn scalarizeIfEquation(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut bl: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    for mut b in &*branches.clone() {
        let mut b = b.clone();
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: __pa1, body: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa0.clone();
        var = __pa1.clone();
        body = __pa2.clone();
        body = scalarizeEquations(body.clone(), false)?;
        if !(body.clone().is_empty()) {
            bl = cons(Equation::makeBranch(cond.clone(), body.clone(), var.clone()), bl.clone());
        }
    }
    if !(bl.clone().is_empty()) {
        equations = cons(Arc::new(Equation::NFEquation::IF { branches: bl.clone().reverse(), scope: scope.clone(), source: source.clone() }), equations.clone());
    }
    Ok(equations)
}

pub fn scalarizeWhenEquation(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut bl: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    for mut b in &*branches.clone() {
        let mut b = b.clone();
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: __pa1, body: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa0.clone();
        var = __pa1.clone();
        body = __pa2.clone();
        body = scalarizeEquations(body.clone(), false)?;
        if Type::isArray(Expression::typeOf(cond.clone())) {
            (cond, _) = ExpandExp::expand(cond.clone(), false, false)?;
        }
        bl = cons(Equation::makeBranch(cond.clone(), body.clone(), var.clone()), bl.clone());
    }
    equations = cons(Arc::new(Equation::NFEquation::WHEN { branches: bl.clone().reverse(), scope: scope.clone(), source: source.clone() }), equations.clone());
    Ok(equations)
}

pub fn scalarizeAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = scalarizeStatements(alg.statements.clone())?);
    Ok(alg)
}

pub fn scalarizeStatements(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    for mut s in &*stmts.clone() {
        let mut s = s.clone();
        statements = scalarizeStatement(s.clone(), statements.clone())?;
    }
    statements = statements.clone().reverse();
    Ok(statements)
}

pub fn scalarizeStatement(mut stmt: Arc<Statement::NFStatement>, mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = statements;
    statements = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::FOR { .. } => cons(Arc::new(Statement::NFStatement::FOR { iterator: var_field!((*stmt).iterator, Statement::NFStatement::FOR).clone(), range: var_field!((*stmt).range, Statement::NFStatement::FOR).clone(), body: scalarizeStatements(var_field!((*stmt).body, Statement::NFStatement::FOR).clone())?, forType: var_field!((*stmt).forType, Statement::NFStatement::FOR).clone(), source: var_field!((*stmt).source, Statement::NFStatement::FOR).clone() }), statements.clone()),
        Deref @ Statement::IF { .. } => scalarizeIfStatement(var_field!((*stmt).branches, Statement::NFStatement::IF).clone(), var_field!((*stmt).source, Statement::NFStatement::IF).clone(), statements.clone())?,
        Deref @ Statement::WHEN { .. } => scalarizeWhenStatement(var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone(), var_field!((*stmt).source, Statement::NFStatement::WHEN).clone(), statements.clone())?,
        Deref @ Statement::WHILE { .. } => cons(Arc::new(Statement::NFStatement::WHILE { condition: var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), body: scalarizeStatements(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone())?, source: var_field!((*stmt).source, Statement::NFStatement::WHILE).clone() }), statements.clone()),
        _ => cons(stmt.clone(), statements.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(statements)
}

pub fn scalarizeIfStatement(mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>, mut source: Arc<DAE::ElementSource>, mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = statements;
    let mut bl: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    for mut b in &*branches.clone() {
        let mut b = b.clone();
        (cond, body) = b.clone();
        body = scalarizeStatements(body.clone())?;
        if !(body.clone().is_empty()) {
            bl = cons((cond.clone(), body.clone()), bl.clone());
        }
    }
    if !(bl.clone().is_empty()) {
        statements = cons(Arc::new(Statement::NFStatement::IF { branches: bl.clone().reverse(), source: source.clone() }), statements.clone());
    }
    Ok(statements)
}

pub fn scalarizeWhenStatement(mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>, mut source: Arc<DAE::ElementSource>, mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = statements;
    let mut bl: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    for mut b in &*branches.clone() {
        let mut b = b.clone();
        (cond, body) = b.clone();
        body = scalarizeStatements(body.clone())?;
        if Type::isArray(Expression::typeOf(cond.clone())) {
            (cond, _) = ExpandExp::expand(cond.clone(), false, false)?;
        }
        bl = cons((cond.clone(), body.clone()), bl.clone());
    }
    statements = cons(Arc::new(Statement::NFStatement::WHEN { branches: bl.clone().reverse(), source: source.clone() }), statements.clone());
    Ok(statements)
}

pub fn variableHasForcedScalarAttribute(mut var: Arc<Variable::NFVariable>) -> bool {
    let mut res: bool = false;
    for mut attribute in &*list![(literal!("min")).clone(), (literal!("max")).clone(), (literal!("nominal")).clone()] {
        let mut attribute = attribute.clone();
        if Binding::isBound(Variable::lookupTypeAttribute((attribute.clone()).clone(), var.clone())) {
            res = true;
            return res;
        }
    }
    res = false;
    res
}

