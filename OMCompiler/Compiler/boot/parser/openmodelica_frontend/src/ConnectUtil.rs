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

use crate::ComponentReference;
use crate::ConnectionGraph;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::FCore;
use crate::InnerOuter;
use crate::Lookup;
use crate::PrefixUtil;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect::ConnectorElement;
use openmodelica_frontend_types::DAE::Connect::ConnectorType;
use openmodelica_frontend_types::DAE::Connect::Face;
use openmodelica_frontend_types::DAE::Connect::OuterConnect;
use openmodelica_frontend_types::DAE::Connect::Set;
use openmodelica_frontend_types::DAE::Connect::SetConnection;
use openmodelica_frontend_types::DAE::Connect::SetTrie;
use openmodelica_frontend_types::DAE::Connect::SetTrieNode;
use openmodelica_frontend_types::DAE::Connect::Sets;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
// Import some types from Connect.
// Set graph represented as an adjacency list.
pub type SetGraph = metamodelica::Array<Arc<metamodelica::List<i32>>>;

pub fn newSet(mut prefix: DAE::Prefix, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut pstr: ArcStr = arcstr::literal!("");
    let mut sc: i32 = 0;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let Sets { setCount: __pa0, .. } = (sets.clone()) else { bail!("pattern mismatch") };
    sc = __pa0.clone();
    match '__try1: {
        cr = unwrap_break_err!(PrefixUtil::prefixFirstCref(prefix.clone()), '__try1);
        pstr = (unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(cr.clone()), '__try1)).clone();
        Ok::<_, anyhow::Error>((cr.clone(), pstr.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            cr = __try1_o0;
            pstr = __try1_o1;
        }
        Err(_) => {
            cr = Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD);
            pstr = (literal!("")).clone();
        }
    }
    sets = Sets { sets: Arc::new(SetTrieNode::SET_TRIE_NODE { name: (pstr.clone()).clone(), cref: cr.clone(), nodes: metamodelica::nil(), connectCount: 0 }), setCount: sc.clone(), connections: metamodelica::nil(), outerConnects: metamodelica::nil() };
    Ok(sets)
}

pub fn addSet(mut parentSets: Sets, mut childSets: Sets) -> Result<Sets> {
    let mut sets: Sets = <Sets as ::std::default::Default>::default();
    sets = 'mc: {
        let __mc_input = (parentSets.clone(), childSets.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    if !((isEmptySet(childSets.clone()))) { bail!("guard") }
                    Ok(parentSets.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Sets { sets: Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { cref: Deref @ DAE::ComponentRef::WILD, .. }, .. }, Sets { sets: Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { cref: Deref @ DAE::ComponentRef::WILD, .. }, .. }) => {
                    Ok(childSets.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Sets { sets: node @ Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. }, .. }, Sets { .. }) => {
                    let _ = setTrieGetNode((setTrieNodeName(childSets.sets.clone())?).clone(), var_field!((*node).nodes, SetTrieNode::SET_TRIE_NODE).clone())?;
                    Ok(parentSets.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Sets { sets: node @ Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. }, setCount: _, connections: c1, outerConnects: o1 }, Sets { sets: _, setCount: sc, connections: c2, outerConnects: o2 }) => {
                    let mut node = (*node).clone();
                    let mut c1 = (*c1).clone();
                    let mut o1 = (*o1).clone();
                    c1 = listAppend(c2.clone(), c1.clone());
                    o1 = listAppend(o2.clone(), o1.clone());
                    assign_variant_field!(node => SetTrieNode::SET_TRIE_NODE; nodes = cons(childSets.sets.clone(), var_field!((*node).nodes, SetTrieNode::SET_TRIE_NODE).clone()));
                    Ok(Sets { sets: node.clone(), setCount: sc.clone(), connections: c1.clone(), outerConnects: o1.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(sets)
}

fn isEmptySet(mut sets: Sets) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(sets.clone()) {
        Sets { outerConnects: Deref @ metamodelica::List::Nil, connections: Deref @ metamodelica::List::Nil, sets: Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { nodes: Deref @ metamodelica::List::Nil, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn addConnection(mut sets: Sets, mut cref1: Arc<DAE::ComponentRef>, mut face1: Face, mut cref2: Arc<DAE::ComponentRef>, mut face2: Face, mut connectorType: Arc<DAE::ConnectorType>, mut source: Arc<DAE::ElementSource>) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut e1: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    let mut e2: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    let mut ty: ConnectorType = ConnectorType::EQU;
    ty = makeConnectorType(connectorType.clone())?;
    e1 = findElement(cref1.clone(), face1.clone(), ty.clone(), source.clone(), sets.clone());
    e2 = findElement(cref2.clone(), face2.clone(), ty.clone(), source.clone(), sets.clone());
    sets = mergeSets(e1.clone(), e2.clone(), sets.clone())?;
    Ok(sets)
}

fn getConnectCount(mut cref: Arc<DAE::ComponentRef>, mut trie: Arc<SetTrieNode>) -> i32 {
    let mut count: i32 = 0;
    let mut node: Arc<SetTrieNode>;
    match '__try0: {
        node = unwrap_break_err!(setTrieGet(cref.clone(), trie.clone(), false), '__try0);
        count = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => var_field!((*node).connectCount, SetTrieNode::SET_TRIE_NODE).clone(),
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => var_field!((*node).connectCount, SetTrieNode::SET_TRIE_LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok::<_, anyhow::Error>((count.clone(),))
    } {
        Ok((__try0_o0,)) => {
            count = __try0_o0;
        }
        Err(_) => {
            count = 0;
        }
    }
    count
}

pub fn addArrayConnection(mut sets: Sets, mut cref1: Arc<DAE::ComponentRef>, mut face1: Face, mut cref2: Arc<DAE::ComponentRef>, mut face2: Face, mut source: Arc<DAE::ElementSource>, mut connectorType: Arc<DAE::ConnectorType>) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    crefs1 = ComponentReference::expandCref(cref1.clone(), false)?;
    crefs2 = ComponentReference::expandCref(cref2.clone(), false)?;
    for mut cr1 in &*crefs1.clone() {
        let mut cr1 = cr1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cr2 = __pa0.clone();
        crefs2 = __pa1.clone();
        sets = addConnection(sets.clone(), cr1.clone(), face1.clone(), cr2.clone(), face2.clone(), connectorType.clone(), source.clone())?;
    }
    Ok(sets)
}

fn makeConnectorType(mut connectorType: Arc<DAE::ConnectorType>) -> Result<ConnectorType> {
    let mut ty: ConnectorType = ConnectorType::EQU;
    let mut flowName: Option<Arc<DAE::ComponentRef>> = None;
    ty = (::match_deref::match_deref! { match &(connectorType.clone()) {
        Deref @ DAE::ConnectorType::POTENTIAL => openmodelica_frontend_types::DAE::Connect::ConnectorType::EQU,
        Deref @ DAE::ConnectorType::FLOW => openmodelica_frontend_types::DAE::Connect::ConnectorType::FLOW,
        Deref @ DAE::ConnectorType::STREAM { associatedFlow: flowName } => ConnectorType::STREAM { associatedFlow: flowName.clone() },
        Deref @ DAE::ConnectorType::NON_CONNECTOR => openmodelica_frontend_types::DAE::Connect::ConnectorType::NO_TYPE,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ConnectUtil.makeConnectorType: invalid connector type.")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn addConnectorVariablesFromDAE(mut ignore: bool, mut classState: ClassInf::State, mut prefix: DAE::Prefix, mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut info: SourceInfo, mut elementSource: Arc<DAE::ElementSource>, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    sets = (match classState.clone() {
        ClassInf::State::CONNECTOR { isExpandable: false, path: ref class_path } if (!(ignore.clone())) => {
            let mut streams: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut flows: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            checkConnectorBalance(vars.clone(), class_path.clone(), info.clone())?;
            if !(Flags::isSet(Flags::DISABLE_SINGLE_FLOW_EQ.clone())?) {
                (flows, streams) = getStreamAndFlowVariables(vars.clone());
                sets = List::fold2(flows.clone(), (std::sync::Arc::new(addFlowVariableFromDAE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ElementSource>, DAE::Prefix, Sets) -> Result<Sets> + 'static>), elementSource.clone(), prefix.clone(), sets.clone());
                sets = addStreamFlowAssociations(sets.clone(), prefix.clone(), streams.clone(), flows.clone())?;
            }
            sets.clone()
        },
        _ => {
            sets.clone()
        },
    });
    Ok(sets)
}

fn addFlowVariableFromDAE(mut variable: Arc<DAE::Var>, mut elementSource: Arc<DAE::ElementSource>, mut prefix: DAE::Prefix, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    crefs = daeVarToCrefs(variable.clone())?;
    for mut cr in &*crefs.clone() {
        let mut cr = cr.clone();
        sets = addInsideFlowVariable(sets.clone(), cr.clone(), elementSource.clone(), prefix.clone())?;
    }
    Ok(sets)
}

pub fn isExpandable(mut name: Arc<DAE::ComponentRef>) -> bool {
    let mut expandableConnector: bool = false;
    expandableConnector = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => Types::isExpandableConnector(var_field!((*name).identType, DAE::ComponentRef::CREF_IDENT).clone()),
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => Types::isExpandableConnector(var_field!((*name).identType, DAE::ComponentRef::CREF_QUAL).clone()) || isExpandable(var_field!((*name).componentRef, DAE::ComponentRef::CREF_QUAL).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    expandableConnector
}

fn daeHasExpandableConnectors(mut DAE: DAE::DAElist) -> Result<bool> {
    let mut hasExpandable: bool = false;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    if System::getHasExpandableConnectors() {
        let DAE::DAE { elementLst: __pa0 } = (DAE.clone()) else { bail!("pattern mismatch") };
        vars = __pa0.clone();
        hasExpandable = List::any(vars.clone(), (std::sync::Arc::new(fnptr!(isVarExpandable, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
    } else {
        hasExpandable = false;
    }
    Ok(hasExpandable)
}

fn isVarExpandable(mut var: Arc<DAE::Element>) -> bool {
    let mut isExpandable: bool = false;
    isExpandable = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Element::VAR { .. } => self::isExpandable(var_field!((*var).componentRef, DAE::Element::VAR).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExpandable
}

fn getExpandableVariablesWithNoBinding(mut variables: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<DAE::ComponentRef>>> {
    let mut potential: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    for mut var in &*variables.clone() {
        let mut var = var.clone();
        let _ = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Element::VAR { binding: None, componentRef: name, .. } => {
            if isExpandable(name.clone()) {
                potential = cons(name.clone(), potential.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    potential
}

fn getStreamAndFlowVariables(mut variables: Arc<metamodelica::List<Arc<DAE::Var>>>) -> (Arc<metamodelica::List<Arc<DAE::Var>>>, Arc<metamodelica::List<Arc<DAE::Var>>>) {
    let mut flows: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut streams: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    for mut var in &*variables.clone() {
        let mut var = var.clone();
        let _ = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::FLOW, .. }, .. } => {
            flows = cons(var.clone(), flows.clone());
            ()
        },
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. }, .. } => {
            streams = cons(var.clone(), streams.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    (flows, streams)
}

fn addStreamFlowAssociations(mut sets: Sets, mut prefix: DAE::Prefix, mut streamVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut flowVars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut flow_var: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    let mut flow_cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut stream_crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    if streamVars.clone().is_empty() {
        return Ok(sets);
    }
    let __pa0 = ::match_deref::match_deref! { match &(flowVars.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    flow_var = __pa0.clone();
    let __pa2 = ::match_deref::match_deref! { match &(daeVarToCrefs(flow_var.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    flow_cr = __pa2.clone();
    flow_cr = PrefixUtil::prefixCrefNoContext(prefix.clone(), flow_cr.clone())?;
    for mut stream_var in &*streamVars.clone() {
        let mut stream_var = stream_var.clone();
        stream_crs = daeVarToCrefs(stream_var.clone())?;
        for mut stream_cr in &*stream_crs.clone() {
            let mut stream_cr = stream_cr.clone();
            sets = addStreamFlowAssociation(stream_cr.clone(), flow_cr.clone(), sets.clone())?;
        }
    }
    Ok(sets)
}

fn daeVarToCrefs(mut var: Arc<DAE::Var>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { ty: __pa0, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    name = __pa1.clone();
    ty = Types::derivedBasicType(ty.clone());
    crefs = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => list![Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() })],
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            crs = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone().reverse()).into_iter().cloned() {
            let __x = daeVarToCrefs(v.clone())?;
            __acc = __x.append(&__acc);
        }
        __acc
    };
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut c in (crs.clone()).into_iter().cloned() {
            let __x = ComponentReference::joinCrefs(cr.clone(), c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        Deref @ DAE::Type::T_ARRAY { .. } => {
            dims = TypesDump::getDimensions(ty.clone());
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() });
            expandArrayCref(cr.clone(), dims.clone(), metamodelica::nil())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown var ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" in ConnectUtil.daeVarToCrefs")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn expandArrayCref(mut cref: Arc<DAE::ComponentRef>, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut accumCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    crefs = 'mc: {
        let __mc_input = dims.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(cons(cref.clone(), accumCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: dim, tail: rest_dims } => {
                    let mut idx: Arc<DAE::Exp>;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut dim = (*dim).clone();
                    (idx, dim) = getNextIndex(dim.clone())?;
                    cr = ComponentReference::subscriptCref(cref.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: idx.clone() })])?;
                    crs = expandArrayCref(cr.clone(), rest_dims.clone(), accumCrefs.clone())?;
                    crs = expandArrayCref(cref.clone(), cons(dim.clone(), rest_dims.clone()), crs.clone())?;
                    Ok(crs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(accumCrefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(crefs)
}

fn reverseEnumType(mut dim: Arc<DAE::Dimension>) -> Arc<DAE::Dimension> {
    let mut dim: Arc<DAE::Dimension> = dim;
    let _ = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_ENUM { .. } => {
            assign_variant_field!(dim => DAE::Dimension::DIM_ENUM; literals = var_field!((*dim).literals, DAE::Dimension::DIM_ENUM).clone().reverse());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dim
}

fn getNextIndex(mut dim: Arc<DAE::Dimension>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Dimension>)> {
    let mut nextIndex: Arc<DAE::Exp>;
    let mut restDim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    (nextIndex, restDim) = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 } => {
            bail!("fail")
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: 0, .. } => {
            bail!("fail")
        },
        Deref @ DAE::Dimension::DIM_INTEGER { integer: new_idx } => {
            let mut dim_size: i32 = 0;
            dim_size = new_idx.clone() - 1;
            (Arc::new(DAE::Exp::ICONST { integer: new_idx.clone() }), Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_size.clone() }))
        },
        Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: p, literals: Deref @ metamodelica::List::Cons { head: l, tail: l_rest }, size: new_idx } => {
            let mut dim_size: i32 = 0;
            let mut ep: Arc<Absyn::Path>;
            ep = AbsynUtil::joinPaths(p.clone(), Arc::new(Absyn::Path::IDENT { name: (l.clone()).clone() }))?;
            dim_size = new_idx.clone() - 1;
            (Arc::new(DAE::Exp::ENUM_LITERAL { name: ep.clone(), index: new_idx.clone() }), Arc::new(DAE::Dimension::DIM_ENUM { enumTypeName: p.clone(), literals: l_rest.clone(), size: dim_size.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((nextIndex, restDim))
}

fn addInsideFlowVariable(mut sets: Sets, mut cref: Arc<DAE::ComponentRef>, mut source: Arc<DAE::ElementSource>, mut prefix: DAE::Prefix) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut e: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    if '__try0: {
        unwrap_break_err!(setTrieGetElement(cref.clone(), openmodelica_frontend_types::DAE::Connect::Face::INSIDE, sets.sets.clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        sets.setCount = sets.setCount.clone() + 1;
        e = newElement(cref.clone(), openmodelica_frontend_types::DAE::Connect::Face::INSIDE, openmodelica_frontend_types::DAE::Connect::ConnectorType::FLOW, source.clone(), sets.setCount.clone());
        sets.sets = setTrieAdd(e.clone(), sets.sets.clone())?;
    }
    Ok(sets)
}

fn addStreamFlowAssociation(mut streamCref: Arc<DAE::ComponentRef>, mut flowCref: Arc<DAE::ComponentRef>, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    sets = updateSetLeaf(sets.clone(), streamCref.clone(), flowCref.clone(), (std::sync::Arc::new(addStreamFlowAssociation2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>))?;
    Ok(sets)
}

fn addStreamFlowAssociation2(mut flowCref: Arc<DAE::ComponentRef>, mut node: Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> {
    let mut node: Arc<SetTrieNode> = node;
    let _ = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            assign_variant_field!(node => SetTrieNode::SET_TRIE_LEAF; flowAssociation = Some(flowCref.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(node)
}

fn getStreamFlowAssociation(mut streamCref: Arc<DAE::ComponentRef>, mut sets: Sets) -> Result<Arc<DAE::ComponentRef>> {
    let mut flowCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let __pa0 = ::match_deref::match_deref! { match &(setTrieGet(streamCref.clone(), sets.sets.clone(), false)?) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { flowAssociation: Some(__pa0), .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    flowCref = __pa0.clone();
    Ok(flowCref)
}

pub fn addOuterConnection(mut scope: DAE::Prefix, mut sets: Sets, mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>, mut io1: Absyn::InnerOuter, mut io2: Absyn::InnerOuter, mut f1: Face, mut f2: Face, mut source: Arc<DAE::ElementSource>) -> Sets {
    let mut sets: Sets = sets;
    let mut new_oc: OuterConnect = <OuterConnect as ::std::default::Default>::default();
    if !(List::any(sets.outerConnects.clone(), Arc::new({ let __pe_b1 = cr1.clone(); let __pe_b2 = cr2.clone(); move |__pe_a0| outerConnectionMatches(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))) {
        new_oc = OuterConnect { scope: scope.clone(), cr1: cr1.clone(), io1: io1.clone(), f1: f1.clone(), cr2: cr2.clone(), io2: io2.clone(), f2: f2.clone(), source: source.clone() };
        sets.outerConnects = cons(new_oc.clone(), sets.outerConnects.clone());
    }
    sets
}

fn outerConnectionMatches(mut oc: OuterConnect, mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut matches: bool = false;
    matches = (match oc.clone() {
        OuterConnect { .. } => ComponentReferenceBasics::crefEqual(oc.cr1.clone(), cr1.clone())? && ComponentReferenceBasics::crefEqual(oc.cr2.clone(), cr2.clone())? || ComponentReferenceBasics::crefEqual(oc.cr1.clone(), cr2.clone())? && ComponentReferenceBasics::crefEqual(oc.cr2.clone(), cr1.clone())?,
        _ => bail!("match: no arm matched"),
    });
    Ok(matches)
}

pub fn addOuterConnectToSets(mut cref1: Arc<DAE::ComponentRef>, mut cref2: Arc<DAE::ComponentRef>, mut io1: Absyn::InnerOuter, mut io2: Absyn::InnerOuter, mut face1: Face, mut face2: Face, mut sets: Sets, mut inInfo: SourceInfo) -> Result<(Sets, bool)> {
    let mut sets: Sets = sets;
    let mut added: bool = false;
    let mut is_outer1: bool = false;
    let mut is_outer2: bool = false;
    is_outer1 = AbsynUtil::isOuter(io1.clone());
    is_outer2 = AbsynUtil::isOuter(io2.clone());
    added = (match (is_outer1.clone(), is_outer2.clone()) {
        (true, true) => {
            Error::addSourceMessage(Error::UNSUPPORTED_LANGUAGE_FEATURE.clone(), list![(literal!("Connections where both connectors are outer references")).clone(), (literal!("No suggestion")).clone()], inInfo.clone())?;
            false
        },
        (false, false) => false,
        (true, false) => {
            (sets, added) = addOuterConnectToSets2(cref1.clone(), cref2.clone(), face1.clone(), face2.clone(), sets.clone());
            added.clone()
        },
        (false, true) => {
            (sets, added) = addOuterConnectToSets2(cref2.clone(), cref1.clone(), face2.clone(), face1.clone(), sets.clone());
            added.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((sets, added))
}

fn addOuterConnectToSets2(mut outerCref: Arc<DAE::ComponentRef>, mut innerCref: Arc<DAE::ComponentRef>, mut outerFace: Face, mut innerFace: Face, mut sets: Sets) -> (Sets, bool) {
    let mut sets: Sets = sets;
    let mut added: bool = false;
    let mut node: Arc<SetTrieNode>;
    let mut outer_els: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    let mut inner_els: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    let mut sc: i32 = 0;
    match '__try0: {
        node = unwrap_break_err!(setTrieGet(outerCref.clone(), sets.sets.clone(), true), '__try0);
        outer_els = unwrap_break_err!(collectOuterElements(node.clone(), outerFace.clone()), '__try0);
        inner_els = {
        let mut __acc: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
        for mut oe in (outer_els.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(findInnerElement(oe.clone(), innerCref.clone(), innerFace.clone(), sets.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        sc = sets.setCount.clone();
        sets = unwrap_break_err!(List::threadFold(outer_els.clone(), inner_els.clone(), (std::sync::Arc::new(mergeSets) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, ConnectorElement, Sets) -> Result<Sets> + 'static>), sets.clone()), '__try0);
        added = sc.clone() != sets.setCount.clone();
        Ok::<_, anyhow::Error>((added.clone(), inner_els.clone(), node.clone(), outer_els.clone(), sc.clone(), sets.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            added = __try0_o0;
            inner_els = __try0_o1;
            node = __try0_o2;
            outer_els = __try0_o3;
            sc = __try0_o4;
            sets = __try0_o5;
        }
        Err(_) => {
            added = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (sets, added)
}

fn collectOuterElements(mut node: Arc<SetTrieNode>, mut face: Face) -> Result<Arc<metamodelica::List<ConnectorElement>>> {
    let mut outerElements: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    outerElements = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => List::mapFlat(var_field!((*node).nodes, SetTrieNode::SET_TRIE_NODE).clone(), Arc::new({ let __pe_b1 = face.clone(); let __pe_b2 = None; move |__pe_a0| collectOuterElements2(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) })),
        _ => collectOuterElements2(node.clone(), face.clone(), None)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outerElements)
}

fn collectOuterElements2(mut node: Arc<SetTrieNode>, mut face: Face, mut prefix: Option<Arc<DAE::ComponentRef>>) -> Result<Arc<metamodelica::List<ConnectorElement>>> {
    let mut outerElements: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    outerElements = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { cref: cr, .. } => {
            let mut cr = (*cr).clone();
            cr = optPrefixCref(prefix.clone(), cr.clone())?;
            List::mapFlat(var_field!((*node).nodes, SetTrieNode::SET_TRIE_NODE).clone(), Arc::new({ let __pe_b1 = face.clone(); let __pe_b2 = Some(cr.clone()); move |__pe_a0| collectOuterElements2(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
            e = setTrieGetLeafElement(node.clone(), face.clone())?;
            cr = getElementName(e.clone())?;
            e = setElementName(e.clone(), optPrefixCref(prefix.clone(), cr.clone())?);
            list![e.clone()]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outerElements)
}

fn findInnerElement(mut outerElement: ConnectorElement, mut innerCref: Arc<DAE::ComponentRef>, mut innerFace: Face, mut sets: Sets) -> Result<ConnectorElement> {
    let mut innerElement: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ty: ConnectorType = ConnectorType::EQU;
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let ConnectorElement { source: __pa0, ty: __pa1, name: __pa2, .. } = (outerElement.clone()) else { bail!("pattern mismatch") };
    src = __pa0.clone();
    ty = __pa1.clone();
    name = __pa2.clone();
    name = ComponentReference::joinCrefs(innerCref.clone(), name.clone())?;
    innerElement = findElement(name.clone(), innerFace.clone(), ty.clone(), src.clone(), sets.clone());
    Ok(innerElement)
}

fn optPrefixCref(mut prefix: Option<Arc<DAE::ComponentRef>>, mut cref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut cref: Arc<DAE::ComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(prefix.clone()) {
        None => {
            cref.clone()
        },
        Some(cr) => {
            ComponentReference::joinCrefs(cr.clone(), cref.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

fn findElement(mut cref: Arc<DAE::ComponentRef>, mut face: Face, mut ty: ConnectorType, mut source: Arc<DAE::ElementSource>, mut sets: Sets) -> ConnectorElement {
    let mut element: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    match '__try0: {
        element = unwrap_break_err!(setTrieGetElement(cref.clone(), face.clone(), sets.sets.clone()), '__try0);
        Ok::<_, anyhow::Error>((element.clone(),))
    } {
        Ok((__try0_o0,)) => {
            element = __try0_o0;
        }
        Err(_) => {
            element = newElement(cref.clone(), face.clone(), ty.clone(), source.clone(), Connect::NEW_SET.clone());
        }
    }
    element
}

fn newElement(mut cref: Arc<DAE::ComponentRef>, mut face: Face, mut ty: ConnectorType, mut source: Arc<DAE::ElementSource>, mut set: i32) -> ConnectorElement {
    let mut element: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    element = ConnectorElement { name: cref.clone(), face: face.clone(), ty: ty.clone(), source: source.clone(), set: set.clone() };
    element
}

fn isNewElement(mut element: ConnectorElement) -> Result<bool> {
    let mut isNew: bool = false;
    let mut set: i32 = 0;
    let ConnectorElement { set: __pa0, .. } = (element.clone()) else { bail!("pattern mismatch") };
    set = __pa0.clone();
    isNew = set.clone() == Connect::NEW_SET.clone();
    Ok(isNew)
}

fn getElementSetIndex(mut inElement: ConnectorElement) -> Result<i32> {
    let mut outIndex: i32 = 0;
    let ConnectorElement { set: __pa0, .. } = (inElement.clone()) else { bail!("pattern mismatch") };
    outIndex = __pa0.clone();
    Ok(outIndex)
}

fn setElementSetIndex(mut element: ConnectorElement, mut index: i32) -> ConnectorElement {
    let mut element: ConnectorElement = element;
    element.set = index.clone();
    element
}

fn getElementName(mut element: ConnectorElement) -> Result<Arc<DAE::ComponentRef>> {
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let ConnectorElement { name: __pa0, .. } = (element.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    Ok(name)
}

fn setElementName(mut element: ConnectorElement, mut name: Arc<DAE::ComponentRef>) -> ConnectorElement {
    let mut element: ConnectorElement = element;
    element.name = name.clone();
    element
}

fn getElementSource(mut element: ConnectorElement) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let ConnectorElement { source: __pa0, .. } = (element.clone()) else { bail!("pattern mismatch") };
    source = __pa0.clone();
    Ok(source)
}

fn setTrieNewLeaf(mut id: ArcStr, mut element: ConnectorElement) -> Result<Arc<SetTrieNode>> {
    let mut leaf: Arc<SetTrieNode>;
    leaf = (match element.clone() {
        ConnectorElement { face: DAE::Connect::Face::INSIDE, .. } => Arc::new(SetTrieNode::SET_TRIE_LEAF { name: (id.clone()).clone(), insideElement: Some(element.clone()), outsideElement: None, flowAssociation: None, connectCount: 0 }),
        ConnectorElement { face: DAE::Connect::Face::OUTSIDE, .. } => Arc::new(SetTrieNode::SET_TRIE_LEAF { name: (id.clone()).clone(), insideElement: None, outsideElement: Some(element.clone()), flowAssociation: None, connectCount: 0 }),
        _ => bail!("match: no arm matched"),
    });
    Ok(leaf)
}

fn setTrieNewNode(mut cref: Arc<DAE::ComponentRef>, mut element: ConnectorElement) -> Result<Arc<SetTrieNode>> {
    let mut node: Arc<SetTrieNode>;
    node = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            let mut id: ArcStr = arcstr::literal!("");
            id = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
            setTrieNewLeaf((id.clone()).clone(), setElementName(element.clone(), cref.clone()))?
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            let mut id: ArcStr = arcstr::literal!("");
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cr = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
            id = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            node = setTrieNewNode(var_field!((*cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), element.clone())?;
            Arc::new(SetTrieNode::SET_TRIE_NODE { name: (id.clone()).clone(), cref: cr.clone(), nodes: list![node.clone()], connectCount: 0 })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(node)
}

fn setTrieNodeName(mut node: Arc<SetTrieNode>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => var_field!((*node).name, SetTrieNode::SET_TRIE_NODE).clone(),
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => var_field!((*node).name, SetTrieNode::SET_TRIE_LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(name)
}

fn mergeSets(mut element1: ConnectorElement, mut element2: ConnectorElement, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut new1: bool = false;
    let mut new2: bool = false;
    new1 = isNewElement(element1.clone())?;
    new2 = isNewElement(element2.clone())?;
    sets = mergeSets2(element1.clone(), element2.clone(), new1.clone(), new2.clone(), sets.clone())?;
    Ok(sets)
}

fn mergeSets2(mut element1: ConnectorElement, mut element2: ConnectorElement, mut isNew1: bool, mut isNew2: bool, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    sets = (match (isNew1.clone(), isNew2.clone()) {
        (true, true) => addNewSet(element1.clone(), element2.clone(), sets.clone())?,
        (true, false) => addToSet(element1.clone(), element2.clone(), sets.clone())?,
        (false, true) => addToSet(element2.clone(), element1.clone(), sets.clone())?,
        (false, false) => connectSets(element1.clone(), element2.clone(), sets.clone())?,
        _ => bail!("match: no arm matched"),
    });
    Ok(sets)
}

fn addNewSet(mut element1: ConnectorElement, mut element2: ConnectorElement, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut node: Arc<SetTrieNode>;
    let mut sc: i32 = 0;
    let mut e1: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    let mut e2: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    sc = sets.setCount.clone() + 1;
    e1 = setElementSetIndex(element1.clone(), sc.clone());
    e2 = setElementSetIndex(element2.clone(), sc.clone());
    node = sets.sets.clone();
    node = setTrieAdd(e1.clone(), node.clone())?;
    sets.sets = setTrieAdd(e2.clone(), node.clone())?;
    sets.setCount = sc.clone();
    Ok(sets)
}

fn addToSet(mut element: ConnectorElement, mut set: ConnectorElement, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut index: i32 = 0;
    let mut e: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    index = getElementSetIndex(set.clone())?;
    e = setElementSetIndex(element.clone(), index.clone());
    sets.sets = setTrieAdd(e.clone(), sets.sets.clone())?;
    Ok(sets)
}

fn connectSets(mut element1: ConnectorElement, mut element2: ConnectorElement, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut set1: i32 = 0;
    let mut set2: i32 = 0;
    set1 = getElementSetIndex(element1.clone())?;
    set2 = getElementSetIndex(element2.clone())?;
    if set1.clone() != set2.clone() {
        sets.connections = cons((set1.clone(), set2.clone()), sets.connections.clone());
    }
    Ok(sets)
}

fn setTrieGetElement(mut cref: Arc<DAE::ComponentRef>, mut face: Face, mut trie: Arc<SetTrieNode>) -> Result<ConnectorElement> {
    let mut element: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    let mut node: Arc<SetTrieNode>;
    node = setTrieGet(cref.clone(), trie.clone(), false)?;
    element = setTrieGetLeafElement(node.clone(), face.clone())?;
    Ok(element)
}

fn setTrieAddLeafElement(mut element: ConnectorElement, mut node: Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> {
    let mut node: Arc<SetTrieNode> = node;
    let _ = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            let _ = (match element.face.clone() {
        DAE::Connect::Face::INSIDE => {
            assign_variant_field!(node => SetTrieNode::SET_TRIE_LEAF; insideElement = Some(element.clone()));
            ()
        },
        DAE::Connect::Face::OUTSIDE => {
            assign_variant_field!(node => SetTrieNode::SET_TRIE_LEAF; outsideElement = Some(element.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    });
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(node)
}

fn setTrieGetLeafElement(mut node: Arc<SetTrieNode>, mut face: Face) -> Result<ConnectorElement> {
    let mut element: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    element = (::match_deref::match_deref! { match &((face.clone(), node.clone())) {
        (DAE::Connect::Face::INSIDE, Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { insideElement: Some(e), .. }) => {
            e.clone()
        },
        (DAE::Connect::Face::OUTSIDE, Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { outsideElement: Some(e), .. }) => {
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

fn setTrieAdd(mut element: ConnectorElement, mut trie: Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> {
    let mut trie: Arc<SetTrieNode> = trie;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut el_cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut el: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    cref = getElementName(element.clone())?;
    el_cr = ComponentReferenceBasics::crefLastCref(cref.clone())?;
    el = setElementName(element.clone(), el_cr.clone());
    trie = setTrieUpdate(cref.clone(), el.clone(), trie.clone(), (std::sync::Arc::new(setTrieAddLeafElement) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>))?;
    Ok(trie)
}

fn updateSetLeaf<Arg: Clone + 'static>(mut sets: Sets, mut cref: Arc<DAE::ComponentRef>, mut arg: Arg, mut updateFunc: Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>) -> Result<Sets> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>;

    let mut sets: Sets = sets;
    sets.sets = setTrieUpdate(cref.clone(), arg.clone(), sets.sets.clone(), updateFunc.clone())?;
    Ok(sets)
}

fn setTrieUpdate<Arg: Clone + 'static>(mut cref: Arc<DAE::ComponentRef>, mut arg: Arg, mut trie: Arc<SetTrieNode>, mut updateFunc: Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>) -> Result<Arc<SetTrieNode>> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>;

    let mut trie: Arc<SetTrieNode> = trie;
    let _ = (::match_deref::match_deref! { match &((cref.clone(), trie.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. }) => {
            let mut id: ArcStr = arcstr::literal!("");
            id = (ComponentReferenceBasics::printComponentRef2Str((var_field!((*cref).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), var_field!((*cref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?).clone();
            assign_variant_field!(trie => SetTrieNode::SET_TRIE_NODE; nodes = setTrieUpdateNode((id.clone()).clone(), cref.clone(), var_field!((*cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), arg.clone(), updateFunc.clone(), var_field!((*trie).nodes, SetTrieNode::SET_TRIE_NODE).clone())?);
            ()
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. }) => {
            let mut id: ArcStr = arcstr::literal!("");
            id = (ComponentReferenceBasics::printComponentRef2Str((var_field!((*cref).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), var_field!((*cref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?).clone();
            assign_variant_field!(trie => SetTrieNode::SET_TRIE_NODE; nodes = setTrieUpdateLeaf((id.clone()).clone(), arg.clone(), var_field!((*trie).nodes, SetTrieNode::SET_TRIE_NODE).clone(), updateFunc.clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(trie)
}

fn setTrieUpdateNode<Arg: Clone + 'static>(mut id: ArcStr, mut wholeCref: Arc<DAE::ComponentRef>, mut cref: Arc<DAE::ComponentRef>, mut arg: Arg, mut updateFunc: Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>, mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>>) -> Result<Arc<metamodelica::List<Arc<SetTrieNode>>>> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>;

    let mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>> = nodes;
    let mut node2: Arc<SetTrieNode>;
    let mut n: i32 = 1;
    for mut node in &*nodes.clone() {
        let mut node = node.clone();
        if setTrieIsNode(node.clone()) && setTrieNodeName(node.clone())? == id.clone() {
            node2 = setTrieUpdate(cref.clone(), arg.clone(), node.clone(), updateFunc.clone())?;
            nodes = List::replaceAt(node2.clone(), n.clone(), nodes.clone())?;
            return Ok(nodes);
        } else {
            n = n.clone() + 1;
        }
    }
    nodes = setTrieUpdateNode2(wholeCref.clone(), arg.clone(), updateFunc.clone(), nodes.clone())?;
    Ok(nodes)
}

fn setTrieUpdateNode2<Arg: Clone + 'static>(mut cref: Arc<DAE::ComponentRef>, mut arg: Arg, mut updateFunc: Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>, mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>>) -> Result<Arc<metamodelica::List<Arc<SetTrieNode>>>> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>;

    let mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>> = nodes;
    nodes = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            let mut id: ArcStr = arcstr::literal!("");
            let mut node: Arc<SetTrieNode>;
            id = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
            node = Arc::new(SetTrieNode::SET_TRIE_LEAF { name: (id.clone()).clone(), insideElement: None, outsideElement: None, flowAssociation: None, connectCount: 0 });
            node = updateFunc(arg.clone(), node.clone())?;
            cons(node.clone(), nodes.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            let mut id: ArcStr = arcstr::literal!("");
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut child_nodes: Arc<metamodelica::List<Arc<SetTrieNode>>> = metamodelica::nil();
            cr = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
            id = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            child_nodes = setTrieUpdateNode2(var_field!((*cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), arg.clone(), updateFunc.clone(), metamodelica::nil())?;
            cons(Arc::new(SetTrieNode::SET_TRIE_NODE { name: (id.clone()).clone(), cref: cr.clone(), nodes: child_nodes.clone(), connectCount: 0 }), nodes.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(nodes)
}

fn setTrieUpdateLeaf<Arg: Clone + 'static>(mut id: ArcStr, mut arg: Arg, mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>>, mut updateFunc: Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>) -> Result<Arc<metamodelica::List<Arc<SetTrieNode>>>> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arg, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>;

    let mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>> = nodes;
    let mut n: i32 = 1;
    for mut node in &*nodes.clone() {
        let mut node = node.clone();
        if setTrieNodeName(node.clone())? == id.clone() {
            nodes = List::replaceAt(updateFunc(arg.clone(), node.clone())?, n.clone(), nodes.clone())?;
            return Ok(nodes);
        }
        n = n.clone() + 1;
    }
    nodes = cons(updateFunc(arg.clone(), Arc::new(SetTrieNode::SET_TRIE_LEAF { name: (id.clone()).clone(), insideElement: None, outsideElement: None, flowAssociation: None, connectCount: 0 }))?, nodes.clone());
    Ok(nodes)
}

pub fn traverseSets<Arg: Clone + 'static>(mut sets: Sets, mut arg: Arg, mut updateFunc: Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, Arg) -> Result<(Arc<SetTrieNode>, Arg)> + 'static>) -> Result<(Sets, Arg)> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, Arg) -> Result<(Arc<SetTrieNode>, Arg)> + 'static>;

    let mut sets: Sets = sets;
    let mut arg: Arg = arg;
    let mut node: Arc<SetTrieNode>;
    (node, arg) = setTrieTraverseLeaves(sets.sets.clone(), updateFunc.clone(), arg.clone())?;
    sets.sets = node.clone();
    Ok((sets, arg))
}

fn setTrieTraverseLeaves<Arg: Clone + 'static>(mut node: Arc<SetTrieNode>, mut updateFunc: Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, Arg) -> Result<(Arc<SetTrieNode>, Arg)> + 'static>, mut arg: Arg) -> Result<(Arc<SetTrieNode>, Arg)> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, Arg) -> Result<(Arc<SetTrieNode>, Arg)> + 'static>;

    let mut node: Arc<SetTrieNode> = node;
    let mut arg: Arg = arg;
    let _ = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => {
            let mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>> = metamodelica::nil();
            (nodes, arg) = List::map1Fold(var_field!((*node).nodes, SetTrieNode::SET_TRIE_NODE).clone(), (std::sync::Arc::new(setTrieTraverseLeaves) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, _, _) -> Result<_> + 'static>), updateFunc.clone(), arg.clone());
            assign_variant_field!(node => SetTrieNode::SET_TRIE_NODE; nodes = nodes.clone());
            ()
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            (node, arg) = updateFunc(node.clone(), arg.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((node, arg))
}

fn setTrieGet(mut cref: Arc<DAE::ComponentRef>, mut trie: Arc<SetTrieNode>, mut matchPrefix: bool) -> Result<Arc<SetTrieNode>> {
    let mut leaf: Arc<SetTrieNode>;
    let mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>> = metamodelica::nil();
    let mut subs_str: ArcStr = arcstr::literal!("");
    let mut id_subs: ArcStr = arcstr::literal!("");
    let mut id_nosubs: ArcStr = arcstr::literal!("");
    let mut node: Arc<SetTrieNode>;
    let __pa0 = ::match_deref::match_deref! { match &(trie.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { nodes: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    nodes = __pa0.clone();
    id_nosubs = (ComponentReferenceBasics::crefFirstIdent(cref.clone())?).clone();
    subs_str = (List::toString(ComponentReference::crefFirstSubs(cref.clone()), (std::sync::Arc::new(ExpressionBasics::printSubscriptStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("[")).clone(), (literal!(",")).clone(), (literal!("]")).clone(), false, 0)?).clone();
    id_subs = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id_nosubs.clone()); __mm_s.push_str(&*subs_str.clone()); ArcStr::from(__mm_s) }).clone();
    match '__try1: {
        leaf = unwrap_break_err!(setTrieGetNode((id_subs.clone()).clone(), nodes.clone()), '__try1);
        Ok::<_, anyhow::Error>((leaf.clone(),))
    } {
        Ok((__try1_o0,)) => {
            leaf = __try1_o0;
        }
        Err(_) => {
            leaf = setTrieGetNode((id_nosubs.clone()).clone(), nodes.clone())?;
        }
    }
    if !(ComponentReference::crefIsIdent(cref.clone())) {
        match '__try2: {
            leaf = unwrap_break_err!(setTrieGet(ComponentReference::crefRest(cref.clone())?, leaf.clone(), matchPrefix.clone()), '__try2);
            Ok::<_, anyhow::Error>((leaf.clone(),))
        } {
            Ok((__try2_o0,)) => {
                leaf = __try2_o0;
            }
            Err(_) => {
                let true = (matchPrefix.clone() && !(setTrieIsNode(leaf.clone()))) else { bail!("pattern mismatch") };
                bail!("try/else: outputs not set in else branch");
            }
        }
    }
    Ok(leaf)
}

fn setTrieGetNode(mut id: ArcStr, mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>>) -> Result<Arc<SetTrieNode>> {
    let mut node: Arc<SetTrieNode>;
    node = List::getMemberOnTrue((id.clone()).clone(), nodes.clone(), (std::sync::Arc::new(fnptr!(setTrieNodeNamed, ArcStr, Arc<SetTrieNode>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<SetTrieNode>) -> Result<bool> + 'static>))?;
    Ok(node)
}

fn setTrieNodeNamed(mut id: ArcStr, mut node: Arc<SetTrieNode>) -> bool {
    let mut isNamed: bool = false;
    isNamed = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => id.clone() == var_field!((*node).name, SetTrieNode::SET_TRIE_NODE).clone(),
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => id.clone() == var_field!((*node).name, SetTrieNode::SET_TRIE_LEAF).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNamed
}

fn setTrieGetLeaf(mut id: ArcStr, mut nodes: Arc<metamodelica::List<Arc<SetTrieNode>>>) -> Result<Arc<SetTrieNode>> {
    let mut node: Arc<SetTrieNode>;
    node = List::getMemberOnTrue((id.clone()).clone(), nodes.clone(), (std::sync::Arc::new(fnptr!(setTrieLeafNamed, ArcStr, Arc<SetTrieNode>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<SetTrieNode>) -> Result<bool> + 'static>))?;
    Ok(node)
}

fn setTrieLeafNamed(mut id: ArcStr, mut node: Arc<SetTrieNode>) -> bool {
    let mut isNamed: bool = false;
    isNamed = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => id.clone() == var_field!((*node).name, SetTrieNode::SET_TRIE_LEAF).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNamed
}

fn setTrieIsNode(mut node: Arc<SetTrieNode>) -> bool {
    let mut isNode: bool = false;
    isNode = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNode
}

pub fn equations(mut topScope: bool, mut sets: Sets, mut DAE: DAE::DAElist, mut connectionGraph: ConnectionGraph::ConnectionGraph, mut modelNameQualified: ArcStr) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE;
    let mut set_list: Arc<metamodelica::List<Set>> = metamodelica::nil();
    let mut set_array: metamodelica::Array<Set>;
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut dae2: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut has_stream: bool = false;
    let mut has_expandable: bool = false;
    let mut has_cardinality: bool = false;
    let mut broken: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>)>> = metamodelica::nil();
    let mut connected: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>)>> = metamodelica::nil();
    openmodelica_util::Globals::isInStream.with(|__root| *__root.borrow_mut() = None);
    if !(topScope.clone()) {
        return Ok(DAE);
    }
    set_array = generateSetArray(sets.clone())?;
    set_list = Arc::new(set_array.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    if daeHasExpandableConnectors(DAE.clone())? {
        (set_list, dae) = removeUnusedExpandableVariablesAndConnections(set_list.clone(), DAE.clone())?;
    } else {
        dae = DAE.clone();
    }
    (dae, connected, broken) = ConnectionGraph::handleOverconstrainedConnections(connectionGraph.clone(), (modelNameQualified.clone()).clone(), dae.clone())?;
    dae2 = equationsDispatch(set_list.clone().reverse(), connected.clone(), broken.clone())?;
    DAE = DAEUtil::joinDaes(dae.clone(), dae2.clone())?;
    DAE = evaluateConnectionOperators(sets.clone(), set_array.clone(), DAE.clone())?;
    DAE = ConnectionGraph::addBrokenEqualityConstraintEquations(DAE.clone(), broken.clone())?;
    Ok(DAE)
}

fn getExpandableEquSetsAsCrefs(mut sets: Arc<metamodelica::List<Set>>) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> {
    let mut crefSets: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let mut cref_set: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    for mut set in &*sets.clone() {
        let mut set = set.clone();
        let _ = (match set.clone() {
        DAE::Connect::Set::SET { ty: DAE::Connect::ConnectorType::EQU, .. } => {
            cref_set = getAllEquCrefs(list![set.clone()]);
            if List::applyAndFold(cref_set.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(isExpandable, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), false) {
                crefSets = cons(cref_set.clone(), crefSets.clone());
            }
            ()
        },
        _ => (),
    });
    }
    crefSets
}

fn removeCrefsFromSets(mut sets: Arc<metamodelica::List<Set>>, mut nonUsefulExpandable: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Arc<metamodelica::List<Set>> {
    let mut sets: Arc<metamodelica::List<Set>> = sets;
    sets = List::select1(sets.clone(), (std::sync::Arc::new(fnptr!(removeCrefsFromSets2, Set, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Set, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), nonUsefulExpandable.clone());
    sets
}

fn removeCrefsFromSets2(mut set: Set, mut nonUsefulExpandable: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> bool {
    let mut isInSet: bool = false;
    let mut setCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    setCrefs = getAllEquCrefs(list![set.clone()]);
    lst = List::intersectionOnTrue(setCrefs.clone(), nonUsefulExpandable.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>));
    isInSet = lst.clone().is_empty();
    isInSet
}

fn mergeEquSetsAsCrefs(mut setsAsCrefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>> {
    let mut setsAsCrefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = setsAsCrefs;
    setsAsCrefs = (::match_deref::match_deref! { match &(setsAsCrefs.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: set, tail: Deref @ metamodelica::List::Nil } => {
            list![set.clone()]
        },
        Deref @ metamodelica::List::Cons { head: set, tail: rest } => {
            let mut sets: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
            let mut set = (*set).clone();
            let mut rest = (*rest).clone();
            (set, rest) = mergeWithRest(set.clone(), rest.clone(), metamodelica::nil())?;
            sets = mergeEquSetsAsCrefs(rest.clone())?;
            cons(set.clone(), sets.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(setsAsCrefs)
}

fn mergeWithRest(mut set: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut sets: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, mut acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>)> {
    let mut set: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = set;
    let mut sets: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = sets;
    (set, sets) = (::match_deref::match_deref! { match &((set.clone(), sets.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            (set.clone(), acc.clone().reverse())
        },
        (set1, Deref @ metamodelica::List::Cons { head: set2, tail: rest }) => {
            let mut b: bool = false;
            let mut rest = (*rest).clone();
            b = List::intersectionOnTrue(set1.clone(), set2.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>)).is_empty();
            set = if (!(b.clone())) {List::unionOnTrue(set1.clone(), set2.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))} else {set1.clone()};
            (set, rest) = mergeWithRest(set.clone(), rest.clone(), List::consOnTrue(b.clone(), set2.clone(), acc.clone()))?;
            (set.clone(), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((set, sets))
}

fn getOnlyExpandableConnectedCrefs(mut sets: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>) -> Arc<metamodelica::List<Arc<DAE::ComponentRef>>> {
    let mut usefulConnectedExpandable: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    for mut set in &*sets.clone() {
        let mut set = set.clone();
        if allCrefsAreExpandable(set.clone()) {
            usefulConnectedExpandable = listAppend(set.clone(), usefulConnectedExpandable.clone());
        }
    }
    usefulConnectedExpandable
}

pub fn allCrefsAreExpandable(mut connects: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> bool {
    let mut allAreExpandable: bool = false;
    for mut cr in &*connects.clone() {
        let mut cr = cr.clone();
        if !(isExpandable(cr.clone())) {
            allAreExpandable = false;
            return allAreExpandable;
        }
    }
    allAreExpandable = true;
    allAreExpandable
}

fn generateSetArray(mut sets: Sets) -> Result<metamodelica::Array<Set>> {
    let mut setArray: metamodelica::Array<Set>;
    setArray = arrayCreate(sets.setCount.clone(), Set::SET { ty: openmodelica_frontend_types::DAE::Connect::ConnectorType::NO_TYPE, elements: metamodelica::nil() });
    setArray = setArrayAddConnections(sets.connections.clone(), sets.setCount.clone(), setArray.clone())?;
    setArray = generateSetArray2(sets.sets.clone(), metamodelica::nil(), setArray.clone())?;
    Ok(setArray)
}

fn setArrayAddConnections(mut connections: Arc<metamodelica::List<(i32, i32)>>, mut setCount: i32, mut sets: metamodelica::Array<Set>) -> Result<metamodelica::Array<Set>> {
    let mut sets: metamodelica::Array<Set> = sets;
    let mut graph: SetGraph;
    graph = arrayCreate(setCount.clone(), metamodelica::nil());
    graph = List::fold(connections.clone(), (std::sync::Arc::new(addConnectionToGraph) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), graph.clone());
    let __range0 = 1..=(graph.clone().borrow().len() as i32);
    for mut i in __range0 {
        (sets, graph) = setArrayAddConnection(i.clone(), graph.borrow()[(i.clone()-1) as usize].clone(), sets.clone(), graph.clone())?;
    }
    Ok(sets)
}

fn addConnectionToGraph(mut connection: (i32, i32), mut graph: SetGraph) -> Result<SetGraph> {
    let mut graph: SetGraph = graph;
    let mut set1: i32 = 0;
    let mut set2: i32 = 0;
    let mut node1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut node2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (set1, set2) = connection.clone();
    node1 = graph.clone().borrow()[(set1.clone()-1) as usize].clone();
    graph = {let _arr = graph.clone(); _arr.borrow_mut()[(set1.clone()-1) as usize] = cons(set2.clone(), node1.clone()); _arr};
    node2 = graph.clone().borrow()[(set2.clone()-1) as usize].clone();
    graph = {let _arr = graph.clone(); _arr.borrow_mut()[(set2.clone()-1) as usize] = cons(set1.clone(), node2.clone()); _arr};
    Ok(graph)
}

fn setArrayAddConnection(mut set: i32, mut edges: Arc<metamodelica::List<i32>>, mut sets: metamodelica::Array<Set>, mut graph: SetGraph) -> Result<(metamodelica::Array<Set>, SetGraph)> {
    let mut sets: metamodelica::Array<Set> = sets;
    let mut graph: SetGraph = graph;
    let mut edge_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut e in &*edges.clone() {
        let mut e = e.clone();
        if e.clone() != set.clone() {
            sets = setArrayAddConnection2(e.clone(), set.clone(), sets.clone())?;
            edge_lst = graph.borrow()[(e.clone()-1) as usize].clone();
            {
                let __cell0 = metamodelica::nil();
                graph.clone().borrow_mut()[(e.clone()-1) as usize] = __cell0;
            }
            (sets, graph) = setArrayAddConnection(set.clone(), edge_lst.clone(), sets.clone(), graph.clone())?;
        }
    }
    Ok((sets, graph))
}

fn setArrayAddConnection2(mut setPointer: i32, mut setPointee: i32, mut sets: metamodelica::Array<Set>) -> Result<metamodelica::Array<Set>> {
    let mut sets: metamodelica::Array<Set> = sets;
    let mut set: Set;
    set = sets.borrow()[(setPointee.clone()-1) as usize].clone();
    sets = (match set.clone() {
        DAE::Connect::Set::SET { .. } => {let _arr = sets.clone(); _arr.borrow_mut()[(setPointer.clone()-1) as usize] = Set::SET_POINTER { index: setPointee.clone() }; _arr},
        DAE::Connect::Set::SET_POINTER { .. } => setArrayAddConnection2(setPointer.clone(), var_field!(set.index, Set::SET_POINTER).clone(), sets.clone())?,
        _ => bail!("match: no arm matched"),
    });
    Ok(sets)
}

fn generateSetArray2(mut sets: Arc<SetTrieNode>, mut prefix: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut setArray: metamodelica::Array<Set>) -> Result<metamodelica::Array<Set>> {
    let mut setArray: metamodelica::Array<Set> = setArray;
    setArray = (::match_deref::match_deref! { match &(sets.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { cref: Deref @ DAE::ComponentRef::WILD, .. } => {
            List::fold1(var_field!((*sets).nodes, SetTrieNode::SET_TRIE_NODE).clone(), (std::sync::Arc::new(generateSetArray2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, metamodelica::Array<Set>) -> Result<metamodelica::Array<Set>> + 'static>), prefix.clone(), setArray.clone())
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => {
            List::fold1(var_field!((*sets).nodes, SetTrieNode::SET_TRIE_NODE).clone(), (std::sync::Arc::new(generateSetArray2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, metamodelica::Array<Set>) -> Result<metamodelica::Array<Set>> + 'static>), cons(var_field!((*sets).cref, SetTrieNode::SET_TRIE_NODE).clone(), prefix.clone()), setArray.clone())
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { flowAssociation: flow_cr, outsideElement: oe, insideElement: ie, .. } => {
            let mut prefix_cr: Option<Arc<DAE::ComponentRef>> = None;
            let mut oe = (*oe).clone();
            let mut ie = (*ie).clone();
            ie = insertFlowAssociationInStreamElement(ie.clone(), flow_cr.clone())?;
            oe = insertFlowAssociationInStreamElement(oe.clone(), flow_cr.clone())?;
            prefix_cr = buildElementPrefix(prefix.clone())?;
            setArray = setArrayAddElement(ie.clone(), prefix_cr.clone(), setArray.clone())?;
            setArray = setArrayAddElement(oe.clone(), prefix_cr.clone(), setArray.clone())?;
            setArray.clone()
        },
        _ => {
            setArray.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(setArray)
}

fn insertFlowAssociationInStreamElement(mut element: Option<ConnectorElement>, mut flowCref: Option<Arc<DAE::ComponentRef>>) -> Result<Option<ConnectorElement>> {
    let mut element: Option<ConnectorElement> = element;
    let mut el: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    if isSome(element.clone()) {
        let Some(__pa0) = (element.clone()) else { bail!("pattern mismatch") };
        el = __pa0.clone();
        element = (::match_deref::match_deref! { match &(el.clone()) {
        ConnectorElement { ty: DAE::Connect::ConnectorType::STREAM { associatedFlow: None }, .. } => {
            el.ty = ConnectorType::STREAM { associatedFlow: flowCref.clone() };
            Some(el.clone())
        },
        _ => element.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(element)
}

fn setArrayAddElement(mut element: Option<ConnectorElement>, mut prefix: Option<Arc<DAE::ComponentRef>>, mut sets: metamodelica::Array<Set>) -> Result<metamodelica::Array<Set>> {
    let mut sets: metamodelica::Array<Set> = sets;
    sets = (::match_deref::match_deref! { match &((element.clone(), prefix.clone())) {
        (None, _) => {
            sets.clone()
        },
        (Some(el @ ConnectorElement { .. }), None) => {
            setArrayUpdate(sets.clone(), el.set.clone(), el.clone())?
        },
        (Some(el @ ConnectorElement { .. }), Some(prefix_cr)) => {
            let mut el = (*el).clone();
            el.name = ComponentReference::joinCrefs(prefix_cr.clone(), el.name.clone())?;
            setArrayUpdate(sets.clone(), el.set.clone(), el.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sets)
}

fn buildElementPrefix(mut prefix: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Option<Arc<DAE::ComponentRef>>> {
    let mut cref: Option<Arc<DAE::ComponentRef>> = None;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut id: ArcStr = arcstr::literal!("");
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    if prefix.clone().is_empty() {
        cref = None;
    } else {
        cr = listHead(prefix.clone())?;
        for mut c in &*listRest(prefix.clone())? {
            let mut c = c.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(c.clone()) {
                Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: __pa0, ident: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            subs = __pa0.clone();
            id = __pa1.clone();
            cr = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: subs.clone(), componentRef: cr.clone() });
        }
        cref = Some(cr.clone());
    }
    Ok(cref)
}

fn setArrayUpdate(mut sets: metamodelica::Array<Set>, mut index: i32, mut element: ConnectorElement) -> Result<metamodelica::Array<Set>> {
    let mut sets: metamodelica::Array<Set> = sets;
    let mut set: Set;
    let mut el: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    set = sets.borrow()[(index.clone()-1) as usize].clone();
    sets = (match (set.clone(), element.clone()) {
        (DAE::Connect::Set::SET { .. }, ConnectorElement { .. }) => {
            if Config::orderConnections()? && isEquType(element.ty.clone()) {
                el = List::mergeSorted(list![element.clone()], var_field!(set.elements, Set::SET).clone(), (std::sync::Arc::new(equSetElementLess) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, ConnectorElement) -> Result<bool> + 'static>))?;
            } else {
                el = cons(element.clone(), var_field!(set.elements, Set::SET).clone());
            }
            {let _arr = sets.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = Set::SET { ty: element.ty.clone(), elements: el.clone() }; _arr}
        },
        (DAE::Connect::Set::SET_POINTER { .. }, _) => setArrayUpdate(sets.clone(), var_field!(set.index, Set::SET_POINTER).clone(), element.clone())?,
        _ => bail!("match: no arm matched"),
    });
    Ok(sets)
}

fn equSetElementLess(mut element1: ConnectorElement, mut element2: ConnectorElement) -> Result<bool> {
    let mut isLess: bool = false;
    isLess = ComponentReferenceBasics::crefSortFunc(element2.name.clone(), element1.name.clone())?;
    Ok(isLess)
}

fn setArrayGet(mut setArray: metamodelica::Array<Set>, mut index: i32) -> Result<Set> {
    let mut set: Set;
    set = setArray.borrow()[(index.clone()-1) as usize].clone();
    set = (match set.clone() {
        DAE::Connect::Set::SET { .. } => set.clone(),
        DAE::Connect::Set::SET_POINTER { .. } => setArrayGet(setArray.clone(), var_field!(set.index, Set::SET_POINTER).clone())?,
        _ => bail!("match: no arm matched"),
    });
    Ok(set)
}

fn equationsDispatch(mut sets: Arc<metamodelica::List<Set>>, mut connected: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>)>>, mut broken: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>)>>) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE.emptyDae.clone();
    let mut eql: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    let mut eqll: Arc<metamodelica::List<Arc<metamodelica::List<ConnectorElement>>>> = metamodelica::nil();
    let mut flowThreshold: metamodelica::Real = Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())?;
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    for mut set in &*sets.clone() {
        let mut set = set.clone();
        DAE = (match set.clone() {
        DAE::Connect::Set::SET_POINTER { .. } => DAE.clone(),
        DAE::Connect::Set::SET { ty: DAE::Connect::ConnectorType::EQU, .. } => {
            eqll = ConnectionGraph::removeBrokenConnects(var_field!(set.elements, Set::SET).clone(), connected.clone(), broken.clone())?;
            for mut eql in &*eqll.clone() {
                let mut eql = eql.clone();
                DAE = DAEUtil::joinDaes(generateEquEquations(eql.clone())?, DAE.clone())?;
            }
            DAE.clone()
        },
        DAE::Connect::Set::SET { elements: ref eql, ty: DAE::Connect::ConnectorType::FLOW } => DAEUtil::joinDaes(generateFlowEquations(eql.clone())?, DAE.clone())?,
        DAE::Connect::Set::SET { elements: ref eql, ty: DAE::Connect::ConnectorType::STREAM { .. } } => DAEUtil::joinDaes(generateStreamEquations(eql.clone(), flowThreshold.clone())?, DAE.clone())?,
        DAE::Connect::Set::SET { ty: DAE::Connect::ConnectorType::NO_TYPE, .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ConnectUtil.equationsDispatch failed on connection set with no type.")).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ConnectUtil.equationsDispatch failed because of unknown reason.")).clone()])?;
            bail!("fail")
        },
    });
    }
    Ok(DAE)
}

fn generateEquEquations(mut elements: Arc<metamodelica::List<ConnectorElement>>) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE.emptyDae.clone();
    let mut eql: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut e1: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut x_src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut y_src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut x: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut y: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    if elements.clone().is_empty() {
        return Ok(DAE);
    }
    e1 = listHead(elements.clone())?;
    if Config::orderConnections()? {
        for mut e2 in &*listRest(elements.clone())? {
            let mut e2 = e2.clone();
            src = ElementSource::mergeSources(e1.source.clone(), e2.source.clone())?;
            src = ElementSource::addElementSourceConnect(src.clone(), (e1.name.clone(), e2.name.clone()))?;
            eql = cons(Arc::new(DAE::Element::EQUEQUATION { cr1: e1.name.clone(), cr2: e2.name.clone(), source: src.clone() }), eql.clone());
        }
    } else {
        for mut e2 in &*listRest(elements.clone())? {
            let mut e2 = e2.clone();
            (x, y) = Util::swap(shouldFlipEquEquation(e1.name.clone(), e1.source.clone())?, e1.name.clone(), e2.name.clone());
            src = ElementSource::mergeSources(e1.source.clone(), e2.source.clone())?;
            src = ElementSource::addElementSourceConnect(src.clone(), (x.clone(), y.clone()))?;
            eql = cons(Arc::new(DAE::Element::EQUEQUATION { cr1: x.clone(), cr2: y.clone(), source: src.clone() }), eql.clone());
            e1 = e2.clone();
        }
    }
    DAE = DAE::DAElist { elementLst: eql.clone().reverse() };
    Ok(DAE)
}

fn shouldFlipEquEquation(mut lhsCref: Arc<DAE::ComponentRef>, mut lhsSource: Arc<DAE::ElementSource>) -> Result<bool> {
    let mut shouldFlip: bool = false;
    shouldFlip = (::match_deref::match_deref! { match &(lhsSource.clone()) {
        Deref @ DAE::ElementSource { connectEquationOptLst: Deref @ metamodelica::List::Cons { head: (lhs, _), tail: _ }, .. } => {
            !(ComponentReferenceBasics::crefPrefixOf(lhs.clone(), lhsCref.clone())?)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(shouldFlip)
}

fn generateFlowEquations(mut elements: Arc<metamodelica::List<ConnectorElement>>) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut sum: Arc<DAE::Exp>;
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    sum = makeFlowExp(listHead(elements.clone())?)?;
    src = getElementSource(listHead(elements.clone())?)?;
    for mut e in &*listRest(elements.clone())? {
        let mut e = e.clone();
        sum = Expression::makeRealAdd(sum.clone(), makeFlowExp(e.clone())?);
        src = ElementSource::mergeSources(src.clone(), e.source.clone())?;
    }
    DAE = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::EQUATION { exp: sum.clone(), scalar: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), source: src.clone() })] };
    Ok(DAE)
}

fn makeFlowExp(mut element: ConnectorElement) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = Expression::crefExp(element.name.clone())?;
    if isOutsideElement(element.clone()) {
        exp = Expression::negateReal(exp.clone());
    }
    Ok(exp)
}

pub fn increaseConnectRefCount(mut lhsCref: Arc<DAE::ComponentRef>, mut rhsCref: Arc<DAE::ComponentRef>, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    if System::getUsesCardinality() {
        crefs = ComponentReference::expandCref(lhsCref.clone(), false)?;
        sets.sets = increaseConnectRefCount2(crefs.clone(), sets.sets.clone())?;
        crefs = ComponentReference::expandCref(rhsCref.clone(), false)?;
        sets.sets = increaseConnectRefCount2(crefs.clone(), sets.sets.clone())?;
    }
    Ok(sets)
}

pub fn increaseConnectRefCount2(mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut sets: Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> {
    let mut sets: Arc<SetTrieNode> = sets;
    for mut cr in &*crefs.clone() {
        let mut cr = cr.clone();
        sets = setTrieUpdate(cr.clone(), 1, sets.clone(), (std::sync::Arc::new(increaseRefCount) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> + 'static>))?;
    }
    Ok(sets)
}

fn increaseRefCount(mut amount: i32, mut node: Arc<SetTrieNode>) -> Result<Arc<SetTrieNode>> {
    let mut node: Arc<SetTrieNode> = node;
    let _ = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => {
            assign_variant_field!(node => SetTrieNode::SET_TRIE_NODE; connectCount = var_field!((*node).connectCount, SetTrieNode::SET_TRIE_NODE).clone() + amount.clone());
            ()
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            assign_variant_field!(node => SetTrieNode::SET_TRIE_LEAF; connectCount = var_field!((*node).connectCount, SetTrieNode::SET_TRIE_LEAF).clone() + amount.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(node)
}

fn generateStreamEquations(mut elements: Arc<metamodelica::List<ConnectorElement>>, mut flowThreshold: metamodelica::Real) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    DAE = (::match_deref::match_deref! { match &(elements.clone()) {
        Deref @ metamodelica::List::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::List::Nil } => {
            DAE.emptyDae.clone()
        },
        Deref @ metamodelica::List::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::List::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::List::Nil } } => {
            DAE.emptyDae.clone()
        },
        Deref @ metamodelica::List::Cons { head: ConnectorElement { source: src1, face: DAE::Connect::Face::OUTSIDE, name: cr1, .. }, tail: Deref @ metamodelica::List::Cons { head: ConnectorElement { source: src2, face: DAE::Connect::Face::OUTSIDE, name: cr2, .. }, tail: Deref @ metamodelica::List::Nil } } => {
            let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
            let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
            let mut cref1: Arc<DAE::Exp>;
            let mut cref2: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            cref1 = Expression::crefExp(cr1.clone())?;
            cref2 = Expression::crefExp(cr2.clone())?;
            e1 = makeInStreamCall(cref2.clone())?;
            e2 = makeInStreamCall(cref1.clone())?;
            src = ElementSource::mergeSources(src1.clone(), src2.clone())?;
            dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::EQUATION { exp: cref1.clone(), scalar: e1.clone(), source: src.clone() }), Arc::new(DAE::Element::EQUATION { exp: cref2.clone(), scalar: e2.clone(), source: src.clone() })] };
            dae.clone()
        },
        Deref @ metamodelica::List::Cons { head: ConnectorElement { source: src1, name: cr1, .. }, tail: Deref @ metamodelica::List::Cons { head: ConnectorElement { source: src2, name: cr2, .. }, tail: Deref @ metamodelica::List::Nil } } => {
            let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
            let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            src = ElementSource::mergeSources(src1.clone(), src2.clone())?;
            e1 = Expression::crefExp(cr1.clone())?;
            e2 = Expression::crefExp(cr2.clone())?;
            dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: src.clone() })] };
            dae.clone()
        },
        _ => {
            let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
            let mut inside: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
            let mut outside: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
            (outside, inside) = List::splitOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(isOutsideElement, ConnectorElement)) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement) -> Result<bool> + 'static>));
            dae = streamEquationGeneral(outside.clone(), inside.clone(), flowThreshold.clone())?;
            dae.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(DAE)
}

fn isOutsideElement(mut element: ConnectorElement) -> bool {
    let mut isOutside: bool = false;
    isOutside = (match element.clone() {
        ConnectorElement { face: DAE::Connect::Face::OUTSIDE, .. } => true,
        _ => false,
    });
    isOutside
}

fn isZeroFlowMinMax(mut streamCref: Arc<DAE::ComponentRef>, mut element: ConnectorElement) -> Result<bool> {
    let mut isZero: bool = false;
    if compareCrefStreamSet(streamCref.clone(), element.clone())? {
        isZero = false;
    } else if isOutsideElement(element.clone()) {
        isZero = isZeroFlow(element.clone(), (literal!("max")).clone())?;
    } else {
        isZero = isZeroFlow(element.clone(), (literal!("min")).clone())?;
    }
    Ok(isZero)
}

fn isZeroFlow(mut element: ConnectorElement, mut attr: ArcStr) -> Result<bool> {
    let mut isZero: bool = false;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut attr_oexp: Option<Arc<DAE::Exp>> = None;
    let mut flow_exp: Arc<DAE::Exp>;
    let mut attr_exp: Arc<DAE::Exp>;
    flow_exp = flowExp(element.clone())?;
    ty = Expression::r#typeof(flow_exp.clone())?;
    attr_oexp = Types::lookupAttributeExp(Types::getAttributes(ty.clone()), (attr.clone()).clone())?;
    if isSome(attr_oexp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(attr_oexp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        attr_exp = __pa0.clone();
        isZero = Expression::isZero(attr_exp.clone());
    } else {
        isZero = false;
    }
    Ok(isZero)
}

fn streamEquationGeneral(mut outsideElements: Arc<metamodelica::List<ConnectorElement>>, mut insideElements: Arc<metamodelica::List<ConnectorElement>>, mut flowThreshold: metamodelica::Real) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut outside: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    let mut cref_exp: Arc<DAE::Exp>;
    let mut res: Arc<DAE::Exp>;
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut eql: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    for mut e in &*outsideElements.clone() {
        let mut e = e.clone();
        cref_exp = Expression::crefExp(e.name.clone())?;
        outside = removeStreamSetElement(e.name.clone(), outsideElements.clone())?;
        res = streamSumEquationExp(outside.clone(), insideElements.clone(), flowThreshold.clone())?;
        src = ElementSource::addAdditionalComment(e.source.clone(), (literal!(" equation generated by stream handling")).clone())?;
        eql = cons(Arc::new(DAE::Element::EQUATION { exp: cref_exp.clone(), scalar: res.clone(), source: src.clone() }), eql.clone());
    }
    DAE = DAE::DAElist { elementLst: eql.clone() };
    Ok(DAE)
}

fn streamSumEquationExp(mut outsideElements: Arc<metamodelica::List<ConnectorElement>>, mut insideElements: Arc<metamodelica::List<ConnectorElement>>, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    let mut sumExp: Arc<DAE::Exp>;
    let mut outside_sum1: Arc<DAE::Exp>;
    let mut outside_sum2: Arc<DAE::Exp>;
    let mut inside_sum1: Arc<DAE::Exp>;
    let mut inside_sum2: Arc<DAE::Exp>;
    let mut res: Arc<DAE::Exp>;
    if outsideElements.clone().is_empty() {
        inside_sum1 = sumMap(insideElements.clone(), (std::sync::Arc::new(sumInside1) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>), flowThreshold.clone())?;
        inside_sum2 = sumMap(insideElements.clone(), (std::sync::Arc::new(sumInside2) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>), flowThreshold.clone())?;
        sumExp = Expression::expDiv(inside_sum1.clone(), inside_sum2.clone())?;
    } else if insideElements.clone().is_empty() {
        outside_sum1 = sumMap(outsideElements.clone(), (std::sync::Arc::new(sumOutside1) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>), flowThreshold.clone())?;
        outside_sum2 = sumMap(outsideElements.clone(), (std::sync::Arc::new(sumOutside2) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>), flowThreshold.clone())?;
        sumExp = Expression::expDiv(outside_sum1.clone(), outside_sum2.clone())?;
    } else {
        outside_sum1 = sumMap(outsideElements.clone(), (std::sync::Arc::new(sumOutside1) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>), flowThreshold.clone())?;
        outside_sum2 = sumMap(outsideElements.clone(), (std::sync::Arc::new(sumOutside2) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>), flowThreshold.clone())?;
        inside_sum1 = sumMap(insideElements.clone(), (std::sync::Arc::new(sumInside1) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>), flowThreshold.clone())?;
        inside_sum2 = sumMap(insideElements.clone(), (std::sync::Arc::new(sumInside2) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>), flowThreshold.clone())?;
        sumExp = Expression::expDiv(Expression::expAdd(outside_sum1.clone(), inside_sum1.clone())?, Expression::expAdd(outside_sum2.clone(), inside_sum2.clone())?)?;
    }
    Ok(sumExp)
}

fn sumMap(mut elements: Arc<metamodelica::List<ConnectorElement>>, mut func: Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>;

    let mut exp: Arc<DAE::Exp>;
    exp = todo!("reduction Expression.expAdd: cannot resolve default value");
    Ok(exp)
}

fn streamFlowExp(mut element: ConnectorElement) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut streamExp: Arc<DAE::Exp>;
    let mut flowExp: Arc<DAE::Exp>;
    let mut flow_cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let ConnectorElement { ty: ConnectorType::STREAM { associatedFlow: Some(__pa0) }, .. } = (element.clone()) else { bail!("pattern mismatch") };
    flow_cr = __pa0.clone();
    streamExp = Expression::crefExp(element.name.clone())?;
    flowExp = Expression::crefExp(flow_cr.clone())?;
    Ok((streamExp, flowExp))
}

fn flowExp(mut element: ConnectorElement) -> Result<Arc<DAE::Exp>> {
    let mut flowExp: Arc<DAE::Exp>;
    let mut flow_cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let ConnectorElement { ty: ConnectorType::STREAM { associatedFlow: Some(__pa0) }, .. } = (element.clone()) else { bail!("pattern mismatch") };
    flow_cr = __pa0.clone();
    flowExp = Expression::crefExp(flow_cr.clone())?;
    Ok(flowExp)
}

fn sumOutside1(mut element: ConnectorElement, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut stream_exp: Arc<DAE::Exp>;
    let mut flow_exp: Arc<DAE::Exp>;
    let mut flow_threshold: Arc<DAE::Exp>;
    (stream_exp, flow_exp) = streamFlowExp(element.clone())?;
    flow_threshold = Arc::new(DAE::Exp::RCONST { real: flowThreshold.clone() });
    exp = Expression::expMul(makePositiveMaxCall(flow_exp.clone(), flow_threshold.clone())?, makeInStreamCall(stream_exp.clone())?)?;
    Ok(exp)
}

fn sumInside1(mut element: ConnectorElement, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut stream_exp: Arc<DAE::Exp>;
    let mut flow_exp: Arc<DAE::Exp>;
    let mut flow_threshold: Arc<DAE::Exp>;
    let mut flowTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut streamTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (stream_exp, flow_exp) = streamFlowExp(element.clone())?;
    flowTy = Expression::r#typeof(flow_exp.clone())?;
    flow_exp = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: flowTy.clone() }, exp: flow_exp.clone() });
    flow_threshold = Arc::new(DAE::Exp::RCONST { real: flowThreshold.clone() });
    exp = Expression::expMul(makePositiveMaxCall(flow_exp.clone(), flow_threshold.clone())?, stream_exp.clone())?;
    Ok(exp)
}

fn sumOutside2(mut element: ConnectorElement, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut flow_exp: Arc<DAE::Exp>;
    flow_exp = flowExp(element.clone())?;
    exp = makePositiveMaxCall(flow_exp.clone(), Arc::new(DAE::Exp::RCONST { real: flowThreshold.clone() }))?;
    Ok(exp)
}

fn sumInside2(mut element: ConnectorElement, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut flow_exp: Arc<DAE::Exp>;
    let mut flowTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    flow_exp = flowExp(element.clone())?;
    flowTy = Expression::r#typeof(flow_exp.clone())?;
    flow_exp = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: flowTy.clone() }, exp: flow_exp.clone() });
    exp = makePositiveMaxCall(flow_exp.clone(), Arc::new(DAE::Exp::RCONST { real: flowThreshold.clone() }))?;
    Ok(exp)
}

pub fn faceEqual(mut face1: Face, mut face2: Face) -> bool {
    let mut sameFaces: bool = metamodelica::valueConstructor((&face1.clone())).unwrap() == metamodelica::valueConstructor((&face2.clone())).unwrap();
    sameFaces
}

fn makeInStreamCall(mut streamExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut inStreamCall: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Expression::r#typeof(streamExp.clone())?;
    inStreamCall = Expression::makeBuiltinCall((literal!("inStream")).clone(), list![streamExp.clone()], ty.clone(), false);
    Ok(inStreamCall)
}

fn makePositiveMaxCall(mut flowExp: Arc<DAE::Exp>, mut flowThreshold: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut positiveMaxCall: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut attr: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut nominal_oexp: Option<Arc<DAE::Exp>> = None;
    let mut nominal_exp: Arc<DAE::Exp>;
    let mut flow_threshold: Arc<DAE::Exp>;
    ty = Expression::r#typeof(flowExp.clone())?;
    nominal_oexp = Types::lookupAttributeExp(Types::getAttributes(ty.clone()), (literal!("nominal")).clone())?;
    if isSome(nominal_oexp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(nominal_oexp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        nominal_exp = __pa0.clone();
        flow_threshold = Expression::expMul(flowThreshold.clone(), nominal_exp.clone())?;
    } else {
        flow_threshold = flowThreshold.clone();
    }
    positiveMaxCall = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$OMC$PositiveMax")).clone() }), expLst: list![flowExp.clone(), flow_threshold.clone()], attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
    openmodelica_util::Globals::isInStream.with(|__root| *__root.borrow_mut() = Some(true));
    Ok(positiveMaxCall)
}

fn evaluateConnectionOperators(mut sets: Sets, mut setArray: metamodelica::Array<Set>, mut DAE: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE;
    let mut flow_threshold: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut has_cardinality: bool = System::getUsesCardinality();
    if System::getHasStreamConnectors() || has_cardinality.clone() {
        flow_threshold = Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())?;
        (DAE, _, _) = DAEUtil::traverseDAE(DAE.clone(), Arc::new(openmodelica_frontend_dump::AvlTreePathFunction::Tree::EMPTY), Arc::new({ let __pe_b2 = setArray.clone(); let __pe_b3 = has_cardinality.clone(); let __pe_b4 = flow_threshold.clone(); move |__pe_a0, __pe_a1| evaluateConnectionOperators2(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }), sets.clone());
        DAE = simplifyDAEElements(has_cardinality.clone(), DAE.clone());
    }
    Ok(DAE)
}

fn evaluateConnectionOperators2(mut exp: Arc<DAE::Exp>, mut sets: Sets, mut setArray: metamodelica::Array<Set>, mut hasCardinality: bool, mut flowThreshold: metamodelica::Real) -> Result<(Arc<DAE::Exp>, Sets)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut sets: Sets = sets;
    let mut changed: bool = false;
    (exp, changed) = Expression::traverseExpBottomUp(exp.clone(), Arc::new({ let __pe_b1 = sets.clone(); let __pe_b2 = setArray.clone(); let __pe_b3 = flowThreshold.clone(); move |__pe_a0, __pe_a4| evaluateConnectionOperatorsExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), false)?;
    if changed.clone() && hasCardinality.clone() {
        (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
    }
    Ok((exp, sets))
}

fn evaluateConnectionOperatorsExp(mut exp: Arc<DAE::Exp>, mut sets: Sets, mut setArray: metamodelica::Array<Set>, mut flowThreshold: metamodelica::Real, mut changed: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut changed: bool = changed;
    (exp, changed) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" }, .. } => {
            let mut e: Arc<DAE::Exp>;
            e = evaluateInStream(cr.clone(), sets.clone(), setArray.clone(), flowThreshold.clone())?;
            (e.clone(), true)
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" }, .. } => {
            let mut e: Arc<DAE::Exp>;
            e = evaluateActualStream(cr.clone(), sets.clone(), setArray.clone(), flowThreshold.clone())?;
            (e.clone(), true)
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cardinality" }, .. } => {
            let mut e: Arc<DAE::Exp>;
            e = evaluateCardinality(cr.clone(), sets.clone());
            (e.clone(), true)
        },
        _ => {
            (exp.clone(), changed.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, changed))
}

fn mkArrayIfNeeded(mut ty: Arc<DAE::Type>, mut exp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp> = exp;
    exp = Expression::arrayFill(TypesDump::getDimensions(ty.clone()), exp.clone())?;
    Ok(exp)
}

fn evaluateInStream(mut streamCref: Arc<DAE::ComponentRef>, mut sets: Sets, mut setArray: metamodelica::Array<Set>, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut e: ConnectorElement = <ConnectorElement as ::std::default::Default>::default();
    let mut sl: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    let mut set: i32 = 0;
    match '__try0: {
        e = findElement(streamCref.clone(), openmodelica_frontend_types::DAE::Connect::Face::INSIDE, ConnectorType::STREAM { associatedFlow: None }, DAE::emptyElementSource().clone(), sets.clone());
        if unwrap_break_err!(isNewElement(e.clone()), '__try0) {
            sl = list![e.clone()];
        } else {
            let ConnectorElement { set: __pa1, .. } = (e.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            set = __pa1.clone();
            let Set::SET { elements: __pa2, ty: ConnectorType::STREAM { .. } } = (unwrap_break_err!(setArrayGet(setArray.clone(), set.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            sl = __pa2.clone();
        }
        exp = unwrap_break_err!(generateInStreamExp(streamCref.clone(), sl.clone(), sets.clone(), setArray.clone(), flowThreshold.clone()), '__try0);
        Ok::<_, anyhow::Error>((e.clone(), exp.clone(), sl.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            e = __try0_o0;
            exp = __try0_o1;
            sl = __try0_o2;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectUtil.evaluateInStream failed for ")); __mm_s.push_str(&*ComponentReference::crefStr(streamCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(exp)
}

fn generateInStreamExp(mut streamCref: Arc<DAE::ComponentRef>, mut streams: Arc<metamodelica::List<ConnectorElement>>, mut sets: Sets, mut setArray: metamodelica::Array<Set>, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut reducedStreams: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
    reducedStreams = List::filterOnFalse(streams.clone(), Arc::new({ let __pe_b0 = streamCref.clone(); move |__pe_a1| isZeroFlowMinMax(__pe_b0.clone(), __pe_a1) }));
    exp = (::match_deref::match_deref! { match &(reducedStreams.clone()) {
        Deref @ metamodelica::List::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, name: c, .. }, tail: Deref @ metamodelica::List::Nil } => {
            Expression::crefExp(c.clone())?
        },
        Deref @ metamodelica::List::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::List::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::List::Nil } } => {
            let mut c: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(removeStreamSetElement(streamCref.clone(), reducedStreams.clone())?) {
                Deref @ metamodelica::List::Cons { head: ConnectorElement { name: __pa0, .. }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            c = __pa0.clone();
            e = Expression::crefExp(c.clone())?;
            e.clone()
        },
        Deref @ metamodelica::List::Cons { head: ConnectorElement { face: f1, .. }, tail: Deref @ metamodelica::List::Cons { head: ConnectorElement { face: f2, .. }, tail: Deref @ metamodelica::List::Nil } } if (!(faceEqual(f1.clone(), f2.clone()))) => {
            let mut c: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(removeStreamSetElement(streamCref.clone(), reducedStreams.clone())?) {
                Deref @ metamodelica::List::Cons { head: ConnectorElement { name: __pa0, .. }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            c = __pa0.clone();
            e = evaluateInStream(c.clone(), sets.clone(), setArray.clone(), flowThreshold.clone())?;
            e.clone()
        },
        _ => {
            let mut e: Arc<DAE::Exp>;
            let mut expr: Arc<DAE::Exp>;
            let mut inside: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
            let mut outside: Arc<metamodelica::List<ConnectorElement>> = metamodelica::nil();
            (outside, inside) = List::splitOnTrue(reducedStreams.clone(), (std::sync::Arc::new(fnptr!(isOutsideElement, ConnectorElement)) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement) -> Result<bool> + 'static>));
            inside = removeStreamSetElement(streamCref.clone(), inside.clone())?;
            e = streamSumEquationExp(outside.clone(), inside.clone(), flowThreshold.clone())?;
            if !(inside.clone().is_empty()) {
                (expr, _) = streamFlowExp(listHead(inside.clone())?)?;
                e = Expression::makePureBuiltinCall((literal!("$OMC$inStreamDiv")).clone(), list![e.clone(), expr.clone()], Expression::r#typeof(e.clone())?);
            }
            (e, _) = evaluateConnectionOperators2(e.clone(), sets.clone(), setArray.clone(), false, flowThreshold.clone())?;
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn evaluateActualStream(mut streamCref: Arc<DAE::ComponentRef>, mut sets: Sets, mut setArray: metamodelica::Array<Set>, mut flowThreshold: metamodelica::Real) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut flow_cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    let mut flow_exp: Arc<DAE::Exp>;
    let mut stream_exp: Arc<DAE::Exp>;
    let mut instream_exp: Arc<DAE::Exp>;
    let mut rel_exp: Arc<DAE::Exp>;
    let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut flow_dir: i32 = 0;
    flow_cr = getStreamFlowAssociation(streamCref.clone(), sets.clone())?;
    ety = ComponentReference::crefLastType(flow_cr.clone())?;
    flow_dir = evaluateFlowDirection(ety.clone())?;
    if flow_dir.clone() == 1 {
        rel_exp = evaluateInStream(streamCref.clone(), sets.clone(), setArray.clone(), flowThreshold.clone())?;
    } else if flow_dir.clone() == -1 {
        rel_exp = Expression::crefExp(streamCref.clone())?;
    } else {
        flow_exp = Expression::crefExp(flow_cr.clone())?;
        stream_exp = Expression::crefExp(streamCref.clone())?;
        instream_exp = evaluateInStream(streamCref.clone(), sets.clone(), setArray.clone(), flowThreshold.clone())?;
        rel_exp = Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: flow_exp.clone(), operator: DAE::Operator::GREATER { ty: ety.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: instream_exp.clone(), expElse: stream_exp.clone() });
    }
    exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("smooth")).clone() }), expLst: list![Arc::new(DAE::Exp::ICONST { integer: 0 }), rel_exp.clone()], attr: DAE::callAttrBuiltinReal().clone() });
    Ok(exp)
}

fn evaluateFlowDirection(mut ty: Arc<DAE::Type>) -> Result<i32> {
    let mut direction: i32 = 0;
    let mut attr: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut min_oval: Option<Arc<Values::Value>> = None;
    let mut max_oval: Option<Arc<Values::Value>> = None;
    let mut min_val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut max_val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    attr = Types::getAttributes(ty.clone());
    if attr.clone().is_empty() {
        return Ok(direction);
    }
    min_oval = Types::lookupAttributeValue(attr.clone(), (literal!("min")).clone())?;
    max_oval = Types::lookupAttributeValue(attr.clone(), (literal!("max")).clone())?;
    direction = (::match_deref::match_deref! { match &((min_oval.clone(), max_oval.clone())) {
        (None, None) => 0,
        (Some(Deref @ Values::Value::REAL { real: min_val }), None) => if (min_val.clone() >= metamodelica::OrderedFloat((0) as f64)) {1} else {0},
        (None, Some(Deref @ Values::Value::REAL { real: max_val })) => if (max_val.clone() <= metamodelica::OrderedFloat((0) as f64)) {-1} else {0},
        (Some(Deref @ Values::Value::REAL { real: min_val }), Some(Deref @ Values::Value::REAL { real: max_val })) => if (min_val.clone() >= metamodelica::OrderedFloat((0) as f64) && max_val.clone() >= min_val.clone()) {1} else if (max_val.clone() <= metamodelica::OrderedFloat((0) as f64) && min_val.clone() <= max_val.clone()) {-1} else {0},
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(direction)
}

fn evaluateCardinality(mut cref: Arc<DAE::ComponentRef>, mut sets: Sets) -> Arc<DAE::Exp> {
    let mut exp: Arc<DAE::Exp>;
    exp = Arc::new(DAE::Exp::ICONST { integer: getConnectCount(cref.clone(), sets.sets.clone()) });
    exp
}

fn simplifyDAEElements(mut hasCardinality: bool, mut DAE: DAE::DAElist) -> DAE::DAElist {
    let mut DAE: DAE::DAElist = DAE;
    if hasCardinality.clone() {
        DAE = DAE::DAElist { elementLst: List::mapFlat(DAE.elementLst.clone(), (std::sync::Arc::new(simplifyDAEElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> + 'static>)) };
    }
    DAE
}

fn simplifyDAEElement(mut element: Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    elements = 'mc: {
        let __mc_input = element.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::IF_EQUATION { condition1: conds, equations2: branches, equations3: else_branch, .. } => {
                    Ok(simplifyDAEIfEquation(conds.clone(), branches.clone(), else_branch.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: conds, equations2: branches, equations3: else_branch, .. } => {
                    Ok(simplifyDAEIfEquation(conds.clone(), branches.clone(), else_branch.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { condition: Deref @ DAE::Exp::BCONST { bool: true }, .. } => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![element.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(elements)
}

fn simplifyDAEIfEquation(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut elseBranch: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut cond_value: bool = false;
    let mut rest_branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = branches.clone();
    for mut cond in &*conditions.clone() {
        let mut cond = cond.clone();
        let __pa0 = ::match_deref::match_deref! { match &(cond.clone()) {
            Deref @ DAE::Exp::BCONST { bool: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        cond_value = __pa0.clone();
        if cond_value.clone() == true {
            elements = listHead(rest_branches.clone())?.reverse();
            return Ok(elements);
        }
        rest_branches = listRest(rest_branches.clone())?;
    }
    elements = elseBranch.clone().reverse();
    Ok(elements)
}

fn removeStreamSetElement(mut cref: Arc<DAE::ComponentRef>, mut elements: Arc<metamodelica::List<ConnectorElement>>) -> Result<Arc<metamodelica::List<ConnectorElement>>> {
    let mut elements: Arc<metamodelica::List<ConnectorElement>> = elements;
    (elements, _) = List::deleteMemberOnTrue(cref.clone(), elements.clone(), (std::sync::Arc::new(compareCrefStreamSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, ConnectorElement) -> Result<bool> + 'static>))?;
    Ok(elements)
}

fn compareCrefStreamSet(mut cref: Arc<DAE::ComponentRef>, mut element: ConnectorElement) -> Result<bool> {
    let mut matches: bool = false;
    matches = ComponentReferenceBasics::crefEqualNoStringCompare(cref.clone(), element.name.clone())?;
    Ok(matches)
}

pub fn componentFace(mut env: FCore::Graph, mut componentRef: Arc<DAE::ComponentRef>) -> Result<Face> {
    let mut face: Face = Face::INSIDE;
    face = 'mc: {
        let __mc_input = componentRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
                    Ok(openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, .. } => {
                    ::match_deref::match_deref! { match &(Lookup::lookupVar(FCore::emptyCache(), env.clone(), ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?) {
                        (_, _, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, _, _, _, _, _, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
                    Ok(openmodelica_frontend_types::DAE::Connect::Face::INSIDE)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(face)
}

pub fn componentFaceType(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Face> {
    let mut outFace: Face = Face::INSIDE;
    outFace = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE,
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, .. } => openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE,
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, .. }, .. } => openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE,
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => openmodelica_frontend_types::DAE::Connect::Face::INSIDE,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFace)
}

pub fn checkConnectorBalance(mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut path: Arc<Absyn::Path>, mut info: SourceInfo) -> Result<()> {
    let mut potentials: i32 = 0;
    let mut flows: i32 = 0;
    let mut streams: i32 = 0;
    (potentials, flows, streams) = countConnectorVars(vars.clone())?;
    let true = (checkConnectorBalance2(potentials.clone(), flows.clone(), streams.clone(), path.clone(), info.clone())?) else { bail!("pattern mismatch") };
    Ok(())
}

fn checkConnectorBalance2(mut potentialVars: i32, mut flowVars: i32, mut streamVars: i32, mut path: Arc<Absyn::Path>, mut info: SourceInfo) -> Result<bool> {
    let mut isBalanced: bool = true;
    let mut error_str: ArcStr = arcstr::literal!("");
    let mut flow_str: ArcStr = arcstr::literal!("");
    let mut potential_str: ArcStr = arcstr::literal!("");
    let mut class_str: ArcStr = arcstr::literal!("");
    if Config::languageStandardAtMost(Config::LanguageStandard::_2_x.clone())? {
        return Ok(isBalanced);
    }
    if potentialVars.clone() != flowVars.clone() {
        flow_str = ArcStr::from(::std::format!("{}", flowVars.clone()));
        potential_str = ArcStr::from(::std::format!("{}", potentialVars.clone()));
        class_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
        Error::addSourceMessage(Error::UNBALANCED_CONNECTOR.clone(), list![(class_str.clone()).clone(), (potential_str.clone()).clone(), (flow_str.clone()).clone()], info.clone())?;
    }
    if streamVars.clone() > 0 && flowVars.clone() != 1 {
        flow_str = ArcStr::from(::std::format!("{}", flowVars.clone()));
        class_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
        Error::addSourceMessage(Error::MISMATCHED_FLOW_IN_STREAM_CONNECTOR.clone(), list![(class_str.clone()).clone(), (flow_str.clone()).clone()], info.clone())?;
        isBalanced = false;
    }
    Ok(isBalanced)
}

fn countConnectorVars(mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<(i32, i32, i32)> {
    let mut potentialVars: i32 = 0;
    let mut flowVars: i32 = 0;
    let mut streamVars: i32 = 0;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    let mut n: i32 = 0;
    let mut p: i32 = 0;
    let mut f: i32 = 0;
    let mut s: i32 = 0;
    for mut var in &*vars.clone() {
        let mut var = var.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var.clone()) {
            Deref @ DAE::Var { attributes: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        attr = __pa0.clone();
        ty = __pa1.clone();
        ty2 = Types::arrayElementType(ty.clone());
        if Types::isConnector(ty2.clone()) {
            n = {
        let mut __acc: i32 = 1;
        for mut dim in (Types::getDimensionSizes(ty.clone())?).into_iter().cloned() {
            let __x = dim.clone();
            __acc *= __x;
        }
        __acc
    };
            (p, f, s) = countConnectorVars(Types::getConnectorVars(ty2.clone())?)?;
            if AbsynUtil::isInputOrOutput(DAEUtil::getAttrDirection(attr.clone()))? {
                p = 0;
            }
            potentialVars = potentialVars.clone() + p.clone() * n.clone();
            flowVars = flowVars.clone() + f.clone() * n.clone();
            streamVars = streamVars.clone() + s.clone() * n.clone();
        } else {
            let _ = (::match_deref::match_deref! { match &(attr.clone()) {
        Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::FLOW, .. } => {
            flowVars = flowVars.clone() + sizeOfType(var.ty.clone())?;
            ()
        },
        Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. } => {
            streamVars = streamVars.clone() + sizeOfType(var.ty.clone())?;
            ()
        },
        Deref @ DAE::Attributes { variability: SCode::Variability::VAR, direction: Absyn::Direction::BIDIR, .. } => {
            potentialVars = potentialVars.clone() + sizeOfType(var.ty.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
    }
    Ok((potentialVars, flowVars, streamVars))
}

fn sizeOfVariableList(mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<i32> {
    let mut size: i32 = 0;
    for mut var in &*vars.clone() {
        let mut var = var.clone();
        size = size.clone() + sizeOfType(var.ty.clone())?;
    }
    Ok(size)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn sizeOfType(mut ty: Arc<DAE::Type>) -> Result<i32> {
    let mut size: i32 = 0;
    size = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            1
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            1
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            1
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            1
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            1
        },
        Deref @ DAE::Type::T_ARRAY { .. } => {
            todo!("reduction intMul: cannot resolve default value") * sizeOfType(var_field!((*ty).ty, DAE::Type::T_ARRAY).clone())?
        },
        Deref @ DAE::Type::T_COMPLEX { equalityConstraint: None, varLst: v, .. } => {
            sizeOfVariableList(v.clone())?
        },
        Deref @ DAE::Type::T_COMPLEX { equalityConstraint: Some((_, n, _)), .. } => {
            n.clone()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => {
            0
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
            sizeOfType(t.clone())?
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectUtil.sizeOfType failed on ")); __mm_s.push_str(&*TypesDump::printTypeStr(ty.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

pub fn checkShortConnectorDef(mut state: ClassInf::State, mut attributes: SCode::Attributes, mut info: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = ({
        let mut pv: i32 = 0;
        let mut fv: i32 = 0;
        let mut sv: i32 = 0;
        (match (state.clone(), attributes.clone()) {
        (ClassInf::State::CONNECTOR { .. }, SCode::Attributes { direction: Absyn::Direction::BIDIR, connectorType: mut ct, .. }) => {
            if SCodeUtil::flowBool(ct.clone()) {
                fv = 1;
            } else if SCodeUtil::streamBool(ct.clone()) {
                sv = 1;
            } else {
                pv = 1;
            }
            checkConnectorBalance2(pv.clone(), fv.clone(), sv.clone(), var_field!(state.path, ClassInf::State::CONNECTOR).clone(), info.clone())?
        },
        _ => {
            true
        },
    })
    });
    Ok(isValid)
}

pub fn isReferenceInConnects(mut connects: Arc<metamodelica::List<ConnectorElement>>, mut cref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut isThere: bool = false;
    for mut ce in &*connects.clone() {
        let mut ce = ce.clone();
        if ComponentReferenceBasics::crefPrefixOf(cref.clone(), ce.name.clone())? {
            isThere = true;
            return Ok(isThere);
        }
    }
    Ok(isThere)
}

pub fn removeReferenceFromConnects(mut connects: Arc<metamodelica::List<ConnectorElement>>, mut cref: Arc<DAE::ComponentRef>) -> Result<(Arc<metamodelica::List<ConnectorElement>>, bool)> {
    let mut connects: Arc<metamodelica::List<ConnectorElement>> = connects;
    let mut wasRemoved: bool = false;
    let mut oe: Option<ConnectorElement> = None;
    (connects, oe) = List::deleteMemberOnTrue(cref.clone(), connects.clone(), (std::sync::Arc::new(removeReferenceFromConnects2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, ConnectorElement) -> Result<bool> + 'static>))?;
    wasRemoved = isSome(oe.clone());
    Ok((connects, wasRemoved))
}

fn removeReferenceFromConnects2(mut cref: Arc<DAE::ComponentRef>, mut element: ConnectorElement) -> Result<bool> {
    let mut matches: bool = false;
    matches = ComponentReferenceBasics::crefPrefixOf(cref.clone(), element.name.clone())?;
    Ok(matches)
}

pub fn printSetsStr(mut sets: Sets) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", sets.setCount.clone()))); __mm_s.push_str(&*literal!(" sets:\n")); ArcStr::from(__mm_s) }).clone();
    string = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*string.clone()); __mm_s.push_str(&*printSetTrieStr(sets.sets.clone(), (literal!("\t")).clone())?); ArcStr::from(__mm_s) }).clone();
    string = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*string.clone()); __mm_s.push_str(&*literal!("Connected sets:\n")); ArcStr::from(__mm_s) }).clone();
    string = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*string.clone()); __mm_s.push_str(&*printSetConnections(sets.connections.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    Ok(string)
}

fn printSetTrieStr(mut trie: Arc<SetTrieNode>, mut accumName: ArcStr) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((::match_deref::match_deref! { match &(trie.clone()) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*accumName.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*trie).name, SetTrieNode::SET_TRIE_LEAF).clone()); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*res.clone()); __mm_s.push_str(&*printLeafElementStr(var_field!((*trie).insideElement, SetTrieNode::SET_TRIE_LEAF).clone())?); ArcStr::from(__mm_s) }).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*res.clone()); __mm_s.push_str(&*printLeafElementStr(var_field!((*trie).outsideElement, SetTrieNode::SET_TRIE_LEAF).clone())?); ArcStr::from(__mm_s) }).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*res.clone()); __mm_s.push_str(&*printOptFlowAssociation(var_field!((*trie).flowAssociation, SetTrieNode::SET_TRIE_LEAF).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { name: Deref @ "", .. } => {
            stringAppendList(List::map1(var_field!((*trie).nodes, SetTrieNode::SET_TRIE_NODE).clone(), (std::sync::Arc::new(printSetTrieStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, ArcStr) -> Result<ArcStr> + 'static>), (accumName.clone()).clone()))
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*accumName.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*trie).name, SetTrieNode::SET_TRIE_NODE).clone()); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(List::map1(var_field!((*trie).nodes, SetTrieNode::SET_TRIE_NODE).clone(), (std::sync::Arc::new(printSetTrieStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetTrieNode>, ArcStr) -> Result<ArcStr> + 'static>), (name.clone()).clone()));
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(string)
}

fn printLeafElementStr(mut element: Option<ConnectorElement>) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((match element.clone() {
        Some(mut e @ ConnectorElement { .. }) => {
            let mut res: ArcStr = arcstr::literal!("");
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*printFaceStr(e.face.clone())?); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*res.clone()); __mm_s.push_str(&*printConnectorTypeStr(e.ty.clone())?); __mm_s.push_str(&*literal!(" [")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", e.set.clone()))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        _ => {
            literal!("")
        },
    })).clone();
    Ok(string)
}

pub fn printElementStr(mut element: ConnectorElement) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(element.name.clone())?); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
    string = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*string.clone()); __mm_s.push_str(&*printFaceStr(element.face.clone())?); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
    string = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*string.clone()); __mm_s.push_str(&*printConnectorTypeStr(element.ty.clone())?); __mm_s.push_str(&*literal!(" [")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", element.set.clone()))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    Ok(string)
}

pub fn printFaceStr(mut face: Face) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((match face.clone() {
        DAE::Connect::Face::INSIDE => literal!("inside"),
        DAE::Connect::Face::OUTSIDE => literal!("outside"),
        DAE::Connect::Face::NO_FACE => literal!("unknown"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(string)
}

fn printConnectorTypeStr(mut ty: ConnectorType) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((match ty.clone() {
        DAE::Connect::ConnectorType::EQU => literal!("equ"),
        DAE::Connect::ConnectorType::FLOW => literal!("flow"),
        DAE::Connect::ConnectorType::STREAM { .. } => literal!("stream"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(string)
}

fn printOptFlowAssociation(mut cref: Option<Arc<DAE::ComponentRef>>) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((::match_deref::match_deref! { match &(cref.clone()) {
        None => {
            literal!("")
        },
        Some(cr) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" associated flow: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(string)
}

fn printSetConnections(mut connections: Arc<metamodelica::List<(i32, i32)>>) -> ArcStr {
    let mut string: ArcStr = arcstr::literal!("");
    string = stringAppendList(List::map(connections.clone(), (std::sync::Arc::new(fnptr!(printSetConnection, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>)));
    string
}

fn printSetConnection(mut connection: (i32, i32)) -> ArcStr {
    let mut string: ArcStr = arcstr::literal!("");
    let mut set1: i32 = 0;
    let mut set2: i32 = 0;
    (set1, set2) = connection.clone();
    string = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", set1.clone()))); __mm_s.push_str(&*literal!(" connected to ")); __mm_s.push_str(&*intString(set2.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    string
}

fn printSetStr(mut set: Set) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((match set.clone() {
        DAE::Connect::Set::SET { .. } => stringDelimitList(List::map(var_field!(set.elements, Set::SET).clone(), (std::sync::Arc::new(printElementStr) as std::sync::Arc<dyn ::std::ops::Fn(ConnectorElement) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone()),
        DAE::Connect::Set::SET_POINTER { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pointer to set ")); __mm_s.push_str(&*intString(var_field!(set.index, Set::SET_POINTER).clone())); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(string)
}

fn getAllEquCrefs(mut sets: Arc<metamodelica::List<Set>>) -> Arc<metamodelica::List<Arc<DAE::ComponentRef>>> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    for mut set in &*sets.clone() {
        let mut set = set.clone();
        let _ = (match set.clone() {
        DAE::Connect::Set::SET { ty: DAE::Connect::ConnectorType::EQU, .. } => {
            for mut e in &*var_field!(set.elements, Set::SET).clone() {
                let mut e = e.clone();
                crefs = cons(e.name.clone(), crefs.clone());
            }
            ()
        },
        _ => (),
    });
    }
    crefs
}

fn removeUnusedExpandableVariablesAndConnections(mut sets: Arc<metamodelica::List<Set>>, mut DAE: DAE::DAElist) -> Result<(Arc<metamodelica::List<Set>>, DAE::DAElist)> {
    let mut sets: Arc<metamodelica::List<Set>> = sets;
    let mut DAE: DAE::DAElist = DAE;
    let mut elems: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut expandableVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut unnecessary: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut usedInDAE: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut onlyExpandableConnected: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut equVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut setsAsCrefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let DAE::DAE { elementLst: __pa0 } = (DAE.clone()) else { bail!("pattern mismatch") };
    elems = __pa0.clone();
    expandableVars = getExpandableVariablesWithNoBinding(elems.clone());
    dae = DAEUtil::removeVariables(DAE.clone(), expandableVars.clone())?;
    usedInDAE = DAEUtil::getAllExpandableCrefsFromDAE(dae.clone())?;
    setsAsCrefs = getExpandableEquSetsAsCrefs(sets.clone());
    setsAsCrefs = mergeEquSetsAsCrefs(setsAsCrefs.clone())?;
    setsAsCrefs = mergeEquSetsAsCrefs(setsAsCrefs.clone())?;
    onlyExpandableConnected = getOnlyExpandableConnectedCrefs(setsAsCrefs.clone());
    unnecessary = List::setDifferenceOnTrue(onlyExpandableConnected.clone(), usedInDAE.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefEqualWithoutSubs, Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
    DAE = DAEUtil::removeVariables(DAE.clone(), unnecessary.clone())?;
    sets = removeCrefsFromSets(sets.clone(), unnecessary.clone());
    equVars = getAllEquCrefs(sets.clone());
    expandableVars = List::setDifferenceOnTrue(expandableVars.clone(), usedInDAE.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefEqualWithoutSubs, Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
    unnecessary = List::setDifferenceOnTrue(expandableVars.clone(), equVars.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefEqualWithoutSubs, Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
    DAE = DAEUtil::removeVariables(DAE.clone(), unnecessary.clone())?;
    Ok((sets, DAE))
}

fn isEquType(mut ty: ConnectorType) -> bool {
    let mut isEqu: bool = false;
    isEqu = (match ty.clone() {
        DAE::Connect::ConnectorType::EQU => true,
        _ => false,
    });
    isEqu
}

