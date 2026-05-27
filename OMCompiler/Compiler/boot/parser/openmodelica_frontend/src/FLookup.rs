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

use crate::FCore;
use crate::FGraph;
use crate::FGraphBuild;
use crate::FNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util_datatypes_basic::List;

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

pub type ImportTable = FCore::ImportTable;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Msg = Option<SourceInfo>;

pub static dummyLookupOption: Option<SourceInfo> = None;

// SOME(Absyn.dummyInfo);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Options {
    pub ignoreImports: bool,
    pub ignoreExtends: bool,
    pub ignoreParents: bool,
}

pub type OPTIONS = Options;


pub static ignoreNothing: Options = Options { ignoreImports: false, ignoreExtends: false, ignoreParents: false };

pub static ignoreParents: Options = Options { ignoreImports: false, ignoreExtends: false, ignoreParents: true };

pub static ignoreParentsAndImports: Options = Options { ignoreImports: true, ignoreExtends: false, ignoreParents: true };

pub static ignoreAll: Options = Options { ignoreImports: true, ignoreExtends: true, ignoreParents: true };

pub fn id(mut inGraph: Graph, mut inRef: Ref, mut inName: Name, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph.clone(), inRef.clone(), inName.clone(), inOptions.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            r = FNode::child(inRef.clone(), (arcstr::literal!(FNode::forNodeName)).clone())?;
            r = FNode::child(r.clone(), (inName.clone()).clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, Options { ignoreImports: _, ignoreExtends: _, ignoreParents: false }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let mut p: Parents = metamodelica::nil();
            let true = (FNode::isRefImplicitScope(inRef.clone())?) else { bail!("pattern mismatch") };
            p = FNode::parents(FNode::fromRef(inRef.clone())?)?;
            r = FNode::original(p.clone())?;
            (g, r) = id(g.clone(), r.clone(), (inName.clone()).clone(), inOptions.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())?) else { bail!("pattern mismatch") };
            r = FNode::child(inRef.clone(), (inName.clone()).clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, Options { ignoreImports: false, ignoreExtends: _, ignoreParents: _ }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())?) else { bail!("pattern mismatch") };
            (g, r) = imp(g.clone(), inRef.clone(), (inName.clone()).clone(), inOptions.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, Options { ignoreImports: _, ignoreExtends: false, ignoreParents: _ }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())?) else { bail!("pattern mismatch") };
            (g, r) = ext(g.clone(), inRef.clone(), (inName.clone()).clone(), inOptions.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, Options { ignoreImports: _, ignoreExtends: _, ignoreParents: false }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isEncapsulated(FNode::fromRef(inRef.clone())?)?) else { bail!("pattern mismatch") };
            r = FNode::top(inRef.clone())?;
            (g, r) = id(g.clone(), r.clone(), (inName.clone()).clone(), inOptions.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, Options { ignoreImports: _, ignoreExtends: _, ignoreParents: false }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let mut p: Parents = metamodelica::nil();
            let false = (FNode::isRefImplicitScope(inRef.clone())?) else { bail!("pattern mismatch") };
            let false = (FNode::isEncapsulated(FNode::fromRef(inRef.clone())?)?) else { bail!("pattern mismatch") };
            let true = (FNode::hasParents(FNode::fromRef(inRef.clone())?)?) else { bail!("pattern mismatch") };
            p = FNode::parents(FNode::fromRef(inRef.clone())?)?;
            r = FNode::original(p.clone())?;
            (g, r) = search(g.clone(), list![r.clone()], (inName.clone()).clone(), inOptions.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, Options { ignoreImports: _, ignoreExtends: _, ignoreParents: false }, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (FNode::hasParents(FNode::fromRef(inRef.clone())?)?) else { bail!("pattern mismatch") };
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, Some(_)) = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FLookup.id failed for: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!(" in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(inRef.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outGraph, outRef))
}

pub fn search(mut inGraph: Graph, mut inRefs: Refs, mut inName: Name, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph.clone(), inRefs.clone(), inName.clone(), inOptions.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ metamodelica::List::Cons { head: r, tail: _ }, _, _, _) => {
                    let mut g = (*g).clone();
                    let mut r = (*r).clone();
                    (g, r) = id(g.clone(), r.clone(), (inName.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = search(g.clone(), rest.clone(), (inName.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Some(_)) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FLookup.search failed for: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!(" in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(listHead(inRefs.clone())?)?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outGraph, outRef))
}

pub fn name(mut inGraph: Graph, mut inRef: Ref, mut inPath: Arc<Absyn::Path>, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph.clone(), inRef.clone(), inPath.clone(), inOptions.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::Path::IDENT { name: i }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), (i.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::Path::QUALIFIED { name: i, path: rest }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), (i.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    (g, r) = name(g.clone(), r.clone(), rest.clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::Path::QUALIFIED { name: i, path: rest }, _, _) => {
                    let mut r: Ref;
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), (i.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    if '__try0: {
                        (_, _) = unwrap_break_err!(name(g.clone(), r.clone(), rest.clone(), inOptions.clone(), inMsg.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("missing: ")); __mm_s.push_str(&*AbsynUtil::pathString(rest.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" in scope: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); ArcStr::from(__mm_s) }).clone();
                    (g, r) = FGraphBuild::mkAssertNode((AbsynUtil::pathFirstIdent(rest.clone())?).clone(), (s.clone()).clone(), r.clone(), g.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::Path::FULLYQUALIFIED { path: rest }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    r = FNode::top(inRef.clone())?;
                    (g, r) = name(g.clone(), r.clone(), rest.clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Some(_)) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FLookup.name failed for: ")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(inRef.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outGraph, outRef))
}

pub fn ext(mut inGraph: Graph, mut inRef: Ref, mut inName: Name, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph.clone(), inRef.clone(), inName.clone(), inOptions.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let true = (FNode::isClassExtends(FNode::fromRef(inRef.clone())?)) else { bail!("pattern mismatch") };
            r = FNode::child(inRef.clone(), (arcstr::literal!(FNode::refNodeName)).clone())?;
            r = FNode::target(FNode::fromRef(r.clone())?)?;
            (g, r) = id(g.clone(), r.clone(), (inName.clone()).clone(), ignoreParents.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let true = (FNode::isClassExtends(FNode::fromRef(inRef.clone())?)) else { bail!("pattern mismatch") };
            r = FNode::original(FNode::parents(FNode::fromRef(inRef.clone())?)?)?;
            (g, r) = id(g.clone(), r.clone(), (inName.clone()).clone(), ignoreNothing.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let mut refs: Refs = metamodelica::nil();
            refs = FNode::extendsRefs(inRef.clone())?;
            let false = (refs.clone().is_empty()) else { bail!("pattern mismatch") };
            refs = List::mapMap(refs.clone(), (std::sync::Arc::new(FNode::fromRef) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<FCore::Node> + 'static>), (std::sync::Arc::new(FNode::target) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Node) -> Result<metamodelica::Array<FCore::Node>> + 'static>));
            (g, r) = search(g.clone(), refs.clone(), (inName.clone()).clone(), ignoreParentsAndImports.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outGraph, outRef))
}

pub fn imp(mut inGraph: Graph, mut inRef: Ref, mut inName: Name, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph.clone(), inRef.clone(), inName.clone(), inOptions.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let mut qi: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
            let true = (FNode::hasImports(FNode::fromRef(inRef.clone())?)?) else { bail!("pattern mismatch") };
            (qi, _) = FNode::imports(FNode::fromRef(inRef.clone())?)?;
            (g, r) = imp_qual(g.clone(), inRef.clone(), (inName.clone()).clone(), qi.clone(), inOptions.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let mut uqi: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
            let true = (FNode::hasImports(FNode::fromRef(inRef.clone())?)?) else { bail!("pattern mismatch") };
            (_, uqi) = FNode::imports(FNode::fromRef(inRef.clone())?)?;
            (g, r) = imp_unqual(g.clone(), inRef.clone(), (inName.clone()).clone(), uqi.clone(), inOptions.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outGraph, outRef))
}

fn imp_qual(mut inGraph: Graph, mut inRef: Ref, mut inName: Name, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph.clone(), inRef.clone(), inName.clone(), inImports.clone(), inOptions.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, _, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: rest_imps }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    let false = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (g, r) = imp_qual(g.clone(), inRef.clone(), (inName.clone()).clone(), rest_imps.clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, _, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { path, name }, tail: _ }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    let true = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (g, r) = fq(g.clone(), path.clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: _ }, _, _) => {
                    let true = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outGraph, outRef))
}

pub fn imp_unqual(mut inGraph: Graph, mut inRef: Ref, mut inName: Name, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph.clone(), inRef.clone(), inName.clone(), inImports.clone(), inOptions.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, _, Deref @ metamodelica::List::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = fq(g.clone(), path.clone(), inOptions.clone(), inMsg.clone())?;
                    (g, r) = id(g.clone(), r.clone(), (inName.clone()).clone(), ignoreParents.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, _, Deref @ metamodelica::List::Cons { head: _, tail: rest_imps }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = imp_unqual(g.clone(), inRef.clone(), (inName.clone()).clone(), rest_imps.clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outGraph, outRef))
}

pub fn fq(mut inGraph: Graph, mut inName: Arc<Absyn::Path>, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = name(inGraph.clone(), FGraph::top(inGraph.clone())?, inName.clone(), inOptions.clone(), inMsg.clone())?;
    Ok((outGraph, outRef))
}

pub fn cr(mut inGraph: Graph, mut inRef: Ref, mut inCref: Arc<Absyn::ComponentRef>, mut inOptions: Options, mut inMsg: Msg) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph.clone(), inRef.clone(), inCref.clone(), inOptions.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: i, subscripts: _ }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), (i.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::ComponentRef::CREF_QUAL { name: i, subscripts: _, componentRef: rest }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), (i.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    let true = (FNode::isRefComponent(r.clone())?) else { bail!("pattern mismatch") };
                    r = FNode::child(r.clone(), (arcstr::literal!(FNode::refNodeName)).clone())?;
                    r = FNode::target(FNode::fromRef(r.clone())?)?;
                    (g, r) = cr(g.clone(), r.clone(), rest.clone(), ignoreParents.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::ComponentRef::CREF_QUAL { name: i, subscripts: _, componentRef: rest }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), (i.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    let true = (FNode::isRefClass(r.clone())?) else { bail!("pattern mismatch") };
                    (g, r) = cr(g.clone(), r.clone(), rest.clone(), ignoreParents.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::ComponentRef::CREF_QUAL { name: i, subscripts: _, componentRef: rest }, _, _) => {
                    let mut r: Ref;
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), (i.clone()).clone(), inOptions.clone(), inMsg.clone())?;
                    let true = (FNode::isRefClass(r.clone())? || FNode::isRefComponent(r.clone())?) else { bail!("pattern mismatch") };
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("missing: ")); __mm_s.push_str(&*AbsynUtil::crefString(rest.clone())?); __mm_s.push_str(&*literal!(" in scope: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); ArcStr::from(__mm_s) }).clone();
                    (g, r) = FGraphBuild::mkAssertNode((AbsynUtil::crefFirstIdent(rest.clone())?).clone(), (s.clone()).clone(), r.clone(), g.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, _, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: rest }, _, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    r = FGraph::top(g.clone())?;
                    (g, r) = cr(g.clone(), r.clone(), rest.clone(), inOptions.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Some(_)) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FLookup.cr failed for: ")); __mm_s.push_str(&*AbsynUtil::crefString(inCref.clone())?); __mm_s.push_str(&*literal!(" in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(inRef.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outGraph, outRef))
}

