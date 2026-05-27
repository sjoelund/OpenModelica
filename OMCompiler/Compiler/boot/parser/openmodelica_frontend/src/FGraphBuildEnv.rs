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
use crate::FNode;
use crate::SCodeInstUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Util;
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

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Graph = FCore::Graph;

pub type Scope = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub fn mkProgramGraph(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inKind: Kind, mut graph: Graph) -> Result<Graph> {
    let mut graph: Graph = graph;
    let mut topRef: Ref;
    topRef = FGraph::top(graph.clone())?;
    for mut cls in &*inProgram.clone() {
        let mut cls = cls.clone();
        graph = mkClassGraph(cls.clone(), topRef.clone(), inKind.clone(), graph.clone(), true)?;
    }
    Ok(graph)
}

fn mkClassGraph(mut inClass: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph, mut checkDuplicate: bool) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inClass.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (Deref @ SCode::Element::CLASS { .. }, _, _, g) => {
            let mut g = (*g).clone();
            g = mkClassNode(inClass.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), inParentRef.clone(), inKind.clone(), g.clone(), checkDuplicate.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub fn mkClassNode(mut inClass: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph, mut checkDuplicate: bool) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inClass.clone(), inPrefix.clone(), inMod.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, _, _, _, _, g) => {
            let mut cls: Arc<SCode::Element>;
            let mut name: ArcStr = arcstr::literal!("");
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            cls = SCodeInstUtil::expandEnumerationClass(inClass.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(cls.clone()) {
                Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::CL { e: cls.clone(), pre: inPrefix.clone(), r#mod: inMod.clone(), kind: inKind.clone(), status: crate::FCore::Status::CLS_UNTYPED });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), checkDuplicate.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn mkConstrainClass(mut inElement: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inElement.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) }, .. }, .. }, _, _, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::ccNodeName)).clone(), list![inParentRef.clone()], FCore::Data::CC { cc: cc.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::ccNodeName)).clone(), nr.clone(), false)?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) }, .. }, .. }, _, _, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::ccNodeName)).clone(), list![inParentRef.clone()], FCore::Data::CC { cc: cc.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::ccNodeName)).clone(), nr.clone(), false)?;
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

pub fn mkModNode(mut inName: Name, mut inMod: Arc<SCode::Mod>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inName.clone(), inMod.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::NOMOD, _, _, g) => {
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::MOD { binding: None, subModLst: Deref @ metamodelica::List::Nil, .. }, _, _, g) => {
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::MOD { binding: b @ Some(_), subModLst: Deref @ metamodelica::List::Nil, .. }, _, _, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::MO { m: inMod.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
                    g = mkBindingNode(b.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::MOD { binding: b, subModLst: sm, .. }, _, _, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::MO { m: inMod.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
                    g = mkSubMods(sm.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    g = mkBindingNode(b.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::REDECL { element: e, .. }, _, _, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::MO { m: inMod.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
                    g = mkElementNode(e.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, _, _, _, g) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraphBuildEnv.mkModNode failed with: ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" mod: ")); __mm_s.push_str(&*SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(g.clone())
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
pub fn mkSubMods(mut inSubMod: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inSubMod.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: id, r#mod: m }, tail: rest }, _, _, g) => {
            let mut g = (*g).clone();
            g = mkModNode((id.clone()).clone(), m.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g = mkSubMods(rest.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub fn mkBindingNode(mut inBinding: Option<Arc<Absyn::Exp>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inBinding.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (None, _, _, g) => {
            g.clone()
        },
        (Some(e), _, _, g) => {
            let mut g = (*g).clone();
            g = mkExpressionNode((arcstr::literal!(FNode::bndNodeName)).clone(), e.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

fn mkClassChildren(mut inClassDef: Arc<SCode::ClassDef>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inClassDef.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { externalDecl, clsattrs, constraintLst, initialAlgorithmLst: ials, normalAlgorithmLst: als, initialEquationLst: ieqs, normalEquationLst: eqs, elementLst: el }, _, _, g) => {
                    let mut g = (*g).clone();
                    g = List::fold2(el.clone(), (std::sync::Arc::new(mkElementNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), inParentRef.clone(), inKind.clone(), g.clone());
                    g = mkEqNode((arcstr::literal!(FNode::eqNodeName)).clone(), eqs.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkEqNode((arcstr::literal!(FNode::ieqNodeName)).clone(), ieqs.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkAlNode((arcstr::literal!(FNode::alNodeName)).clone(), als.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkAlNode((arcstr::literal!(FNode::ialNodeName)).clone(), ials.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkOptNode((arcstr::literal!(FNode::optNodeName)).clone(), constraintLst.clone(), clsattrs.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkExternalNode((arcstr::literal!(FNode::edNodeName)).clone(), externalDecl.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: m, composition: cdef }, _, _, g) => {
                    let mut g = (*g).clone();
                    g = mkClassChildren(cdef.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkModNode((arcstr::literal!(FNode::modNodeName)).clone(), m.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), inParentRef.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::DERIVED { modifications: m, typeSpec: ts, .. }, _, _, g) => {
                    let mut nr: Ref;
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut g = (*g).clone();
                    let _ = AbsynUtil::typeSpecPath(ts.clone())?;
                    nr = inParentRef.clone();
                    g = mkModNode((arcstr::literal!(FNode::modNodeName)).clone(), m.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    ad = AbsynUtil::typeSpecDimensions(ts.clone());
                    g = mkDimsNode((arcstr::literal!(FNode::tydimsNodeName)).clone(), Some(ad.clone()), nr.clone(), inKind.clone(), g.clone())?;
                    g = mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), nr.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::OVERLOAD { pathLst: _ }, _, _, g) => {
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PDER { functionPath: _, derivedVariables: _ }, _, _, g) => {
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

pub fn mkElementNode(mut inElement: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inElement.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (Deref @ SCode::Element::COMPONENT { .. }, _, _, g) => {
            let mut g = (*g).clone();
            g = mkCompNode(inElement.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        (Deref @ SCode::Element::CLASS { .. }, _, _, g) => {
            let mut g = (*g).clone();
            g = mkClassNode(inElement.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), inParentRef.clone(), inKind.clone(), g.clone(), false)?;
            g.clone()
        },
        (Deref @ SCode::Element::EXTENDS { modifications: m, baseClassPath: p, .. }, _, _, g) => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            name = (FNode::mkExtendsName(p.clone())?).clone();
            (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::EX { e: inElement.clone(), r#mod: Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD) });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
            g = mkModNode((arcstr::literal!(FNode::modNodeName)).clone(), m.clone(), nr.clone(), inKind.clone(), g.clone())?;
            g = mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), nr.clone(), g.clone())?;
            g.clone()
        },
        (Deref @ SCode::Element::IMPORT { .. }, _, _, g) => {
            let mut g = (*g).clone();
            g = mkImportNode(inElement.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        (Deref @ SCode::Element::DEFINEUNIT { .. }, _, _, g) => {
            let mut g = (*g).clone();
            g = mkUnitsNode(inElement.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub fn mkUnitsNode(mut inElement: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inElement.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut r: Ref;
                    r = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::duNodeName)).clone())?;
                    FNode::addDefinedUnitToRef(r.clone(), inElement.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut n: Node;
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::duNodeName)).clone(), list![inParentRef.clone()], FCore::Data::DU { els: list![inElement.clone()] });
                    r = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::duNodeName)).clone(), r.clone(), false)?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn mkImportNode(mut inElement: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inElement.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut r: Ref;
                    r = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::imNodeName)).clone())?;
                    FNode::addImportToRef(r.clone(), inElement.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut n: Node;
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::imNodeName)).clone(), list![inParentRef.clone()], FCore::Data::IM { i: FCore::emptyImportTable.clone() });
                    r = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::imNodeName)).clone(), r.clone(), false)?;
                    FNode::addImportToRef(r.clone(), inElement.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn mkDimsNode(mut inName: Name, mut inArrayDims: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inName.clone(), inArrayDims.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, None, _, _, g) => {
            g.clone()
        },
        (_, Some(Deref @ metamodelica::List::Nil), _, _, g) => {
            g.clone()
        },
        (_, Some(a @ Deref @ metamodelica::List::Cons { head: _, tail: _ }), _, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::DIMS { name: (inName.clone()).clone(), dims: a.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (inName.clone()).clone(), nr.clone(), false)?;
            g = mkDimsNode_helper(0, a.clone(), nr.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn mkDimsNode_helper(mut inStartWith: i32, mut inArrayDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inStartWith.clone(), inArrayDims.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (i, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB, tail: rest }, _, _, g) => {
            let mut name: Name = arcstr::literal!("");
            let mut g = (*g).clone();
            name = (intString(i.clone())).clone();
            g = mkExpressionNode((name.clone()).clone(), Arc::new(openmodelica_ast::Absyn::Exp::END), inParentRef.clone(), inKind.clone(), g.clone())?;
            g = mkDimsNode_helper(i.clone() + 1, rest.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        (i, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e }, tail: rest }, _, _, g) => {
            let mut name: Name = arcstr::literal!("");
            let mut g = (*g).clone();
            name = (intString(i.clone())).clone();
            g = mkExpressionNode((name.clone()).clone(), e.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g = mkDimsNode_helper(i.clone() + 1, rest.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub fn mkCompNode(mut inComp: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut name: ArcStr = arcstr::literal!("");
    let mut g: Graph;
    let mut n: Node;
    let mut nr: Ref;
    let mut m: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut cnd: Option<Arc<Absyn::Exp>> = None;
    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut ts: Arc<Absyn::TypeSpec>;
    let mut tad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut nd: Data = FCore::Data::TOP;
    let mut i: Arc<DAE::Var>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ SCode::Element::COMPONENT { condition: __pa0, modifications: __pa1, typeSpec: __pa2, attributes: SCode::Attributes { arrayDims: __pa3, .. }, name: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cnd = __pa0.clone();
    m = __pa1.clone();
    ts = __pa2.clone();
    ad = __pa3.clone();
    name = __pa4.clone();
    (nd, i) = FNode::element2Data(inComp.clone(), inKind.clone())?;
    (g, n) = FGraph::node(inGraph.clone(), (name.clone()).clone(), list![inParentRef.clone()], nd.clone());
    nr = FNode::toRef(n.clone());
    FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
    g = mkInstNode(i.clone(), nr.clone(), g.clone())?;
    g = mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), nr.clone(), g.clone())?;
    outGraph = g.clone();
    Ok(outGraph)
}

pub fn mkInstNode(mut inVar: Arc<DAE::Var>, mut inParentRef: Ref, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut nr: Ref;
    let mut n: Node;
    let mut g: Graph;
    (g, n) = FGraph::node(inGraph.clone(), (arcstr::literal!(FNode::itNodeName)).clone(), list![inParentRef.clone()], FCore::Data::IT { i: inVar.clone() });
    nr = FNode::toRef(n.clone());
    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::itNodeName)).clone(), nr.clone(), false)?;
    outGraph = g.clone();
    Ok(outGraph)
}

pub fn mkConditionNode(mut inCondition: Option<Arc<Absyn::Exp>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inCondition.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (None, _, _, g) => {
            g.clone()
        },
        (Some(e), _, _, g) => {
            let mut g = (*g).clone();
            g = mkExpressionNode((arcstr::literal!(FNode::cndNodeName)).clone(), e.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub fn mkExpressionNode(mut inName: Name, mut inExp: Arc<Absyn::Exp>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inName.clone(), inExp.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, e, _, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::EXP { name: (inName.clone()).clone(), e: e.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (inName.clone()).clone(), nr.clone(), false)?;
            g = analyseExp(e.clone(), nr.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn mkCrefsNodes(mut inCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inCrefs.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, _, _, g) => {
            let mut g = (*g).clone();
            g = mkCrefNode(cr.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g = mkCrefsNodes(rest.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub fn mkCrefNode(mut inCref: Arc<Absyn::ComponentRef>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inCref.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, _, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut name: Name = arcstr::literal!("");
            let mut g = (*g).clone();
            name = (Dump::printComponentRefStr(inCref.clone())?).clone();
            (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::CR { r: inCref.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
            g = mkDimsNode((arcstr::literal!(FNode::subsNodeName)).clone(), List::mkOption(AbsynUtil::getSubsFromCref(inCref.clone(), true, true)?), nr.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn mkTypeNode(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inParentRef: Ref, mut inName: Name, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inTypes.clone(), inParentRef.clone(), inName.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut nr: Ref;
                    let mut pr: Ref;
                    pr = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone())?;
                    nr = FNode::child(pr.clone(), (inName.clone()).clone())?;
                    FNode::addTypesToRef(nr.clone(), inTypes.clone())?;
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut nr: Ref;
                    let mut pr: Ref;
                    let mut n: Node;
                    let mut g = (*g).clone();
                    if '__try0: {
                        let _ = unwrap_break_err!(FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::tyNodeName)).clone(), list![inParentRef.clone()], FCore::Data::ND { scopeType: None });
                    pr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone(), pr.clone(), false)?;
                    (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![pr.clone()], FCore::Data::FT { tys: inTypes.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(pr.clone(), (inName.clone()).clone(), nr.clone(), false)?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut nr: Ref;
                    let mut pr: Ref;
                    let mut n: Node;
                    let mut g = (*g).clone();
                    pr = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone())?;
                    if '__try0: {
                        let _ = unwrap_break_err!(FNode::child(pr.clone(), (inName.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![pr.clone()], FCore::Data::FT { tys: inTypes.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(pr.clone(), (inName.clone()).clone(), nr.clone(), false)?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pr: Ref;
                    pr = FGraph::top(inGraph.clone())?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraphBuildEnv.mkTypeNode: Error making type node: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!(" in parent: ")); __mm_s.push_str(&*FNode::name(FNode::fromRef(pr.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn mkEqNode(mut inName: Name, mut inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inName.clone(), inEqs.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (_, _, _, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::EQ { name: (inName.clone()).clone(), e: inEqs.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (inName.clone()).clone(), nr.clone(), false)?;
            g = List::fold2(inEqs.clone(), (std::sync::Arc::new(fnptr!(analyseEquation, Arc<SCode::Equation>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), nr.clone(), inKind.clone(), g.clone());
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn mkAlNode(mut inName: Name, mut inAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inName.clone(), inAlgs.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (_, _, _, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::AL { name: (inName.clone()).clone(), a: inAlgs.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (inName.clone()).clone(), nr.clone(), false)?;
            g = List::fold2(inAlgs.clone(), (std::sync::Arc::new(analyseAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), nr.clone(), inKind.clone(), g.clone());
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn mkOptNode(mut inName: Name, mut inConstraintLst: Arc<metamodelica::List<SCode::ConstraintSection>>, mut inClsAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inName.clone(), inConstraintLst.clone(), inClsAttrs.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (_, _, _, _, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::OT { constrainLst: inConstraintLst.clone(), clsAttrs: inClsAttrs.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (inName.clone()).clone(), nr.clone(), false)?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub fn mkExternalNode(mut inName: Name, mut inExternalDeclOpt: Option<Arc<SCode::ExternalDecl>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inName.clone(), inExternalDeclOpt.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (_, None, _, _, g) => {
            g.clone()
        },
        (_, Some(ed @ Deref @ SCode::ExternalDecl { args: exps, output_: ocr, .. }), _, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut oae: Option<Arc<Absyn::Exp>> = None;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::ED { ed: ed.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (inName.clone()).clone(), nr.clone(), false)?;
            oae = Util::applyOption(ocr.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefExp, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Exp>> + 'static>));
            g = mkCrefsFromExps(List::consOption(oae.clone(), exps.clone()), nr.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn mkCrefsFromExps(mut inExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inExps.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, g) => {
            let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            let mut g = (*g).clone();
            crefs = AbsynUtil::getCrefFromExp(e.clone(), true, true)?;
            g = mkCrefsNodes(crefs.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g = mkCrefsFromExps(rest.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

fn analyseExp(mut inExp: Arc<Absyn::Exp>, mut inRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    (_, outGraph) = AbsynUtil::traverseExpBidir(inExp.clone(), Arc::new({ let __pe_b1 = inRef.clone(); let __pe_b2 = inKind.clone(); move |__pe_a0, __pe_a3| analyseExpTraverserEnter(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), (std::sync::Arc::new(fnptr!(analyseExpTraverserExit, Arc<Absyn::Exp>, FCore::Graph)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), inGraph.clone())?;
    Ok(outGraph)
}

fn analyseOptExp(mut inExp: Option<Arc<Absyn::Exp>>, mut inRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inExp.clone(), inRef.clone(), inKind.clone(), inGraph.clone())) {
        (None, _, _, g) => {
            g.clone()
        },
        (Some(exp), _, _, g) => {
            let mut g = (*g).clone();
            g = analyseExp(exp.clone(), inRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

fn analyseExpTraverserEnter(mut inExp: Arc<Absyn::Exp>, mut r#ref: Ref, mut kind: Kind, mut graph: Graph) -> Result<(Arc<Absyn::Exp>, Graph)> {
    let mut inExp: Arc<Absyn::Exp> = inExp;
    let mut graph: Graph = graph;
    graph = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: cref } => {
            analyseCref(cref.clone(), r#ref.clone(), kind.clone(), graph.clone())?
        },
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterators: iters, .. }, .. } => {
            addIterators(iters.clone(), r#ref.clone(), kind.clone(), graph.clone())?
        },
        Deref @ Absyn::Exp::CALL { function_: cref, .. } => {
            analyseCref(cref.clone(), r#ref.clone(), kind.clone(), graph.clone())?
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, .. } => {
            analyseCref(cref.clone(), r#ref.clone(), kind.clone(), graph.clone())?
        },
        Deref @ Absyn::Exp::MATCHEXP { .. } => {
            addMatchScope(inExp.clone(), r#ref.clone(), kind.clone(), graph.clone())?
        },
        _ => {
            graph.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((inExp, graph))
}

fn analyseCref(mut inCref: Arc<Absyn::ComponentRef>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inCref.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::WILD, _, _, g) => {
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut g = (*g).clone();
                    g = mkCrefNode(inCref.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

fn analyseExpTraverserExit(mut exp: Arc<Absyn::Exp>, mut graph: Graph) -> (Arc<Absyn::Exp>, Graph) {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut graph: Graph = graph;
    (exp, graph)
}

fn analyseEquation(mut inEquation: Arc<SCode::Equation>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    (_, outGraph) = SCodeUtil::mapFoldEquations(inEquation.clone(), Arc::new({ let __pe_b1 = inParentRef.clone(); let __pe_b2 = inKind.clone(); move |__pe_a0, __pe_a3| analyseEquationTraverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), inGraph.clone());
    outGraph
}

fn analyseEquationTraverser(mut eq: Arc<SCode::Equation>, mut r#ref: Ref, mut kind: Kind, mut graph: Graph) -> Result<(Arc<SCode::Equation>, Graph)> {
    let mut eq: Arc<SCode::Equation> = eq;
    let mut graph: Graph = graph;
    (eq, graph) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_FOR { index: iter_name, .. } => {
            graph = addIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], r#ref.clone(), kind.clone(), graph.clone())?;
            SCodeUtil::mapFoldEquationExps(eq.clone(), Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind.clone(); move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), graph.clone())?
        },
        Deref @ SCode::Equation::EQ_REINIT { cref: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, .. } => {
            graph = analyseCref(cref1.clone(), r#ref.clone(), kind.clone(), graph.clone())?;
            SCodeUtil::mapFoldEquationExps(eq.clone(), Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind.clone(); move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), graph.clone())?
        },
        _ => {
            let _ = SCodeUtil::getEquationInfo(eq.clone())?;
            SCodeUtil::mapFoldEquationExps(eq.clone(), Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind.clone(); move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), graph.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, graph))
}

fn traverseExp(mut exp: Arc<Absyn::Exp>, mut graph: Graph, mut r#ref: Ref, mut kind: Kind) -> Result<(Arc<Absyn::Exp>, Graph)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut graph: Graph = graph;
    (exp, graph) = AbsynUtil::traverseExpBidir(exp.clone(), Arc::new({ let __pe_b1 = r#ref.clone(); let __pe_b2 = kind.clone(); move |__pe_a0, __pe_a3| analyseExpTraverserEnter(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), (std::sync::Arc::new(fnptr!(analyseExpTraverserExit, Arc<Absyn::Exp>, FCore::Graph)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), graph.clone())?;
    Ok((exp, graph))
}

fn analyseAlgorithm(mut inAlgorithm: Arc<SCode::AlgorithmSection>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inAlgorithm.clone()) {
        Deref @ SCode::AlgorithmSection { statements: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stmts = __pa0.clone();
    outGraph = List::fold2(stmts.clone(), (std::sync::Arc::new(fnptr!(analyseStatement, Arc<SCode::Statement>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), inParentRef.clone(), inKind.clone(), inGraph.clone());
    Ok(outGraph)
}

fn analyseStatement(mut inStatement: Arc<SCode::Statement>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    (_, outGraph) = SCodeUtil::mapFoldStatements(inStatement.clone(), Arc::new({ let __pe_b1 = inParentRef.clone(); let __pe_b2 = inKind.clone(); move |__pe_a0, __pe_a3| analyseStatementTraverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), inGraph.clone());
    outGraph
}

fn analyseStatementTraverser(mut stmt: Arc<SCode::Statement>, mut r#ref: Ref, mut kind: Kind, mut graph: Graph) -> Result<(Arc<SCode::Statement>, Graph)> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    let mut graph: Graph = graph;
    (stmt, graph) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            graph = addIterators(list![Arc::new(Absyn::ForIterator { name: (var_field!((*stmt).index, SCode::Statement::ALG_FOR).clone()).clone(), guardExp: None, range: None })], r#ref.clone(), kind.clone(), graph.clone())?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(stmt.clone(), Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind.clone(); move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), graph.clone());
            (stmt.clone(), graph.clone())
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            graph = addIterators(list![Arc::new(Absyn::ForIterator { name: (var_field!((*stmt).index, SCode::Statement::ALG_PARFOR).clone()).clone(), guardExp: None, range: None })], r#ref.clone(), kind.clone(), graph.clone())?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(stmt.clone(), Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind.clone(); move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), graph.clone());
            (stmt.clone(), graph.clone())
        },
        _ => {
            let _ = SCodeUtil::getStatementInfo(stmt.clone())?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(stmt.clone(), Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind.clone(); move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), graph.clone());
            (stmt.clone(), graph.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((stmt, graph))
}

pub fn addIterators(mut inIterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inIterators.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    nr = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::forNodeName)).clone())?;
                    FNode::addIteratorsToRef(nr.clone(), inIterators.clone())?;
                    g = addIterators_helper(inIterators.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::forNodeName)).clone(), list![inParentRef.clone()], FCore::Data::FS { fis: inIterators.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::forNodeName)).clone(), nr.clone(), false)?;
                    g = addIterators_helper(inIterators.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
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
pub fn addIterators_helper(mut inIterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inIterators.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (Deref @ metamodelica::List::Cons { head: i @ Deref @ Absyn::ForIterator { name, .. }, tail: rest }, _, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::FI { fi: i.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
            g = addIterators_helper(rest.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub fn addMatchScope(mut inMatchExp: Arc<Absyn::Exp>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut n: Node;
    let mut nr: Ref;
    let mut local_decls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut g: Graph;
    (g, n) = FGraph::node(inGraph.clone(), (arcstr::literal!(FNode::matchNodeName)).clone(), list![inParentRef.clone()], FCore::Data::MS { e: inMatchExp.clone() });
    nr = FNode::toRef(n.clone());
    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::matchNodeName)).clone(), nr.clone(), false)?;
    let __pa0 = ::match_deref::match_deref! { match &(inMatchExp.clone()) {
        Deref @ Absyn::Exp::MATCHEXP { localDecls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    local_decls = __pa0.clone();
    outGraph = addMatchScope_helper(local_decls.clone(), nr.clone(), inKind.clone(), g.clone())?;
    Ok(outGraph)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn addMatchScope_helper(mut inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inElements.clone(), inParentRef.clone(), inKind.clone(), inGraph.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, g) => {
            g.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element }, tail: rest }, _, _, g) => {
            let mut el: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut g = (*g).clone();
            el = AbsynToSCode::translateElement(element.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
            g = List::fold2(el.clone(), (std::sync::Arc::new(mkElementNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), inParentRef.clone(), inKind.clone(), g.clone());
            g = addMatchScope_helper(rest.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, g) => {
            let mut g = (*g).clone();
            g = addMatchScope_helper(rest.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub fn mkRefNode(mut inName: Name, mut inTargetScope: Scope, mut inParentRef: Ref, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inName.clone(), inTargetScope.clone(), inParentRef.clone(), inGraph.clone())) {
        (_, _, _, g) => {
            let mut n: Node;
            let mut rn: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::REF { target: inTargetScope.clone() });
            rn = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (inName.clone()).clone(), rn.clone(), false)?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

