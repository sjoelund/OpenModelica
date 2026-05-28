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
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

pub fn convert(mut flatModel: Arc<FlatModel::NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>) -> Result<(DAE::DAElist, Arc<AvlTreePathFunction::Tree>)> {
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut daeFunctions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut elems: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut class_elem: Arc<DAE::Element>;
    daeFunctions = convertFunctionTree(functions.clone())?;
    dae = convertModel(flatModel.clone())?;
    execStat((literal!("NFConvertDAE.convert")).clone())?;
    Ok((dae, daeFunctions))
}

pub fn convertModel(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut elems: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut class_elem: Arc<DAE::Element>;
    elems = convertVariables(flatModel.variables.clone(), metamodelica::nil())?;
    elems = convertEquations(flatModel.equations.clone(), elems.clone())?;
    elems = convertInitialEquations(flatModel.initialEquations.clone(), elems.clone())?;
    elems = convertAlgorithms(flatModel.algorithms.clone(), elems.clone())?;
    elems = convertInitialAlgorithms(flatModel.initialAlgorithms.clone(), elems.clone())?;
    class_elem = Arc::new(DAE::Element::COMP { ident: (FlatModel::fullName(flatModel.clone())).clone(), dAElist: elems.clone(), source: flatModel.source.clone(), comment: ElementSource::getOptComment(flatModel.source.clone())? });
    dae = DAE::DAElist { elementLst: list![class_elem.clone()] };
    Ok(dae)
}

pub fn convertStatements(mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    elements = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for mut s in (statements.clone()).into_iter().cloned() {
            let __x = convertStatement(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(elements)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableConversionSettings {
    pub isFunctionParameter: bool,
    pub addTypeToSource: bool,
}

impl Default for VariableConversionSettings {
    fn default() -> Self {
        Self {
            isFunctionParameter: Default::default(),
            addTypeToSource: Default::default(),
        }
    }
}

pub type VARIABLE_CONVERSION_SETTINGS = VariableConversionSettings;


pub static FUNCTION_VARIABLE_CONVERSION_SETTINGS: VariableConversionSettings = VariableConversionSettings { isFunctionParameter: true, addTypeToSource: false };

fn convertVariables(mut variables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    let mut settings: VariableConversionSettings = <VariableConversionSettings as ::std::default::Default>::default();
    settings = VariableConversionSettings { addTypeToSource: Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?, isFunctionParameter: false };
    for mut var in &*variables.clone().reverse() {
        let mut var = var.clone();
        elements = cons(convertVariable(var.clone(), settings.clone())?, elements.clone());
    }
    Ok(elements)
}

fn convertVariable(mut var: Arc<Variable::NFVariable>, mut settings: VariableConversionSettings) -> Result<Arc<DAE::Element>> {
    let mut daeVar: Arc<DAE::Element>;
    let mut var_attr: Option<Arc<DAE::VariableAttributes>> = None;
    let mut binding_exp: Option<Arc<DAE::Exp>> = None;
    binding_exp = Binding::toDAEExp(var.binding.clone())?;
    var_attr = convertVarAttributes(var.typeAttributes.clone(), var.ty.clone(), var.attributes.clone())?;
    daeVar = makeDAEVar(var.name.clone(), var.ty.clone(), binding_exp.clone(), var.attributes.clone(), var.visibility.clone(), var_attr.clone(), var.comment.clone(), settings.clone(), var.info.clone(), Variable::isEncrypted(var.clone())?)?;
    Ok(daeVar)
}

fn makeDAEVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut binding: Option<Arc<DAE::Exp>>, mut attr: Arc<Attributes::NFAttributes>, mut vis: Visibility, mut vattr: Option<Arc<DAE::VariableAttributes>>, mut comment: Arc<SCode::Comment>, mut settings: VariableConversionSettings, mut info: SourceInfo, mut encrypted: bool) -> Result<Arc<DAE::Element>> {
    let mut var: Arc<DAE::Element>;
    let mut dcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut dty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut dir: Direction = Direction::NONE;
    dcref = ComponentRef::toDAE(cref.clone())?;
    dty = Type::toDAE(if (settings.isFunctionParameter.clone()) {Type::arrayElementType(ty.clone())} else {ty.clone()}, true)?;
    source = ElementSource::createElementSource(info.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?;
    if settings.addTypeToSource.clone() {
        source = addComponentTypeToSource(cref.clone(), source.clone())?;
    }
    var = (::match_deref::match_deref! { match &(attr.clone()) {
        Deref @ Attributes::ATTRIBUTES { .. } => Arc::new(DAE::Element::VAR { componentRef: dcref.clone(), kind: Prefixes::variabilityToDAE(attr.variability.clone()), direction: Prefixes::directionToDAE(attr.direction.clone()), parallelism: Prefixes::parallelismToDAE(attr.parallelism.clone())?, protection: Prefixes::visibilityToDAE(vis.clone()), ty: dty.clone(), binding: binding.clone(), dims: ComponentReferenceBasics::crefDims(dcref.clone())?, connectorType: Prefixes::ConnectorType::toDAE(attr.connectorType.clone()), source: source.clone(), variableAttributesOption: vattr.clone(), comment: Some(comment.clone()), innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: encrypted.clone() }),
        _ => Arc::new(DAE::Element::VAR { componentRef: dcref.clone(), kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE, direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR, parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, protection: Prefixes::visibilityToDAE(vis.clone()), ty: dty.clone(), binding: binding.clone(), dims: metamodelica::nil(), connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), source: source.clone(), variableAttributesOption: vattr.clone(), comment: Some(comment.clone()), innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: encrypted.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

fn addComponentTypeToSource(mut cref: Arc<ComponentRef::NFComponentRef>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    source = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } => {
            source = ElementSource::addElementSourceType(source.clone(), InstNode::scopePath(InstNode::classScope(InstNode::getDerivedNode(InstNode::parent(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone()), true)), InstNode::ScopeType::RELATIVE.clone(), false)?)?;
            addComponentTypeToSource(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), source.clone())?
        },
        _ => source.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

fn convertVarAttributes(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut ty: Arc<Type::NFType>, mut compAttrs: Arc<Attributes::NFAttributes>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut attributes: Option<Arc<DAE::VariableAttributes>> = None;
    let mut is_final: bool = false;
    let mut is_final_opt: Option<bool> = None;
    let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut is_array: bool = false;
    is_final = compAttrs.isFinal.clone() || compAttrs.variability.clone() == Variability::STRUCTURAL_PARAMETER.clone();
    if attrs.clone().is_empty() && !(is_final.clone()) {
        attributes = None;
        return Ok(attributes);
    }
    is_final_opt = Some(is_final.clone());
    attributes = (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::REAL => convertRealVarAttributes(attrs.clone(), is_final_opt.clone())?,
        Deref @ Type::INTEGER => convertIntVarAttributes(attrs.clone(), is_final_opt.clone())?,
        Deref @ Type::BOOLEAN => convertBoolVarAttributes(attrs.clone(), is_final_opt.clone())?,
        Deref @ Type::STRING => convertStringVarAttributes(attrs.clone(), is_final_opt.clone())?,
        Deref @ Type::ENUMERATION { .. } => convertEnumVarAttributes(attrs.clone(), is_final_opt.clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attributes)
}

fn convertRealVarAttributes(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: Option<bool>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut attributes: Option<Arc<DAE::VariableAttributes>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut quantity: Option<Arc<DAE::Exp>> = None;
    let mut unit: Option<Arc<DAE::Exp>> = None;
    let mut displayUnit: Option<Arc<DAE::Exp>> = None;
    let mut min: Option<Arc<DAE::Exp>> = None;
    let mut max: Option<Arc<DAE::Exp>> = None;
    let mut start: Option<Arc<DAE::Exp>> = None;
    let mut fixed: Option<Arc<DAE::Exp>> = None;
    let mut nominal: Option<Arc<DAE::Exp>> = None;
    let mut state_select: Option<DAE::StateSelect> = None;
    let mut uncertain: Option<DAE::Uncertainty> = None;
    let mut start_origin: Option<Arc<DAE::Exp>> = None;
    for mut attr in &*attrs.clone() {
        let mut attr = attr.clone();
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "displayUnit" => {
            displayUnit = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "fixed" => {
            fixed = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "max" => {
            max = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "min" => {
            min = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "nominal" => {
            nominal = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "quantity" => {
            quantity = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = convertVarAttribute(b.clone());
            start_origin = convertStartOrigin(b.clone());
            ()
        },
        Deref @ "stateSelect" => {
            state_select = convertStateSelectAttribute(b.clone())?;
            ()
        },
        Deref @ "unbounded" => (),
        Deref @ "uncertain" => {
            uncertain = convertUncertaintyAttribute(b.clone())?;
            ()
        },
        Deref @ "unit" => {
            unit = convertVarAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertRealVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    attributes = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), nominal: nominal.clone(), stateSelectOption: state_select.clone(), uncertainOption: uncertain.clone(), distributionOption: None, equationBound: None, isProtected: None, finalPrefix: isFinal.clone(), startOrigin: start_origin.clone() }));
    Ok(attributes)
}

fn convertIntVarAttributes(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: Option<bool>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut attributes: Option<Arc<DAE::VariableAttributes>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut quantity: Option<Arc<DAE::Exp>> = None;
    let mut min: Option<Arc<DAE::Exp>> = None;
    let mut max: Option<Arc<DAE::Exp>> = None;
    let mut start: Option<Arc<DAE::Exp>> = None;
    let mut fixed: Option<Arc<DAE::Exp>> = None;
    let mut start_origin: Option<Arc<DAE::Exp>> = None;
    for mut attr in &*attrs.clone() {
        let mut attr = attr.clone();
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "quantity" => {
            quantity = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "min" => {
            min = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "max" => {
            max = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = convertVarAttribute(b.clone());
            start_origin = convertStartOrigin(b.clone());
            ()
        },
        Deref @ "fixed" => {
            fixed = convertVarAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertIntVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    attributes = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: isFinal.clone(), startOrigin: start_origin.clone() }));
    Ok(attributes)
}

fn convertBoolVarAttributes(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: Option<bool>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut attributes: Option<Arc<DAE::VariableAttributes>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut quantity: Option<Arc<DAE::Exp>> = None;
    let mut start: Option<Arc<DAE::Exp>> = None;
    let mut fixed: Option<Arc<DAE::Exp>> = None;
    let mut start_origin: Option<Arc<DAE::Exp>> = None;
    for mut attr in &*attrs.clone() {
        let mut attr = attr.clone();
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "quantity" => {
            quantity = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = convertVarAttribute(b.clone());
            start_origin = convertStartOrigin(b.clone());
            ()
        },
        Deref @ "fixed" => {
            fixed = convertVarAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertBoolVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    attributes = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: None, isProtected: None, finalPrefix: isFinal.clone(), startOrigin: start_origin.clone() }));
    Ok(attributes)
}

fn convertStringVarAttributes(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: Option<bool>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut attributes: Option<Arc<DAE::VariableAttributes>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut quantity: Option<Arc<DAE::Exp>> = None;
    let mut start: Option<Arc<DAE::Exp>> = None;
    let mut fixed: Option<Arc<DAE::Exp>> = None;
    let mut start_origin: Option<Arc<DAE::Exp>> = None;
    for mut attr in &*attrs.clone() {
        let mut attr = attr.clone();
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "quantity" => {
            quantity = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = convertVarAttribute(b.clone());
            start_origin = convertStartOrigin(b.clone());
            ()
        },
        Deref @ "fixed" => {
            fixed = convertVarAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertStringVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    attributes = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: None, isProtected: None, finalPrefix: isFinal.clone(), startOrigin: start_origin.clone() }));
    Ok(attributes)
}

fn convertEnumVarAttributes(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: Option<bool>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut attributes: Option<Arc<DAE::VariableAttributes>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut quantity: Option<Arc<DAE::Exp>> = None;
    let mut min: Option<Arc<DAE::Exp>> = None;
    let mut max: Option<Arc<DAE::Exp>> = None;
    let mut start: Option<Arc<DAE::Exp>> = None;
    let mut fixed: Option<Arc<DAE::Exp>> = None;
    let mut start_origin: Option<Arc<DAE::Exp>> = None;
    for mut attr in &*attrs.clone() {
        let mut attr = attr.clone();
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "fixed" => {
            fixed = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "max" => {
            max = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "min" => {
            min = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "quantity" => {
            quantity = convertVarAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = convertVarAttribute(b.clone());
            start_origin = convertStartOrigin(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertEnumVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    attributes = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: None, isProtected: None, finalPrefix: isFinal.clone(), startOrigin: start_origin.clone() }));
    Ok(attributes)
}

fn convertVarAttribute(mut binding: Arc<Binding::NFBinding>) -> Option<Arc<DAE::Exp>> {
    let mut attribute: Option<Arc<DAE::Exp>> = Some(Expression::toDAE(Binding::getTypedExp(binding.clone()).unwrap(), false).unwrap());
    attribute
}

fn convertStateSelectAttribute(mut binding: Arc<Binding::NFBinding>) -> Result<Option<DAE::StateSelect>> {
    let mut stateSelect: Option<DAE::StateSelect> = None;
    let mut name: ArcStr = arcstr::literal!("");
    name = (getStateSelectName(Expression::arrayFirstScalar(Binding::getTypedExp(binding.clone())?)?)?).clone();
    stateSelect = Some(lookupStateSelectMember((name.clone()).clone())?);
    Ok(stateSelect)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getStateSelectName(mut exp: Arc<Expression::NFExpression>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    name = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ENUM_LITERAL { .. } => var_field!((*exp).name, Expression::NFExpression::ENUM_LITERAL).clone(),
        Deref @ Expression::CREF { .. } => InstNode::name(ComponentRef::node(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?)?,
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: e, .. } } => getStateSelectName(e.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.getStateSelectName")); __mm_s.push_str(&*literal!(" got invalid StateSelect expression ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(name)
}

fn lookupStateSelectMember(mut name: ArcStr) -> Result<DAE::StateSelect> {
    let mut stateSelect: DAE::StateSelect = DAE::StateSelect::ALWAYS;
    stateSelect = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "never" => openmodelica_frontend_types::DAE::StateSelect::NEVER,
        Deref @ "avoid" => openmodelica_frontend_types::DAE::StateSelect::AVOID,
        Deref @ "default" => openmodelica_frontend_types::DAE::StateSelect::DEFAULT,
        Deref @ "prefer" => openmodelica_frontend_types::DAE::StateSelect::PREFER,
        Deref @ "always" => openmodelica_frontend_types::DAE::StateSelect::ALWAYS,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.lookupStateSelectMember")); __mm_s.push_str(&*literal!(" got unknown StateSelect literal ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stateSelect)
}

fn convertUncertaintyAttribute(mut binding: Arc<Binding::NFBinding>) -> Result<Option<DAE::Uncertainty>> {
    let mut stateSelect: Option<DAE::Uncertainty> = None;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut name: ArcStr = arcstr::literal!("");
    let mut exp: Arc<Expression::NFExpression> = Expression::arrayFirstScalar(Binding::getTypedExp(binding.clone())?)?;
    name = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ENUM_LITERAL { .. } => var_field!((*exp).name, Expression::NFExpression::ENUM_LITERAL).clone(),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { node, .. }, .. } => InstNode::name(node.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertUncertaintyAttribute")); __mm_s.push_str(&*literal!(" got invalid Uncertainty expression ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    stateSelect = Some(lookupUncertaintyMember((name.clone()).clone())?);
    Ok(stateSelect)
}

fn lookupUncertaintyMember(mut name: ArcStr) -> Result<DAE::Uncertainty> {
    let mut stateSelect: DAE::Uncertainty = DAE::Uncertainty::GIVEN;
    stateSelect = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "given" => openmodelica_frontend_types::DAE::Uncertainty::GIVEN,
        Deref @ "sought" => openmodelica_frontend_types::DAE::Uncertainty::SOUGHT,
        Deref @ "refine" => openmodelica_frontend_types::DAE::Uncertainty::REFINE,
        Deref @ "propagate" => openmodelica_frontend_types::DAE::Uncertainty::PROPAGATE,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.lookupUncertaintyMember")); __mm_s.push_str(&*literal!(" got unknown Uncertainty literal ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stateSelect)
}

fn convertStartOrigin(mut binding: Arc<Binding::NFBinding>) -> Option<Arc<DAE::Exp>> {
    let mut startOrigin: Option<Arc<DAE::Exp>> = Some(Arc::new(DAE::Exp::SCONST { string: (if (Binding::source(binding.clone()) == Binding::Source::TYPE.clone()) {literal!("binding")} else {literal!("type")}).clone() }));
    startOrigin
}

fn convertEquations(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    for mut eq in &*equations.clone().reverse() {
        let mut eq = eq.clone();
        elements = convertEquation(eq.clone(), elements.clone())?;
    }
    Ok(elements)
}

fn convertEquation(mut eq: Arc<Equation::NFEquation>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    elements = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { rhs: rhs @ Deref @ Expression::CREF { .. }, lhs: lhs @ Deref @ Expression::CREF { .. }, .. } if (Type::isScalarBuiltin(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone())) => {
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cr1 = ComponentRef::toDAE(var_field!((**lhs).cref, Expression::NFExpression::CREF).clone())?;
            cr2 = ComponentRef::toDAE(var_field!((**rhs).cref, Expression::NFExpression::CREF).clone())?;
            cons(Arc::new(DAE::Element::EQUEQUATION { cr1: cr1.clone(), cr2: cr2.clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone() }), elements.clone())
        },
        Deref @ Equation::EQUALITY { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            e1 = Expression::toDAE(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), false)?;
            e2 = Expression::toDAE(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), false)?;
            cons(if (Type::isComplex(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone())) {Arc::new(DAE::Element::COMPLEX_EQUATION { lhs: e1.clone(), rhs: e2.clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone() })} else if (Type::isArray(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone())) {Arc::new(DAE::Element::ARRAY_EQUATION { dimension: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        for mut d in (Type::arrayDims(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone())).into_iter().cloned() {
            let __x = Dimension::toDAE(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, exp: e1.clone(), array: e2.clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone() })} else {Arc::new(DAE::Element::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone() })}, elements.clone())
        },
        Deref @ Equation::FOR { .. } => {
            cons(convertForEquation(eq.clone(), false)?, elements.clone())
        },
        Deref @ Equation::IF { .. } => {
            cons(convertIfEquation(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), var_field!((*eq).source, Equation::NFEquation::IF).clone(), false)?, elements.clone())
        },
        Deref @ Equation::WHEN { .. } => {
            cons(convertWhenEquation(var_field!((*eq).branches, Equation::NFEquation::WHEN).clone(), var_field!((*eq).source, Equation::NFEquation::WHEN).clone())?, elements.clone())
        },
        Deref @ Equation::ASSERT { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            e1 = Expression::toDAE(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), false)?;
            e2 = Expression::toDAE(var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), false)?;
            e3 = Expression::toDAE(var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), false)?;
            cons(Arc::new(DAE::Element::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), source: var_field!((*eq).source, Equation::NFEquation::ASSERT).clone() }), elements.clone())
        },
        Deref @ Equation::TERMINATE { .. } => {
            cons(Arc::new(DAE::Element::TERMINATE { message: Expression::toDAE(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), false)?, source: var_field!((*eq).source, Equation::NFEquation::TERMINATE).clone() }), elements.clone())
        },
        Deref @ Equation::REINIT { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cr1 = ComponentRef::toDAE(Expression::toCref(var_field!((*eq).cref, Equation::NFEquation::REINIT).clone())?)?;
            e1 = Expression::toDAE(var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), false)?;
            cons(Arc::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e1.clone(), source: var_field!((*eq).source, Equation::NFEquation::REINIT).clone() }), elements.clone())
        },
        Deref @ Equation::NORETCALL { .. } => {
            cons(Arc::new(DAE::Element::NORETCALL { exp: Expression::toDAE(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), false)?, source: var_field!((*eq).source, Equation::NFEquation::NORETCALL).clone() }), elements.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertEquation")); __mm_s.push_str(&*literal!(" got unknown equation ")); __mm_s.push_str(&*Equation::toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

fn convertForEquation(mut forEquation: Arc<Equation::NFEquation>, mut isInitial: bool) -> Result<Arc<DAE::Element>> {
    let mut forDAE: Arc<DAE::Element>;
    let mut iterator: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut dbody: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(forEquation.clone()) {
        Deref @ Equation::FOR { source: __pa0, body: __pa1, range: Some(__pa2), iterator: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    source = __pa0.clone();
    body = __pa1.clone();
    range = __pa2.clone();
    iterator = __pa3.clone();
    if isInitial.clone() {
        dbody = convertInitialEquations(body.clone(), metamodelica::nil())?;
    } else {
        dbody = convertEquations(body.clone(), metamodelica::nil())?;
    }
    let __pa4 = ::match_deref::match_deref! { match &(InstNode::component(iterator.clone())?) {
        Deref @ Component::ITERATOR { ty: __pa4, .. } => __pa4.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa4.clone();
    if isInitial.clone() {
        forDAE = Arc::new(DAE::Element::INITIAL_FOR_EQUATION { type_: Type::toDAE(ty.clone(), true)?, iterIsArray: Type::isArray(ty.clone()), iter: (InstNode::name(iterator.clone())?).clone(), index: 0, range: Expression::toDAE(range.clone(), false)?, equations: dbody.clone(), source: source.clone() });
    } else {
        forDAE = Arc::new(DAE::Element::FOR_EQUATION { type_: Type::toDAE(ty.clone(), true)?, iterIsArray: Type::isArray(ty.clone()), iter: (InstNode::name(iterator.clone())?).clone(), index: 0, range: Expression::toDAE(range.clone(), false)?, equations: dbody.clone(), source: source.clone() });
    }
    Ok(forDAE)
}

fn convertIfEquation(mut ifBranches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut source: Arc<DAE::ElementSource>, mut isInitial: bool) -> Result<Arc<DAE::Element>> {
    let mut ifEquation: Arc<DAE::Element>;
    let mut conds: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Equation::NFEquation>>>>> = metamodelica::nil();
    let mut dconds: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut dbranches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
    let mut else_branch: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    for mut branch in &*ifBranches.clone() {
        let mut branch = branch.clone();
        (conds, branches) = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => (cons(var_field!((*branch).condition, Equation::Branch::Branch::BRANCH).clone(), conds.clone()), cons(var_field!((*branch).body, Equation::Branch::Branch::BRANCH).clone(), branches.clone())),
        Deref @ Equation::Branch::INVALID_BRANCH { .. } => {
            Equation::Branch::triggerErrors(branch.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    dbranches = if (isInitial.clone()) {{
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
        for mut b in (branches.clone()).into_iter().cloned() {
            let __x = convertInitialEquations(b.clone(), metamodelica::nil())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }} else {{
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
        for mut b in (branches.clone()).into_iter().cloned() {
            let __x = convertEquations(b.clone(), metamodelica::nil())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }};
    if Expression::isTrue(listHead(conds.clone())?) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dbranches.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        else_branch = __pa0.clone();
        dbranches = __pa1.clone();
        conds = listRest(conds.clone())?;
    } else {
        else_branch = metamodelica::nil();
    }
    dconds = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (conds.clone()).into_iter().cloned() {
            let __x = Expression::toDAE(c.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc
    };
    dbranches = dbranches.clone().reverse();
    ifEquation = if (isInitial.clone()) {Arc::new(DAE::Element::INITIAL_IF_EQUATION { condition1: dconds.clone(), equations2: dbranches.clone(), equations3: else_branch.clone(), source: source.clone() })} else {Arc::new(DAE::Element::IF_EQUATION { condition1: dconds.clone(), equations2: dbranches.clone(), equations3: else_branch.clone(), source: source.clone() })};
    Ok(ifEquation)
}

fn convertWhenEquation(mut whenBranches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Element>> {
    let mut whenEquation: Arc<DAE::Element>;
    let mut cond: Arc<DAE::Exp>;
    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut when_eq: Option<Arc<DAE::Element>> = None;
    for mut b in &*whenBranches.clone().reverse() {
        let mut b = b.clone();
        when_eq = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => {
            cond = Expression::toDAE(var_field!((*b).condition, Equation::Branch::Branch::BRANCH).clone(), false)?;
            els = convertEquations(var_field!((*b).body, Equation::Branch::Branch::BRANCH).clone(), metamodelica::nil())?;
            Some(Arc::new(DAE::Element::WHEN_EQUATION { condition: cond.clone(), equations: els.clone(), elsewhen_: when_eq.clone(), source: source.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    let __pa0 = ::match_deref::match_deref! { match &(when_eq.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    whenEquation = __pa0.clone();
    Ok(whenEquation)
}

fn convertInitialEquations(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    for mut eq in &*equations.clone().reverse() {
        let mut eq = eq.clone();
        elements = convertInitialEquation(eq.clone(), elements.clone())?;
    }
    Ok(elements)
}

fn convertInitialEquation(mut eq: Arc<Equation::NFEquation>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    elements = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            e1 = Expression::toDAE(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), false)?;
            e2 = Expression::toDAE(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), false)?;
            cons(if (Type::isComplex(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone())) {Arc::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1.clone(), rhs: e2.clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone() })} else if (Type::isArray(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone())) {Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        for mut d in (Type::arrayDims(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone())).into_iter().cloned() {
            let __x = Dimension::toDAE(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, exp: e1.clone(), array: e2.clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone() })} else {Arc::new(DAE::Element::INITIALEQUATION { exp1: e1.clone(), exp2: e2.clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone() })}, elements.clone())
        },
        Deref @ Equation::FOR { .. } => {
            cons(convertForEquation(eq.clone(), true)?, elements.clone())
        },
        Deref @ Equation::IF { .. } => {
            cons(convertIfEquation(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), var_field!((*eq).source, Equation::NFEquation::IF).clone(), true)?, elements.clone())
        },
        Deref @ Equation::ASSERT { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            e1 = Expression::toDAE(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), false)?;
            e2 = Expression::toDAE(var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), false)?;
            e3 = Expression::toDAE(var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), false)?;
            cons(Arc::new(DAE::Element::INITIAL_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), source: var_field!((*eq).source, Equation::NFEquation::ASSERT).clone() }), elements.clone())
        },
        Deref @ Equation::TERMINATE { .. } => {
            cons(Arc::new(DAE::Element::INITIAL_TERMINATE { message: Expression::toDAE(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), false)?, source: var_field!((*eq).source, Equation::NFEquation::TERMINATE).clone() }), elements.clone())
        },
        Deref @ Equation::NORETCALL { .. } => {
            cons(Arc::new(DAE::Element::INITIAL_NORETCALL { exp: Expression::toDAE(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), false)?, source: var_field!((*eq).source, Equation::NFEquation::NORETCALL).clone() }), elements.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertInitialEquation")); __mm_s.push_str(&*literal!(" got unknown equation ")); __mm_s.push_str(&*Equation::toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

fn convertAlgorithms(mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    for mut alg in &*algorithms.clone().reverse() {
        let mut alg = alg.clone();
        elements = convertAlgorithm(alg.clone(), elements.clone())?;
    }
    Ok(elements)
}

fn convertAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut dalg: Arc<DAE::Algorithm> = Arc::new(<DAE::Algorithm as ::std::default::Default>::default());
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    stmts = convertStatements(alg.statements.clone())?;
    dalg = Arc::new(DAE::Algorithm { statementLst: stmts.clone() });
    elements = cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: dalg.clone(), source: alg.source.clone() }), elements.clone());
    Ok(elements)
}

fn convertStatement(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<DAE::Statement>> {
    let mut elem: Arc<DAE::Statement>;
    elem = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            convertAssignment(stmt.clone())?
        },
        Deref @ Statement::FUNCTION_ARRAY_INIT { .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ty = Type::toDAE(var_field!((*stmt).ty, Statement::NFStatement::FUNCTION_ARRAY_INIT).clone(), true)?;
            Arc::new(DAE::Statement::STMT_ARRAY_INIT { name: (var_field!((*stmt).name, Statement::NFStatement::FUNCTION_ARRAY_INIT).clone()).clone(), ty: ty.clone(), source: var_field!((*stmt).source, Statement::NFStatement::FUNCTION_ARRAY_INIT).clone() })
        },
        Deref @ Statement::FOR { .. } => {
            convertForStatement(stmt.clone())?
        },
        Deref @ Statement::IF { .. } => {
            convertIfStatement(var_field!((*stmt).branches, Statement::NFStatement::IF).clone(), var_field!((*stmt).source, Statement::NFStatement::IF).clone())?
        },
        Deref @ Statement::WHEN { .. } => {
            convertWhenStatement(var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone(), var_field!((*stmt).source, Statement::NFStatement::WHEN).clone())?
        },
        Deref @ Statement::ASSERT { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            e1 = Expression::toDAE(var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), false)?;
            e2 = Expression::toDAE(var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), false)?;
            e3 = Expression::toDAE(var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), false)?;
            Arc::new(DAE::Statement::STMT_ASSERT { cond: e1.clone(), msg: e2.clone(), level: e3.clone(), source: var_field!((*stmt).source, Statement::NFStatement::ASSERT).clone() })
        },
        Deref @ Statement::TERMINATE { .. } => {
            Arc::new(DAE::Statement::STMT_TERMINATE { msg: Expression::toDAE(var_field!((*stmt).message, Statement::NFStatement::TERMINATE).clone(), false)?, source: var_field!((*stmt).source, Statement::NFStatement::TERMINATE).clone() })
        },
        Deref @ Statement::REINIT { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            e1 = Expression::toDAE(var_field!((*stmt).cref, Statement::NFStatement::REINIT).clone(), false)?;
            e2 = Expression::toDAE(var_field!((*stmt).reinitExp, Statement::NFStatement::REINIT).clone(), false)?;
            Arc::new(DAE::Statement::STMT_REINIT { var: e1.clone(), value: e2.clone(), source: var_field!((*stmt).source, Statement::NFStatement::REINIT).clone() })
        },
        Deref @ Statement::NORETCALL { .. } => {
            Arc::new(DAE::Statement::STMT_NORETCALL { exp: Expression::toDAE(var_field!((*stmt).exp, Statement::NFStatement::NORETCALL).clone(), false)?, source: var_field!((*stmt).source, Statement::NFStatement::NORETCALL).clone() })
        },
        Deref @ Statement::WHILE { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut body: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            e1 = Expression::toDAE(var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), false)?;
            body = convertStatements(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone())?;
            Arc::new(DAE::Statement::STMT_WHILE { exp: e1.clone(), statementLst: body.clone(), source: var_field!((*stmt).source, Statement::NFStatement::WHILE).clone() })
        },
        Deref @ Statement::RETURN { .. } => {
            Arc::new(DAE::Statement::STMT_RETURN { source: var_field!((*stmt).source, Statement::NFStatement::RETURN).clone() })
        },
        Deref @ Statement::BREAK { .. } => {
            Arc::new(DAE::Statement::STMT_BREAK { source: var_field!((*stmt).source, Statement::NFStatement::BREAK).clone() })
        },
        Deref @ Statement::FAILURE { .. } => {
            Arc::new(DAE::Statement::STMT_FAILURE { body: convertStatements(var_field!((*stmt).body, Statement::NFStatement::FAILURE).clone())?, source: var_field!((*stmt).source, Statement::NFStatement::FAILURE).clone() })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertStatement")); __mm_s.push_str(&*literal!(" got unknown statement ")); __mm_s.push_str(&*Statement::toString(stmt.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elem)
}

fn convertAssignment(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<DAE::Statement>> {
    let mut daeStmt: Arc<DAE::Statement>;
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dlhs: Arc<DAE::Exp>;
    let mut drhs: Arc<DAE::Exp>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { lhs: __pa0, rhs: __pa1, ty: __pa2, source: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhs = __pa0.clone();
    rhs = __pa1.clone();
    ty = __pa2.clone();
    src = __pa3.clone();
    if Type::isTuple(ty.clone()) {
        let __pa4 = ::match_deref::match_deref! { match &(lhs.clone()) {
            Deref @ Expression::TUPLE { elements: __pa4, .. } => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        expl = __pa4.clone();
        daeStmt = (::match_deref::match_deref! { match &(expl.clone()) {
        Deref @ metamodelica::List::Nil => Arc::new(DAE::Statement::STMT_NORETCALL { exp: Expression::toDAE(rhs.clone(), false)?, source: src.clone() }),
        Deref @ metamodelica::List::Cons { head: lhs, tail: Deref @ metamodelica::List::Nil } => {
            dty = Type::toDAE(ty.clone(), true)?;
            dlhs = Expression::toDAE(lhs.clone(), false)?;
            drhs = Arc::new(DAE::Exp::TSUB { exp: Expression::toDAE(rhs.clone(), false)?, ix: 1, ty: dty.clone() });
            if Type::isArray(ty.clone()) {
                daeStmt = Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: dty.clone(), lhs: dlhs.clone(), exp: drhs.clone(), source: src.clone() });
            } else {
                daeStmt = Arc::new(DAE::Statement::STMT_ASSIGN { type_: dty.clone(), exp1: dlhs.clone(), exp: drhs.clone(), source: src.clone() });
            }
            daeStmt.clone()
        },
        _ => {
            dty = Type::toDAE(ty.clone(), true)?;
            drhs = Expression::toDAE(rhs.clone(), false)?;
            Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: dty.clone(), expExpLst: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = Expression::toDAE(e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, exp: drhs.clone(), source: src.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        dty = Type::toDAE(ty.clone(), true)?;
        dlhs = Expression::toDAE(lhs.clone(), false)?;
        drhs = Expression::toDAE(rhs.clone(), false)?;
        if Type::isArray(ty.clone()) {
            daeStmt = Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: dty.clone(), lhs: dlhs.clone(), exp: drhs.clone(), source: src.clone() });
        } else {
            daeStmt = Arc::new(DAE::Statement::STMT_ASSIGN { type_: dty.clone(), exp1: dlhs.clone(), exp: drhs.clone(), source: src.clone() });
        }
    }
    Ok(daeStmt)
}

fn convertForStatement(mut forStmt: Arc<Statement::NFStatement>) -> Result<Arc<DAE::Statement>> {
    let mut forDAE: Arc<DAE::Statement>;
    let mut iterator: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut dbody: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut for_type: Arc<Statement::ForType> = Arc::new(Statement::ForType::NORMAL);
    let mut loop_vars: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(forStmt.clone()) {
        Deref @ Statement::FOR { source: __pa0, forType: __pa1, body: __pa2, range: Some(__pa3), iterator: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    source = __pa0.clone();
    for_type = __pa1.clone();
    body = __pa2.clone();
    range = __pa3.clone();
    iterator = __pa4.clone();
    dbody = convertStatements(body.clone())?;
    ty = InstNode::getType(iterator.clone())?;
    forDAE = (::match_deref::match_deref! { match &(for_type.clone()) {
        Deref @ Statement::ForType::NORMAL => Arc::new(DAE::Statement::STMT_FOR { type_: Type::toDAE(ty.clone(), true)?, iterIsArray: Type::isArray(ty.clone()), iter: (InstNode::name(iterator.clone())?).clone(), range: Expression::toDAE(range.clone(), false)?, statementLst: dbody.clone(), source: source.clone() }),
        Deref @ Statement::ForType::PARALLEL { .. } => {
            loop_vars = {
        let mut __acc: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
        for mut v in (var_field!((*for_type).vars, Statement::ForType::PARALLEL).clone()).into_iter().cloned() {
            let __x = convertForStatementParallelVar(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(DAE::Statement::STMT_PARFOR { type_: Type::toDAE(ty.clone(), true)?, iterIsArray: Type::isArray(ty.clone()), iter: (InstNode::name(iterator.clone())?).clone(), range: Expression::toDAE(range.clone(), false)?, statementLst: dbody.clone(), loopPrlVars: loop_vars.clone(), source: source.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(forDAE)
}

fn convertForStatementParallelVar(mut var: (Arc<ComponentRef::NFComponentRef>, SourceInfo)) -> Result<(Arc<DAE::ComponentRef>, SourceInfo)> {
    let mut outVar: (Arc<DAE::ComponentRef>, SourceInfo);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut dcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    (cref, info) = var.clone();
    dcref = ComponentRef::toDAE(cref.clone())?;
    outVar = (dcref.clone(), info.clone());
    Ok(outVar)
}

fn convertIfStatement(mut ifBranches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut ifStatement: Arc<DAE::Statement>;
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut dcond: Arc<DAE::Exp>;
    let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    let mut dstmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut first: bool = true;
    let mut single: bool = (ifBranches.clone().len() as i32) == 1;
    let mut else_stmt: Arc<DAE::Else> = Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE);
    for mut b in &*ifBranches.clone().reverse() {
        let mut b = b.clone();
        (cond, stmts) = b.clone();
        dcond = Expression::toDAE(cond.clone(), false)?;
        dstmts = convertStatements(stmts.clone())?;
        if first.clone() && !(single.clone()) && Expression::isTrue(cond.clone()) {
            else_stmt = Arc::new(DAE::Else::ELSE { statementLst: dstmts.clone() });
        } else {
            else_stmt = Arc::new(DAE::Else::ELSEIF { exp: dcond.clone(), statementLst: dstmts.clone(), else_: else_stmt.clone() });
        }
        first = false;
    }
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(else_stmt.clone()) {
        Deref @ DAE::Else::ELSEIF { exp: __pa0, statementLst: __pa1, else_: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dcond = __pa0.clone();
    dstmts = __pa1.clone();
    else_stmt = __pa2.clone();
    ifStatement = Arc::new(DAE::Statement::STMT_IF { exp: dcond.clone(), statementLst: dstmts.clone(), else_: else_stmt.clone(), source: source.clone() });
    Ok(ifStatement)
}

fn convertWhenStatement(mut whenBranches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut whenStatement: Arc<DAE::Statement>;
    let mut co: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut conditions: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cond: Arc<DAE::Exp>;
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut when_stmt: Option<Arc<DAE::Statement>> = None;
    for mut b in &*whenBranches.clone().reverse() {
        let mut b = b.clone();
        co = Util::tuple21(b.clone());
        conditions = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut c in (UnorderedSet::toList(Expression::extractCrefs(co.clone()))).into_iter().cloned() {
            if !(Type::isBoolean(ComponentRef::getSubscriptedType(c.clone(), false)?)) { continue; }
            let __x = c.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        cond = Expression::toDAE(co.clone(), false)?;
        stmts = convertStatements(Util::tuple22(b.clone()))?;
        when_stmt = Some(Arc::new(DAE::Statement::STMT_WHEN { exp: cond.clone(), conditions: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut c in (conditions.clone()).into_iter().cloned() {
            let __x = ComponentRef::toDAE(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, initialCall: false, statementLst: stmts.clone(), elseWhen: when_stmt.clone(), source: source.clone() }));
    }
    let __pa0 = ::match_deref::match_deref! { match &(when_stmt.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    whenStatement = __pa0.clone();
    Ok(whenStatement)
}

fn convertInitialAlgorithms(mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    for mut alg in &*algorithms.clone().reverse() {
        let mut alg = alg.clone();
        elements = convertInitialAlgorithm(alg.clone(), elements.clone())?;
    }
    Ok(elements)
}

fn convertInitialAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut dalg: Arc<DAE::Algorithm> = Arc::new(<DAE::Algorithm as ::std::default::Default>::default());
    stmts = convertStatements(alg.statements.clone())?;
    dalg = Arc::new(DAE::Algorithm { statementLst: stmts.clone() });
    elements = cons(Arc::new(DAE::Element::INITIALALGORITHM { algorithm_: dalg.clone(), source: alg.source.clone() }), elements.clone());
    Ok(elements)
}

pub fn convertFunctionTree(mut funcs: Arc<Flatten::FunctionTreeImpl::Tree>) -> Result<Arc<AvlTreePathFunction::Tree>> {
    let mut dfuncs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    dfuncs = (::match_deref::match_deref! { match &(funcs.clone()) {
        Deref @ Flatten::FunctionTreeImpl::Tree::NODE { .. } => {
            let mut left: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut right: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut r#fn: DAE::Function;
            r#fn = convertFunction(var_field!((*funcs).value, Flatten::FunctionTreeImpl::Tree::NODE).clone())?;
            left = convertFunctionTree(var_field!((*funcs).left, Flatten::FunctionTreeImpl::Tree::NODE).clone())?;
            right = convertFunctionTree(var_field!((*funcs).right, Flatten::FunctionTreeImpl::Tree::NODE).clone())?;
            Arc::new(AvlTreePathFunction::Tree::NODE { key: var_field!((*funcs).key, Flatten::FunctionTreeImpl::Tree::NODE).clone(), value: Some(r#fn.clone()), height: var_field!((*funcs).height, Flatten::FunctionTreeImpl::Tree::NODE).clone(), left: left.clone(), right: right.clone() })
        },
        Deref @ Flatten::FunctionTreeImpl::Tree::LEAF { .. } => {
            let mut r#fn: DAE::Function;
            r#fn = convertFunction(var_field!((*funcs).value, Flatten::FunctionTreeImpl::Tree::LEAF).clone())?;
            Arc::new(AvlTreePathFunction::Tree::LEAF { key: var_field!((*funcs).key, Flatten::FunctionTreeImpl::Tree::LEAF).clone(), value: Some(r#fn.clone()) })
        },
        Deref @ Flatten::FunctionTreeImpl::Tree::EMPTY => {
            Arc::new(openmodelica_frontend_dump::AvlTreePathFunction::Tree::EMPTY)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(dfuncs)
}

fn convertFunction(mut func: Arc<Function::Function>) -> Result<DAE::Function> {
    let mut dfunc: DAE::Function;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut elems: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut def: DAE::FunctionDefinition;
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    cls = InstNode::getClass(Function::instance(func.clone()))?;
    dfunc = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::TYPED_DERIVED { restriction: Deref @ Restriction::FUNCTION, .. } if (Function::isPartialDerivative(func.clone())) => {
            def = DAE::FunctionDefinition::FUNCTION_PARTIAL_DERIVATIVE { derivedFunction: Function::getDerivedFunctionName(func.clone()), derivedVars: Function::getDerivedInputNames(func.clone())? };
            Function::toDAE(func.clone(), def.clone())?
        },
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::FUNCTION, sections, .. } => {
            elems = convertFunctionParams(func.inputs.clone(), metamodelica::nil())?;
            elems = convertFunctionParams(func.outputs.clone(), elems.clone())?;
            elems = convertFunctionParams(func.locals.clone(), elems.clone())?;
            def = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { .. } => {
            elems = convertAlgorithms(var_field!((**sections).algorithms, Sections::NFSections::SECTIONS).clone(), elems.clone())?;
            DAE::FunctionDefinition::FUNCTION_DEF { body: elems.clone().reverse() }
        },
        Deref @ Sections::EXTERNAL { .. } => convertExternalDecl(sections.clone(), elems.clone().reverse())?,
        _ => DAE::FunctionDefinition::FUNCTION_DEF { body: elems.clone().reverse() },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Function::toDAE(func.clone(), def.clone())?
        },
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::RECORD_CONSTRUCTOR, .. } => DAE::Function::RECORD_CONSTRUCTOR { path: Function::name(func.clone()), type_: Function::makeDAEType(func.clone(), false)?, source: DAE::emptyElementSource().clone() },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertFunction")); __mm_s.push_str(&*literal!(" got unknown function")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dfunc)
}

fn convertFunctionParams(mut params: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    for mut p in &*params.clone() {
        let mut p = p.clone();
        elements = cons(convertFunctionParam(p.clone())?, elements.clone());
    }
    Ok(elements)
}

fn convertFunctionParam(mut node: Arc<InstNode::InstNode>) -> Result<Arc<DAE::Element>> {
    let mut element: Arc<DAE::Element>;
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut var_attr: Option<Arc<DAE::VariableAttributes>> = None;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut binding: Option<Arc<DAE::Exp>> = None;
    let mut ty_attr: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
    comp = InstNode::component(node.clone())?;
    element = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT { attributes: attr, info, ty, .. } => {
            cref = ComponentRef::fromNode(node.clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone());
            binding = Binding::toDAEExp(var_field!((*comp).binding, Component::NFComponent::COMPONENT).clone())?;
            cls = InstNode::getClass(var_field!((*comp).classInst, Component::NFComponent::COMPONENT).clone())?;
            ty_attr = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut m in (Class::getTypeAttributes(cls.clone())).into_iter().cloned() {
            let __x = (Modifier::name(m.clone())?, Modifier::binding(m.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            var_attr = convertVarAttributes(ty_attr.clone(), ty.clone(), attr.clone())?;
            makeDAEVar(cref.clone(), ty.clone(), binding.clone(), attr.clone(), InstNode::visibility(node.clone()), var_attr.clone(), var_field!((*comp).comment, Component::NFComponent::COMPONENT).clone(), FUNCTION_VARIABLE_CONVERSION_SETTINGS.clone(), info.clone(), false)?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertFunctionParam")); __mm_s.push_str(&*literal!(" got invalid component.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

fn convertExternalDecl(mut extDecl: Arc<Sections::NFSections>, mut parameters: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<DAE::FunctionDefinition> {
    let mut funcDef: DAE::FunctionDefinition;
    let mut decl: DAE::ExternalDecl = <DAE::ExternalDecl as ::std::default::Default>::default();
    let mut args: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
    let mut ret_arg: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
    funcDef = (::match_deref::match_deref! { match &(extDecl.clone()) {
        Deref @ Sections::EXTERNAL { .. } => {
            args = {
        let mut __acc: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
        for mut e in (var_field!((*extDecl).args, Sections::NFSections::EXTERNAL).clone()).into_iter().cloned() {
            let __x = convertExternalDeclArg(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ret_arg = convertExternalDeclOutput(var_field!((*extDecl).outputRef, Sections::NFSections::EXTERNAL).clone())?;
            decl = DAE::ExternalDecl { name: (var_field!((*extDecl).name, Sections::NFSections::EXTERNAL).clone()).clone(), args: args.clone(), returnArg: ret_arg.clone(), language: (var_field!((*extDecl).language, Sections::NFSections::EXTERNAL).clone()).clone(), ann: var_field!((*extDecl).ann, Sections::NFSections::EXTERNAL).clone() };
            DAE::FunctionDefinition::FUNCTION_EXT { body: parameters.clone(), externalDecl: decl.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(funcDef)
}

fn convertExternalDeclArg(mut exp: Arc<Expression::NFExpression>) -> Result<DAE::ExtArg> {
    let mut arg: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
    arg = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: cref @ Deref @ ComponentRef::CREF { .. }, .. } => {
            let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
            dir = Prefixes::directionToAbsyn(Component::direction(InstNode::component(var_field!((**cref).node, ComponentRef::NFComponentRef::CREF).clone())?));
            DAE::ExtArg::EXTARG { componentRef: ComponentRef::toDAE(cref.clone())?, direction: dir.clone(), type_: Type::toDAE(var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), true)? }
        },
        Deref @ Expression::SIZE { dimIndex: Some(e), exp: Deref @ Expression::CREF { cref: cref @ Deref @ ComponentRef::CREF { .. }, .. } } => {
            DAE::ExtArg::EXTARGSIZE { componentRef: ComponentRef::toDAE(cref.clone())?, type_: Type::toDAE(var_field!((**cref).ty, ComponentRef::NFComponentRef::CREF).clone(), true)?, exp: Expression::toDAE(e.clone(), false)? }
        },
        _ => {
            DAE::ExtArg::EXTARGEXP { exp: Expression::toDAE(exp.clone(), false)?, type_: Type::toDAE(Expression::typeOf(exp.clone()), true)? }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

fn convertExternalDeclOutput(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<DAE::ExtArg> {
    let mut arg: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
    arg = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } => {
            let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
            dir = Prefixes::directionToAbsyn(Component::direction(InstNode::component(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?));
            DAE::ExtArg::EXTARG { componentRef: ComponentRef::toDAE(cref.clone())?, direction: dir.clone(), type_: Type::toDAE(var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone(), true)? }
        },
        _ => {
            openmodelica_frontend_types::DAE::ExtArg::NOEXTARG
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

pub fn makeTypeVars(mut complexCls: Arc<InstNode::InstNode>) -> Result<Arc<metamodelica::List<Arc<DAE::Var>>>> {
    let mut typeVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut type_var: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    typeVars = { let cls = InstNode::getClass(complexCls.clone())?; (::match_deref::match_deref! { match &(cls) {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::RECORD { .. }, .. } => {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut c in (ClassTree::getComponents(var_field!((*cls).elements, Class::NFClass::INSTANCED_CLASS).clone())?).borrow().iter() {
            let __x = makeTypeRecordVar(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::RECORD_CONSTRUCTOR, .. } => {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut c in (ClassTree::getComponents(var_field!((*cls).elements, Class::NFClass::INSTANCED_CLASS).clone())?).borrow().iter() {
            if !(!(InstNode::isOutput(c.clone()))) { continue; }
            let __x = makeTypeRecordVar(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut c in (ClassTree::getComponents(var_field!((*cls).elements, Class::NFClass::INSTANCED_CLASS).clone())?).borrow().iter() {
            if !(!(InstNode::isOnlyOuter(c.clone())?)) { continue; }
            let __x = makeTypeVar(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) };
    Ok(typeVars)
}

pub fn makeTypeVar(mut component: Arc<InstNode::InstNode>) -> Result<Arc<DAE::Var>> {
    let mut typeVar: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    comp = InstNode::component(InstNode::resolveOuter(component.clone()))?;
    attr = Component::getAttributes(comp.clone());
    typeVar = Arc::new(DAE::Var { name: (InstNode::name(component.clone())?).clone(), attributes: Attributes::toDAE(attr.clone(), InstNode::visibility(component.clone()))?, ty: Type::toDAE(Component::getType(comp.clone())?, true)?, binding: Binding::toDAE(Component::getBinding(comp.clone()))?, bind_from_outside: false, constOfForIteratorRange: None });
    Ok(typeVar)
}

pub fn makeTypeRecordVar(mut component: Arc<InstNode::InstNode>) -> Result<Arc<DAE::Var>> {
    let mut typeVar: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut vis: Visibility = Visibility::PUBLIC;
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut bind_from_outside: bool = false;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    comp = InstNode::component(component.clone())?;
    attr = Component::getAttributes(comp.clone());
    if Component::isFinal(comp.clone())? {
        vis = Visibility::PROTECTED.clone();
    } else {
        vis = InstNode::visibility(component.clone());
    }
    binding = Component::getBinding(comp.clone());
    binding = Binding::mapExp(binding.clone(), (std::sync::Arc::new(stripScopePrefixExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    binding = Flatten::flattenBinding(binding.clone(), Flatten::EMPTY_PREFIX().clone(), false)?;
    bind_from_outside = Binding::source(binding.clone()) == Binding::Source::MODIFIER.clone();
    ty = Component::getType(comp.clone())?;
    ty = Type::mapDims(ty.clone(), (std::sync::Arc::new(stripScopePrefixFromDim) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static>));
    typeVar = Arc::new(DAE::Var { name: (InstNode::name(component.clone())?).clone(), attributes: Attributes::toDAE(attr.clone(), vis.clone())?, ty: Type::toDAE(ty.clone(), true)?, binding: Binding::toDAE(binding.clone())?, bind_from_outside: bind_from_outside.clone(), constOfForIteratorRange: None });
    Ok(typeVar)
}

fn stripScopePrefixFromDim(mut dim: Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = dim;
    dim = Dimension::mapExp(dim.clone(), (std::sync::Arc::new(fnptr!(stripScopePrefixCrefExp, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(dim)
}

fn stripScopePrefixExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), (std::sync::Arc::new(fnptr!(stripScopePrefixCrefExp, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

fn stripScopePrefixCrefExp(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = stripScopePrefixCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

fn stripScopePrefixCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    if ComponentRef::isSimple(cref.clone()) {
        return cref;
    }
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } => {
            if ComponentRef::isFromCref(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone()) {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = stripScopePrefixCref(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone()));
            } else {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = Arc::new(crate::NFComponentRef::EMPTY));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

