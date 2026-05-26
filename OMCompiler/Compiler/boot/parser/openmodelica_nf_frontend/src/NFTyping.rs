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
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFConnection as Connection;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFModifier::ModTable;
use crate::NFModifier::Modifier;
use crate::NFOperator as Operator;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPackage as Package;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ErrorTypes;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub mod TypingError {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum TypingError {
        NO_ERROR,
        OUT_OF_BOUNDS {
            upperBound: i32,
        },
    }
    impl Default for TypingError {
        fn default() -> Self { Self::NO_ERROR }
    }
    pub use self::TypingError::{NO_ERROR,OUT_OF_BOUNDS};
    pub fn isError(mut error: Arc<TypingError>) -> bool {
        let mut isError: bool = false;
        isError = (::match_deref::match_deref! { match &(error.clone()) {
        Deref @ NO_ERROR { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isError
    }

}

// Used by typeDimension for catching cyclic dimension involving :
// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<Expression::NFExpression>
// Expr: Constructor { name: 'NFExpression.CREF', args: [Constructor { name: 'NFType.UNKNOWN', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Constructor { name: 'NFComponentRef.CREF', args: [Constructor { name: 'NFInstNode.InstNode.NAME_NODE', args: [Lit(Str(':'))], named_args: [], ty: RustStruct('NFInstNode.InstNode.NAME_NODE'), field_names: ['name'] }, Array { elems: [], ty: List(Unknown) }, Constructor { name: 'NFType.UNKNOWN', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Var { name: 'NFComponentRef.Origin.CREF', segments: [CrefSegment { name: 'NFComponentRef', subscripts: [] }, CrefSegment { name: 'Origin', subscripts: [] }, CrefSegment { name: 'CREF', subscripts: [] }], ty: Enumeration('NFComponentRef.Origin') }, Constructor { name: 'NFComponentRef.EMPTY', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }], named_args: [], ty: RustStruct('NFComponentRef.CREF'), field_names: ['node', 'subscripts', 'ty', 'origin', 'restCref'] }], named_args: [], ty: RustStruct('NFExpression.CREF'), field_names: ['ty', 'cref'] }
pub fn WHOLEDIM_CREF() -> Arc<Expression::NFExpression> { todo!("non-Sync, non-const-emittable constant WHOLEDIM_CREF — extend codegen") }

pub fn typeClass(mut cls: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut next_context: i32 = 0;
    next_context = InstContext::set(context.clone(), InstContext::CLASS.clone());
    typeClassType(cls.clone(), Binding::EMPTY_BINDING().clone(), next_context.clone(), cls.clone())?;
    typeComponents(cls.clone(), next_context.clone(), false)?;
    execStat((literal!("NFTyping.typeComponents")).clone())?;
    typeBindings(cls.clone(), next_context.clone())?;
    execStat((literal!("NFTyping.typeBindings")).clone())?;
    typeClassSections(cls.clone(), next_context.clone())?;
    execStat((literal!("NFTyping.typeClassSections")).clone())?;
    Ok(())
}

pub fn typeComponents(mut cls: Arc<InstNode::InstNode>, mut context: i32, mut preserveDerived: bool) -> Result<()> {
    let mut c: Arc<Class::NFClass> = InstNode::getClass(cls.clone())?;
    let mut c2: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut ext_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut con: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut de: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::TYPE, .. } => (),
        Deref @ Class::INSTANCED_CLASS { elements: cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            if InstContext::inInstanceAPI(context.clone()) {
                let __range0 = var_field!((**cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range0 {
                    typeComponentTry(c.clone(), context.clone())?;
                }
            } else {
                let __range1 = var_field!((**cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range1 {
                    typeComponent(c.clone(), context.clone(), true)?;
                }
            }
            let () = (::match_deref::match_deref! { match &(var_field!((*c).ty, Class::NFClass::INSTANCED_CLASS).clone()) {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: con, .. }, .. } => {
            typeStructor(con.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Class::TYPED_DERIVED { .. } if (preserveDerived.clone() || Type::isArray(var_field!((*c).ty, Class::NFClass::TYPED_DERIVED).clone())) => {
            typeComponents(var_field!((*c).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context.clone(), false)?;
            ()
        },
        Deref @ Class::TYPED_DERIVED { .. } => {
            typeComponents(var_field!((*c).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context.clone(), false)?;
            if !(InstContext::inInstanceAPI(context.clone())) {
                c2 = InstNode::getClass(var_field!((*c).baseClass, Class::NFClass::TYPED_DERIVED).clone())?;
                c2 = Class::setRestriction(var_field!((*c).restriction, Class::NFClass::TYPED_DERIVED).clone(), c2.clone())?;
                InstNode::updateClass(c2.clone(), cls.clone())?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { destructor: de, constructor: con }, .. }, .. } => {
            typeStructor(con.clone())?;
            typeStructor(de.clone())?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponents")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*InstNode::name(cls.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeStructor(mut node: Arc<InstNode::InstNode>) -> Result<()> {
    let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
    let mut fnl: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut context: i32 = 0;
    cache = InstNode::getFuncCache(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::FUNCTION { typed: false, funcs: fnl, .. } => {
            let mut fnl = (*fnl).clone();
            context = InstContext::set(InstContext::FUNCTION.clone(), InstContext::RELAXED.clone());
            fnl = {
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut r#fn in (fnl.clone()).into_iter().cloned() {
            let __x = Function::typeFunction(r#fn.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            fnl = {
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut r#fn in (fnl.clone()).into_iter().cloned() {
            let __x = OperatorOverloading::patchOperatorRecordConstructorBinding(r#fn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            InstNode::setFuncCache(node.clone(), Arc::new(CachedData::CachedData::FUNCTION { funcs: fnl.clone(), typed: true, specialBuiltin: var_field!((*cache).specialBuiltin, CachedData::CachedData::FUNCTION).clone() }))?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeClassType(mut clsNode: Arc<InstNode::InstNode>, mut componentBinding: Arc<Binding::NFBinding>, mut context: i32, mut instanceNode: Arc<InstNode::InstNode>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut ty_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ty_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut r#fn: Arc<Function::Function>;
    let mut is_expandable: bool = false;
    cls = InstNode::getClass(clsNode.clone())?;
    ty = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::CONNECTOR { isExpandable: is_expandable }, .. } => {
            ty = Arc::new(Type::NFType::COMPLEX { cls: clsNode.clone(), complexTy: Arc::new(makeConnectorType(var_field!((*cls).elements, Class::NFClass::INSTANCED_CLASS).clone(), is_expandable.clone())?) });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            InstNode::updateClass(cls.clone(), clsNode.clone())?;
            ty.clone()
        },
        Deref @ Class::INSTANCED_CLASS { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: node, .. }, cls: ty_node }, .. } => {
            ty = Arc::new(Type::NFType::COMPLEX { cls: ty_node.clone(), complexTy: makeRecordType(node.clone())? });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            InstNode::updateClass(cls.clone(), clsNode.clone())?;
            ty.clone()
        },
        Deref @ Class::INSTANCED_CLASS { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTENDS_TYPE { baseClass: node }, .. }, .. } => {
            ty = typeClassType(node.clone(), componentBinding.clone(), context.clone(), instanceNode.clone())?;
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            InstNode::updateClass(cls.clone(), clsNode.clone())?;
            ty.clone()
        },
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::FUNCTION, .. } if (InstNode::isComponent(instanceNode.clone())) => {
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(clsNode.clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            ty = Arc::new(Type::NFType::FUNCTION { r#fn: r#fn.clone(), fnType: Type::FunctionType::FUNCTIONAL_PARAMETER.clone() });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            InstNode::updateClass(cls.clone(), clsNode.clone())?;
            ty.clone()
        },
        Deref @ Class::INSTANCED_CLASS { .. } => var_field!((*cls).ty, Class::NFClass::INSTANCED_CLASS).clone(),
        Deref @ Class::EXPANDED_DERIVED { .. } => {
            typeDimensions(var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone(), clsNode.clone(), componentBinding.clone(), context.clone(), InstNode::info(clsNode.clone())?)?;
            ty = typeClassType(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), componentBinding.clone(), context.clone(), instanceNode.clone())?;
            ty = Type::liftArrayLeftList(ty.clone(), Arc::new(var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
            ty_cls = Arc::new(Class::NFClass::TYPED_DERIVED { ty: ty.clone(), baseClass: var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), restriction: var_field!((*cls).restriction, Class::NFClass::EXPANDED_DERIVED).clone() });
            InstNode::updateClass(ty_cls.clone(), clsNode.clone())?;
            ty.clone()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => var_field!((*cls).ty, Class::NFClass::INSTANCED_BUILTIN).clone(),
        Deref @ Class::TYPED_DERIVED { .. } => var_field!((*cls).ty, Class::NFClass::TYPED_DERIVED).clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeClassType")); __mm_s.push_str(&*literal!(" got noninstantiated class ")); __mm_s.push_str(&*InstNode::name(clsNode.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn checkConnectorTypeBalance(mut component: Arc<InstNode::InstNode>) -> Result<()> {
    let mut pots: i32 = 0;
    let mut flows: i32 = 0;
    let mut streams: i32 = 0;
    let mut known_size: bool = false;
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    comp = InstNode::component(component.clone())?;
    if !(Prefixes::ConnectorType::isConnector(Component::connectorType(comp.clone()))) {
        return Ok(());
    }
    parent = InstNode::instanceParent(component.clone());
    if InstNode::isComponent(parent.clone()) && Component::isConnector(InstNode::component(parent.clone())?) {
        return Ok(());
    }
    (pots, flows, streams, known_size) = Component::countConnectorVars(comp.clone(), true)?;
    if !(known_size.clone()) {
        return Ok(());
    }
    if pots.clone() != flows.clone() && !(Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("unbalancedModel")).clone())?) {
        Error::addStrictMessage(Error::UNBALANCED_CONNECTOR.clone(), list![(InstNode::name(component.clone())?).clone(), ArcStr::from(::std::format!("{}", pots.clone())), ArcStr::from(::std::format!("{}", flows.clone()))], InstNode::info(component.clone())?)?;
    }
    if streams.clone() > 0 && flows.clone() != 1 {
        Error::addSourceMessage(Error::MISMATCHED_FLOW_IN_STREAM_CONNECTOR.clone(), list![(InstNode::name(component.clone())?).clone(), ArcStr::from(::std::format!("{}", flows.clone()))], InstNode::info(component.clone())?)?;
        bail!("fail");
    }
    Ok(())
}

pub fn makeRecordType(mut constructor: Arc<InstNode::InstNode>) -> Result<Arc<ComplexType::NFComplexType>> {
    let mut recordTy: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
    let mut r#fn: Arc<Function::Function>;
    let mut fields: metamodelica::Array<Arc<Record::Field::Field>>;
    let mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>;
    cache = InstNode::getFuncCache(constructor.clone())?;
    recordTy = 'mc: {
        let __mc_input = cache.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ CachedData::FUNCTION { .. } => {
                    let mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>;
                    let mut r#fn: Arc<Function::Function>;
                    let mut fields: metamodelica::Array<Arc<Record::Field::Field>>;
                    r#fn = List::find(var_field!((*cache).funcs, CachedData::CachedData::FUNCTION).clone(), Arc::new(fnptr!(Function::isDefaultRecordConstructor, Arc<Function::Function>)))?;
                    (fields, indexMap) = Record::collectRecordFields(r#fn.node.clone())?;
                    Ok(Arc::new(ComplexType::NFComplexType::RECORD { constructor: constructor.clone(), fields: fields.clone(), indexMap: indexMap.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.makeRecordType")); __mm_s.push_str(&*literal!(" got record type without constructor")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(recordTy)
}

pub fn typeComponent(mut component: Arc<InstNode::InstNode>, mut context: i32, mut typeChildren: bool) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut is_deleted: bool = false;
    let mut dims: metamodelica::Array<Arc<Dimension::NFDimension>>;
    if InstNode::isEmpty(component.clone()) || InstNode::isOnlyOuter(component.clone())? {
        return Ok(ty);
    }
    node = InstNode::resolveOuter(component.clone());
    c = InstNode::component(node.clone())?;
    ty = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { ty: Deref @ Type::UNTYPED { dimensions: dims, .. }, .. } => {
            typeDimensions(dims.clone(), node.clone(), var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context.clone(), var_field!((*c).info, Component::NFComponent::COMPONENT).clone())?;
            if InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone()) {
                ty = Arc::new(crate::NFType::UNKNOWN);
            } else {
                ty = typeClassType(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context.clone(), component.clone())?;
            }
            ty = Type::liftArrayLeftList(ty.clone(), Arc::new(dims.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
            if Binding::isBound(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone()) {
                assign_variant_field!(c => Component::NFComponent::COMPONENT; condition = typeComponentCondition(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone(), context.clone(), true)?);
                is_deleted = Expression::isFalse(Binding::getExp(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone())?);
            } else {
                is_deleted = false;
            }
            if typeChildren.clone() {
                assign_variant_field!(c => Component::NFComponent::COMPONENT;
                    ty = ty.clone(),
                    state = ComponentState::Typed.clone()
                );
                InstNode::updateComponent(c.clone(), node.clone())?;
                if !(is_deleted.clone()) && !(InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                    checkComponentStreamAttribute(var_field!((*c).attributes, Component::NFComponent::COMPONENT).connectorType.clone(), ty.clone(), component.clone())?;
                    typeComponents(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context.clone(), false)?;
                    checkConnectorTypeBalance(node.clone())?;
                }
            }
            ty.clone()
        },
        Deref @ Component::COMPONENT { .. } => var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(),
        Deref @ Component::ITERATOR { .. } => var_field!((*c).ty, Component::NFComponent::ITERATOR).clone(),
        Deref @ Component::ENUM_LITERAL { literal: Deref @ Expression::ENUM_LITERAL { ty, .. }, .. } => ty.clone(),
        Deref @ Component::INVALID_COMPONENT { .. } => Component::getType(c.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponent")); __mm_s.push_str(&*literal!(" got noninstantiated component ")); __mm_s.push_str(&*InstNode::name(component.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn typeComponentTry(mut componentNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    ErrorExt::setCheckpoint((literal!("NFTyping.typeComponentTry")).clone());
    if '__try0: {
        unwrap_break_err!(typeComponent(componentNode.clone(), context.clone(), true), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        comp = InstNode::component(componentNode.clone())?;
        comp = Arc::new(Component::NFComponent::INVALID_COMPONENT { component: comp.clone(), errors: (ErrorExt::printCheckpointMessagesStr(false)).clone() });
        InstNode::updateComponent(comp.clone(), componentNode.clone())?;
    }
    ErrorExt::delCheckpoint((literal!("NFTyping.typeComponentTry")).clone());
    Ok(())
}

pub fn typeIterator(mut iterator: Arc<InstNode::InstNode>, mut range: Arc<Expression::NFExpression>, mut context: i32, mut structural: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut outRange: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut c: Arc<Component::NFComponent> = InstNode::component(iterator.clone())?;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: SourceInfo;
    (outRange, ty, var) = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::ITERATOR { info, .. } => {
            (exp, ty, var, purity) = typeExp(range.clone(), InstContext::set(context.clone(), InstContext::ITERATION_RANGE.clone()), info.clone(), false)?;
            if structural.clone() && var.clone() > Variability::PARAMETER.clone() && (!(var.clone() == Variability::NON_STRUCTURAL_PARAMETER.clone()) || Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
                Error::addSourceMessageAndFail(Error::NON_PARAMETER_ITERATOR_RANGE.clone(), list![(Expression::toString(exp.clone())?).clone()], info.clone())?;
            }
            if !(Type::isVector(ty.clone())) {
                Error::addSourceMessageAndFail(Error::FOR_EXPRESSION_TYPE_ERROR.clone(), list![(Expression::toString(exp.clone())?).clone(), (Type::toString(ty.clone())?).clone()], info.clone())?;
            }
            c = Arc::new(Component::NFComponent::ITERATOR { ty: Type::arrayElementType(ty.clone()), variability: var.clone(), info: info.clone() });
            InstNode::updateComponent(c.clone(), iterator.clone())?;
            (exp.clone(), ty.clone(), var.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeIterator")); __mm_s.push_str(&*literal!(" got non-iterator ")); __mm_s.push_str(&*InstNode::name(iterator.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outRange, ty, var, purity))
}

pub fn typeDimensions(mut dimensions: metamodelica::Array<Arc<Dimension::NFDimension>>, mut component: Arc<InstNode::InstNode>, mut binding: Arc<Binding::NFBinding>, mut context: i32, mut info: SourceInfo) -> Result<metamodelica::Array<Arc<Dimension::NFDimension>>> {
    let mut dimensions: metamodelica::Array<Arc<Dimension::NFDimension>> = dimensions;
    let __range0 = 1..=(dimensions.clone().borrow().len() as i32);
    for mut i in __range0 {
        typeDimension(dimensions.clone(), i.clone(), component.clone(), binding.clone(), context.clone(), info.clone())?;
    }
    Ok(dimensions)
}

pub fn typeDimension(mut dimensions: metamodelica::Array<Arc<Dimension::NFDimension>>, mut index: i32, mut component: Arc<InstNode::InstNode>, mut binding: Arc<Binding::NFBinding>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut dimension: Arc<Dimension::NFDimension> = dimensions.borrow()[(index.clone()-1) as usize].clone();
    dimension = (::match_deref::match_deref! { match &(dimension.clone()) {
        Deref @ Dimension::UNTYPED { isProcessing: true, .. } => {
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            if InstContext::inFunction(context.clone()) {
                dim = Arc::new(crate::NFDimension::UNKNOWN);
                {let _arr = dimensions.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = dim.clone(); _arr};
            } else {
                dim = dimension.clone();
            }
            dim.clone()
        },
        Deref @ Dimension::UNTYPED { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var: Variability = Variability::CONSTANT;
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut target: Arc<Ceval::EvalTarget::EvalTarget>;
            {let _arr = dimensions.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = Arc::new(Dimension::NFDimension::UNTYPED { dimension: var_field!((*dimension).dimension, Dimension::NFDimension::UNTYPED).clone(), isProcessing: true }); _arr};
            (exp, ty, var, _) = typeExp(var_field!((*dimension).dimension, Dimension::NFDimension::UNTYPED).clone(), InstContext::set(context.clone(), InstContext::DIMENSION.clone()), info.clone(), false)?;
            TypeCheck::checkDimensionType(exp.clone(), ty.clone(), info.clone())?;
            if !(InstContext::inFunction(context.clone())) {
                if var.clone() <= Variability::PARAMETER.clone() {
                    if InstContext::inRelaxed(context.clone()) {
                        exp = Ceval::tryEvalExp(exp.clone(), Ceval::noTarget().clone());
                    } else {
                        target = Ceval::EvalTarget::new(info.clone(), context.clone(), Some(Arc::new(Ceval::EvalTargetData { component: component.clone(), index: index.clone(), exp: exp.clone() })));
                        exp = Ceval::tryEvalExpResizable(exp.clone(), target.clone())?;
                    }
                } else if !(var.clone() == Variability::NON_STRUCTURAL_PARAMETER.clone()) {
                    Error::addSourceMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![(Expression::toString(exp.clone())?).clone()], info.clone())?;
                    bail!("fail");
                }
            } else {
                if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() && !(Expression::contains(exp.clone(), Arc::new(fnptr!(Expression::isFunctionInputCref, Arc<Expression::NFExpression>)))?) {
                    exp = Ceval::tryEvalExp(exp.clone(), Ceval::noTarget().clone());
                }
            }
            exp = subscriptDimExp(exp.clone(), component.clone())?;
            dim = Dimension::fromExp(exp.clone(), var.clone())?;
            {let _arr = dimensions.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = dim.clone(); _arr};
            dim.clone()
        },
        Deref @ Dimension::UNKNOWN if (InstContext::inFunction(context.clone()) && (Binding::isUnbound(binding.clone()) && InstNode::isOutput(component.clone()) || !(InstNode::isOutput(component.clone())))) => {
            dimension.clone()
        },
        Deref @ Dimension::UNKNOWN if (InstContext::inFunction(context.clone()) && Binding::hasExp(binding.clone()) && Expression::contains(Binding::getExp(binding.clone())?, Arc::new(fnptr!(Expression::isCref, Arc<Expression::NFExpression>)))?) => {
            dimension.clone()
        },
        Deref @ Dimension::UNKNOWN => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
            let mut ty_err: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
            let mut parent_dims: i32 = 0;
            let mut target: Arc<Ceval::EvalTarget::EvalTarget>;
            b = binding.clone();
            parent_dims = 0;
            {let _arr = dimensions.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = Arc::new(Dimension::NFDimension::UNTYPED { dimension: WHOLEDIM_CREF().clone(), isProcessing: true }); _arr};
            if Binding::isUnbound(binding.clone()) {
                (b, parent_dims) = getRecordElementBinding(component.clone(), context.clone())?;
                if Binding::isUnbound(b.clone()) {
                    parent_dims = 0;
                    b = Class::lookupAttributeBinding((literal!("start")).clone(), InstNode::getClass(component.clone())?);
                    b = Binding::mapExp(b.clone(), Arc::new({ let __pe_b1 = component.clone(); move |__pe_a0| Expression::filterSplitIndices(__pe_a0, __pe_b1.clone()) }))?;
                }
            }
            (dim, ty_err) = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Binding::UNBOUND if (!(InstContext::inRelaxed(context.clone()))) => {
            Error::addSourceMessage(Error::FAILURE_TO_DEDUCE_DIMS_NO_MOD.clone(), list![ArcStr::from(::std::format!("{}", index.clone())), (InstNode::name(component.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        Deref @ Binding::UNTYPED_BINDING { .. } => deduceDimensionFromExp(var_field!((*b).bindingExp, Binding::NFBinding::UNTYPED_BINDING).clone(), None, index.clone(), parent_dims.clone(), component.clone(), context.clone(), info.clone())?,
        Deref @ Binding::TYPED_BINDING { .. } => deduceDimensionFromExp(var_field!((*b).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), Some(var_field!((*b).bindingType, Binding::NFBinding::TYPED_BINDING).clone()), index.clone(), parent_dims.clone(), component.clone(), context.clone(), info.clone())?,
        _ => (dimension.clone(), Arc::new(crate::NFTyping::TypingError::NO_ERROR)),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            let () = (::match_deref::match_deref! { match &(ty_err.clone()) {
        Deref @ TypingError::OUT_OF_BOUNDS { .. } if (!(InstContext::inRelaxed(context.clone()))) => {
            Error::addSourceMessage(Error::DIMENSION_DEDUCTION_FROM_BINDING_FAILURE.clone(), list![ArcStr::from(::std::format!("{}", index.clone())), (InstNode::name(component.clone())?).clone(), (Binding::toString(b.clone(), (literal!("")).clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { exp, .. } => {
            let mut exp = (*exp).clone();
            Structural::markExp(exp.clone())?;
            if InstContext::inRelaxed(context.clone()) {
                exp = Ceval::tryEvalExp(exp.clone(), Ceval::noTarget().clone());
            } else {
                target = Ceval::EvalTarget::new(info.clone(), context.clone(), Some(Arc::new(Ceval::EvalTargetData { component: component.clone(), index: index.clone(), exp: exp.clone() })));
                exp = Ceval::evalExp(exp.clone(), target.clone())?;
            }
            exp = subscriptDimExp(exp.clone(), component.clone())?;
            Dimension::fromExp(exp.clone(), var_field!((*dim).var, Dimension::NFDimension::EXP).clone())?
        },
        Deref @ Dimension::UNKNOWN if (!(InstContext::inRelaxed(context.clone()))) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeDimension")); __mm_s.push_str(&*literal!(" returned unknown dimension in a non-function context")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
            bail!("fail")
        },
        _ => dim.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            {let _arr = dimensions.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = dim.clone(); _arr};
            dim.clone()
        },
        _ => {
            dimension.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dimension)
}

pub fn deduceDimensionFromExp(mut exp: Arc<Expression::NFExpression>, mut ty: Option<Arc<Type::NFType>>, mut index: i32, mut parentDims: i32, mut component: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut error: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    let mut oe: Option<Arc<Expression::NFExpression>> = None;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut dim_index: i32 = 0;
    dim_index = index.clone() + parentDims.clone();
    if isSome(ty.clone()) {
        (dim, error) = nthDimensionBoundsChecked(Util::getOption(ty.clone())?, dim_index.clone(), 0)?;
        oe = None;
    } else {
        (dim, oe, error) = typeExpDim(exp.clone(), dim_index.clone(), InstContext::set(context.clone(), InstContext::DIMENSION.clone()), info.clone())?;
    }
    if Dimension::isUnknown(dim.clone()) && !(TypingError::isError(error.clone())) {
        e = if (isSome(oe.clone())) {Util::getOption(oe.clone())?} else {exp.clone()};
        if InstContext::inRelaxed(context.clone()) {
            e = Ceval::tryEvalExp(e.clone(), Ceval::noTarget().clone());
        } else {
            e = Ceval::evalExp(e.clone(), Ceval::EvalTarget::new(info.clone(), context.clone(), Some(Arc::new(Ceval::EvalTargetData { component: component.clone(), index: index.clone(), exp: e.clone() }))))?;
        }
        (dim, error) = nthDimensionBoundsChecked(Expression::typeOf(e.clone()), dim_index.clone(), 0)?;
    }
    Ok((dim, error))
}

pub fn subscriptDimExp(mut dimExp: Arc<Expression::NFExpression>, mut component: Arc<InstNode::InstNode>) -> Result<Arc<Expression::NFExpression>> {
    let mut dimExp: Arc<Expression::NFExpression> = dimExp;
    let mut exp_dims: i32 = 0;
    let mut parent_dims: i32 = 0;
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    exp_dims = Expression::dimensionCount(dimExp.clone())?;
    if exp_dims.clone() == 0 {
        return Ok(dimExp);
    }
    subs = metamodelica::nil();
    parent = InstNode::instanceParent(component.clone());
    while exp_dims.clone() > 0 && !(InstNode::isEmpty(parent.clone())) {
        parent_dims = InstNode::dimensionCount(parent.clone());
        for mut i in (1..=parent_dims.clone()).rev() {
            subs = cons(Subscript::makeSplitIndex(parent.clone(), i.clone())?, subs.clone());
            exp_dims = exp_dims.clone() - 1;
            if exp_dims.clone() == 0 {
                break;
            }
        }
        parent = InstNode::instanceParent(parent.clone());
    }
    dimExp = Expression::applySubscripts(subs.clone(), dimExp.clone(), false)?;
    Ok(dimExp)
}

pub fn simplifyDimExp(mut dimExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut dimExp: Arc<Expression::NFExpression> = dimExp;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    dimExp = (::match_deref::match_deref! { match &(dimExp.clone()) {
        Deref @ Expression::ARRAY { .. } if (Expression::arrayAllEqual(dimExp.clone())?) => Expression::arrayFirstScalar(dimExp.clone())?,
        Deref @ Expression::SUBSCRIPTED_EXP { split: true, .. } if (Expression::isArray(var_field!((*dimExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone()) && Expression::arrayAllEqual(var_field!((*dimExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone())?) => Expression::arrayFirstScalar(var_field!((*dimExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone())?,
        _ => dimExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dimExp)
}

pub fn makeDimension(mut dimExp: Arc<Expression::NFExpression>, mut unevaledExp: Arc<Expression::NFExpression>, mut variability: Variability) -> Result<Arc<Dimension::NFDimension>> {
    let mut outDimension: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut exp: Arc<Expression::NFExpression> = dimExp.clone();
    if Expression::isArray(exp.clone()) {
        if Expression::arrayAllEqual(exp.clone())? {
            exp = Expression::arrayFirstScalar(exp.clone())?;
        }
    }
    outDimension = Dimension::fromExp(exp.clone(), variability.clone())?;
    Ok(outDimension)
}

pub fn getRecordElementBinding(mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<Binding::NFBinding>, i32)> {
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut parentDims: i32 = 0;
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut parent_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    parent = InstNode::instanceParent(component.clone());
    if InstNode::isComponent(parent.clone()) {
        comp = InstNode::component(parent.clone())?;
        parent_binding = Component::getBinding(comp.clone());
        if Binding::isUnbound(parent_binding.clone()) {
            (binding, parentDims) = getRecordElementBinding(parent.clone(), context.clone())?;
        } else {
            binding = typeBinding(parent_binding.clone(), InstContext::set(context.clone(), InstContext::DIMENSION.clone()))?;
            if !(referenceEq(&parent_binding.clone(),&binding.clone())) {
                InstNode::componentApply(parent.clone(), Arc::new(Component::setBinding), binding.clone())?;
            }
        }
        parentDims = parentDims.clone() + Component::dimensionCount(comp.clone());
        if Binding::isBound(binding.clone()) {
            binding = Binding::recordFieldBinding(component.clone(), binding.clone())?;
        }
    } else {
        binding = Binding::EMPTY_BINDING().clone();
    }
    Ok((binding, parentDims))
}

pub fn typeBindings(mut cls: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut c: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    c = InstNode::getClass(cls.clone())?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Class::INSTANCED_CLASS { elements: cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            let __range0 = var_field!((**cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentBinding(c.clone(), context.clone(), true)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { elements: cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            let __range0 = var_field!((**cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentBinding(c.clone(), context.clone(), true)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        Deref @ Class::TYPED_DERIVED { .. } => {
            typeBindings(var_field!((*c).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeBindings")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*InstNode::name(cls.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeComponentBinding(mut component: Arc<InstNode::InstNode>, mut context: i32, mut typeChildren: bool) -> Result<()> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut name: ArcStr = arcstr::literal!("");
    let mut comp_var: Variability = Variability::CONSTANT;
    let mut comp_eff_var: Variability = Variability::CONSTANT;
    let mut bind_var: Variability = Variability::CONSTANT;
    let mut bind_eff_var: Variability = Variability::CONSTANT;
    let mut attrs: Arc<Attributes::NFAttributes>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if InstNode::isEmpty(component.clone()) || InstNode::isOnlyOuter(component.clone())? {
        return Ok(());
    }
    node = InstNode::resolveOuter(component.clone());
    c = InstNode::component(node.clone())?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { .. } if (Component::isDeleted(c.clone())? || Component::isInvalid(c.clone())) => (),
        Deref @ Component::COMPONENT { attributes: attrs, binding: Deref @ Binding::UNTYPED_BINDING { .. }, .. } if (var_field!((*c).state, Component::NFComponent::COMPONENT).clone() == ComponentState::Typed.clone()) => {
            let mut attrs = (*attrs).clone();
            name = (InstNode::name(component.clone())?).clone();
            binding = var_field!((*c).binding, Component::NFComponent::COMPONENT).clone();
            ErrorExt::setCheckpoint((literal!("NFTyping.typeComponentBinding")).clone());
            match '__try0: {
                binding = unwrap_break_err!(typeBinding(binding.clone(), InstContext::set(context.clone(), InstContext::BINDING.clone())), '__try0);
                if !(InstContext::inAnnotation(context.clone()) && stringEq((name.clone()).clone(), (literal!("graphics")).clone()) || InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                    binding = unwrap_break_err!(TypeCheck::matchBinding(binding.clone(), var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(), (name.clone()).clone(), node.clone(), context.clone()), '__try0);
                }
                comp_var = unwrap_break_err!(checkComponentBindingVariability((name.clone()).clone(), c.clone(), binding.clone(), context.clone()), '__try0);
                if comp_var.clone() != attrs.variability.clone() {
                    assign_field!(attrs.variability = comp_var.clone());
                    assign_variant_field!(c => Component::NFComponent::COMPONENT; attributes = attrs.clone());
                }
                Ok::<_, anyhow::Error>((binding.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    binding = __try0_o0;
                }
                Err(_) => {
                    if Binding::isBound(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone()) || InstContext::inInstanceAPI(context.clone()) {
                        binding = Arc::new(Binding::NFBinding::INVALID_BINDING { binding: binding.clone(), errors: ErrorExt::getCheckpointMessages() });
                    } else {
                        ErrorExt::delCheckpoint((literal!("NFTyping.typeComponentBinding")).clone());
                        bail!("fail");
                    }
                }
            }
            ErrorExt::delCheckpoint((literal!("NFTyping.typeComponentBinding")).clone());
            assign_variant_field!(c => Component::NFComponent::COMPONENT;
                binding = binding.clone(),
                state = ComponentState::TypeChecked.clone()
            );
            InstNode::updateComponent(c.clone(), node.clone())?;
            if typeChildren.clone() && !(InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                typeBindings(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context.clone())?;
            }
            ()
        },
        Deref @ Component::COMPONENT { .. } if (var_field!((*c).state, Component::NFComponent::COMPONENT).clone() >= ComponentState::Typed.clone()) => {
            if var_field!((*c).state, Component::NFComponent::COMPONENT).clone() == ComponentState::Typed.clone() {
                if Binding::isTyped(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone()) {
                    assign_variant_field!(c => Component::NFComponent::COMPONENT; binding = TypeCheck::matchBinding(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(), (InstNode::name(component.clone())?).clone(), node.clone(), context.clone())?);
                    checkComponentBindingVariability((InstNode::name(component.clone())?).clone(), c.clone(), var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context.clone())?;
                }
                assign_variant_field!(c => Component::NFComponent::COMPONENT; state = ComponentState::TypeChecked.clone());
                InstNode::updateComponent(c.clone(), node.clone())?;
            }
            if typeChildren.clone() && !(InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                typeBindings(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context.clone())?;
            }
            ()
        },
        Deref @ Component::COMPONENT { attributes: attrs, binding: Deref @ Binding::UNTYPED_BINDING { .. }, .. } if (var_field!((*c).state, Component::NFComponent::COMPONENT).clone() < ComponentState::Typed.clone()) => {
            let mut attrs = (*attrs).clone();
            name = (InstNode::name(component.clone())?).clone();
            binding = typeBinding(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), InstContext::set(context.clone(), InstContext::BINDING.clone()))?;
            comp_var = checkComponentBindingVariability((name.clone()).clone(), c.clone(), binding.clone(), context.clone())?;
            if comp_var.clone() != attrs.variability.clone() {
                assign_field!(attrs.variability = comp_var.clone());
                assign_variant_field!(c => Component::NFComponent::COMPONENT; attributes = attrs.clone());
            }
            assign_variant_field!(c => Component::NFComponent::COMPONENT; binding = binding.clone());
            InstNode::updateComponent(c.clone(), node.clone())?;
            ()
        },
        Deref @ Component::COMPONENT { .. } => (),
        Deref @ Component::ENUM_LITERAL { .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { modifier: Deref @ Modifier::NOMOD, .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { .. } => {
            assign_variant_field!(c => Component::NFComponent::TYPE_ATTRIBUTE; modifier = typeTypeAttribute(var_field!((*c).modifier, Component::NFComponent::TYPE_ATTRIBUTE).clone(), var_field!((*c).ty, Component::NFComponent::TYPE_ATTRIBUTE).clone(), component.clone(), context.clone())?);
            InstNode::updateComponent(c.clone(), node.clone())?;
            ()
        },
        Deref @ Component::INVALID_COMPONENT { .. } => (),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponentBinding")); __mm_s.push_str(&*literal!(" got invalid node ")); __mm_s.push_str(&*InstNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkComponentBindingVariability(mut name: ArcStr, mut component: Arc<Component::NFComponent>, mut binding: Arc<Binding::NFBinding>, mut context: i32) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    let mut comp_var: Variability = Variability::CONSTANT;
    let mut comp_eff_var: Variability = Variability::CONSTANT;
    let mut bind_var: Variability = Variability::CONSTANT;
    let mut bind_eff_var: Variability = Variability::CONSTANT;
    comp_var = Component::variability(component.clone());
    comp_eff_var = Prefixes::effectiveVariability(comp_var.clone());
    bind_var = Binding::variability(binding.clone())?;
    bind_eff_var = Prefixes::effectiveVariability(bind_var.clone());
    if bind_eff_var.clone() > comp_eff_var.clone() && !(InstContext::inFunction(context.clone())) {
        Error::addSourceMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![(name.clone()).clone(), (Prefixes::variabilityString(comp_eff_var.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Binding::toString(Component::getBinding(component.clone()), (literal!("")).clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (Prefixes::variabilityString(bind_eff_var.clone())?).clone()], Binding::getInfo(binding.clone()))?;
        if !(InstContext::inRelaxed(context.clone())) {
            bail!("fail");
        }
    }
    if comp_var.clone() == Variability::PARAMETER.clone() && (bind_var.clone() == Variability::STRUCTURAL_PARAMETER.clone() && Binding::isCrefExp(binding.clone()) || bind_var.clone() == Variability::NON_STRUCTURAL_PARAMETER.clone()) {
        var = bind_var.clone();
    } else {
        var = comp_var.clone();
    }
    Ok(var)
}

pub fn typeBinding(mut binding: Arc<Binding::NFBinding>, mut context: i32) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    binding = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::UNTYPED_BINDING { bindingExp: exp, .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut var: Variability = Variability::CONSTANT;
            let mut purity: Purity = Purity::PURE;
            let mut info: SourceInfo;
            let mut exp = (*exp).clone();
            info = Binding::getInfo(binding.clone());
            (exp, ty, var, purity) = typeExp(exp.clone(), context.clone(), info.clone(), false)?;
            Arc::new(Binding::NFBinding::TYPED_BINDING { bindingExp: exp.clone(), bindingType: ty.clone(), variability: var.clone(), purity: purity.clone(), eachType: var_field!((*binding).eachType, Binding::NFBinding::UNTYPED_BINDING).clone(), evalState: Mutable::create(Binding::EvalState::NOT_EVALUATED.clone()), isFlattened: false, source: var_field!((*binding).source, Binding::NFBinding::UNTYPED_BINDING).clone(), info: var_field!((*binding).info, Binding::NFBinding::UNTYPED_BINDING).clone() })
        },
        Deref @ Binding::TYPED_BINDING { .. } => {
            binding.clone()
        },
        Deref @ Binding::UNBOUND => {
            binding.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeBinding")); __mm_s.push_str(&*literal!(" got uninstantiated binding")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

pub fn typeComponentCondition(mut condition: Arc<Binding::NFBinding>, mut context: i32, mut evaluate: bool) -> Result<Arc<Binding::NFBinding>> {
    let mut condition: Arc<Binding::NFBinding> = condition;
    condition = (::match_deref::match_deref! { match &(condition.clone()) {
        Deref @ Binding::UNTYPED_BINDING { bindingExp: exp, .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut var: Variability = Variability::CONSTANT;
            let mut purity: Purity = Purity::PURE;
            let mut info: SourceInfo;
            let mut mk: MatchKind = MatchKind::EXACT;
            let mut eval_state: Binding::EvalState = Binding::EvalState::NOT_EVALUATED;
            let mut next_context: i32 = 0;
            let mut exp = (*exp).clone();
            next_context = InstContext::set(context.clone(), InstContext::CONDITION.clone());
            info = Binding::getInfo(condition.clone());
            (exp, ty, var, purity) = typeExp(exp.clone(), next_context.clone(), info.clone(), false)?;
            (exp, _, mk) = TypeCheck::matchTypes(ty.clone(), Arc::new(crate::NFType::BOOLEAN), exp.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isIncompatibleMatch(mk.clone()) {
                Error::addSourceMessage(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(Expression::toString(exp.clone())?).clone(), (Type::toString(ty.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            if var.clone() > Variability::PARAMETER.clone() {
                Error::addSourceMessage(Error::COMPONENT_CONDITION_VARIABILITY.clone(), list![(Expression::toString(exp.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            eval_state = Binding::EvalState::NOT_EVALUATED.clone();
            if evaluate.clone() {
                ErrorExt::setCheckpoint((literal!("NFTyping.typeComponentCondition")).clone());
                if '__try0: {
                    exp = unwrap_break_err!(Ceval::evalExp(exp.clone(), Ceval::EvalTarget::new(info.clone(), next_context.clone(), None)), '__try0);
                    exp = unwrap_break_err!(simplifyDimExp(exp.clone()), '__try0);
                    eval_state = Binding::EvalState::EVALUATED.clone();
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
                ErrorExt::rollBack((literal!("NFTyping.typeComponentCondition")).clone());
            }
            Arc::new(Binding::NFBinding::TYPED_BINDING { bindingExp: exp.clone(), bindingType: ty.clone(), variability: var.clone(), purity: purity.clone(), eachType: Binding::EachType::NOT_EACH.clone(), evalState: Mutable::create(eval_state.clone()), isFlattened: false, source: var_field!((*condition).source, Binding::NFBinding::UNTYPED_BINDING).clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(condition)
}

pub fn typeTypeAttribute(mut attribute: Arc<Modifier::Modifier>, mut attrType: Arc<Type::NFType>, mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Modifier::Modifier>> {
    let mut attribute: Arc<Modifier::Modifier> = attribute;
    let mut name: ArcStr = arcstr::literal!("");
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    attribute = (::match_deref::match_deref! { match &(attribute.clone()) {
        Deref @ Modifier::MODIFIER { .. } if (!(ModTable::isEmpty(var_field!((*attribute).subModifiers, Modifier::Modifier::MODIFIER).clone()))) => {
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*attribute).name, Modifier::Modifier::MODIFIER).clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Util::tuple21(listHead(ModTable::toList(var_field!((*attribute).subModifiers, Modifier::Modifier::MODIFIER).clone(), metamodelica::nil()))?)); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(name.clone()).clone(), (Type::toString(attrType.clone())?).clone()], var_field!((*attribute).info, Modifier::Modifier::MODIFIER).clone())?;
            bail!("fail")
        },
        Deref @ Modifier::MODIFIER { .. } if (Binding::isUnbound(var_field!((*attribute).binding, Modifier::Modifier::MODIFIER).clone())) => Arc::new(crate::NFModifier::Modifier::NOMOD),
        Deref @ Modifier::MODIFIER { binding: Deref @ Binding::TYPED_BINDING { .. }, .. } => attribute.clone(),
        Deref @ Modifier::MODIFIER { binding, name, .. } => {
            let mut binding = (*binding).clone();
            if Binding::isBound(binding.clone()) {
                binding = typeBinding(binding.clone(), context.clone())?;
                parent = InstNode::parent(component.clone());
                binding = TypeCheck::matchBinding(binding.clone(), attrType.clone(), (name.clone()).clone(), parent.clone(), context.clone())?;
                if Binding::variability(binding.clone())? >= Variability::DISCRETE.clone() && !(InstContext::inFunction(context.clone())) {
                    Error::addSourceMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![(name.clone()).clone(), (Prefixes::variabilityString(Variability::PARAMETER.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Binding::toString(binding.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (Prefixes::variabilityString(Binding::variability(binding.clone())?)?).clone()], Binding::getInfo(binding.clone()))?;
                    bail!("fail");
                }
                assign_variant_field!(attribute => Modifier::Modifier::MODIFIER; binding = binding.clone());
            }
            attribute.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(attribute)
}

pub fn typeExp(mut exp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    (exp, ty, variability, purity) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            (exp.clone(), Arc::new(crate::NFType::INTEGER), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::REAL { .. } => {
            (exp.clone(), Arc::new(crate::NFType::REAL), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::STRING { .. } => {
            (exp.clone(), Arc::new(crate::NFType::STRING), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::BOOLEAN { .. } => {
            (exp.clone(), Arc::new(crate::NFType::BOOLEAN), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::ENUM_LITERAL { .. } => {
            (exp.clone(), var_field!((*exp).ty, Expression::NFExpression::ENUM_LITERAL).clone(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::CREF { .. } => {
            typeCrefExp(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::TYPENAME { .. } => {
            if !(InstContext::inValidTypenameScope(context.clone())) {
                Error::addSourceMessage(Error::INVALID_TYPENAME_USE.clone(), list![(Type::typenameString(Type::arrayElementType(var_field!((*exp).ty, Expression::NFExpression::TYPENAME).clone()))?).clone()], info.clone())?;
                bail!("fail");
            }
            (exp.clone(), var_field!((*exp).ty, Expression::NFExpression::TYPENAME).clone(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::ARRAY { .. } => {
            typeArray(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone(), var_field!((*exp).ty, Expression::NFExpression::ARRAY).clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::MATRIX { .. } => {
            typeMatrix(var_field!((*exp).elements, Expression::NFExpression::MATRIX).clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::RANGE { .. } => {
            typeRange(exp.clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::TUPLE { .. } => {
            typeTuple(var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::SIZE { .. } => {
            typeSize(exp.clone(), context.clone(), info.clone(), true)?
        },
        Deref @ Expression::END => {
            Error::addSourceMessage(Error::END_ILLEGAL_USE_ERROR.clone(), metamodelica::nil(), info.clone())?;
            bail!("fail")
        },
        Deref @ Expression::BINARY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var1: Variability = Variability::CONSTANT;
            let mut var2: Variability = Variability::CONSTANT;
            let mut pur1: Purity = Purity::PURE;
            let mut pur2: Purity = Purity::PURE;
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut next_context: i32 = 0;
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), next_context.clone(), info.clone(), false)?;
            (e2, ty2, var2, pur2) = typeExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), next_context.clone(), info.clone(), false)?;
            (exp, ty) = TypeCheck::checkBinaryOperation(e1.clone(), ty1.clone(), var1.clone(), var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), e2.clone(), ty2.clone(), var2.clone(), context.clone(), info.clone(), retype.clone())?;
            (exp.clone(), ty.clone(), Prefixes::variabilityMax(var1.clone(), var2.clone()), Prefixes::purityMin(pur1.clone(), pur2.clone()))
        },
        Deref @ Expression::UNARY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var1: Variability = Variability::CONSTANT;
            let mut pur1: Purity = Purity::PURE;
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut next_context: i32 = 0;
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), next_context.clone(), info.clone(), false)?;
            (exp, ty) = TypeCheck::checkUnaryOperation(e1.clone(), ty1.clone(), var1.clone(), var_field!((*exp).operator, Expression::NFExpression::UNARY).clone(), context.clone(), info.clone())?;
            (exp.clone(), ty.clone(), var1.clone(), pur1.clone())
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var1: Variability = Variability::CONSTANT;
            let mut var2: Variability = Variability::CONSTANT;
            let mut pur1: Purity = Purity::PURE;
            let mut pur2: Purity = Purity::PURE;
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut next_context: i32 = 0;
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), next_context.clone(), info.clone(), false)?;
            (e2, ty2, var2, pur2) = typeExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), next_context.clone(), info.clone(), false)?;
            (exp, ty) = TypeCheck::checkLogicalBinaryOperation(e1.clone(), ty1.clone(), var1.clone(), var_field!((*exp).operator, Expression::NFExpression::LBINARY).clone(), e2.clone(), ty2.clone(), var2.clone(), context.clone(), info.clone())?;
            (exp.clone(), ty.clone(), Prefixes::variabilityMax(var1.clone(), var2.clone()), Prefixes::purityMin(pur1.clone(), pur2.clone()))
        },
        Deref @ Expression::LUNARY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var1: Variability = Variability::CONSTANT;
            let mut pur1: Purity = Purity::PURE;
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut next_context: i32 = 0;
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), next_context.clone(), info.clone(), false)?;
            (exp, ty) = TypeCheck::checkLogicalUnaryOperation(e1.clone(), ty1.clone(), var1.clone(), var_field!((*exp).operator, Expression::NFExpression::LUNARY).clone(), context.clone(), info.clone())?;
            (exp.clone(), ty.clone(), var1.clone(), pur1.clone())
        },
        Deref @ Expression::RELATION { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var1: Variability = Variability::CONSTANT;
            let mut var2: Variability = Variability::CONSTANT;
            let mut pur1: Purity = Purity::PURE;
            let mut pur2: Purity = Purity::PURE;
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut next_context: i32 = 0;
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), next_context.clone(), info.clone(), false)?;
            (e2, ty2, var2, pur2) = typeExp(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), next_context.clone(), info.clone(), false)?;
            (exp, ty) = TypeCheck::checkRelationOperation(e1.clone(), ty1.clone(), var1.clone(), var_field!((*exp).operator, Expression::NFExpression::RELATION).clone(), e2.clone(), ty2.clone(), var2.clone(), var_field!((*exp).index, Expression::NFExpression::RELATION).clone(), context.clone(), info.clone())?;
            variability = Prefixes::variabilityMax(var1.clone(), var2.clone());
            purity = Prefixes::purityMin(pur1.clone(), pur2.clone());
            if !(InstContext::inNoEvent(context.clone())) && variability.clone() == Variability::CONTINUOUS.clone() {
                variability = Variability::DISCRETE.clone();
            }
            (exp.clone(), ty.clone(), variability.clone(), purity.clone())
        },
        Deref @ Expression::IF { .. } => {
            typeIfExpression(exp.clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::RECORD { .. } => {
            typeRecordExp(exp.clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::CALL { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var1: Variability = Variability::CONSTANT;
            let mut pur1: Purity = Purity::PURE;
            (e1, ty, var1, pur1) = Call::typeCall(exp.clone(), context.clone(), info.clone(), retype.clone())?;
            if Type::isTuple(ty.clone()) && !(InstContext::isSingleExpression(context.clone())) {
                ty = Type::firstTupleType(ty.clone())?;
                e1 = Expression::tupleElement(e1.clone(), ty.clone(), 1)?;
            }
            (e1.clone(), ty.clone(), var1.clone(), pur1.clone())
        },
        Deref @ Expression::CAST { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut next_context: i32 = 0;
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            (e1, ty, variability, purity) = typeExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), next_context.clone(), info.clone(), retype.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::CAST;
                exp = e1.clone(),
                ty = Type::copyDims(ty.clone(), var_field!((*exp).ty, Expression::NFExpression::CAST).clone())
            );
            (exp.clone(), var_field!((*exp).ty, Expression::NFExpression::CAST).clone(), variability.clone(), purity.clone())
        },
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            typeSubscriptedExp(exp.clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::MUTABLE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone());
            (e1, ty, variability, purity) = typeExp(e1.clone(), context.clone(), info.clone(), retype.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::MUTABLE; exp = Mutable::create(e1.clone()));
            (exp.clone(), ty.clone(), variability.clone(), purity.clone())
        },
        Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. } => {
            Function::typePartialApplication(exp.clone(), context.clone(), info.clone())?
        },
        Deref @ Expression::FILENAME { .. } => {
            (exp.clone(), Arc::new(crate::NFType::STRING), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::MULTARY { .. } => {
            typeExp(SimplifyExp::splitMultary(exp.clone())?, context.clone(), info.clone(), retype.clone())?
        },
        _ => {
            (exp.clone(), Expression::typeOf(exp.clone()), Expression::variability(exp.clone())?, Expression::purity(exp.clone())?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if InstContext::inDiscreteScope(context.clone()) && variability.clone() == Variability::CONTINUOUS.clone() {
        variability = Variability::DISCRETE.clone();
    }
    Ok((exp, ty, variability, purity))
}

pub fn typeExpl(mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<metamodelica::List<Arc<Type::NFType>>>, Arc<metamodelica::List<Variability>>)> {
    let mut explTyped: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut tyl: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut varl: Arc<metamodelica::List<Variability>> = metamodelica::nil();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Variability = Variability::CONSTANT;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    for mut e in &*expl.clone().reverse() {
        let mut e = e.clone();
        (exp, ty, var, _) = typeExp(e.clone(), context.clone(), info.clone(), false)?;
        explTyped = cons(exp.clone(), explTyped.clone());
        tyl = cons(ty.clone(), tyl.clone());
        varl = cons(var.clone(), varl.clone());
    }
    Ok((explTyped, tyl, varl))
}

pub fn typeRecordExp(mut exp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut path: Arc<Absyn::Path>;
    let mut elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty_elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut next_context: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD { path: __pa0, ty: __pa1, elements: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    ty = __pa1.clone();
    elems = __pa2.clone();
    next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
    for mut e in &*elems.clone() {
        let mut e = e.clone();
        (e, _, var, pur) = typeExp(e.clone(), context.clone(), info.clone(), false)?;
        variability = Prefixes::variabilityMax(var.clone(), variability.clone());
        purity = Prefixes::purityMin(pur.clone(), purity.clone());
        ty_elems = cons(e.clone(), ty_elems.clone());
    }
    exp = Expression::makeRecord(path.clone(), ty.clone(), ty_elems.clone().reverse());
    Ok((exp, ty, variability, purity))
}

pub fn typeSubscriptedExp(mut exp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut expanded_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut fill_dims: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut split: bool = false;
    let mut subs_var: Variability = Variability::CONSTANT;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { exp: __pa0, subscripts: __pa1, ty: __pa2, split: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    subs = __pa1.clone();
    ty = __pa2.clone();
    split = __pa3.clone();
    if split.clone() {
        (expanded_subs, fill_dims) = expandProxySubscripts(subs.clone(), context.clone())?;
        (exp, ty, variability, purity) = typeSubscriptedExp2(e.clone(), expanded_subs.clone(), context.clone(), info.clone())?;
        if !(fill_dims.clone().is_empty()) {
            fill_dims = fill_dims.clone().reverse();
            ty = Type::liftArrayLeftList(ty.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (fill_dims.clone()).into_iter().cloned() {
            let __x = Dimension::fromExp(d.clone(), Variability::CONSTANT.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FILL_FUNC().clone(), cons(exp.clone(), fill_dims.clone()), variability.clone(), purity.clone(), ty.clone()) });
        }
        if !(expanded_subs.clone().is_empty()) {
            ty = Type::subscript(ty.clone(), expanded_subs.clone(), false)?;
            if Type::isUnknown(ty.clone()) {
                exp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: exp.clone(), subscripts: expanded_subs.clone(), ty: ty.clone(), split: true });
            } else {
                exp = Expression::applySubscripts(expanded_subs.clone(), exp.clone(), false)?;
            }
            if purity.clone() == Purity::PURE.clone() {
                purity = Subscript::purityList(expanded_subs.clone())?;
            }
            if variability.clone() != Variability::CONTINUOUS.clone() {
                variability = Prefixes::variabilityMax(variability.clone(), Subscript::variabilityList(expanded_subs.clone())?);
            }
        }
    } else {
        (e, ty, variability, purity) = typeExp(e.clone(), context.clone(), info.clone(), false)?;
        (subs, subs_var) = typeSubscripts(subs.clone(), ty.clone(), exp.clone(), context.clone(), info.clone(), true)?;
        ty = Type::subscript(ty.clone(), subs.clone(), true)?;
        exp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: e.clone(), subscripts: subs.clone(), ty: ty.clone(), split: false });
    }
    Ok((exp, ty, variability, purity))
}

pub fn expandProxySubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut context: i32) -> Result<(Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>)> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut fillDimensions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut dim_count: i32 = 0;
    let mut cr_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        outSubscripts = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Subscript::SPLIT_PROXY { .. } => {
            dim_count = InstNode::dimensionCount(var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone());
            for mut i in 1..=dim_count.clone() {
                outSubscripts = cons(Subscript::makeSplitIndex(var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone(), i.clone())?, outSubscripts.clone());
            }
            if !(InstNode::refEqual(var_field!((*s).origin, Subscript::NFSubscript::SPLIT_PROXY).clone(), var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone())) {
                dim_count = dim_count.clone() - InstNode::dimensionCount(var_field!((*s).origin, Subscript::NFSubscript::SPLIT_PROXY).clone());
                if dim_count.clone() > 0 {
                    ty = InstNode::getType(var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone())?;
                    cr_exp = Expression::fromCref(ComponentRef::fromNode(var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone()), false)?;
                    dims = Type::arrayDims(ty.clone());
                    for mut i in 1..=dim_count.clone() {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        dim = __pa0.clone();
                        dims = __pa1.clone();
                        if Dimension::isKnown(dim.clone(), true) {
                            fillDimensions = cons(Dimension::sizeExp(dim.clone())?, fillDimensions.clone());
                        } else {
                            fillDimensions = cons(Arc::new(Expression::NFExpression::SIZE { exp: cr_exp.clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: i.clone() })) }), fillDimensions.clone());
                        }
                    }
                }
            }
            outSubscripts.clone()
        },
        _ => cons(s.clone(), outSubscripts.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outSubscripts = List::trim(outSubscripts.clone(), Arc::new(fnptr!(Subscript::isWhole, Arc<Subscript::NFSubscript>)))?;
    outSubscripts = outSubscripts.clone().reverse();
    Ok((outSubscripts, fillDimensions))
}

pub fn typeSubscriptedExp2(mut exp: Arc<Expression::NFExpression>, mut splitSubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    (outExp, ty, variability, purity) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { .. } if (!(splitSubs.clone().is_empty()) && !(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().is_empty())) => {
            expl = metamodelica::nil();
            variability = Variability::CONSTANT.clone();
            purity = Purity::PURE.clone();
            let __range0 = var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range0 {
                (e, ty, variability, purity) = typeSubscriptedExp2(e.clone(), listRest(splitSubs.clone())?, context.clone(), info.clone())?;
                expl = cons(e.clone(), expl.clone());
            }
            expl = expl.clone().reverse();
            ty = Type::liftArrayLeft(ty.clone(), Dimension::fromInteger((expl.clone().len() as i32), Prefixes::Variability::CONSTANT.clone()));
            outExp = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(expl.clone().into_iter().cloned().collect()), var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone());
            (outExp.clone(), ty.clone(), variability.clone(), purity.clone())
        },
        _ => typeExp(exp.clone(), context.clone(), info.clone(), false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, ty, variability, purity))
}

pub fn typeExpDim(mut exp: Arc<Expression::NFExpression>, mut dimIndex: i32, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Dimension::NFDimension>, Option<Arc<Expression::NFExpression>>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut typedExp: Option<Arc<Expression::NFExpression>> = None;
    let mut error: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut next_context: i32 = 0;
    ty = Expression::typeOf(exp.clone());
    if Type::isKnown(ty.clone()) {
        (dim, error) = nthDimensionBoundsChecked(ty.clone(), dimIndex.clone(), 0)?;
        typedExp = Some(exp.clone());
        if !(Dimension::isUnknown(dim.clone())) {
            return Ok((dim, typedExp, error));
        }
    }
    next_context = InstContext::clearExpFlags(context.clone());
    (dim, error) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::UNKNOWN, .. } => typeArrayDim(exp.clone(), dimIndex.clone())?,
        Deref @ Expression::CREF { .. } => typeCrefDim(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), dimIndex.clone(), next_context.clone(), info.clone())?,
        _ => {
            (e, ty, _, _) = typeExp(exp.clone(), next_context.clone(), info.clone(), false)?;
            if Type::isTuple(ty.clone()) {
                ty = Type::firstTupleType(ty.clone())?;
                e = Expression::tupleElement(e.clone(), ty.clone(), 1)?;
            }
            if Type::isConditionalArray(ty.clone()) {
                e = Expression::map(e.clone(), Arc::new({ let __pe_b1 = Ceval::EvalTarget::new(info.clone(), next_context.clone(), None); move |__pe_a0| evaluateArrayIf(__pe_a0, __pe_b1.clone()) }))?;
                (e, ty, _, _) = typeExp(e.clone(), next_context.clone(), info.clone(), false)?;
            }
            typedExp = Some(e.clone());
            nthDimensionBoundsChecked(ty.clone(), dimIndex.clone(), 0)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((dim, typedExp, error))
}

pub fn evaluateArrayIf(mut exp: Arc<Expression::NFExpression>, mut target: Arc<Ceval::EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::IF { .. } if (Type::isConditionalArray(var_field!((*exp).ty, Expression::NFExpression::IF).clone())) => {
            let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            cond = Ceval::evalExp(var_field!((*exp).condition, Expression::NFExpression::IF).clone(), target.clone())?;
            if Expression::isTrue(cond.clone()) {
                outExp = var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone();
            } else if Expression::isFalse(cond.clone()) {
                outExp = var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone();
            } else {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.evaluateArrayIf")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), Ceval::EvalTarget::getInfo(target.clone()))?;
                bail!("fail");
            }
            outExp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn typeArrayDim(mut arrayExp: Arc<Expression::NFExpression>, mut dimIndex: i32) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut error: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    if dimIndex.clone() < 1 {
        dim = Arc::new(crate::NFDimension::UNKNOWN);
        error = Arc::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: Expression::dimensionCount(arrayExp.clone())? });
    } else {
        (dim, error) = typeArrayDim2(arrayExp.clone(), dimIndex.clone(), 0)?;
    }
    Ok((dim, error))
}

pub fn typeArrayDim2(mut arrayExp: Arc<Expression::NFExpression>, mut dimIndex: i32, mut dimCount: i32) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut error: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    (dim, error) = (::match_deref::match_deref! { match &((arrayExp.clone(), dimIndex.clone())) {
        (Deref @ Expression::ARRAY { .. }, 1) => (Dimension::fromExpArray(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone()), Arc::new(crate::NFTyping::TypingError::NO_ERROR)),
        (Deref @ Expression::ARRAY { .. }, _) => typeArrayDim2(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone().borrow()[(1-1) as usize].clone(), dimIndex.clone() - 1, dimCount.clone() + 1)?,
        _ => {
            dim = Arc::new(crate::NFDimension::UNKNOWN);
            error = Arc::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: dimCount.clone() });
            (dim.clone(), error.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((dim, error))
}

pub fn typeCrefDim(mut cref: Arc<ComponentRef::NFComponentRef>, mut dimIndex: i32, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut error: Arc<TypingError::TypingError> = Arc::new(crate::NFTyping::TypingError::NO_ERROR);
    let mut crl: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut dim_count: i32 = 0;
    let mut dim_total: i32 = 0;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims: metamodelica::Array<Arc<Dimension::NFDimension>>;
    if ComponentRef::hasSubscripts(cref.clone()) {
        (_, ty, _, _) = typeCref(cref.clone(), context.clone(), info.clone())?;
        (dim, error) = nthDimensionBoundsChecked(ty.clone(), dimIndex.clone(), 0)?;
        return Ok((dim, error));
    }
    crl = ComponentRef::toListReverse(cref.clone(), false, metamodelica::nil());
    index = dimIndex.clone();
    for mut cr in &*crl.clone() {
        let mut cr = cr.clone();
        let () = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ ComponentRef::CREF { subscripts: subs, node: Deref @ InstNode::COMPONENT_NODE { .. }, .. } => {
            node = InstNode::resolveOuter(var_field!((*cr).node, ComponentRef::NFComponentRef::CREF).clone());
            c = InstNode::component(node.clone())?;
            if Class::hasDimensions(InstNode::getClass(Component::classInstance(c.clone()))?)? {
                typeComponent(node.clone(), context.clone(), true)?;
                c = InstNode::component(node.clone())?;
            }
            dim_count = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { ty: Deref @ Type::UNTYPED { dimensions: dims, .. }, .. } => {
            dim_count = (dims.clone().borrow().len() as i32);
            if index.clone() <= dim_count.clone() && index.clone() > 0 {
                dim = typeDimension(dims.clone(), index.clone(), node.clone(), var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context.clone(), var_field!((*c).info, Component::NFComponent::COMPONENT).clone())?;
                checkCyclicDimension(dim.clone(), node.clone(), index.clone(), var_field!((*c).info, Component::NFComponent::COMPONENT).clone())?;
                return Ok((dim, error));
            }
            dim_count.clone()
        },
        Deref @ Component::COMPONENT { .. } => {
            dim_count = Type::dimensionCount(var_field!((*c).ty, Component::NFComponent::COMPONENT).clone());
            if index.clone() <= dim_count.clone() && index.clone() > 0 {
                dim = Type::nthDimension(var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(), index.clone())?;
                return Ok((dim, error));
            }
            dim_count.clone()
        },
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            index = index.clone() - dim_count.clone();
            dim_total = dim_total.clone() + dim_count.clone();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    dim = Arc::new(crate::NFDimension::UNKNOWN);
    error = Arc::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: dim_total.clone() });
    Ok((dim, error))
}

pub fn checkCyclicDimension(mut dim: Arc<Dimension::NFDimension>, mut component: Arc<InstNode::InstNode>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let mut dim_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let () = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::UNTYPED { isProcessing: true, .. } => {
            Error::addSourceMessage(Error::CYCLIC_DIMENSIONS.clone(), list![ArcStr::from(::std::format!("{}", index.clone())), (InstNode::name(component.clone())?).clone(), (Expression::toString(var_field!((*dim).dimension, Dimension::NFDimension::UNTYPED).clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn nthDimensionBoundsChecked(mut ty: Arc<Type::NFType>, mut dimIndex: i32, mut offset: i32) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut error: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    let mut dim_size: i32 = Type::dimensionCount(ty.clone());
    let mut index: i32 = dimIndex.clone() + offset.clone();
    if index.clone() < 1 || index.clone() > dim_size.clone() {
        dim = Arc::new(crate::NFDimension::UNKNOWN);
        error = Arc::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: dim_size.clone() - offset.clone() });
    } else {
        dim = Type::nthDimension(ty.clone(), index.clone())?;
        error = Arc::new(crate::NFTyping::TypingError::NO_ERROR);
    }
    Ok((dim, error))
}

pub fn typeCrefExp(mut cref: Arc<ComponentRef::NFComponentRef>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node_var: Variability = Variability::CONSTANT;
    let mut subs_var: Variability = Variability::CONSTANT;
    let mut eval: bool = false;
    (cr, ty, node_var, subs_var) = typeCref(cref.clone(), context.clone(), info.clone())?;
    exp = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: cr.clone() });
    variability = Prefixes::variabilityMax(node_var.clone(), subs_var.clone());
    purity = ComponentRef::purity(cref.clone());
    Ok((exp, ty, variability, purity))
}

pub fn typeCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<Type::NFType>, Variability, Variability)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut nodeVariability: Variability = Variability::CONSTANT;
    let mut subsVariability: Variability = Variability::CONSTANT;
    let mut subs_var: Variability = Variability::CONSTANT;
    if InstContext::inFunction(context.clone()) && ComponentRef::isTime(cref.clone()) {
        Error::addSourceMessage(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(literal!("time")).clone()], info.clone())?;
        bail!("fail");
    }
    (cref, subsVariability) = typeCref2(cref.clone(), context.clone(), info.clone(), true)?;
    if ComponentRef::hasImplicitTrailingIndex(cref.clone()) {
        cref = ComponentRef::fillSubscripts(cref.clone());
    }
    ty = ComponentRef::getSubscriptedType(cref.clone(), false)?;
    nodeVariability = ComponentRef::nodeVariability(cref.clone())?;
    Ok((cref, ty, nodeVariability, subsVariability))
}

pub fn typeCref2(mut cref: Arc<ComponentRef::NFComponentRef>, mut context: i32, mut info: SourceInfo, mut firstPart: bool) -> Result<(Arc<ComponentRef::NFComponentRef>, Variability)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut subsVariability: Variability = Variability::CONSTANT;
    (cref, subsVariability) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::SCOPE, .. } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF;
                ty = InstNode::getType(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?,
                restCref = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context.clone(), info.clone(), false)?.0
            );
            (cref.clone(), Variability::CONSTANT.clone())
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::COMPONENT_NODE { .. }, .. } => {
            let mut rest_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut node_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            let mut subs_var: Variability = Variability::CONSTANT;
            let mut rest_var: Variability = Variability::CONSTANT;
            node_ty = typeComponent(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), InstContext::nodeContext(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), context.clone()), firstPart.clone() || !(InstContext::inDimension(context.clone())))?;
            (subs, subs_var) = typeSubscripts(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone(), node_ty.clone(), Arc::new(Expression::NFExpression::CREF { ty: node_ty.clone(), cref: cref.clone() }), context.clone(), info.clone(), true)?;
            (rest_cr, rest_var) = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context.clone(), info.clone(), false)?;
            subsVariability = Prefixes::variabilityMax(subs_var.clone(), rest_var.clone());
            (Arc::new(ComponentRef::NFComponentRef::CREF { node: var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: node_ty.clone(), origin: var_field!((*cref).origin, ComponentRef::NFComponentRef::CREF).clone(), restCref: rest_cr.clone() }), subsVariability.clone())
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::CLASS_NODE { .. }, .. } if (firstPart.clone() && InstNode::isFunction(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?) => {
            let mut r#fn: Arc<Function::Function>;
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF;
                ty = Arc::new(Type::NFType::FUNCTION { r#fn: r#fn.clone(), fnType: Type::FunctionType::FUNCTION_REFERENCE.clone() }),
                restCref = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context.clone(), info.clone(), false)?.0
            );
            (cref.clone(), Variability::CONSTANT.clone())
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::CLASS_NODE { .. }, .. } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; ty = InstNode::getType(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?);
            (cref.clone(), Variability::CONSTANT.clone())
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::NAME_NODE { .. }, .. } => {
            let mut rest_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut subs_var: Variability = Variability::CONSTANT;
            let mut rest_var: Variability = Variability::CONSTANT;
            (_, subs_var) = typeSubscripts(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone(), var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone(), Arc::new(Expression::NFExpression::CREF { ty: var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone(), cref: cref.clone() }), context.clone(), info.clone(), false)?;
            (rest_cr, rest_var) = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context.clone(), info.clone(), false)?;
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = rest_cr.clone());
            subsVariability = Prefixes::variabilityMax(subs_var.clone(), rest_var.clone());
            (cref.clone(), rest_var.clone())
        },
        _ => {
            (cref.clone(), Variability::CONSTANT.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, subsVariability))
}

pub fn typeSubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut crefType: Arc<Type::NFType>, mut subscriptedExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo, mut checkSubscripts: bool) -> Result<(Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, Variability)> {
    let mut typedSubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut next_context: i32 = 0;
    let mut i: i32 = 0;
    let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut var: Variability = Variability::CONSTANT;
    if subscripts.clone().is_empty() {
        typedSubs = subscripts.clone();
        return Ok((typedSubs, variability));
    }
    dims = Type::arrayDims(crefType.clone());
    typedSubs = metamodelica::nil();
    next_context = InstContext::set(context.clone(), InstContext::SUBSCRIPT.clone());
    i = 1;
    if (subscripts.clone().len() as i32) > (dims.clone().len() as i32) && checkSubscripts.clone() {
        Error::addSourceMessage(Error::WRONG_NUMBER_OF_SUBSCRIPTS.clone(), list![(Expression::toString(subscriptedExp.clone())?).clone(), ArcStr::from(::std::format!("{}", (subscripts.clone().len() as i32))), ArcStr::from(::std::format!("{}", (dims.clone().len() as i32)))], info.clone())?;
        bail!("fail");
    }
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        if checkSubscripts.clone() {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim = __pa0.clone();
            dims = __pa1.clone();
        } else {
            dim = Arc::new(crate::NFDimension::UNKNOWN);
        }
        (sub, var) = typeSubscript(s.clone(), dim.clone(), subscriptedExp.clone(), i.clone(), next_context.clone(), info.clone(), checkSubscripts.clone())?;
        typedSubs = cons(sub.clone(), typedSubs.clone());
        variability = Prefixes::variabilityMax(variability.clone(), var.clone());
        i = i.clone() + 1;
        if var.clone() == Variability::PARAMETER.clone() {
            Structural::markSubscript(sub.clone())?;
        }
    }
    typedSubs = typedSubs.clone().reverse();
    Ok((typedSubs, variability))
}

pub fn typeSubscript(mut subscript: Arc<Subscript::NFSubscript>, mut dimension: Arc<Dimension::NFDimension>, mut subscriptedExp: Arc<Expression::NFExpression>, mut index: i32, mut context: i32, mut info: SourceInfo, mut checkSubscript: bool) -> Result<(Arc<Subscript::NFSubscript>, Variability)> {
    let mut outSubscript: Arc<Subscript::NFSubscript> = subscript.clone();
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::EMPTY { ty: Arc::new(crate::NFType::UNKNOWN) });
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matched_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    (ty, variability) = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Subscript::UNTYPED { .. } => {
            e = evaluateEnd(var_field!((*subscript).exp, Subscript::NFSubscript::UNTYPED).clone(), dimension.clone(), subscriptedExp.clone(), index.clone(), context.clone(), info.clone())?;
            (e, ty, variability, _) = typeExp(e.clone(), context.clone(), info.clone(), false)?;
            if Type::isArray(ty.clone()) && InstContext::inEquation(context.clone()) {
                Structural::markExp(e.clone())?;
                e = Ceval::tryEvalExp(e.clone(), Ceval::noTarget().clone());
                ty = Expression::typeOf(e.clone());
            }
            if checkSubscript.clone() {
                (e, matched_ty) = checkSubscriptType(e.clone(), Type::arrayElementType(ty.clone()), dimension.clone(), info.clone())?;
            } else {
                matched_ty = ty.clone();
            }
            outSubscript = if (Type::isArray(ty.clone())) {Arc::new(Subscript::NFSubscript::SLICE { slice: e.clone() })} else {Arc::new(Subscript::NFSubscript::INDEX { index: e.clone() })};
            (matched_ty.clone(), variability.clone())
        },
        Deref @ Subscript::INDEX { index: e } => {
            let mut e = (*e).clone();
            if checkSubscript.clone() {
                (e, ty) = checkSubscriptType(e.clone(), Expression::typeOf(e.clone()), dimension.clone(), info.clone())?;
            } else {
                ty = Expression::typeOf(e.clone());
            }
            outSubscript = Arc::new(Subscript::NFSubscript::INDEX { index: e.clone() });
            (ty.clone(), Expression::variability(e.clone())?)
        },
        Deref @ Subscript::SLICE { slice: e } => {
            let mut e = (*e).clone();
            if checkSubscript.clone() {
                (e, ty) = checkSubscriptType(e.clone(), Type::unliftArray(Expression::typeOf(e.clone()))?, dimension.clone(), info.clone())?;
            } else {
                ty = Type::unliftArray(Expression::typeOf(e.clone()))?;
            }
            outSubscript = Arc::new(Subscript::NFSubscript::SLICE { slice: e.clone() });
            (ty.clone(), Expression::variability(e.clone())?)
        },
        Deref @ Subscript::WHOLE => (Arc::new(crate::NFType::UNKNOWN), Dimension::variability(dimension.clone())?),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeSubscript")); __mm_s.push_str(&*literal!(" got unknown subscript")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSubscript, variability))
}

pub fn checkSubscriptType(mut subscriptExp: Arc<Expression::NFExpression>, mut subscriptType: Arc<Type::NFType>, mut dimension: Arc<Dimension::NFDimension>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut subscriptExp: Arc<Expression::NFExpression> = subscriptExp;
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut expected_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    expected_ty = Dimension::subscriptType(dimension.clone());
    (subscriptExp, outType, mk) = TypeCheck::matchTypes(subscriptType.clone(), expected_ty.clone(), subscriptExp.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
    if TypeCheck::isIncompatibleMatch(mk.clone()) {
        Error::addSourceMessage(Error::SUBSCRIPT_TYPE_MISMATCH.clone(), list![(Expression::toString(subscriptExp.clone())?).clone(), (Type::toString(subscriptType.clone())?).clone(), (Type::toString(expected_ty.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((subscriptExp, outType))
}

pub fn typeArray(mut elements: metamodelica::Array<Arc<Expression::NFExpression>>, mut isLiteral: bool, mut ty: Arc<Type::NFType>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut arrayExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arrayType: Arc<Type::NFType> = Arc::new(crate::NFType::UNKNOWN);
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut expl2: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut ty1: Arc<Type::NFType> = Arc::new(crate::NFType::UNKNOWN);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty3: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut array_len: i32 = 0;
    let mut idx: i32 = 0;
    let mut next_context: i32 = 0;
    next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
    array_len = (elements.clone().borrow().len() as i32);
    if array_len.clone() > 0 {
        (exp, ty1, variability, purity) = typeExp(elements.clone().borrow()[(1-1) as usize].clone(), next_context.clone(), info.clone(), false)?;
        expl = cons(exp.clone(), expl.clone());
        tys = cons(ty1.clone(), tys.clone());
        for mut i in 2..=array_len.clone() {
            (exp, ty2, var, pur) = typeExp(elements.clone().borrow()[(i.clone()-1) as usize].clone(), next_context.clone(), info.clone(), false)?;
            variability = Prefixes::variabilityMax(var.clone(), variability.clone());
            purity = Prefixes::purityMin(pur.clone(), purity.clone());
            (_, ty3, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), exp.clone(), TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone())?;
            if TypeCheck::isIncompatibleMatch(mk.clone()) {
                (_, ty3, mk) = TypeCheck::matchTypes(ty1.clone(), ty2.clone(), exp.clone(), TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone())?;
                if TypeCheck::isCompatibleMatch(mk.clone()) {
                    ty1 = ty3.clone();
                }
            } else {
                ty1 = ty3.clone();
            }
            expl = cons(exp.clone(), expl.clone());
            tys = cons(ty2.clone(), tys.clone());
        }
    } else {
        ty1 = Type::arrayElementType(ty.clone());
    }
    idx = array_len.clone();
    for mut e in &*expl.clone() {
        let mut e = e.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty2 = __pa0.clone();
        tys = __pa1.clone();
        (exp, _, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), e.clone(), TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone())?;
        expl2 = cons(exp.clone(), expl2.clone());
        if !(InstContext::inAnnotation(context.clone())) {
            if TypeCheck::isIncompatibleMatch(mk.clone()) {
                Error::addSourceMessage(Error::NF_ARRAY_TYPE_MISMATCH.clone(), list![ArcStr::from(::std::format!("{}", idx.clone())), (Expression::toString(exp.clone())?).clone(), (Type::toString(ty2.clone())?).clone(), (Type::toString(ty1.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
        }
        idx = idx.clone() - 1;
    }
    arrayType = Type::liftArrayLeft(ty1.clone(), Dimension::fromExpList(expl2.clone()));
    arrayExp = Expression::makeArray(arrayType.clone(), metamodelica::arrayFromVec(expl2.clone().into_iter().cloned().collect()), isLiteral.clone());
    Ok((arrayExp, arrayType, variability, purity))
}

pub fn typeMatrix(mut elements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut arrayExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arrayType: Arc<Type::NFType> = Arc::new(crate::NFType::UNKNOWN);
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut ty: Arc<Type::NFType> = Arc::new(crate::NFType::UNKNOWN);
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut resTys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut n: i32 = 2;
    let mut next_context: i32 = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
    if (elements.clone().len() as i32) > 1 {
        for mut el in &*elements.clone() {
            let mut el = el.clone();
            (exp, ty, var, pur) = typeMatrixComma(el.clone(), next_context.clone(), info.clone())?;
            variability = Prefixes::variabilityMax(var.clone(), variability.clone());
            purity = Prefixes::purityMin(pur.clone(), purity.clone());
            expl = cons(exp.clone(), expl.clone());
            tys = cons(ty.clone(), tys.clone());
            n = std::cmp::max(n.clone(), Type::dimensionCount(ty.clone()));
        }
        for mut e in &*expl.clone() {
            let mut e = e.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            tys = __pa1.clone();
            (e, ty) = Expression::promote(e.clone(), ty.clone(), n.clone())?;
            resTys = cons(ty.clone(), resTys.clone());
            res = cons(e.clone(), res.clone());
        }
        (arrayExp, arrayType) = BuiltinCall::makeCatExp(1, res.clone(), resTys.clone(), variability.clone(), purity.clone(), info.clone())?;
    } else {
        (arrayExp, arrayType, variability, purity) = typeMatrixComma(listHead(elements.clone())?, next_context.clone(), info.clone())?;
        if Type::dimensionCount(arrayType.clone()) < 2 {
            (arrayExp, arrayType) = Expression::promote(arrayExp.clone(), arrayType.clone(), n.clone())?;
        }
    }
    Ok((arrayExp, arrayType, variability, purity))
}

pub fn typeMatrixComma(mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut arrayExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arrayType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut ty: Arc<Type::NFType> = Arc::new(crate::NFType::UNKNOWN);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty3: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut tys2: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut n: i32 = 2;
    let mut pos: i32 = 0;
    let mut mk: MatchKind = MatchKind::EXACT;
    Error::assertion(!(elements.clone().is_empty()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeMatrixComma")); __mm_s.push_str(&*literal!(" expected non-empty arguments")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    if (elements.clone().len() as i32) > 1 {
        for mut e in &*elements.clone() {
            let mut e = e.clone();
            (exp, ty1, var, pur) = typeExp(e.clone(), context.clone(), info.clone(), false)?;
            expl = cons(exp.clone(), expl.clone());
            if Type::isEqual(ty.clone(), Arc::new(crate::NFType::UNKNOWN)) {
                ty = ty1.clone();
            } else {
                (_, _, ty2, mk) = TypeCheck::matchExpressions(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Type::arrayElementType(ty1.clone()), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Type::arrayElementType(ty.clone()), TypeCheck::DEFAULT_OPTIONS.clone())?;
                if TypeCheck::isCompatibleMatch(mk.clone()) {
                    ty = ty2.clone();
                }
            }
            tys = cons(ty1.clone(), tys.clone());
            variability = Prefixes::variabilityMax(variability.clone(), var.clone());
            purity = Prefixes::purityMin(purity.clone(), pur.clone());
            n = std::cmp::max(n.clone(), Type::dimensionCount(ty.clone()));
        }
        tys2 = metamodelica::nil();
        res = metamodelica::nil();
        pos = n.clone() + 1;
        for mut e in &*expl.clone() {
            let mut e = e.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty1 = __pa0.clone();
            tys = __pa1.clone();
            pos = pos.clone() - 1;
            if Type::dimensionCount(ty1.clone()) != n.clone() {
                (e, ty1) = Expression::promote(e.clone(), ty1.clone(), n.clone())?;
            }
            ty2 = Type::setArrayElementType(ty1.clone(), ty.clone());
            (e, ty3, mk) = TypeCheck::matchTypes(ty1.clone(), ty2.clone(), e.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isIncompatibleMatch(mk.clone()) {
                Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![ArcStr::from(::std::format!("{}", pos.clone())), (literal!("matrix constructor ")).clone(), (literal!("arg")).clone(), (Expression::toString(e.clone())?).clone(), (Type::toString(ty1.clone())?).clone(), (Type::toString(ty2.clone())?).clone()], info.clone())?;
            }
            res = cons(e.clone(), res.clone());
            tys2 = cons(ty3.clone(), tys2.clone());
        }
        (arrayExp, arrayType) = BuiltinCall::makeCatExp(2, res.clone(), tys2.clone(), variability.clone(), purity.clone(), info.clone())?;
    } else {
        (arrayExp, arrayType, variability, _) = typeExp(listHead(elements.clone())?, context.clone(), info.clone(), false)?;
    }
    Ok((arrayExp, arrayType, variability, purity))
}

pub fn typeRange(mut rangeExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut rangeExp: Arc<Expression::NFExpression> = rangeExp;
    let mut rangeType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut start_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut step_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stop_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut start_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut step_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut stop_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ostep_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut ostep_ty: Option<Arc<Type::NFType>> = None;
    let mut start_var: Variability = Variability::CONSTANT;
    let mut step_var: Variability = Variability::CONSTANT;
    let mut stop_var: Variability = Variability::CONSTANT;
    let mut start_pur: Purity = Purity::PURE;
    let mut step_pur: Purity = Purity::PURE;
    let mut stop_pur: Purity = Purity::PURE;
    let mut ty_match: MatchKind = MatchKind::EXACT;
    let mut next_context: i32 = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(rangeExp.clone()) {
        Deref @ Expression::RANGE { stop: __pa0, step: __pa1, start: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stop_exp = __pa0.clone();
    ostep_exp = __pa1.clone();
    start_exp = __pa2.clone();
    (start_exp, start_ty, start_var, start_pur) = typeExp(start_exp.clone(), next_context.clone(), info.clone(), false)?;
    (stop_exp, stop_ty, stop_var, stop_pur) = typeExp(stop_exp.clone(), next_context.clone(), info.clone(), false)?;
    variability = Prefixes::variabilityMax(start_var.clone(), stop_var.clone());
    purity = Prefixes::purityMin(start_pur.clone(), stop_pur.clone());
    (start_exp, stop_exp, rangeType, ty_match) = TypeCheck::matchExpressions(start_exp.clone(), start_ty.clone(), stop_exp.clone(), stop_ty.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(ty_match.clone()) {
        printRangeTypeError(start_exp.clone(), start_ty.clone(), stop_exp.clone(), stop_ty.clone(), info.clone())?;
    }
    if isSome(ostep_exp.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(ostep_exp.clone()) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        step_exp = __pa3.clone();
        (step_exp, step_ty, step_var, step_pur) = typeExp(step_exp.clone(), next_context.clone(), info.clone(), false)?;
        variability = Prefixes::variabilityMax(step_var.clone(), variability.clone());
        purity = Prefixes::purityMin(step_pur.clone(), purity.clone());
        (start_exp, step_exp, rangeType, ty_match) = TypeCheck::matchExpressions(start_exp.clone(), start_ty.clone(), step_exp.clone(), step_ty.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
        if TypeCheck::isIncompatibleMatch(ty_match.clone()) {
            printRangeTypeError(start_exp.clone(), start_ty.clone(), step_exp.clone(), step_ty.clone(), info.clone())?;
        }
        (stop_exp, _, _) = TypeCheck::matchTypes_cast(stop_ty.clone(), rangeType.clone(), stop_exp.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
        ostep_exp = Some(step_exp.clone());
        ostep_ty = Some(step_ty.clone());
    } else {
        ostep_exp = None;
        ostep_ty = None;
    }
    rangeType = TypeCheck::getRangeType(start_exp.clone(), ostep_exp.clone(), stop_exp.clone(), rangeType.clone(), info.clone())?;
    rangeExp = Arc::new(Expression::NFExpression::RANGE { ty: rangeType.clone(), start: start_exp.clone(), step: ostep_exp.clone(), stop: stop_exp.clone() });
    if variability.clone() <= Variability::PARAMETER.clone() && purity.clone() == Purity::PURE.clone() && !(InstContext::inFunction(context.clone())) {
        Structural::markExp(rangeExp.clone())?;
    }
    Ok((rangeExp, rangeType, variability, purity))
}

pub fn typeTuple(mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut tupleExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tupleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE.clone();
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut tyl: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut valr: Arc<metamodelica::List<Variability>> = metamodelica::nil();
    let mut next_context: i32 = 0;
    if !(InstContext::onLHS(context.clone())) || InstContext::inSubexpression(context.clone()) {
        Error::addSourceMessage(Error::RHS_TUPLE_EXPRESSION.clone(), list![(Expression::toString(Arc::new(Expression::NFExpression::TUPLE { ty: Arc::new(crate::NFType::UNKNOWN), elements: elements.clone() }))?).clone()], info.clone())?;
        bail!("fail");
    }
    next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
    (expl, tyl, valr) = typeExpl(elements.clone(), next_context.clone(), info.clone())?;
    tupleType = Arc::new(Type::NFType::TUPLE { types: tyl.clone(), names: None });
    tupleExp = Arc::new(Expression::NFExpression::TUPLE { ty: tupleType.clone(), elements: expl.clone() });
    if !(List::all(expl.clone(), Arc::new(fnptr!(Expression::isCref, Arc<Expression::NFExpression>)))) {
        Error::addSourceMessage(Error::TUPLE_ASSIGN_CREFS_ONLY.clone(), list![(Expression::toString(tupleExp.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    variability = if (valr.clone().is_empty()) {Variability::CONSTANT.clone()} else {listHead(valr.clone())?};
    Ok((tupleExp, tupleType, variability, purity))
}

pub fn printRangeTypeError(mut exp1: Arc<Expression::NFExpression>, mut ty1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut ty2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<()> {
    Error::addSourceMessage(Error::RANGE_TYPE_MISMATCH.clone(), list![(Expression::toString(exp1.clone())?).clone(), (Type::toString(ty1.clone())?).clone(), (Expression::toString(exp2.clone())?).clone(), (Type::toString(ty2.clone())?).clone()], info.clone())?;
    bail!("fail");
    Ok(())
}

pub fn typeSize(mut sizeExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo, mut evaluate: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut sizeExp: Arc<Expression::NFExpression> = sizeExp;
    let mut sizeType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut index_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty_match: MatchKind = MatchKind::EXACT;
    let mut iindex: i32 = 0;
    let mut dim_size: i32 = 0;
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut ty_err: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    let mut oexp: Option<Arc<Expression::NFExpression>> = None;
    let mut next_context: i32 = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
    let mut expl: metamodelica::Array<Arc<Expression::NFExpression>>;
    (sizeExp, sizeType, variability, purity) = (::match_deref::match_deref! { match &(sizeExp.clone()) {
        Deref @ Expression::SIZE { dimIndex: Some(index), exp } => {
            let mut index = (*index).clone();
            let mut exp = (*exp).clone();
            (index, index_ty, variability, purity) = typeExp(index.clone(), next_context.clone(), info.clone(), false)?;
            (index, _, ty_match) = TypeCheck::matchTypes(index_ty.clone(), Arc::new(crate::NFType::INTEGER), index.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isIncompatibleMatch(ty_match.clone()) {
                Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (literal!("size ")).clone(), (literal!("dim")).clone(), (Expression::toString(index.clone())?).clone(), (Type::toString(index_ty.clone())?).clone(), (literal!("Integer")).clone()], info.clone())?;
                bail!("fail");
            }
            if variability.clone() <= Variability::STRUCTURAL_PARAMETER.clone() && purity.clone() == Purity::PURE.clone() {
                index = Ceval::evalExp(index.clone(), Ceval::noTarget().clone())?;
                let __pa0 = ::match_deref::match_deref! { match &(index.clone()) {
                    Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                iindex = __pa0.clone();
                (dim, oexp, ty_err) = typeExpDim(exp.clone(), iindex.clone(), next_context.clone(), info.clone())?;
                checkSizeTypingError(ty_err.clone(), exp.clone(), iindex.clone(), info.clone())?;
                if Dimension::isKnown(dim.clone(), false) && evaluate.clone() {
                    exp = Dimension::sizeExp(dim.clone())?;
                } else {
                    if isSome(oexp.clone()) {
                        let __pa1 = ::match_deref::match_deref! { match &(oexp.clone()) {
                            Some(__pa1) => __pa1.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        exp = __pa1.clone();
                    } else {
                        (exp, _, _, _) = typeExp(exp.clone(), next_context.clone(), info.clone(), false)?;
                    }
                    exp = Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
                }
                if !(InstContext::inFunction(context.clone())) || Dimension::isKnown(dim.clone(), false) {
                    variability = Variability::CONSTANT.clone();
                } else {
                    variability = Variability::DISCRETE.clone();
                    purity = Purity::IMPURE.clone();
                }
            } else {
                (exp, exp_ty, _, purity) = typeExp(var_field!((*sizeExp).exp, Expression::NFExpression::SIZE).clone(), next_context.clone(), info.clone(), false)?;
                if !(Type::isArray(exp_ty.clone())) {
                    Error::addSourceMessage(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone(), list![(literal!("size")).clone()], info.clone())?;
                    bail!("fail");
                }
                if Type::isEmptyArray(exp_ty.clone()) && !(InstContext::inFunction(context.clone())) {
                    expl = Array::mapList(Type::arrayDims(exp_ty.clone()), Arc::new(Dimension::sizeExp))?;
                    exp = Expression::makeExpArray(expl.clone(), Arc::new(crate::NFType::INTEGER), false);
                    exp = Expression::makeSubscriptedExp(list![Subscript::makeIndex(index.clone())?], exp.clone(), false)?;
                } else {
                    exp = Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
                }
            }
            (exp.clone(), Arc::new(crate::NFType::INTEGER), variability.clone(), purity.clone())
        },
        Deref @ Expression::SIZE { .. } => {
            (exp, exp_ty, _, _) = typeExp(var_field!((*sizeExp).exp, Expression::NFExpression::SIZE).clone(), next_context.clone(), info.clone(), false)?;
            sizeType = Type::sizeType(exp_ty.clone());
            (Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: None }), sizeType.clone(), Variability::PARAMETER.clone(), Purity::PURE.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((sizeExp, sizeType, variability, purity))
}

pub fn checkSizeTypingError(mut typingError: Arc<TypingError::TypingError>, mut exp: Arc<Expression::NFExpression>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(typingError.clone()) {
        Deref @ TypingError::NO_ERROR { .. } => (),
        Deref @ TypingError::OUT_OF_BOUNDS { upperBound: 0 } => {
            Error::addSourceMessage(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone(), list![(literal!("size")).clone()], info.clone())?;
            bail!("fail")
        },
        Deref @ TypingError::OUT_OF_BOUNDS { .. } => {
            Error::addSourceMessage(Error::INVALID_SIZE_INDEX.clone(), list![ArcStr::from(::std::format!("{}", index.clone())), (Expression::toString(exp.clone())?).clone(), ArcStr::from(::std::format!("{}", var_field!((*typingError).upperBound, TypingError::TypingError::OUT_OF_BOUNDS).clone()))], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn evaluateEnd(mut exp: Arc<Expression::NFExpression>, mut dim: Arc<Dimension::NFDimension>, mut subscriptedExp: Arc<Expression::NFExpression>, mut index: i32, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::END => {
            Dimension::endExp(dim.clone(), subscriptedExp.clone(), index.clone())?
        },
        Deref @ Expression::CREF { .. } => {
            exp.clone()
        },
        _ => {
            Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = dim.clone(); let __pe_b2 = subscriptedExp.clone(); let __pe_b3 = index.clone(); let __pe_b4 = context.clone(); let __pe_b5 = info.clone(); move |__pe_a0| evaluateEnd(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn typeIfExpression(mut ifExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut ifExp: Arc<Expression::NFExpression> = ifExp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tb2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fb2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut next_context: i32 = 0;
    let mut cond_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tb_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut fb_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cond_var: Variability = Variability::CONSTANT;
    let mut tb_var: Variability = Variability::CONSTANT;
    let mut fb_var: Variability = Variability::CONSTANT;
    let mut cond_pur: Purity = Purity::PURE;
    let mut tb_pur: Purity = Purity::PURE;
    let mut fb_pur: Purity = Purity::PURE;
    let mut ty_match: MatchKind = MatchKind::EXACT;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ifExp.clone()) {
        Deref @ Expression::IF { falseBranch: __pa0, trueBranch: __pa1, condition: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    fb = __pa0.clone();
    tb = __pa1.clone();
    cond = __pa2.clone();
    next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
    (cond, cond_ty, cond_var, cond_pur) = typeExp(cond.clone(), next_context.clone(), info.clone(), false)?;
    (cond, _, ty_match) = TypeCheck::matchTypes(cond_ty.clone(), Arc::new(crate::NFType::BOOLEAN), cond.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(ty_match.clone()) {
        Error::addSourceMessage(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(Expression::toString(cond.clone())?).clone(), (Type::toString(cond_ty.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    (tb, tb_ty, tb_var, tb_pur) = typeExp(tb.clone(), next_context.clone(), info.clone(), false)?;
    (fb, fb_ty, fb_var, fb_pur) = typeExp(fb.clone(), next_context.clone(), info.clone(), false)?;
    (tb2, fb2, ty, ty_match) = TypeCheck::matchIfBranches(tb.clone(), tb_ty.clone(), fb.clone(), fb_ty.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(ty_match.clone()) {
        Error::addSourceMessage(Error::TYPE_MISMATCH_IF_EXP.clone(), list![(literal!("")).clone(), (Expression::toString(tb.clone())?).clone(), (Type::toString(tb_ty.clone())?).clone(), (Expression::toString(fb.clone())?).clone(), (Type::toString(fb_ty.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    if Expression::contains(tb2.clone(), Arc::new({ let __pe_b1 = (literal!("der")).clone(); move |__pe_a0| Expression::isCallNamed(__pe_a0, __pe_b1.clone()) }))? != Expression::contains(fb2.clone(), Arc::new({ let __pe_b1 = (literal!("der")).clone(); move |__pe_a0| Expression::isCallNamed(__pe_a0, __pe_b1.clone()) }))? && Flags::getConfigString(Flags::EVALUATE_STRUCTURAL_PARAMETERS.clone())? == literal!("all") {
        Structural::markExp(cond.clone())?;
    }
    ifExp = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: tb2.clone(), falseBranch: fb2.clone() });
    var = Prefixes::variabilityMax(cond_var.clone(), Prefixes::variabilityMax(tb_var.clone(), fb_var.clone()));
    purity = Prefixes::purityMin(cond_pur.clone(), Prefixes::purityMin(tb_pur.clone(), fb_pur.clone()));
    Ok((ifExp, ty, var, purity))
}

pub fn typeClassSections(mut classNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut typed_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut components: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut info: SourceInfo;
    let mut initial_context: i32 = 0;
    cls = InstNode::getClass(classNode.clone())?;
    let _ = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { .. } if (Type::isBasic(Type::arrayElementType(var_field!((*cls).ty, Class::NFClass::INSTANCED_CLASS).clone()))) => (),
        Deref @ Class::INSTANCED_CLASS { sections, elements: Deref @ ClassTree::FLAT_TREE { components, .. }, .. } => {
            let mut sections = (*sections).clone();
            sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { .. } => {
            initial_context = InstContext::set(context.clone(), InstContext::INITIAL.clone());
            Sections::map(sections.clone(), Arc::new({ let __pe_b1 = InstContext::set(context.clone(), InstContext::EQUATION.clone()); move |__pe_a0| typeEquation(__pe_a0, __pe_b1.clone()) }), Arc::new({ let __pe_b1 = InstContext::set(context.clone(), InstContext::ALGORITHM.clone()); move |__pe_a0| typeAlgorithm(__pe_a0, __pe_b1.clone()) }), Arc::new({ let __pe_b1 = InstContext::set(initial_context.clone(), InstContext::EQUATION.clone()); move |__pe_a0| typeEquation(__pe_a0, __pe_b1.clone()) }), Arc::new({ let __pe_b1 = InstContext::set(initial_context.clone(), InstContext::ALGORITHM.clone()); move |__pe_a0| typeAlgorithm(__pe_a0, __pe_b1.clone()) }))
        },
        Deref @ Sections::EXTERNAL { .. } => {
            Error::addSourceMessage(Error::TRANS_VIOLATION.clone(), list![(InstNode::name(classNode.clone())?).clone(), (Restriction::toString(var_field!((*cls).restriction, Class::NFClass::INSTANCED_CLASS).clone())).clone(), (literal!("external declaration")).clone()], InstNode::info(classNode.clone())?)?;
            bail!("fail")
        },
        _ => sections.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            typed_cls = Class::setSections(sections.clone(), cls.clone())?;
            let __range0 = components.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentSections(InstNode::resolveOuter(c.clone()), context.clone())?;
            }
            InstNode::updateClass(typed_cls.clone(), classNode.clone())?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        Deref @ Class::TYPED_DERIVED { .. } => {
            typeClassSections(var_field!((*cls).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeClassSections")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*InstNode::name(classNode.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeFunctionSections(mut classNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut typed_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut info: SourceInfo;
    let mut alg: Arc<Algorithm::NFAlgorithm>;
    cls = InstNode::getClass(classNode.clone())?;
    let _ = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { sections, .. } => {
            let mut sections = (*sections).clone();
            sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { equations: Deref @ metamodelica::List::Nil, initialEquations: Deref @ metamodelica::List::Nil, algorithms: Deref @ metamodelica::List::Cons { head: alg, tail: Deref @ metamodelica::List::Nil }, initialAlgorithms: Deref @ metamodelica::List::Nil } => {
            assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = list![typeAlgorithm(alg.clone(), InstContext::set(context.clone(), InstContext::ALGORITHM.clone()))?]);
            sections.clone()
        },
        Deref @ Sections::SECTIONS { .. } => {
            Error::addSourceMessage(Error::MULTIPLE_SECTIONS_IN_FUNCTION.clone(), list![(InstNode::name(classNode.clone())?).clone()], InstNode::info(classNode.clone())?)?;
            bail!("fail")
        },
        Deref @ Sections::EXTERNAL { explicit: true, .. } => {
            info = InstNode::info(classNode.clone())?;
            assign_variant_field!(sections => Sections::NFSections::EXTERNAL;
                args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*sections).args, Sections::NFSections::EXTERNAL).clone()).into_iter().cloned() {
            let __x = typeExternalArg(arg.clone(), info.clone(), classNode.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                outputRef = typeCref(var_field!((*sections).outputRef, Sections::NFSections::EXTERNAL).clone(), context.clone(), info.clone())?.0
            );
            checkExternalCallResult(var_field!((*sections).outputRef, Sections::NFSections::EXTERNAL).clone(), info.clone())?;
            sections.clone()
        },
        Deref @ Sections::EXTERNAL { .. } => makeDefaultExternalCall(sections.clone(), classNode.clone())?,
        _ => sections.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            typed_cls = Class::setSections(sections.clone(), cls.clone())?;
            InstNode::updateClass(typed_cls.clone(), classNode.clone())?;
            ()
        },
        Deref @ Class::TYPED_DERIVED { .. } => {
            typeFunctionSections(var_field!((*cls).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeFunctionSections")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*InstNode::name(classNode.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeExternalArg(mut arg: Arc<Expression::NFExpression>, mut info: SourceInfo, mut node: Arc<InstNode::InstNode>) -> Result<Arc<Expression::NFExpression>> {
    let mut outArg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut index: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outArg = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::SIZE { dimIndex: Some(_), .. } => {
            (outArg, _, _, _) = typeSize(arg.clone(), InstContext::FUNCTION.clone(), info.clone(), false)?;
            let __pa0 = ::match_deref::match_deref! { match &(outArg.clone()) {
                Deref @ Expression::SIZE { dimIndex: Some(__pa0), .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            index = __pa0.clone();
            if !(Expression::isInteger(index.clone())) {
                Error::addSourceMessage(Error::EXTERNAL_ARG_NONCONSTANT_SIZE_INDEX.clone(), list![(Expression::toString(arg.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            outArg.clone()
        },
        _ => {
            (outArg, ty, var, _) = typeExp(arg.clone(), InstContext::FUNCTION.clone(), info.clone(), false)?;
            Call::updateExternalRecordArgsInType(ty.clone())?;
            (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { .. } => outArg.clone(),
        _ => {
            if Type::isScalarBuiltin(ty.clone()) && var.clone() == Variability::CONSTANT.clone() {
                outArg = Ceval::evalExp(outArg.clone(), Ceval::EvalTarget::new(info.clone(), InstContext::FUNCTION.clone(), None))?;
            } else {
                Error::addSourceMessage(Error::EXTERNAL_ARG_WRONG_EXP.clone(), list![(Expression::toString(outArg.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            outArg.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

pub fn makeDefaultExternalCall(mut extDecl: Arc<Sections::NFSections>, mut fnNode: Arc<InstNode::InstNode>) -> Result<Arc<Sections::NFSections>> {
    let mut extDecl: Arc<Sections::NFSections> = extDecl;
    extDecl = (::match_deref::match_deref! { match &(extDecl.clone()) {
        Deref @ Sections::EXTERNAL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut r#fn: Arc<Function::Function>;
            let mut single_output: bool = false;
            let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
            let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if var_field!((*extDecl).language, Sections::NFSections::EXTERNAL).clone() == literal!("builtin") {
                return Ok(extDecl);
            }
            let __pa0 = ::match_deref::match_deref! { match &(InstNode::getFuncCache(fnNode.clone())?) {
                Deref @ CachedData::FUNCTION { funcs: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            single_output = (r#fn.outputs.clone().len() as i32) == 1;
            if single_output.clone() && Type::isArray(Function::returnType(r#fn.clone())) {
                single_output = false;
                Error::addSourceMessage(Error::EXT_FN_SINGLE_RETURN_ARRAY.clone(), list![(var_field!((*extDecl).language, Sections::NFSections::EXTERNAL).clone()).clone()], InstNode::info(fnNode.clone())?)?;
            }
            if single_output.clone() {
                let __pa2 = ::match_deref::match_deref! { match &(r#fn.outputs.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                node = __pa2.clone();
                ty = InstNode::getType(node.clone())?;
                assign_variant_field!(extDecl => Sections::NFSections::EXTERNAL; outputRef = ComponentRef::fromNode(node.clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone()));
            }
            comps = ClassTree::getComponents(Class::classTree(InstNode::getClass(r#fn.node.clone())?)?)?;
            if (comps.clone().borrow().len() as i32) > 0 {
                args = metamodelica::nil();
                let __range4 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range4 {
                    comp = InstNode::component(c.clone())?;
                    if !(single_output.clone()) || Component::direction(comp.clone()) != Direction::OUTPUT.clone() {
                        ty = Component::getType(comp.clone())?;
                        exp = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: ComponentRef::fromNode(c.clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone()) });
                        args = cons(exp.clone(), args.clone());
                        for mut i in 1..=Type::dimensionCount(ty.clone()) {
                            args = cons(Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: i.clone() })) }), args.clone());
                        }
                    }
                }
                assign_variant_field!(extDecl => Sections::NFSections::EXTERNAL; args = args.clone().reverse());
            }
            extDecl.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(extDecl)
}

pub fn checkExternalCallResult(mut result: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<()> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if !(ComponentRef::isCref(result.clone())) {
        return Ok(());
    }
    ty = ComponentRef::nodeType(result.clone())?;
    if Type::isArray(ty.clone()) {
        Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_ARRAY_TYPE.clone(), list![(Type::toString(ty.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    if ComponentRef::variability(result.clone()) < Variability::DISCRETE.clone() {
        Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_NOT_VAR.clone(), metamodelica::nil(), info.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn typeComponentSections(mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    if InstNode::isEmpty(component.clone()) {
        return Ok(());
    }
    comp = InstNode::component(component.clone())?;
    if Component::isDeleted(comp.clone())? || InstNode::isOnlyOuter(component.clone())? {
        return Ok(());
    }
    let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT { .. } if (var_field!((*comp).state, Component::NFComponent::COMPONENT).clone() >= ComponentState::TypeChecked.clone()) => {
            typeClassSections(var_field!((*comp).classInst, Component::NFComponent::COMPONENT).clone(), context.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponentSections")); __mm_s.push_str(&*literal!(" got uninstantiated component ")); __mm_s.push_str(&*InstNode::name(component.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeEquation(mut eq: Arc<Equation::NFEquation>, mut context: i32) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            typeEqualityEquation(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), context.clone(), var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone())?
        },
        Deref @ Equation::CONNECT { .. } => {
            typeConnect(var_field!((*eq).lhs, Equation::NFEquation::CONNECT).clone(), var_field!((*eq).rhs, Equation::NFEquation::CONNECT).clone(), context.clone(), var_field!((*eq).scope, Equation::NFEquation::CONNECT).clone(), var_field!((*eq).source, Equation::NFEquation::CONNECT).clone())?
        },
        Deref @ Equation::FOR { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*eq).source, Equation::NFEquation::FOR).clone());
            if isSome(var_field!((*eq).range, Equation::NFEquation::FOR).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*eq).range, Equation::NFEquation::FOR).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e1 = __pa0.clone();
            } else {
                e1 = deduceIterationRangeEq(eq.clone(), var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), info.clone())?;
            }
            (e1, _, _, _) = typeIterator(var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), e1.clone(), context.clone(), true)?;
            next_context = InstContext::set(context.clone(), InstContext::FOR.clone());
            body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).body, Equation::NFEquation::FOR).clone()).into_iter().cloned() {
            let __x = typeEquation(e.clone(), next_context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(Equation::NFEquation::FOR { iterator: var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), range: Some(e1.clone()), body: body.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::FOR).clone(), source: var_field!((*eq).source, Equation::NFEquation::FOR).clone() })
        },
        Deref @ Equation::IF { .. } => {
            typeIfEquation(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), context.clone(), var_field!((*eq).scope, Equation::NFEquation::IF).clone(), var_field!((*eq).source, Equation::NFEquation::IF).clone())?
        },
        Deref @ Equation::WHEN { .. } => {
            typeWhenEquation(var_field!((*eq).branches, Equation::NFEquation::WHEN).clone(), context.clone(), var_field!((*eq).scope, Equation::NFEquation::WHEN).clone(), var_field!((*eq).source, Equation::NFEquation::WHEN).clone())?
        },
        Deref @ Equation::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*eq).source, Equation::NFEquation::ASSERT).clone());
            (e1, e2, e3) = typeAssert(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), context.clone(), info.clone())?;
            Arc::new(Equation::NFEquation::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::ASSERT).clone(), source: var_field!((*eq).source, Equation::NFEquation::ASSERT).clone() })
        },
        Deref @ Equation::TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*eq).source, Equation::NFEquation::TERMINATE).clone());
            (e1, _) = typeOperatorArg(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), Arc::new(crate::NFType::STRING), context.clone(), (literal!("terminate")).clone(), (literal!("message")).clone(), 1, info.clone())?;
            Arc::new(Equation::NFEquation::TERMINATE { message: e1.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::TERMINATE).clone(), source: var_field!((*eq).source, Equation::NFEquation::TERMINATE).clone() })
        },
        Deref @ Equation::REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (e1, e2) = typeReinit(var_field!((*eq).cref, Equation::NFEquation::REINIT).clone(), var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), context.clone(), var_field!((*eq).source, Equation::NFEquation::REINIT).clone())?;
            Arc::new(Equation::NFEquation::REINIT { cref: e1.clone(), reinitExp: e2.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::REINIT).clone(), source: var_field!((*eq).source, Equation::NFEquation::REINIT).clone() })
        },
        Deref @ Equation::NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (e1, _, _, _) = typeExp(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), context.clone(), ElementSource::getInfo(var_field!((*eq).source, Equation::NFEquation::NORETCALL).clone()), false)?;
            Arc::new(Equation::NFEquation::NORETCALL { exp: e1.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::NORETCALL).clone(), source: var_field!((*eq).source, Equation::NFEquation::NORETCALL).clone() })
        },
        _ => {
            eq.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub fn typeConnect(mut lhsConn: Arc<Expression::NFExpression>, mut rhsConn: Arc<Expression::NFExpression>, mut context: i32, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut connEq: Arc<Equation::NFEquation>;
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lhs_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut rhs_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut lhs_var: Variability = Variability::CONSTANT;
    let mut rhs_var: Variability = Variability::CONSTANT;
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut next_context: i32 = 0;
    let mut info: SourceInfo;
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut lhs_deleted: bool = false;
    let mut rhs_deleted: bool = false;
    info = ElementSource::getInfo(source.clone());
    if InstContext::inNonexpandable(context.clone()) {
        Error::addSourceMessage(Error::CONNECT_IN_IF.clone(), list![(Expression::toString(lhsConn.clone())?).clone(), (Expression::toString(rhsConn.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    next_context = InstContext::set(context.clone(), InstContext::CONNECT.clone());
    (lhs, lhs_ty, lhs_deleted) = typeConnector(lhsConn.clone(), next_context.clone(), info.clone())?;
    (rhs, rhs_ty, rhs_deleted) = typeConnector(rhsConn.clone(), next_context.clone(), info.clone())?;
    if !(lhs_deleted.clone() || rhs_deleted.clone()) && !(Type::isExpandableConnector(Type::arrayElementType(lhs_ty.clone())) || Type::isExpandableConnector(Type::arrayElementType(rhs_ty.clone()))) {
        (lhs, rhs, _, mk) = TypeCheck::matchExpressions(lhs.clone(), lhs_ty.clone(), rhs.clone(), rhs_ty.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
        if TypeCheck::isIncompatibleMatch(mk.clone()) {
            Error::addSourceMessage(Error::CONNECT_TYPE_MISMATCH.clone(), list![(Expression::toString(lhsConn.clone())?).clone(), (Expression::toString(rhsConn.clone())?).clone()], info.clone())?;
            bail!("fail");
        }
    }
    connEq = Arc::new(Equation::NFEquation::CONNECT { lhs: lhs.clone(), rhs: rhs.clone(), scope: scope.clone(), source: source.clone() });
    Ok(connEq)
}

pub fn typeConnector(mut connExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, bool)> {
    let mut connExp: Arc<Expression::NFExpression> = connExp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut deleted: bool = false;
    (connExp, ty, _, _) = typeExp(connExp.clone(), context.clone(), info.clone(), false)?;
    deleted = checkConnector(connExp.clone(), info.clone())?;
    Ok((connExp, ty, deleted))
}

pub fn checkConnector(mut connExp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<bool> {
    let mut deleted: bool = false;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(connExp.clone()) {
        Deref @ Expression::CREF { cref: cr @ Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF { .. }, .. }, .. } => {
            if !(InstNode::isConnector(var_field!((**cr).node, ComponentRef::NFComponentRef::CREF).clone())?) {
                Error::addSourceMessageAndFail(Error::INVALID_CONNECTOR_TYPE.clone(), list![(ComponentRef::toString(cr.clone())?).clone()], info.clone())?;
            }
            if !(checkConnectorForm(cr.clone(), true)?) {
                Error::addSourceMessageAndFail(Error::INVALID_CONNECTOR_FORM.clone(), list![(ComponentRef::toString(cr.clone())?).clone()], info.clone())?;
            }
            if ComponentRef::subscriptsVariability(cr.clone(), Prefixes::Variability::CONSTANT.clone())? > Variability::PARAMETER.clone() {
                subs = ComponentRef::subscriptsAllFlat(cr.clone());
                for mut sub in &*subs.clone() {
                    let mut sub = sub.clone();
                    if Subscript::variability(sub.clone())? > Variability::PARAMETER.clone() {
                        Error::addSourceMessage(Error::CONNECTOR_NON_PARAMETER_SUBSCRIPT.clone(), list![(Expression::toString(connExp.clone())?).clone(), (Subscript::toString(sub.clone())?).clone()], info.clone())?;
                        bail!("fail");
                    }
                }
            }
            deleted = ComponentRef::isDeleted(cr.clone())?;
            ()
        },
        _ => {
            Error::addSourceMessage(Error::INVALID_CONNECTOR_TYPE.clone(), list![(Expression::toString(connExp.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(deleted)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn checkConnectorForm(mut cref: Arc<ComponentRef::NFComponentRef>, mut isConnector: bool) -> Result<bool> {
    let mut valid: bool = false;
    valid = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF { .. }, .. } => if (isConnector.clone()) {checkConnectorForm(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), InstNode::isConnector(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?)?} else {false},
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(valid)
}

pub fn checkLhsInWhen(mut exp: Arc<Expression::NFExpression>) -> bool {
    let mut isValid: bool = false;
    isValid = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => true,
        Deref @ Expression::TUPLE { .. } => {
            for mut e in &*var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut e = e.clone();
                checkLhsInWhen(e.clone());
            }
            true
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isValid
}

pub fn typeAssert(mut condition: Arc<Expression::NFExpression>, mut message: Arc<Expression::NFExpression>, mut level: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)> {
    let mut condition: Arc<Expression::NFExpression> = condition;
    let mut message: Arc<Expression::NFExpression> = message;
    let mut level: Arc<Expression::NFExpression> = level;
    let mut next_context: i32 = 0;
    let mut level_var: Variability = Variability::CONSTANT;
    next_context = InstContext::set(context.clone(), InstContext::ASSERT.clone());
    (condition, _) = typeOperatorArg(condition.clone(), Arc::new(crate::NFType::BOOLEAN), InstContext::set(next_context.clone(), InstContext::CONDITION.clone()), (literal!("assert")).clone(), (literal!("condition")).clone(), 1, info.clone())?;
    (message, _) = typeOperatorArg(message.clone(), Arc::new(crate::NFType::STRING), next_context.clone(), (literal!("assert")).clone(), (literal!("message")).clone(), 2, info.clone())?;
    (level, level_var) = typeOperatorArg(level.clone(), Builtin::ASSERTIONLEVEL_TYPE().clone(), next_context.clone(), (literal!("assert")).clone(), (literal!("level")).clone(), 3, info.clone())?;
    if level_var.clone() > Variability::PARAMETER.clone() {
        Error::addSourceMessage(Error::FUNCTION_SLOT_VARIABILITY.clone(), list![(literal!("level")).clone(), (Expression::toString(level.clone())?).clone(), (literal!("assert")).clone(), (Prefixes::variabilityString(level_var.clone())?).clone(), (literal!("parameter")).clone()], info.clone())?;
        bail!("fail");
    }
    Structural::markExp(level.clone())?;
    Ok((condition, message, level))
}

pub fn typeAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut context: i32) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (alg.statements.clone()).into_iter().cloned() {
            let __x = typeStatement(s.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(alg)
}

pub fn typeStatements(mut alg: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut context: i32) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut alg: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = alg;
    alg = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut stmt in (alg.clone()).into_iter().cloned() {
            let __x = typeStatement(stmt.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(alg)
}

pub fn typeStatement(mut st: Arc<Statement::NFStatement>, mut context: i32) -> Result<Arc<Statement::NFStatement>> {
    let mut st: Arc<Statement::NFStatement> = st;
    st = (::match_deref::match_deref! { match &(st.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut mk: MatchKind = MatchKind::EXACT;
            let mut info: SourceInfo;
            let mut var: Variability = Variability::CONSTANT;
            info = ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::ASSIGNMENT).clone());
            (e1, ty1, var, _) = typeExp(var_field!((*st).lhs, Statement::NFStatement::ASSIGNMENT).clone(), InstContext::set(context.clone(), InstContext::LHS.clone()), info.clone(), false)?;
            (e2, ty2, _, _) = typeExp(var_field!((*st).rhs, Statement::NFStatement::ASSIGNMENT).clone(), InstContext::set(context.clone(), InstContext::RHS.clone()), info.clone(), false)?;
            (e2, _, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), e2.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
            if TypeCheck::isIncompatibleMatch(mk.clone()) {
                Error::addSourceMessage(Error::ASSIGN_TYPE_MISMATCH_ERROR.clone(), list![(Expression::toString(e1.clone())?).clone(), (Expression::toString(e2.clone())?).clone(), (Type::toString(ty1.clone())?).clone(), (Type::toString(ty2.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            checkAssignment(e1.clone(), e2.clone(), var.clone(), context.clone(), info.clone())?;
            if Expression::isExternalCall(e2.clone())? {
                Call::updateExternalRecordArgs(Expression::tupleElements(e1.clone()))?;
            }
            Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: e1.clone(), rhs: e2.clone(), ty: ty1.clone(), source: var_field!((*st).source, Statement::NFStatement::ASSIGNMENT).clone() })
        },
        Deref @ Statement::FOR { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::FOR).clone());
            if isSome(var_field!((*st).range, Statement::NFStatement::FOR).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*st).range, Statement::NFStatement::FOR).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e1 = __pa0.clone();
            } else {
                e1 = deduceIterationRangeStmt(st.clone(), var_field!((*st).iterator, Statement::NFStatement::FOR).clone(), info.clone())?;
            }
            (e1, _, _, _) = typeIterator(var_field!((*st).iterator, Statement::NFStatement::FOR).clone(), e1.clone(), context.clone(), false)?;
            next_context = InstContext::set(context.clone(), InstContext::FOR.clone());
            body = typeStatements(var_field!((*st).body, Statement::NFStatement::FOR).clone(), next_context.clone())?;
            Arc::new(Statement::NFStatement::FOR { iterator: var_field!((*st).iterator, Statement::NFStatement::FOR).clone(), range: Some(e1.clone()), body: body.clone(), forType: var_field!((*st).forType, Statement::NFStatement::FOR).clone(), source: var_field!((*st).source, Statement::NFStatement::FOR).clone() })
        },
        Deref @ Statement::IF { .. } => {
            let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut sts1: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut tybrs: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            let mut cond_context: i32 = 0;
            next_context = InstContext::set(context.clone(), InstContext::IF.clone());
            cond_context = InstContext::set(next_context.clone(), InstContext::CONDITION.clone());
            tybrs = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut br in (var_field!((*st).branches, Statement::NFStatement::IF).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(br.clone()) {
        (cond, body) => {
            (e1, _, _) = typeCondition(cond.clone(), cond_context.clone(), var_field!((*st).source, Statement::NFStatement::IF).clone(), Error::IF_CONDITION_TYPE_ERROR.clone(), false, false)?;
            sts1 = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut bst in (body.clone()).into_iter().cloned() {
            let __x = typeStatement(bst.clone(), next_context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (e1.clone(), sts1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(Statement::NFStatement::IF { branches: tybrs.clone(), source: var_field!((*st).source, Statement::NFStatement::IF).clone() })
        },
        Deref @ Statement::WHEN { .. } => {
            let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut sts1: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut tybrs: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            next_context = InstContext::set(context.clone(), InstContext::WHEN.clone());
            tybrs = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut br in (var_field!((*st).branches, Statement::NFStatement::WHEN).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(br.clone()) {
        (cond, body) => {
            (e1, _, _) = typeWhenCondition(cond.clone(), context.clone(), var_field!((*st).source, Statement::NFStatement::WHEN).clone(), false)?;
            sts1 = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut bst in (body.clone()).into_iter().cloned() {
            let __x = typeStatement(bst.clone(), next_context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (e1.clone(), sts1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(Statement::NFStatement::WHEN { branches: tybrs.clone(), source: var_field!((*st).source, Statement::NFStatement::WHEN).clone() })
        },
        Deref @ Statement::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::ASSERT).clone());
            (e1, e2, e3) = typeAssert(var_field!((*st).condition, Statement::NFStatement::ASSERT).clone(), var_field!((*st).message, Statement::NFStatement::ASSERT).clone(), var_field!((*st).level, Statement::NFStatement::ASSERT).clone(), context.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), source: var_field!((*st).source, Statement::NFStatement::ASSERT).clone() })
        },
        Deref @ Statement::TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::TERMINATE).clone());
            if InstContext::inFunction(context.clone()) {
                Error::addSourceMessage(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(literal!("terminate")).clone()], info.clone())?;
                bail!("fail");
            }
            (e1, _) = typeOperatorArg(var_field!((*st).message, Statement::NFStatement::TERMINATE).clone(), Arc::new(crate::NFType::STRING), context.clone(), (literal!("terminate")).clone(), (literal!("message")).clone(), 1, info.clone())?;
            Arc::new(Statement::NFStatement::TERMINATE { message: e1.clone(), source: var_field!((*st).source, Statement::NFStatement::TERMINATE).clone() })
        },
        Deref @ Statement::REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if InstContext::inFunction(context.clone()) {
                Error::addSourceMessage(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(literal!("reinit")).clone()], ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::REINIT).clone()))?;
                bail!("fail");
            }
            (e1, e2) = typeReinit(var_field!((*st).cref, Statement::NFStatement::REINIT).clone(), var_field!((*st).reinitExp, Statement::NFStatement::REINIT).clone(), context.clone(), var_field!((*st).source, Statement::NFStatement::REINIT).clone())?;
            Arc::new(Statement::NFStatement::REINIT { cref: e1.clone(), reinitExp: e2.clone(), source: var_field!((*st).source, Statement::NFStatement::REINIT).clone() })
        },
        Deref @ Statement::NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (e1, _, _, _) = typeExp(var_field!((*st).exp, Statement::NFStatement::NORETCALL).clone(), context.clone(), ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::NORETCALL).clone()), false)?;
            Arc::new(Statement::NFStatement::NORETCALL { exp: e1.clone(), source: var_field!((*st).source, Statement::NFStatement::NORETCALL).clone() })
        },
        Deref @ Statement::WHILE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut sts1: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            (e1, _, _) = typeCondition(var_field!((*st).condition, Statement::NFStatement::WHILE).clone(), context.clone(), var_field!((*st).source, Statement::NFStatement::WHILE).clone(), Error::WHILE_CONDITION_TYPE_ERROR.clone(), false, false)?;
            sts1 = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut bst in (var_field!((*st).body, Statement::NFStatement::WHILE).clone()).into_iter().cloned() {
            let __x = typeStatement(bst.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(Statement::NFStatement::WHILE { condition: e1.clone(), body: sts1.clone(), source: var_field!((*st).source, Statement::NFStatement::WHILE).clone() })
        },
        Deref @ Statement::FAILURE { .. } => {
            let mut sts1: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            sts1 = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut bst in (var_field!((*st).body, Statement::NFStatement::FAILURE).clone()).into_iter().cloned() {
            let __x = typeStatement(bst.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(Statement::NFStatement::FAILURE { body: sts1.clone(), source: var_field!((*st).source, Statement::NFStatement::FAILURE).clone() })
        },
        _ => {
            st.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(st)
}

pub fn checkAssignment(mut lhsExp: Arc<Expression::NFExpression>, mut rhsExp: Arc<Expression::NFExpression>, mut lhsVar: Variability, mut context: i32, mut info: SourceInfo) -> Result<()> {
    if InstContext::inInstanceAPI(context.clone()) {
        return Ok(());
    }
    let () = (::match_deref::match_deref! { match &(lhsExp.clone()) {
        Deref @ Expression::TUPLE { .. } => {
            let mut i: i32 = 0;
            i = 1;
            for mut e in &*var_field!((*lhsExp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut e = e.clone();
                checkAssignment(e.clone(), Expression::tupleElement(rhsExp.clone(), var_field!((*lhsExp).ty, Expression::NFExpression::TUPLE).clone(), i.clone())?, Expression::variability(e.clone())?, context.clone(), info.clone())?;
                i = i.clone() + 1;
            }
            ()
        },
        Deref @ Expression::CREF { .. } if (InstContext::inFunction(context.clone())) => {
            if ComponentRef::isCref(var_field!((*lhsExp).cref, Expression::NFExpression::CREF).clone()) && InstNode::isInput(ComponentRef::node(var_field!((*lhsExp).cref, Expression::NFExpression::CREF).clone())?) {
                Error::addSourceMessage(Error::ASSIGN_READONLY_ERROR.clone(), list![(literal!("input")).clone(), (ComponentRef::toString(var_field!((*lhsExp).cref, Expression::NFExpression::CREF).clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            ()
        },
        _ => {
            if lhsVar.clone() < Variability::DISCRETE.clone() {
                if lhsVar.clone() == Variability::CONSTANT.clone() {
                    Error::addSourceMessage(Error::ASSIGN_CONSTANT_ERROR.clone(), list![(Expression::toString(lhsExp.clone())?).clone(), (Expression::toString(rhsExp.clone())?).clone()], info.clone())?;
                    bail!("fail");
                } else if !(InstContext::inInitial(context.clone())) {
                    Error::addSourceMessage(Error::ASSIGN_PARAM_ERROR.clone(), list![(Expression::toString(lhsExp.clone())?).clone(), (Expression::toString(rhsExp.clone())?).clone()], info.clone())?;
                    bail!("fail");
                }
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeEqualityEquation(mut lhsExp: Arc<Expression::NFExpression>, mut rhsExp: Arc<Expression::NFExpression>, mut context: i32, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation>;
    let mut info: SourceInfo = ElementSource::getInfo(source.clone());
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    if InstContext::inWhen(context.clone()) && !(InstContext::inClocked(context.clone())) {
        if checkLhsInWhen(lhsExp.clone()) {
            Structural::markSubscriptsInExp(lhsExp.clone())?;
        } else {
            Error::addSourceMessage(Error::WHEN_EQ_LHS.clone(), list![(Expression::toString(lhsExp.clone())?).clone()], info.clone())?;
            bail!("fail");
        }
    }
    (e1, ty1, _, _) = typeExp(lhsExp.clone(), InstContext::set(context.clone(), InstContext::LHS.clone()), info.clone(), false)?;
    (e2, ty2, _, _) = typeExp(rhsExp.clone(), InstContext::set(context.clone(), InstContext::RHS.clone()), info.clone(), false)?;
    (e2, e1, ty, mk) = TypeCheck::matchExpressions(e2.clone(), ty2.clone(), e1.clone(), ty1.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk.clone()) {
        Error::addSourceMessage(Error::EQUATION_TYPE_MISMATCH_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(lhsExp.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(rhsExp.clone())?); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Type::toString(ty1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Type::toString(ty2.clone())?); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        bail!("fail");
    }
    eq = Equation::makeEquality(e1.clone(), e2.clone(), ty.clone(), source.clone(), scope.clone(), Equation::ScalarizeMode::NO_PREFERENCE.clone());
    if Expression::isExternalCall(e2.clone())? {
        Call::updateExternalRecordArgs(Expression::tupleElements(e1.clone()))?;
    }
    Ok(eq)
}

pub fn typeCondition(mut condition: Arc<Expression::NFExpression>, mut context: i32, mut source: Arc<DAE::ElementSource>, mut errorMsg: ErrorTypes::Message, mut allowVector: bool, mut allowClock: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability)> {
    let mut condition: Arc<Expression::NFExpression> = condition;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut info: SourceInfo;
    let mut ety: Arc<Type::NFType> = Arc::new(Type::ANY);
    info = ElementSource::getInfo(source.clone());
    (condition, ty, variability, _) = typeExp(condition.clone(), context.clone(), info.clone(), false)?;
    if allowVector.clone() && Type::isArray(ty.clone()) {
        ety = Type::unliftArray(ty.clone())?;
    } else {
        ety = ty.clone();
    }
    if !(Type::isBoolean(ety.clone()) || allowClock.clone() && Type::isClock(ety.clone())) {
        Error::addSourceMessage(errorMsg.clone(), list![(Expression::toString(condition.clone())?).clone(), (Type::toString(ty.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((condition, ty, variability))
}

pub fn typeIfEquation(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut context: i32, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut ifEq: Arc<Equation::NFEquation>;
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut accum_var: Variability = Variability::CONSTANT.clone();
    let mut var: Variability = Variability::CONSTANT;
    let mut bl: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut bl2: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut next_context: i32 = InstContext::set(context.clone(), InstContext::IF.clone());
    let mut cond_context: i32 = InstContext::set(next_context.clone(), InstContext::CONDITION.clone());
    for mut b in &*branches.clone() {
        let mut b = b.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: _, body: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa0.clone();
        eql = __pa1.clone();
        (cond, _, var) = typeCondition(cond.clone(), cond_context.clone(), source.clone(), Error::IF_CONDITION_TYPE_ERROR.clone(), false, false)?;
        if var.clone() > Variability::PARAMETER.clone() || Structural::isExpressionNotFixed(cond.clone(), false, 100)? {
            next_context = InstContext::set(next_context.clone(), InstContext::NONEXPANDABLE.clone());
        } else if var.clone() == Variability::PARAMETER.clone() && (accum_var.clone() <= Variability::PARAMETER.clone() || Equation::containsList(eql.clone(), Arc::new(Equation::isConnection))) {
            var = Variability::STRUCTURAL_PARAMETER.clone();
        }
        accum_var = Prefixes::variabilityMax(accum_var.clone(), var.clone());
        bl = cons(Arc::new(Equation::Branch::Branch::BRANCH { condition: cond.clone(), conditionVar: var.clone(), body: eql.clone() }), bl.clone());
    }
    for mut b in &*bl.clone() {
        let mut b = b.clone();
        let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa2, conditionVar: __pa3, body: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa2.clone();
        var = __pa3.clone();
        eql = __pa4.clone();
        ErrorExt::setCheckpoint((literal!("NFTyping.typeIfEquation")).clone());
        match '__try5: {
            eql = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (eql.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(typeEquation(e.clone(), next_context.clone()), '__try5);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            bl2 = cons(Equation::makeBranch(cond.clone(), eql.clone(), var.clone()), bl2.clone());
            Ok::<_, anyhow::Error>((bl2.clone(),))
        } {
            Ok((__try5_o0,)) => {
                bl2 = __try5_o0;
            }
            Err(_) => {
                bl2 = cons(Arc::new(Equation::Branch::Branch::INVALID_BRANCH { branch: Equation::makeBranch(cond.clone(), eql.clone(), var.clone()), errors: ErrorExt::getCheckpointMessages() }), bl2.clone());
            }
        }
        ErrorExt::delCheckpoint((literal!("NFTyping.typeIfEquation")).clone());
    }
    ifEq = Arc::new(Equation::NFEquation::IF { branches: bl2.clone(), scope: scope.clone(), source: source.clone() });
    Ok(ifEq)
}

pub fn typeWhenEquation(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut context: i32, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut whenEq: Arc<Equation::NFEquation>;
    let mut next_context: i32 = InstContext::set(context.clone(), InstContext::WHEN.clone());
    let mut accum_branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: _, body: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa0.clone();
        body = __pa1.clone();
        (cond, ty, var) = typeWhenCondition(cond.clone(), context.clone(), source.clone(), true)?;
        if Type::isClock(ty.clone()) {
            if (branches.clone().len() as i32) != 1 {
                if referenceEq(&branch.clone(),&listHead(branches.clone())?) {
                    Error::addSourceMessage(Error::ELSE_WHEN_CLOCK.clone(), metamodelica::nil(), ElementSource::getInfo(source.clone()))?;
                } else {
                    Error::addSourceMessage(Error::CLOCKED_WHEN_BRANCH.clone(), metamodelica::nil(), ElementSource::getInfo(source.clone()))?;
                }
                bail!("fail");
            } else {
                next_context = InstContext::set(context.clone(), InstContext::CLOCKED.clone());
            }
        }
        body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (body.clone()).into_iter().cloned() {
            let __x = typeEquation(eq.clone(), next_context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        accum_branches = cons(Equation::makeBranch(cond.clone(), body.clone(), var.clone()), accum_branches.clone());
    }
    whenEq = Arc::new(Equation::NFEquation::WHEN { branches: accum_branches.clone().reverse(), scope: scope.clone(), source: source.clone() });
    Ok(whenEq)
}

pub fn typeWhenCondition(mut condition: Arc<Expression::NFExpression>, mut context: i32, mut source: Arc<DAE::ElementSource>, mut allowClock: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability)> {
    let mut outCondition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    (outCondition, ty, variability) = typeCondition(condition.clone(), context.clone(), source.clone(), Error::WHEN_CONDITION_TYPE_ERROR.clone(), true, allowClock.clone())?;
    if variability.clone() > Variability::IMPLICITLY_DISCRETE.clone() && !(Type::isClock(ty.clone())) {
        Error::addSourceMessage(Error::NON_DISCRETE_WHEN_CONDITION.clone(), list![(Expression::toString(condition.clone())?).clone()], ElementSource::getInfo(source.clone()))?;
        bail!("fail");
    }
    if !(checkWhenInitial(outCondition.clone())?) {
        Error::addSourceMessage(Error::INITIAL_CALL_WARNING.clone(), list![(Expression::toString(condition.clone())?).clone()], ElementSource::getInfo(source.clone()))?;
    }
    Ok((outCondition, ty, variability))
}

pub fn checkWhenInitial(mut condition: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut invalid: bool = false;
    invalid = (::match_deref::match_deref! { match &(condition.clone()) {
        Deref @ Expression::ARRAY { .. } => {
            let __range0 = var_field!((*condition).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range0 {
                if checkWhenInitial(e.clone())? {
                    invalid = true;
                    return Ok(invalid);
                }
            }
            false
        },
        _ => !(Expression::containsShallow(condition.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static> = Arc::new({ let __pe_b1 = (literal!("initial")).clone(); move |__pe_a0| Expression::isCallNamed(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Expression::contains(__pe_a0, __pe_b1.clone()) }))?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(invalid)
}

pub fn typeOperatorArg(mut arg: Arc<Expression::NFExpression>, mut expectedType: Arc<Type::NFType>, mut context: i32, mut operatorName: ArcStr, mut argName: ArcStr, mut argIndex: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Variability)> {
    let mut arg: Arc<Expression::NFExpression> = arg;
    let mut var: Variability = Variability::CONSTANT;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    (arg, ty, var, _) = typeExp(arg.clone(), context.clone(), info.clone(), false)?;
    (arg, _, mk) = TypeCheck::matchTypes(ty.clone(), expectedType.clone(), arg.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk.clone()) {
        Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(intString(argIndex.clone())).clone(), (operatorName.clone()).clone(), (argName.clone()).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (Type::toString(expectedType.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((arg, var))
}

pub fn typeReinit(mut crefExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut context: i32, mut source: Arc<DAE::ElementSource>) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)> {
    let mut crefExp: Arc<Expression::NFExpression> = crefExp;
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut var: Variability = Variability::CONSTANT;
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut info: SourceInfo;
    info = ElementSource::getInfo(source.clone());
    (crefExp, ty1, _, _) = typeExp(crefExp.clone(), context.clone(), info.clone(), false)?;
    (exp, ty2, _, _) = typeExp(exp.clone(), context.clone(), info.clone(), false)?;
    cref = (::match_deref::match_deref! { match &(crefExp.clone()) {
        Deref @ Expression::CREF { .. } => {
            if ComponentRef::isIterator(var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone()) {
                Error::addSourceMessage(Error::ASSIGN_ITERATOR_ERROR.clone(), list![(ComponentRef::toString(var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone()
        },
        _ => {
            Error::addSourceMessage(Error::REINIT_MUST_BE_VAR_OR_ARRAY.clone(), metamodelica::nil(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if ComponentRef::nodeVariability(cref.clone())? < Variability::IMPLICITLY_DISCRETE.clone() {
        Error::addSourceMessage(Error::REINIT_MUST_BE_VAR.clone(), list![(Expression::toString(crefExp.clone())?).clone(), (Prefixes::variabilityString(ComponentRef::nodeVariability(cref.clone())?)?).clone()], info.clone())?;
        bail!("fail");
    }
    (_, _, mk) = TypeCheck::matchTypes(Type::arrayElementType(ty1.clone()), Arc::new(crate::NFType::REAL), crefExp.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk.clone()) {
        Error::addSourceMessage(Error::REINIT_MUST_BE_REAL.clone(), list![(Expression::toString(crefExp.clone())?).clone(), (Type::toString(Type::arrayElementType(ty1.clone()))?).clone()], info.clone())?;
        bail!("fail");
    }
    (exp, _, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), exp.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk.clone()) {
        Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (literal!("reinit")).clone(), (literal!("")).clone(), (Expression::toString(exp.clone())?).clone(), (Type::toString(ty2.clone())?).clone(), (Type::toString(ty1.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((crefExp, exp))
}

pub fn deduceIterationRangeEq(mut eq: Arc<Equation::NFEquation>, mut iterator: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut iterationRange: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>> = metamodelica::nil();
    crefs = Equation::foldExp(eq.clone(), Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0, __pe_a2| collectIteratorCrefs(__pe_a0, __pe_b1.clone(), __pe_a2) }), metamodelica::nil())?;
    iterationRange = deduceIterationRange(crefs.clone(), iterator.clone(), info.clone())?;
    Ok(iterationRange)
}

pub fn deduceIterationRangeStmt(mut stmt: Arc<Statement::NFStatement>, mut iterator: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut iterationRange: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>> = metamodelica::nil();
    crefs = Statement::foldExp(stmt.clone(), Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0, __pe_a2| collectIteratorCrefs(__pe_a0, __pe_b1.clone(), __pe_a2) }), metamodelica::nil())?;
    iterationRange = deduceIterationRange(crefs.clone(), iterator.clone(), info.clone())?;
    Ok(iterationRange)
}

pub fn deduceIterationRangeExp(mut exp: Arc<Expression::NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut iterationRange: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>> = metamodelica::nil();
    crefs = Expression::fold(exp.clone(), Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0, __pe_a2| collectIteratorCrefs2(__pe_a0, __pe_b1.clone(), __pe_a2) }), metamodelica::nil())?;
    iterationRange = deduceIterationRange(crefs.clone(), iterator.clone(), info.clone())?;
    Ok(iterationRange)
}

pub fn deduceIterationRange(mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>, mut iterator: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut iterationRange: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut range_cr: (Arc<ComponentRef::NFComponentRef>, i32);
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut dim_index: i32 = 0;
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut start_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stop_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if crefs.clone().is_empty() {
        Error::addSourceMessage(Error::IMPLICIT_ITERATOR_NOT_FOUND_IN_LOOP_BODY.clone(), list![(InstNode::name(iterator.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    range_cr = List::reduce(crefs.clone(), Arc::new({ let __pe_b2 = info.clone(); move |__pe_a0, __pe_a1| deduceIterationRange2(__pe_a0, __pe_a1, __pe_b2.clone()) }))?;
    (cr, dim_index) = range_cr.clone();
    dim = Type::nthDimension(InstNode::getType(ComponentRef::node(cr.clone())?)?, dim_index.clone())?;
    start_exp = Dimension::lowerBoundExp(dim.clone())?;
    stop_exp = Dimension::endExp(dim.clone(), Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::UNKNOWN), cref: cr.clone() }), dim_index.clone())?;
    iterationRange = Arc::new(Expression::NFExpression::RANGE { ty: Arc::new(crate::NFType::UNKNOWN), start: start_exp.clone(), step: None, stop: stop_exp.clone() });
    Ok(iterationRange)
}

pub fn collectIteratorCrefs(mut exp: Arc<Expression::NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>> {
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>> = crefs;
    crefs = Expression::fold(exp.clone(), Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0, __pe_a2| collectIteratorCrefs2(__pe_a0, __pe_b1.clone(), __pe_a2) }), crefs.clone())?;
    Ok(crefs)
}

pub fn collectIteratorCrefs2(mut exp: Arc<Expression::NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>> {
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>> = crefs;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut index: i32 = 0;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref, .. } => {
            let mut cref = (*cref).clone();
            while ComponentRef::isCref(cref.clone()) {
                (cref, subs) = ComponentRef::stripSubscripts(cref.clone());
                index = 1;
                for mut sub in &*subs.clone() {
                    let mut sub = sub.clone();
                    if Subscript::equalsIterator(sub.clone(), iterator.clone())? {
                        crefs = cons((cref.clone(), index.clone()), crefs.clone());
                    }
                    index = index.clone() + 1;
                }
                cref = ComponentRef::rest(cref.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

pub fn deduceIterationRange2(mut range1: (Arc<ComponentRef::NFComponentRef>, i32), mut range2: (Arc<ComponentRef::NFComponentRef>, i32), mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, i32)> {
    let mut range: (Arc<ComponentRef::NFComponentRef>, i32) = range2.clone();
    let mut cref1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cref2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut index1: i32 = 0;
    let mut index2: i32 = 0;
    let mut node1: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut node2: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut dim1: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim2: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    (cref1, index1) = range1.clone();
    (cref2, index2) = range2.clone();
    node1 = ComponentRef::node(cref1.clone())?;
    node2 = ComponentRef::node(cref2.clone())?;
    if index1.clone() == index2.clone() && InstNode::refEqual(node1.clone(), node2.clone()) {
        return Ok(range);
    }
    dim1 = Type::nthDimension(InstNode::getType(node1.clone())?, index1.clone())?;
    dim2 = Type::nthDimension(InstNode::getType(node2.clone())?, index2.clone())?;
    if !(Dimension::isEqualKnownSize(dim1.clone(), node1.clone(), index1.clone(), dim2.clone(), node2.clone(), index2.clone())?) {
        Error::addSourceMessage(Error::INCOMPATIBLE_IMPLICIT_RANGES.clone(), list![ArcStr::from(::std::format!("{}", index1.clone())), (ComponentRef::toString(cref1.clone())?).clone(), ArcStr::from(::std::format!("{}", index2.clone())), (ComponentRef::toString(cref2.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    Ok(range)
}

