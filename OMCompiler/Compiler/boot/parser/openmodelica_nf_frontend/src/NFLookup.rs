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
use crate::NFBuiltin;
use crate::NFBuiltinFuncs;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFLookupState::LookupState;
use crate::NFModifier as Modifier;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::BackendInterface;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ErrorTypes;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum MatchType {
    FOUND = 1,
    NOT_FOUND = 2,
    PARTIAL = 3,
}
impl PartialOrd for MatchType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for MatchType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn lookupClassName(mut name: Arc<Absyn::Path>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo, mut checkAccessViolations: bool) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    (node, state) = lookupNameWithError(name.clone(), scope.clone(), context.clone(), info.clone(), Error::LOOKUP_ERROR.clone(), checkAccessViolations.clone())?;
    LookupState::assertClass(state.clone(), node.clone(), name.clone(), context.clone(), info.clone())?;
    Ok(node)
}

pub fn lookupBaseClassName(mut name: Arc<Absyn::Path>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> {
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    if let Ok((__pa0, __pa1)) = lookupNames(name.clone(), scope.clone(), context.clone()) {
        nodes = __pa0.clone();
        state = __pa1.clone();
    } else {
        Error::addSourceMessage(Error::LOOKUP_BASECLASS_ERROR.clone(), list![(AbsynUtil::pathString(name.clone(), (literal!(".")).clone(), true, false)?).clone(), (InstNode::scopeName(scope.clone())).clone()], info.clone())?;
        bail!("fail");
    }
    LookupState::assertClass(state.clone(), listHead(nodes.clone())?, name.clone(), context.clone(), info.clone())?;
    Ok(nodes)
}

pub fn lookupComponent(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    match '__try0: {
        (foundCref, foundScope, state) = unwrap_break_err!(lookupCref(cref.clone(), scope.clone(), context.clone()), '__try0);
        node = unwrap_break_err!(ComponentRef::node(foundCref.clone()), '__try0);
        let false = (InstNode::isName(node.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        Ok::<_, anyhow::Error>((foundCref.clone(), foundScope.clone(), node.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            foundCref = __try0_o0;
            foundScope = __try0_o1;
            node = __try0_o2;
            state = __try0_o3;
        }
        Err(_) => {
            Error::addSourceMessageAndFail(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(Dump::printComponentRefStr(cref.clone())?).clone(), (InstNode::scopeName(scope.clone())).clone()], info.clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    state = fixTypenameState(node.clone(), state.clone(), context.clone())?;
    LookupState::assertComponent(state.clone(), node.clone(), cref.clone(), context.clone(), info.clone())?;
    Ok((foundCref, foundScope))
}

pub fn lookupConnector(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    match '__try0: {
        (foundCref, foundScope, state) = unwrap_break_err!(lookupCref(cref.clone(), scope.clone(), context.clone()), '__try0);
        Ok::<_, anyhow::Error>((foundCref.clone(), foundScope.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            foundCref = __try0_o0;
            foundScope = __try0_o1;
            state = __try0_o2;
        }
        Err(_) => {
            Error::addSourceMessageAndFail(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(Dump::printComponentRefStr(cref.clone())?).clone(), (InstNode::scopeName(scope.clone())).clone()], info.clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    node = ComponentRef::node(foundCref.clone())?;
    state = fixTypenameState(node.clone(), state.clone(), context.clone())?;
    LookupState::assertComponent(state.clone(), node.clone(), cref.clone(), context.clone(), info.clone())?;
    Ok((foundCref, foundScope))
}

pub fn fixTypenameState(mut component: Arc<InstNode::InstNode>, mut state: Arc<LookupState::LookupState>, mut context: i32) -> Result<Arc<LookupState::LookupState>> {
    let mut state: Arc<LookupState::LookupState> = state;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if InstNode::isClass(component.clone()) {
        ty = InstNode::getType(Inst::expand(component.clone(), context.clone())?)?;
        state = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ENUMERATION { .. } => Arc::new(crate::NFLookupState::LookupState::COMP),
        Deref @ Type::BOOLEAN => Arc::new(crate::NFLookupState::LookupState::COMP),
        _ => state.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(state)
}

pub fn lookupLocalComponent(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    (foundCref, foundScope, state) = lookupLocalCref(cref.clone(), scope.clone(), context.clone(), info.clone())?;
    LookupState::assertComponent(state.clone(), ComponentRef::node(foundCref.clone())?, cref.clone(), context.clone(), info.clone())?;
    Ok((foundCref, foundScope))
}

pub fn lookupFunctionName(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    match '__try0: {
        (foundCref, foundScope, state) = unwrap_break_err!(lookupCref(cref.clone(), scope.clone(), context.clone()), '__try0);
        node = unwrap_break_err!(ComponentRef::node(foundCref.clone()), '__try0);
        let false = (InstNode::isName(node.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        Ok::<_, anyhow::Error>((foundCref.clone(), foundScope.clone(), node.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            foundCref = __try0_o0;
            foundScope = __try0_o1;
            node = __try0_o2;
            state = __try0_o3;
        }
        Err(_) => {
            Error::addSourceMessageAndFail(Error::LOOKUP_FUNCTION_ERROR.clone(), list![(Dump::printComponentRefStr(cref.clone())?).clone(), (InstNode::scopeName(scope.clone())).clone()], info.clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    (foundCref, state) = fixExternalObjectCall(node.clone(), foundCref.clone(), state.clone())?;
    LookupState::assertFunction(state.clone(), node.clone(), cref.clone(), context.clone(), info.clone())?;
    Ok((foundCref, foundScope))
}

pub fn lookupFunctionNameSilent(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    (foundCref, foundScope, state) = lookupCref(cref.clone(), scope.clone(), context.clone())?;
    node = ComponentRef::node(foundCref.clone())?;
    (foundCref, state) = fixExternalObjectCall(node.clone(), foundCref.clone(), state.clone())?;
    let true = (LookupState::isFunction(state.clone(), node.clone())?) else { bail!("pattern mismatch") };
    Ok((foundCref, foundScope))
}

pub fn fixExternalObjectCall(mut node: Arc<InstNode::InstNode>, mut cref: Arc<ComponentRef::NFComponentRef>, mut state: Arc<LookupState::LookupState>) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<LookupState::LookupState>)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut state: Arc<LookupState::LookupState> = state;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut constructor: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if !(LookupState::isClass(state.clone())) {
        return Ok((cref, state));
    }
    Inst::expand(node.clone(), InstContext::NO_CONTEXT.clone())?;
    cls = InstNode::getClass(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::PARTIAL_BUILTIN { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { constructor, .. }, .. }, .. } => {
            cref = ComponentRef::prefixCref(constructor.clone(), Arc::new(crate::NFType::UNKNOWN), metamodelica::nil(), cref.clone());
            state = Arc::new(crate::NFLookupState::LookupState::FUNC);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, state))
}

pub fn lookupImport(mut name: Arc<Absyn::Path>, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<InstNode::InstNode>> {
    let mut element: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    (element, state) = lookupNameWithError(name.clone(), InstNode::topScope(scope.clone()), InstContext::NO_CONTEXT.clone(), info.clone(), Error::LOOKUP_IMPORT_ERROR.clone(), true)?;
    LookupState::assertImport(state.clone(), element.clone(), name.clone(), info.clone())?;
    Ok(element)
}

pub fn lookupCrefWithError(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo, mut errMsg: ErrorTypes::Message) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, Arc<LookupState::LookupState>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    if let Ok((__pa0, __pa1, __pa2)) = lookupCref(cref.clone(), scope.clone(), context.clone()) {
        foundCref = __pa0.clone();
        foundScope = __pa1.clone();
        state = __pa2.clone();
    } else {
        Error::addSourceMessage(errMsg.clone(), list![(Dump::printComponentRefStr(cref.clone())?).clone(), (InstNode::scopeName(scope.clone())).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((foundCref, foundScope, state))
}

pub fn lookupCref(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, Arc<LookupState::LookupState>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut in_enclosing: bool = false;
    (foundCref, foundScope, state) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            (_, foundCref, foundScope, in_enclosing, state) = lookupSimpleCref((var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), scope.clone(), context.clone())?;
            state = LookupState::checkCrefVariability(foundCref.clone(), in_enclosing.clone(), context.clone(), state.clone())?;
            (foundCref.clone(), foundScope.clone(), state.clone())
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            (node, foundCref, foundScope, in_enclosing, state) = lookupSimpleCref((var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), scope.clone(), context.clone())?;
            (foundCref, foundScope, state) = lookupCrefInNode(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), node.clone(), foundCref.clone(), foundScope.clone(), state.clone(), context.clone())?;
            state = LookupState::checkCrefVariability(foundCref.clone(), in_enclosing.clone(), context.clone(), state.clone())?;
            (foundCref.clone(), foundScope.clone(), state.clone())
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => lookupCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), InstNode::topScope(scope.clone()), context.clone())?,
        Deref @ Absyn::ComponentRef::WILD { .. } => (Arc::new(crate::NFComponentRef::WILD), scope.clone(), Arc::new(crate::NFLookupState::LookupState::PREDEF_COMP)),
        Deref @ Absyn::ComponentRef::ALLWILD { .. } => (Arc::new(crate::NFComponentRef::WILD), scope.clone(), Arc::new(crate::NFLookupState::LookupState::PREDEF_COMP)),
        _ => bail!("match: no arm matched"),
    } });
    Ok((foundCref, foundScope, state))
}

pub fn lookupLocalCref(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, Arc<LookupState::LookupState>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut match_ty: MatchType = MatchType::FOUND;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    (foundCref, foundScope, state) = 'mc: {
        let __mc_input = cref.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
                    let mut state: Arc<LookupState::LookupState> = state.clone();
                    let mut node: Arc<InstNode::InstNode> = node.clone();
                    let mut foundScope: Arc<InstNode::InstNode> = foundScope.clone();
                    (node, foundScope) = lookupLocalSimpleCref((var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), scope.clone())?;
                    state = LookupState::nodeState(node.clone())?;
                    Ok((ComponentRef::fromAbsyn(node.clone(), var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), Arc::new(crate::NFComponentRef::EMPTY)), foundScope.clone(), state.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
                    let mut node: Arc<InstNode::InstNode> = node.clone();
                    let mut foundCref: Arc<ComponentRef::NFComponentRef> = foundCref.clone();
                    let mut foundScope: Arc<InstNode::InstNode> = foundScope.clone();
                    let mut state: Arc<LookupState::LookupState> = state.clone();
                    (node, foundScope) = lookupLocalSimpleCref((var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), scope.clone())?;
                    state = LookupState::nodeState(node.clone())?;
                    foundCref = ComponentRef::fromAbsyn(node.clone(), var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), Arc::new(crate::NFComponentRef::EMPTY));
                    (foundCref, foundScope, state) = lookupCrefInNode(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), node.clone(), foundCref.clone(), foundScope.clone(), state.clone(), context.clone())?;
                    Ok((foundCref.clone(), foundScope.clone(), state.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(Dump::printComponentRefStr(cref.clone())?).clone(), (InstNode::scopeName(scope.clone())).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((foundCref, foundScope, state))
}

pub fn lookupInner(mut outerNode: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
    let mut innerNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut name: ArcStr = InstNode::name(outerNode.clone())?;
    let mut cur_scope: Arc<InstNode::InstNode> = scope.clone();
    let mut prev_scope: Arc<InstNode::InstNode> = scope.clone();
    while !(InstNode::isEmpty(cur_scope.clone())) {
        if '__try0: {
            innerNode = InstNode::resolveOuter((Class::lookupElement((name.clone()).clone(), InstNode::getClass(cur_scope.clone())?)?).0);
            let true = (unwrap_break_err!(InstNode::isInner(innerNode.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            return Ok(innerNode);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            if InstNode::isRootClass(cur_scope.clone()) {
                prev_scope = InstNode::topScope(cur_scope.clone());
                cur_scope = Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE);
            } else {
                prev_scope = cur_scope.clone();
                cur_scope = InstNode::instanceParent(cur_scope.clone());
            }
        }
    }
    innerNode = generateInner(outerNode.clone(), InstNode::topScope(prev_scope.clone()))?;
    Ok(innerNode)
}

pub fn lookupLocalSimpleName(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>) -> Result<(Arc<InstNode::InstNode>, bool)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut isImport: bool = false;
    (node, isImport) = Class::lookupElement((name.clone()).clone(), InstNode::getClass(scope.clone())?)?;
    node = InstNode::resolveInner(node.clone());
    Ok((node, isImport))
}

pub fn lookupSimpleName(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<InstNode::InstNode>, bool)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut selfReference: bool = false;
    let mut cur_scope: Arc<InstNode::InstNode> = scope.clone();
    let mut require_builtin: bool = false;
    let mut loaded: bool = false;
    if InstContext::inAnnotation(context.clone()) {
        if '__try0: {
            (node, _) = unwrap_break_err!(lookupLocalSimpleName((name.clone()).clone(), InstNode::annotationScope(scope.clone())?), '__try0);
            return Ok((node, selfReference));
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    for mut i in 1..=Global::recursionDepthLimit.clone() {
        if '__try1: {
            (node, _) = unwrap_break_err!(lookupLocalSimpleName((name.clone()).clone(), cur_scope.clone()), '__try1);
            if require_builtin.clone() {
                let true = (InstNode::isBuiltin(node.clone())) else { break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            }
            return Ok((node, selfReference));
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            if InstNode::isEncapsulated(cur_scope.clone())? {
                cur_scope = InstNode::topScope(InstNode::parentScope(cur_scope.clone(), false)?);
                require_builtin = true;
            } else if name.clone() == InstNode::name(cur_scope.clone())? && InstNode::isClass(cur_scope.clone()) || name.clone() == InstNode::name(InstNode::classScope(cur_scope.clone()))? {
                node = InstNode::classScope(cur_scope.clone());
                selfReference = true;
                return Ok((node, selfReference));
            } else {
                if InstNode::isTopScope(cur_scope.clone()) && !(loaded.clone()) && !(require_builtin.clone()) {
                    loaded = true;
                    loadLibrary((name.clone()).clone(), cur_scope.clone());
                } else {
                    cur_scope = InstNode::parentScope(cur_scope.clone(), false)?;
                }
            }
        }
    }
    Error::addMessage(Error::RECURSION_DEPTH_REACHED.clone(), list![ArcStr::from(::std::format!("{}", Global::recursionDepthLimit.clone())), (InstNode::name(scope.clone())?).clone()])?;
    bail!("fail");
    Ok((node, selfReference))
}

pub fn lookupSimpleNameRootPath(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cur_scope: Arc<InstNode::InstNode> = scope.clone();
    let mut in_root_class: bool = true;
    if InstContext::inAnnotation(context.clone()) {
        if '__try0: {
            let _ = unwrap_break_err!(lookupLocalSimpleName((name.clone()).clone(), InstNode::annotationScope(scope.clone())?), '__try0);
            path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
            return Ok(path);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    for mut i in 1..=Global::recursionDepthLimit.clone() {
        if '__try1: {
            (node, _) = unwrap_break_err!(Class::lookupElement((name.clone()).clone(), InstNode::getClass(cur_scope.clone())?), '__try1);
            if in_root_class.clone() {
                path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
            } else {
                path = AbsynUtil::makeFullyQualified(InstNode::fullPath(node.clone(), false));
            }
            return Ok(path);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            if InstNode::isEncapsulated(cur_scope.clone())? {
                path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
                return Ok(path);
            } else if name.clone() == InstNode::name(cur_scope.clone())? && InstNode::isClass(cur_scope.clone()) {
                path = if (in_root_class.clone()) {Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() })} else {AbsynUtil::makeFullyQualified(InstNode::fullPath(cur_scope.clone(), false))};
                return Ok(path);
            } else if InstNode::isTopScope(cur_scope.clone()) {
                path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
                return Ok(path);
            } else {
                if in_root_class.clone() && InstNode::isRootClass(cur_scope.clone()) {
                    in_root_class = false;
                }
                cur_scope = InstNode::parentScope(cur_scope.clone(), false)?;
            }
        }
    }
    bail!("fail");
    Ok(path)
}

pub fn lookupNameWithError(mut name: Arc<Absyn::Path>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo, mut errorType: ErrorTypes::Message, mut checkAccessViolations: bool) -> Result<(Arc<InstNode::InstNode>, Arc<LookupState::LookupState>)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    if let Ok((__pa0, __pa1)) = lookupName(name.clone(), scope.clone(), context.clone(), checkAccessViolations.clone()) {
        node = __pa0.clone();
        state = __pa1.clone();
    } else {
        Error::addSourceMessage(errorType.clone(), list![(AbsynUtil::pathString(name.clone(), (literal!(".")).clone(), true, false)?).clone(), (InstNode::scopeName(scope.clone())).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((node, state))
}

pub fn lookupName(mut name: Arc<Absyn::Path>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut checkAccessViolations: bool) -> Result<(Arc<InstNode::InstNode>, Arc<LookupState::LookupState>)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut self_reference: bool = false;
    (node, state) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            (node, state, _) = lookupFirstIdent((var_field!((*name).name, Absyn::Path::IDENT).clone()).clone(), scope.clone(), context.clone())?;
            (node.clone(), state.clone())
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            (node, state, self_reference) = lookupFirstIdent((var_field!((*name).name, Absyn::Path::QUALIFIED).clone()).clone(), scope.clone(), context.clone())?;
            lookupLocalName(var_field!((*name).path, Absyn::Path::QUALIFIED).clone(), node.clone(), state.clone(), context.clone(), checkAccessViolations.clone(), self_reference.clone())?
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => lookupName(var_field!((*name).path, Absyn::Path::FULLYQUALIFIED).clone(), InstNode::topScope(scope.clone()), context.clone(), checkAccessViolations.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok((node, state))
}

pub fn lookupNames(mut name: Arc<Absyn::Path>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<LookupState::LookupState>)> {
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut self_reference: bool = false;
    (nodes, state) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            (node, state, _) = lookupFirstIdent((var_field!((*name).name, Absyn::Path::IDENT).clone()).clone(), scope.clone(), context.clone())?;
            (list![node.clone()], state.clone())
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            (node, state, self_reference) = lookupFirstIdent((var_field!((*name).name, Absyn::Path::QUALIFIED).clone()).clone(), scope.clone(), context.clone())?;
            lookupLocalNames(var_field!((*name).path, Absyn::Path::QUALIFIED).clone(), node.clone(), list![node.clone()], state.clone(), context.clone(), self_reference.clone())?
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
            lookupNames(var_field!((*name).path, Absyn::Path::FULLYQUALIFIED).clone(), InstNode::topScope(scope.clone()), context.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((nodes, state))
}

pub fn lookupFirstIdent(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<InstNode::InstNode>, Arc<LookupState::LookupState>, bool)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut selfReference: bool = false;
    match '__try0: {
        node = unwrap_break_err!(lookupSimpleBuiltinName((name.clone()).clone()), '__try0);
        state = Arc::new(crate::NFLookupState::LookupState::PREDEF_CLASS);
        selfReference = false;
        Ok::<_, anyhow::Error>((node.clone(), selfReference.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            node = __try0_o0;
            selfReference = __try0_o1;
            state = __try0_o2;
        }
        Err(_) => {
            (node, selfReference) = lookupSimpleName((name.clone()).clone(), scope.clone(), context.clone())?;
            state = LookupState::nodeState(node.clone())?;
        }
    }
    Ok((node, state, selfReference))
}

pub fn lookupLocalName(mut name: Arc<Absyn::Path>, mut node: Arc<InstNode::InstNode>, mut state: Arc<LookupState::LookupState>, mut context: i32, mut checkAccessViolations: bool, mut selfReference: bool) -> Result<(Arc<InstNode::InstNode>, Arc<LookupState::LookupState>)> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut state: Arc<LookupState::LookupState> = state;
    let mut is_import: bool = false;
    if !(InstNode::isClass(node.clone())) {
        state = Arc::new(crate::NFLookupState::LookupState::COMP_CLASS);
        return Ok((node, state));
    }
    if !(selfReference.clone()) {
        node = Inst::instPackage(node.clone(), context.clone())?;
        if InstNode::isPartial(node.clone()) && !(InstContext::inRelaxed(context.clone()) || InstContext::inRedeclared(context.clone())) {
            state = Arc::new(LookupState::LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::PARTIAL_CLASS) });
            return Ok((node, state));
        }
    }
    let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            (node, is_import) = lookupLocalSimpleName((var_field!((*name).name, Absyn::Path::IDENT).clone()).clone(), node.clone())?;
            if is_import.clone() {
                state = Arc::new(LookupState::LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::IMPORT) });
            } else {
                state = LookupState::next(node.clone(), state.clone(), context.clone(), checkAccessViolations.clone())?;
            }
            ()
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            (node, is_import) = lookupLocalSimpleName((var_field!((*name).name, Absyn::Path::QUALIFIED).clone()).clone(), node.clone())?;
            if is_import.clone() {
                state = Arc::new(LookupState::LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::IMPORT) });
            } else {
                state = LookupState::next(node.clone(), state.clone(), context.clone(), checkAccessViolations.clone())?;
                (node, state) = lookupLocalName(var_field!((*name).path, Absyn::Path::QUALIFIED).clone(), node.clone(), state.clone(), context.clone(), checkAccessViolations.clone(), false)?;
            }
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookup.lookupLocalName")); __mm_s.push_str(&*literal!(" was called with an invalid path.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((node, state))
}

pub fn lookupLocalNames(mut name: Arc<Absyn::Path>, mut scope: Arc<InstNode::InstNode>, mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut state: Arc<LookupState::LookupState>, mut context: i32, mut selfReference: bool) -> Result<(Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<LookupState::LookupState>)> {
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = nodes;
    let mut state: Arc<LookupState::LookupState> = state;
    let mut node: Arc<InstNode::InstNode> = scope.clone();
    if !(InstNode::isClass(scope.clone())) {
        state = Arc::new(crate::NFLookupState::LookupState::COMP_CLASS);
        return Ok((nodes, state));
    }
    if !(selfReference.clone()) {
        node = Inst::instPackage(node.clone(), context.clone())?;
        if InstNode::isPartial(node.clone()) && !(InstContext::inRelaxed(context.clone()) || InstContext::inRedeclared(context.clone())) && !(InstNode::name(node.clone())? == literal!("PartialModelicaServices")) {
            state = Arc::new(LookupState::LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::PARTIAL_CLASS) });
            return Ok((nodes, state));
        }
    }
    (nodes, state) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            (node, _) = lookupLocalSimpleName((var_field!((*name).name, Absyn::Path::IDENT).clone()).clone(), node.clone())?;
            state = LookupState::next(node.clone(), state.clone(), context.clone(), true)?;
            (cons(node.clone(), nodes.clone()), state.clone())
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            (node, _) = lookupLocalSimpleName((var_field!((*name).name, Absyn::Path::QUALIFIED).clone()).clone(), node.clone())?;
            state = LookupState::next(node.clone(), state.clone(), context.clone(), true)?;
            lookupLocalNames(var_field!((*name).path, Absyn::Path::QUALIFIED).clone(), node.clone(), cons(node.clone(), nodes.clone()), state.clone(), context.clone(), false)?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookup.lookupLocalNames")); __mm_s.push_str(&*literal!(" was called with an invalid path.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((nodes, state))
}

pub fn lookupSimpleBuiltinName(mut name: ArcStr) -> Result<Arc<InstNode::InstNode>> {
    let mut builtin: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    builtin = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "Real" => NFBuiltin::REAL_NODE().clone(),
        Deref @ "Integer" => NFBuiltin::INTEGER_NODE().clone(),
        Deref @ "Boolean" => NFBuiltin::BOOLEAN_NODE().clone(),
        Deref @ "String" => NFBuiltin::STRING_NODE().clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(builtin)
}

pub fn lookupSimpleBuiltinCref(mut name: ArcStr, mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<(Arc<InstNode::InstNode>, Arc<ComponentRef::NFComponentRef>, Arc<LookupState::LookupState>)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    (node, cref, state) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "time" => (NFBuiltin::TIME().clone(), NFBuiltin::TIME_CREF().clone(), Arc::new(crate::NFLookupState::LookupState::PREDEF_COMP)),
        Deref @ "Boolean" => (NFBuiltin::BOOLEAN_NODE().clone(), NFBuiltin::BOOLEAN_CREF().clone(), Arc::new(crate::NFLookupState::LookupState::PREDEF_CLASS)),
        Deref @ "Integer" => (NFBuiltinFuncs::INTEGER_NODE().clone(), NFBuiltinFuncs::INTEGER_CREF().clone(), Arc::new(crate::NFLookupState::LookupState::FUNC)),
        Deref @ "String" => (NFBuiltinFuncs::STRING_NODE().clone(), NFBuiltinFuncs::STRING_CREF().clone(), Arc::new(crate::NFLookupState::LookupState::FUNC)),
        _ => bail!("match: no arm matched"),
    } });
    if !(subs.clone().is_empty()) {
        cref = ComponentRef::setSubscripts({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Arc::new(Subscript::NFSubscript::RAW_SUBSCRIPT { subscript: s.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, cref.clone())?;
    }
    Ok((node, cref, state))
}

pub fn lookupSimpleCref(mut name: ArcStr, mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<InstNode::InstNode>, Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, bool, Arc<LookupState::LookupState>)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut foundScope: Arc<InstNode::InstNode> = scope.clone();
    let mut inEnclosingScope: bool = false;
    let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
    let mut require_builtin: bool = false;
    let mut loaded: bool = false;
    let mut is_enclosing: bool = false;
    match '__try0: {
        (node, cref, state) = unwrap_break_err!(lookupSimpleBuiltinCref((name.clone()).clone(), subs.clone()), '__try0);
        foundScope = InstNode::topScope(foundScope.clone());
        Ok::<_, anyhow::Error>((cref.clone(), foundScope.clone(), node.clone(), state.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cref = __try0_o0;
            foundScope = __try0_o1;
            node = __try0_o2;
            state = __try0_o3;
        }
        Err(_) => {
            if InstContext::inAnnotation(context.clone()) {
                if '__try1: {
                    (node, foundScope) = unwrap_break_err!(lookupLocalSimpleCref((name.clone()).clone(), InstNode::annotationScope(foundScope.clone())?), '__try1);
                    state = unwrap_break_err!(LookupState::nodeState(node.clone()), '__try1);
                    cref = ComponentRef::fromAbsyn(node.clone(), subs.clone(), Arc::new(crate::NFComponentRef::EMPTY));
                    return Ok((node, cref, foundScope, inEnclosingScope, state));
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
            }
            for mut i in 1..=Global::recursionDepthLimit.clone() {
                if '__try2: {
                    (node, foundScope) = unwrap_break_err!(lookupLocalSimpleCref((name.clone()).clone(), foundScope.clone()), '__try2);
                    if require_builtin.clone() {
                        let true = (InstNode::isBuiltin(node.clone())) else { break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                    }
                    state = unwrap_break_err!(LookupState::nodeState(node.clone()), '__try2);
                    cref = ComponentRef::fromAbsyn(node.clone(), subs.clone(), Arc::new(crate::NFComponentRef::EMPTY));
                    return Ok((node, cref, foundScope, inEnclosingScope, state));
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                    if InstNode::isEncapsulated(foundScope.clone())? {
                        foundScope = InstNode::topScope(InstNode::parentScope(foundScope.clone(), false)?);
                        require_builtin = true;
                    } else {
                        if InstNode::isTopScope(foundScope.clone()) && !(loaded.clone()) && !(require_builtin.clone()) {
                            loaded = true;
                            loadLibrary((name.clone()).clone(), foundScope.clone());
                        } else {
                            inEnclosingScope = !(InstNode::isImplicit(foundScope.clone()));
                            foundScope = InstNode::parentScope(foundScope.clone(), false)?;
                        }
                    }
                }
            }
            Error::addMessage(Error::RECURSION_DEPTH_REACHED.clone(), list![ArcStr::from(::std::format!("{}", Global::recursionDepthLimit.clone())), (InstNode::scopeName(foundScope.clone())).clone()])?;
            bail!("fail");
        }
    }
    Ok((node, cref, foundScope, inEnclosingScope, state))
}

pub fn lookupLocalSimpleCref(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>) -> Result<(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut foundScope: Arc<InstNode::InstNode> = scope.clone();
    let mut is_import: bool = false;
    (node, is_import) = (::match_deref::match_deref! { match &(foundScope.clone()) {
        Deref @ InstNode::IMPLICIT_SCOPE { .. } => (lookupIterator((name.clone()).clone(), var_field!((*foundScope).locals, InstNode::InstNode::IMPLICIT_SCOPE).clone())?, false),
        Deref @ InstNode::CLASS_NODE { .. } => Class::lookupElement((name.clone()).clone(), InstNode::getClass(foundScope.clone())?)?,
        Deref @ InstNode::COMPONENT_NODE { .. } => Class::lookupElement((name.clone()).clone(), InstNode::getClass(foundScope.clone())?)?,
        Deref @ InstNode::INNER_OUTER_NODE { .. } => Class::lookupElement((name.clone()).clone(), InstNode::getClass(var_field!((*foundScope).innerNode, InstNode::InstNode::INNER_OUTER_NODE).clone())?)?,
        _ => bail!("match: no arm matched"),
    } });
    if is_import.clone() {
        foundScope = InstNode::parent(node.clone());
    } else if InstNode::isInnerOuterNode(node.clone()) {
        node = InstNode::resolveInner(node.clone());
        foundScope = InstNode::parent(node.clone());
    }
    Ok((node, foundScope))
}

pub fn lookupIterator(mut name: ArcStr, mut iterators: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<InstNode::InstNode>> {
    let mut iterator: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    for mut i in &*iterators.clone() {
        let mut i = i.clone();
        if name.clone() == InstNode::name(i.clone())? {
            iterator = i.clone();
            return Ok(iterator);
        }
    }
    bail!("fail");
    Ok(iterator)
}

pub fn lookupCrefInNode(mut cref: Arc<Absyn::ComponentRef>, mut node: Arc<InstNode::InstNode>, mut foundCref: Arc<ComponentRef::NFComponentRef>, mut foundScope: Arc<InstNode::InstNode>, mut state: Arc<LookupState::LookupState>, mut context: i32) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, Arc<LookupState::LookupState>)> {
    let mut foundCref: Arc<ComponentRef::NFComponentRef> = foundCref;
    let mut foundScope: Arc<InstNode::InstNode> = foundScope;
    let mut state: Arc<LookupState::LookupState> = state;
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut n: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut name: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut is_import: bool = false;
    let mut scope_is_class: bool = false;
    if LookupState::isError(state.clone()) || InstContext::inConnect(context.clone()) && InstNode::isEmpty(node.clone()) {
        return Ok((foundCref, foundScope, state));
    }
    scope = node.clone();
    scope_is_class = InstNode::isClass(scope.clone());
    if scope_is_class.clone() {
        scope = Inst::instPackage(node.clone(), context.clone())?;
        if InstNode::isPartial(scope.clone()) && !(InstContext::inRelaxed(context.clone()) || InstContext::inRedeclared(context.clone())) {
            state = Arc::new(LookupState::LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::PARTIAL_CLASS) });
            return Ok((foundCref, foundScope, state));
        }
    } else if InstNode::isGeneratedInner(scope.clone()) && Component::isDefinition(InstNode::component(scope.clone())?) {
        Inst::instComponent(scope.clone(), Attributes::DEFAULT_ATTR().clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), true, 0, InstContext::CLASS.clone(), None, metamodelica::nil())?;
    }
    name = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
    cls_node = InstNode::classScope(scope.clone());
    if InstNode::isEmpty(cls_node.clone()) {
        foundCref = ComponentRef::fromAbsynCref(cref.clone(), foundCref.clone())?;
        return Ok((foundCref, foundScope, state));
    }
    cls = InstNode::getClass(cls_node.clone())?;
    if let Ok((__pa0, __pa1)) = Class::lookupElement((name.clone()).clone(), cls.clone()) {
        n = __pa0.clone();
        is_import = __pa1.clone();
    } else {
        let true = (InstNode::isComponent(node.clone())) else { bail!("pattern mismatch") };
        let true = (Class::isExpandableConnectorClass(cls.clone()) || InstContext::inInstanceAPI(context.clone())) else { bail!("pattern mismatch") };
        foundCref = ComponentRef::fromAbsynCref(cref.clone(), foundCref.clone())?;
        return Ok((foundCref, foundScope, state));
    }
    if is_import.clone() {
        state = Arc::new(LookupState::LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::IMPORT) });
        foundCref = ComponentRef::fromAbsyn(n.clone(), metamodelica::nil(), foundCref.clone());
        return Ok((foundCref, foundScope, state));
    }
    (n, foundCref, foundScope) = resolveInnerCref(n.clone(), foundCref.clone(), foundScope.clone())?;
    foundCref = ComponentRef::fromAbsyn(n.clone(), AbsynUtil::crefFirstSubs(cref.clone())?, foundCref.clone());
    if scope_is_class.clone() && !(InstContext::inRelaxed(context.clone())) && LookupState::isNonConstantComponent(n.clone())? {
        state = Arc::new(LookupState::LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::NON_ENCAPSULATED) });
        return Ok((foundCref, foundScope, state));
    } else {
        state = LookupState::next(n.clone(), state.clone(), context.clone(), true)?;
    }
    (foundCref, foundScope, state) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => (foundCref.clone(), foundScope.clone(), state.clone()),
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => lookupCrefInNode(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), n.clone(), foundCref.clone(), foundScope.clone(), state.clone(), context.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok((foundCref, foundScope, state))
}

pub fn resolveInnerCref(mut node: Arc<InstNode::InstNode>, mut cref: Arc<ComponentRef::NFComponentRef>, mut foundScope: Arc<InstNode::InstNode>) -> Result<(Arc<InstNode::InstNode>, Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>)> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut foundScope: Arc<InstNode::InstNode> = foundScope;
    let mut prev_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if InstNode::isInnerOuterNode(node.clone()) {
        node = InstNode::resolveInner(node.clone());
        scope = InstNode::parent(node.clone());
        while !(ComponentRef::isEmpty(cref.clone())) {
            if referenceEq(&ComponentRef::node(cref.clone())?,&scope.clone()) {
                break;
            } else {
                cref = ComponentRef::rest(cref.clone())?;
            }
        }
        if ComponentRef::isEmpty(cref.clone()) {
            foundScope = scope.clone();
        }
    }
    Ok((node, cref, foundScope))
}

pub fn generateInner(mut outerNode: Arc<InstNode::InstNode>, mut topScope: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
    let mut innerNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut node_ty: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
    let mut name: ArcStr = arcstr::literal!("");
    let mut inner_node_opt: Option<Arc<InstNode::InstNode>> = None;
    let mut inner_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut parent_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    node_ty = InstNode::nodeType(topScope.clone())?;
    let () = (::match_deref::match_deref! { match &(node_ty.clone()) {
        Deref @ InstNodeType::TOP_SCOPE { .. } => {
            name = (InstNode::name(outerNode.clone())?).clone();
            inner_node_opt = UnorderedMap::get((name.clone()).clone(), var_field!((*node_ty).generatedInners, InstNodeType::TOP_SCOPE).clone());
            if isSome(inner_node_opt.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(inner_node_opt.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                innerNode = __pa0.clone();
            } else {
                innerNode = makeInnerNode(outerNode.clone())?;
                innerNode = InstNode::setNodeType(Arc::new(crate::NFInstNode::InstNodeType::GENERATED_INNER), innerNode.clone());
                UnorderedMap::add((name.clone()).clone(), innerNode.clone(), var_field!((*node_ty).generatedInners, InstNodeType::TOP_SCOPE).clone())?;
            }
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookup.generateInner")); __mm_s.push_str(&*literal!(" got invalid top node")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(innerNode)
}

pub fn makeInnerNode(mut node: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    node = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ InstNode::CLASS_NODE { definition: def @ Deref @ SCode::Element::CLASS { prefixes: prefs, .. }, .. } => {
            let mut def = (*def).clone();
            let mut prefs = (*prefs).clone();
            assign_field!(prefs.innerOuter = openmodelica_ast::Absyn::InnerOuter::INNER);
            assign_variant_field!(def => SCode::Element::CLASS; prefixes = prefs.clone());
            assign_variant_field!(node => InstNode::InstNode::CLASS_NODE; definition = def.clone());
            node.clone()
        },
        Deref @ InstNode::COMPONENT_NODE { .. } => {
            let mut def: Arc<SCode::Element>;
            let mut prefs: Arc<SCode::Prefixes>;
            let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
            comp = InstNode::component(node.clone())?;
            comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT_DEF { definition: def @ Deref @ SCode::Element::COMPONENT { prefixes: prefs, .. }, .. } => {
            let mut def = (*def).clone();
            let mut prefs = (*prefs).clone();
            assign_field!(prefs.innerOuter = openmodelica_ast::Absyn::InnerOuter::INNER);
            assign_variant_field!(def => SCode::Element::COMPONENT; prefixes = prefs.clone());
            assign_variant_field!(comp => Component::NFComponent::COMPONENT_DEF; definition = def.clone());
            comp.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookup.makeInnerNode")); __mm_s.push_str(&*literal!(" got unknown component")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            InstNode::replaceComponent(comp.clone(), InstNode::setDefinition(def.clone(), node.clone())?)?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookup.makeInnerNode")); __mm_s.push_str(&*literal!(" got unknown node")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub fn loadLibrary(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>) -> () {
    let mut version: ArcStr = arcstr::literal!("");
    ErrorExt::setCheckpoint((literal!("NFLookup.loadLibrary")).clone());
    if '__try0: {
        let true = (unwrap_break_err!(Flags::getConfigBool(Flags::LOAD_MISSING_LIBRARIES.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        version = (unwrap_break_err!(loadLibrary_work((name.clone()).clone(), scope.clone()), '__try0)).clone();
        unwrap_break_err!(Error::addMessage(Error::NOTIFY_IMPLICIT_LOAD.clone(), list![(name.clone()).clone(), (version.clone()).clone()]), '__try0);
        System::loadModelCallBack((name.clone()).clone());
        ErrorExt::delCheckpoint((literal!("NFLookup.loadLibrary")).clone());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        ErrorExt::rollBack((literal!("NFLookup.loadLibrary")).clone());
    }
    ()
}

pub fn loadLibrary_work(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>) -> Result<ArcStr> {
    let mut version: ArcStr = literal!("(default)");
    let mut modelica_path: ArcStr = arcstr::literal!("");
    let mut cls_name: ArcStr = arcstr::literal!("");
    let mut aprog: Absyn::Program;
    let mut scls: Arc<SCode::Element>;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut lib_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut new_libs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    modelica_path = (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone();
    let (__pa0, true) = (BackendInterface::appendLibrary(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), (modelica_path.clone()).clone())) else { bail!("pattern mismatch") };
    aprog = __pa0.clone();
    for mut c in &*aprog.classes.clone() {
        let mut c = c.clone();
        if '__try1: {
            unwrap_break_err!(lookupLocalSimpleName((AbsynUtil::getClassName(c.clone())?).clone(), scope.clone()), '__try1);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            scls = AbsynToSCode::translateClass(c.clone())?;
            lib_node = InstNode::new(scls.clone(), scope.clone())?;
            new_libs = cons(lib_node.clone(), new_libs.clone());
            if name.clone() == SCodeUtil::getElementName(scls.clone())? {
                if '__try2: {
                    let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(SCodeUtil::lookupElementAnnotationBinding(scls.clone(), (literal!("version")).clone()), '__try2)) {
                        Some(Deref @ Absyn::Exp::STRING { value: __pa3 }) => __pa3.clone(),
                        _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    version = __pa3.clone();
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
            }
        }
    }
    cls = InstNode::getClass(scope.clone())?;
    cls = Class::classTreeApply(cls.clone(), Arc::new({ let __pe_b0 = new_libs.clone(); move |__pe_a1| ClassTree::appendClasses(__pe_b0.clone(), __pe_a1) }));
    InstNode::updateClass(cls.clone(), scope.clone())?;
    Ok(version)
}

