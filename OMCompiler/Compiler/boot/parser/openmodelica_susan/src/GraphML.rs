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

use crate::GraphMLDumpTpl;
use crate::Tpl;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

//TODO: Use HashTable for nodes to prevent duplicates
// -------------------------
// Constant types
// -------------------------
pub const COLOR_BLACK: &'static str = "000000";

pub const COLOR_BLUE: &'static str = "0000FF";

pub const COLOR_GREEN: &'static str = "339966";

pub const COLOR_RED: &'static str = "FF0000";

pub const COLOR_DARKRED: &'static str = "800000";

pub const COLOR_WHITE: &'static str = "FFFFFF";

pub const COLOR_YELLOW: &'static str = "FFFF00";

pub const COLOR_GRAY: &'static str = "C0C0C0";

pub const COLOR_PURPLE: &'static str = "993366";

pub const COLOR_ORANGE: &'static str = "FFCC00";

pub const COLOR_ORANGE2: &'static str = "FF6600";

pub const COLOR_DARKGRAY: &'static str = "666666";

pub const COLOR_RED2: &'static str = "F0988E";

pub const COLOR_GREEN2: &'static str = "98B954";

pub const COLOR_CYAN: &'static str = "46BED8";

pub const COLOR_PINK: &'static str = "CF8CB7";

pub const COLOR_GREEN3: &'static str = "008080";

pub const LINEWIDTH_STANDARD: metamodelica::Real = metamodelica::OrderedFloat(2.0_f64);

pub const LINEWIDTH_BOLD: metamodelica::Real = metamodelica::OrderedFloat(4.0_f64);

pub const FONTSIZE_STANDARD: i32 = 12;

pub const FONTSIZE_BIG: i32 = 20;

pub const FONTSIZE_SMALL: i32 = 8;

pub const BORDERWIDTH_STANDARD: metamodelica::Real = metamodelica::OrderedFloat(1.0_f64);

pub const BORDERWIDTH_BOLD: metamodelica::Real = metamodelica::OrderedFloat(4.0_f64);

// -------------------------
// Data structures
// -------------------------
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphInfo {
    GRAPHINFO {
        graphs: Arc<metamodelica::List<Graph>>,
        graphCount: i32,
        nodes: Arc<metamodelica::List<Node>>,
        nodeCount: i32,
        edges: Arc<metamodelica::List<Edge>>,
        edgeCount: i32,
        attributes: Arc<metamodelica::List<Attribute>>,
        graphNodeKey: ArcStr,
        graphEdgeKey: ArcStr,
    },
    GRAPHINFOARR {
        graphs: metamodelica::Array<Graph>,
        nodes: metamodelica::Array<Node>,
        edges: Arc<metamodelica::List<Edge>>,
        attributes: metamodelica::Array<Attribute>,
        graphNodeKey: ArcStr,
        graphEdgeKey: ArcStr,
    },
}
pub use self::GraphInfo::{GRAPHINFO,GRAPHINFOARR};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Graph {
    pub id: ArcStr,
    pub directed: bool,
    pub nodeIdc: Arc<metamodelica::List<i32>>,
    pub attValues: Arc<metamodelica::List<(i32, ArcStr)>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            id: Default::default(),
            directed: Default::default(),
            nodeIdc: Default::default(),
            attValues: Default::default(),
        }
    }
}

pub type GRAPH = Graph;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Node {
    NODE {
        id: ArcStr,
        color: ArcStr,
        border: metamodelica::Real,
        nodeLabels: Arc<metamodelica::List<NodeLabel>>,
        shapeType: ShapeType,
        optDesc: Option<ArcStr>,
        attValues: Arc<metamodelica::List<(i32, ArcStr)>>,
    },
    GROUPNODE {
        id: ArcStr,
        internalGraphIdx: i32,
        isFolded: bool,
        header: ArcStr,
    },
}
pub use self::Node::{NODE,GROUPNODE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge {
    pub id: ArcStr,
    pub target: ArcStr,
    pub source: ArcStr,
    pub color: ArcStr,
    pub lineType: LineType,
    pub lineWidth: metamodelica::Real,
    pub smooth: bool,
    pub edgeLabels: Arc<metamodelica::List<EdgeLabel>>,
    pub arrows: (ArrowType, ArrowType),
    pub attValues: Arc<metamodelica::List<(i32, ArcStr)>>,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            id: Default::default(),
            target: Default::default(),
            source: Default::default(),
            color: Default::default(),
            lineType: Default::default(),
            lineWidth: Default::default(),
            smooth: Default::default(),
            edgeLabels: Default::default(),
            arrows: Default::default(),
            attValues: Default::default(),
        }
    }
}

pub type EDGE = Edge;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attribute {
    pub attIdx: i32,
    pub defaultValue: ArcStr,
    pub name: ArcStr,
    pub attType: AttributeType,
    pub attTarget: AttributeTarget,
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            attIdx: Default::default(),
            defaultValue: Default::default(),
            name: Default::default(),
            attType: Default::default(),
            attTarget: Default::default(),
        }
    }
}

pub type ATTRIBUTE = Attribute;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeLabel {
    NODELABEL_INTERNAL {
        text: ArcStr,
        backgroundColor: Option<ArcStr>,
        fontStyle: FontStyle,
    },
    NODELABEL_CORNER {
        text: ArcStr,
        backgroundColor: Option<ArcStr>,
        fontStyle: FontStyle,
        position: ArcStr,
    },
}
pub use self::NodeLabel::{NODELABEL_INTERNAL,NODELABEL_CORNER};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeLabel {
    pub text: ArcStr,
    pub backgroundColor: Option<ArcStr>,
    pub fontSize: i32,
}

impl Default for EdgeLabel {
    fn default() -> Self {
        Self {
            text: Default::default(),
            backgroundColor: Default::default(),
            fontSize: Default::default(),
        }
    }
}

pub type EDGELABEL = EdgeLabel;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontStyle {
    FONTPLAIN,
    FONTBOLD,
    FONTITALIC,
    FONTBOLDITALIC,
}
pub use self::FontStyle::{FONTPLAIN,FONTBOLD,FONTITALIC,FONTBOLDITALIC};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShapeType {
    RECTANGLE,
    ROUNDRECTANGLE,
    ELLIPSE,
    PARALLELOGRAM,
    HEXAGON,
    TRIANGLE,
    OCTAGON,
    DIAMOND,
    TRAPEZOID,
    TRAPEZOID2,
}
pub use self::ShapeType::{RECTANGLE,ROUNDRECTANGLE,ELLIPSE,PARALLELOGRAM,HEXAGON,TRIANGLE,OCTAGON,DIAMOND,TRAPEZOID,TRAPEZOID2};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LineType {
    LINE,
    DASHED,
    DASHEDDOTTED,
}
impl Default for LineType {
    fn default() -> Self { Self::LINE }
}
pub use self::LineType::{LINE,DASHED,DASHEDDOTTED};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrowType {
    ARROWSTANDART,
    ARROWNONE,
    ARROWCONCAVE,
}
impl Default for ArrowType {
    fn default() -> Self { Self::ARROWSTANDART }
}
pub use self::ArrowType::{ARROWSTANDART,ARROWNONE,ARROWCONCAVE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeType {
    TYPE_STRING,
    TYPE_BOOLEAN,
    TYPE_INTEGER,
    TYPE_DOUBLE,
}
impl Default for AttributeType {
    fn default() -> Self { Self::TYPE_STRING }
}
pub use self::AttributeType::{TYPE_STRING,TYPE_BOOLEAN,TYPE_INTEGER,TYPE_DOUBLE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeTarget {
    TARGET_NODE,
    TARGET_EDGE,
    TARGET_GRAPH,
}
impl Default for AttributeTarget {
    fn default() -> Self { Self::TARGET_NODE }
}
pub use self::AttributeTarget::{TARGET_NODE,TARGET_EDGE,TARGET_GRAPH};

// -------------------------
// Logic
// -------------------------
pub fn createGraphInfo() -> GraphInfo {
    let mut oGraphInfo: GraphInfo;
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: metamodelica::nil(), graphCount: 0, nodes: metamodelica::nil(), nodeCount: 0, edges: metamodelica::nil(), edgeCount: 0, attributes: metamodelica::nil(), graphNodeKey: (literal!("gi1")).clone(), graphEdgeKey: (literal!("gi2")).clone() };
    oGraphInfo
}

pub fn addGraph(mut id: ArcStr, mut directed: bool, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Graph, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oGraph: (Graph, i32);
    let mut tmpGraph: Graph = <Graph as ::std::default::Default>::default();
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut graphCount: i32 = 0;
    let mut nodes: Arc<metamodelica::List<Node>> = metamodelica::nil();
    let mut nodeCount: i32 = 0;
    let mut edges: Arc<metamodelica::List<Edge>> = metamodelica::nil();
    let mut edgeCount: i32 = 0;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut graphNodeKey: ArcStr = arcstr::literal!("");
    let mut graphEdgeKey: ArcStr = arcstr::literal!("");
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    graphCount = graphCount.clone() + 1;
    tmpGraph = Graph { id: (id.clone()).clone(), directed: directed.clone(), nodeIdc: metamodelica::nil(), attValues: metamodelica::nil() };
    graphs = cons(tmpGraph.clone(), graphs.clone());
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs.clone(), graphCount: graphCount.clone(), nodes: nodes.clone(), nodeCount: nodeCount.clone(), edges: edges.clone(), edgeCount: edgeCount.clone(), attributes: attributes.clone(), graphNodeKey: (graphNodeKey.clone()).clone(), graphEdgeKey: (graphEdgeKey.clone()).clone() };
    oGraph = (tmpGraph.clone(), graphCount.clone());
    Ok((oGraphInfo, oGraph))
}

pub fn addNode(mut id: ArcStr, mut backgroundColor: ArcStr, mut borderWidth: metamodelica::Real, mut nodeLabels: Arc<metamodelica::List<NodeLabel>>, mut shapeType: ShapeType, mut optDesc: Option<ArcStr>, mut attValues: Arc<metamodelica::List<(i32, ArcStr)>>, mut iGraphIdx: i32, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Node, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oNode: (Node, i32);
    let mut tmpNode: Node;
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut graphCount: i32 = 0;
    let mut nodes: Arc<metamodelica::List<Node>> = metamodelica::nil();
    let mut nodeCount: i32 = 0;
    let mut edges: Arc<metamodelica::List<Edge>> = metamodelica::nil();
    let mut edgeCount: i32 = 0;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut graphNodeKey: ArcStr = arcstr::literal!("");
    let mut graphEdgeKey: ArcStr = arcstr::literal!("");
    let mut iGraph: Graph = <Graph as ::std::default::Default>::default();
    let mut gid: ArcStr = arcstr::literal!("");
    let mut directed: bool = false;
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut gAttValues: Arc<metamodelica::List<(i32, ArcStr)>> = metamodelica::nil();
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    iGraph = (graphs.clone()).get(graphCount.clone() - iGraphIdx.clone() + 1)?;
    let Graph { id: __pa9, directed: __pa10, nodeIdc: __pa11, attValues: __pa12 } = (iGraph.clone()) else { bail!("pattern mismatch") };
    gid = __pa9.clone();
    directed = __pa10.clone();
    nodeIdc = __pa11.clone();
    gAttValues = __pa12.clone();
    nodeCount = nodeCount.clone() + 1;
    tmpNode = Node::NODE { id: (id.clone()).clone(), color: (backgroundColor.clone()).clone(), border: borderWidth.clone(), nodeLabels: nodeLabels.clone(), shapeType: shapeType.clone(), optDesc: optDesc.clone(), attValues: attValues.clone() };
    nodes = cons(tmpNode.clone(), nodes.clone());
    nodeIdc = cons(nodeCount.clone(), nodeIdc.clone());
    iGraph = Graph { id: (gid.clone()).clone(), directed: directed.clone(), nodeIdc: nodeIdc.clone(), attValues: gAttValues.clone() };
    graphs = List::set(graphs.clone(), graphCount.clone() - iGraphIdx.clone() + 1, iGraph.clone())?;
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs.clone(), graphCount: graphCount.clone(), nodes: nodes.clone(), nodeCount: nodeCount.clone(), edges: edges.clone(), edgeCount: edgeCount.clone(), attributes: attributes.clone(), graphNodeKey: (graphNodeKey.clone()).clone(), graphEdgeKey: (graphEdgeKey.clone()).clone() };
    oNode = (tmpNode.clone(), nodeCount.clone());
    Ok((oGraphInfo, oNode))
}

pub fn addGroupNode(mut id: ArcStr, mut iGraphIdx: i32, mut isFolded: bool, mut iHeader: ArcStr, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Node, i32), (Graph, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oNode: (Node, i32);
    let mut oGraph: (Graph, i32);
    let mut tmpGraphInfo: GraphInfo;
    let mut tmpNode: Node;
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut graphCount: i32 = 0;
    let mut nodes: Arc<metamodelica::List<Node>> = metamodelica::nil();
    let mut nodeCount: i32 = 0;
    let mut edges: Arc<metamodelica::List<Edge>> = metamodelica::nil();
    let mut edgeCount: i32 = 0;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut graphNodeKey: ArcStr = arcstr::literal!("");
    let mut graphEdgeKey: ArcStr = arcstr::literal!("");
    let mut iGraph: Graph = <Graph as ::std::default::Default>::default();
    let mut newGraph: Graph = <Graph as ::std::default::Default>::default();
    let mut gid: ArcStr = arcstr::literal!("");
    let mut directed: bool = false;
    let mut newGraphIdx: i32 = 0;
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut attValues: Arc<metamodelica::List<(i32, ArcStr)>> = metamodelica::nil();
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    iGraph = (graphs.clone()).get(graphCount.clone() - iGraphIdx.clone() + 1)?;
    let Graph { id: __pa9, directed: __pa10, nodeIdc: __pa11, attValues: __pa12 } = (iGraph.clone()) else { bail!("pattern mismatch") };
    gid = __pa9.clone();
    directed = __pa10.clone();
    nodeIdc = __pa11.clone();
    attValues = __pa12.clone();
    let (__pa13, (__pa14, __pa15)) = addGraph(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("g")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone(), directed.clone(), iGraphInfo.clone())?;
    tmpGraphInfo = __pa13.clone();
    newGraph = __pa14.clone();
    newGraphIdx = __pa15.clone();
    let GraphInfo::GRAPHINFO { graphs: __pa16, graphCount: __pa17, nodes: __pa18, nodeCount: __pa19, edges: __pa20, edgeCount: __pa21, attributes: __pa22, graphNodeKey: __pa23, graphEdgeKey: __pa24 } = (tmpGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa16.clone();
    graphCount = __pa17.clone();
    nodes = __pa18.clone();
    nodeCount = __pa19.clone();
    edges = __pa20.clone();
    edgeCount = __pa21.clone();
    attributes = __pa22.clone();
    graphNodeKey = __pa23.clone();
    graphEdgeKey = __pa24.clone();
    nodeCount = nodeCount.clone() + 1;
    tmpNode = Node::GROUPNODE { id: (id.clone()).clone(), internalGraphIdx: newGraphIdx.clone(), isFolded: isFolded.clone(), header: (iHeader.clone()).clone() };
    nodes = cons(tmpNode.clone(), nodes.clone());
    nodeIdc = cons(nodeCount.clone(), nodeIdc.clone());
    iGraph = Graph { id: (gid.clone()).clone(), directed: directed.clone(), nodeIdc: nodeIdc.clone(), attValues: attValues.clone() };
    graphs = List::set(graphs.clone(), graphCount.clone() - iGraphIdx.clone() + 1, iGraph.clone())?;
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs.clone(), graphCount: graphCount.clone(), nodes: nodes.clone(), nodeCount: nodeCount.clone(), edges: edges.clone(), edgeCount: edgeCount.clone(), attributes: attributes.clone(), graphNodeKey: (graphNodeKey.clone()).clone(), graphEdgeKey: (graphEdgeKey.clone()).clone() };
    oNode = (tmpNode.clone(), nodeCount.clone());
    oGraph = (newGraph.clone(), newGraphIdx.clone());
    Ok((oGraphInfo, oNode, oGraph))
}

pub fn addEdge(mut id: ArcStr, mut target: ArcStr, mut source: ArcStr, mut color: ArcStr, mut lineType: LineType, mut lineWidth: metamodelica::Real, mut smooth: bool, mut labels: Arc<metamodelica::List<EdgeLabel>>, mut arrows: (ArrowType, ArrowType), mut attValues: Arc<metamodelica::List<(i32, ArcStr)>>, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Edge, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oEdge: (Edge, i32);
    let mut tmpEdge: Edge = <Edge as ::std::default::Default>::default();
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut graphCount: i32 = 0;
    let mut nodes: Arc<metamodelica::List<Node>> = metamodelica::nil();
    let mut nodeCount: i32 = 0;
    let mut edges: Arc<metamodelica::List<Edge>> = metamodelica::nil();
    let mut edgeCount: i32 = 0;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut graphNodeKey: ArcStr = arcstr::literal!("");
    let mut graphEdgeKey: ArcStr = arcstr::literal!("");
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    edgeCount = edgeCount.clone() + 1;
    tmpEdge = Edge { id: (id.clone()).clone(), target: (target.clone()).clone(), source: (source.clone()).clone(), color: (color.clone()).clone(), lineType: lineType.clone(), lineWidth: lineWidth.clone(), smooth: smooth.clone(), edgeLabels: labels.clone(), arrows: arrows.clone(), attValues: attValues.clone() };
    edges = cons(tmpEdge.clone(), edges.clone());
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs.clone(), graphCount: graphCount.clone(), nodes: nodes.clone(), nodeCount: nodeCount.clone(), edges: edges.clone(), edgeCount: edgeCount.clone(), attributes: attributes.clone(), graphNodeKey: (graphNodeKey.clone()).clone(), graphEdgeKey: (graphEdgeKey.clone()).clone() };
    oEdge = (tmpEdge.clone(), edgeCount.clone());
    Ok((oGraphInfo, oEdge))
}

pub fn addAttribute(mut defaultValue: ArcStr, mut name: ArcStr, mut attType: AttributeType, mut attTarget: AttributeTarget, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Attribute, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oAttribute: (Attribute, i32);
    let mut tmpAttribute: Attribute = <Attribute as ::std::default::Default>::default();
    let mut attIdx: i32 = 0;
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut graphCount: i32 = 0;
    let mut nodes: Arc<metamodelica::List<Node>> = metamodelica::nil();
    let mut nodeCount: i32 = 0;
    let mut edges: Arc<metamodelica::List<Edge>> = metamodelica::nil();
    let mut edgeCount: i32 = 0;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut graphNodeKey: ArcStr = arcstr::literal!("");
    let mut graphEdgeKey: ArcStr = arcstr::literal!("");
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    attIdx = (attributes.clone().len() as i32) + 1;
    tmpAttribute = Attribute { attIdx: attIdx.clone(), defaultValue: (defaultValue.clone()).clone(), name: (name.clone()).clone(), attType: attType.clone(), attTarget: attTarget.clone() };
    attributes = cons(tmpAttribute.clone(), attributes.clone());
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs.clone(), graphCount: graphCount.clone(), nodes: nodes.clone(), nodeCount: nodeCount.clone(), edges: edges.clone(), edgeCount: edgeCount.clone(), attributes: attributes.clone(), graphNodeKey: (graphNodeKey.clone()).clone(), graphEdgeKey: (graphEdgeKey.clone()).clone() };
    oAttribute = (tmpAttribute.clone(), attIdx.clone());
    Ok((oGraphInfo, oAttribute))
}

pub fn addGraphAttributeValue(mut iValue: (i32, ArcStr), mut iGraphIdx: i32, mut iGraphInfo: GraphInfo) -> Result<GraphInfo> {
    let mut oGraphInfo: GraphInfo;
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut graphCount: i32 = 0;
    let mut nodes: Arc<metamodelica::List<Node>> = metamodelica::nil();
    let mut nodeCount: i32 = 0;
    let mut edges: Arc<metamodelica::List<Edge>> = metamodelica::nil();
    let mut edgeCount: i32 = 0;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut graphNodeKey: ArcStr = arcstr::literal!("");
    let mut graphEdgeKey: ArcStr = arcstr::literal!("");
    let mut iGraph: Graph = <Graph as ::std::default::Default>::default();
    let mut gid: ArcStr = arcstr::literal!("");
    let mut directed: bool = false;
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut attValues: Arc<metamodelica::List<(i32, ArcStr)>> = metamodelica::nil();
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    iGraph = (graphs.clone()).get(graphCount.clone() - iGraphIdx.clone() + 1)?;
    let Graph { id: __pa9, directed: __pa10, nodeIdc: __pa11, attValues: __pa12 } = (iGraph.clone()) else { bail!("pattern mismatch") };
    gid = __pa9.clone();
    directed = __pa10.clone();
    nodeIdc = __pa11.clone();
    attValues = __pa12.clone();
    attValues = cons(iValue.clone(), attValues.clone());
    iGraph = Graph { id: (gid.clone()).clone(), directed: directed.clone(), nodeIdc: nodeIdc.clone(), attValues: attValues.clone() };
    graphs = List::set(graphs.clone(), graphCount.clone() - iGraphIdx.clone() + 1, iGraph.clone())?;
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs.clone(), graphCount: graphCount.clone(), nodes: nodes.clone(), nodeCount: nodeCount.clone(), edges: edges.clone(), edgeCount: edgeCount.clone(), attributes: attributes.clone(), graphNodeKey: (graphNodeKey.clone()).clone(), graphEdgeKey: (graphEdgeKey.clone()).clone() };
    Ok(oGraphInfo)
}

// -------------------------
// Helper
// -------------------------
pub fn getMainGraph(mut iGraphInfo: GraphInfo) -> Result<Option<(i32, Graph)>> {
    let mut oGraph: Option<(i32, Graph)> = None;
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut firstGraph: Graph = <Graph as ::std::default::Default>::default();
    oGraph = (match iGraphInfo.clone() {
        GraphInfo::GRAPHINFO { graphCount: 0, .. } => None,
        GraphInfo::GRAPHINFO { graphs: mut graphs, .. } => {
            firstGraph = listHead(graphs.clone())?;
            Some((1, firstGraph.clone()))
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(oGraph)
}

pub fn getAttributeByNameAndTarget(mut iAttributeName: ArcStr, mut iAttributeTarget: AttributeTarget, mut iGraphInfo: GraphInfo) -> Result<Option<(Attribute, i32)>> {
    let mut oAttribute: Option<(Attribute, i32)> = None;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut tmpRes: Option<(Attribute, i32)> = None;
    oAttribute = (match iGraphInfo.clone() {
        GraphInfo::GRAPHINFO { attributes: mut attributes, .. } => {
            tmpRes = getAttributeByNameAndTargetTail(attributes.clone(), (iAttributeName.clone()).clone(), iAttributeTarget.clone())?;
            tmpRes.clone()
        },
        GraphInfo::GRAPHINFO { attributes: mut attributes, .. } => {
            tmpRes = getAttributeByNameAndTargetTail(attributes.clone(), (iAttributeName.clone()).clone(), iAttributeTarget.clone())?;
            tmpRes.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(oAttribute)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAttributeByNameAndTargetTail(mut iList: Arc<metamodelica::List<Attribute>>, mut iAttributeName: ArcStr, mut iAttributeTarget: AttributeTarget) -> Result<Option<(Attribute, i32)>> {
    let mut oAttribute: Option<(Attribute, i32)> = None;
    let mut rest: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut attIdx: i32 = 0;
    let mut name: ArcStr = arcstr::literal!("");
    let mut head: Attribute = <Attribute as ::std::default::Default>::default();
    let mut attTarget: AttributeTarget = AttributeTarget::TARGET_EDGE;
    let mut tmpAttribute: Option<(Attribute, i32)> = None;
    oAttribute = 'mc: {
        let __mc_input = iList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head @ Attribute { attTarget, name, attIdx, .. }, tail: rest } => {
                    let true = (stringEq((name.clone()).clone(), (iAttributeName.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (compareAttributeTargets(iAttributeTarget.clone(), attTarget.clone())?) else { bail!("pattern mismatch") };
                    Ok(Some((head.clone(), attIdx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut tmpAttribute: Option<(Attribute, i32)> = tmpAttribute.clone();
                    tmpAttribute = getAttributeByNameAndTargetTail(rest.clone(), (iAttributeName.clone()).clone(), iAttributeTarget.clone())?;
                    Ok(tmpAttribute.clone())
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
    Ok(oAttribute)
}

fn compareAttributeTargets(mut iTarget1: AttributeTarget, mut iTarget2: AttributeTarget) -> Result<bool> {
    let mut oEqual: bool = false;
    let mut tarInt1: i32 = 0;
    let mut tarInt2: i32 = 0;
    tarInt1 = compareAttributeTarget0(iTarget1.clone())?;
    tarInt2 = compareAttributeTarget0(iTarget2.clone())?;
    oEqual = intEq(tarInt1.clone(), tarInt2.clone());
    Ok(oEqual)
}

fn compareAttributeTarget0(mut iTarget: AttributeTarget) -> Result<i32> {
    let mut oCodec: i32 = 0;
    oCodec = (match iTarget.clone() {
        AttributeTarget::TARGET_NODE { .. } => 0,
        AttributeTarget::TARGET_EDGE { .. } => 1,
        AttributeTarget::TARGET_GRAPH { .. } => 1,
        _ => bail!("match: no arm matched"),
    });
    Ok(oCodec)
}

// -------------------------
// Dump
// -------------------------
pub fn dumpGraph(mut iGraphInfo: GraphInfo, mut iFileName: ArcStr) -> Result<()> {
    let mut iGraphInfoArr: GraphInfo;
    iGraphInfoArr = convertToGraphInfoArr(iGraphInfo.clone())?;
    Tpl::tplNoret2((std::sync::Arc::new(GraphMLDumpTpl::dumpGraphInfo) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, GraphInfo, ArcStr) -> Result<Tpl::Text> + 'static>), iGraphInfoArr.clone(), (iFileName.clone()).clone())?;
    Ok(())
}

fn convertToGraphInfoArr(mut iGraphInfo: GraphInfo) -> Result<GraphInfo> {
    let mut oGraphInfo: GraphInfo;
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut graphsArr: metamodelica::Array<Graph>;
    let mut graphCount: i32 = 0;
    let mut nodes: Arc<metamodelica::List<Node>> = metamodelica::nil();
    let mut nodesArr: metamodelica::Array<Node>;
    let mut nodeCount: i32 = 0;
    let mut edges: Arc<metamodelica::List<Edge>> = metamodelica::nil();
    let mut edgeCount: i32 = 0;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut attributesArr: metamodelica::Array<Attribute>;
    let mut graphNodeKey: ArcStr = arcstr::literal!("");
    let mut graphEdgeKey: ArcStr = arcstr::literal!("");
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    graphsArr = metamodelica::arrayFromVec(graphs.clone().into_iter().cloned().collect());
    nodesArr = metamodelica::arrayFromVec(nodes.clone().into_iter().cloned().collect());
    attributesArr = List::listArrayReverse(attributes.clone())?;
    oGraphInfo = GraphInfo::GRAPHINFOARR { graphs: graphsArr.clone(), nodes: nodesArr.clone(), edges: edges.clone(), attributes: attributesArr.clone(), graphNodeKey: (graphNodeKey.clone()).clone(), graphEdgeKey: (graphEdgeKey.clone()).clone() };
    Ok(oGraphInfo)
}

// -------------------------
// debug prints
// -------------------------
pub fn printGraphInfo(mut iGraphInfo: GraphInfo) -> Result<()> {
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut graphCount: i32 = 0;
    let mut nodes: Arc<metamodelica::List<Node>> = metamodelica::nil();
    let mut nodeCount: i32 = 0;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut graphNodeKey: ArcStr = arcstr::literal!("");
    let mut graphEdgeKey: ArcStr = arcstr::literal!("");
    let GraphInfo::GRAPHINFO { graphEdgeKey: __pa0, graphNodeKey: __pa1, attributes: __pa2, nodeCount: __pa3, nodes: __pa4, graphCount: __pa5, graphs: __pa6, .. } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphEdgeKey = __pa0.clone();
    graphNodeKey = __pa1.clone();
    attributes = __pa2.clone();
    nodeCount = __pa3.clone();
    nodes = __pa4.clone();
    graphCount = __pa5.clone();
    graphs = __pa6.clone();
    List::map_0(nodes.clone(), (std::sync::Arc::new(printNode) as std::sync::Arc<dyn ::std::ops::Fn(Node) -> Result<()> + 'static>));
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("nodeCount: ")); __mm_s.push_str(&*intString(nodeCount.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("graphCount: ")); __mm_s.push_str(&*intString(graphCount.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn printNode(mut node: Node) -> Result<()> {
    let mut id: ArcStr = arcstr::literal!("");
    let mut atts: ArcStr = arcstr::literal!("");
    let mut optDesc: Option<ArcStr> = None;
    let mut attValues: Arc<metamodelica::List<(i32, ArcStr)>> = metamodelica::nil();
    let Node::NODE { attValues: __pa0, optDesc: __pa1, id: __pa2, .. } = (node.clone()) else { bail!("pattern mismatch") };
    attValues = __pa0.clone();
    optDesc = __pa1.clone();
    id = __pa2.clone();
    atts = stringDelimitList(List::map(attValues.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _))), (literal!(" | ")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("node: ")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" desc: ")); __mm_s.push_str(&*Util::getOption(optDesc.clone())?); __mm_s.push_str(&*literal!("\n\tatts: ")); __mm_s.push_str(&*atts.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

