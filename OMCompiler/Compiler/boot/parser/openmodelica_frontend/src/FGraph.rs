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
use crate::FCore::RefTree;
use crate::FCore;
use crate::FGraphBuildEnv;
use crate::FNode;
use crate::InnerOuter;
use crate::Mod;
use crate::PrefixUtil;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
pub type Name = ArcStr;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = FCore::Node;

pub type Data = FCore::Data;

pub type Kind = FCore::Kind;

pub type Ref = metamodelica::Array<FCore::Node>;

pub type Refs = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Children = Arc<FCore::RefTree::Tree>;

pub type Parents = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Scope = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Top = FCore::Top;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Status = FCore::Status;

pub const fn emptyGraph() -> FCore::Graph { FCore::Graph::EG { name: literal!("empty") } }

pub fn top(mut inGraph: Graph) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (match inGraph.clone() {
        FCore::Graph::G { .. } => var_field!(inGraph.top, FCore::Graph::G).node.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outRef)
}

pub fn extra(mut inGraph: Graph) -> Result<Extra> {
    let mut outExtra: Extra = <FCore::Extra as ::std::default::Default>::default();
    outExtra = (match inGraph.clone() {
        FCore::Graph::G { .. } => var_field!(inGraph.top, FCore::Graph::G).extra.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outExtra)
}

pub fn currentScope(mut inGraph: Graph) -> Result<Scope> {
    let mut outScope: Scope = metamodelica::nil();
    outScope = (match inGraph.clone() {
        FCore::Graph::G { scope: ref outScope, .. } => outScope.clone(),
        FCore::Graph::EG { name: _ } => metamodelica::nil(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outScope)
}

pub fn lastScopeRef(mut inGraph: Graph) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = listHead(currentScope(inGraph.clone())?)?;
    Ok(outRef)
}

pub fn setLastScopeRef(mut inRef: Ref, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph = inGraph.clone();
    outGraph = (match outGraph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = cons(inRef.clone(), listRest(var_field!(outGraph.scope, FCore::Graph::G).clone())?);
            if let FCore::Graph::G { scope, .. } = &mut outGraph {
                *scope = __owned_variant_scope_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G"); }
            outGraph.clone()
        },
        _ => outGraph.clone(),
    });
    Ok(outGraph)
}

pub fn stripLastScopeRef(mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    let mut t: Top = <FCore::Top as ::std::default::Default>::default();
    let mut s: Scope = metamodelica::nil();
    let FCore::G { top: __pa0, scope: metamodelica::List::Cons { head: __pa1, tail: __pa2 } } = (inGraph.clone()) else { bail!("pattern mismatch") };
    t = __pa0.clone();
    outRef = __pa1.clone();
    s = __pa2.clone();
    outGraph = FCore::Graph::G { top: t.clone(), scope: s.clone() };
    Ok((outGraph, outRef))
}

pub fn topScope(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut t: Ref;
    let mut r: Ref;
    let mut s: Scope = metamodelica::nil();
    let mut gn: Name = arcstr::literal!("");
    let mut v: Visited = <FCore::Visited as ::std::default::Default>::default();
    let mut e: Extra = <FCore::Extra as ::std::default::Default>::default();
    let mut next: Next = 0;
    outGraph = (match inGraph.clone() {
        FCore::Graph::G { .. } => var_field!(inGraph.top, FCore::Graph::G).graph.clone().borrow()[(1-1) as usize].clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outGraph)
}

pub fn empty() -> Graph {
    let mut outGraph: Graph;
    outGraph = emptyGraph().clone();
    outGraph
}

pub fn new(mut inGraphName: Name, mut inPath: Arc<Absyn::Path>) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut n: Node = <FCore::Node as ::std::default::Default>::default();
    let mut s: Scope = metamodelica::nil();
    let mut v: Visited = <FCore::Visited as ::std::default::Default>::default();
    let mut nr: Ref;
    let mut next: Next = 0;
    let mut id: Id = 0;
    let mut ag: metamodelica::Array<FCore::Graph>;
    let mut top: Top = <FCore::Top as ::std::default::Default>::default();
    id = System::tmpTickIndex(Global::fgraph_nextId.clone());
    n = FNode::new((arcstr::literal!(FNode::topNodeName)).clone(), id.clone(), metamodelica::nil(), crate::FCore::Data::TOP);
    nr = FNode::toRef(n.clone());
    s = list![nr.clone()];
    ag = metamodelica::arrayCreate(1, emptyGraph().clone());
    top = FCore::Top { graph: ag.clone(), name: (inGraphName.clone()).clone(), node: nr.clone(), extra: FCore::Extra { topModel: inPath.clone() } };
    outGraph = FCore::Graph::G { top: top.clone(), scope: s.clone() };
    unsafe { metamodelica::Dangerous::arrayInitSlot(ag.clone(), 1, FCore::Graph::G { top: top.clone(), scope: list![nr.clone()] }) };
    Ok(outGraph)
}

pub fn node(mut inGraph: Graph, mut inName: Name, mut inParents: Parents, mut inData: Data) -> (Graph, Node) {
    let mut outGraph: Graph;
    let mut outNode: Node = <FCore::Node as ::std::default::Default>::default();
    (outGraph, outNode) = (::match_deref::match_deref! { match &((inGraph.clone(), inName.clone(), inParents.clone(), inData.clone())) {
        (g, _, _, _) => {
            let mut i: i32 = 0;
            let mut n: Node = <FCore::Node as ::std::default::Default>::default();
            i = System::tmpTickIndex(Global::fgraph_nextId.clone());
            n = FNode::new((inName.clone()).clone(), i.clone(), inParents.clone(), inData.clone());
            (g.clone(), n.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outGraph, outNode)
}

pub fn clone(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        FCore::Graph::G { top: mut t, scope: ref s } => {
            let mut g: Graph;
            let mut nt: Ref;
            let mut ag: metamodelica::Array<FCore::Graph>;
            let mut s = s.clone();
            nt = FNode::toRef(FNode::fromRef(t.node.clone())?);
            (g, nt) = FNode::copyRef(nt.clone(), inGraph.clone())?;
            s = List::map1r(s.clone(), (std::sync::Arc::new(FNode::lookupRefFromRef) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>, metamodelica::Array<FCore::Node>) -> Result<metamodelica::Array<FCore::Node>> + 'static>), nt.clone());
            ag = arrayCreate(1, emptyGraph().clone());
            t = FCore::Top { graph: ag.clone(), name: (t.name.clone()).clone(), node: nt.clone(), extra: t.extra.clone() };
            g = FCore::Graph::G { top: t.clone(), scope: s.clone() };
            {let _arr = ag.clone(); _arr.borrow_mut()[(1-1) as usize] = g.clone(); _arr};
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outGraph)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn updateComp(mut inGraph: Graph, mut inVar: Arc<DAE::Var>, mut instStatus: FCore::Status, mut inTargetGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inVar.clone(), instStatus.clone(), inTargetGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }, _, _) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id = 0;
                    let mut p: Parents = metamodelica::nil();
                    let mut c: Children = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut e: Arc<SCode::Element>;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut k: Kind = FCore::Kind::BASIC_TYPE;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(g.clone())?;
                    r = FNode::child(pr.clone(), (n.clone()).clone())?;
                    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::CO { e: __pa4, r#mod: __pa5, kind: __pa6, status: _ } } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    n = __pa0.clone();
                    id = __pa1.clone();
                    p = __pa2.clone();
                    c = __pa3.clone();
                    e = __pa4.clone();
                    m = __pa5.clone();
                    k = __pa6.clone();
                    r = FNode::updateRef(r.clone(), FCore::Node { name: (n.clone()).clone(), id: id.clone(), parents: p.clone(), children: c.clone(), data: FCore::Data::CO { e: e.clone(), r#mod: m.clone(), kind: k.clone(), status: instStatus.clone() } })?;
                    r = updateSourceTargetScope(r.clone(), currentScope(inTargetGraph.clone())?)?;
                    r = updateInstance(r.clone(), v.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v, _, _) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(g.clone())?;
                    let true = (FNode::isImplicitRefName(pr.clone())?) else { bail!("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateComp(g.clone(), v.clone(), instStatus.clone(), inTargetGraph.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn updateSourceTargetScope(mut inRef: Ref, mut inTargetScope: Scope) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = (inRef.clone(), inTargetScope.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r, _) => {
                    let mut r = (*r).clone();
                    r = FNode::refRef(r.clone())?;
                    r = FNode::updateRef(r.clone(), FNode::setData(FNode::fromRef(r.clone())?, FCore::Data::REF { target: inTargetScope.clone() })?)?;
                    Ok(inRef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r, _) => {
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FNode.updateSourceTargetScope: node does not yet have a reference child: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!(" target scope: ")); __mm_s.push_str(&*FNode::scopeStr(inTargetScope.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(inRef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRef)
}

pub fn updateInstance(mut inRef: Ref, mut inVar: Arc<DAE::Var>) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = (inRef.clone(), inVar.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r, _) => {
                    let mut r = (*r).clone();
                    r = FNode::refInstance(r.clone())?;
                    r = FNode::updateRef(r.clone(), FNode::setData(FNode::fromRef(r.clone())?, FCore::Data::IT { i: inVar.clone() })?)?;
                    Ok(inRef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.updateInstance failed for node: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(inRef.clone())?)?); __mm_s.push_str(&*literal!(" variable:")); __mm_s.push_str(&*TypesDump::printVarStr(inVar.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRef)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn updateVarAndMod(mut inGraph: Graph, mut inVar: Arc<DAE::Var>, mut inMod: Arc<DAE::Mod>, mut instStatus: FCore::Status, mut inTargetGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inVar.clone(), inMod.clone(), instStatus.clone(), inTargetGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }, _, _, _) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id = 0;
                    let mut p: Parents = metamodelica::nil();
                    let mut c: Children = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut e: Arc<SCode::Element>;
                    let mut k: Kind = FCore::Kind::BASIC_TYPE;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(g.clone())?;
                    r = FNode::child(pr.clone(), (n.clone()).clone())?;
                    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::CO { e: __pa4, r#mod: _, kind: __pa5, status: _ } } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    n = __pa0.clone();
                    id = __pa1.clone();
                    p = __pa2.clone();
                    c = __pa3.clone();
                    e = __pa4.clone();
                    k = __pa5.clone();
                    r = FNode::updateRef(r.clone(), FCore::Node { name: (n.clone()).clone(), id: id.clone(), parents: p.clone(), children: c.clone(), data: FCore::Data::CO { e: e.clone(), r#mod: inMod.clone(), kind: k.clone(), status: instStatus.clone() } })?;
                    r = updateSourceTargetScope(r.clone(), currentScope(inTargetGraph.clone())?)?;
                    r = updateInstance(r.clone(), v.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v, _, _, _) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(g.clone())?;
                    let true = (FNode::isImplicitRefName(pr.clone())?) else { bail!("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateVarAndMod(g.clone(), v.clone(), inMod.clone(), instStatus.clone(), inTargetGraph.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn updateClass(mut inGraph: Graph, mut inElement: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut instStatus: FCore::Status, mut inTargetGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inElement.clone(), inPrefix.clone(), inMod.clone(), instStatus.clone(), inTargetGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, e @ Deref @ SCode::Element::CLASS { name: n, .. }, _, _, _, _) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id = 0;
                    let mut p: Parents = metamodelica::nil();
                    let mut c: Children = Arc::new(FCore::RefTree::Tree::EMPTY);
                    let mut k: Kind = FCore::Kind::BASIC_TYPE;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(g.clone())?;
                    r = FNode::child(pr.clone(), (n.clone()).clone())?;
                    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::CL { e: _, pre: _, r#mod: _, kind: __pa4, status: _ } } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    n = __pa0.clone();
                    id = __pa1.clone();
                    p = __pa2.clone();
                    c = __pa3.clone();
                    k = __pa4.clone();
                    r = FNode::updateRef(r.clone(), FCore::Node { name: (n.clone()).clone(), id: id.clone(), parents: p.clone(), children: c.clone(), data: FCore::Data::CL { e: e.clone(), pre: inPrefix.clone(), r#mod: inMod.clone(), kind: k.clone(), status: instStatus.clone() } })?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, e, _, _, _, _) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(g.clone())?;
                    let true = (FNode::isImplicitRefName(pr.clone())?) else { bail!("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateClass(g.clone(), e.clone(), inPrefix.clone(), inMod.clone(), instStatus.clone(), inTargetGraph.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn updateClassElement(mut inRef: Ref, mut inElement: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut instStatus: FCore::Status, mut inTargetGraph: Graph) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (::match_deref::match_deref! { match &((inRef.clone(), inElement.clone(), inPrefix.clone(), inMod.clone(), instStatus.clone(), inTargetGraph.clone())) {
        (r, e @ Deref @ SCode::Element::CLASS { name: n, .. }, _, _, _, _) => {
            let mut id: Id = 0;
            let mut p: Parents = metamodelica::nil();
            let mut c: Children = Arc::new(FCore::RefTree::Tree::EMPTY);
            let mut k: Kind = FCore::Kind::BASIC_TYPE;
            let mut r = (*r).clone();
            let FCore::N { name: _, id: __pa0, parents: __pa1, children: __pa2, data: FCore::CL { e: _, pre: _, r#mod: _, kind: __pa3, status: _ } } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
            id = __pa0.clone();
            p = __pa1.clone();
            c = __pa2.clone();
            k = __pa3.clone();
            r = FNode::updateRef(r.clone(), FCore::Node { name: (n.clone()).clone(), id: id.clone(), parents: p.clone(), children: c.clone(), data: FCore::Data::CL { e: e.clone(), pre: inPrefix.clone(), r#mod: inMod.clone(), kind: k.clone(), status: instStatus.clone() } })?;
            r.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRef)
}

pub fn addForIterator(mut inGraph: Graph, mut name: ArcStr, mut ty: Arc<DAE::Type>, mut binding: Arc<DAE::Binding>, mut variability: SCode::Variability, mut constOfForIteratorRange: Option<DAE::Const>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inGraph.clone(), name.clone(), ty.clone(), binding.clone(), variability.clone(), constOfForIteratorRange.clone())) {
        (g, _, _, _, _, _) => {
            let mut r: Ref;
            let mut c: Arc<SCode::Element>;
            let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
            let mut g = (*g).clone();
            c = Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
            v = Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: Arc::new(DAE::Attributes { connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: variability.clone(), direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC }), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: constOfForIteratorRange.clone() });
            r = lastScopeRef(g.clone())?;
            g = FGraphBuildEnv::mkCompNode(c.clone(), r.clone(), crate::FCore::Kind::BUILTIN, g.clone())?;
            g = updateVarAndMod(g.clone(), v.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_UNTYPED, empty())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn printGraphPathStr(mut inGraph: Graph) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Graph::G { scope: s @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut s = (*s).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(s.clone().reverse()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    s = __pa0.clone();
                    r#str = stringDelimitList(List::map(s.clone(), (std::sync::Arc::new(FNode::refName) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<ArcStr> + 'static>)), (literal!(".")).clone());
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("<global scope>"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn openNewScope(mut inGraph: Graph, mut encapsulatedPrefix: SCode::Encapsulated, mut inName: Option<ArcStr>, mut inScopeType: Option<FCore::ScopeType>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), encapsulatedPrefix.clone(), inName.clone(), inScopeType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, Some(mut n), _) = __mc_input.clone() else { bail!("nomatch") };
            let mut no: Node = <FCore::Node as ::std::default::Default>::default();
            let mut r: Ref;
            let mut p: Ref;
            p = lastScopeRef(g.clone())?;
            (g, no) = node(g.clone(), (n.clone()).clone(), list![p.clone()], FCore::Data::ND { scopeType: inScopeType.clone() });
            r = FNode::toRef(no.clone());
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.openNewScope: failed to open new scope in scope: ")); __mm_s.push_str(&*getGraphNameStr(inGraph.clone())?); __mm_s.push_str(&*literal!(" name: ")); __mm_s.push_str(&*Util::getOptionOrDefault(inName.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn openScope(mut inGraph: Graph, mut encapsulatedPrefix: SCode::Encapsulated, mut inName: Name, mut inScopeType: Option<FCore::ScopeType>) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut p: Ref;
    p = lastScopeRef(inGraph.clone())?;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), encapsulatedPrefix.clone(), inName.clone(), inScopeType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, mut n, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            r = FNode::child(p.clone(), (n.clone()).clone())?;
            let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: _ }, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            FNode::addChildRef(p.clone(), (n.clone()).clone(), r.clone(), false)?;
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, mut n, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            r = FNode::child(p.clone(), (n.clone()).clone())?;
            r = FNode::copyRefNoUpdate(r.clone());
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, mut n, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut no: Node = <FCore::Node as ::std::default::Default>::default();
            let mut r: Ref;
            (g, no) = node(g.clone(), (n.clone()).clone(), list![p.clone()], FCore::Data::ND { scopeType: inScopeType.clone() });
            r = FNode::toRef(no.clone());
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.openScope: failed to open new scope in scope: ")); __mm_s.push_str(&*getGraphNameStr(inGraph.clone())?); __mm_s.push_str(&*literal!(" name: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn inForLoopScope(mut inGraph: Graph) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut name: ArcStr = arcstr::literal!("");
            name = (FNode::refName(listHead(currentScope(inGraph.clone())?)?)?).clone();
            let true = (stringEq((name.clone()).clone(), (arcstr::literal!(FCore::forScopeName)).clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn inForOrParforIterLoopScope(mut inGraph: Graph) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut name: ArcStr = arcstr::literal!("");
            name = (FNode::refName(listHead(currentScope(inGraph.clone())?)?)?).clone();
            let true = (stringEq((name.clone()).clone(), (arcstr::literal!(FCore::forIterScopeName)).clone()) || stringEq((name.clone()).clone(), (arcstr::literal!(FCore::parForIterScopeName)).clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn getScopePath(mut inGraph: Graph) -> Result<Option<Arc<Absyn::Path>>> {
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    outPath = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let __pa0 = ::match_deref::match_deref! { match &(currentScope(inGraph.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            let true = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
            Ok(None)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Arc<Absyn::Path>;
            p = getGraphName(inGraph.clone())?;
            Ok(Some(p.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

pub fn getGraphNameStr(mut inGraph: Graph) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(AbsynUtil::pathString(getGraphName(inGraph.clone())?, (literal!(".")).clone(), true, false)?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!("."))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn getGraphName(mut inGraph: Graph) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut p: Arc<Absyn::Path>;
    let mut s: Scope = metamodelica::nil();
    let mut r: Ref;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(currentScope(inGraph.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r = __pa0.clone();
    s = __pa1.clone();
    p = AbsynUtil::makeIdentPathFromString((FNode::refName(r.clone())?).clone());
    for mut r in &*s.clone() {
        let mut r = r.clone();
        p = Arc::new(Absyn::Path::QUALIFIED { name: (FNode::refName(r.clone())?).clone(), path: p.clone() });
    }
    let __pa2 = ::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::Path::QUALIFIED { name: _, path: __pa2 } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outPath = __pa2.clone();
    Ok(outPath)
}

pub fn getGraphNameNoImplicitScopes(mut inGraph: Graph) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut p: Arc<Absyn::Path>;
    let mut s: Scope = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(currentScope(inGraph.clone())?.reverse()) {
        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    s = __pa0.clone();
    outPath = AbsynUtil::stringListPath({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut r#str in ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (s.clone()).into_iter().cloned() {
            let __x = FNode::refName(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned() {
            if !(stringGet((r#str.clone()).clone(),1)? != 36) { continue; }
            let __x = r#str.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outPath)
}

pub fn pushScopeRef(mut graph: Graph, mut inRef: Ref) -> Result<Graph> {
    let mut graph: Graph = graph;
    let _ = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = cons(inRef.clone(), var_field!(graph.scope, FCore::Graph::G).clone());
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(graph)
}

pub fn pushScope(mut graph: Graph, mut inScope: Scope) -> Result<Graph> {
    let mut graph: Graph = graph;
    let _ = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = listAppend(inScope.clone(), var_field!(graph.scope, FCore::Graph::G).clone());
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(graph)
}

pub fn setScope(mut graph: Graph, mut inScope: Scope) -> Result<Graph> {
    let mut graph: Graph = graph;
    let _ = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = inScope.clone();
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(graph)
}

pub fn restrictionToScopeType(mut inRestriction: SCode::Restriction) -> Option<FCore::ScopeType> {
    let mut outType: Option<FCore::ScopeType> = None;
    outType = (match inRestriction.clone() {
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_PARALLEL_FUNCTION } => Some(crate::FCore::ScopeType::PARALLEL_SCOPE),
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_KERNEL_FUNCTION } => Some(crate::FCore::ScopeType::PARALLEL_SCOPE),
        SCode::Restriction::R_FUNCTION { functionRestriction: _ } => Some(crate::FCore::ScopeType::FUNCTION_SCOPE),
        _ => Some(crate::FCore::ScopeType::CLASS_SCOPE),
    });
    outType
}

pub fn scopeTypeToRestriction(mut inScopeType: FCore::ScopeType) -> SCode::Restriction {
    let mut outRestriction: SCode::Restriction = SCode::Restriction::R_BLOCK;
    outRestriction = (match inScopeType.clone() {
        FCore::ScopeType::PARALLEL_SCOPE => SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_PARALLEL_FUNCTION },
        FCore::ScopeType::FUNCTION_SCOPE => SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: openmodelica_ast::Absyn::FunctionPurity::NO_PURITY } },
        _ => openmodelica_frontend_types::SCode::Restriction::R_CLASS,
    });
    outRestriction
}

pub fn isTopScope(mut graph: Graph) -> Result<bool> {
    let mut isTop: bool = false;
    isTop = 'mc: {
        let __mc_input = graph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefTop(lastScopeRef(graph.clone())?)?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isTop)
}

pub fn crefStripGraphScopePrefix(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Graph, mut stripPartial: bool) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), inEnv.clone(), stripPartial.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else { bail!("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut env_path: Arc<Absyn::Path>;
                    let mut cref1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut cref2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let __pa0 = ::match_deref::match_deref! { match &(getScopePath(inEnv.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    env_path = __pa0.clone();
                    cref1 = AbsynUtil::unqualifyCref(inCref.clone());
                    env_path = AbsynUtil::makeNotFullyQualified(env_path.clone());
                    cref2 = crefStripGraphScopePrefix2(cref1.clone(), env_path.clone(), stripPartial.clone())?;
                    let false = (AbsynUtil::crefEqual(cref1.clone(), cref2.clone())) else { bail!("pattern mismatch") };
                    Ok(cref2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn crefStripGraphScopePrefix2(mut inCref: Arc<Absyn::ComponentRef>, mut inEnvPath: Arc<Absyn::Path>, mut stripPartial: bool) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), inEnvPath.clone(), stripPartial.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cref, subscripts: Deref @ metamodelica::List::Nil, name: id1 }, Deref @ Absyn::Path::QUALIFIED { path: env_path, name: id2 }, _) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(crefStripGraphScopePrefix2(cref.clone(), env_path.clone(), stripPartial.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cref, subscripts: Deref @ metamodelica::List::Nil, name: id1 }, Deref @ Absyn::Path::IDENT { name: id2 }, _) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { subscripts: Deref @ metamodelica::List::Nil, name: id1, .. }, env_path, true) => {
                    let false = (stringEqual((id1.clone()).clone(), (AbsynUtil::pathFirstIdent(env_path.clone())?).clone())) else { bail!("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

pub fn pathStripGraphScopePrefix(mut inPath: Arc<Absyn::Path>, mut inEnv: Graph, mut stripPartial: bool) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = (inPath.clone(), inEnv.clone(), stripPartial.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else { bail!("pattern mismatch") };
                    Ok(inPath.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut env_path: Arc<Absyn::Path>;
                    let mut path1: Arc<Absyn::Path>;
                    let mut path2: Arc<Absyn::Path>;
                    let __pa0 = ::match_deref::match_deref! { match &(getScopePath(inEnv.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    env_path = __pa0.clone();
                    path1 = AbsynUtil::makeNotFullyQualified(inPath.clone());
                    env_path = AbsynUtil::makeNotFullyQualified(env_path.clone());
                    path2 = pathStripGraphScopePrefix2(path1.clone(), env_path.clone(), stripPartial.clone())?;
                    let false = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    Ok(path2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inPath.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn pathStripGraphScopePrefix2(mut inPath: Arc<Absyn::Path>, mut inEnvPath: Arc<Absyn::Path>, mut stripPartial: bool) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath.clone(), inEnvPath.clone(), stripPartial.clone())) {
        (Deref @ Absyn::Path::QUALIFIED { path, name: id1 }, Deref @ Absyn::Path::QUALIFIED { path: env_path, name: id2 }, _) if (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) => {
            pathStripGraphScopePrefix2(path.clone(), env_path.clone(), stripPartial.clone())?
        },
        (Deref @ Absyn::Path::QUALIFIED { path, name: id1 }, Deref @ Absyn::Path::IDENT { name: id2 }, _) if (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) => {
            path.clone()
        },
        (Deref @ Absyn::Path::QUALIFIED { name: id1, .. }, env_path, true) if (!(stringEqual((id1.clone()).clone(), (AbsynUtil::pathFirstIdent(env_path.clone())?).clone()))) => {
            inPath.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn mkComponentNode(mut inGraph: Graph, mut inVar: Arc<DAE::Var>, mut inVarEl: Arc<SCode::Element>, mut inMod: Arc<DAE::Mod>, mut instStatus: Status, mut inCompGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inVar.clone(), inVarEl.clone(), inMod.clone(), instStatus.clone(), inCompGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Var { name: n, .. }, c, _, _, _) => {
                    let false = (stringEq((n.clone()).clone(), (SCodeUtil::elementName(c.clone())?).clone())) else { bail!("pattern mismatch") };
                    Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.mkComponentNode: The component name: ")); __mm_s.push_str(&*SCodeUtil::elementName(c.clone())?); __mm_s.push_str(&*literal!(" is not the same as its DAE.TYPES_VAR: ")); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }, c, m, i, cg) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    let true = (stringEq((n.clone()).clone(), (SCodeUtil::elementName(c.clone())?).clone())) else { bail!("pattern mismatch") };
                    r = lastScopeRef(g.clone())?;
                    g = FGraphBuildEnv::mkCompNode(c.clone(), r.clone(), crate::FCore::Kind::USERDEFINED, g.clone())?;
                    g = updateVarAndMod(g.clone(), v.clone(), m.clone(), i.clone(), cg.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn mkClassNode(mut inGraph: Graph, mut inClass: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut checkDuplicate: bool) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inClass.clone(), inPrefix.clone(), inMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ SCode::Element::CLASS { name: n, .. }, _, _) => {
                    let mut r: Ref;
                    r = lastScopeRef(g.clone())?;
                    r = FNode::child(r.clone(), (n.clone()).clone())?;
                    let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: _ }, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    r = lastScopeRef(g.clone())?;
                    g = FGraphBuildEnv::mkClassNode(inClass.clone(), inPrefix.clone(), inMod.clone(), r.clone(), crate::FCore::Kind::USERDEFINED, g.clone(), checkDuplicate.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn mkTypeNode(mut inGraph: Graph, mut inName: Name, mut inType: Arc<DAE::Type>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inGraph.clone(), inName.clone(), inType.clone())) {
        (g, _, _) => {
            let mut r: Ref;
            let mut g = (*g).clone();
            r = lastScopeRef(g.clone())?;
            g = FGraphBuildEnv::mkTypeNode(list![inType.clone()], r.clone(), (inName.clone()).clone(), g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn mkImportNode(mut inGraph: Graph, mut inImport: Arc<SCode::Element>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inGraph.clone(), inImport.clone())) {
        (g, _) => {
            let mut r: Ref;
            let mut g = (*g).clone();
            r = lastScopeRef(g.clone())?;
            g = FGraphBuildEnv::mkElementNode(inImport.clone(), r.clone(), crate::FCore::Kind::USERDEFINED, g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn mkDefunitNode(mut inGraph: Graph, mut inDu: Arc<SCode::Element>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inGraph.clone(), inDu.clone())) {
        (g, _) => {
            let mut r: Ref;
            let mut g = (*g).clone();
            r = lastScopeRef(g.clone())?;
            g = FGraphBuildEnv::mkElementNode(inDu.clone(), r.clone(), crate::FCore::Kind::USERDEFINED, g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn classInfToScopeType(mut inState: ClassInf::State) -> Option<FCore::ScopeType> {
    let mut outType: Option<FCore::ScopeType> = None;
    outType = (match inState.clone() {
        ClassInf::State::FUNCTION { .. } => Some(crate::FCore::ScopeType::FUNCTION_SCOPE),
        _ => Some(crate::FCore::ScopeType::CLASS_SCOPE),
    });
    outType
}

pub fn isEmpty(mut inGraph: Graph) -> bool {
    let mut b: bool = false;
    b = (match inGraph.clone() {
        FCore::Graph::EG { name: _ } => true,
        _ => false,
    });
    b
}

pub fn isNotEmpty(mut inGraph: Graph) -> bool {
    let mut b: bool = false;
    b = !(isEmpty(inGraph.clone()));
    b
}

pub fn isEmptyScope(mut graph: Graph) -> bool {
    let mut isEmpty: bool = false;
    match '__try0: {
        isEmpty = FCore::RefTree::isEmpty(FNode::children(FNode::fromRef(lastScopeRef(graph.clone()).unwrap()).unwrap()).unwrap());
        Ok::<_, anyhow::Error>((isEmpty.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isEmpty = __try0_o0;
        }
        Err(_) => {
            isEmpty = true;
        }
    }
    isEmpty
}

pub fn printGraphStr(mut inGraph: Graph) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = (literal!("NOT IMPLEMENTED YET")).clone();
    s
}

pub fn inFunctionScope(mut inGraph: Graph) -> Result<bool> {
    let mut inFunction: bool = false;
    inFunction = (match inGraph.clone() {
        FCore::Graph::G { scope: ref s, .. } if (checkScopeType(s.clone(), Some(crate::FCore::ScopeType::FUNCTION_SCOPE))? || checkScopeType(s.clone(), Some(crate::FCore::ScopeType::PARALLEL_SCOPE))?) => {
            true
        },
        _ => {
            false
        },
    });
    Ok(inFunction)
}

pub fn getScopeName(mut inGraph: Graph) -> Result<Name> {
    let mut name: Name = arcstr::literal!("");
    name = ((match inGraph.clone() {
        _ => {
            let mut r: Ref;
            r = lastScopeRef(inGraph.clone())?;
            let false = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
            name = (FNode::refName(r.clone())?).clone();
            name.clone()
        },
    })).clone();
    Ok(name)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn checkScopeType(mut inScope: Scope, mut inScopeType: Option<FCore::ScopeType>) -> Result<bool> {
    let mut yes: bool = false;
    yes = 'mc: {
        let __mc_input = (inScope.clone(), inScopeType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: _ }, _) => {
                    let mut restr: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let true = (FNode::isRefClass(r.clone())?) else { bail!("pattern mismatch") };
                    restr = SCodeUtil::getClassRestriction(FNode::getElement(FNode::fromRef(r.clone())?)?)?;
                    let true = (restrictionToScopeType(restr.clone()) == inScopeType.clone()) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: _ }, _) => {
                    let mut st: Option<FCore::ScopeType> = None;
                    let FCore::N { data: FCore::ND { scopeType: __pa0 }, .. } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    st = __pa0.clone();
                    let true = (st.clone() == inScopeType.clone()) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    Ok(checkScopeType(rest.clone(), inScopeType.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(yes)
}

pub fn lastScopeRestriction(mut inGraph: Graph) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let mut s: Scope = metamodelica::nil();
    let FCore::G { scope: __pa0, .. } = (inGraph.clone()) else { bail!("pattern mismatch") };
    s = __pa0.clone();
    outRestriction = getScopeRestriction(s.clone())?;
    Ok(outRestriction)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getScopeRestriction(mut inScope: Scope) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction = SCode::Restriction::R_BLOCK;
    outRestriction = 'mc: {
        let __mc_input = inScope.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    if !((FNode::isRefClass(r.clone())?)) { bail!("guard") }
                    Ok(SCodeUtil::getClassRestriction(FNode::getElement(FNode::fromRef(r.clone())?)?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let mut st: FCore::ScopeType = FCore::ScopeType::CLASS_SCOPE;
                    let FCore::N { data: FCore::ND { scopeType: Some(__pa0) }, .. } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    st = __pa0.clone();
                    Ok(scopeTypeToRestriction(st.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(getScopeRestriction(listRest(inScope.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRestriction)
}

pub fn getGraphPathNoImplicitScope(mut inGraph: Graph) -> Result<Option<Arc<Absyn::Path>>> {
    let mut outAbsynPathOption: Option<Arc<Absyn::Path>> = None;
    outAbsynPathOption = getGraphPathNoImplicitScope_dispatch(currentScope(inGraph.clone())?)?;
    Ok(outAbsynPathOption)
}

fn getGraphPathNoImplicitScope_dispatch(mut inScope: Scope) -> Result<Option<Arc<Absyn::Path>>> {
    let mut outAbsynPathOption: Option<Arc<Absyn::Path>> = None;
    let mut opath: Option<Arc<Absyn::Path>> = None;
    outAbsynPathOption = 'mc: {
        let __mc_input = inScope.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#ref, tail: rest } => {
                    if !((!(FNode::isRefTop(r#ref.clone())?))) { bail!("guard") }
                    let mut id: Name = arcstr::literal!("");
                    let mut path: Arc<Absyn::Path>;
                    let mut path_1: Arc<Absyn::Path>;
                    let mut opath: Option<Arc<Absyn::Path>> = opath.clone();
                    id = (FNode::refName(r#ref.clone())?).clone();
                    if isImplicitScope((id.clone()).clone())? {
                        opath = getGraphPathNoImplicitScope_dispatch(rest.clone())?;
                    } else {
                        opath = getGraphPathNoImplicitScope_dispatch(rest.clone())?;
                        if isSome(opath.clone()) {
                            let __pa0 = ::match_deref::match_deref! { match &(opath.clone()) {
                                        Some(__pa0) => __pa0.clone(),
                                        _ => bail!("pattern mismatch"),
                            } };
                            path = __pa0.clone();
                            path_1 = AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
                            opath = Some(path_1.clone());
                        } else {
                            opath = Some(Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }));
                        }
                    }
                    Ok(opath.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynPathOption)
}

pub fn isImplicitScope(mut inName: Name) -> Result<bool> {
    let mut isImplicit: bool = false;
    isImplicit = FCore::isImplicitScope((inName.clone()).clone())?;
    Ok(isImplicit)
}

pub fn joinScopePath(mut inGraph: Graph, mut inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut opath: Option<Arc<Absyn::Path>> = None;
    let mut envPath: Arc<Absyn::Path>;
    opath = getScopePath(inGraph.clone())?;
    if isSome(opath.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(opath.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        envPath = __pa0.clone();
        outPath = AbsynUtil::joinPaths(envPath.clone(), inPath.clone())?;
    } else {
        outPath = inPath.clone();
    }
    Ok(outPath)
}

pub fn splitGraphScope(mut inGraph: Graph) -> Result<(Graph, Scope)> {
    let mut outRealGraph: Graph;
    let mut outForScope: Scope = metamodelica::nil();
    (outRealGraph, outForScope) = splitGraphScope_dispatch(inGraph.clone(), metamodelica::nil())?;
    Ok((outRealGraph, outForScope))
}

pub fn splitGraphScope_dispatch(mut inGraph: Graph, mut inAcc: Scope) -> Result<(Graph, Scope)> {
    let mut outRealGraph: Graph;
    let mut outForScope: Scope = metamodelica::nil();
    (outRealGraph, outForScope) = (::match_deref::match_deref! { match &((inGraph.clone(), inAcc.clone())) {
        (FCore::Graph::EG { name: _ }, _) => {
            (inGraph.clone(), inAcc.clone().reverse())
        },
        (FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. }, _) => {
            let mut g: Graph;
            let mut s: Scope = metamodelica::nil();
            if FNode::isImplicitRefName(r.clone())? {
                (g, _) = stripLastScopeRef(inGraph.clone())?;
                (g, s) = splitGraphScope_dispatch(g.clone(), cons(r.clone(), inAcc.clone()))?;
            } else {
                g = inGraph.clone();
                s = inAcc.clone().reverse();
            }
            (g.clone(), s.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outRealGraph, outForScope))
}

pub fn getVariablesFromGraphScope(mut inGraph: Graph) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut variables: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    variables = (::match_deref::match_deref! { match &(inGraph.clone()) {
        FCore::Graph::EG { name: _ } => {
            metamodelica::nil()
        },
        FCore::Graph::G { scope: Deref @ metamodelica::List::Nil, .. } => {
            metamodelica::nil()
        },
        FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. } => {
            let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            lst = List::map(FNode::filter(r.clone(), (std::sync::Arc::new(FNode::isRefComponent) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>))?, (std::sync::Arc::new(FNode::refName) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<ArcStr> + 'static>));
            lst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(variables)
}

pub fn removeComponentsFromScope(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut r: Ref;
    let mut n: Node = <FCore::Node as ::std::default::Default>::default();
    r = lastScopeRef(inGraph.clone())?;
    r = FNode::copyRefNoUpdate(r.clone());
    n = FNode::fromRef(r.clone())?;
    n = FNode::setChildren(n.clone(), FCore::RefTree::new())?;
    r = FNode::updateRef(r.clone(), n.clone())?;
    (outGraph, _) = stripLastScopeRef(inGraph.clone())?;
    outGraph = pushScopeRef(outGraph.clone(), r.clone())?;
    Ok(outGraph)
}

pub fn cloneLastScopeRef(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut r: Ref;
    (outGraph, r) = stripLastScopeRef(inGraph.clone())?;
    r = FNode::copyRefNoUpdate(r.clone());
    outGraph = pushScopeRef(outGraph.clone(), r.clone())?;
    Ok(outGraph)
}

pub fn updateScope(mut inGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        _ => inGraph.clone(),
    });
    outGraph
}

pub fn mkVersionNode(mut inSourceEnv: Graph, mut inSourceName: Name, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inTargetClassEnv: Graph, mut inTargetClass: Arc<SCode::Element>, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>) -> Result<(Graph, Arc<SCode::Element>, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outVersionedTargetClassEnv: Graph;
    let mut outVersionedTargetClass: Arc<SCode::Element>;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    (outVersionedTargetClassEnv, outVersionedTargetClass, outIH) = 'mc: {
        let __mc_input = (inSourceEnv.clone(), inSourceName.clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut gclass: Graph;
                    let mut classRef: Ref;
                    let mut sourceRef: Ref;
                    let mut targetClassParentRef: Ref;
                    let mut crefPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
                    let mut c: Arc<SCode::Element>;
                    let mut targetClassName: Name = arcstr::literal!("");
                    let mut newTargetClassName: Name = arcstr::literal!("");
                    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
                    c = inTargetClass.clone();
                    gclass = inTargetClassEnv.clone();
                    targetClassName = (SCodeUtil::elementName(c.clone())?).clone();
                    (newTargetClassName, crefPrefix) = mkVersionName(inSourceEnv.clone(), (inSourceName.clone()).clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), (targetClassName.clone()).clone())?;
                    sourceRef = FNode::child(lastScopeRef(inSourceEnv.clone())?, (inSourceName.clone()).clone())?;
                    let _ = cons(sourceRef.clone(), currentScope(inSourceEnv.clone())?);
                    targetClassParentRef = lastScopeRef(inTargetClassEnv.clone())?;
                    classRef = FNode::child(targetClassParentRef.clone(), (targetClassName.clone()).clone())?;
                    classRef = FNode::copyRefNoUpdate(classRef.clone());
                    let FCore::CL { e: __pa0, .. } = (FNode::refData(classRef.clone())?) else { bail!("pattern mismatch") };
                    c = __pa0.clone();
                    c = SCodeUtil::setClassName((newTargetClassName.clone()).clone(), c.clone())?;
                    classRef = updateClassElement(classRef.clone(), c.clone(), crefPrefix.clone(), inMod.clone(), FCore::Status::CLS_INSTANCE { instanceOf: (targetClassName.clone()).clone() }, empty())?;
                    FNode::addChildRef(targetClassParentRef.clone(), (newTargetClassName.clone()).clone(), classRef.clone(), false)?;
                    sourceRef = updateSourceTargetScope(sourceRef.clone(), cons(classRef.clone(), currentScope(gclass.clone())?))?;
                    ih = inIH.clone();
                    Ok((gclass.clone(), c.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut c: Arc<SCode::Element>;
                    let mut targetClassName: Name = arcstr::literal!("");
                    let mut newTargetClassName: Name = arcstr::literal!("");
                    c = inTargetClass.clone();
                    targetClassName = (SCodeUtil::elementName(c.clone())?).clone();
                    (newTargetClassName, _) = mkVersionName(inSourceEnv.clone(), (inSourceName.clone()).clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), (targetClassName.clone()).clone())?;
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.mkVersionNode: failed to create version node:\n")); __mm_s.push_str(&*literal!("Instance: CL(")); __mm_s.push_str(&*getGraphNameStr(inSourceEnv.clone())?); __mm_s.push_str(&*literal!(").CO(")); __mm_s.push_str(&*inSourceName.clone()); __mm_s.push_str(&*literal!(").CL(")); __mm_s.push_str(&*getGraphNameStr(inTargetClassEnv.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*targetClassName.clone()); __mm_s.push_str(&*SCodeDump::printModStr(Mod::unelabMod(inMod.clone())?, SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(")\n\t")); __mm_s.push_str(&*newTargetClassName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVersionedTargetClassEnv, outVersionedTargetClass, outIH))
}

pub fn createVersionScope(mut inSourceEnv: Graph, mut inSourceName: Name, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inTargetClassEnv: Graph, mut inTargetClass: Arc<SCode::Element>, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>) -> Result<(Graph, Arc<SCode::Element>, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outVersionedTargetClassEnv: Graph;
    let mut outVersionedTargetClass: Arc<SCode::Element>;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    (outVersionedTargetClassEnv, outVersionedTargetClass, outIH) = 'mc: {
        let __mc_input = (inSourceEnv.clone(), inSourceName.clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Mod::NOMOD, _, _, _) => {
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, .. }, _, _, _) => {
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let true = (Config::acceptMetaModelicaGrammar()? || isTargetClassBuiltin(inTargetClassEnv.clone(), inTargetClass.clone())? || inFunctionScope(inSourceEnv.clone())? || SCodeUtil::isOperatorRecord(inTargetClass.clone())) else { bail!("pattern mismatch") };
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let true = (stringEq((AbsynUtil::pathFirstIdent(getGraphName(inTargetClassEnv.clone())?)?).clone(), (literal!("OpenModelica")).clone())) else { bail!("pattern mismatch") };
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut gclass: Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    (gclass, c, outIH) = mkVersionNode(inSourceEnv.clone(), (inSourceName.clone()).clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone())?;
                    Ok((gclass.clone(), c.clone(), outIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVersionedTargetClassEnv, outVersionedTargetClass, outIH))
}

pub fn isTargetClassBuiltin(mut inGraph: Graph, mut inClass: Arc<SCode::Element>) -> Result<bool> {
    let mut yes: bool = false;
    yes = 'mc: {
        let __mc_input = (inGraph.clone(), inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut r: Ref;
                    let mut yes: bool = yes.clone();
                    r = FNode::child(lastScopeRef(inGraph.clone())?, (SCodeUtil::elementName(inClass.clone())?).clone())?;
                    yes = FNode::isRefBasicType(r.clone())? || FNode::isRefBuiltin(r.clone())?;
                    Ok(yes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(yes)
}

pub fn mkVersionName(mut inSourceEnv: Graph, mut inSourceName: Name, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inTargetClassEnv: Graph, mut inTargetClassName: Name) -> Result<(Name, DAE::Prefix)> {
    let mut outName: Name = arcstr::literal!("");
    let mut outCrefPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
    (outName, outCrefPrefix) = (::match_deref::match_deref! { match &((inSourceEnv.clone(), inSourceName.clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), inTargetClassName.clone())) {
        (_, _, _, _, _, _) => {
            let mut crefPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
            let mut name: Name = arcstr::literal!("");
            crefPrefix = PrefixUtil::prefixAdd((inSourceName.clone()).clone(), metamodelica::nil(), metamodelica::nil(), inPrefix.clone(), openmodelica_frontend_types::SCode::Variability::CONST, ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, Absyn::dummyInfo.clone())?;
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inTargetClassName.clone()); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*AbsynUtil::pathString(AbsynUtil::stringListPath(AbsynUtil::pathToStringList(PrefixUtil::prefixToPath(crefPrefix.clone())?)?.reverse()), (literal!("$")).clone(), false, false)?); ArcStr::from(__mm_s) }).clone();
            (name.clone(), crefPrefix.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outName, outCrefPrefix))
}

pub fn getClassPrefix(mut inEnv: FCore::Graph, mut inClassName: Name) -> Result<DAE::Prefix> {
    let mut outPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
    outPrefix = 'mc: {
        let __mc_input = (inEnv.clone(), inClassName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut p: DAE::Prefix = DAE::Prefix::NOPRE;
            let mut r: Ref;
            r = FNode::child(lastScopeRef(inEnv.clone())?, (inClassName.clone()).clone())?;
            let FCore::CL { pre: __pa0, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            p = __pa0.clone();
            Ok(p.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(openmodelica_frontend_types::DAE::Prefix::NOPRE)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPrefix)
}

pub fn isInstance(mut inEnv: FCore::Graph, mut inName: ArcStr) -> Result<bool> {
    let mut yes: bool = false;
    yes = 'mc: {
        let __mc_input = (inEnv.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: _ }, .. } = (FNode::refData(FNode::child(lastScopeRef(inEnv.clone())?, (inName.clone()).clone())?)?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(yes)
}

pub fn getInstanceOriginalName(mut inEnv: FCore::Graph, mut inName: ArcStr) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    outName = ('mc: {
        let __mc_input = (inEnv.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut outName: ArcStr = outName.clone();
            let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: __pa0 }, .. } = (FNode::refData(FNode::child(lastScopeRef(inEnv.clone())?, (inName.clone()).clone())?)?) else { bail!("pattern mismatch") };
            outName = __pa0.clone();
            Ok(outName.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inName.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outName)
}

pub fn graphPrefixOf(mut inPrefixEnv: Graph, mut inEnv: Graph) -> Result<bool> {
    let mut outIsPrefix: bool = false;
    outIsPrefix = graphPrefixOf2(currentScope(inPrefixEnv.clone())?.reverse(), currentScope(inEnv.clone())?.reverse())?;
    Ok(outIsPrefix)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn graphPrefixOf2(mut inPrefixEnv: Scope, mut inEnv: Scope) -> Result<bool> {
    let mut outIsPrefix: bool = false;
    outIsPrefix = (::match_deref::match_deref! { match &((inPrefixEnv.clone(), inEnv.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            true
        },
        (Deref @ metamodelica::List::Cons { head: r1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: r2, tail: rest2 }) if (stringEq((FNode::refName(r1.clone())?).clone(), (FNode::refName(r2.clone())?).clone())) => {
            graphPrefixOf2(rest1.clone(), rest2.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsPrefix)
}

pub fn setStatus(mut inEnv: Graph, mut inName: Name, mut inStatus: FCore::Data) -> Result<Graph> {
    let mut outEnv: Graph;
    outEnv = 'mc: {
        let __mc_input = (inEnv.clone(), inName.clone(), inStatus.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut n: Node = <FCore::Node as ::std::default::Default>::default();
            let mut r#ref: Ref;
            let mut refParent: Ref;
            refParent = lastScopeRef(g.clone())?;
            if FNode::refHasChild(refParent.clone(), (inName.clone()).clone())? {
                r#ref = FNode::child(refParent.clone(), (inName.clone()).clone())?;
                if FNode::refHasChild(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone())? {
                    r#ref = FNode::child(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone())?;
                    n = FNode::setData(FNode::fromRef(r#ref.clone())?, inStatus.clone())?;
                    r#ref = FNode::updateRef(r#ref.clone(), n.clone())?;
                } else {
                    (g, n) = node(g.clone(), (arcstr::literal!(FNode::statusNodeName)).clone(), list![r#ref.clone()], inStatus.clone());
                    FNode::addChildRef(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone(), FNode::toRef(n.clone()), false)?;
                }
            }
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _) = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.setStatus failed on: ")); __mm_s.push_str(&*getGraphNameStr(g.clone())?); __mm_s.push_str(&*literal!(" element: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(g.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

pub fn getStatus(mut inEnv: Graph, mut inName: Name) -> Result<FCore::Data> {
    let mut outStatus: FCore::Data = FCore::Data::TOP;
    outStatus = (match (inEnv.clone(), inName.clone()) {
        (mut g, _) => {
            let mut r#ref: Ref;
            let mut refParent: Ref;
            let mut s: FCore::Data = FCore::Data::TOP;
            refParent = lastScopeRef(g.clone())?;
            let true = (FNode::refHasChild(refParent.clone(), (inName.clone()).clone())?) else { bail!("pattern mismatch") };
            r#ref = FNode::child(refParent.clone(), (inName.clone()).clone())?;
            let true = (FNode::refHasChild(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone())?) else { bail!("pattern mismatch") };
            r#ref = FNode::child(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone())?;
            s = FNode::refData(r#ref.clone())?;
            s.clone()
        },
        (_, _) => {
            bail!("fail")
        },
    });
    Ok(outStatus)
}

pub fn selectScope(mut inEnv: Graph, mut inPath: Arc<Absyn::Path>) -> Result<Graph> {
    let mut outEnv: Graph;
    outEnv = (::match_deref::match_deref! { match &((inEnv.clone(), inPath.clone())) {
        (_, _) => {
            let mut env: Graph;
            let mut pl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut lp: i32 = 0;
            let mut le: i32 = 0;
            let mut diff: i32 = 0;
            let mut cs: Scope = metamodelica::nil();
            let mut p: Arc<Absyn::Path>;
            p = AbsynUtil::stripLast(inPath.clone())?;
            let true = (AbsynUtil::pathPrefixOf(p.clone(), getGraphName(inEnv.clone())?)) else { bail!("pattern mismatch") };
            pl = AbsynUtil::pathToStringList(p.clone())?;
            lp = (pl.clone().len() as i32);
            cs = currentScope(inEnv.clone())?;
            le = (cs.clone().len() as i32) - 1;
            diff = le.clone() - lp.clone();
            cs = List::stripN(cs.clone(), diff.clone())?;
            env = setScope(inEnv.clone(), cs.clone())?;
            env.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

pub fn makeScopePartial(mut inEnv: Graph) -> Graph {
    let mut outEnv: Graph = inEnv.clone();
    let mut node: Node = <FCore::Node as ::std::default::Default>::default();
    let mut data: Data = FCore::Data::TOP;
    let mut el: Arc<SCode::Element>;
    if '__try0: {
        node = unwrap_break_err!(FNode::fromRef(lastScopeRef(inEnv.clone()).unwrap()), '__try0);
        node = (match node.clone() {
        FCore::Node { data: ref data @ FCore::Data::CL { e: ref el, .. }, .. } => {
            let mut data = data.clone();
            let mut el = el.clone();
            el = SCodeUtil::makeClassPartial(el.clone());
            let __owned_variant_e_0 = el.clone();
            if let FCore::Data::CL { e, .. } = &mut data {
                *e = __owned_variant_e_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Data::CL"); }
            node.data = data.clone();
            node.clone()
        },
        _ => node.clone(),
    });
        outEnv = unwrap_break_err!(setLastScopeRef(FNode::toRef(node.clone()), outEnv.clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    outEnv
}

pub fn isPartialScope(mut inEnv: Graph) -> bool {
    let mut outIsPartial: bool = false;
    let mut el: Arc<SCode::Element>;
    match '__try0: {
        let FCore::N { data: FCore::CL { e: __pa1, .. }, .. } = (unwrap_break_err!(FNode::fromRef(lastScopeRef(inEnv.clone()).unwrap()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        el = __pa1.clone();
        outIsPartial = SCodeUtil::isPartial(el.clone());
        Ok::<_, anyhow::Error>((outIsPartial.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outIsPartial = __try0_o0;
        }
        Err(_) => {
            outIsPartial = false;
        }
    }
    outIsPartial
}

