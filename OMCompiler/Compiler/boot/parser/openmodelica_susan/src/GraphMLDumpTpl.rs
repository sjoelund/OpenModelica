// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::GraphML;
use crate::Tpl;
use openmodelica_util::Util;

pub fn dumpGraphInfo(mut txt: Tpl::Text, mut a_graphInfo: GraphML::GraphInfo, mut a_fileName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut txt_0: Tpl::Text;
    txt_0 = dumpGraphInfoInternal(Tpl::emptyTxt.clone(), a_graphInfo.clone())?;
    Tpl::textFile(txt_0.clone(), (a_fileName.clone()).clone())?;
    out_txt = txt.clone();
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_5(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<GraphML::Attribute>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_att, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAttDef(txt.clone(), i_att.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_5(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_6(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<GraphML::Edge>>, mut in_a_attributes: metamodelica::Array<GraphML::Attribute>, mut in_a_graphInfo_graphEdgeKey: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_attributes.clone(), in_a_graphInfo_graphEdgeKey.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_edge, tail: rest }, a_attributes, a_graphInfo_graphEdgeKey) => {
            let mut txt = (*txt).clone();
            txt = dumpEdge(txt.clone(), i_edge.clone(), (a_graphInfo_graphEdgeKey.clone()).clone(), a_attributes.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_6(txt.clone(), rest.clone(), a_attributes.clone(), (a_graphInfo_graphEdgeKey.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpGraphInfoInternal(mut in_txt: Tpl::Text, mut in_a_graphInfo: GraphML::GraphInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_graphInfo.clone()) {
        (mut txt, GraphML::GraphInfo::GRAPHINFOARR { nodes: mut i_nodes, graphs: mut i_graphs, graphNodeKey: mut i_graphNodeKey, graphEdgeKey: ref i_graphEdgeKey @ ref i_graphInfo_graphEdgeKey, edges: ref i_edges, attributes: mut i_attributes }) => {
            let mut ret_5: GraphML::Graph;
            let mut ret_4: i32 = 0;
            let mut ret_3: i32 = 0;
            let mut l_edgeDump: Tpl::Text;
            let mut ret_1: Arc<metamodelica::List<GraphML::Attribute>> = metamodelica::nil();
            let mut l_attDefDump: Tpl::Text;
            ret_1 = Arc::new(i_attributes.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_attDefDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attDefDump = lm_5(l_attDefDump.clone(), ret_1.clone())?;
            l_attDefDump = Tpl::popIter(l_attDefDump.clone())?;
            l_edgeDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_edgeDump = lm_6(l_edgeDump.clone(), i_edges.clone(), i_attributes.clone(), (i_graphInfo_graphEdgeKey.clone()).clone())?;
            l_edgeDump = Tpl::popIter(l_edgeDump.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n")).clone(), (literal!("<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:y=\"http://www.yworks.com/xml/graphml\" xmlns:yed=\"http://www.yworks.com/xml/yed/3\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns http://www.yworks.com/xml/schema/graphml/1.1/ygraphml.xsd\">\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<key for=\"node\" id=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_graphNodeKey.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\" yfiles.type=\"nodegraphics\"/>\n")).clone(), (literal!("<key attr.name=\"description\" attr.type=\"string\" for=\"node\" id=\"ddesc\" />\n")).clone(), (literal!("<key for=\"edge\" id=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_graphEdgeKey.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\" yfiles.type=\"edgegraphics\"/>\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_attDefDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("<!-- Graph Idx: ")).clone()], lastHasNewLine: false }))?;
            ret_3 = (i_graphs.clone().borrow().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_3.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" -->\n")).clone() }))?;
            ret_4 = (i_graphs.clone().borrow().len() as i32);
            ret_5 = i_graphs.clone().borrow()[(ret_4.clone()-1) as usize].clone();
            txt = dumpGraph(txt.clone(), ret_5.clone(), i_graphs.clone(), i_nodes.clone(), (Tpl::textString(l_edgeDump.clone())?).clone(), (i_graphNodeKey.clone()).clone(), i_attributes.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</graphml>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_8(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_graphAttributes: metamodelica::Array<GraphML::Attribute>, mut in_a_graphNodeKey: ArcStr, mut in_a_allGraphs: metamodelica::Array<GraphML::Graph>, mut in_a_allNodes: metamodelica::Array<GraphML::Node>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_graphAttributes.clone(), in_a_graphNodeKey.clone(), in_a_allGraphs.clone(), in_a_allNodes.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_idc, tail: rest }, a_graphAttributes, a_graphNodeKey, a_allGraphs, a_allNodes) => {
            let mut ret_3: GraphML::Node;
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = (a_allNodes.clone().borrow().len() as i32);
            ret_1 = intSub(ret_0.clone(), i_idc.clone());
            ret_2 = intAdd(1, ret_1.clone());
            ret_3 = a_allNodes.clone().borrow()[(ret_2.clone()-1) as usize].clone();
            txt = dumpNode(txt.clone(), ret_3.clone(), a_allGraphs.clone(), a_allNodes.clone(), (a_graphNodeKey.clone()).clone(), a_graphAttributes.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_8(txt.clone(), rest.clone(), a_graphAttributes.clone(), (a_graphNodeKey.clone()).clone(), a_allGraphs.clone(), a_allNodes.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_9(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, ArcStr)>>, mut in_a_graphAttributes: metamodelica::Array<GraphML::Attribute>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_graphAttributes.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_val, tail: rest }, a_graphAttributes) => {
            let mut txt = (*txt).clone();
            txt = dumpAttKey(txt.clone(), i_val.clone(), a_graphAttributes.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_9(txt.clone(), rest.clone(), a_graphAttributes.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpGraph(mut in_txt: Tpl::Text, mut in_a_graph: GraphML::Graph, mut in_a_allGraphs: metamodelica::Array<GraphML::Graph>, mut in_a_allNodes: metamodelica::Array<GraphML::Node>, mut in_a_edgeDesc: ArcStr, mut in_a_graphNodeKey: ArcStr, mut in_a_graphAttributes: metamodelica::Array<GraphML::Attribute>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_graph.clone(), in_a_allGraphs.clone(), in_a_allNodes.clone(), in_a_edgeDesc.clone(), in_a_graphNodeKey.clone(), in_a_graphAttributes.clone()) {
        (mut txt, GraphML::Graph { id: mut i_id, directed: mut i_directed, attValues: ref i_attValues, nodeIdc: ref i_nodeIdc }, mut a_allGraphs, mut a_allNodes, mut a_edgeDesc, mut a_graphNodeKey, mut a_graphAttributes) => {
            let mut l_attKeys: Tpl::Text;
            let mut l_graphNodes: Tpl::Text;
            l_graphNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_graphNodes = lm_8(l_graphNodes.clone(), i_nodeIdc.clone(), a_graphAttributes.clone(), (a_graphNodeKey.clone()).clone(), a_allGraphs.clone(), a_allNodes.clone())?;
            l_graphNodes = Tpl::popIter(l_graphNodes.clone())?;
            l_attKeys = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attKeys = lm_9(l_attKeys.clone(), i_attValues.clone(), a_graphAttributes.clone())?;
            l_attKeys = Tpl::popIter(l_attKeys.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<graph edgedefault=\"")).clone() }))?;
            txt = dumpDirected(txt.clone(), i_directed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" id=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_attKeys.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_graphNodes.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(crate::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeStr(txt.clone(), (a_edgeDesc.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</graph>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_11(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<GraphML::NodeLabel>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_label, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpNodeLabel(txt.clone(), i_label.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_11(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_12(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, ArcStr)>>, mut in_a_graphAttributes: metamodelica::Array<GraphML::Attribute>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_graphAttributes.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_val, tail: rest }, a_graphAttributes) => {
            let mut txt = (*txt).clone();
            txt = dumpAttKey(txt.clone(), i_val.clone(), a_graphAttributes.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_12(txt.clone(), rest.clone(), a_graphAttributes.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_13(mut in_txt: Tpl::Text, mut in_mArg: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, Some(mut i_val)) => {
            txt = Tpl::writeStr(txt.clone(), (i_val.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_14(mut in_txt: Tpl::Text, mut in_a_isFolded: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isFolded.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("group")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("folder")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_15(mut in_txt: Tpl::Text, mut in_a_isFolded: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isFolded.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpNode(mut in_txt: Tpl::Text, mut in_a_node: GraphML::Node, mut in_a_allGraphs: metamodelica::Array<GraphML::Graph>, mut in_a_allNodes: metamodelica::Array<GraphML::Node>, mut in_a_graphNodeKey: ArcStr, mut in_a_graphAttributes: metamodelica::Array<GraphML::Attribute>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_node.clone(), in_a_allGraphs.clone(), in_a_allNodes.clone(), in_a_graphNodeKey.clone(), in_a_graphAttributes.clone()) {
        (mut txt, GraphML::Node::NODE { shapeType: mut i_shapeType, border: mut i_border, color: mut i_color, optDesc: mut i_optDesc, id: mut i_id, attValues: ref i_attValues, nodeLabels: ref i_nodeLabels }, _, _, mut a_graphNodeKey, mut a_graphAttributes) => {
            let mut l_attKeys: Tpl::Text;
            let mut l_nodeLabelDump: Tpl::Text;
            l_nodeLabelDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_nodeLabelDump = lm_11(l_nodeLabelDump.clone(), i_nodeLabels.clone())?;
            l_nodeLabelDump = Tpl::popIter(l_nodeLabelDump.clone())?;
            l_attKeys = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attKeys = lm_12(l_attKeys.clone(), i_attValues.clone(), a_graphAttributes.clone())?;
            l_attKeys = Tpl::popIter(l_attKeys.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<node id=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_attKeys.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<data key=\"ddesc\"><![CDATA[")).clone() }))?;
            txt = fun_13(txt.clone(), i_optDesc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("]]></data>\n")).clone(), (literal!("<data key=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_graphNodeKey.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("    <y:ShapeNode>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:Fill color=\"#")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_color.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\" transparent=\"false\"/>\n")).clone(), (literal!("<y:BorderStyle color=\"#000000\" type=\"line\" width=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_border.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_nodeLabelDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:Shape type=\"")).clone() }))?;
            txt = dumpShapeType(txt.clone(), i_shapeType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </y:ShapeNode>\n")).clone(), (literal!("</data>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</node>")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::Node::GROUPNODE { internalGraphIdx: mut i_internalGraphIdx, header: mut i_header, id: mut i_id, isFolded: mut i_isFolded }, mut a_allGraphs, mut a_allNodes, mut a_graphNodeKey, mut a_graphAttributes) => {
            let mut ret_10: GraphML::Graph;
            let mut ret_9: i32 = 0;
            let mut ret_8: i32 = 0;
            let mut ret_7: i32 = 0;
            let mut ret_6: i32 = 0;
            let mut ret_5: i32 = 0;
            let mut ret_4: i32 = 0;
            let mut l_activeType: Tpl::Text;
            let mut l_folderType: Tpl::Text;
            l_folderType = fun_14(Tpl::emptyTxt.clone(), i_isFolded.clone())?;
            l_activeType = fun_15(Tpl::emptyTxt.clone(), i_isFolded.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<node id=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" yfiles.foldertype=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_folderType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<data key=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_graphNodeKey.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <y:ProxyAutoBoundsNode>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:Realizers active=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_activeType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <y:GroupNode>\n")).clone(), (literal!("    <y:Fill color=\"#F5F5F5\" transparent=\"false\"/>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:NodeLabel alignment=\"right\" autoSizePolicy=\"node_width\" backgroundColor=\"#EBEBEB\" borderDistance=\"0.0\" fontFamily=\"Dialog\" fontSize=\"15\" fontStyle=\"plain\" hasLineColor=\"false\" modelName=\"internal\" modelPosition=\"t\" textColor=\"#000000\" visible=\"true\">")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_header.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</y:NodeLabel>\n")).clone(), (literal!("<y:Shape type=\"roundrectangle\"/>\n")).clone(), (literal!("<y:State closed=\"false\" closedHeight=\"50.0\" closedWidth=\"50.0\" innerGraphDisplayEnabled=\"false\"/>\n")).clone(), (literal!("<y:Insets bottom=\"15\" bottomF=\"15.0\" left=\"15\" leftF=\"15.0\" right=\"15\" rightF=\"15.0\" top=\"15\" topF=\"15.0\"/>\n")).clone(), (literal!("<y:BorderInsets bottom=\"0\" bottomF=\"0.0\" left=\"0\" leftF=\"0.0\" right=\"0\" rightF=\"0.0\" top=\"0\" topF=\"0.0\"/>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </y:GroupNode>\n")).clone(), (literal!("  <y:GroupNode>\n")).clone(), (literal!("    <y:Geometry height=\"50.0\" width=\"50.0\" x=\"0.0\" y=\"60.0\"/>\n")).clone(), (literal!("    <y:Fill color=\"#F5F5F5\" transparent=\"false\"/>\n")).clone(), (literal!("    <y:BorderStyle color=\"#000000\" type=\"dashed\" width=\"1.0\"/>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:NodeLabel alignment=\"right\" autoSizePolicy=\"node_width\" backgroundColor=\"#EBEBEB\" borderDistance=\"0.0\" fontFamily=\"Dialog\" fontSize=\"15\" fontStyle=\"plain\" hasLineColor=\"false\" height=\"22.37646484375\" modelName=\"internal\" modelPosition=\"t\" textColor=\"#000000\" visible=\"true\">")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_header.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</y:NodeLabel>\n")).clone(), (literal!("<y:Shape type=\"roundrectangle\"/>\n")).clone(), (literal!("<y:State closed=\"true\" closedHeight=\"50.0\" closedWidth=\"50.0\" innerGraphDisplayEnabled=\"false\"/>\n")).clone(), (literal!("<y:Insets bottom=\"5\" bottomF=\"5.0\" left=\"5\" leftF=\"5.0\" right=\"5\" rightF=\"5.0\" top=\"5\" topF=\"5.0\"/>\n")).clone(), (literal!("<y:BorderInsets bottom=\"0\" bottomF=\"0.0\" left=\"0\" leftF=\"0.0\" right=\"0\" rightF=\"0.0\" top=\"0\" topF=\"0.0\"/>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </y:GroupNode>\n")).clone(), (literal!("</y:Realizers>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </y:ProxyAutoBoundsNode>\n")).clone(), (literal!("</data>\n")).clone(), (literal!("<!-- Graph Idx: ")).clone()], lastHasNewLine: false }))?;
            ret_4 = (a_allGraphs.clone().borrow().len() as i32);
            ret_5 = intSub(ret_4.clone(), i_internalGraphIdx.clone());
            ret_6 = intAdd(1, ret_5.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_6.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" -->\n")).clone() }))?;
            ret_7 = (a_allGraphs.clone().borrow().len() as i32);
            ret_8 = intSub(ret_7.clone(), i_internalGraphIdx.clone());
            ret_9 = intAdd(1, ret_8.clone());
            ret_10 = a_allGraphs.clone().borrow()[(ret_9.clone()-1) as usize].clone();
            txt = dumpGraph(txt.clone(), ret_10.clone(), a_allGraphs.clone(), a_allNodes.clone(), (literal!("")).clone(), (a_graphNodeKey.clone()).clone(), a_graphAttributes.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</node>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_17(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<GraphML::EdgeLabel>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_label, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEdgeLabel(txt.clone(), i_label.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_17(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_18(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, ArcStr)>>, mut in_a_graphAttributes: metamodelica::Array<GraphML::Attribute>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_graphAttributes.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_val, tail: rest }, a_graphAttributes) => {
            let mut txt = (*txt).clone();
            txt = dumpAttKey(txt.clone(), i_val.clone(), a_graphAttributes.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_18(txt.clone(), rest.clone(), a_graphAttributes.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpEdge(mut in_txt: Tpl::Text, mut in_a_edge: GraphML::Edge, mut in_a_graphEdgeKey: ArcStr, mut in_a_graphAttributes: metamodelica::Array<GraphML::Attribute>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_edge.clone(), in_a_graphEdgeKey.clone(), in_a_graphAttributes.clone()) {
        (mut txt, GraphML::Edge { smooth: mut i_smooth, arrows: mut i_arrows, lineWidth: mut i_lineWidth, lineType: mut i_lineType, color: mut i_color, target: mut i_target, source: mut i_source, id: mut i_id, attValues: ref i_attValues, edgeLabels: ref i_edgeLabels }, mut a_graphEdgeKey, mut a_graphAttributes) => {
            let mut ret_3: GraphML::ArrowType = GraphML::ArrowType::ARROWCONCAVE;
            let mut ret_2: GraphML::ArrowType = GraphML::ArrowType::ARROWCONCAVE;
            let mut l_attKeys: Tpl::Text;
            let mut l_edgeLabelDump: Tpl::Text;
            l_edgeLabelDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_edgeLabelDump = lm_17(l_edgeLabelDump.clone(), i_edgeLabels.clone())?;
            l_edgeLabelDump = Tpl::popIter(l_edgeLabelDump.clone())?;
            l_attKeys = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attKeys = lm_18(l_attKeys.clone(), i_attValues.clone(), a_graphAttributes.clone())?;
            l_attKeys = Tpl::popIter(l_attKeys.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<edge id=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" source=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_source.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" target=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_target.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_attKeys.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<data key=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_graphEdgeKey.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <y:PolyLineEdge>\n")).clone(), (literal!("    <y:Path sx=\"0.0\" sy=\"0.0\" tx=\"0.0\" ty=\"0.0\"/>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:LineStyle color=\"#")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_color.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" type=\"")).clone() }))?;
            txt = dumpLineType(txt.clone(), i_lineType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" width=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_lineWidth.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"/>\n")).clone(), (literal!("<y:Arrows source=\"")).clone()], lastHasNewLine: false }))?;
            ret_2 = Util::tuple21(i_arrows.clone());
            txt = dumpArrowType(txt.clone(), ret_2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" target=\"")).clone() }))?;
            ret_3 = Util::tuple22(i_arrows.clone());
            txt = dumpArrowType(txt.clone(), ret_3.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"/>\n")).clone(), (literal!("<y:BendStyle smoothed=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_smooth.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_edgeLabelDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </y:PolyLineEdge>\n")).clone(), (literal!("</data>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</edge>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpEdgeLabel(mut in_txt: Tpl::Text, mut in_a_edgeLabel: GraphML::EdgeLabel) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_edgeLabel.clone()) {
        (mut txt, GraphML::EdgeLabel { text: mut i_text, fontSize: mut i_fontSize, backgroundColor: mut i_backgroundColor }) => {
            let mut l_bgColor: Tpl::Text;
            l_bgColor = dumpColorOpt(Tpl::emptyTxt.clone(), i_backgroundColor.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:EdgeLabel alignment=\"center\" distance=\"2.0\" fontFamily=\"Dialog\" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_bgColor.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" fontSize=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_fontSize.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" fontStyle=\"plain\" hasBackgroundColor=\"false\" hasLineColor=\"false\" modelName=\"side_slider\" preferredPlacement=\"anywhere\" visible=\"true\">")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_text.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</y:EdgeLabel>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpNodeLabel(mut in_txt: Tpl::Text, mut in_a_nodeLabel: GraphML::NodeLabel) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_nodeLabel.clone()) {
        (mut txt, GraphML::NodeLabel::NODELABEL_INTERNAL { text: mut i_text, fontStyle: mut i_fontStyle, backgroundColor: mut i_backgroundColor }) => {
            let mut l_bgColor: Tpl::Text;
            l_bgColor = dumpColorOpt(Tpl::emptyTxt.clone(), i_backgroundColor.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:NodeLabel alignment=\"center\" autoSizePolicy=\"content\" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_bgColor.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" fontFamily=\"Dialog\" fontSize=\"12\" fontStyle=\"")).clone() }))?;
            txt = dumpFontStyle(txt.clone(), i_fontStyle.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" hasLineColor=\"false\" modelName=\"internal\" modelPosition=\"c\" textColor=\"#000000\" visible=\"true\">")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_text.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</y:NodeLabel>")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::NodeLabel::NODELABEL_CORNER { text: mut i_text, position: mut i_position, fontStyle: mut i_fontStyle, backgroundColor: mut i_backgroundColor }) => {
            let mut l_bgColor: Tpl::Text;
            l_bgColor = dumpColorOpt(Tpl::emptyTxt.clone(), i_backgroundColor.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<y:NodeLabel alignment=\"center\" autoSizePolicy=\"content\" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_bgColor.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" fontFamily=\"Dialog\" fontSize=\"12\" fontStyle=\"")).clone() }))?;
            txt = dumpFontStyle(txt.clone(), i_fontStyle.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" hasLineColor=\"false\" modelName=\"corners\" modelPosition=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_position.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" textColor=\"#000000\" visible=\"true\">")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_text.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</y:NodeLabel>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_22(mut in_txt: Tpl::Text, mut in_mArg: GraphML::Attribute, mut in_a_val: ArcStr, mut in_a_idx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_val.clone(), in_a_idx.clone()) {
        (mut txt, GraphML::Attribute { attType: GraphML::AttributeType::TYPE_STRING, .. }, mut a_val, mut a_idx) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<data key=\"cust")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_idx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"><![CDATA[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_val.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]]></data>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_val, mut a_idx) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<data key=\"cust")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_idx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\">")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_val.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</data>")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpAttKey(mut in_txt: Tpl::Text, mut in_a_key: (i32, ArcStr), mut in_a_graphAttributes: metamodelica::Array<GraphML::Attribute>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_key.clone(), in_a_graphAttributes.clone()) {
        (mut txt, (mut i_idx, mut i_val), mut a_graphAttributes) => {
            let mut ret_0: GraphML::Attribute;
            ret_0 = a_graphAttributes.clone().borrow()[(i_idx.clone()-1) as usize].clone();
            txt = fun_22(txt.clone(), ret_0.clone(), (i_val.clone()).clone(), i_idx.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpAttDef(mut in_txt: Tpl::Text, mut in_a_attribute: GraphML::Attribute) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_attribute.clone()) {
        (mut txt, GraphML::Attribute { defaultValue: mut i_defaultValue, attIdx: mut i_attIdx, attTarget: mut i_attTarget, attType: mut i_attType, name: mut i_name }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<key attr.name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" attr.type=\"")).clone() }))?;
            txt = dumpAttType(txt.clone(), i_attType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" for=\"")).clone() }))?;
            txt = dumpAttTarget(txt.clone(), i_attTarget.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" id=\"cust")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_attIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<default>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_defaultValue.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</default>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</key>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpAttType(mut in_txt: Tpl::Text, mut in_a_type: GraphML::AttributeType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_type.clone()) {
        (mut txt, GraphML::AttributeType::TYPE_STRING) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("string")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::AttributeType::TYPE_BOOLEAN) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::AttributeType::TYPE_INTEGER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::AttributeType::TYPE_DOUBLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("double")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpAttTarget(mut in_txt: Tpl::Text, mut in_a_target: GraphML::AttributeTarget) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_target.clone()) {
        (mut txt, GraphML::AttributeTarget::TARGET_NODE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("node")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::AttributeTarget::TARGET_EDGE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("edge")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::AttributeTarget::TARGET_GRAPH) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("graph")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpDirected(mut in_txt: Tpl::Text, mut in_a_directed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_directed.clone()) {
        (mut txt, true) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("directed")).clone() }))?;
            txt.clone()
        },
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("undirected")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpColorOpt(mut in_txt: Tpl::Text, mut in_a_colorOpt: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_colorOpt.clone()) {
        (mut txt, Some(mut i_col)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("backgroundColor=\"#")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_col.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFontStyle(mut in_txt: Tpl::Text, mut in_a_fontStyle: GraphML::FontStyle) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fontStyle.clone()) {
        (mut txt, GraphML::FontStyle::FONTPLAIN) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("plain")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::FontStyle::FONTBOLD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("bold")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::FontStyle::FONTITALIC) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("italic")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::FontStyle::FONTBOLDITALIC) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("bolditalic")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpLineType(mut in_txt: Tpl::Text, mut in_a_lineType: GraphML::LineType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_lineType.clone()) {
        (mut txt, GraphML::LineType::LINE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("line")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::LineType::DASHED) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dashed")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::LineType::DASHEDDOTTED) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dasheddotted")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpArrowType(mut in_txt: Tpl::Text, mut in_a_arrowType: GraphML::ArrowType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_arrowType.clone()) {
        (mut txt, GraphML::ArrowType::ARROWSTANDART) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("standard")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ArrowType::ARROWNONE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("none")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ArrowType::ARROWCONCAVE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("concave")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpShapeType(mut in_txt: Tpl::Text, mut in_a_shape: GraphML::ShapeType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_shape.clone()) {
        (mut txt, GraphML::ShapeType::RECTANGLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rectangle")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::ROUNDRECTANGLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("roundrectangle")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::ELLIPSE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ellipse")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::PARALLELOGRAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parallelogram")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::HEXAGON) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("hexagon")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::TRIANGLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("triangle")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::OCTAGON) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("octagon")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::DIAMOND) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("diamond")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::TRAPEZOID) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("trapezoid")).clone() }))?;
            txt.clone()
        },
        (mut txt, GraphML::ShapeType::TRAPEZOID2) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("trapezoid2")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

