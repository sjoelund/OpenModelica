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
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableAttributes;
use crate::NFBackendExtension::VariableKind;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInst as Inst;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::AccessLevel;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NFVariable {
    pub name: Arc<ComponentRef::NFComponentRef>,
    pub ty: Arc<Type::NFType>,
    pub binding: Arc<Binding::NFBinding>,
    pub visibility: Visibility,
    pub attributes: Arc<Attributes::NFAttributes>,
    pub typeAttributes: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>,
    pub children: Arc<metamodelica::List<Arc<NFVariable>>>,
    pub comment: Arc<SCode::Comment>,
    pub info: SourceInfo,
    /// NFBackendExtension.DUMMY_BACKEND_INFO for all of frontend. Only used in Backend.
    pub backendinfo: Arc<BackendInfo::BackendInfo>,
}

impl Default for NFVariable {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ty: Default::default(),
            binding: Default::default(),
            visibility: Default::default(),
            attributes: Default::default(),
            typeAttributes: Default::default(),
            children: Default::default(),
            comment: Default::default(),
            info: Default::default(),
            backendinfo: Default::default(),
        }
    }
}

pub type VARIABLE = NFVariable;

pub fn fromCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFVariable>> {
    let mut variable: Arc<NFVariable> = Arc::new(<NFVariable as ::std::default::Default>::default());
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut class_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut vis: Visibility = Visibility::PUBLIC;
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut binfo: Arc<BackendInfo::BackendInfo> = NFBackendExtension::DUMMY_BACKEND_INFO().clone();
    let mut child_nodes: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut children: Arc<metamodelica::List<Arc<NFVariable>>> = metamodelica::nil();
    node = ComponentRef::node(cref.clone())?;
    comp = InstNode::component(node.clone())?;
    ty = ComponentRef::getSubscriptedType(cref.clone(), false)?;
    vis = InstNode::visibility(node.clone());
    attr = Component::getAttributes(comp.clone());
    cmt = Component::comment(comp.clone())?;
    info = InstNode::info(node.clone())?;
    if ComponentRef::isIterator(cref.clone()) {
        binding = Binding::EMPTY_BINDING().clone();
        assign_field!(binfo.varKind = Arc::new(crate::NFBackendExtension::VariableKind::ITERATOR));
    } else {
        binding = Component::getImplicitBinding(comp.clone(), InstNode::instanceParent(node.clone()));
    }
    if !(Type::isExternalObject(ty.clone())) {
        children = (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::COMPLEX { cls: class_node, .. } => {
            child_nodes = Class::getComponents(InstNode::getClass(class_node.clone())?);
            children = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFVariable>>> = metamodelica::nil();
        for mut c in (child_nodes.clone()).borrow().iter() {
            let __x = fromCref(ComponentRef::prefixCref(c.clone(), InstNode::getType(c.clone())?, metamodelica::nil(), cref.clone()))?;
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
    variable = Arc::new(NFVariable { name: cref.clone(), ty: ty.clone(), binding: binding.clone(), visibility: vis.clone(), attributes: attr.clone(), typeAttributes: metamodelica::nil(), children: children.clone(), comment: cmt.clone(), info: info.clone(), backendinfo: binfo.clone() });
    Ok(variable)
}

pub fn name(mut var: Arc<NFVariable>) -> Arc<ComponentRef::NFComponentRef> {
    let mut name: Arc<ComponentRef::NFComponentRef> = var.name.clone();
    name
}

pub fn size(mut var: Arc<NFVariable>, mut resize: bool) -> i32 {
    let mut s: i32 = Type::sizeOf(var.ty.clone(), resize.clone()).unwrap();
    s
}

pub fn hash(mut var: Arc<NFVariable>) -> i32 {
    let mut i: i32 = ComponentRef::hash(var.name.clone());
    i
}

pub fn equalName(mut var1: Arc<NFVariable>, mut var2: Arc<NFVariable>) -> bool {
    let mut b: bool = ComponentRef::isEqual(var1.name.clone(), var2.name.clone()).unwrap();
    b
}

pub fn expand(mut var: Arc<NFVariable>, mut backend: bool) -> Result<Arc<metamodelica::List<Arc<NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Arc<NFVariable>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut v: Arc<NFVariable> = Arc::new(<NFVariable as ::std::default::Default>::default());
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut bind_var: Variability = Variability::CONSTANT;
    let mut bind_src: Binding::Source = Binding::Source::BINDING;
    let mut bind_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut crefs_len: i32 = 0;
    let mut expl_len: i32 = 0;
    if Type::isArray(var.ty.clone()) {
        exp = Expression::fromCref(var.name.clone(), false)?;
        (exp, _) = ExpandExp::expandCref(exp.clone(), backend.clone(), false)?;
        expl = Expression::arrayScalarElements(exp.clone());
        crefs = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = Expression::toCref(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        v = var.clone();
        assign_field!(v.ty = Type::arrayElementType(v.ty.clone()));
        vars = metamodelica::nil();
        binding = var.binding.clone();
        if Binding::isBound(binding.clone()) {
            bind_exp = Binding::getTypedExp(binding.clone())?;
            expl = Expression::arrayScalarElements((ExpandExp::expand(bind_exp.clone(), false, false)?).0);
            crefs_len = (crefs.clone().len() as i32);
            expl_len = (expl.clone().len() as i32);
            if expl_len.clone() < crefs_len.clone() {
                if intMod(crefs_len.clone(), expl_len.clone()) != 0 {
                    Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFVariable.expand")); __mm_s.push_str(&*literal!(" failed to expand ")); __mm_s.push_str(&*ComponentRef::toString(var.name.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                }
                expl = List::flatten(List::fill(expl.clone(), intDiv(crefs_len.clone(), expl_len.clone())));
            }
            bind_var = Binding::variability(binding.clone())?;
            bind_src = Binding::source(binding.clone());
            for mut cr in &*crefs.clone() {
                let mut cr = cr.clone();
                assign_field!(v.name = cr.clone());
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expl.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
                expl = __pa1.clone();
                assign_field!(v.binding = Binding::makeFlat(exp.clone(), bind_var.clone(), bind_src.clone(), Binding::NO_CONFIDENCE.clone()));
                vars = cons(v.clone(), vars.clone());
            }
        } else {
            for mut cr in &*crefs.clone() {
                let mut cr = cr.clone();
                assign_field!(v.name = cr.clone());
                vars = cons(v.clone(), vars.clone());
            }
        }
        vars = vars.clone().reverse();
    } else {
        vars = list![var.clone()];
    }
    Ok(vars)
}

pub fn expandChildren(mut var: Arc<NFVariable>, mut arrayDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut addDimensions: bool) -> Arc<metamodelica::List<Arc<NFVariable>>> {
    let mut children: Arc<metamodelica::List<Arc<NFVariable>>> = metamodelica::nil();
    let mut newArrayDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    if addDimensions.clone() && !(arrayDims.clone().is_empty()) {
        assign_field!(var.ty = Type::liftArrayLeftList(var.ty.clone(), arrayDims.clone()));
    }
    newArrayDims = Type::arrayDims(var.ty.clone());
    children = cons(var.clone(), List::flatten(({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFVariable>>>>> = metamodelica::nil();
        for mut v in (var.children.clone()).into_iter().cloned() {
            let __x = expandChildren(v.clone(), newArrayDims.clone(), addDimensions.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })));
    children
}

pub fn typeOf(mut var: Arc<NFVariable>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = var.ty.clone();
    ty
}

pub fn attributes(mut variable: Arc<NFVariable>) -> Arc<Attributes::NFAttributes> {
    let mut attributes: Arc<Attributes::NFAttributes> = variable.attributes.clone();
    attributes
}

pub fn variability(mut variable: Arc<NFVariable>) -> Variability {
    let mut variability: Variability = variable.attributes.variability.clone();
    variability
}

pub fn setVariability(mut variable: Arc<NFVariable>, mut variability: Variability) -> Arc<NFVariable> {
    let mut variable: Arc<NFVariable> = variable;
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    attr = variable.attributes.clone();
    assign_field!(attr.variability = variability.clone());
    assign_field!(variable.attributes = attr.clone());
    variable
}

pub fn visibility(mut variable: Arc<NFVariable>) -> Visibility {
    let mut visibility: Visibility = variable.visibility.clone();
    visibility
}

pub fn isComplex(mut var: Arc<NFVariable>) -> bool {
    let mut b: bool = Type::isComplex(var.ty.clone());
    b
}

pub fn isComplexArray(mut var: Arc<NFVariable>) -> bool {
    let mut b: bool = Type::isComplexArray(var.ty.clone());
    b
}

pub fn isStructural(mut variable: Arc<NFVariable>) -> bool {
    let mut structural: bool = variable.attributes.variability.clone() <= Variability::STRUCTURAL_PARAMETER.clone();
    structural
}

pub fn isEmptyArray(mut variable: Arc<NFVariable>) -> bool {
    let mut isEmpty: bool = Type::isEmptyArray(variable.ty.clone());
    isEmpty
}

pub fn isDeleted(mut variable: Arc<NFVariable>) -> Result<bool> {
    let mut deleted: bool = false;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    node = ComponentRef::node(variable.name.clone())?;
    deleted = InstNode::isComponent(node.clone()) && Component::isDeleted(InstNode::component(node.clone())?)?;
    Ok(deleted)
}

pub fn isPresent(mut variable: Arc<NFVariable>) -> bool {
    let mut present: bool = !(ConnectorType::isPotentiallyPresent(variable.attributes.connectorType.clone()));
    present
}

pub fn isPotential(mut variable: Arc<NFVariable>) -> bool {
    let mut potential: bool = ConnectorType::isPotential(variable.attributes.connectorType.clone());
    potential
}

pub fn isFlow(mut variable: Arc<NFVariable>) -> bool {
    let mut potential: bool = ConnectorType::isFlow(variable.attributes.connectorType.clone());
    potential
}

pub fn isStream(mut variable: Arc<NFVariable>) -> bool {
    let mut potential: bool = ConnectorType::isStream(variable.attributes.connectorType.clone());
    potential
}

pub fn isInput(mut variable: Arc<NFVariable>) -> bool {
    let mut b: bool = variable.attributes.direction.clone() == Direction::INPUT.clone();
    b
}

pub fn isTopLevelInput(mut variable: Arc<NFVariable>) -> bool {
    let mut topInput: bool = ComponentRef::isTopLevel(variable.name.clone()) && variable.attributes.direction.clone() == Direction::INPUT.clone();
    topInput
}

pub fn isPublic(mut variable: Arc<NFVariable>) -> bool {
    let mut isPublic: bool = variable.visibility.clone() == Visibility::PUBLIC.clone();
    isPublic
}

pub fn isProtected(mut variable: Arc<NFVariable>) -> bool {
    let mut isProtected: bool = variable.visibility.clone() == Visibility::PROTECTED.clone();
    isProtected
}

pub fn isEncrypted(mut variable: Arc<NFVariable>) -> Result<bool> {
    let mut isEncrypted: bool = false;
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    name = variable.name.clone();
    while ComponentRef::isCref(name.clone()) {
        info = InstNode::info(ComponentRef::node(name.clone())?)?;
        if StringUtil::endsWith(info.fileName.clone(), (literal!(".moc")).clone()) {
            isEncrypted = true;
            return Ok(isEncrypted.clone());
        }
        name = ComponentRef::rest(name.clone())?;
    }
    isEncrypted = false;
    Ok(isEncrypted)
}

pub fn isAccessible(mut variable: Arc<NFVariable>) -> Result<bool> {
    let mut isAccessible: bool = false;
    let mut oaccess: Option<AccessLevel> = None;
    let mut access: AccessLevel = AccessLevel::HIDE;
    oaccess = InstNode::getAccessLevel(ComponentRef::node(variable.name.clone())?)?;
    if isSome(oaccess.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(oaccess.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        access = __pa0.clone();
    } else {
        access = if (isEncrypted(variable.clone())?) {AccessLevel::DOCUMENTATION.clone()} else {AccessLevel::PACKAGE_DUPLICATE.clone()};
    }
    if access.clone() < AccessLevel::ICON.clone() {
        isAccessible = false;
    } else if access.clone() < AccessLevel::NON_PACKAGE_TEXT.clone() {
        isAccessible = !(isProtected(variable.clone()));
    } else {
        isAccessible = true;
    }
    Ok(isAccessible)
}

pub fn lookupTypeAttribute(mut name: ArcStr, mut var: Arc<NFVariable>) -> Arc<Binding::NFBinding> {
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    for mut attr in &*var.typeAttributes.clone() {
        let mut attr = attr.clone();
        if Util::tuple21(attr.clone()) == name.clone() {
            binding = Util::tuple22(attr.clone());
            return binding.clone();
        }
    }
    binding = Binding::EMPTY_BINDING().clone();
    binding
}

pub fn applyToType(mut var: Arc<NFVariable>, mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>) -> Arc<NFVariable> {
    pub type typeFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>;

    let mut var: Arc<NFVariable> = var;
    assign_field!(
        var.ty = func(var.ty.clone()).unwrap(),
        var.name = ComponentRef::applyToType(var.name.clone(), func.clone())
    );
    var
}

pub fn propagateAnnotation(mut name: ArcStr, mut overwrite: bool, mut evaluate: bool, mut var: Arc<NFVariable>) -> Result<Arc<NFVariable>> {
    let mut var: Arc<NFVariable> = var;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut anno: Arc<SCode::Annotation> = Arc::new(<SCode::Annotation as ::std::default::Default>::default());
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if ComponentRef::isCref(var.name.clone()) {
        node = ComponentRef::node(var.name.clone())?;
        if overwrite.clone() && InstNode::isComponent(node.clone()) {
            node = InstNode::parent(node.clone());
        }
        (r#mod, scope) = InstNode::getAnnotation((name.clone()).clone(), node.clone())?;
        if !(SCodeUtil::isEmptyMod(r#mod.clone())) {
            if evaluate.clone() {
                let () = 'mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { binding: Some(aexp), .. } => {
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    let mut r#mod: Arc<SCode::Mod> = r#mod.clone();
                    exp = Inst::instExp(aexp.clone(), scope.clone(), NFInstContext::ANNOTATION.clone(), var_field!((*r#mod).info, SCode::Mod::MOD).clone())?;
                    (exp, _, _, _) = Typing::typeExp(exp.clone(), NFInstContext::ANNOTATION.clone(), var_field!((*r#mod).info, SCode::Mod::MOD).clone(), false)?;
                    exp = Ceval::evalExp(exp.clone(), Ceval::noTarget().clone())?;
                    assign_variant_field!(r#mod => SCode::Mod::MOD; binding = Some(Expression::toAbsyn(exp.clone())?));
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
            }
            anno = Arc::new(SCode::Annotation { modification: Arc::new(SCode::Mod::MOD { info: metamodelica::sourceInfo!(), comment: None, binding: None, subModLst: list![Arc::new(SCode::SubMod { ident: (name.clone()).clone(), r#mod: r#mod.clone() })], eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL }) });
            assign_field!(var.comment = SCodeUtil::appendAnnotationToComment(anno.clone(), var.comment.clone(), true)?);
        }
    }
    Ok(var)
}

pub fn removeNonTopLevelDirection(mut var: Arc<NFVariable>) -> Result<Arc<NFVariable>> {
    let mut var: Arc<NFVariable> = var;
    let mut rest_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    if var.attributes.direction.clone() == Direction::NONE.clone() {
        return Ok(var.clone());
    }
    rest_name = ComponentRef::rest(var.name.clone())?;
    while !(ComponentRef::isEmpty(rest_name.clone())) {
        node = ComponentRef::node(rest_name.clone())?;
        if !(InstNode::isConnector(node.clone())? || InstNode::isInput(node.clone())) {
            attr = var.attributes.clone();
            assign_field!(attr.direction = Direction::NONE.clone());
            assign_field!(var.attributes = attr.clone());
            return Ok(var.clone());
        }
        rest_name = ComponentRef::rest(rest_name.clone())?;
    }
    Ok(var)
}

pub type ApplyFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

pub fn applyExp(mut var: Arc<NFVariable>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    Binding::applyExp(var.binding.clone(), r#fn.clone())?;
    for mut ty_attr in &*var.typeAttributes.clone() {
        let mut ty_attr = ty_attr.clone();
        Binding::applyExp(Util::tuple22(ty_attr.clone()), r#fn.clone())?;
    }
    for mut c in &*var.children.clone() {
        let mut c = c.clone();
        applyExp(c.clone(), r#fn.clone())?;
    }
    Ok(())
}

pub fn applyExpShallow(mut var: Arc<NFVariable>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> () {
    Binding::applyExpShallow(var.binding.clone(), r#fn.clone());
    for mut ty_attr in &*var.typeAttributes.clone() {
        let mut ty_attr = ty_attr.clone();
        Binding::applyExpShallow(Util::tuple22(ty_attr.clone()), r#fn.clone());
    }
    for mut c in &*var.children.clone() {
        let mut c = c.clone();
        applyExpShallow(c.clone(), r#fn.clone());
    }
    ()
}

pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

pub fn mapExp(mut var: Arc<NFVariable>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFVariable>> {
    let mut var: Arc<NFVariable> = var;
    assign_field!(
        var.binding = Binding::mapExp(var.binding.clone(), r#fn.clone())?,
        var.typeAttributes = ({
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
            let __x = (Util::tuple21(a.clone()), Binding::mapExp(Util::tuple22(a.clone()), r#fn.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        var.children = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFVariable>>> = metamodelica::nil();
        for mut v in (var.children.clone()).into_iter().cloned() {
            let __x = mapExp(v.clone(), r#fn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        var.backendinfo = BackendInfo::map(var.backendinfo.clone(), r#fn.clone()),
        var.ty = Type::applyToDims(var.ty.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = r#fn.clone(); move |__pe_a0| Dimension::mapExp(__pe_a0, __pe_b1.clone()) }))?,
        var.name = ComponentRef::mapTypes(var.name.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static> = Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = r#fn.clone(); move |__pe_a0| Dimension::mapExp(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Type::applyToDims(__pe_a0, __pe_b1.clone()) }))
    );
    Ok(var)
}

pub fn mapExpShallow(mut var: Arc<NFVariable>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFVariable> {
    let mut var: Arc<NFVariable> = var;
    assign_field!(
        var.binding = Binding::mapExpShallow(var.binding.clone(), r#fn.clone()),
        var.typeAttributes = ({
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
            let __x = (Util::tuple21(a.clone()), Binding::mapExpShallow(Util::tuple22(a.clone()), r#fn.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        var.children = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFVariable>>> = metamodelica::nil();
        for mut v in (var.children.clone()).into_iter().cloned() {
            let __x = mapExpShallow(v.clone(), r#fn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
    );
    var
}

pub fn toString(mut var: Arc<NFVariable>, mut indent: ArcStr, mut printBindingType: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
    s = IOStream::create((literal!("NFVariable.toString")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toStream(var.clone(), (indent.clone()).clone(), printBindingType.clone(), s.clone())?;
    r#str = (IOStream::string(s.clone())?).clone();
    IOStream::delete(s.clone())?;
    Ok(r#str)
}

pub fn toStream(mut var: Arc<NFVariable>, mut indent: ArcStr, mut printBindingType: bool, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut first: bool = false;
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    s = IOStream::append(s.clone(), (indent.clone()).clone())?;
    if var.visibility.clone() == Visibility::PROTECTED.clone() {
        s = IOStream::append(s.clone(), (literal!("protected ")).clone())?;
    }
    s = IOStream::append(s.clone(), (Attributes::toString(var.attributes.clone(), var.ty.clone())).clone())?;
    s = IOStream::append(s.clone(), (Type::toString(var.ty.clone())?).clone())?;
    s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
    s = IOStream::append(s.clone(), (ComponentRef::toString(var.name.clone())?).clone())?;
    if !(var.typeAttributes.clone().is_empty()) {
        s = IOStream::append(s.clone(), (literal!("(")).clone())?;
        first = true;
        for mut a in &*var.typeAttributes.clone() {
            let mut a = a.clone();
            if first.clone() {
                first = false;
            } else {
                s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            }
            b = Util::tuple22(a.clone());
            if Binding::isEach(b.clone()) {
                s = IOStream::append(s.clone(), (literal!("each ")).clone())?;
            }
            s = IOStream::append(s.clone(), (Util::tuple21(a.clone())).clone())?;
            s = IOStream::append(s.clone(), (literal!(" = ")).clone())?;
            s = IOStream::append(s.clone(), (Binding::toString(b.clone(), (literal!("")).clone())?).clone())?;
        }
        s = IOStream::append(s.clone(), (literal!(")")).clone())?;
    }
    if Binding::isBound(var.binding.clone()) {
        s = IOStream::append(s.clone(), (literal!(" = ")).clone())?;
        if printBindingType.clone() {
            s = IOStream::append(s.clone(), (literal!("(")).clone())?;
            s = IOStream::append(s.clone(), (Type::toString(Binding::getType(var.binding.clone())?)?).clone())?;
            s = IOStream::append(s.clone(), (literal!(") ")).clone())?;
        }
        s = IOStream::append(s.clone(), (Binding::toString(var.binding.clone(), (literal!("")).clone())?).clone())?;
    }
    Ok(s)
}

pub fn toFlatStream(mut var: Arc<NFVariable>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut printBindingType: bool, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    s = IOStream::append(s.clone(), (indent.clone()).clone())?;
    s = Attributes::toFlatStream(var.attributes.clone(), var.ty.clone(), s.clone(), ComponentRef::isSimple(var.name.clone()))?;
    s = IOStream::append(s.clone(), (Type::toFlatString(Type::arrayElementType(var.ty.clone()), format.clone())?).clone())?;
    s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
    s = IOStream::append(s.clone(), (ComponentRef::toFlatString(var.name.clone(), format.clone())?).clone())?;
    dims = Type::arrayDims(var.ty.clone());
    if !(dims.clone().is_empty()) {
        s = IOStream::append(s.clone(), (Dimension::toFlatStringList(dims.clone(), format.clone(), (literal!("")).clone())?).clone())?;
    }
    if !(var.typeAttributes.clone().is_empty()) {
        s = Component::typeAttrsToFlatStream(var.typeAttributes.clone(), var.ty.clone(), format.clone(), s.clone())?;
    } else if !(var.children.clone().is_empty()) {
        s = toFlatStreamModifier(var.children.clone(), format.moveBindings.clone() || Binding::isBound(var.binding.clone()), printBindingType.clone(), format.clone(), s.clone())?;
    }
    s = toFlatStreamBinding(var.binding.clone(), printBindingType.clone(), format.clone(), s.clone())?;
    s = FlatModelicaUtil::appendComment(var.comment.clone(), FlatModelicaUtil::ElementType::COMPONENT.clone(), s.clone())?;
    Ok(s)
}

pub fn toFlatStreamBinding(mut binding: Arc<Binding::NFBinding>, mut printBindingType: bool, mut format: BaseModelica::OutputFormat, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    if Binding::isBound(binding.clone()) {
        s = IOStream::append(s.clone(), (literal!(" = ")).clone())?;
        if printBindingType.clone() {
            s = IOStream::append(s.clone(), (literal!("(")).clone())?;
            s = IOStream::append(s.clone(), (Type::toFlatString(Binding::getType(binding.clone())?, format.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(") ")).clone())?;
        }
        s = IOStream::append(s.clone(), (Binding::toFlatString(binding.clone(), format.clone(), (literal!("")).clone())?).clone())?;
    }
    Ok(s)
}

pub fn toFlatStreamModifier(mut children: Arc<metamodelica::List<Arc<NFVariable>>>, mut overwrittenBinding: bool, mut printBindingType: bool, mut format: BaseModelica::OutputFormat, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut empty: bool = true;
    let mut overwritten_binding: bool = false;
    let mut ss: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
    let mut src: Binding::Source = Binding::Source::BINDING;
    for mut child in &*children.clone() {
        let mut child = child.clone();
        ss = IOStream::create((literal!("NFVariable.toFlatStreamModifier")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
        if !(child.typeAttributes.clone().is_empty()) {
            ss = Component::typeAttrsToFlatStream(child.typeAttributes.clone(), child.ty.clone(), format.clone(), ss.clone())?;
        } else if !(child.children.clone().is_empty()) {
            overwritten_binding = overwrittenBinding.clone() || Binding::isBound(child.binding.clone());
            ss = toFlatStreamModifier(child.children.clone(), overwritten_binding.clone(), printBindingType.clone(), format.clone(), ss.clone())?;
        }
        if !(overwrittenBinding.clone()) {
            src = Binding::source(child.binding.clone());
            if src.clone() == Binding::Source::MODIFIER.clone() || src.clone() == Binding::Source::GENERATED.clone() {
                ss = toFlatStreamBinding(child.binding.clone(), printBindingType.clone(), format.clone(), ss.clone())?;
            }
        }
        if !(IOStream::empty(ss.clone())?) {
            if empty.clone() {
                s = IOStream::append(s.clone(), (literal!("(")).clone())?;
                empty = false;
            } else {
                s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            }
            s = IOStream::append(s.clone(), (Util::makeQuotedIdentifier((ComponentRef::firstName(child.name.clone(), false)?).clone())?).clone())?;
            s = IOStream::appendListStream(ss.clone(), s.clone())?;
        }
    }
    if !(empty.clone()) {
        s = IOStream::append(s.clone(), (literal!(")")).clone())?;
    }
    Ok(s)
}

pub fn moveBinding(mut var: Arc<NFVariable>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<(Arc<NFVariable>, Arc<metamodelica::List<Arc<Equation::NFEquation>>>)> {
    let mut var: Arc<NFVariable> = var;
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    if variability(var.clone()) >= Variability::DISCRETE.clone() && Binding::isBound(var.binding.clone()) {
        equations = cons(Equation::makeEquality(Expression::fromCref(var.name.clone(), false)?, Binding::getExp(var.binding.clone())?, var.ty.clone(), ElementSource::createElementSource(var.info.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?, Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone()), equations.clone());
        assign_field!(var.binding = Binding::EMPTY_BINDING().clone());
    }
    Ok((var, equations))
}

pub fn getVariableAttributes(mut var: Arc<NFVariable>) -> Arc<VariableAttributes::VariableAttributes> {
    let mut variableAttributes: Arc<VariableAttributes::VariableAttributes> = var.backendinfo.attributes.clone();
    variableAttributes
}

pub fn getNominal(mut var: Arc<NFVariable>) -> Option<Arc<Expression::NFExpression>> {
    let mut nominal: Option<Arc<Expression::NFExpression>> = VariableAttributes::getNominal(getVariableAttributes(var.clone()));
    nominal
}

pub fn asBinding(mut var: Arc<NFVariable>, mut source: Binding::Source) -> Arc<Binding::NFBinding> {
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    binding = Binding::makeFlat(Expression::fromTypedCref(var.name.clone(), var.ty.clone()), variability(var.clone()), source.clone(), Binding::NO_CONFIDENCE.clone());
    binding
}


