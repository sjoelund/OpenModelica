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
use crate::NFBackendExtension::Annotations;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBinding as Binding;
use crate::NFCeval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::JSON;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

#[derive(Clone, Debug, PartialEq)]
pub enum NFComponentRef {
    CREF {
        node: Arc<InstNode::InstNode>,
        subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>,
        /// The type of the node, without taking subscripts into account.
        ty: Arc<Type::NFType>,
        origin: Origin,
        restCref: Arc<NFComponentRef>,
    },
    EMPTY,
    WILD,
}
pub use self::NFComponentRef::{CREF,EMPTY,WILD};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Origin {
    /// From an Absyn cref.
    CREF = 1,
    /// From prefixing the cref with its scope.
    SCOPE = 2,
    /// From an iterator.
    ITERATOR = 3,
}
impl PartialOrd for Origin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Origin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn fromNode(mut node: Arc<InstNode::InstNode>, mut ty: Arc<Type::NFType>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut origin: Origin) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::CREF { node: node.clone(), subscripts: subs.clone(), ty: ty.clone(), origin: origin.clone(), restCref: Arc::new(crate::NFComponentRef::EMPTY) });
    cref
}

pub fn prefixCref(mut node: Arc<InstNode::InstNode>, mut ty: Arc<Type::NFType>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut restCref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::CREF { node: node.clone(), subscripts: subs.clone(), ty: ty.clone(), origin: Origin::CREF.clone(), restCref: restCref.clone() });
    cref
}

pub fn prefixScope(mut node: Arc<InstNode::InstNode>, mut ty: Arc<Type::NFType>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut restCref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::CREF { node: node.clone(), subscripts: subs.clone(), ty: ty.clone(), origin: Origin::SCOPE.clone(), restCref: restCref.clone() });
    cref
}

pub fn fromAbsyn(mut node: Arc<InstNode::InstNode>, mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut restCref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let mut sl: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    sl = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Arc::new(Subscript::NFSubscript::RAW_SUBSCRIPT { subscript: s.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    cref = Arc::new(NFComponentRef::CREF { node: node.clone(), subscripts: sl.clone(), ty: Arc::new(crate::NFType::UNKNOWN), origin: Origin::CREF.clone(), restCref: restCref.clone() });
    cref
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn fromAbsynCref(mut acref: Arc<Absyn::ComponentRef>, mut restCref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    cref = (::match_deref::match_deref! { match &(acref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => fromAbsyn(Arc::new(InstNode::InstNode::NAME_NODE { name: (var_field!((*acref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone() }), var_field!((*acref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), restCref.clone()),
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => fromAbsynCref(var_field!((*acref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), fromAbsyn(Arc::new(InstNode::InstNode::NAME_NODE { name: (var_field!((*acref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone() }), var_field!((*acref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), restCref.clone()))?,
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => fromAbsynCref(var_field!((*acref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), Arc::new(crate::NFComponentRef::EMPTY))?,
        Deref @ Absyn::ComponentRef::WILD { .. } => Arc::new(crate::NFComponentRef::WILD),
        Deref @ Absyn::ComponentRef::ALLWILD { .. } => Arc::new(crate::NFComponentRef::WILD),
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn fromBuiltin(mut node: Arc<InstNode::InstNode>, mut ty: Arc<Type::NFType>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::CREF { node: node.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: Origin::SCOPE.clone(), restCref: Arc::new(crate::NFComponentRef::EMPTY) });
    cref
}

pub fn makeIterator(mut node: Arc<InstNode::InstNode>, mut ty: Arc<Type::NFType>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::CREF { node: node.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: Origin::ITERATOR.clone(), restCref: Arc::new(crate::NFComponentRef::EMPTY) });
    cref
}

pub fn isWild(mut cref: Arc<NFComponentRef>) -> bool {
    let mut isWild: bool = false;
    isWild = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ WILD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWild
}

pub fn isEmpty(mut cref: Arc<NFComponentRef>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn isSimple(mut cref: Arc<NFComponentRef>) -> bool {
    let mut isSimple: bool = false;
    isSimple = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { restCref: Deref @ EMPTY { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isSimple
}

pub fn isQualified(mut cref: Arc<NFComponentRef>) -> bool {
    let mut qualified: bool = false;
    qualified = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { restCref: Deref @ CREF { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    qualified
}

pub fn isTopLevel(mut cref: Arc<NFComponentRef>) -> bool {
    fn isTopLevelRecord(mut cref: Arc<NFComponentRef>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => Type::isRecord(var_field!((*cref).ty, NFComponentRef::CREF).clone()) && isTopLevelRecord(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        Deref @ EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { restCref: Deref @ EMPTY { .. }, .. } => true,
        Deref @ CREF { .. } => isTopLevelRecord(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isCref(mut cref: Arc<NFComponentRef>) -> bool {
    let mut isCref: bool = false;
    isCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCref
}

pub fn isIterator(mut cref: Arc<NFComponentRef>) -> bool {
    let mut isIterator: bool = false;
    isIterator = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::ITERATOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isIterator
}

pub fn isInput(mut cref: Arc<NFComponentRef>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => InstNode::isInput(var_field!((*cref).node, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isOutput(mut cref: Arc<NFComponentRef>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => InstNode::isOutput(var_field!((*cref).node, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isNameNode(mut cref: Arc<NFComponentRef>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { node: Deref @ InstNode::NAME_NODE { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isEqualRecordChild(mut child: Arc<NFComponentRef>, mut recd: Arc<NFComponentRef>) -> Result<bool> {
    let mut b: bool = size(child.clone(), true, false) == size(recd.clone(), true, false);
    if b.clone() {
        b = isRecordChild(child.clone(), recd.clone())?;
    }
    Ok(b)
}

pub fn isRecordChild(mut child: Arc<NFComponentRef>, mut recd: Arc<NFComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(recd.clone()) {
        Deref @ CREF { .. } => isEqual(child.clone(), recd.clone())? || isRecordChild(child.clone(), var_field!((*recd).restCref, NFComponentRef::CREF).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn node(mut cref: Arc<NFComponentRef>) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let __pa0 = ::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { node: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    node = __pa0.clone();
    Ok(node)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn nodes(mut cref: Arc<NFComponentRef>, mut accum: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Arc<metamodelica::List<Arc<InstNode::InstNode>>> {
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    nodes = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => self::nodes(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(var_field!((*cref).node, NFComponentRef::CREF).clone(), accum.clone())),
        _ => accum.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    nodes
}

pub fn nodesIncludingSplitSubs(mut cref: Arc<NFComponentRef>, mut accum: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> {
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = accum.clone();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    nodes = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            for mut s in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                let mut s = s.clone();
                if Subscript::isSplitIndex(s.clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(s.clone()) {
                        Deref @ Subscript::SPLIT_INDEX { node: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    node = __pa0.clone();
                    nodes = cons(node.clone(), nodes.clone());
                }
            }
            nodesIncludingSplitSubs(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(var_field!((*cref).node, NFComponentRef::CREF).clone(), nodes.clone()))?
        },
        _ => nodes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(nodes)
}

pub fn containsNode(mut cref: Arc<NFComponentRef>, mut node: Arc<InstNode::InstNode>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => InstNode::refEqual(var_field!((*cref).node, NFComponentRef::CREF).clone(), node.clone()) || containsNode(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), node.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn nodeType(mut cref: Arc<NFComponentRef>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let __pa0 = ::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { ty: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    Ok(ty)
}

pub fn setNodeType(mut ty: Arc<Type::NFType>, mut cref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; ty = ty.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn updateNodeType(mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } if (InstNode::isComponent(var_field!((*cref).node, NFComponentRef::CREF).clone())) => {
            assign_variant_field!(cref => NFComponentRef::CREF; ty = InstNode::getType(var_field!((*cref).node, NFComponentRef::CREF).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn scalarType(mut cref: Arc<NFComponentRef>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let __pa0 = ::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { ty: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    ty = Type::arrayElementType(ty.clone());
    Ok(ty)
}

pub fn applyToType(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>) -> Arc<NFComponentRef> {
    pub type typeFunc = fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>>;

    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF;
                ty = func(var_field!((*cref).ty, NFComponentRef::CREF).clone()).unwrap(),
                restCref = applyToType(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone())
            );
            cref.clone()
        },
        _ => cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn firstName(mut cref: Arc<NFComponentRef>, mut baseModelica: bool) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?,
        Deref @ WILD { .. } => if (baseModelica.clone()) {literal!("")} else {literal!("_")},
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(name)
}

pub fn first(mut cref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = Arc::new(crate::NFComponentRef::EMPTY));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn rest(mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut restCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let __pa0 = ::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { restCref: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    restCref = __pa0.clone();
    Ok(restCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn last(mut cref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut lastCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    lastCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { restCref: Deref @ CREF { .. }, .. } => last(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        _ => cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lastCref
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn firstNonScope(mut cref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut first: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let mut rest_cr: Arc<NFComponentRef> = rest(cref.clone()).unwrap();
    first = (::match_deref::match_deref! { match &(rest_cr.clone()) {
        Deref @ CREF { origin: Origin::SCOPE, .. } => cref.clone(),
        Deref @ EMPTY { .. } => cref.clone(),
        _ => firstNonScope(rest_cr.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    first
}

pub fn append(mut cref: Arc<NFComponentRef>, mut restCref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = append(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), restCref.clone())?);
            cref.clone()
        },
        Deref @ EMPTY { .. } => restCref.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn appendScope(mut scope: Arc<InstNode::InstNode>, mut cref: Arc<NFComponentRef>, mut includeRoot: bool) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let mut prefix: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    prefix = fromNodeList(InstNode::scopeList(scope.clone(), includeRoot.clone(), metamodelica::nil())?)?;
    if !(isEmpty(prefix.clone())) {
        cref = append(cref.clone(), prefix.clone())?;
        cref = removeOuterCrefPrefix(cref.clone());
    }
    Ok(cref)
}

pub fn prepend(mut restCref: Arc<NFComponentRef>, mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = restCref.clone());
            cref.clone()
        },
        Deref @ EMPTY { .. } => restCref.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn getComponentType(mut cref: Arc<NFComponentRef>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => var_field!((*cref).ty, NFComponentRef::CREF).clone(),
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn getSubscriptedType(mut cref: Arc<NFComponentRef>, mut includeScope: bool) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => getSubscriptedType2(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), Type::subscript(var_field!((*cref).ty, NFComponentRef::CREF).clone(), var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), true)?, includeScope.clone())?,
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn getSubscriptedType2(mut restCref: Arc<NFComponentRef>, mut accumTy: Arc<Type::NFType>, mut includeScope: bool) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(restCref.clone()) {
        Deref @ CREF { .. } if (var_field!((*restCref).origin, NFComponentRef::CREF).clone() == Origin::CREF.clone() || includeScope.clone()) => {
            ty = Type::liftArrayLeftList(accumTy.clone(), Type::arrayDims(Type::subscript(var_field!((*restCref).ty, NFComponentRef::CREF).clone(), var_field!((*restCref).subscripts, NFComponentRef::CREF).clone(), true)?));
            getSubscriptedType2(var_field!((*restCref).restCref, NFComponentRef::CREF).clone(), ty.clone(), includeScope.clone())?
        },
        _ => accumTy.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn lookupVarAttr(mut cref: Arc<NFComponentRef>, mut attr_name: ArcStr) -> Option<Arc<Expression::NFExpression>> {
    let mut attrValue: Option<Arc<Expression::NFExpression>> = None;
    attrValue = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { node: Deref @ InstNode::VAR_NODE { varPointer: v, .. }, .. } => {
            Binding::typedExp(Variable::lookupTypeAttribute((attr_name.clone()).clone(), Pointer::access(v.clone())))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attrValue
}

pub fn nodeVariability(mut cref: Arc<NFComponentRef>) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    var = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { node: Deref @ InstNode::COMPONENT_NODE { .. }, .. } => {
            Component::variability(InstNode::component(var_field!((*cref).node, NFComponentRef::CREF).clone())?)
        },
        Deref @ CREF { node: Deref @ InstNode::CLASS_NODE { .. }, .. } => {
            Variability::CONSTANT.clone()
        },
        Deref @ CREF { node: Deref @ InstNode::VAR_NODE { varPointer: v, .. }, .. } => {
            Variable::variability(Pointer::access(v.clone()))
        },
        _ => {
            Variability::CONTINUOUS.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn isResizable(mut cref: Arc<NFComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { node: Deref @ InstNode::COMPONENT_NODE { .. }, .. } => {
            Component::isResizable(InstNode::component(var_field!((*cref).node, NFComponentRef::CREF).clone())?)
        },
        Deref @ CREF { node: Deref @ InstNode::VAR_NODE { varPointer: v, .. }, .. } => {
            (::match_deref::match_deref! { match &(Pointer::access(v.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ Annotations::ANNOTATIONS { resizable: b, .. }, .. }, .. } => b.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn subscriptsVariability(mut cref: Arc<NFComponentRef>, mut var: Variability) -> Result<Variability> {
    let mut var: Variability = var;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => {
            for mut sub in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                let mut sub = sub.clone();
                var = Prefixes::variabilityMax(var.clone(), Subscript::variability(sub.clone())?);
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn variability(mut cref: Arc<NFComponentRef>) -> Variability {
    let mut var: Variability = Prefixes::variabilityMax(nodeVariability(cref.clone()).unwrap(), subscriptsVariability(cref.clone(), Prefixes::Variability::CONSTANT.clone()).unwrap());
    var
}

pub fn purity(mut cref: Arc<NFComponentRef>) -> Purity {
    fn sub_purity(mut sub: Arc<Subscript::NFSubscript>, mut pur: Purity) -> Result<Purity> {
        let mut pur: Purity = pur;
        pur = Prefixes::purityMin(pur.clone(), Subscript::purity(sub.clone())?);
        Ok(pur)
    }

    let mut pur: Purity = Purity::PURE;
    pur = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::ITERATOR { .. }, .. } => Purity::IMPURE.clone(),
        Deref @ CREF { .. } => foldSubscripts(cref.clone(), Arc::new(sub_purity), Purity::PURE.clone(), false),
        _ => Purity::IMPURE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    pur
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn visibility(mut cref: Arc<NFComponentRef>) -> Visibility {
    let mut vis: Visibility = Visibility::PUBLIC;
    vis = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => if (InstNode::isProtected(var_field!((*cref).node, NFComponentRef::CREF).clone())) {Visibility::PROTECTED.clone()} else {visibility(var_field!((*cref).restCref, NFComponentRef::CREF).clone())},
        _ => Visibility::PUBLIC.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    vis
}

pub fn rename(mut name: ArcStr, mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; node = InstNode::rename((name.clone()).clone(), var_field!((*cref).node, NFComponentRef::CREF).clone())?);
            cref.clone()
        },
        _ => cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn addSubscript(mut subscript: Arc<Subscript::NFSubscript>, mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; subscripts = listAppend(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), list![subscript.clone()]));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn mergeSubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut cref: Arc<NFComponentRef>, mut applyToScope: bool, mut backend: bool, mut reverse: bool) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let mut old_cref: Arc<NFComponentRef> = cref.clone();
    let mut new_subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    (new_subscripts, cref) = mergeSubscripts2(subscripts.clone(), cref.clone(), applyToScope.clone(), backend.clone(), reverse.clone())?;
    if !(new_subscripts.clone().is_empty()) {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFComponentRef.mergeSubscripts")); __mm_s.push_str(&*literal!(" failed because the subscripts ")); __mm_s.push_str(&*List::toString(subscripts.clone(), Arc::new(Subscript::toString), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!(" could not be fully merged onto ")); __mm_s.push_str(&*toString(old_cref.clone())?); __mm_s.push_str(&*literal!(".\nResult: ")); __mm_s.push_str(&*toString(cref.clone())?); __mm_s.push_str(&*literal!(" with leftover: ")); __mm_s.push_str(&*List::toString(new_subscripts.clone(), Arc::new(Subscript::toString), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok(cref)
}

pub fn mergeSubscripts2(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut cref: Arc<NFComponentRef>, mut applyToScope: bool, mut backend: bool, mut reverse: bool) -> Result<(Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, Arc<NFComponentRef>)> {
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = subscripts;
    let mut cref: Arc<NFComponentRef> = cref;
    (subscripts, cref) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { subscripts: cref_subs, .. } if (applyToScope.clone() || var_field!((*cref).origin, NFComponentRef::CREF).clone() == Origin::CREF.clone()) => {
            let mut rest_cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            let mut cref_subs = (*cref_subs).clone();
            if !(reverse.clone()) {
                (subscripts, rest_cref) = mergeSubscripts2(subscripts.clone(), var_field!((*cref).restCref, NFComponentRef::CREF).clone(), applyToScope.clone(), backend.clone(), reverse.clone())?;
            }
            if !(subscripts.clone().is_empty()) {
                (cref_subs, subscripts) = Subscript::mergeList(subscripts.clone(), cref_subs.clone(), Type::dimensionCount(var_field!((*cref).ty, NFComponentRef::CREF).clone()), backend.clone())?;
            }
            if reverse.clone() {
                cref_subs = cref_subs.clone().reverse();
                (subscripts, rest_cref) = mergeSubscripts2(subscripts.clone(), var_field!((*cref).restCref, NFComponentRef::CREF).clone(), applyToScope.clone(), backend.clone(), reverse.clone())?;
            }
            (subscripts.clone(), Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: cref_subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest_cref.clone() }))
        },
        _ => {
            (subscripts.clone(), cref.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((subscripts, cref))
}

pub fn mergeSubscriptsMapped(mut cref: Arc<NFComponentRef>, mut dims_map: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, Arc<metamodelica::List<Arc<NFComponentRef>>>>>, mut iter_map: Arc<UnorderedMap::UnorderedMap<Arc<NFComponentRef>, Arc<Subscript::NFSubscript>>>) -> Result<Arc<NFComponentRef>> {
    fn checkLocalDimensions(mut cref: Arc<NFComponentRef>, mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut dims_map: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, Arc<metamodelica::List<Arc<NFComponentRef>>>>>, mut iter_map: Arc<UnorderedMap::UnorderedMap<Arc<NFComponentRef>, Arc<Subscript::NFSubscript>>>) -> Result<Arc<NFComponentRef>> {
        let mut cref: Arc<NFComponentRef> = cref;
        let mut iter_crefs: Option<Arc<metamodelica::List<Arc<NFComponentRef>>>> = None;
        let mut new_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        iter_crefs = UnorderedMap::get(dims.clone(), dims_map.clone());
        if isSome(iter_crefs.clone()) {
            new_subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut iter_name in (Util::getOption(iter_crefs.clone())?).into_iter().cloned() {
            let __x = UnorderedMap::getSafe(iter_name.clone(), iter_map.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            cref = mergeSubscripts(new_subs.clone(), cref.clone(), true, true, true)?;
        }
        Ok(cref)
    }

    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } if (Type::isArray(ty.clone())) => {
            let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            let mut new_cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            let mut ty: Arc<Type::NFType> = getSubscriptedType(cref.clone(), false)?;
            let mut num_local_dims: i32 = 0;
            dims = Type::arrayDims(ty.clone());
            num_local_dims = (Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone()).len() as i32);
            new_cref = cref.clone();
            while num_local_dims.clone() > 0 {
                new_cref = checkLocalDimensions(new_cref.clone(), dims.clone(), dims_map.clone(), iter_map.clone())?;
                dims = List::stripLast(dims.clone())?;
                num_local_dims = num_local_dims.clone() - 1;
            }
            new_cref = (::match_deref::match_deref! { match &(new_cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(new_cref => NFComponentRef::CREF; restCref = mergeSubscriptsMapped(var_field!((*new_cref).restCref, NFComponentRef::CREF).clone(), dims_map.clone(), iter_map.clone())?);
            new_cref.clone()
        },
        _ => new_cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            new_cref.clone()
        },
        Deref @ CREF { .. } => {
            let mut ty: Arc<Type::NFType> = getSubscriptedType(cref.clone(), false)?;
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = mergeSubscriptsMapped(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), dims_map.clone(), iter_map.clone())?);
            cref.clone()
        },
        _ => {
            let mut ty: Arc<Type::NFType> = getSubscriptedType(cref.clone(), false)?;
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn hasSubscripts(mut cref: Arc<NFComponentRef>) -> bool {
    let mut hasSubscripts: bool = false;
    hasSubscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => !(var_field!((*cref).subscripts, NFComponentRef::CREF).clone().is_empty()) || self::hasSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasSubscripts
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn hasNonModelSubscripts(mut cref: Arc<NFComponentRef>) -> bool {
    let mut hasSubscripts: bool = false;
    hasSubscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } if (InstNode::isModel(var_field!((*cref).node, NFComponentRef::CREF).clone())) => hasNonModelSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        Deref @ CREF { .. } => !(var_field!((*cref).subscripts, NFComponentRef::CREF).clone().is_empty()) || hasNonModelSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasSubscripts
}

pub fn hasSplitSubscripts(mut cref: Arc<NFComponentRef>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => List::any(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), Arc::new(fnptr!(Subscript::isSplitIndex, Arc<Subscript::NFSubscript>))) || hasSplitSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn expandSplitSubscripts(mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF;
                subscripts = Subscript::expandSplitIndices(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), metamodelica::nil())?,
                restCref = expandSplitSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn getSubscripts(mut cref: Arc<NFComponentRef>) -> Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    subscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => var_field!((*cref).subscripts, NFComponentRef::CREF).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    subscripts
}

pub fn setSubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; subscripts = subscripts.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn setSubscriptsList(mut subscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>, mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &((subscripts.clone(), cref.clone())) {
        (Deref @ metamodelica::List::Cons { head: subs, tail: rest_subs }, Deref @ CREF { .. }) => {
            let mut rest_cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            rest_cref = setSubscriptsList(rest_subs.clone(), var_field!((*cref).restCref, NFComponentRef::CREF).clone())?;
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest_cref.clone() })
        },
        (Deref @ metamodelica::List::Nil, _) => {
            cref.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn copySubscripts(mut origin: Arc<NFComponentRef>, mut target: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut target: Arc<NFComponentRef> = target;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = subscriptsAllFlat(origin.clone());
    if !(subs.clone().is_empty()) {
        target = mergeSubscripts(subs.clone(), target.clone(), true, true, false)?;
    }
    Ok(target)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn subscriptsAllWithWhole(mut cref: Arc<NFComponentRef>, mut accumSubs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    subscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { subscripts: Deref @ metamodelica::List::Nil, .. } => {
            let mut sizes_: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            sizes_ = sizes_local_exp(cref.clone(), false)?;
            subs = metamodelica::nil();
            for mut size in &*sizes_.clone().reverse() {
                let mut size = size.clone();
                if !(Expression::isOne(size.clone())) {
                    subs = cons(Arc::new(Subscript::NFSubscript::SLICE { slice: Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), None, size.clone())? }), subs.clone());
                }
            }
            subscriptsAllWithWhole(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(subs.clone(), accumSubs.clone()))?
        },
        Deref @ CREF { .. } => {
            subscriptsAllWithWhole(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), accumSubs.clone()))?
        },
        _ => {
            accumSubs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(subscripts)
}

pub fn subscriptsAllWithWholeFlat(mut cref: Arc<NFComponentRef>) -> Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = List::flatten(subscriptsAllWithWhole(cref.clone(), metamodelica::nil()).unwrap());
    subscripts
}

pub fn subscriptsAll(mut cref: Arc<NFComponentRef>) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = subscriptsAllReverse(cref.clone(), metamodelica::nil()).reverse();
    subscripts
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn subscriptsAllReverse(mut cref: Arc<NFComponentRef>, mut accumSubs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    subscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => subscriptsAllReverse(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), accumSubs.clone())),
        _ => accumSubs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    subscripts
}

pub fn subscriptsAllFlat(mut cref: Arc<NFComponentRef>) -> Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = List::flattenReverse(subscriptsAll(cref.clone()));
    subscripts
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn subscriptsExceptModel(mut cref: Arc<NFComponentRef>, mut accumSubs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    subscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } if (InstNode::isModel(var_field!((*cref).node, NFComponentRef::CREF).clone())) => subscriptsExceptModel(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(metamodelica::nil(), accumSubs.clone())),
        Deref @ CREF { .. } => subscriptsExceptModel(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), accumSubs.clone())),
        _ => accumSubs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    subscripts
}

pub fn subscriptsN(mut cref: Arc<NFComponentRef>, mut n: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut rest: Arc<NFComponentRef> = cref.clone();
    for mut i in 1..=n.clone() {
        if isEmpty(rest.clone()) {
            break;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ CREF { restCref: __pa0, subscripts: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        rest = __pa0.clone();
        subs = __pa1.clone();
        subscripts = cons(subs.clone(), subscripts.clone());
    }
    Ok(subscripts)
}

pub fn transferSubscripts(mut srcCref: Arc<NFComponentRef>, mut dstCref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    cref = (::match_deref::match_deref! { match &((srcCref.clone(), dstCref.clone())) {
        (Deref @ EMPTY { .. }, _) => dstCref.clone(),
        (_, Deref @ EMPTY { .. }) => dstCref.clone(),
        (_, Deref @ WILD { .. }) => dstCref.clone(),
        (_, Deref @ CREF { origin: Origin::ITERATOR { .. }, .. }) => dstCref.clone(),
        (Deref @ CREF { .. }, Deref @ CREF { origin: Origin::CREF { .. }, .. }) => {
            assign_variant_field!(dstCref => NFComponentRef::CREF; restCref = transferSubscripts(srcCref.clone(), var_field!((*dstCref).restCref, NFComponentRef::CREF).clone())?);
            dstCref.clone()
        },
        (Deref @ CREF { .. }, Deref @ CREF { .. }) if (InstNode::refEqual(var_field!((*srcCref).node, NFComponentRef::CREF).clone(), var_field!((*dstCref).node, NFComponentRef::CREF).clone())) => {
            cref = transferSubscripts(var_field!((*srcCref).restCref, NFComponentRef::CREF).clone(), var_field!((*dstCref).restCref, NFComponentRef::CREF).clone())?;
            subs = if (var_field!((*srcCref).subscripts, NFComponentRef::CREF).clone().is_empty()) {var_field!((*dstCref).subscripts, NFComponentRef::CREF).clone()} else {var_field!((*srcCref).subscripts, NFComponentRef::CREF).clone()};
            Arc::new(NFComponentRef::CREF { node: var_field!((*dstCref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*dstCref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*dstCref).origin, NFComponentRef::CREF).clone(), restCref: cref.clone() })
        },
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => transferSubscripts(var_field!((*srcCref).restCref, NFComponentRef::CREF).clone(), dstCref.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFComponentRef.transferSubscripts")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn applySubscripts(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<()> + 'static>, mut applyToScope: bool) -> () {
    pub type FuncT = fn(Arc<Subscript::NFSubscript>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } if (applyToScope.clone() || var_field!((*cref).origin, NFComponentRef::CREF).clone() == Origin::CREF.clone()) => {
            for mut sub in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                let mut sub = sub.clone();
                func(sub.clone()).unwrap();
            }
            applySubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone(), applyToScope.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn foldSubscripts<ArgT: Clone + 'static>(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT, mut applyToScope: bool) -> ArgT {
    pub type FuncT<ArgT: Clone> = fn(Arc<Subscript::NFSubscript>, ArgT) -> Result<ArgT>;

    let mut arg: ArgT = arg;
    arg = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } if (applyToScope.clone() || var_field!((*cref).origin, NFComponentRef::CREF).clone() == Origin::CREF.clone()) => {
            for mut sub in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                let mut sub = sub.clone();
                arg = func(sub.clone(), arg.clone()).unwrap();
            }
            foldSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone(), arg.clone(), applyToScope.clone())
        },
        _ => arg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    arg
}

pub fn mapSubscripts(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<Arc<Subscript::NFSubscript>> + 'static>, mut applyToScope: bool) -> Arc<NFComponentRef> {
    pub type FuncT = fn(Arc<Subscript::NFSubscript>) -> Result<Arc<Subscript::NFSubscript>>;

    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } if (applyToScope.clone() || var_field!((*cref).origin, NFComponentRef::CREF).clone() == Origin::CREF.clone()) => {
            if !(var_field!((*cref).subscripts, NFComponentRef::CREF).clone().is_empty()) {
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = func(s.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = mapSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone(), applyToScope.clone()));
            cref.clone()
        },
        _ => cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn fillSubscripts(mut cref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            let mut dim_count: i32 = 0;
            let mut sub_count: i32 = 0;
            dims = Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone());
            dim_count = (dims.clone().len() as i32);
            sub_count = (var_field!((*cref).subscripts, NFComponentRef::CREF).clone().len() as i32);
            if sub_count.clone() < dim_count.clone() {
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = listAppend(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), List::fill(Arc::new(crate::NFSubscript::WHOLE), dim_count.clone() - sub_count.clone())));
            }
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = fillSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone()));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn replaceWholeSubscripts(mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            if List::any(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), Arc::new(fnptr!(Subscript::isWhole, Arc<Subscript::NFSubscript>))) {
                dims = Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone());
                subs = metamodelica::nil();
                for mut s in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                    let mut s = s.clone();
                    if Subscript::isWhole(s.clone()) {
                        s = Subscript::fromDimension(listHead(dims.clone())?)?;
                    }
                    subs = cons(s.clone(), subs.clone());
                    dims = listRest(dims.clone())?;
                }
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = subs.clone().reverse());
            }
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = replaceWholeSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?);
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn combineSubscripts(mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    cref = fillSubscripts(cref.clone());
    subs = List::flatten(subscriptsAllReverse(cref.clone(), metamodelica::nil()));
    if subs.clone().is_empty() {
        return Ok(cref);
    }
    cref = setSubscripts(subs.clone(), stripSubscriptsAll(cref.clone()))?;
    Ok(cref)
}

pub fn compare(mut cref1: Arc<NFComponentRef>, mut cref2: Arc<NFComponentRef>) -> Result<i32> {
    let mut comp: i32 = 0;
    comp = (::match_deref::match_deref! { match &((cref1.clone(), cref2.clone())) {
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => {
            comp = stringCompare((InstNode::name(var_field!((*cref1).node, NFComponentRef::CREF).clone())?).clone(), (InstNode::name(var_field!((*cref2).node, NFComponentRef::CREF).clone())?).clone());
            if comp.clone() != 0 {
                return Ok(comp);
            }
            comp = Subscript::compareList(var_field!((*cref1).subscripts, NFComponentRef::CREF).clone(), var_field!((*cref2).subscripts, NFComponentRef::CREF).clone())?;
            if comp.clone() != 0 {
                return Ok(comp);
            }
            compare(var_field!((*cref1).restCref, NFComponentRef::CREF).clone(), var_field!((*cref2).restCref, NFComponentRef::CREF).clone())?
        },
        (Deref @ EMPTY { .. }, Deref @ EMPTY { .. }) => 0,
        (Deref @ WILD { .. }, Deref @ WILD { .. }) => 0,
        (_, Deref @ EMPTY { .. }) => 1,
        (_, Deref @ WILD { .. }) => 1,
        (Deref @ EMPTY { .. }, _) => -1,
        (Deref @ WILD { .. }, _) => -1,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFComponentRef.compare")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub fn isEqual(mut cref1: Arc<NFComponentRef>, mut cref2: Arc<NFComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    if referenceEq(&cref1.clone(),&cref2.clone()) {
        b = true;
        return Ok(b);
    }
    b = (::match_deref::match_deref! { match &((cref1.clone(), cref2.clone())) {
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => InstNode::name(var_field!((*cref1).node, NFComponentRef::CREF).clone())? == InstNode::name(var_field!((*cref2).node, NFComponentRef::CREF).clone())? && Subscript::isEqualList(var_field!((*cref1).subscripts, NFComponentRef::CREF).clone(), var_field!((*cref2).subscripts, NFComponentRef::CREF).clone())? && isEqual(var_field!((*cref1).restCref, NFComponentRef::CREF).clone(), var_field!((*cref2).restCref, NFComponentRef::CREF).clone())?,
        (Deref @ EMPTY { .. }, Deref @ EMPTY { .. }) => true,
        (Deref @ WILD { .. }, Deref @ WILD { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isEqualStrip(mut cref1: Arc<NFComponentRef>, mut cref2: Arc<NFComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    if referenceEq(&cref1.clone(),&cref2.clone()) {
        b = true;
        return Ok(b);
    }
    b = (::match_deref::match_deref! { match &((cref1.clone(), cref2.clone())) {
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => InstNode::name(var_field!((*cref1).node, NFComponentRef::CREF).clone())? == InstNode::name(var_field!((*cref2).node, NFComponentRef::CREF).clone())? && isEqualStrip(var_field!((*cref1).restCref, NFComponentRef::CREF).clone(), var_field!((*cref2).restCref, NFComponentRef::CREF).clone())?,
        (Deref @ EMPTY { .. }, Deref @ EMPTY { .. }) => true,
        (Deref @ WILD { .. }, Deref @ WILD { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isLess(mut cref1: Arc<NFComponentRef>, mut cref2: Arc<NFComponentRef>) -> bool {
    let mut isLess: bool = compare(cref1.clone(), cref2.clone()).unwrap() < 0;
    isLess
}

pub fn isGreater(mut cref1: Arc<NFComponentRef>, mut cref2: Arc<NFComponentRef>) -> bool {
    let mut isGreater: bool = compare(cref1.clone(), cref2.clone()).unwrap() > 0;
    isGreater
}

pub fn isPrefix(mut cref1: Arc<NFComponentRef>, mut cref2: Arc<NFComponentRef>) -> Result<bool> {
    let mut isPrefix: bool = false;
    if referenceEq(&cref1.clone(),&cref2.clone()) {
        isPrefix = true;
        return Ok(isPrefix);
    }
    isPrefix = (::match_deref::match_deref! { match &((cref1.clone(), cref2.clone())) {
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => if (InstNode::name(var_field!((*cref1).node, NFComponentRef::CREF).clone())? == InstNode::name(var_field!((*cref2).node, NFComponentRef::CREF).clone())?) {isEqual(var_field!((*cref1).restCref, NFComponentRef::CREF).clone(), var_field!((*cref2).restCref, NFComponentRef::CREF).clone())?} else {isEqual(cref1.clone(), var_field!((*cref2).restCref, NFComponentRef::CREF).clone())?},
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isPrefix)
}

pub fn toAbsyn(mut cref: Arc<NFComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut acref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    acref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            acref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?).clone(), subscripts: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = Subscript::toAbsyn(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
            toAbsyn_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), acref.clone())?
        },
        Deref @ WILD { .. } => Arc::new(openmodelica_ast::Absyn::ComponentRef::WILD),
        _ => bail!("match: no arm matched"),
    } });
    Ok(acref)
}

pub fn toAbsyn_impl(mut cref: Arc<NFComponentRef>, mut accumCref: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut acref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    acref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ EMPTY { .. } => accumCref.clone(),
        Deref @ CREF { .. } => {
            acref = Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?).clone(), subscripts: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = Subscript::toAbsyn(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, componentRef: accumCref.clone() });
            toAbsyn_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), acref.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(acref)
}

pub fn toDAE(mut cref: Arc<NFComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut dcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    dcref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            dcref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?).clone(), identType: Type::toDAE(var_field!((*cref).ty, NFComponentRef::CREF).clone(), true)?, subscriptLst: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = Subscript::toDAE(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
            toDAE_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), dcref.clone())?
        },
        Deref @ WILD { .. } => Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD),
        _ => bail!("match: no arm matched"),
    } });
    Ok(dcref)
}

pub fn toDAE_impl(mut cref: Arc<NFComponentRef>, mut accumCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut dcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    dcref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ EMPTY { .. } => {
            accumCref.clone()
        },
        Deref @ CREF { .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut dty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ty = if (Type::isUnknown(var_field!((*cref).ty, NFComponentRef::CREF).clone())) {InstNode::getType(var_field!((*cref).node, NFComponentRef::CREF).clone())?} else {var_field!((*cref).ty, NFComponentRef::CREF).clone()};
            dty = Type::toDAE(ty.clone(), false)?;
            dcref = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?).clone(), identType: dty.clone(), subscriptLst: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = Subscript::toDAE(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, componentRef: accumCref.clone() });
            toDAE_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), dcref.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(dcref)
}

pub fn toString(mut cref: Arc<NFComponentRef>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList(toString_impl(cref.clone(), metamodelica::nil())?, (literal!(".")).clone());
    Ok(r#str)
}

pub fn toString_impl(mut cref: Arc<NFComponentRef>, mut strl: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strl: Arc<metamodelica::List<ArcStr>> = strl;
    strl = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?); __mm_s.push_str(&*Subscript::toStringList(var_field!((*cref).subscripts, NFComponentRef::CREF).clone())?); ArcStr::from(__mm_s) }).clone();
            toString_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(r#str.clone(), strl.clone()))?
        },
        Deref @ WILD { .. } => {
            cons(literal!("_"), strl.clone())
        },
        _ => {
            strl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(strl)
}

pub fn toFlatString(mut cref: Arc<NFComponentRef>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<NFComponentRef>>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut cr: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let mut escapeQuotes: bool = false;
    r#str = (firstName(cref.clone(), true)?).clone();
    if r#str.clone() == literal!("time") || r#str.clone() == literal!("") {
        return Ok(r#str);
    }
    crefs = toListReverse(cref.clone(), true, metamodelica::nil());
    strl = list![(literal!("'")).clone()];
    subs = metamodelica::nil();
    if format.scalarizeMode.clone() == BaseModelica::ScalarizeMode::NOT_SCALARIZED.clone() {
        while !(crefs.clone().is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            crefs = __pa1.clone();
            strl = cons(Util::escapeQuotes((firstName(cr.clone(), true)?).clone())?, strl.clone());
            subs = listAppend(getSubscripts(cr.clone()), subs.clone());
            if format.recordMode.clone() == BaseModelica::RecordMode::WITH_RECORDS.clone() && isCref(cr.clone()) && Type::isRecord(scalarType(cr.clone())?) && !(crefs.clone().is_empty()) {
                strl = cons(literal!("'"), strl.clone());
                if !(subs.clone().is_empty()) {
                    strl = cons(Subscript::toFlatStringList(subs.clone(), format.clone(), false)?, strl.clone());
                    subs = metamodelica::nil();
                }
                if !(crefs.clone().is_empty()) {
                    strl = cons(literal!(".'"), strl.clone());
                }
            } else if !(crefs.clone().is_empty()) {
                strl = cons(literal!("."), strl.clone());
            }
        }
    } else {
        while !(crefs.clone().is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(crefs.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa2.clone();
            crefs = __pa3.clone();
            strl = cons(Util::escapeQuotes((firstName(cr.clone(), true)?).clone())?, strl.clone());
            subs = getSubscripts(cr.clone());
            if !(subs.clone().is_empty()) && !(format.scalarizeMode.clone() == BaseModelica::ScalarizeMode::PARTIALLY_SCALARIZED.clone() && crefs.clone().is_empty()) {
                strl = cons(Subscript::toFlatStringList(subs.clone(), format.clone(), true)?, strl.clone());
            }
            if !(crefs.clone().is_empty()) {
                if format.recordMode.clone() == BaseModelica::RecordMode::WITH_RECORDS.clone() && isCref(cr.clone()) && Type::isRecord(scalarType(cr.clone())?) {
                    strl = cons(literal!("'.'"), strl.clone());
                } else {
                    strl = cons(literal!("."), strl.clone());
                }
            }
        }
        if format.scalarizeMode.clone() == BaseModelica::ScalarizeMode::PARTIALLY_SCALARIZED.clone() {
            subs = getSubscripts(cref.clone());
        } else {
            subs = metamodelica::nil();
        }
    }
    strl = cons(literal!("'"), strl.clone());
    if !(subs.clone().is_empty()) {
        strl = cons(Subscript::toFlatStringList(subs.clone(), format.clone(), format.scalarizeMode.clone() == BaseModelica::ScalarizeMode::SCALARIZED.clone())?, strl.clone());
    }
    r#str = stringAppendList(strl.clone().reverse());
    Ok(r#str)
}

pub fn listToString(mut crs: Arc<metamodelica::List<Arc<NFComponentRef>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(crs.clone(), Arc::new(toString)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    r#str
}

pub fn toJSON(mut cref: Arc<NFComponentRef>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), JSON::makeString((literal!("cref")).clone()), json.clone())?;
            json = JSON::addPair((literal!("parts")).clone(), JSON::makeList(toJSON_impl(cref.clone(), metamodelica::nil())?), json.clone())?;
            json.clone()
        },
        Deref @ EMPTY { .. } => JSON::makeNull(),
        Deref @ WILD { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), JSON::makeString((literal!("cref")).clone()), json.clone())?;
            json = JSON::addPair((literal!("parts")).clone(), JSON::makeList(list![JSON::fromPair((literal!("name")).clone(), JSON::makeString((literal!("_")).clone()))?]), json.clone())?;
            json.clone()
        },
        _ => JSON::makeString((toString(cref.clone())?).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toJSON_impl(mut cref: Arc<NFComponentRef>, mut accum: Arc<metamodelica::List<Arc<JSON::JSON>>>) -> Result<Arc<metamodelica::List<Arc<JSON::JSON>>>> {
    let mut objs: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
    let mut obj: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut subs: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    objs = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            obj = JSON::emptyListObject();
            obj = JSON::addPair((literal!("name")).clone(), JSON::makeString((InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?).clone()), obj.clone())?;
            if !(var_field!((*cref).subscripts, NFComponentRef::CREF).clone().is_empty()) {
                obj = JSON::addPair((literal!("subscripts")).clone(), Subscript::toJSONList(var_field!((*cref).subscripts, NFComponentRef::CREF).clone())?, obj.clone())?;
            }
            if (isEmpty(var_field!((*cref).restCref, NFComponentRef::CREF).clone())) {toJSON_context(var_field!((*cref).node, NFComponentRef::CREF).clone(), cons(obj.clone(), accum.clone()))?} else {toJSON_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), cons(obj.clone(), accum.clone()))?}
        },
        _ => accum.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(objs)
}

pub fn toJSON_context(mut node: Arc<InstNode::InstNode>, mut accum: Arc<metamodelica::List<Arc<JSON::JSON>>>) -> Result<Arc<metamodelica::List<Arc<JSON::JSON>>>> {
    let mut accum: Arc<metamodelica::List<Arc<JSON::JSON>>> = accum;
    let mut opt_context: Option<Arc<Absyn::Path>> = None;
    opt_context = InstNode::rootClassContext(InstNode::instanceParent(node.clone()));
    if isSome(opt_context.clone()) {
        for mut name in &*AbsynUtil::pathToStringListReverse(Util::getOption(opt_context.clone())?, metamodelica::nil())? {
            let mut name = name.clone();
            accum = cons(JSON::addPair((literal!("name")).clone(), JSON::makeString((name.clone()).clone()), JSON::emptyListObject())?, accum.clone());
        }
    }
    Ok(accum)
}

pub fn hash(mut cref: Arc<NFComponentRef>) -> i32 {
    let mut hash: i32 = hashContinue(cref.clone(), false, Util::HASH_SEED.clone()).unwrap();
    hash
}

pub fn hashStrip(mut cref: Arc<NFComponentRef>) -> i32 {
    let mut hash: i32 = hashContinue(cref.clone(), true, Util::HASH_SEED.clone()).unwrap();
    hash
}

pub fn hashContinue(mut cref: Arc<NFComponentRef>, mut strip: bool, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            hash = stringHashDjb2Continue((InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?).clone(), hash.clone());
            if !(strip.clone()) {
                for mut s in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                    let mut s = s.clone();
                    hash = stringHashDjb2Continue((Subscript::toString(s.clone())?).clone(), hash.clone());
                }
            }
            hashContinue(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), strip.clone(), hash.clone())?
        },
        Deref @ WILD { .. } => stringHashDjb2Continue((literal!("_")).clone(), hash.clone()),
        _ => hash.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hash)
}

pub fn toPath(mut cref: Arc<NFComponentRef>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    path = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => toPath_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), Arc::new(Absyn::Path::IDENT { name: (InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?).clone() }))?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(path)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toPath_impl(mut cref: Arc<NFComponentRef>, mut accumPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    path = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => toPath_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), Arc::new(Absyn::Path::QUALIFIED { name: (InstNode::name(var_field!((*cref).node, NFComponentRef::CREF).clone())?).clone(), path: accumPath.clone() }))?,
        _ => accumPath.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(path)
}

pub fn fromNodeList(mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = Arc::new(crate::NFComponentRef::EMPTY);
    for mut n in &*nodes.clone() {
        let mut n = n.clone();
        cref = Arc::new(NFComponentRef::CREF { node: n.clone(), subscripts: metamodelica::nil(), ty: InstNode::getType(n.clone())?, origin: Origin::SCOPE.clone(), restCref: cref.clone() });
    }
    Ok(cref)
}

pub fn scalarize(mut cref: Arc<NFComponentRef>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<NFComponentRef>>> = metamodelica::nil();
    crefs = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { ty: Deref @ Type::ARRAY { .. }, .. } => {
            let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
            dims = Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone());
            subs = Subscript::scalarizeList(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), dims.clone(), resize.clone())?;
            subs = List::combination(subs.clone());
            {
        let mut __acc: Arc<metamodelica::List<Arc<NFComponentRef>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = setSubscripts(s.clone(), cref.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        _ => {
            list![cref.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

pub fn scalarizeAll(mut cref: Arc<NFComponentRef>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<NFComponentRef>>> = metamodelica::nil();
    let mut next: Arc<NFComponentRef> = cref.clone();
    let mut nested_crefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFComponentRef>>>>> = metamodelica::nil();
    while !(isEmpty(next.clone())) {
        nested_crefs = cons(scalarize(next.clone(), resize.clone())?, nested_crefs.clone());
        let __pa0 = ::match_deref::match_deref! { match &(next.clone()) {
            Deref @ CREF { restCref: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        next = __pa0.clone();
    }
    crefs = scalarizeAll_Nesting(nested_crefs.clone(), Arc::new(crate::NFComponentRef::EMPTY), metamodelica::nil())?;
    Ok(crefs)
}

pub fn scalarizeAll_Nesting(mut nested_crefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFComponentRef>>>>>, mut cref: Arc<NFComponentRef>, mut crefs: Arc<metamodelica::List<Arc<NFComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<NFComponentRef>>> = crefs;
    crefs = (::match_deref::match_deref! { match &(nested_crefs.clone()) {
        Deref @ metamodelica::List::Cons { head: head, tail: tail } => {
            let mut empty: bool = false;
            empty = tail.clone().is_empty();
            for mut head_cref in &*head.clone() {
                let mut head_cref = head_cref.clone();
                crefs = (::match_deref::match_deref! { match &(head_cref.clone()) {
        Deref @ CREF { .. } => {
            assign_variant_field!(head_cref => NFComponentRef::CREF; restCref = cref.clone());
            if empty.clone() {
                crefs = cons(head_cref.clone(), crefs.clone());
            } else {
                crefs = scalarizeAll_Nesting(tail.clone(), head_cref.clone(), crefs.clone())?;
            }
            crefs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
            }
            crefs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(crefs)
}

pub fn scalarizeSlice(mut cref: Arc<NFComponentRef>, mut slice: Arc<metamodelica::List<i32>>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<NFComponentRef>>> = metamodelica::nil();
    let mut next: Arc<NFComponentRef> = cref.clone();
    let mut nested_crefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFComponentRef>>>>> = metamodelica::nil();
    while !(isEmpty(next.clone())) {
        nested_crefs = cons(scalarize(next.clone(), resize.clone())?, nested_crefs.clone());
        let __pa0 = ::match_deref::match_deref! { match &(next.clone()) {
            Deref @ CREF { restCref: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        next = __pa0.clone();
    }
    crefs = scalarizeAll_Nesting(nested_crefs.clone(), Arc::new(crate::NFComponentRef::EMPTY), metamodelica::nil())?;
    if !(slice.clone().is_empty()) {
        crefs = List::getAtIndexLst(crefs.clone(), slice.clone(), true);
    }
    Ok(crefs)
}

pub fn isPackageConstant(mut cref: Arc<NFComponentRef>) -> Result<bool> {
    let mut isPkgConst: bool = false;
    isPkgConst = nodeVariability(cref.clone())? <= Variability::PARAMETER.clone() && isPackageConstant2(cref.clone());
    Ok(isPkgConst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isPackageConstant2(mut cref: Arc<NFComponentRef>) -> bool {
    let mut isPkgConst: bool = false;
    isPkgConst = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { node: Deref @ InstNode::CLASS_NODE { .. }, .. } => InstNode::isUserdefinedClass(var_field!((*cref).node, NFComponentRef::CREF).clone()),
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => isPackageConstant2(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isPkgConst
}

pub fn stripSubscripts(mut cref: Arc<NFComponentRef>) -> (Arc<NFComponentRef>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) {
    let mut strippedCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    (strippedCref, subs) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => (Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: metamodelica::nil(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: var_field!((*cref).restCref, NFComponentRef::CREF).clone() }), var_field!((*cref).subscripts, NFComponentRef::CREF).clone()),
        _ => (cref.clone(), metamodelica::nil()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (strippedCref, subs)
}

pub fn stripSubscriptsAll(mut cref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut strippedCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    strippedCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: metamodelica::nil(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: stripSubscriptsAll(var_field!((*cref).restCref, NFComponentRef::CREF).clone()) }),
        _ => cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    strippedCref
}

pub fn stripSubscriptsExceptModel(mut cref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { restCref, node, .. } if (InstNode::isModel(node.clone())) => {
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: stripSubscriptsExceptModel(restCref.clone()) })
        },
        Deref @ CREF { restCref, .. } => {
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: metamodelica::nil(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: stripSubscriptsExceptModel(restCref.clone()) })
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn stripIteratorSubscripts(mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            if !(var_field!((*cref).subscripts, NFComponentRef::CREF).clone().is_empty()) && Subscript::isIterator(List::last(var_field!((*cref).subscripts, NFComponentRef::CREF).clone())?) {
                subs = var_field!((*cref).subscripts, NFComponentRef::CREF).clone().reverse();
                subs = List::trim(subs.clone(), Arc::new(fnptr!(Subscript::isIterator, Arc<Subscript::NFSubscript>)))?;
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = subs.clone().reverse());
            }
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = stripIteratorSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn simplifySubscripts(mut cref: Arc<NFComponentRef>, mut trim: bool) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut rest_cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let mut dirty: bool = false;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { subscripts: subs, .. } => {
            let mut subs = (*subs).clone();
            if !(subs.clone().is_empty()) {
                subs = Subscript::simplifyList(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone()), trim.clone())?;
                dirty = true;
            }
            rest_cref = simplifySubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), trim.clone())?;
            dirty = dirty.clone() || !(referenceEq(&rest_cref.clone(),&var_field!((*cref).restCref, NFComponentRef::CREF).clone()));
            if (dirty.clone()) {Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest_cref.clone() })} else {cref.clone()}
        },
        _ => cref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn evaluateSubscripts(mut cref: Arc<NFComponentRef>) -> Result<Arc<NFComponentRef>> {
    let mut cref: Arc<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, subscripts: Deref @ metamodelica::List::Nil, .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = evaluateSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?);
            cref.clone()
        },
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = Subscript::eval(s.clone(), NFCeval::noTarget().clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: evaluateSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone())? })
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn isDeleted(mut cref: Arc<NFComponentRef>) -> Result<bool> {
    let mut isDeleted: bool = false;
    isDeleted = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, node, .. } => {
            InstNode::isComponent(node.clone()) && Component::isDeleted(InstNode::component(node.clone())?)? || self::isDeleted(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isDeleted)
}

pub fn isFromCref(mut cref: Arc<NFComponentRef>) -> bool {
    let mut fromCref: bool = false;
    fromCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => true,
        Deref @ WILD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    fromCref
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toListReverse(mut cref: Arc<NFComponentRef>, mut includeScope: bool, mut accum: Arc<metamodelica::List<Arc<NFComponentRef>>>) -> Arc<metamodelica::List<Arc<NFComponentRef>>> {
    let mut crefs: Arc<metamodelica::List<Arc<NFComponentRef>>> = metamodelica::nil();
    crefs = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } if (includeScope.clone()) => toListReverse(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), includeScope.clone(), cons(cref.clone(), accum.clone())),
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => toListReverse(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), includeScope.clone(), cons(cref.clone(), accum.clone())),
        _ => accum.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    crefs
}

pub fn depth(mut cref: Arc<NFComponentRef>) -> i32 {
    let mut d: i32 = 0;
    d = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { restCref: Deref @ EMPTY { .. }, .. } => d.clone() + 1,
        Deref @ CREF { .. } => {
            d = 1 + depth(var_field!((*cref).restCref, NFComponentRef::CREF).clone());
            d.clone()
        },
        Deref @ WILD { .. } => 0,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    d
}

pub fn size(mut cref: Arc<NFComponentRef>, mut withComplex: bool, mut resize: bool) -> i32 {
    let mut s: i32 = {
        let mut __acc: i32 = 1;
        for mut i in (sizes(cref.clone(), withComplex.clone(), resize.clone(), metamodelica::nil()).unwrap()).into_iter().cloned() {
            let __x = i.clone();
            __acc *= __x;
        }
        __acc
    };
    s
}

pub fn sizes(mut cref: Arc<NFComponentRef>, mut withComplex: bool, mut resize: bool, mut s_lst: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut s_lst: Arc<metamodelica::List<i32>> = s_lst;
    s_lst = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ EMPTY { .. } => {
            let mut local_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            s_lst.clone().reverse()
        },
        Deref @ CREF { .. } => {
            let mut local_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            local_lst = sizes_local(cref.clone(), withComplex.clone(), resize.clone())?;
            s_lst = listAppend(local_lst.clone(), s_lst.clone());
            sizes(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), withComplex.clone(), resize.clone(), s_lst.clone())?
        },
        Deref @ WILD { .. } => {
            let mut local_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            list![0]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(s_lst)
}

pub fn sizes_local(mut cref: Arc<NFComponentRef>, mut withComplex: bool, mut resize: bool) -> Result<Arc<metamodelica::List<i32>>> {
    let mut s_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut complex_size: Option<i32> = None;
    s_lst = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            complex_size = Type::complexSize(var_field!((*cref).ty, NFComponentRef::CREF).clone(), false)?;
            s_lst = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dim in (Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone())).into_iter().cloned() {
            let __x = Dimension::size(dim.clone(), resize.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if withComplex.clone() && isSome(complex_size.clone()) {
                s_lst = cons(Util::getOption(complex_size.clone())?, s_lst.clone());
            }
            s_lst = if (s_lst.clone().is_empty()) {list![1]} else {s_lst.clone()};
            s_lst.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s_lst)
}

pub fn sizes_local_exp(mut cref: Arc<NFComponentRef>, mut withComplex: bool) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut s_lst: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut complex_size: Option<i32> = None;
    s_lst = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            complex_size = Type::complexSize(var_field!((*cref).ty, NFComponentRef::CREF).clone(), false)?;
            s_lst = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut dim in (Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone())).into_iter().cloned() {
            let __x = Dimension::sizeExp(dim.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if withComplex.clone() && isSome(complex_size.clone()) {
                s_lst = cons(Arc::new(Expression::NFExpression::INTEGER { value: Util::getOption(complex_size.clone())? }), s_lst.clone());
            }
            s_lst = if (s_lst.clone().is_empty()) {list![Arc::new(Expression::NFExpression::INTEGER { value: 1 })]} else {s_lst.clone()};
            s_lst.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s_lst)
}

pub fn sizeKnown(mut cref: Arc<NFComponentRef>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => Type::sizeKnown(var_field!((*cref).ty, NFComponentRef::CREF).clone()),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn subscriptsToInteger(mut cref: Arc<NFComponentRef>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut s_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut subs_tmp in &*subscriptsAllReverse(cref.clone(), metamodelica::nil()) {
        let mut subs_tmp = subs_tmp.clone();
        if subs_tmp.clone().is_empty() {
            s_lst = cons(1, s_lst.clone());
        } else {
            for mut sub in &*subs_tmp.clone() {
                let mut sub = sub.clone();
                s_lst = cons(Expression::integerValueOrDefault(Subscript::toExp(sub.clone())?, 1), s_lst.clone());
            }
        }
    }
    Ok(s_lst)
}

pub fn subscriptsToExpression(mut cref: Arc<NFComponentRef>, mut addScalar: bool) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut e_lst: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    for mut subs_tmp in &*subscriptsAllReverse(cref.clone(), metamodelica::nil()) {
        let mut subs_tmp = subs_tmp.clone();
        if addScalar.clone() && subs_tmp.clone().is_empty() {
            e_lst = cons(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), e_lst.clone());
        } else {
            for mut sub in &*subs_tmp.clone() {
                let mut sub = sub.clone();
                e_lst = cons(Subscript::toExp(sub.clone())?, e_lst.clone());
            }
        }
    }
    Ok(e_lst)
}

pub fn isEmptyArray(mut cref: Arc<NFComponentRef>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => Type::isEmptyArray(var_field!((*cref).ty, NFComponentRef::CREF).clone()) || isEmptyArray(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn isComplexArray(mut cref: Arc<NFComponentRef>) -> Result<bool> {
    let mut complexArray: bool = false;
    complexArray = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => isComplexArray2(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(complexArray)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isComplexArray2(mut cref: Arc<NFComponentRef>) -> Result<bool> {
    let mut complexArray: bool = false;
    complexArray = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { ty: Deref @ Type::ARRAY { .. }, .. } if (Type::isArray(Type::subscript(var_field!((*cref).ty, NFComponentRef::CREF).clone(), var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), true)?)) => true,
        Deref @ CREF { .. } => isComplexArray2(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(complexArray)
}

pub fn containsExp(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<Expression::NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => Subscript::listContainsExp(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), func.clone())? || containsExp(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn containsExpShallow(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> bool {
    pub type ContainsPred = fn(Arc<Expression::NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => Subscript::listContainsExpShallow(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), func.clone()) || containsExpShallow(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn applyExp(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<Expression::NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            for mut s in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                let mut s = s.clone();
                Subscript::applyExp(s.clone(), func.clone())?;
            }
            applyExp(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn applyExpShallow(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> () {
    pub type ApplyFunc = fn(Arc<Expression::NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            for mut s in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                let mut s = s.clone();
                Subscript::applyExpShallow(s.clone(), func.clone());
            }
            applyExpShallow(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn mapExp(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFComponentRef>> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    outCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            let mut rest: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = Subscript::mapExp(s.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            rest = mapExp(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone())?;
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest.clone() })
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

pub fn mapExpShallow(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFComponentRef> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    outCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            let mut rest: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = Subscript::mapShallowExp(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            rest = mapExpShallow(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone());
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest.clone() })
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub fn foldExp<ArgT: Clone + 'static>(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> ArgT {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut arg: ArgT = arg;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            arg = List::fold(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Subscript::foldExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
            arg = foldExp(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone(), arg.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    arg
}

pub fn mapFoldExp<ArgT: Clone + 'static>(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<NFComponentRef>, ArgT) {
    pub type MapFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)>;

    let mut outCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let mut arg: ArgT = arg;
    outCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            let mut rest: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            (subs, arg) = List::map1Fold(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), Arc::new(Subscript::mapFoldExp), func.clone(), arg.clone());
            (rest, arg) = mapFoldExp(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone(), arg.clone());
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest.clone() })
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outCref, arg)
}

pub fn mapFoldExpShallow<ArgT: Clone + 'static>(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<NFComponentRef>, ArgT) {
    pub type MapFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)>;

    let mut outCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    let mut arg: ArgT = arg;
    outCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            let mut rest: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            (subs, arg) = List::map1Fold(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), Arc::new(fnptr!(Subscript::mapFoldExpShallow, Arc<Subscript::NFSubscript>, _, _)), func.clone(), arg.clone());
            (rest, arg) = mapFoldExpShallow(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone(), arg.clone());
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest.clone() })
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outCref, arg)
}

pub fn isTime(mut cref: Arc<NFComponentRef>) -> bool {
    let mut b: bool = firstName(cref.clone(), false).unwrap() == literal!("time");
    b
}

pub fn isSubstitute(mut cref: Arc<NFComponentRef>) -> bool {
    let mut b: bool = firstName(cref.clone(), false).unwrap() == literal!("$SUBST_CREF");
    b
}

pub fn isDiscrete(mut cref: Arc<NFComponentRef>) -> bool {
    let mut result: bool = Type::isDiscrete(nodeType(cref.clone()).unwrap());
    result
}

pub fn removeOuterCrefPrefix(mut cref: Arc<NFComponentRef>) -> Arc<NFComponentRef> {
    let mut cref: Arc<NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            if InstNode::isGeneratedInner(var_field!((*cref).node, NFComponentRef::CREF).clone()) {
                assign_variant_field!(cref => NFComponentRef::CREF; restCref = Arc::new(crate::NFComponentRef::EMPTY));
            } else {
                assign_variant_field!(cref => NFComponentRef::CREF; restCref = removeOuterCrefPrefix(var_field!((*cref).restCref, NFComponentRef::CREF).clone()));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn mapTypes(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>) -> Arc<NFComponentRef> {
    pub type MapFunc = fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>>;

    let mut outCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    outCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut rest: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            ty = func(var_field!((*cref).ty, NFComponentRef::CREF).clone()).unwrap();
            rest = mapTypes(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone());
            Arc::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), ty: ty.clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest.clone() })
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub fn mapNodes(mut cref: Arc<NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> + 'static>) -> Arc<NFComponentRef> {
    pub type MapFunc = fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>>;

    let mut outCref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    outCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            let mut rest: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            node = func(var_field!((*cref).node, NFComponentRef::CREF).clone()).unwrap();
            rest = mapNodes(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func.clone());
            Arc::new(NFComponentRef::CREF { node: node.clone(), subscripts: var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest.clone() })
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub fn getArrayCrefOpt(mut scal: Arc<NFComponentRef>) -> Result<Option<Arc<NFComponentRef>>> {
    let mut arr: Option<Arc<NFComponentRef>> = None;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    if Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())? {
        subs = subscriptsAllFlat(scal.clone());
        if subs.clone().is_empty() {
            arr = None;
        } else if List::all(subs.clone(), Arc::new(fnptr!(Subscript::isFirst, Arc<Subscript::NFSubscript>))) {
            arr = Some(stripSubscriptsAll(scal.clone()));
        } else {
            arr = None;
        }
    } else {
        arr = if (Type::isArray(getSubscriptedType(scal.clone(), false)?)) {Some(scal.clone())} else {None};
    }
    Ok(arr)
}

pub fn isSliced(mut cref: Arc<NFComponentRef>) -> bool {
    fn is_sliced_impl(mut cref: Arc<NFComponentRef>) -> bool {
        let mut sliced: bool = false;
        sliced = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => {
            sliced = Type::dimensionCount(var_field!((*cref).ty, NFComponentRef::CREF).clone()) > (var_field!((*cref).subscripts, NFComponentRef::CREF).clone().len() as i32) || List::any(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), Arc::new(fnptr!(Subscript::isSliced, Arc<Subscript::NFSubscript>)));
            sliced.clone() || is_sliced_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        sliced
    }

    let mut sliced: bool = false;
    sliced = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => is_sliced_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sliced
}

pub fn hasImplicitTrailingIndex(mut cref: Arc<NFComponentRef>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => !(var_field!((*cref).subscripts, NFComponentRef::CREF).clone().is_empty()) && (var_field!((*cref).subscripts, NFComponentRef::CREF).clone().len() as i32) < Type::dimensionCount(var_field!((*cref).ty, NFComponentRef::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn iterate(mut cref: Arc<NFComponentRef>) -> Result<(Arc<NFComponentRef>, Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>)> {
    fn iterate_impl(mut cref: Arc<NFComponentRef>, mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>) -> Result<(Arc<NFComponentRef>, Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>)> {
        let mut cref: Arc<NFComponentRef> = cref;
        let mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = iterators;
        let mut rest_cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
        let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
        let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut dim_count: i32 = 0;
        let mut sub_count: i32 = 0;
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        let mut isubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        let mut dim_index: i32 = 0;
        let mut iterator: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { origin: Origin::CREF { .. }, .. } => {
            dims = Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone()).reverse();
            dim_count = (dims.clone().len() as i32);
            sub_count = (var_field!((*cref).subscripts, NFComponentRef::CREF).clone().len() as i32);
            subs = List::consN(dim_count.clone() - sub_count.clone(), Arc::new(crate::NFSubscript::WHOLE), var_field!((*cref).subscripts, NFComponentRef::CREF).clone());
            isubs = metamodelica::nil();
            dim_index = dim_count.clone();
            for mut s in &*subs.clone().reverse() {
                let mut s = s.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                dim = __pa0.clone();
                dims = __pa1.clone();
                if !(Subscript::isIndex(s.clone())) {
                    range = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Subscript::SLICE { .. } => var_field!((*s).slice, Subscript::NFSubscript::SLICE).clone(),
        Deref @ Subscript::WHOLE => Expression::makeRange(Dimension::lowerBoundExp(dim.clone())?, None, Dimension::endExp(dim.clone(), Arc::new(Expression::NFExpression::CREF { ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), cref: cref.clone() }), dim_index.clone())?)?,
        _ => bail!("match: no arm matched"),
    } });
                    iterator = InstNode::newUniqueIterator(Absyn::dummyInfo.clone(), Arc::new(crate::NFType::INTEGER));
                    iterators = cons((iterator.clone(), range.clone()), iterators.clone());
                    dim_index = dim_index.clone() - 1;
                    s = Arc::new(Subscript::NFSubscript::INDEX { index: Expression::fromCref(makeIterator(iterator.clone(), Arc::new(crate::NFType::INTEGER)), false)? });
                }
                isubs = cons(s.clone(), isubs.clone());
            }
            assign_variant_field!(cref => NFComponentRef::CREF; subscripts = isubs.clone());
            (rest_cref, iterators) = iterate_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), iterators.clone())?;
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = rest_cref.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((cref, iterators))
    }

    let mut cref: Arc<NFComponentRef> = cref;
    let mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut rest_cref: Arc<NFComponentRef> = Arc::new(NFComponentRef::EMPTY);
    iterators = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => {
            (rest_cref, iterators) = iterate_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), metamodelica::nil())?;
            if !(iterators.clone().is_empty()) {
                assign_variant_field!(cref => NFComponentRef::CREF; restCref = rest_cref.clone());
                iterators = iterators.clone().reverse();
            }
            iterators.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, iterators))
}

pub fn getRecordChildren(mut cref: Arc<NFComponentRef>) -> Result<Arc<metamodelica::List<Arc<NFComponentRef>>>> {
    let mut children: Arc<metamodelica::List<Arc<NFComponentRef>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Type::arrayElementType(getComponentType(cref.clone()));
    let mut children_nodes: metamodelica::Array<Arc<InstNode::InstNode>> = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    if Type::isComplex(ty.clone()) {
        children_nodes = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { .. } => ClassTree::getComponents(Class::classTree(InstNode::getClass(Component::classInstance(InstNode::component(var_field!((*cref).node, NFComponentRef::CREF).clone())?))?)?)?,
        _ => metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if !(children_nodes.clone().borrow().is_empty()) {
        children = {
        let mut __acc: Arc<metamodelica::List<Arc<NFComponentRef>>> = metamodelica::nil();
        for mut node in (children_nodes.clone()).borrow().iter() {
            let __x = prefixCref(node.clone(), InstNode::getType(node.clone())?, metamodelica::nil(), cref.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    Ok(children)
}


