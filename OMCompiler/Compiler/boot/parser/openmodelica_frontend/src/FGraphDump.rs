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

use crate::FCore::RefTree;
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_susan::GraphML;
use openmodelica_util::Flags;
use openmodelica_util::Util;

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

pub type Type = Arc<DAE::Type>;

pub type Types = Arc<metamodelica::List<Arc<DAE::Type>>>;

pub fn dumpGraph(mut inGraph: Graph, mut fileName: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = fileName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (Flags::isSet(Flags::GRAPH_INST_GEN_GRAPH.clone())?) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut g: i32 = 0;
            let mut gi: GraphML::GraphInfo;
            let mut nr: Ref;
            gi = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("G")).clone(), false, gi.clone())?;
            gi = __pa0.clone();
            g = __pa1.clone();
            nr = FGraph::top(inGraph.clone())?;
            (gi, g) = addNodes((gi.clone(), g.clone()), list![nr.clone()])?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dumping graph file: ")); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!(" ....\n")); ArcStr::from(__mm_s) }).clone());
            GraphML::dumpGraph(gi.clone(), (fileName.clone()).clone())?;
            println!("{}", (literal!("Dumped\n")).clone());
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addNodes(mut gin: (GraphML::GraphInfo, i32), mut inRefs: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>) -> Result<(GraphML::GraphInfo, i32)> {
    let mut gout: (GraphML::GraphInfo, i32);
    gout = (::match_deref::match_deref! { match &((gin.clone(), inRefs.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            gin.clone()
        },
        (g, Deref @ metamodelica::List::Cons { head: n, tail: rest }) if (!(FNode::isRefTop(n.clone())?) && !(FNode::isRefUserDefined(n.clone())?)) => {
            addNodes(g.clone(), rest.clone())?
        },
        (g, Deref @ metamodelica::List::Cons { head: n, tail: rest }) => {
            let mut g = (*g).clone();
            g = addNode(g.clone(), FNode::fromRef(n.clone())?)?;
            addNodes(g.clone(), rest.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(gout)
}

fn addNode(mut gin: (GraphML::GraphInfo, i32), mut node: Node) -> Result<(GraphML::GraphInfo, i32)> {
    let mut gout: (GraphML::GraphInfo, i32);
    gout = (::match_deref::match_deref! { match &((gin.clone(), node.clone())) {
        ((gi, i), FCore::Node { children: kids, parents: Deref @ metamodelica::List::Nil, .. }) => {
            let mut nds: ArcStr = arcstr::literal!("");
            let mut color: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            let mut shape: GraphML::ShapeType = GraphML::ShapeType::DIAMOND;
            let mut nrefs: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
            let mut label: GraphML::NodeLabel;
            let mut gi = (*gi).clone();
            let mut i = (*i).clone();
            (color, shape, nds) = graphml(node.clone(), true)?;
            labelText = (nds.clone()).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (gi, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], shape.clone(), None, metamodelica::nil(), i.clone(), gi.clone())?;
            nrefs = FCore::RefTree::listValues(kids.clone(), metamodelica::nil());
            (gi, i) = addNodes((gi.clone(), i.clone()), nrefs.clone())?;
            (gi.clone(), i.clone())
        },
        ((gi, i), FCore::Node { data: FCore::Data::REF { target: Deref @ metamodelica::List::Nil }, children: kids, parents: Deref @ metamodelica::List::Cons { head: nr, tail: _ }, .. }) => {
            let mut nds: ArcStr = arcstr::literal!("");
            let mut color: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            let mut shape: GraphML::ShapeType = GraphML::ShapeType::DIAMOND;
            let mut nrefs: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
            let mut label: GraphML::NodeLabel;
            let mut gi = (*gi).clone();
            let mut i = (*i).clone();
            (color, shape, nds) = graphml(node.clone(), true)?;
            labelText = (nds.clone()).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (gi, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], shape.clone(), None, metamodelica::nil(), i.clone(), gi.clone())?;
            (gi, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("r")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(FNode::fromRef(nr.clone())?)?)); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_RED)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), gi.clone())?;
            nrefs = FCore::RefTree::listValues(kids.clone(), metamodelica::nil());
            (gi, i) = addNodes((gi.clone(), i.clone()), nrefs.clone())?;
            (gi.clone(), i.clone())
        },
        ((gi, i), FCore::Node { data: FCore::Data::REF { target: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, children: kids, parents: Deref @ metamodelica::List::Cons { head: nr, tail: _ }, .. }) => {
            let mut nds: ArcStr = arcstr::literal!("");
            let mut color: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            let mut shape: GraphML::ShapeType = GraphML::ShapeType::DIAMOND;
            let mut nrefs: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
            let mut label: GraphML::NodeLabel;
            let mut gi = (*gi).clone();
            let mut i = (*i).clone();
            (color, shape, nds) = graphml(node.clone(), true)?;
            labelText = (nds.clone()).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (gi, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], shape.clone(), None, metamodelica::nil(), i.clone(), gi.clone())?;
            (gi, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("r")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(FNode::fromRef(nr.clone())?)?)); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_GREEN)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), gi.clone())?;
            nrefs = FCore::RefTree::listValues(kids.clone(), metamodelica::nil());
            (gi, i) = addNodes((gi.clone(), i.clone()), nrefs.clone())?;
            (gi.clone(), i.clone())
        },
        ((gi, i), FCore::Node { data: FCore::Data::VR { .. }, parents: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }) => {
            (gi.clone(), i.clone())
        },
        ((gi, i), FCore::Node { children: kids, parents: Deref @ metamodelica::List::Cons { head: nr, tail: _ }, .. }) => {
            let mut nds: ArcStr = arcstr::literal!("");
            let mut color: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            let mut shape: GraphML::ShapeType = GraphML::ShapeType::DIAMOND;
            let mut nrefs: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
            let mut label: GraphML::NodeLabel;
            let mut gi = (*gi).clone();
            let mut i = (*i).clone();
            (color, shape, nds) = graphml(node.clone(), true)?;
            labelText = (nds.clone()).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (gi, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], shape.clone(), None, metamodelica::nil(), i.clone(), gi.clone())?;
            (gi, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(node.clone())?)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(FNode::id(FNode::fromRef(nr.clone())?)?)); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), gi.clone())?;
            nrefs = FCore::RefTree::listValues(kids.clone(), metamodelica::nil());
            (gi, i) = addNodes((gi.clone(), i.clone()), nrefs.clone())?;
            (gi.clone(), i.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(gout)
}

pub fn graphml(mut node: Node, mut escape: bool) -> Result<(ArcStr, GraphML::ShapeType, ArcStr)> {
    let mut color: ArcStr = arcstr::literal!("");
    let mut shape: GraphML::ShapeType = GraphML::ShapeType::DIAMOND;
    let mut nname: ArcStr = arcstr::literal!("");
    (color, shape, nname) = 'mc: {
        let __mc_input = node.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: FCore::Data::CL { e, .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let true = (SCodeUtil::isElementRedeclare(e.clone())?) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::isElementReplaceable(e.clone())?) else { bail!("pattern mismatch") };
                    b = FNode::isClassExtends(node.clone());
                    s = (if (b.clone()) {literal!("rdrpCE:")} else {literal!("rdrpC:")}).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_YELLOW), openmodelica_susan::GraphML::ShapeType::HEXAGON, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: FCore::Data::CL { e, .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let true = (SCodeUtil::isElementRedeclare(e.clone())?) else { bail!("pattern mismatch") };
                    b = FNode::isClassExtends(node.clone());
                    s = (if (b.clone()) {literal!("rdCE:")} else {literal!("rdC:")}).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_YELLOW), openmodelica_susan::GraphML::ShapeType::HEXAGON, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: FCore::Data::CL { e, .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (SCodeUtil::isElementReplaceable(e.clone())?) else { bail!("pattern mismatch") };
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rpC:")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_RED), openmodelica_susan::GraphML::ShapeType::RECTANGLE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: FCore::Data::CO { e, .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (SCodeUtil::isElementRedeclare(e.clone())?) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::isElementReplaceable(e.clone())?) else { bail!("pattern mismatch") };
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rdrpc:")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_YELLOW), openmodelica_susan::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: FCore::Data::CO { e, .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (SCodeUtil::isElementRedeclare(e.clone())?) else { bail!("pattern mismatch") };
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rdc:")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_YELLOW), openmodelica_susan::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: FCore::Data::CO { e, .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (SCodeUtil::isElementReplaceable(e.clone())?) else { bail!("pattern mismatch") };
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rpc:")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_RED), openmodelica_susan::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::CL { .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_GRAY), openmodelica_susan::GraphML::ShapeType::RECTANGLE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::CO { .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_WHITE), openmodelica_susan::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::EX { .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_GREEN), openmodelica_susan::GraphML::ShapeType::ROUNDRECTANGLE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::EXP { e: exp, .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (Dump::printExpStr(exp.clone())?).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*if (escape.clone()) {Util::escapeModelicaStringToXmlString((s.clone()).clone())?} else {Util::stringTrunc((s.clone()).clone(), 100)?}); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_PURPLE), openmodelica_susan::GraphML::ShapeType::HEXAGON, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::DIMS { dims, .. } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (Dump::printArraydimStr(dims.clone())?).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*if (escape.clone()) {Util::escapeModelicaStringToXmlString((s.clone()).clone())?} else {Util::stringTrunc((s.clone()).clone(), 100)?}); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_PINK), openmodelica_susan::GraphML::ShapeType::TRIANGLE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::CR { r } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*Dump::printComponentRefStr(r.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_PURPLE), openmodelica_susan::GraphML::ShapeType::OCTAGON, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::ASSERT { message: s } } => {
                    let mut s = (*s).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_RED), openmodelica_susan::GraphML::ShapeType::PARALLELOGRAM, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::REF { target: Deref @ metamodelica::List::Nil } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*literal!("UNRESOLVED")); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_RED), openmodelica_susan::GraphML::ShapeType::PARALLELOGRAM, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd @ FCore::Data::REF { target: Deref @ metamodelica::List::Cons { head: target, tail: _ } } } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(target.clone())?)?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_GREEN), openmodelica_susan::GraphML::ShapeType::TRAPEZOID, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: _, children: _, data: nd } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*FNode::dataStr(nd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*FNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((arcstr::literal!(GraphML::COLOR_BLUE), openmodelica_susan::GraphML::ShapeType::ELLIPSE, s.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((color, shape, nname))
}

