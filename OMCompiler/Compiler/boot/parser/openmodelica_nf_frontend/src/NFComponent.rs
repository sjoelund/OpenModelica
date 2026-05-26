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
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRestriction as Restriction;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode::Element;
use openmodelica_frontend_types::SCode;
use openmodelica_util::IOStream;
use openmodelica_util::Util;

#[derive(Clone, Debug, PartialEq)]
pub enum NFComponent {
    COMPONENT_DEF {
        definition: Arc<Element>,
        modifier: Arc<Modifier::Modifier>,
    },
    COMPONENT {
        classInst: Arc<InstNode::InstNode>,
        ty: Arc<Type::NFType>,
        binding: Arc<Binding::NFBinding>,
        condition: Arc<Binding::NFBinding>,
        attributes: Arc<Attributes::NFAttributes>,
        comment: Arc<SCode::Comment>,
        state: ComponentState,
        info: SourceInfo,
    },
    ITERATOR {
        ty: Arc<Type::NFType>,
        variability: Prefixes::Variability,
        info: SourceInfo,
    },
    ENUM_LITERAL {
        literal: Arc<Expression::NFExpression>,
        comment: Arc<SCode::Comment>,
    },
    TYPE_ATTRIBUTE {
        ty: Arc<Type::NFType>,
        modifier: Arc<Modifier::Modifier>,
    },
    INVALID_COMPONENT {
        component: Arc<NFComponent>,
        errors: ArcStr,
    },
    /// needed for new crefs in the backend
    WILD,
}
impl Default for NFComponent {
    fn default() -> Self { Self::WILD }
}
pub use self::NFComponent::{COMPONENT_DEF,COMPONENT,ITERATOR,ENUM_LITERAL,TYPE_ATTRIBUTE,INVALID_COMPONENT,WILD};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ComponentState {
    /// Component instance has been created
    PartiallyInstantiated = 1,
    /// All component expressions have been instantiated
    FullyInstantiated = 2,
    /// The component's type has been determined
    Typed = 3,
    /// The component's binding has been typed and type checked
    TypeChecked = 4,
}
impl PartialOrd for ComponentState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ComponentState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn new(mut definition: Arc<Element>) -> Arc<NFComponent> {
    let mut component: Arc<NFComponent> = Arc::new(NFComponent::WILD);
    component = Arc::new(NFComponent::COMPONENT_DEF { definition: definition.clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD) });
    component
}

pub fn newEnum(mut enumType: Arc<Type::NFType>, mut literalName: ArcStr, mut comment: Arc<SCode::Comment>, mut literalIndex: i32) -> Arc<NFComponent> {
    let mut component: Arc<NFComponent> = Arc::new(NFComponent::WILD);
    component = Arc::new(NFComponent::ENUM_LITERAL { literal: Arc::new(Expression::NFExpression::ENUM_LITERAL { ty: enumType.clone(), name: (literalName.clone()).clone(), index: literalIndex.clone() }), comment: comment.clone() });
    component
}

pub fn newIterator(mut iterType: Arc<Type::NFType>, mut info: SourceInfo) -> Arc<NFComponent> {
    let mut component: Arc<NFComponent> = Arc::new(NFComponent::WILD);
    component = Arc::new(NFComponent::ITERATOR { ty: iterType.clone(), variability: Variability::IMPLICITLY_DISCRETE.clone(), info: info.clone() });
    component
}

pub fn definition(mut component: Arc<NFComponent>) -> Result<Arc<Element>> {
    let mut definition: Arc<Element>;
    let __pa0 = ::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { definition: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    definition = __pa0.clone();
    Ok(definition)
}

pub fn isDefinition(mut component: Arc<NFComponent>) -> bool {
    let mut isDefinition: bool = false;
    isDefinition = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDefinition
}

pub fn info(mut component: Arc<NFComponent>) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { .. } => SCodeUtil::elementInfo(var_field!((*component).definition, NFComponent::COMPONENT_DEF).clone()),
        Deref @ COMPONENT { .. } => var_field!((*component).info, NFComponent::COMPONENT).clone(),
        Deref @ ITERATOR { .. } => var_field!((*component).info, NFComponent::ITERATOR).clone(),
        Deref @ TYPE_ATTRIBUTE { .. } => Modifier::info(var_field!((*component).modifier, NFComponent::TYPE_ATTRIBUTE).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(info)
}

pub fn classInstance(mut component: Arc<NFComponent>) -> Arc<InstNode::InstNode> {
    let mut classInst: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    classInst = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => var_field!((*component).classInst, NFComponent::COMPONENT).clone(),
        Deref @ ITERATOR { ty: Deref @ Type::COMPLEX { cls: classInst, .. }, .. } => classInst.clone(),
        Deref @ ITERATOR { .. } => Arc::new(InstNode::InstNode::ITERATOR_NODE { exp: Arc::new(Expression::NFExpression::EMPTY { ty: var_field!((*component).ty, NFComponent::ITERATOR).clone() }) }),
        _ => Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    classInst
}

pub fn setClassInstance(mut classInst: Arc<InstNode::InstNode>, mut component: Arc<NFComponent>) -> Result<Arc<NFComponent>> {
    let mut component: Arc<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; classInst = classInst.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(component)
}

pub fn getModifier(mut component: Arc<NFComponent>) -> Arc<Modifier::Modifier> {
    let mut modifier: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    modifier = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { .. } => var_field!((*component).modifier, NFComponent::COMPONENT_DEF).clone(),
        Deref @ TYPE_ATTRIBUTE { .. } => var_field!((*component).modifier, NFComponent::TYPE_ATTRIBUTE).clone(),
        _ => Arc::new(crate::NFModifier::Modifier::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    modifier
}

pub fn setModifier(mut modifier: Arc<Modifier::Modifier>, mut component: Arc<NFComponent>) -> Result<Arc<NFComponent>> {
    let mut component: Arc<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT_DEF; modifier = modifier.clone());
            ()
        },
        Deref @ TYPE_ATTRIBUTE { .. } => {
            assign_variant_field!(component => NFComponent::TYPE_ATTRIBUTE; modifier = modifier.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(component)
}

pub fn mergeModifier(mut modifier: Arc<Modifier::Modifier>, mut component: Arc<NFComponent>) -> Result<Arc<NFComponent>> {
    let mut component: Arc<NFComponent> = component;
    component = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT_DEF; modifier = Modifier::merge(modifier.clone(), var_field!((*component).modifier, NFComponent::COMPONENT_DEF).clone(), (literal!("")).clone())?);
            component.clone()
        },
        Deref @ TYPE_ATTRIBUTE { .. } => Arc::new(NFComponent::TYPE_ATTRIBUTE { ty: var_field!((*component).ty, NFComponent::TYPE_ATTRIBUTE).clone(), modifier: Modifier::merge(modifier.clone(), var_field!((*component).modifier, NFComponent::TYPE_ATTRIBUTE).clone(), (literal!("")).clone())? }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(component)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getType(mut component: Arc<NFComponent>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { ty: Deref @ Type::UNTYPED { .. }, .. } => InstNode::getType(var_field!((*component).classInst, NFComponent::COMPONENT).clone())?,
        Deref @ COMPONENT { .. } => var_field!((*component).ty, NFComponent::COMPONENT).clone(),
        Deref @ ITERATOR { .. } => var_field!((*component).ty, NFComponent::ITERATOR).clone(),
        Deref @ TYPE_ATTRIBUTE { .. } => var_field!((*component).ty, NFComponent::TYPE_ATTRIBUTE).clone(),
        Deref @ INVALID_COMPONENT { .. } => getType(var_field!((*component).component, NFComponent::INVALID_COMPONENT).clone())?,
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn setType(mut ty: Arc<Type::NFType>, mut component: Arc<NFComponent>) -> Result<Arc<NFComponent>> {
    let mut component: Arc<NFComponent> = component;
    component = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; ty = ty.clone());
            component.clone()
        },
        Deref @ ITERATOR { .. } => {
            assign_variant_field!(component => NFComponent::ITERATOR; ty = ty.clone());
            component.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(component)
}

pub fn isTyped(mut component: Arc<NFComponent>) -> bool {
    let mut isTyped: bool = false;
    isTyped = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => var_field!((*component).state, NFComponent::COMPONENT).clone() >= ComponentState::Typed.clone(),
        Deref @ ITERATOR { ty: Deref @ Type::UNKNOWN, .. } => false,
        Deref @ ITERATOR { .. } => true,
        Deref @ TYPE_ATTRIBUTE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTyped
}

pub fn unliftType(mut component: Arc<NFComponent>) -> Arc<NFComponent> {
    let mut component: Arc<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { ty: Deref @ Type::ARRAY { elementType: ty, .. }, .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; ty = ty.clone());
            ()
        },
        Deref @ ITERATOR { ty: Deref @ Type::ARRAY { elementType: ty, .. }, .. } => {
            assign_variant_field!(component => NFComponent::ITERATOR; ty = ty.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    component
}

pub fn getAttributes(mut component: Arc<NFComponent>) -> Arc<Attributes::NFAttributes> {
    let mut attr: Arc<Attributes::NFAttributes>;
    attr = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => var_field!((*component).attributes, NFComponent::COMPONENT).clone(),
        _ => Attributes::DEFAULT_ATTR().clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attr
}

pub fn setAttributes(mut attr: Arc<Attributes::NFAttributes>, mut component: Arc<NFComponent>) -> Result<Arc<NFComponent>> {
    let mut component: Arc<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(component)
}

pub fn setComment(mut comment: Arc<SCode::Comment>, mut component: Arc<NFComponent>) -> Result<Arc<NFComponent>> {
    let mut component: Arc<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; comment = comment.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(component)
}

pub fn getBinding(mut component: Arc<NFComponent>) -> Arc<Binding::NFBinding> {
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    b = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => var_field!((*component).binding, NFComponent::COMPONENT).clone(),
        Deref @ TYPE_ATTRIBUTE { .. } => Modifier::binding(var_field!((*component).modifier, NFComponent::TYPE_ATTRIBUTE).clone()),
        Deref @ WILD { .. } => Arc::new(crate::NFBinding::WILD),
        _ => Binding::EMPTY_BINDING().clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn getImplicitBinding(mut component: Arc<NFComponent>, mut scope: Arc<InstNode::InstNode>) -> Arc<Binding::NFBinding> {
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut record_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    binding = getBinding(component.clone());
    if Binding::isUnbound(binding.clone()) {
        cls_node = classInstance(component.clone());
        if InstNode::isRecord(cls_node.clone()) {
            match '__try0: {
                if isTyped(component.clone()) {
                    record_exp = unwrap_break_err!(Class::makeRecordExp(cls_node.clone(), scope.clone(), true), '__try0);
                    binding = unwrap_break_err!(Binding::makeTyped(record_exp.clone(), Binding::EachType::NOT_EACH.clone(), Binding::Source::GENERATED.clone(), info(component.clone()).unwrap(), Binding::EvalState::NOT_EVALUATED.clone()), '__try0);
                } else {
                    record_exp = unwrap_break_err!(Class::makeRecordExp(cls_node.clone(), scope.clone(), false), '__try0);
                    binding = Binding::makeUntyped(record_exp.clone(), scope.clone(), Binding::EachType::NOT_EACH.clone(), Binding::Source::GENERATED.clone(), info(component.clone()).unwrap());
                }
                Ok::<_, anyhow::Error>((binding.clone(), record_exp.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    binding = __try0_o0;
                    record_exp = __try0_o1;
                }
                Err(_) => {
                    panic!("try/else: outputs not set in else branch");
                }
            }
        }
    }
    binding
}

pub fn getTypeAttributeBinding(mut component: Arc<NFComponent>, mut attrName: ArcStr) -> Arc<Binding::NFBinding> {
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut start_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut start_comp: Arc<NFComponent> = Arc::new(NFComponent::WILD);
    match '__try0: {
        (start_node, _) = unwrap_break_err!(Class::lookupElement((attrName.clone()).clone(), InstNode::getClass(classInstance(component.clone())).unwrap()), '__try0);
        start_comp = unwrap_break_err!(InstNode::component(start_node.clone()), '__try0);
        let true = (isTypeAttribute(start_comp.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        binding = getBinding(start_comp.clone());
        Ok::<_, anyhow::Error>((binding.clone(),))
    } {
        Ok((__try0_o0,)) => {
            binding = __try0_o0;
        }
        Err(_) => {
            binding = Binding::EMPTY_BINDING().clone();
        }
    }
    binding
}

pub fn setBinding(mut binding: Arc<Binding::NFBinding>, mut component: Arc<NFComponent>) -> Result<Arc<NFComponent>> {
    let mut component: Arc<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => {
            assign_variant_field!(component => NFComponent::COMPONENT; binding = binding.clone());
            ()
        },
        Deref @ TYPE_ATTRIBUTE { .. } => {
            assign_variant_field!(component => NFComponent::TYPE_ATTRIBUTE; modifier = Modifier::setBinding(binding.clone(), var_field!((*component).modifier, NFComponent::TYPE_ATTRIBUTE).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(component)
}

pub fn hasBinding(mut component: Arc<NFComponent>, mut parent: Arc<InstNode::InstNode>) -> Result<bool> {
    fn has_missing_binding(mut component: Arc<InstNode::InstNode>) -> Result<bool> {
        let mut noBinding: bool = false;
        noBinding = InstNode::isComponent(component.clone()) && !(hasBinding(InstNode::component(component.clone())?, Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?);
        Ok(noBinding)
    }

    let mut b: bool = false;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    if Binding::isBound(getBinding(component.clone())) {
        b = true;
        return Ok(b);
    }
    cls = InstNode::getClass(classInstance(component.clone()))?;
    if !(Restriction::isRecord(Class::restriction(cls.clone()))) {
        b = false;
        return Ok(b);
    }
    if isSome(ClassTree::findComponent(Class::classTree(cls.clone())?, Arc::new(has_missing_binding))) {
        b = false;
    }
    b = true;
    Ok(b)
}

pub fn getCondition(mut component: Arc<NFComponent>) -> Arc<Binding::NFBinding> {
    let mut cond: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    cond = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => var_field!((*component).condition, NFComponent::COMPONENT).clone(),
        _ => Binding::EMPTY_BINDING().clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cond
}

pub fn hasCondition(mut component: Arc<NFComponent>) -> bool {
    let mut b: bool = false;
    b = Binding::isBound(getCondition(component.clone()));
    b
}

pub fn direction(mut component: Arc<NFComponent>) -> Prefixes::Direction {
    let mut direction: Prefixes::Direction = Prefixes::Direction::NONE;
    direction = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { direction, .. }, .. } => direction.clone(),
        _ => Direction::NONE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    direction
}

pub fn isInput(mut component: Arc<NFComponent>) -> bool {
    let mut isInput: bool = direction(component.clone()) == Direction::INPUT.clone();
    isInput
}

pub fn setDirection(mut direction: Prefixes::Direction, mut component: Arc<NFComponent>) -> Arc<NFComponent> {
    let mut component: Arc<NFComponent> = component;
    let mut attr: Arc<Attributes::NFAttributes>;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: attr, .. } => {
            let mut attr = (*attr).clone();
            assign_field!(attr.direction = direction.clone());
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    component
}

pub fn isOutput(mut component: Arc<NFComponent>) -> bool {
    let mut isOutput: bool = direction(component.clone()) == Direction::OUTPUT.clone();
    isOutput
}

pub fn parallelism(mut component: Arc<NFComponent>) -> Prefixes::Parallelism {
    let mut parallelism: Prefixes::Parallelism = Prefixes::Parallelism::NON_PARALLEL;
    parallelism = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { parallelism, .. }, .. } => parallelism.clone(),
        _ => Parallelism::NON_PARALLEL.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    parallelism
}

pub fn variability(mut component: Arc<NFComponent>) -> Prefixes::Variability {
    let mut variability: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    variability = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { variability, .. }, .. } => variability.clone(),
        Deref @ ITERATOR { .. } => var_field!((*component).variability, NFComponent::ITERATOR).clone(),
        Deref @ ENUM_LITERAL { .. } => Variability::CONSTANT.clone(),
        Deref @ INVALID_COMPONENT { .. } => self::variability(var_field!((*component).component, NFComponent::INVALID_COMPONENT).clone()),
        _ => Variability::CONTINUOUS.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    variability
}

pub fn setVariability(mut variability: Prefixes::Variability, mut component: Arc<NFComponent>) -> Arc<NFComponent> {
    let mut component: Arc<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: attr, .. } => {
            let mut attr = (*attr).clone();
            assign_field!(attr.variability = variability.clone());
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    component
}

pub fn isConst(mut component: Arc<NFComponent>) -> bool {
    let mut isConst: bool = variability(component.clone()) == Variability::CONSTANT.clone();
    isConst
}

pub fn isParameter(mut component: Arc<NFComponent>) -> bool {
    let mut b: bool = variability(component.clone()) == Variability::PARAMETER.clone();
    b
}

pub fn isStructuralParameter(mut component: Arc<NFComponent>) -> bool {
    let mut b: bool = variability(component.clone()) == Variability::STRUCTURAL_PARAMETER.clone();
    b
}

pub fn isVar(mut component: Arc<NFComponent>) -> bool {
    let mut isVar: bool = variability(component.clone()) == Variability::CONTINUOUS.clone();
    isVar
}

pub fn isRedeclare(mut component: Arc<NFComponent>) -> Result<bool> {
    let mut isRedeclare: bool = false;
    isRedeclare = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { .. } => SCodeUtil::isElementRedeclare(var_field!((*component).definition, NFComponent::COMPONENT_DEF).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isRedeclare)
}

pub fn isFinal(mut component: Arc<NFComponent>) -> Result<bool> {
    let mut isFinal: bool = false;
    isFinal = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { .. } => SCodeUtil::finalBool(SCodeUtil::prefixesFinal(SCodeUtil::elementPrefixes(var_field!((*component).definition, NFComponent::COMPONENT_DEF).clone())?)?)?,
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { isFinal, .. }, .. } => isFinal.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isFinal)
}

pub fn setFinal(mut component: Arc<NFComponent>, mut isFinal: bool) -> Arc<NFComponent> {
    let mut component: Arc<NFComponent> = component;
    let mut attr: Arc<Attributes::NFAttributes>;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: attr, .. } => {
            let mut attr = (*attr).clone();
            assign_field!(attr.isFinal = isFinal.clone());
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    component
}

pub fn isResizable(mut component: Arc<NFComponent>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { isResizable: b, .. }, .. } => b.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn innerOuter(mut component: Arc<NFComponent>) -> Result<Prefixes::InnerOuter> {
    let mut io: Prefixes::InnerOuter = Prefixes::InnerOuter::NOT_INNER_OUTER;
    io = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { innerOuter: io, .. }, .. } => io.clone(),
        Deref @ COMPONENT_DEF { .. } => Prefixes::innerOuterFromSCode(SCodeUtil::prefixesInnerOuter(SCodeUtil::elementPrefixes(var_field!((*component).definition, NFComponent::COMPONENT_DEF).clone())?)?)?,
        _ => InnerOuter::NOT_INNER_OUTER.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(io)
}

pub fn isInnerOuter(mut component: Arc<NFComponent>) -> Result<bool> {
    let mut isInnerOuter: bool = false;
    isInnerOuter = innerOuter(component.clone())? != InnerOuter::NOT_INNER_OUTER.clone();
    Ok(isInnerOuter)
}

pub fn isInner(mut component: Arc<NFComponent>) -> bool {
    let mut isInner: bool = false;
    let mut io: Prefixes::InnerOuter = innerOuter(component.clone()).unwrap();
    isInner = io.clone() == InnerOuter::INNER.clone() || io.clone() == InnerOuter::INNER_OUTER.clone();
    isInner
}

pub fn isOuter(mut component: Arc<NFComponent>) -> bool {
    let mut isOuter: bool = false;
    let mut io: Prefixes::InnerOuter = innerOuter(component.clone()).unwrap();
    isOuter = io.clone() == InnerOuter::OUTER.clone() || io.clone() == InnerOuter::INNER_OUTER.clone();
    isOuter
}

pub fn isOnlyOuter(mut component: Arc<NFComponent>) -> bool {
    let mut isOuter: bool = innerOuter(component.clone()).unwrap() == InnerOuter::OUTER.clone();
    isOuter
}

pub fn connectorType(mut component: Arc<NFComponent>) -> i32 {
    let mut cty: i32 = 0;
    cty = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: Deref @ Attributes::ATTRIBUTES { connectorType: cty, .. }, .. } => cty.clone(),
        _ => ConnectorType::NON_CONNECTOR.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cty
}

pub fn setConnectorType(mut cty: i32, mut component: Arc<NFComponent>) -> Arc<NFComponent> {
    let mut component: Arc<NFComponent> = component;
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { attributes: attr, .. } => {
            let mut attr = (*attr).clone();
            assign_field!(attr.connectorType = cty.clone());
            assign_variant_field!(component => NFComponent::COMPONENT; attributes = attr.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    component
}

pub fn isFlow(mut component: Arc<NFComponent>) -> bool {
    let mut isFlow: bool = Prefixes::ConnectorType::isFlow(connectorType(component.clone()));
    isFlow
}

pub fn isConnector(mut component: Arc<NFComponent>) -> bool {
    let mut isConnector: bool = Prefixes::ConnectorType::isConnectorType(connectorType(component.clone()));
    isConnector
}

pub fn isExpandableConnector(mut component: Arc<NFComponent>) -> bool {
    let mut isConnector: bool = Prefixes::ConnectorType::isExpandable(connectorType(component.clone()));
    isConnector
}

pub fn isExternalObject(mut component: Arc<NFComponent>) -> Result<bool> {
    let mut isEO: bool = false;
    isEO = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { ty: Deref @ Type::UNTYPED { .. }, .. } => Class::isExternalObject(InstNode::getClass(var_field!((*component).classInst, NFComponent::COMPONENT).clone())?),
        Deref @ COMPONENT { .. } => Type::isExternalObject(var_field!((*component).ty, NFComponent::COMPONENT).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEO)
}

pub fn isIdentical(mut comp1: Arc<NFComponent>, mut comp2: Arc<NFComponent>) -> Result<bool> {
    let mut identical: bool = false;
    if referenceEq(&comp1.clone(),&comp2.clone()) {
        identical = true;
    } else {
        identical = (::match_deref::match_deref! { match &((comp1.clone(), comp2.clone())) {
        (Deref @ COMPONENT { .. }, Deref @ COMPONENT { .. }) => {
            if !(Class::isIdentical(InstNode::getClass(var_field!((*comp1).classInst, NFComponent::COMPONENT).clone())?, InstNode::getClass(var_field!((*comp2).classInst, NFComponent::COMPONENT).clone())?)) {
                return Ok(identical);
            }
            if !(Binding::isEqual(var_field!((*comp1).binding, NFComponent::COMPONENT).clone(), var_field!((*comp2).binding, NFComponent::COMPONENT).clone())?) {
                return Ok(identical);
            }
            true
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(identical)
}

pub fn toString(mut name: ArcStr, mut component: Arc<NFComponent>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { definition: def @ Deref @ SCode::Element::COMPONENT { .. }, .. } => {
            SCodeDump::unparseElementStr(def.clone(), SCodeDump::defaultOptions.clone())?
        },
        Deref @ COMPONENT { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Attributes::toString(var_field!((*component).attributes, NFComponent::COMPONENT).clone(), var_field!((*component).ty, NFComponent::COMPONENT).clone())); __mm_s.push_str(&*Type::toString(var_field!((*component).ty, NFComponent::COMPONENT).clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*Binding::toString(var_field!((*component).binding, NFComponent::COMPONENT).clone(), (literal!(" = ")).clone())?); ArcStr::from(__mm_s) }
        },
        Deref @ TYPE_ATTRIBUTE { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*Modifier::toString(var_field!((*component).modifier, NFComponent::TYPE_ATTRIBUTE).clone(), false)?); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn toFlatStream(mut name: ArcStr, mut component: Arc<NFComponent>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut ty_attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => {
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = Attributes::toFlatStream(var_field!((*component).attributes, NFComponent::COMPONENT).clone(), var_field!((*component).ty, NFComponent::COMPONENT).clone(), s.clone(), true)?;
            s = IOStream::append(s.clone(), (Type::toFlatString(Type::arrayElementType(var_field!((*component).ty, NFComponent::COMPONENT).clone()), format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
            s = IOStream::append(s.clone(), (Util::makeQuotedIdentifier((name.clone()).clone())?).clone())?;
            dims = Type::arrayDims(var_field!((*component).ty, NFComponent::COMPONENT).clone());
            if !(dims.clone().is_empty()) {
                s = IOStream::append(s.clone(), (Dimension::toFlatStringList(dims.clone(), format.clone(), (literal!("")).clone())?).clone())?;
            }
            ty_attrs = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut a in (Class::getTypeAttributes(InstNode::getClass(var_field!((*component).classInst, NFComponent::COMPONENT).clone())?)).into_iter().cloned() {
            let __x = (Modifier::name(a.clone())?, Modifier::binding(a.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            s = typeAttrsToFlatStream(ty_attrs.clone(), var_field!((*component).ty, NFComponent::COMPONENT).clone(), format.clone(), s.clone())?;
            s = IOStream::append(s.clone(), (Binding::toFlatString(var_field!((*component).binding, NFComponent::COMPONENT).clone(), format.clone(), (literal!(" = ")).clone())?).clone())?;
            ()
        },
        Deref @ TYPE_ATTRIBUTE { .. } => {
            s = IOStream::append(s.clone(), (name.clone()).clone())?;
            s = IOStream::append(s.clone(), (Modifier::toFlatString(var_field!((*component).modifier, NFComponent::TYPE_ATTRIBUTE).clone(), format.clone(), false)?).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(s)
}

pub fn typeAttrsToFlatStream(mut typeAttrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut componentType: Arc<Type::NFType>, mut format: BaseModelica::OutputFormat, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut var_dims: i32 = 0;
    let mut binding_dims: i32 = 0;
    let mut ty_attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = typeAttrs.clone();
    let mut name: ArcStr = arcstr::literal!("");
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut bind_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if ty_attrs.clone().is_empty() {
        return Ok(s);
    }
    s = IOStream::append(s.clone(), (literal!("(")).clone())?;
    var_dims = Type::dimensionCount(componentType.clone());
    while true {
        (name, binding) = listHead(ty_attrs.clone())?;
        bind_exp = Expression::expandSplitIndices(Binding::getExp(binding.clone())?)?;
        binding_dims = Type::dimensionCount(Expression::typeOf(bind_exp.clone()));
        if var_dims.clone() > binding_dims.clone() {
            s = IOStream::append(s.clone(), (literal!("each ")).clone())?;
        }
        s = IOStream::append(s.clone(), (name.clone()).clone())?;
        s = IOStream::append(s.clone(), (literal!(" = ")).clone())?;
        s = IOStream::append(s.clone(), (Expression::toFlatString(bind_exp.clone(), format.clone())?).clone())?;
        ty_attrs = listRest(ty_attrs.clone())?;
        if ty_attrs.clone().is_empty() {
            break;
        } else {
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
        }
    }
    s = IOStream::append(s.clone(), (literal!(")")).clone())?;
    Ok(s)
}

pub fn toFlatString(mut name: ArcStr, mut component: Arc<NFComponent>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: IOStream::IOStream;
    s = IOStream::create((name.clone()).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toFlatStream((name.clone()).clone(), component.clone(), format.clone(), (indent.clone()).clone(), s.clone())?;
    r#str = (IOStream::string(s.clone())?).clone();
    IOStream::delete(s.clone())?;
    Ok(r#str)
}

pub fn dimensionCount(mut component: Arc<NFComponent>) -> i32 {
    let mut count: i32 = 0;
    count = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { .. } => Type::dimensionCount(var_field!((*component).ty, NFComponent::COMPONENT).clone()),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    count
}

pub fn comment(mut component: Arc<NFComponent>) -> Result<Arc<SCode::Comment>> {
    let mut comment: Arc<SCode::Comment>;
    comment = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_DEF { .. } => Util::getOption(SCodeUtil::getElementComment(var_field!((*component).definition, NFComponent::COMPONENT_DEF).clone()))?,
        Deref @ COMPONENT { .. } => var_field!((*component).comment, NFComponent::COMPONENT).clone(),
        Deref @ ENUM_LITERAL { .. } => var_field!((*component).comment, NFComponent::ENUM_LITERAL).clone(),
        _ => SCode::noComment.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comment)
}

pub fn getEvaluateAnnotation(mut component: Arc<NFComponent>) -> Result<Option<bool>> {
    let mut evaluate: Option<bool> = None;
    let mut cmt: Arc<SCode::Comment>;
    evaluate = SCodeUtil::getEvaluateAnnotation(comment(component.clone())?)?;
    Ok(evaluate)
}

pub fn isFixed(mut component: Arc<NFComponent>) -> Result<bool> {
    let mut fixed: bool = false;
    let mut typeAttrs: Arc<metamodelica::List<Arc<Modifier::Modifier>>> = metamodelica::nil();
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    fixed = isParameter(component.clone()) || isStructuralParameter(component.clone());
    binding = Class::lookupAttributeBinding((literal!("fixed")).clone(), InstNode::getClass(classInstance(component.clone()))?);
    if Binding::isUnbound(binding.clone()) {
        return Ok(fixed);
    }
    if Binding::hasExp(binding.clone()) {
        fixed = fixed.clone() && Expression::isTrue(Binding::getExp(binding.clone())?);
    } else {
        fixed = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::RAW_BINDING { bindingExp: Deref @ Absyn::Exp::BOOL { value: true }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(fixed)
}

pub fn getUnitAttribute(mut component: Arc<NFComponent>, mut defaultUnit: ArcStr) -> Result<ArcStr> {
    let mut unitString: ArcStr = arcstr::literal!("");
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut unit: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    binding = Class::lookupAttributeBinding((literal!("unit")).clone(), InstNode::getClass(classInstance(component.clone()))?);
    if Binding::isUnbound(binding.clone()) {
        unitString = (defaultUnit.clone()).clone();
        return Ok(unitString);
    }
    unit = Binding::getExp(binding.clone())?;
    unitString = ((::match_deref::match_deref! { match &(unit.clone()) {
        Deref @ Expression::STRING { .. } => var_field!((*unit).value, Expression::NFExpression::STRING).clone(),
        _ => defaultUnit.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(unitString)
}

pub fn isDeleted(mut component: Arc<NFComponent>) -> Result<bool> {
    let mut isDeleted: bool = false;
    isDeleted = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT { condition, .. } => {
            Binding::isTyped(condition.clone()) && Expression::isFalse(Binding::getTypedExp(condition.clone())?)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isDeleted)
}

pub fn isInvalid(mut component: Arc<NFComponent>) -> bool {
    let mut invalid: bool = false;
    invalid = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ INVALID_COMPONENT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    invalid
}

pub fn isTypeAttribute(mut component: Arc<NFComponent>) -> bool {
    let mut isAttribute: bool = false;
    isAttribute = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ TYPE_ATTRIBUTE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isAttribute
}

pub fn countConnectorVars(mut component: Arc<NFComponent>, mut isRoot: bool) -> Result<(i32, i32, i32, bool)> {
    let mut potentials: i32 = 0;
    let mut flows: i32 = 0;
    let mut streams: i32 = 0;
    let mut knownSize: bool = true;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cty: i32 = 0;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut eq_node_opt: Option<Arc<InstNode::InstNode>> = None;
    let mut eq_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comp_size: i32 = 0;
    let mut p: i32 = 0;
    let mut f: i32 = 0;
    let mut s: i32 = 0;
    let mut r#fn: Arc<Function::Function>;
    let mut known_size: bool = false;
    cls = InstNode::getClass(classInstance(component.clone()))?;
    (eq_node_opt, _) = Class::tryLookupElement((literal!("equalityConstraint")).clone(), cls.clone());
    if isSome(eq_node_opt.clone()) && SCodeUtil::isFunction(InstNode::definition(Util::getOption(eq_node_opt.clone())?)?) {
        let __pa0 = ::match_deref::match_deref! { match &(eq_node_opt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        eq_node = __pa0.clone();
        Function::instFunctionNode(eq_node.clone(), NFInstContext::NO_CONTEXT.clone(), info(component.clone())?)?;
        r#fn = listHead(Function::typeNodeCache(eq_node.clone(), NFInstContext::FUNCTION.clone())?)?;
        ty = Function::returnType(r#fn.clone());
        if Type::hasKnownSize(ty.clone()) {
            comp_size = Type::sizeOf(ty.clone(), false)?;
        } else {
            comp_size = 0;
            knownSize = false;
        }
    } else {
        ty = getType(component.clone())?;
        if isRoot.clone() {
            comp_size = 1;
        } else if Type::hasKnownSize(ty.clone()) {
            comp_size = Dimension::sizesProduct(Type::arrayDims(ty.clone()), false);
        } else {
            comp_size = 0;
            knownSize = false;
        }
        ty = Type::arrayElementType(ty.clone());
        if Type::isComplex(ty.clone()) {
            if Type::isRecord(ty.clone()) || isRoot.clone() {
                let __range1 = ClassTree::getComponents(Class::classTree(cls.clone())?)?.borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range1 {
                    (p, f, s, known_size) = countConnectorVars(InstNode::component(c.clone())?, false)?;
                    potentials = potentials.clone() + p.clone() * comp_size.clone();
                    flows = flows.clone() + f.clone() * comp_size.clone();
                    streams = streams.clone() + s.clone() * comp_size.clone();
                    knownSize = known_size.clone() && knownSize.clone();
                }
            }
            comp_size = 0;
        }
    }
    if comp_size.clone() > 0 {
        cty = connectorType(component.clone());
        if Prefixes::ConnectorType::isFlow(cty.clone()) {
            flows = flows.clone() + comp_size.clone();
        } else if Prefixes::ConnectorType::isStream(cty.clone()) {
            streams = streams.clone() + comp_size.clone();
        } else if variability(component.clone()) >= Variability::DISCRETE.clone() && direction(component.clone()) == Direction::NONE.clone() {
            potentials = potentials.clone() + comp_size.clone();
        }
    }
    Ok((potentials, flows, streams, knownSize))
}


