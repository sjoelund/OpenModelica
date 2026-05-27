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
use crate::FGraphBuild;
use crate::FLookup;
use crate::FNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::SCode;
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

pub type ImportTable = FCore::ImportTable;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Graph = FCore::Graph;

pub type Msg = Option<SourceInfo>;

pub fn ext(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match (inRef.clone(), ig.clone()) {
        (_, mut g) => {
            g = FNode::apply1(inRef.clone(), (std::sync::Arc::new(ext_one) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<FCore::Graph> + 'static>), g.clone())?;
            g.clone()
        },
    });
    Ok(og)
}

pub fn ext_one(mut name: Name, mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef.clone(), ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefExtends(r.clone())?) else { bail!("pattern mismatch") };
            let false = (FNode::isRefDerived(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefRefResolved(r.clone())?) else { bail!("pattern mismatch") };
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut rr: Ref;
            let mut p: Arc<Absyn::Path>;
            let mut e: Arc<SCode::Element>;
            let true = (FNode::isRefExtends(r.clone())?) else { bail!("pattern mismatch") };
            let false = (FNode::isRefDerived(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::EX { e: __pa0, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            e = __pa0.clone();
            p = SCodeUtil::getBaseClassPath(e.clone())?;
            let _ = SCodeUtil::elementInfo(e.clone());
            (g, rr) = FLookup::name(g.clone(), r.clone(), p.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone())?;
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), list![rr.clone()], r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Arc<Absyn::Path>;
            let mut e: Arc<SCode::Element>;
            let true = (FNode::isRefExtends(r.clone())?) else { bail!("pattern mismatch") };
            let false = (FNode::isRefDerived(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::EX { e: __pa0, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            e = __pa0.clone();
            p = SCodeUtil::getBaseClassPath(e.clone())?;
            let _ = SCodeUtil::elementInfo(e.clone());
            if '__try1: {
                (_, _) = unwrap_break_err!(FLookup::name(g.clone(), r.clone(), p.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try1);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FResolve.ext_one: baseclass: ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" not found in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone());
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(ig.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(og)
}

pub fn derived(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match (inRef.clone(), ig.clone()) {
        (_, mut g) => {
            g = FNode::apply1(inRef.clone(), (std::sync::Arc::new(derived_one) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<FCore::Graph> + 'static>), g.clone())?;
            g.clone()
        },
    });
    Ok(og)
}

pub fn derived_one(mut name: Name, mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef.clone(), ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefDerived(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefRefResolved(r.clone())?) else { bail!("pattern mismatch") };
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut rr: Ref;
            let mut p: Arc<Absyn::Path>;
            let true = (FNode::isRefDerived(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CL { e: __t1, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &((*__t1).clone()) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ }, .. }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            (g, rr) = FLookup::name(g.clone(), r.clone(), p.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone())?;
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), list![rr.clone()], r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Arc<Absyn::Path>;
            let true = (FNode::isRefDerived(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CL { e: __t1, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &((*__t1).clone()) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ }, .. }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            if '__try3: {
                (_, _) = unwrap_break_err!(FLookup::name(g.clone(), r.clone(), p.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try3);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FResolve.derived_one: baseclass: ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" not found in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone());
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(ig.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(og)
}

pub fn ty(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match (inRef.clone(), ig.clone()) {
        (_, mut g) => {
            g = FNode::apply1(inRef.clone(), (std::sync::Arc::new(ty_one) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<FCore::Graph> + 'static>), g.clone())?;
            g.clone()
        },
    });
    Ok(og)
}

pub fn ty_one(mut name: Name, mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef.clone(), ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefComponent(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefRefResolved(r.clone())?) else { bail!("pattern mismatch") };
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut rr: Ref;
            let mut p: Arc<Absyn::Path>;
            let mut e: Arc<SCode::Element>;
            let true = (FNode::isRefComponent(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CO { e: __pa0, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            e = __pa0.clone();
            p = SCodeUtil::getElementTypePath(e.clone())?;
            (g, rr) = FLookup::name(g.clone(), r.clone(), p.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone())?;
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), list![rr.clone()], r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Arc<Absyn::Path>;
            let mut e: Arc<SCode::Element>;
            let true = (FNode::isRefComponent(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CO { e: __pa0, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            e = __pa0.clone();
            p = SCodeUtil::getElementTypePath(e.clone())?;
            if '__try1: {
                (_, _) = unwrap_break_err!(FLookup::name(g.clone(), r.clone(), p.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try1);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FResolve.ty_one: component type path: ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" not found in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone());
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(ig.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(og)
}

pub fn cc(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match (inRef.clone(), ig.clone()) {
        (_, mut g) => {
            g = FNode::apply1(inRef.clone(), (std::sync::Arc::new(cc_one) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<FCore::Graph> + 'static>), g.clone())?;
            g.clone()
        },
    });
    Ok(og)
}

pub fn cc_one(mut name: Name, mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef.clone(), ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefConstrainClass(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefRefResolved(r.clone())?) else { bail!("pattern mismatch") };
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut rr: Ref;
            let mut p: Arc<Absyn::Path>;
            let true = (FNode::isRefConstrainClass(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CC { cc: __t1 } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &((*__t1).clone()) {
                Deref @ SCode::ConstrainClass { constrainingClass: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            (g, rr) = FLookup::name(g.clone(), r.clone(), p.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone())?;
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), list![rr.clone()], r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Arc<Absyn::Path>;
            let true = (FNode::isRefConstrainClass(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CC { cc: __t1 } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &((*__t1).clone()) {
                Deref @ SCode::ConstrainClass { constrainingClass: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            if '__try2: {
                (_, _) = unwrap_break_err!(FLookup::name(g.clone(), r.clone(), p.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try2);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FResolve.cc_one: constrained class: ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" not found in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone());
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(ig.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(og)
}

pub fn clsext(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match (inRef.clone(), ig.clone()) {
        (_, mut g) => {
            g = FNode::apply1(inRef.clone(), (std::sync::Arc::new(clsext_one) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<FCore::Graph> + 'static>), g.clone())?;
            g.clone()
        },
    });
    Ok(og)
}

pub fn clsext_one(mut name: Name, mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef.clone(), ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefClassExtends(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefRefResolved(r.clone())?) else { bail!("pattern mismatch") };
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut rr: Ref;
            let mut p: Ref;
            let mut id: Name = arcstr::literal!("");
            let true = (FNode::isRefClassExtends(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CL { e: __t1, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &((*__t1).clone()) {
                Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            id = __pa0.clone();
            let __pa2 = ::match_deref::match_deref! { match &(FNode::parents(FNode::fromRef(r.clone())?)?) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: _ } => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa2.clone();
            (g, rr) = FLookup::ext(g.clone(), p.clone(), (id.clone()).clone(), FLookup::ignoreParentsAndImports.clone(), FLookup::dummyLookupOption.clone())?;
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), list![rr.clone()], r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Ref;
            let mut id: Name = arcstr::literal!("");
            let true = (FNode::isRefClassExtends(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CL { e: __t1, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &((*__t1).clone()) {
                Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            id = __pa0.clone();
            let __pa2 = ::match_deref::match_deref! { match &(FNode::parents(FNode::fromRef(r.clone())?)?) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: _ } => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa2.clone();
            if '__try3: {
                (_, _) = unwrap_break_err!(FLookup::ext(g.clone(), p.clone(), (id.clone()).clone(), FLookup::ignoreParentsAndImports.clone(), FLookup::dummyLookupOption.clone()), '__try3);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FResolve.clsext_one: class extends: ")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" scope: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!(" not found in extends of: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(p.clone())?)?); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*stringDelimitList(List::map(List::map(FNode::extendsRefs(p.clone())?, (std::sync::Arc::new(FNode::fromRef) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<FCore::Node> + 'static>)), (std::sync::Arc::new(FNode::toPathStr) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Node) -> Result<ArcStr> + 'static>)), (literal!("\n\t")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(ig.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(og)
}

pub fn cr(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match (inRef.clone(), ig.clone()) {
        (_, mut g) => {
            g = FNode::apply1(inRef.clone(), (std::sync::Arc::new(cr_one) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<FCore::Graph> + 'static>), g.clone())?;
            g.clone()
        },
    });
    Ok(og)
}

pub fn cr_one(mut name: Name, mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef.clone(), ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefCref(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefRefResolved(r.clone())?) else { bail!("pattern mismatch") };
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut rr: Ref;
            let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let true = (FNode::isRefCref(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CR { r: __pa0 } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            cr = __pa0.clone();
            (g, rr) = FLookup::cr(g.clone(), r.clone(), cr.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone())?;
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), list![rr.clone()], r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let true = (FNode::isRefCref(r.clone())?) else { bail!("pattern mismatch") };
            let FCore::CR { r: __pa0 } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            cr = __pa0.clone();
            if '__try1: {
                (_, _) = unwrap_break_err!(FLookup::cr(g.clone(), r.clone(), cr.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try1);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FResolve.cr_one: component reference: ")); __mm_s.push_str(&*AbsynUtil::crefString(cr.clone())?); __mm_s.push_str(&*literal!(" not found in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone());
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(ig.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(og)
}

pub fn r#mod(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match (inRef.clone(), ig.clone()) {
        (_, mut g) => {
            g = FNode::apply1(inRef.clone(), (std::sync::Arc::new(mod_one) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<FCore::Graph> + 'static>), g.clone())?;
            g.clone()
        },
    });
    Ok(og)
}

pub fn mod_one(mut name: Name, mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef.clone(), ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefMod(r.clone())? && !(FNode::isRefModHolder(r.clone())?) && !(ClassInfUtil::isBasicTypeComponentName((FNode::refName(r.clone())?).clone()))) else { bail!("pattern mismatch") };
            let true = (FNode::isRefRefResolved(r.clone())?) else { bail!("pattern mismatch") };
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut rr: Ref;
            let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let true = (FNode::isRefMod(r.clone())? && !(FNode::isRefModHolder(r.clone())?) && !(ClassInfUtil::isBasicTypeComponentName((FNode::refName(r.clone())?).clone()))) else { bail!("pattern mismatch") };
            cr = AbsynUtil::pathToCref(AbsynUtil::stringListPath(FNode::namesUpToParentName(r.clone(), (arcstr::literal!(FNode::modNodeName)).clone())?))?;
            (g, rr) = FLookup::cr(g.clone(), FNode::getModifierTarget(r.clone())?, cr.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone())?;
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), list![rr.clone()], r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let true = (FNode::isRefMod(r.clone())? && !(FNode::isRefModHolder(r.clone())?) && !(ClassInfUtil::isBasicTypeComponentName((FNode::refName(r.clone())?).clone()))) else { bail!("pattern mismatch") };
            cr = AbsynUtil::pathToCref(AbsynUtil::stringListPath(FNode::namesUpToParentName(r.clone(), (arcstr::literal!(FNode::modNodeName)).clone())?))?;
            if '__try0: {
                (_, _) = unwrap_break_err!(FLookup::cr(g.clone(), FNode::getModifierTarget(r.clone())?, cr.clone(), FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FResolve.mod_one: modifier: ")); __mm_s.push_str(&*AbsynUtil::crefString(cr.clone())?); __mm_s.push_str(&*literal!(" not found in: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone());
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(ig.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(og)
}

pub fn elred(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match (inRef.clone(), ig.clone()) {
        (_, mut g) => {
            g = FNode::apply1(inRef.clone(), (std::sync::Arc::new(elred_one) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<FCore::Graph> + 'static>), g.clone())?;
            g.clone()
        },
    });
    Ok(og)
}

pub fn elred_one(mut name: Name, mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef.clone(), ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefRedeclare(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefClass(r.clone())? && !(FNode::isRefClassExtends(r.clone())?) || FNode::isRefComponent(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefRefResolved(r.clone())?) else { bail!("pattern mismatch") };
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut rr: Ref;
            let mut p: Ref;
            let mut id: Name = arcstr::literal!("");
            let true = (FNode::isRefRedeclare(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefClass(r.clone())? && !(FNode::isRefClassExtends(r.clone())?) || FNode::isRefComponent(r.clone())?) else { bail!("pattern mismatch") };
            id = (SCodeUtil::elementName(FNode::getElement(FNode::fromRef(r.clone())?)?)?).clone();
            let __pa0 = ::match_deref::match_deref! { match &(FNode::parents(FNode::fromRef(r.clone())?)?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            (g, rr) = FLookup::ext(g.clone(), p.clone(), (id.clone()).clone(), FLookup::ignoreParentsAndImports.clone(), FLookup::dummyLookupOption.clone())?;
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), list![rr.clone()], r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Ref;
            let mut id: Name = arcstr::literal!("");
            let true = (FNode::isRefRedeclare(r.clone())?) else { bail!("pattern mismatch") };
            let true = (FNode::isRefClass(r.clone())? && !(FNode::isRefClassExtends(r.clone())?) || FNode::isRefComponent(r.clone())?) else { bail!("pattern mismatch") };
            id = (SCodeUtil::elementName(FNode::getElement(FNode::fromRef(r.clone())?)?)?).clone();
            let __pa0 = ::match_deref::match_deref! { match &(FNode::parents(FNode::fromRef(r.clone())?)?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            if '__try1: {
                (_, _) = unwrap_break_err!(FLookup::ext(g.clone(), p.clone(), (id.clone()).clone(), FLookup::ignoreParentsAndImports.clone(), FLookup::dummyLookupOption.clone()), '__try1);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FResolve.elred_one: redeclare as element: ")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" scope: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!(" not found in extends of: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(p.clone())?)?); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*stringDelimitList(List::map(List::map(FNode::extendsRefs(p.clone())?, (std::sync::Arc::new(FNode::fromRef) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<FCore::Node> + 'static>)), (std::sync::Arc::new(FNode::toPathStr) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Node) -> Result<ArcStr> + 'static>)), (literal!("\n\t")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            g = FGraphBuild::mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), r.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(ig.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(og)
}

