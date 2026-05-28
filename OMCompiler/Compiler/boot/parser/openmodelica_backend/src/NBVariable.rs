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

use crate::NBAdjacency::Mapping;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation as BEquation;
use crate::NBEquation::Iterator;
use crate::NBSlice as Slice;
use crate::NBackendDAE as BackendDAE;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_nf_frontend::NFAttributes as Attributes;
use openmodelica_nf_frontend::NFBackendExtension as BackendExtension;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::OptimizerExpression;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::TearingSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltin;
use openmodelica_nf_frontend::NFCeval as Ceval;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFScalarize as Scalarize;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

//OF Imports
//NF Imports
// Backend Imports
//Util Imports
// mainly used for mapping purposes
pub type VariablePointer = Pointer::Pointer<Arc<Variable::NFVariable>>;

pub type VarSlice = Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;

// ==========================================================================
//               Single Variable constants and functions
// ==========================================================================
thread_local! { static __DUMMY_VARIABLE_TLS: Arc<Variable::NFVariable> = Arc::new(Variable::NFVariable { name: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY), ty: Arc::new(openmodelica_nf_frontend::NFType::ANY), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone() }); }
pub fn DUMMY_VARIABLE() -> Arc<Variable::NFVariable> { __DUMMY_VARIABLE_TLS.with(|__t| __t.clone()) }

thread_local! { static __SUBST_VARIABLE_TLS: Arc<Variable::NFVariable> = Arc::new(Variable::NFVariable { name: NFBuiltin::SUBST_CREF().clone(), ty: Arc::new(openmodelica_nf_frontend::NFType::ANY), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone() }); }
pub fn SUBST_VARIABLE() -> Arc<Variable::NFVariable> { __SUBST_VARIABLE_TLS.with(|__t| __t.clone()) }

thread_local! { static __TIME_VARIABLE_TLS: Arc<Variable::NFVariable> = Arc::new(Variable::NFVariable { name: NFBuiltin::TIME_CREF().clone(), ty: Arc::new(openmodelica_nf_frontend::NFType::REAL), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: Arc::new(BackendInfo::BackendInfo { varKind: Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::TIME), attributes: BackendExtension::EMPTY_VAR_ATTR_REAL().clone(), annotations: BackendExtension::EMPTY_ANNOTATIONS.clone(), var_pre: None, var_seed: None, var_pder: None, var_start: None, parent: None }) }); }
pub fn TIME_VARIABLE() -> Arc<Variable::NFVariable> { __TIME_VARIABLE_TLS.with(|__t| __t.clone()) }

pub const DERIVATIVE_STR: &'static str = "$DER";

pub const DUMMY_DERIVATIVE_STR: &'static str = "$dDER";

pub const PARTIAL_DERIVATIVE_STR: &'static str = "$pDER";

pub const FUNCTION_DERIVATIVE_STR: &'static str = "$fDER";

pub const FUNCTION_STR: &'static str = "$FUN";

pub const PREVIOUS_STR: &'static str = "$PRE";

pub const AUXILIARY_STR: &'static str = "$AUX";

pub const STATE_ALIAS_STR: &'static str = "$STA";

pub const START_STR: &'static str = "$START";

pub const RESIDUAL_STR: &'static str = "$RES";

pub const TEMPORARY_STR: &'static str = "$TMP";

pub const SEED_STR: &'static str = "$SEED";

pub const TIME_EVENT_STR: &'static str = "$TEV";

pub const STATE_EVENT_STR: &'static str = "$SEV";

pub const CLOCK_STR: &'static str = "$CLK";

pub fn toString(mut var: Arc<Variable::NFVariable>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut attr: ArcStr = arcstr::literal!("");
    attr = (BackendExtension::VariableAttributes::toString(var.backendinfo.attributes.clone())?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*BackendExtension::VariableKind::toString(var.backendinfo.varKind.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(Variable::size(var.clone(), true))); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*Variable::toString(var.clone(), (literal!("")).clone(), false)?); __mm_s.push_str(&*if (attr.clone() == literal!("")) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*attr.clone()); ArcStr::from(__mm_s) }}); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn pointerToString(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> ArcStr {
    let mut r#str: ArcStr = toString(Pointer::access(var_ptr.clone()), (literal!("")).clone()).unwrap();
    r#str
}

pub fn nameString(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> ArcStr {
    let mut r#str: ArcStr = ComponentRef::toString(getVarName(var_ptr.clone())).unwrap();
    r#str
}

pub fn hash(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> i32 {
    let mut i: i32 = Variable::hash(Pointer::access(var_ptr.clone()));
    i
}

pub fn equalName(mut var_ptr1: Pointer::Pointer<Arc<Variable::NFVariable>>, mut var_ptr2: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = Variable::equalName(Pointer::access(var_ptr1.clone()), Pointer::access(var_ptr2.clone()));
    b
}

pub fn size(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut resize: bool) -> i32 {
    let mut s: i32 = Variable::size(Pointer::access(var_ptr.clone()), resize.clone());
    s
}

pub fn applyToType(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>) -> () {
    pub type typeFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>;

    let mut new: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    new = Variable::applyToType(var.clone(), func.clone());
    if !(referenceEq(&var.clone(),&new.clone())) {
        Pointer::update(var_ptr.clone(), new.clone());
    }
    ()
}

pub fn fromCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut attr: Arc<Attributes::NFAttributes>, mut binding: Arc<Binding::NFBinding>) -> Result<Arc<Variable::NFVariable>> {
    let mut variable: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut class_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut child_nodes: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut vis: Prefixes::Visibility = Prefixes::Visibility::PUBLIC;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut children: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    node = ComponentRef::node(cref.clone())?;
    ty = ComponentRef::getSubscriptedType(cref.clone(), true)?;
    vis = InstNode::visibility(node.clone());
    info = InstNode::info(node.clone())?;
    if !(Type::isExternalObject(ty.clone())) {
        children = (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::COMPLEX { cls: class_node, .. } => {
            child_nodes = Class::getComponents(InstNode::getClass(class_node.clone())?);
            children = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut c in (child_nodes.clone()).borrow().iter() {
            let __x = fromCref(ComponentRef::prefixCref(c.clone(), InstNode::getType(c.clone())?, metamodelica::nil(), cref.clone()), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            children.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    variable = Arc::new(Variable::NFVariable { name: cref.clone(), ty: ty.clone(), binding: binding.clone(), visibility: vis.clone(), attributes: attr.clone(), typeAttributes: metamodelica::nil(), children: children.clone(), comment: SCode::noComment.clone(), info: info.clone(), backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone() });
    Ok(variable)
}

pub fn makeVarPtrCyclic(mut var: Arc<Variable::NFVariable>, mut name: Arc<ComponentRef::NFComponentRef>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut name: Arc<ComponentRef::NFComponentRef> = name;
    var_ptr = Pointer::create(var.clone());
    name = BackendDAE::lowerComponentReferenceInstNode(name.clone(), var_ptr.clone())?;
    assign_field!(var.name = name.clone());
    Pointer::update(var_ptr.clone(), var.clone());
    Ok((var_ptr, name))
}

pub fn connectPartners(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut par_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut func: Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>) -> () {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut par: Arc<Variable::NFVariable> = Pointer::access(par_ptr.clone());
    assign_field!(var.backendinfo = func(var.backendinfo.clone(), Some(par_ptr.clone())).unwrap());
    assign_field!(par.backendinfo = func(par.backendinfo.clone(), Some(var_ptr.clone())).unwrap());
    Pointer::update(var_ptr.clone(), var.clone());
    Pointer::update(par_ptr.clone(), par.clone());
    ()
}

pub fn removePartner(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut func: Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>) -> () {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(var.backendinfo = func(var.backendinfo.clone(), None).unwrap());
    Pointer::update(var_ptr.clone(), var.clone());
    ()
}

pub fn getVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    var = Pointer::access(getVarPointer(cref.clone(), info.clone())?);
    Ok(var)
}

// The following functions provide layers of protection. Whenever accessing names or pointers use these!
pub fn getVarPointer(mut cref: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    var = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::VAR_NODE { varPointer, .. }, .. } => {
            varPointer.clone()
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::NAME_NODE { .. }, .. } => {
            Pointer::create(DUMMY_VARIABLE().clone())
        },
        Deref @ ComponentRef::WILD => {
            Pointer::create(DUMMY_VARIABLE().clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getVarPointer")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(", because of wrong InstNode (not VAR_NODE). Show lowering errors with -d=failtrace.")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn getVarName(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<ComponentRef::NFComponentRef> {
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    name = var.name.clone();
    name
}

pub fn setVarName(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut name: Arc<ComponentRef::NFComponentRef>) -> Pointer::Pointer<Arc<Variable::NFVariable>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(var.name = name.clone());
    Pointer::update(var_ptr.clone(), var.clone());
    var_ptr
}

pub fn subIdxName(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut index: Pointer::Pointer<i32>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(var.name = ComponentRef::rename(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::firstName(var.name.clone(), false)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(Pointer::access(index.clone()))); ArcStr::from(__mm_s) }).clone(), var.name.clone())?);
    var_ptr = Pointer::create(var.clone());
    Ok(var_ptr)
}

pub fn getVarKind(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<VariableKind::VariableKind> {
    let mut kind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    kind = BackendExtension::BackendInfo::getVarKind(var.backendinfo.clone());
    kind
}

pub fn toExpression(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = Expression::fromCref(getVarName(var_ptr.clone()), false).unwrap();
    exp
}

pub type checkVar = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>;

pub fn isArray(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = Type::isArray(var.ty.clone());
    b
}

pub fn getDimensions(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    var = Pointer::access(var_ptr.clone());
    dims = Type::arrayDims(var.ty.clone());
    dims
}

pub fn isEmpty(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = ComponentRef::isEmpty(var.name.clone());
    b
}

pub fn isForcedState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE { natural, .. } => {
            !(natural.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStateDerivative(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE_DER { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isAlgebraic(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::ALGEBRAIC => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStart(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::START { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isExtObj(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::EXTOBJ { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isTime(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::TIME => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isContinuous(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut init: bool) -> Result<bool> {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DISCRETE_STATE => false,
        Deref @ BackendExtension::VariableKind::DISCRETE => false,
        Deref @ BackendExtension::VariableKind::PREVIOUS => false,
        Deref @ BackendExtension::VariableKind::CONSTANT => false,
        Deref @ BackendExtension::VariableKind::ITERATOR => false,
        Deref @ BackendExtension::VariableKind::EXTOBJ { .. } => false,
        Deref @ BackendExtension::VariableKind::PARAMETER { .. } => init.clone() && Type::isContinuous(var.ty.clone())?,
        Deref @ BackendExtension::VariableKind::RECORD { .. } => List::all(getRecordChildren(var_ptr.clone()), Arc::new({ let __pe_b1 = init.clone(); move |__pe_a0| isContinuous(__pe_a0, __pe_b1.clone()) })),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isDiscontinuous(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut init: bool) -> Result<bool> {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isContinuous(var_ptr.clone(), init.clone())?);
    Ok(b)
}

pub fn isContinuousRecordAware(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut init: bool) -> Result<bool> {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match getParent(var_ptr.clone()) {
        Some(mut parent) => {
            isContinuousRecordAware(parent.clone(), init.clone())?
        },
        _ => {
            isContinuous(var_ptr.clone(), init.clone())?
        },
    });
    Ok(b)
}

pub fn isDiscreteState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DISCRETE_STATE => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isDiscrete(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DISCRETE => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isPrevious(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::PREVIOUS => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isRecord(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isKnownRecord(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::RECORD { max_var: variability, .. } if (variability.clone() < Prefixes::Variability::DISCRETE.clone()) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isUnknownRecord(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::RECORD { min_var: variability, .. } if (variability.clone() > Prefixes::Variability::NON_STRUCTURAL_PARAMETER.clone()) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isClock(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::CLOCK => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isClocked(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::CLOCKED => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isClockOrClocked(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::CLOCK => true,
        Deref @ BackendExtension::VariableKind::CLOCKED => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isIterator(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::ITERATOR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isPDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::JAC_VAR => true,
        Deref @ BackendExtension::VariableKind::JAC_TMP_VAR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hasTearingSelect(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut compareTS: TearingSelect, mut func: Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>) -> bool {
    pub type compare = std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>;

    let mut b: bool = func(((getTearingSelect(varPointer.clone()).unwrap()) as i32), ((compareTS.clone()) as i32)).unwrap();
    b
}

pub type getVarPartner = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>;

pub fn getVarPre(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr = arcstr::literal!("");
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("pre variable")).clone();
    partner = var.backendinfo.var_pre.clone();
    (partner, partnerName)
}

pub fn getVarSeed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr = arcstr::literal!("");
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("seed variable")).clone();
    partner = var.backendinfo.var_seed.clone();
    (partner, partnerName)
}

pub fn getVarPDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr = arcstr::literal!("");
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("partial derivative")).clone();
    partner = var.backendinfo.var_pder.clone();
    (partner, partnerName)
}

pub fn getVarDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr = arcstr::literal!("");
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("derivative")).clone();
    partner = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE { derivative: partner, .. } => partner.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (partner, partnerName)
}

pub fn getVarState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr = arcstr::literal!("");
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("state")).clone();
    partner = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE_DER { state: p, .. } => {
            Some(p.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (partner, partnerName)
}

pub fn getVarDummyDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr = arcstr::literal!("");
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("dummy derivative")).clone();
    partner = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DUMMY_STATE { dummy_der: p } => {
            Some(p.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (partner, partnerName)
}

pub fn getVarStart(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr = arcstr::literal!("");
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("start")).clone();
    partner = var.backendinfo.var_start.clone();
    (partner, partnerName)
}

pub fn getPartnerCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>, mut scalarized: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut partner_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr = arcstr::literal!("");
    (partner, partnerName) = func(getVarPointer(cref.clone(), metamodelica::sourceInfo!())?)?;
    if isSome(partner.clone()) {
        partner_cref = getVarName(Util::getOption(partner.clone())?);
        if !(scalarized.clone()) {
            partner_cref = ComponentRef::copySubscripts(cref.clone(), partner_cref.clone())?;
        }
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getPartnerCref")); __mm_s.push_str(&*literal!(" failed because ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" has no corresponding ")); __mm_s.push_str(&*partnerName.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(partner_cref)
}

pub fn hasStartAttr(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = Util::isSome(BackendExtension::VariableAttributes::getStartAttribute(var.backendinfo.attributes.clone()));
    b
}

pub fn hasPre(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isPrevious(var_ptr.clone())) && Util::isSome(getVarPre(var_ptr.clone()));
    b
}

pub fn isJacobianResultVar(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match getVarPDer(var_ptr.clone()) {
        Some(mut der_var) => {
            isJacobianResultVarPDer(der_var.clone())
        },
        _ => {
            false
        },
    });
    b
}

pub fn isJacobianResultVarPDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::JAC_VAR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isDummyState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DUMMY_STATE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isDummyDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DUMMY_DER { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isParamOrConst(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::PARAMETER { .. } => true,
        Deref @ BackendExtension::VariableKind::CONSTANT => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isConst(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::CONSTANT => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isKnown(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::PARAMETER { .. } => true,
        Deref @ BackendExtension::VariableKind::CONSTANT => true,
        Deref @ BackendExtension::VariableKind::STATE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isOptimizable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizable: true, .. }, varKind: Deref @ BackendExtension::VariableKind::PARAMETER { .. }, .. } => true,
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizable: true, .. }, .. } if (isInput(var_ptr.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isInitialTime(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(optExp), .. }, .. } => optExp.clone() == OptimizerExpression::INITIAL_TIME.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isFinalTime(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(optExp), .. }, .. } => optExp.clone() == OptimizerExpression::FINAL_TIME.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isLagrangeOrPathConstraint(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(optExp), .. }, .. } => optExp.clone() == OptimizerExpression::LAGRANGE.clone() || optExp.clone() == OptimizerExpression::PATH_CONSTRAINT.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isMayerOrFinalConstraint(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(optExp), .. }, .. } => optExp.clone() == OptimizerExpression::MAYER.clone() || optExp.clone() == OptimizerExpression::FINAL_CONSTRAINT.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isInitialConstraint(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(optExp), .. }, .. } => optExp.clone() == OptimizerExpression::INITIAL_CONSTRAINT.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStateOrOptimizable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = isState(var_ptr.clone()) || isOptimizable(var_ptr.clone());
    b
}

pub fn isResizable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = List::any(Type::arrayDims(var.ty.clone()), (std::sync::Arc::new(fnptr!(Dimension::isResizable, Arc<Dimension::NFDimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<bool> + 'static>));
    b
}

pub fn isResizableParameter(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { resizable: true, .. }, varKind: Deref @ BackendExtension::VariableKind::PARAMETER { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn updateResizableParameter(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> () {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut val: Option<Arc<Expression::NFExpression>> = UnorderedMap::get(var.name.clone(), optimal_values.clone());
    let () = (::match_deref::match_deref! { match &((val.clone(), var.backendinfo.clone())) {
        (Some(Deref @ Expression::INTEGER { value: i }), Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { resizable: true, .. }, varKind: varKind @ Deref @ BackendExtension::VariableKind::PARAMETER { .. }, .. }) => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::PARAMETER; resize_value = Some(i.clone()));
            setVarKind(var_ptr.clone(), varKind.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn getResizableValue(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> {
    let mut val: i32 = 0;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let _ = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::PARAMETER { resize_value: Some(val) }, .. } => val.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getResizableValue")); __mm_s.push_str(&*literal!(" failed because following variable is not a resizable parameter: ")); __mm_s.push_str(&*toString(var.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(val)
}

pub fn isResidual(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::RESIDUAL_VAR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isSeed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::SEED_VAR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isInput(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = var.attributes.direction.clone() == Prefixes::Direction::INPUT.clone();
    b
}

pub fn isOutput(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = var.attributes.direction.clone() == Prefixes::Direction::OUTPUT.clone();
    b
}

pub fn isFixed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.attributes.clone()) {
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_REAL { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_INT { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_STRING { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isFixable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE { .. } => !(isFixed(var_ptr.clone())),
        Deref @ BackendExtension::VariableKind::DISCRETE_STATE => !(isFixed(var_ptr.clone())) || hasPre(var_ptr.clone()),
        Deref @ BackendExtension::VariableKind::PARAMETER { .. } => !(isFixed(var_ptr.clone())),
        Deref @ BackendExtension::VariableKind::PREVIOUS => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStateSelect(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut stateSelect: StateSelect) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = BackendExtension::VariableAttributes::getStateSelect(var.backendinfo.attributes.clone()) == stateSelect.clone();
    b
}

pub fn setVariableAttributes(mut var: Arc<Variable::NFVariable>, mut variableAttributes: Arc<VariableAttributes::VariableAttributes>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = variableAttributes.clone());
            assign_field!(var.backendinfo = backendinfo.clone());
            var.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub fn setMin(mut var: Arc<Variable::NFVariable>, mut min_val: Option<Arc<Expression::NFExpression>>, mut overwrite: bool) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setMin(variableAttributes.clone(), min_val.clone(), overwrite.clone()));
            assign_field!(var.backendinfo = backendinfo.clone());
            var.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub fn setMax(mut var: Arc<Variable::NFVariable>, mut max_val: Option<Arc<Expression::NFExpression>>, mut overwrite: bool) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setMax(variableAttributes.clone(), max_val.clone(), overwrite.clone()));
            assign_field!(var.backendinfo = backendinfo.clone());
            var.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub fn setStartAttribute(mut var: Arc<Variable::NFVariable>, mut start_val: Arc<Expression::NFExpression>, mut overwrite: bool) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setStartAttribute(variableAttributes.clone(), start_val.clone(), overwrite.clone()));
            assign_field!(var.backendinfo = backendinfo.clone());
            var.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub fn setStateSelect(mut var: Arc<Variable::NFVariable>, mut stateSelect_val: StateSelect, mut overwrite: bool) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setStateSelect(variableAttributes.clone(), stateSelect_val.clone(), overwrite.clone()));
            assign_field!(var.backendinfo = backendinfo.clone());
            var.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub fn setTearingSelect(mut var: Arc<Variable::NFVariable>, mut tearingSelect_val: TearingSelect, mut overwrite: bool) -> Arc<Variable::NFVariable> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setTearingSelect(variableAttributes.clone(), tearingSelect_val.clone(), overwrite.clone()));
            assign_field!(var.backendinfo = backendinfo.clone());
            var.clone()
        },
        _ => {
            var.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    var
}

pub fn getTearingSelect(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<TearingSelect> {
    let mut tearingSelect_val: TearingSelect = TearingSelect::NEVER;
    tearingSelect_val = (::match_deref::match_deref! { match &(Pointer::access(varPointer.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            BackendExtension::VariableAttributes::getTearingSelect(variableAttributes.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getTearingSelect")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*pointerToString(varPointer.clone())); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tearingSelect_val)
}

pub fn setVarKind(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut varKind: Arc<VariableKind::VariableKind>) -> () {
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    var = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
    Pointer::update(varPointer.clone(), var.clone());
    ()
}

pub fn setParent(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Pointer::Pointer<Arc<Variable::NFVariable>> {
    let mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>> = varPointer;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setParent(var.backendinfo.clone(), parent.clone()));
    Pointer::update(varPointer.clone(), var.clone());
    varPointer
}

pub fn getParent(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Option<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut parent: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(varPointer.clone());
    parent = var.backendinfo.parent.clone();
    parent
}

pub fn isDummyVariable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::FRONTEND_DUMMY => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isArtificial(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith((ComponentRef::firstName(getVarName(var_ptr.clone()), false)?).clone(), (literal!("$")).clone());
    Ok(b)
}

pub fn isFunctionAlias(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith((ComponentRef::firstName(getVarName(var_ptr.clone()), false)?).clone(), (arcstr::literal!(FUNCTION_STR)).clone());
    Ok(b)
}

pub fn isClockAlias(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith((ComponentRef::firstName(getVarName(var_ptr.clone()), false)?).clone(), (arcstr::literal!(CLOCK_STR)).clone());
    Ok(b)
}

pub fn createTimeVar() -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut var: Arc<Variable::NFVariable> = TIME_VARIABLE().clone();
    (var_ptr, _) = makeVarPtrCyclic(var.clone(), var.name.clone())?;
    Ok(var_ptr)
}

pub fn setStateDerivativeVar(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut derivative: Pointer::Pointer<Arc<Variable::NFVariable>>) -> () {
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    var = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::STATE { index: 1, derivative: Some(derivative.clone()), natural: true })));
    Pointer::update(varPointer.clone(), var.clone());
    ()
}

pub fn makeAlgStateVar(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> () {
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    if isAlgebraic(varPointer.clone()) {
        var = Pointer::access(varPointer.clone());
        assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::ALG_STATE)));
        Pointer::update(varPointer.clone(), var.clone());
    }
    ()
}

pub fn makeDerVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut scalarized: bool) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut der_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut state_cref: Arc<ComponentRef::NFComponentRef> = if (scalarized.clone()) {cref.clone()} else {ComponentRef::stripSubscriptsAll(cref.clone())};
    let () = ({
        let mut dummy_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = Pointer::create(DUMMY_VARIABLE().clone());
        (::match_deref::match_deref! { match &(ComponentRef::node(state_cref.clone())?) {
        Deref @ InstNode::VAR_NODE { .. } => {
            let mut derNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut state: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            state = getVarPointer(state_cref.clone(), metamodelica::sourceInfo!())?;
            derNode = Arc::new(InstNode::InstNode::VAR_NODE { name: (arcstr::literal!(DERIVATIVE_STR)).clone(), varPointer: dummy_ptr.clone() });
            der_cref = ComponentRef::append(state_cref.clone(), ComponentRef::fromNode(derNode.clone(), ComponentRef::scalarType(state_cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
            var = fromCref(ComponentRef::stripSubscriptsAll(der_cref.clone()), Variable::attributes(Pointer::access(state.clone())), Binding::EMPTY_BINDING().clone())?;
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::STATE_DER { state: state.clone(), alias: None })));
            (var_ptr, der_cref) = makeVarPtrCyclic(var.clone(), der_cref.clone())?;
            if !(scalarized.clone()) {
                der_cref = ComponentRef::copySubscripts(cref.clone(), der_cref.clone())?;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeDerVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((der_cref, var_ptr))
}

pub fn hasDerVar(mut state_var: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(Pointer::access(state_var.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::STATE { derivative: Some(_), .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn addRecordChild(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut child: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = cons(child.clone(), var_field!((*varKind).children, VariableKind::VariableKind::RECORD).clone()));
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
            var.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.addRecordChild")); __mm_s.push_str(&*literal!(" failed adding ")); __mm_s.push_str(&*ComponentRef::toString(getVarName(child.clone()))?); __mm_s.push_str(&*literal!(" as a child to ")); __mm_s.push_str(&*ComponentRef::toString(getVarName(var_ptr.clone()))?); __mm_s.push_str(&*literal!(" because it is not a record.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr.clone(), var.clone());
    Ok(())
}

pub fn setRecordChildren(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = children.clone());
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
            var.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setRecordChildren")); __mm_s.push_str(&*literal!(" failed adding new children to ")); __mm_s.push_str(&*ComponentRef::toString(getVarName(var_ptr.clone()))?); __mm_s.push_str(&*literal!(" because it is not a record.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr.clone(), var.clone());
    Ok(())
}

pub fn getRecordChildren(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    children = (::match_deref::match_deref! { match &(Pointer::access(var.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            var_field!((**varKind).children, VariableKind::VariableKind::RECORD).clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    children
}

pub fn getRecordChildrenCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut children: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut arg_children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    subscripts = ComponentRef::subscriptsAllFlat(cref.clone());
    arg_children = getRecordChildren(getVarPointer(cref.clone(), metamodelica::sourceInfo!())?);
    children = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut child in (arg_children.clone()).into_iter().cloned() {
            let __x = ComponentRef::mergeSubscripts(subscripts.clone(), getVarName(child.clone()), true, true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(children)
}

pub fn getRecordChildrenCrefOrSelf(mut cref: Arc<ComponentRef::NFComponentRef>) -> Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> {
    let mut children: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = getRecordChildrenCref(cref.clone()).unwrap();
    children = if (children.clone().is_empty()) {list![cref.clone()]} else {children.clone()};
    children
}

pub fn makeDummyState(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut derivative: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    var = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = (::match_deref::match_deref! { match &(BackendExtension::BackendInfo::getVarKind(var.backendinfo.clone())) {
        Deref @ BackendExtension::VariableKind::STATE { derivative: Some(derivative), .. } => {
            let mut der_var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            der_var = Pointer::access(derivative.clone());
            assign_field!(
                der_var.backendinfo = BackendExtension::BackendInfo::setVarKind(der_var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::DUMMY_DER { dummy_state: varPointer.clone() })),
                der_var.backendinfo = BackendExtension::BackendInfo::setStateSelect(der_var.backendinfo.clone(), BackendExtension::StateSelect::AVOID.clone(), false)
            );
            Pointer::update(derivative.clone(), der_var.clone());
            BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::DUMMY_STATE { dummy_der: derivative.clone() }))
        },
        Deref @ BackendExtension::VariableKind::DUMMY_STATE { dummy_der: derivative } => {
            var.backendinfo.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeDummyState")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(getVarName(varPointer.clone()))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }));
    Pointer::update(varPointer.clone(), var.clone());
    Ok(derivative)
}

pub fn makeDiscreteStateVar(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> () {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::DISCRETE_STATE)));
    Pointer::update(varPointer.clone(), var.clone());
    ()
}

pub fn makePreVar(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut pre_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut pre_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut pre: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut qual = (*qual).clone();
            var_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(PREVIOUS_STR));
            pre_cref = ComponentRef::append(cref.clone(), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
            pre = fromCref(pre_cref.clone(), Variable::attributes(Pointer::access(var_ptr.clone())), Binding::EMPTY_BINDING().clone())?;
            assign_field!(pre.backendinfo = BackendExtension::BackendInfo::setVarKind(pre.backendinfo.clone(), Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::PREVIOUS)));
            (pre_ptr, pre_cref) = makeVarPtrCyclic(pre.clone(), pre_cref.clone())?;
            connectPartners(var_ptr.clone(), pre_ptr.clone(), (std::sync::Arc::new(fnptr!(BackendInfo::setVarPre, Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>));
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makePreVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((pre_cref, pre_ptr))
}

pub fn makeSeedVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut name: ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut ovar: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
            let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut varKind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
            let mut qual = (*qual).clone();
            old_var_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
            (ovar, _) = getVarSeed(old_var_ptr.clone());
            if isSome(ovar.clone()) {
                var_ptr = Util::getOption(ovar.clone())?;
                cref = getVarName(var_ptr.clone());
            } else {
                assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(SEED_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) });
                cref = ComponentRef::append(cref.clone(), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
                var = fromCref(cref.clone(), Attributes::IMPL_DISCRETE_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
                varKind = (::match_deref::match_deref! { match &(getVarKind(old_var_ptr.clone())) {
        varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = metamodelica::nil());
            varKind.clone()
        },
        _ => Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::SEED_VAR),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
                (var_ptr, cref) = makeVarPtrCyclic(var.clone(), cref.clone())?;
                connectPartners(old_var_ptr.clone(), var_ptr.clone(), (std::sync::Arc::new(fnptr!(BackendInfo::setVarSeed, Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>));
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeSeedVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, var_ptr))
}

pub fn makePDerVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut name: ArcStr, mut isTmp: bool) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut res_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut ovar: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
            let mut varKind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
            let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut qual = (*qual).clone();
            res_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
            (ovar, _) = getVarPDer(res_ptr.clone());
            if isSome(ovar.clone()) {
                var_ptr = Util::getOption(ovar.clone())?;
                cref = getVarName(var_ptr.clone());
            } else {
                assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(PARTIAL_DERIVATIVE_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) });
                cref = ComponentRef::append(cref.clone(), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
                var = fromCref(cref.clone(), Variable::attributes(Pointer::access(res_ptr.clone())), Binding::EMPTY_BINDING().clone())?;
                varKind = (::match_deref::match_deref! { match &(getVarKind(res_ptr.clone())) {
        varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = metamodelica::nil());
            varKind.clone()
        },
        _ => if (isTmp.clone()) {Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::JAC_TMP_VAR)} else {Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::JAC_VAR)},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
                (var_ptr, cref) = makeVarPtrCyclic(var.clone(), cref.clone())?;
                connectPartners(res_ptr.clone(), var_ptr.clone(), (std::sync::Arc::new(fnptr!(BackendInfo::setVarPDer, Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>));
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makePDerVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, var_ptr))
}

pub fn makeFDerVar(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { restCref: Deref @ ComponentRef::EMPTY, .. } => (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::COMPONENT_NODE { .. } => {
            let mut qual = (*qual).clone();
            assign_variant_field!(qual => InstNode::InstNode::COMPONENT_NODE; name = BackendUtil::makeFDerString((ComponentRef::toString(cref.clone())?).clone(), None));
            ComponentRef::fromNode(qual.clone(), ComponentRef::nodeType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())
        },
        qual @ Deref @ InstNode::CLASS_NODE { .. } => {
            let mut qual = (*qual).clone();
            assign_variant_field!(qual => InstNode::InstNode::CLASS_NODE; name = BackendUtil::makeFDerString((ComponentRef::toString(cref.clone())?).clone(), None));
            ComponentRef::fromNode(qual.clone(), ComponentRef::nodeType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeFDerVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ ComponentRef::CREF { .. } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = makeFDerVar(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone())?);
            cref.clone()
        },
        _ => cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn makeStartVar(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut start_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    (start_cref, var_ptr) = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut old_var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut qual = (*qual).clone();
            old_var_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
            (start_cref, var_ptr) = (match getVarStart(old_var_ptr.clone()) {
        Some(mut var_ptr) => (getVarName(var_ptr.clone()), var_ptr.clone()),
        _ => {
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(START_STR));
            start_cref = ComponentRef::append(ComponentRef::stripSubscriptsAll(cref.clone()), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
            var = fromCref(start_cref.clone(), Variable::attributes(getVar(cref.clone(), metamodelica::sourceInfo!())?), Binding::EMPTY_BINDING().clone())?;
            if isRecord(old_var_ptr.clone()) {
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::RECORD { children: metamodelica::nil(), min_var: Prefixes::Variability::PARAMETER.clone(), max_var: Prefixes::Variability::CONTINUOUS.clone() })));
            } else {
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::START { original: old_var_ptr.clone() })));
            }
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarStart(var.backendinfo.clone(), Some(old_var_ptr.clone())));
            (var_ptr, start_cref) = makeVarPtrCyclic(var.clone(), start_cref.clone())?;
            old_var = Pointer::access(old_var_ptr.clone());
            assign_field!(old_var.backendinfo = BackendExtension::BackendInfo::setVarStart(old_var.backendinfo.clone(), Some(var_ptr.clone())));
            Pointer::update(old_var_ptr.clone(), old_var.clone());
            (start_cref.clone(), var_ptr.clone())
        },
    });
            start_cref = ComponentRef::copySubscripts(cref.clone(), start_cref.clone())?;
            (start_cref.clone(), var_ptr.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeStartVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start_cref, var_ptr))
}

pub fn makeResidualVar(mut name: ArcStr, mut uniqueIndex: i32, mut ty: Arc<Type::NFType>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(RESIDUAL_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex.clone())); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY) });
    var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::RESIDUAL_VAR)));
    (var_ptr, cref) = makeVarPtrCyclic(var.clone(), cref.clone())?;
    Ok((var_ptr, cref))
}

pub fn makeEventVar(mut name: ArcStr, mut uniqueIndex: i32, mut var_ty: Arc<Type::NFType>, mut iterator: Arc<Iterator::Iterator>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut var_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut iter_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut iter_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    (iter_crefs, _, _) = BEquation::Iterator::getFrames(iterator.clone())?;
    iter_subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut iter in (iter_crefs.clone()).into_iter().cloned() {
            let __x = Subscript::fromTypedExp(Expression::fromCref(iter.clone(), false)?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if iter_subs.clone().is_empty() {
        ty = var_ty.clone();
    } else {
        ty = Type::liftArrayLeftList(var_ty.clone(), BEquation::Iterator::dimensions(iterator.clone()));
    }
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex.clone())); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node.clone(), subscripts: iter_subs.clone(), ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY) });
    var_cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY) });
    var = fromCref(var_cref.clone(), Attributes::IMPL_DISCRETE_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    assign_field!(
        var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::DISCRETE)),
        var.backendinfo = BackendExtension::BackendInfo::setHideResult(var.backendinfo.clone(), true)
    );
    (var_ptr, cref) = makeVarPtrCyclic(var.clone(), cref.clone())?;
    Ok((var_ptr, cref))
}

pub fn makeAuxVar(mut name: ArcStr, mut uniqueIndex: i32, mut ty: Arc<Type::NFType>, mut makeParam: bool) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    fn updateBackendInfo(mut var: Arc<Variable::NFVariable>, mut makeParam: bool) -> Arc<Variable::NFVariable> {
        let mut var: Arc<Variable::NFVariable> = var;
        assign_field!(
            var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), BackendExtension::VariableKind::fromType(Variable::typeOf(var.clone()), makeParam.clone())),
            var.backendinfo = BackendExtension::BackendInfo::setHideResult(var.backendinfo.clone(), true)
        );
        var
    }

    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex.clone())); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY) });
    var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    var = updateBackendInfo(var.clone(), makeParam.clone());
    assign_field!(var.children = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut child in (var.children.clone()).into_iter().cloned() {
            let __x = updateBackendInfo(child.clone(), makeParam.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    (var_ptr, cref) = makeVarPtrCyclic(var.clone(), cref.clone())?;
    Ok((var_ptr, cref))
}

pub fn makeAuxStateVar(mut uniqueIndex: i32, mut binding: Option<Arc<Expression::NFExpression>>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut der_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut der_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut bnd: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(AUXILIARY_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex.clone())); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node.clone(), subscripts: metamodelica::nil(), ty: Arc::new(openmodelica_nf_frontend::NFType::REAL), origin: ComponentRef::Origin::CREF.clone(), restCref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY) });
    if isSome(binding.clone()) {
        bnd = Util::getOption(binding.clone())?;
        var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::makeFlat(bnd.clone(), Expression::variability(bnd.clone())?, Binding::Source::BINDING.clone(), Binding::NO_CONFIDENCE.clone()))?;
    } else {
        var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    }
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setStateSelect(var.backendinfo.clone(), BackendExtension::StateSelect::AVOID.clone(), false));
    (var_ptr, cref) = makeVarPtrCyclic(var.clone(), cref.clone())?;
    (der_cref, der_var) = makeDerVar(cref.clone(), false)?;
    setStateDerivativeVar(var_ptr.clone(), der_var.clone());
    Ok((var_ptr, cref, der_var, der_cref))
}

pub fn makeTmpVar(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut tmp_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut qual = (*qual).clone();
            old_var_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(TEMPORARY_STR));
            tmp_cref = ComponentRef::append(cref.clone(), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
            var = fromCref(tmp_cref.clone(), Variable::attributes(getVar(cref.clone(), metamodelica::sourceInfo!())?), Binding::EMPTY_BINDING().clone())?;
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), getVarKind(old_var_ptr.clone())));
            (var_ptr, tmp_cref) = makeVarPtrCyclic(var.clone(), tmp_cref.clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeTmpVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tmp_cref)
}

pub fn makeClockVar(mut uniqueIndex: i32, mut ty: Arc<Type::NFType>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(CLOCK_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex.clone())); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY) });
    var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::CLOCK)));
    (var_ptr, cref) = makeVarPtrCyclic(var.clone(), cref.clone())?;
    Ok((var_ptr, cref))
}

pub fn getBindingVariability(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<Prefixes::Variability> {
    let mut variability: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    variability = (::match_deref::match_deref! { match &(Pointer::access(var_ptr.clone())) {
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::TYPED_BINDING { variability: tmp, .. }, .. } => {
            tmp.clone()
        },
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::FLAT_BINDING { variability: tmp, .. }, .. } => {
            tmp.clone()
        },
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::UNBOUND, .. } => {
            Prefixes::Variability::CONTINUOUS.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getBindingVariability")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(variability)
}

pub fn hasEvaluableBinding(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    fn isEvaluable(mut exp: Arc<Expression::NFExpression>) -> Result<bool> {
        let mut b: bool = false;
        let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        b = Expression::isLiteralXML(exp.clone());
        if !(b.clone()) {
            (_, new_exp) = BEquation::Iterator::extract(exp.clone(), UnorderedSet::new((std::sync::Arc::new(fnptr!(hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13), UnorderedMap::new((std::sync::Arc::new(Dimension::hashList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<i32> + 'static>), Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(Dimension::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, Arc<Dimension::NFDimension>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| Ok(List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone())) }), 1))?;
            new_exp = SimplifyExp::simplifyDump(new_exp.clone(), true, (literal!("NBVariable.hasEvaluableBinding.isEvaluable")).clone(), (literal!("")).clone())?;
            b = Expression::isLiteralXML(Ceval::tryEvalExp(new_exp.clone(), Ceval::noTarget().clone()));
        }
        Ok(b)
    }

    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut binding: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if isBound(var_ptr.clone()) {
        binding = Binding::getExp(var.binding.clone())?;
        b = isEvaluable(binding.clone())?;
    } else {
        b = false;
    }
    Ok(b)
}

pub fn mapExp(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut opt_start: Option<Arc<Expression::NFExpression>> = None;
    let mut binding: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_binding: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut start: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_start: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut changed: bool = false;
    if isBound(var_ptr.clone()) {
        binding = Binding::getExp(var.binding.clone())?;
        new_binding = mapFunc(binding.clone(), funcExp.clone())?;
        if !(referenceEq(&binding.clone(),&new_binding.clone())) {
            assign_field!(var.binding = Binding::setExp(new_binding.clone(), var.binding.clone())?);
            changed = true;
        }
    }
    opt_start = getStartAttribute(var_ptr.clone());
    if Util::isSome(opt_start.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(opt_start.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        start = __pa0.clone();
        new_start = mapFunc(start.clone(), funcExp.clone())?;
        if !(referenceEq(&start.clone(),&new_start.clone())) {
            var = setStartAttribute(var.clone(), new_start.clone(), true)?;
            changed = true;
        }
    }
    if changed.clone() {
        Pointer::update(var_ptr.clone(), var.clone());
    }
    Ok(())
}

pub fn setFixed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut b: bool, mut overwrite: bool) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    var = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: binfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { .. }, .. } => {
            let mut binfo = (*binfo).clone();
            assign_field!(binfo.attributes = BackendExtension::VariableAttributes::setFixed(binfo.attributes.clone(), var.ty.clone(), b.clone(), overwrite.clone())?);
            assign_field!(var.backendinfo = binfo.clone());
            var.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setFixed")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr.clone(), var.clone());
    Ok(var_ptr)
}

pub fn setBindingAsStart(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut overwrite: bool) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    var = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: binfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { .. }, .. } => {
            let mut start: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut binfo = (*binfo).clone();
            start = Binding::getExp(var.binding.clone())?;
            assign_field!(binfo.attributes = BackendExtension::VariableAttributes::setStartAttribute(binfo.attributes.clone(), start.clone(), overwrite.clone()));
            assign_field!(var.backendinfo = binfo.clone());
            var.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setBindingAsStart")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr.clone(), var.clone());
    Ok(())
}

pub fn setBindingAsStartAndFix(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut b: bool, mut overwrite: bool) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    setBindingAsStart(var_ptr.clone(), overwrite.clone())?;
    var_ptr = setFixed(var_ptr.clone(), b.clone(), false)?;
    Ok(var_ptr)
}

pub fn getStartAttribute(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Option<Arc<Expression::NFExpression>> {
    let mut start: Option<Arc<Expression::NFExpression>> = BackendExtension::VariableAttributes::getStartAttribute(Variable::getVariableAttributes(Pointer::access(var_ptr.clone())));
    start
}

pub fn hasNonTrivialAliasBinding(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut binding: Arc<Expression::NFExpression> = Binding::getExp(var.binding.clone())?;
    b = !(Expression::isTrivialCref(binding.clone())) && checkExpMap(binding.clone(), (std::sync::Arc::new(fnptr!(isTimeDependent, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!())?;
    Ok(b)
}

pub fn hasConstOrParamAliasBinding(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(checkExpMap(Binding::getExp(var.binding.clone())?, (std::sync::Arc::new(fnptr!(isTimeDependent, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!())?);
    Ok(b)
}

pub fn isTimeDependent(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = BackendExtension::VariableKind::isTimeDependent(var.backendinfo.varKind.clone());
    b
}

pub fn isBound(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool = false;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.binding.clone()) {
        Deref @ Binding::TYPED_BINDING { .. } => true,
        Deref @ Binding::UNTYPED_BINDING { .. } => true,
        Deref @ Binding::FLAT_BINDING { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// ==========================================================================
//                        Other type wrappers
//
// ==========================================================================
pub fn checkExp(mut exp: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut info: SourceInfo) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref, .. } => {
            func(getVarPointer(cref.clone(), info.clone())?)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn checkExpMap(mut exp: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut info: SourceInfo) -> Result<bool> {
    pub fn checkExpTraverse(mut exp: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut info: SourceInfo, mut b: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut b: bool = b;
        if !(b.clone()) {
            b = checkExp(exp.clone(), func.clone(), info.clone())?;
        }
        Ok((exp, b))
    }

    let mut b: bool = false;
    (_, b) = Expression::mapFold(exp.clone(), Arc::new({ let __pe_b1 = func.clone(); let __pe_b2 = info.clone(); move |__pe_a0, __pe_a3| checkExpTraverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), false)?;
    Ok(b)
}

pub fn checkCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut info: SourceInfo) -> bool {
    let mut b: bool = func(getVarPointer(cref.clone(), info.clone()).unwrap()).unwrap();
    b
}

// ==========================================================================
//                        Variable Array Stuff
//    All variable arrays are pointer arrays to avoid duplicates
// ==========================================================================
pub mod VariablePointers {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct VariablePointers {
        /// Map for cref->index
        pub map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>,
        /// Array of variable pointers
        pub varArr: Arc<ExpandableArray::ExpandableArray<Pointer::Pointer<Arc<Variable::NFVariable>>>>,
        /// true if the variables are scalarized
        pub scalarized: bool,
    }

    impl Default for VariablePointers {
        fn default() -> Self {
            Self {
                map: Default::default(),
                varArr: Default::default(),
                scalarized: Default::default(),
            }
        }
    }

    pub type VARIABLE_POINTERS = VariablePointers;

    pub fn toString(mut variables: Arc<VariablePointers>, mut r#str: ArcStr, mut mapping_opt: Option<metamodelica::Array<(i32, i32)>>, mut printEmpty: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut numberOfElements: i32 = size(variables.clone());
        let mut length: i32 = 0;
        let mut scal_start: i32 = 0;
        let mut index: ArcStr = arcstr::literal!("");
        let mut useMapping: bool = Util::isSome(mapping_opt.clone());
        let mut mapping: metamodelica::Array<(i32, i32)>;
        if useMapping.clone() {
            length = 15;
            mapping = Util::getOption(mapping_opt.clone())?;
        } else {
            length = 10;
        }
        if printEmpty.clone() || numberOfElements.clone() > 0 {
            r#str = (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" Variables (")); __mm_s.push_str(&*intString(numberOfElements.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(scalarSize(variables.clone(), true))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            for mut i in 1..=numberOfElements.clone() {
                if useMapping.clone() {
                    (scal_start, _) = mapping.borrow()[(i.clone()-1) as usize].clone();
                    index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*intString(scal_start.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                } else {
                    index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                }
                index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*index.clone()); __mm_s.push_str(&*StringUtil::repeat((literal!(" ")).clone(), length.clone() - ((index.clone()).clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*super::toString(Pointer::access(ExpandableArray::get(i.clone(), variables.varArr.clone())?), (index.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = (literal!("")).clone();
        }
        Ok(r#str)
    }

    pub fn map(mut variables: Arc<VariablePointers>, mut func: Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> + 'static>) -> Result<Arc<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> + 'static>;

        let mut variables: Arc<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        let mut new_var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i.clone(), variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i.clone(), variables.varArr.clone())?;
                var = Pointer::access(var_ptr.clone());
                new_var = func(var.clone())?;
                if !(referenceEq(&var.clone(),&new_var.clone())) {
                    Pointer::update(var_ptr.clone(), new_var.clone());
                }
            }
        }
        Ok(variables)
    }

    pub fn mapPtr(mut variables: Arc<VariablePointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>) -> Result<Arc<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>;

        let mut variables: Arc<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i.clone(), variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i.clone(), variables.varArr.clone())?;
                func(var_ptr.clone())?;
            }
        }
        Ok(variables)
    }

    pub fn mapRemovePtr(mut variables: Arc<VariablePointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>) -> Result<Arc<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>;

        let mut variables: Arc<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i.clone(), variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i.clone(), variables.varArr.clone())?;
                if func(var_ptr.clone())? {
                    variables = remove(var_ptr.clone(), variables.clone())?;
                }
            }
        }
        variables = compress(variables.clone())?;
        Ok(variables)
    }

    pub fn empty(mut size: i32, mut scalarized: bool) -> Arc<VariablePointers> {
        let mut variables: Arc<VariablePointers> = Arc::new(<VariablePointers as ::std::default::Default>::default());
        let mut arr_size: i32 = 0;
        let mut bucketSize: i32 = 0;
        let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        arr_size = std::cmp::max(size.clone(), BaseHashTable::lowBucketSize.clone());
        bucketSize = Util::nextPrime(arr_size.clone());
        if scalarized.clone() {
            map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), bucketSize.clone());
        } else {
            map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hashStrip, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqualStrip) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), bucketSize.clone());
        }
        variables = Arc::new(VariablePointers { map: map.clone(), varArr: ExpandableArray::new(arr_size.clone(), Pointer::create(DUMMY_VARIABLE().clone())), scalarized: scalarized.clone() });
        variables
    }

    pub fn clone(mut variables: Arc<VariablePointers>, mut shallow: bool) -> Result<Arc<VariablePointers>> {
        let mut new: Arc<VariablePointers> = Arc::new(<VariablePointers as ::std::default::Default>::default());
        if shallow.clone() {
            new = fromList(toList(variables.clone())?, false);
        } else {
            new = fromList(({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut eqn in (toList(variables.clone())?).into_iter().cloned() {
            let __x = Pointer::create(Pointer::access(eqn.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), false);
        }
        Ok(new)
    }

    pub fn size(mut variables: Arc<VariablePointers>) -> i32 {
        let mut sz: i32 = ExpandableArray::getNumberOfElements(variables.varArr.clone());
        sz
    }

    pub fn scalarSize(mut variables: Arc<VariablePointers>, mut resize: bool) -> i32 {
        let mut sz: i32 = 0;
        for mut var_ptr in &*toList(variables.clone()).unwrap() {
            let mut var_ptr = var_ptr.clone();
            sz = sz.clone() + super::size(var_ptr.clone(), resize.clone());
        }
        sz
    }

    pub fn toList(mut variables: Arc<VariablePointers>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
        let mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        var_lst = ExpandableArray::toList(variables.varArr.clone())?;
        Ok(var_lst)
    }

    pub fn fromList(mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut scalarized: bool) -> Arc<VariablePointers> {
        let mut variables: Arc<VariablePointers> = Arc::new(<VariablePointers as ::std::default::Default>::default());
        variables = empty((var_lst.clone().len() as i32), scalarized.clone());
        variables = addList(var_lst.clone(), variables.clone());
        variables
    }

    pub fn addList(mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut variables: Arc<VariablePointers>) -> Arc<VariablePointers> {
        let mut variables: Arc<VariablePointers> = variables;
        variables = List::fold(var_lst.clone(), Arc::new(move |__pe_a0, __pe_a1| add(__pe_a0, __pe_a1)), variables.clone());
        variables
    }

    pub fn removeList(mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        variables = List::fold(var_lst.clone(), Arc::new(move |__pe_a0, __pe_a1| remove(__pe_a0, __pe_a1)), variables.clone());
        variables = compress(variables.clone())?;
        Ok(variables)
    }

    pub fn removeCheck(mut variables: Arc<VariablePointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (toList(variables.clone())?).into_iter().cloned() {
            if !(!(func(var.clone())?)) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        variables = fromList(vars.clone(), false);
        Ok(variables)
    }

    pub fn add(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        let mut index: i32 = 0;
        var = Pointer::access(varPointer.clone());
        let () = (match UnorderedMap::get(var.name.clone(), variables.map.clone()) {
        Some(mut index) if (index.clone() > 0) => {
            ExpandableArray::update(index.clone(), varPointer.clone(), variables.varArr.clone())?;
            ()
        },
        _ => {
            (_, index) = ExpandableArray::add(varPointer.clone(), variables.varArr.clone())?;
            UnorderedMap::add(var.name.clone(), index.clone(), variables.map.clone())?;
            ()
        },
    });
        Ok(variables)
    }

    pub fn remove(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        let mut index: i32 = 0;
        var = Pointer::access(var_ptr.clone());
        let () = (match UnorderedMap::get(var.name.clone(), variables.map.clone()) {
        Some(mut index) if (index.clone() > 0) => {
            ExpandableArray::delete(index.clone(), variables.varArr.clone())?;
            UnorderedMap::add(var.name.clone(), -1, variables.map.clone())?;
            ()
        },
        _ => (),
    });
        Ok(variables)
    }

    pub fn setVarAt(mut variables: Arc<VariablePointers>, mut idx: i32, mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> {
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        ExpandableArray::set(idx.clone(), var_ptr.clone(), variables.varArr.clone())?;
        var = Pointer::access(var_ptr.clone());
        UnorderedMap::add(var.name.clone(), idx.clone(), variables.map.clone())?;
        Ok(())
    }

    pub fn getVarAt(mut variables: Arc<VariablePointers>, mut idx: i32) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
        let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        var = ExpandableArray::get(idx.clone(), variables.varArr.clone())?;
        Ok(var)
    }

    pub fn getVarSafe(mut variables: Arc<VariablePointers>, mut cref: Arc<ComponentRef::NFComponentRef>, mut info: Option<SourceInfo>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut index: i32 = 0;
        var_ptr = (match UnorderedMap::get(cref.clone(), variables.map.clone()) {
        Some(mut index) if (index.clone() > 0) => ExpandableArray::get(index.clone(), variables.varArr.clone())?,
        _ => {
            if Util::isSome(info.clone()) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VariablePointers.getVarSafe")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone(), Util::getOption(info.clone())?)?;
            }
            bail!("fail")
        },
    });
        Ok(var_ptr)
    }

    pub fn getVarIndex(mut variables: Arc<VariablePointers>, mut cref: Arc<ComponentRef::NFComponentRef>) -> i32 {
        let mut index: i32 = UnorderedMap::getOrDefault(cref.clone(), variables.map.clone(), -1);
        index
    }

    pub fn contains(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut variables: Arc<VariablePointers>) -> bool {
        let mut b: bool = containsCref(getVarName(var.clone()), variables.clone());
        b
    }

    pub fn containsCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut variables: Arc<VariablePointers>) -> bool {
        let mut b: bool = getVarIndex(variables.clone(), cref.clone()) > 0;
        b
    }

    pub fn getVarNames(mut variables: Arc<VariablePointers>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut acc: Pointer::Pointer<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> = Pointer::create(metamodelica::nil());
        mapPtr(variables.clone(), Arc::new({ let __pe_b1 = acc.clone(); move |__pe_a0| Ok(getVarNameTraverse(__pe_a0, __pe_b1.clone())) }))?;
        names = Pointer::access(acc.clone()).reverse();
        Ok(names)
    }

    pub fn getScalarVarNames(mut variables: Arc<VariablePointers>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        for mut var_ptr in &*toList(variables.clone())? {
            let mut var_ptr = var_ptr.clone();
            var = Pointer::access(var_ptr.clone());
            if Type::isArray(var.ty.clone()) {
                for mut cr in &*ComponentRef::scalarizeAll(ComponentRef::stripSubscriptsAll(var.name.clone()), resize.clone())? {
                    let mut cr = cr.clone();
                    if Type::isComplex(ComponentRef::nodeType(cr.clone())?) {
                        names = listAppend(ComponentRef::getRecordChildren(cr.clone())?, names.clone());
                    } else {
                        names = cons(cr.clone(), names.clone());
                    }
                }
            } else {
                names = cons(var.name.clone(), names.clone());
            }
        }
        Ok(names)
    }

    pub fn getMarkedVars(mut variables: Arc<VariablePointers>, mut marks: metamodelica::Array<bool>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
        let mut marked_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut indices: Arc<metamodelica::List<i32>> = BackendUtil::findTrueIndices(marks.clone());
        if (marks.clone().borrow().len() as i32) == size(variables.clone()) {
            marked_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut index in (indices.clone()).into_iter().cloned() {
            let __x = getVarAt(variables.clone(), index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VariablePointers.getMarkedVars")); __mm_s.push_str(&*literal!(" failed because the number var marks (")); __mm_s.push_str(&*intString((marks.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(") is not equal to the number of variables (")); __mm_s.push_str(&*intString(size(variables.clone()))); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(marked_vars)
    }

    pub fn compress(mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut i in (1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone())).rev() {
            if ExpandableArray::occupied(i.clone(), variables.varArr.clone()) {
                vars = cons(ExpandableArray::get(i.clone(), variables.varArr.clone())?, vars.clone());
            }
        }
        variables = fromList(vars.clone(), false);
        Ok(variables)
    }

    pub fn sort(mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut size: i32 = 0;
        let mut hash_lst: Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Variable::NFVariable>>)>> = metamodelica::nil();
        let mut hash_lst_ptr: Pointer::Pointer<Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Variable::NFVariable>>)>>> = Pointer::create(metamodelica::nil());
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        size = ExpandableArray::getNumberOfElements(variables.varArr.clone());
        mapPtr(variables.clone(), Arc::new({ let __pe_b1 = ((metamodelica::OrderedFloat((size.clone()) as f64) * (size.clone()).ln()).0 as i32); let __pe_b2 = hash_lst_ptr.clone(); move |__pe_a0| createSortHashTpl(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
        hash_lst = List::sort(Pointer::access(hash_lst_ptr.clone()), std::sync::Arc::new(fnptr!(BackendUtil::indexTplGt, _, _)))?;
        variables = empty(size.clone(), variables.scalarized.clone());
        for mut tpl in &*hash_lst.clone() {
            let mut tpl = tpl.clone();
            (_, var_ptr) = tpl.clone();
            variables = add(var_ptr.clone(), variables.clone())?;
        }
        Ok(variables)
    }

    pub fn scalarize(mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut flattened: bool = false;
        (vars, flattened) = scalarizeList(toList(variables.clone())?)?;
        if flattened.clone() {
            variables = fromList(vars.clone(), true);
        }
        Ok(variables)
    }

    pub fn scalarizeList(mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, bool)> {
        let mut new_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut flattened: bool = false;
        let mut scalar_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        let mut element_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        for mut var_ptr in &*vars.clone() {
            let mut var_ptr = var_ptr.clone();
            var = Pointer::access(var_ptr.clone());
            if Type::isArray(var.ty.clone()) {
                flattened = true;
                scalar_vars = Scalarize::scalarizeBackendVariable(var.clone(), metamodelica::nil())?;
            } else {
                scalar_vars = list![Pointer::access(var_ptr.clone())];
            }
            for mut var in &*scalar_vars.clone() {
                let mut var = var.clone();
                if Type::isComplex(var.ty.clone()) {
                    flattened = true;
                    element_vars = Scalarize::scalarizeComplexVariable(var.clone(), metamodelica::nil())?;
                    for mut elem_var in &*element_vars.clone().reverse() {
                        let mut elem_var = elem_var.clone();
                        new_vars = cons(Pointer::create(elem_var.clone()), new_vars.clone());
                    }
                } else {
                    new_vars = cons(Pointer::create(var.clone()), new_vars.clone());
                }
            }
        }
        new_vars = new_vars.clone().reverse();
        Ok((new_vars, flattened))
    }

    pub fn varSlice(mut vars: Arc<VariablePointers>, mut scal: i32, mut arr: i32, mut mapping: Arc<Mapping::Mapping>, mut resize: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut start: i32 = 0;
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut vals: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        (start, _) = mapping.var_AtS.borrow()[(arr.clone()-1) as usize].clone();
        var = getVarAt(vars.clone(), arr.clone())?;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Pointer::access(var.clone())) {
            Deref @ Variable::VARIABLE { ty: __pa0, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa0.clone();
        cref = __pa1.clone();
        dims = Type::arrayDims(ty.clone());
        sizes = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dim in (dims.clone()).into_iter().cloned() {
            let __x = Dimension::size(dim.clone(), resize.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        vals = Slice::indexToLocation(scal.clone() - start.clone(), sizes.clone()).reverse();
        subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for (dim, val) in (&(dims.clone())).into_iter().zip((&(vals.clone())).into_iter()) {
            let __x = Subscript::nth(dim.clone(), val.clone() + 1)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        cref = ComponentRef::mergeSubscripts(subs.clone(), cref.clone(), true, true, false)?;
        Ok(cref)
    }

    fn createSortHashTpl(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut r#mod: i32, mut hash_lst_ptr: Pointer::Pointer<Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Variable::NFVariable>>)>>>) -> Result<()> {
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        let mut hash: i32 = 0;
        var = Pointer::access(var_ptr.clone());
        hash = stringHashDjb2Mod((BackendExtension::BackendInfo::toString(var.backendinfo.clone())?).clone(), r#mod.clone());
        Pointer::update(hash_lst_ptr.clone(), cons((hash.clone(), var_ptr.clone()), Pointer::access(hash_lst_ptr.clone())));
        Ok(())
    }

}

// ==========================================================================
//                        Variable Data
//    All variable arrays are pointer arrays to avoid duplicates
// ==========================================================================
pub mod VarData {
    use super::*;
    /// All variable arrays are pointer subsets of an array of variables indicated
    ///    by preceding comment. Used to traverse all variables of a special kind.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum VarData {
        /// Only to be used for simulation systems.
        VAR_DATA_SIM {
            /// use when trying to create unique variables
            uniqueIndex: Pointer::Pointer<i32>,
            /// All variables
            variables: Arc<VariablePointers::VariablePointers>,
            /// All state derivatives, algebraic variables,
            ///                                          discrete variables
            unknowns: Arc<VariablePointers::VariablePointers>,
            /// Parameters, constants, states
            knowns: Arc<VariablePointers::VariablePointers>,
            /// All initial unknowns (unknowns + states + previous + parameters(non const binding))
            initials: Arc<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: Arc<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal with 1 or -1 coefficient
            aliasVars: Arc<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal with gain * alias + offset function
            nonTrivialAlias: Arc<VariablePointers::VariablePointers>,
            /// State derivatives (der(x) -> $DER.x)
            derivatives: Arc<VariablePointers::VariablePointers>,
            /// Algebraic variables
            algebraics: Arc<VariablePointers::VariablePointers>,
            /// Discrete variables
            discretes: Arc<VariablePointers::VariablePointers>,
            /// Discrete state variables
            discrete_states: Arc<VariablePointers::VariablePointers>,
            /// Clocked state variables
            clocked_states: Arc<VariablePointers::VariablePointers>,
            /// Previous variables (pre(d) -> $PRE.d)
            previous: Arc<VariablePointers::VariablePointers>,
            /// clock variables
            clocks: Arc<VariablePointers::VariablePointers>,
            /// States
            states: Arc<VariablePointers::VariablePointers>,
            /// Top level inputs
            top_level_inputs: Arc<VariablePointers::VariablePointers>,
            /// Resizable Parameters
            resizables: Arc<VariablePointers::VariablePointers>,
            /// Parameters
            parameters: Arc<VariablePointers::VariablePointers>,
            /// Constants
            constants: Arc<VariablePointers::VariablePointers>,
            /// Records
            records: Arc<VariablePointers::VariablePointers>,
            /// External Objects
            external_objects: Arc<VariablePointers::VariablePointers>,
            /// artificial variables to have pointers on crefs
            artificials: Arc<VariablePointers::VariablePointers>,
            state_order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>,
        },
        /// Only to be used for Jacobians.
        VAR_DATA_JAC {
            /// All jacobian variables
            variables: Arc<VariablePointers::VariablePointers>,
            /// All result and temporary vars
            unknowns: Arc<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: Arc<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal
            aliasVars: Arc<VariablePointers::VariablePointers>,
            /// Differentiation variables z where J = dF/dz
            diffVars: Arc<VariablePointers::VariablePointers>,
            /// All occurring unknowns for linearity analysis
            dependencies: Arc<VariablePointers::VariablePointers>,
            /// Result variable depending on current seed
            ///                                          ($RES.[jacname].[eq_idx])
            resultVars: Arc<VariablePointers::VariablePointers>,
            /// Temporary variables (inner partial derivatives)
            ///                                          dy/dz with y!=z for all y and z
            ///                                          ($TMP.[jacname].y)
            tmpVars: Arc<VariablePointers::VariablePointers>,
            /// Seed variables representing a generic derivative
            ///                                          dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED.[jacname].x)
            seedVars: Arc<VariablePointers::VariablePointers>,
        },
        /// Only to be used for Hessians.
        VAR_DATA_HES {
            /// All hessian variables
            variables: Arc<VariablePointers::VariablePointers>,
            /// All state derivatives, algebraic variables,
            ///                                          discrete variables
            unknowns: Arc<VariablePointers::VariablePointers>,
            /// Parameters, constants
            knowns: Arc<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: Arc<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal
            aliasVars: Arc<VariablePointers::VariablePointers>,
            /// Differentiation variables z where J = dF/dz
            diffVars: Arc<VariablePointers::VariablePointers>,
            /// All occurring unknowns for linearity analysis
            dependencies: Arc<VariablePointers::VariablePointers>,
            /// Result variable depending on current seed
            ///                                          ($RES.[jacname].[eq_idx])
            resultVars: Arc<VariablePointers::VariablePointers>,
            /// Temporary variables (inner partial derivatives)
            ///                                          dy/dz with y!=z for all y and z
            ///                                          ($TMP.[jacname].y)
            tmpVars: Arc<VariablePointers::VariablePointers>,
            /// Seed variables representing a generic derivative
            ///                                          dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED.[jacname].x)
            seedVars: Arc<VariablePointers::VariablePointers>,
            /// Second seed variables representing a generic
            ///                                          derivative dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED2.[jacname].x)
            seedVars2: Arc<VariablePointers::VariablePointers>,
            /// Lambda variables for optimization
            lambdaVars: Option<Arc<VariablePointers::VariablePointers>>,
        },
        VAR_DATA_EMPTY,
    }
    impl Default for VarData {
        fn default() -> Self { Self::VAR_DATA_EMPTY }
    }
    pub use self::VarData::{VAR_DATA_SIM,VAR_DATA_JAC,VAR_DATA_HES,VAR_DATA_EMPTY};
    pub fn size(mut varData: Arc<VarData>) -> Result<i32> {
        let mut s: i32 = 0;
        s = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => VariablePointers::size(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone()),
        Deref @ VAR_DATA_JAC { .. } => VariablePointers::size(var_field!((*varData).unknowns, VarData::VAR_DATA_JAC).clone()),
        Deref @ VAR_DATA_HES { .. } => VariablePointers::size(var_field!((*varData).unknowns, VarData::VAR_DATA_HES).clone()),
        _ => bail!("match: no arm matched"),
    } });
        Ok(s)
    }

    pub fn scalarSize(mut varData: Arc<VarData>, mut resize: bool) -> Result<i32> {
        let mut s: i32 = 0;
        s = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => VariablePointers::scalarSize(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), resize.clone()),
        Deref @ VAR_DATA_JAC { .. } => VariablePointers::scalarSize(var_field!((*varData).unknowns, VarData::VAR_DATA_JAC).clone(), resize.clone()),
        Deref @ VAR_DATA_HES { .. } => VariablePointers::scalarSize(var_field!((*varData).unknowns, VarData::VAR_DATA_HES).clone(), resize.clone()),
        _ => bail!("match: no arm matched"),
    } });
        Ok(s)
    }

    pub fn toString(mut varData: Arc<VarData>, mut level: i32) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (if (level.clone() == 0) {(::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => VariablePointers::toString(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), (literal!("Simulation")).clone(), None, true)?,
        Deref @ VAR_DATA_JAC { .. } => VariablePointers::toString(var_field!((*varData).variables, VarData::VAR_DATA_JAC).clone(), (literal!("Jacobian")).clone(), None, true)?,
        Deref @ VAR_DATA_HES { .. } => VariablePointers::toString(var_field!((*varData).variables, VarData::VAR_DATA_HES).clone(), (literal!("Hessian")).clone(), None, true)?,
        Deref @ VAR_DATA_EMPTY { .. } => literal!("Empty variable Data!\n"),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })} else if (level.clone() == 1) {toStringVerbose(varData.clone(), false)?} else {toStringVerbose(varData.clone(), true)?}).clone();
        Ok(r#str)
    }

    pub fn toStringVerbose(mut varData: Arc<VarData>, mut full: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (({
        let mut tmp: ArcStr = literal!("");
        (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => {
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable Data Simulation (scalar unknowns: ")); __mm_s.push_str(&*intString(VariablePointers::scalarSize(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), true))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((tmp.clone()).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            if !(full.clone()) {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), (literal!("Unknown")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).states, VarData::VAR_DATA_SIM).clone(), (literal!("Local Known")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), (literal!("Global Known")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            } else {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).states, VarData::VAR_DATA_SIM).clone(), (literal!("State")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone(), (literal!("Derivative")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone(), (literal!("Algebraic")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).discretes, VarData::VAR_DATA_SIM).clone(), (literal!("Discrete")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).discrete_states, VarData::VAR_DATA_SIM).clone(), (literal!("Discrete State")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).clocked_states, VarData::VAR_DATA_SIM).clone(), (literal!("Clocked State")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).previous, VarData::VAR_DATA_SIM).clone(), (literal!("Previous")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).clocks, VarData::VAR_DATA_SIM).clone(), (literal!("Clock")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).top_level_inputs, VarData::VAR_DATA_SIM).clone(), (literal!("Top Level Input")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).resizables, VarData::VAR_DATA_SIM).clone(), (literal!("Resizable Parameter")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).parameters, VarData::VAR_DATA_SIM).clone(), (literal!("Parameter")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).constants, VarData::VAR_DATA_SIM).clone(), (literal!("Constant")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).records, VarData::VAR_DATA_SIM).clone(), (literal!("Record")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).external_objects, VarData::VAR_DATA_SIM).clone(), (literal!("External Object")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).artificials, VarData::VAR_DATA_SIM).clone(), (literal!("Artificial")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            }
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).auxiliaries, VarData::VAR_DATA_SIM).clone(), (literal!("Auxiliary")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).aliasVars, VarData::VAR_DATA_SIM).clone(), (literal!("Alias")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            tmp.clone()
        },
        Deref @ VAR_DATA_JAC { .. } => {
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).unknowns, VarData::VAR_DATA_JAC).clone(), (literal!("Partial Derivative")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).seedVars, VarData::VAR_DATA_JAC).clone(), (literal!("Seed")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            if full.clone() {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).diffVars, VarData::VAR_DATA_JAC).clone(), (literal!("Differentiation")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).resultVars, VarData::VAR_DATA_JAC).clone(), (literal!("Residual")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).tmpVars, VarData::VAR_DATA_JAC).clone(), (literal!("Inner")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).dependencies, VarData::VAR_DATA_JAC).clone(), (literal!("Dependencies")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).auxiliaries, VarData::VAR_DATA_JAC).clone(), (literal!("Auxiliary")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).aliasVars, VarData::VAR_DATA_JAC).clone(), (literal!("Alias")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            }
            tmp.clone()
        },
        Deref @ VAR_DATA_HES { .. } => {
            let mut lambdaVars: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Variable Data Hessian")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).unknowns, VarData::VAR_DATA_HES).clone(), (literal!("Unknown")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).knowns, VarData::VAR_DATA_HES).clone(), (literal!("Known")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).auxiliaries, VarData::VAR_DATA_HES).clone(), (literal!("Auxiliary")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).aliasVars, VarData::VAR_DATA_HES).clone(), (literal!("Alias")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            if full.clone() {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).diffVars, VarData::VAR_DATA_HES).clone(), (literal!("Differentiation")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).dependencies, VarData::VAR_DATA_HES).clone(), (literal!("Dependencies")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).resultVars, VarData::VAR_DATA_HES).clone(), (literal!("Result")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).tmpVars, VarData::VAR_DATA_HES).clone(), (literal!("Temporary")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).seedVars, VarData::VAR_DATA_HES).clone(), (literal!("First Seed")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).seedVars2, VarData::VAR_DATA_HES).clone(), (literal!("Second Seed")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
                if isSome(var_field!((*varData).lambdaVars, VarData::VAR_DATA_HES).clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(var_field!((*varData).lambdaVars, VarData::VAR_DATA_HES).clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    lambdaVars = __pa0.clone();
                    tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*VariablePointers::toString(lambdaVars.clone(), (literal!("Lagrangian Lambda")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
                }
            }
            tmp.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    })).clone();
        Ok(r#str)
    }

    pub fn getVariables(mut varData: Arc<VarData>) -> Result<Arc<VariablePointers::VariablePointers>> {
        let mut variables: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
        variables = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(),
        Deref @ VAR_DATA_JAC { .. } => var_field!((*varData).variables, VarData::VAR_DATA_JAC).clone(),
        Deref @ VAR_DATA_HES { .. } => var_field!((*varData).variables, VarData::VAR_DATA_HES).clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(variables)
    }

    pub fn setVariables(mut varData: Arc<VarData>, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<Arc<VarData>> {
        let mut varData: Arc<VarData> = varData;
        varData = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM; variables = variables.clone());
            varData.clone()
        },
        Deref @ VAR_DATA_JAC { .. } => {
            assign_variant_field!(varData => VarData::VAR_DATA_JAC; variables = variables.clone());
            varData.clone()
        },
        Deref @ VAR_DATA_HES { .. } => {
            assign_variant_field!(varData => VarData::VAR_DATA_HES; variables = variables.clone());
            varData.clone()
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(varData)
    }

    pub fn getUniqueIndex(mut varData: Arc<VarData>) -> Result<Pointer::Pointer<i32>> {
        let mut uniqueIndex: Pointer::Pointer<i32>;
        uniqueIndex = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => var_field!((*varData).uniqueIndex, VarData::VAR_DATA_SIM).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.getUniqueIndex")); __mm_s.push_str(&*literal!(" failed because of incorrect record type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(uniqueIndex)
    }

    pub fn getStateOrder(mut varData: Arc<VarData>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>> {
        let mut state_order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;
        state_order = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => var_field!((*varData).state_order, VarData::VAR_DATA_SIM).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.getStateOrder")); __mm_s.push_str(&*literal!(" failed because of incorrect record type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(state_order)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    #[repr(i32)]
    pub enum VarType {
        STATE = 1,
        STATE_DER = 2,
        ALGEBRAIC = 3,
        DISCRETE = 4,
        DISC_STATE = 5,
        PREVIOUS = 6,
        START = 7,
        PARAMETER = 8,
        ITERATOR = 9,
        RECORD = 10,
        CLOCK = 11,
    }
    impl PartialOrd for VarType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl Ord for VarType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
    }

    pub fn addTypedList(mut varData: Arc<VarData>, mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut varType: VarType) -> Result<Arc<VarData>> {
        let mut varData: Arc<VarData> = varData;
        varData = (::match_deref::match_deref! { match &((varData.clone(), varType.clone())) {
        (Deref @ VAR_DATA_SIM { .. }, VarType::STATE { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone()),
                knowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone()),
                states = VariablePointers::addList(var_lst.clone(), var_field!((*varData).states, VarData::VAR_DATA_SIM).clone()),
                initials = VariablePointers::addList(var_lst.clone(), var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone()),
                unknowns = VariablePointers::removeList(var_lst.clone(), var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                algebraics = VariablePointers::removeList(var_lst.clone(), var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::STATE_DER) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone()),
                unknowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone()),
                derivatives = VariablePointers::addList(var_lst.clone(), var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone()),
                initials = VariablePointers::addList(var_lst.clone(), var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::ALGEBRAIC) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone()),
                unknowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone()),
                algebraics = VariablePointers::addList(var_lst.clone(), var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone()),
                initials = VariablePointers::addList(var_lst.clone(), var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone()),
                states = VariablePointers::removeList(var_lst.clone(), var_field!((*varData).states, VarData::VAR_DATA_SIM).clone())?,
                derivatives = VariablePointers::removeList(var_lst.clone(), var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone())?,
                knowns = VariablePointers::removeList(var_lst.clone(), var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::DISCRETE) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone()),
                unknowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone()),
                discretes = VariablePointers::addList(var_lst.clone(), var_field!((*varData).discretes, VarData::VAR_DATA_SIM).clone()),
                initials = VariablePointers::addList(var_lst.clone(), var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::START { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone()),
                initials = VariablePointers::addList(var_lst.clone(), var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::PARAMETER { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone()),
                parameters = VariablePointers::addList(var_lst.clone(), var_field!((*varData).parameters, VarData::VAR_DATA_SIM).clone()),
                knowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::ITERATOR { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone()),
                knowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone()),
                artificials = VariablePointers::addList(var_lst.clone(), var_field!((*varData).artificials, VarData::VAR_DATA_SIM).clone())
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::CLOCK) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM; clocks = VariablePointers::addList(var_lst.clone(), var_field!((*varData).clocks, VarData::VAR_DATA_SIM).clone()));
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::RECORD { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone()),
                records = VariablePointers::addList(var_lst.clone(), var_field!((*varData).records, VarData::VAR_DATA_SIM).clone()),
                knowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone()),
                records = VariablePointers::mapPtr(var_field!((*varData).records, VarData::VAR_DATA_SIM).clone(), Arc::new({ let __pe_b1 = var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(); move |__pe_a0| BackendDAE::lowerRecordChildren(__pe_a0, __pe_b1.clone()) }))?
            );
            varData.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.addTypedList")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(varData)
    }

    pub fn removeTypedCheck(mut varData: Arc<VarData>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut varType: VarType) -> Result<Arc<VarData>> {
        let mut varData: Arc<VarData> = varData;
        varData = (::match_deref::match_deref! { match &((varData.clone(), varType.clone())) {
        (Deref @ VAR_DATA_SIM { .. }, VarType::STATE { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                states = VariablePointers::removeCheck(var_field!((*varData).states, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::STATE_DER) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                derivatives = VariablePointers::removeCheck(var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::ALGEBRAIC) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                algebraics = VariablePointers::removeCheck(var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::DISCRETE) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                discretes = VariablePointers::removeCheck(var_field!((*varData).discretes, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::START { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::PARAMETER { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                parameters = VariablePointers::removeCheck(var_field!((*varData).parameters, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::ITERATOR { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                artificials = VariablePointers::removeCheck(var_field!((*varData).artificials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::CLOCK) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM; clocks = VariablePointers::removeCheck(var_field!((*varData).clocks, VarData::VAR_DATA_SIM).clone(), func.clone())?);
            varData.clone()
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::RECORD { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                records = VariablePointers::removeCheck(var_field!((*varData).records, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.removeTypedCheck")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(varData)
    }

}

// ==========================================================================
//                      Protected utility functions
// ==========================================================================
fn getVarNameTraverse(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut acc: Pointer::Pointer<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>) -> () {
    Pointer::update(acc.clone(), cons(getVarName(var.clone()), Pointer::access(acc.clone())));
    ()
}

