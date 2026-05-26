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

use crate::Error;
use openmodelica_util_datatypes_basic::List;

pub fn buildGraph<NodeType: Clone + 'static, ArgType: Clone + 'static>(mut inNodes: Arc<metamodelica::List<NodeType>>, mut inEdgeFunc: Arc<dyn ::std::ops::Fn(NodeType, ArgType) -> Result<Arc<metamodelica::List<NodeType>>> + 'static>, mut inEdgeArg: ArgType) -> Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> {
    pub type EdgeFunc<NodeType: Clone, ArgType: Clone> = fn(NodeType, ArgType) -> Result<Arc<metamodelica::List<NodeType>>>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    outGraph = List::zip(inNodes.clone(), List::map1(inNodes.clone(), inEdgeFunc.clone(), inEdgeArg.clone()));
    outGraph
}

pub fn emptyGraph<NodeType: Clone + 'static>(mut inNodes: Arc<metamodelica::List<NodeType>>) -> Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> {
    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    outGraph = List::map(inNodes.clone(), Arc::new(fnptr!(emptyGraphHelper, _)));
    outGraph
}

fn emptyGraphHelper<NodeType: Clone + 'static>(mut nt: NodeType) -> (NodeType, Arc<metamodelica::List<NodeType>>) {
    let mut out: (NodeType, Arc<metamodelica::List<NodeType>>);
    out = (nt.clone(), metamodelica::nil());
    out
}

pub fn topologicalSort<NodeType: Clone + 'static>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<NodeType>>, Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>)> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outNodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
    let mut outRemainingGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    let mut start_nodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    let mut rest_nodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    (rest_nodes, start_nodes) = List::splitOnTrue(inGraph.clone(), Arc::new(fnptr!(hasOutgoingEdges, _)));
    (outNodes, outRemainingGraph) = topologicalSort2(start_nodes.clone(), rest_nodes.clone(), metamodelica::nil(), inEqualFunc.clone())?;
    Ok((outNodes, outRemainingGraph))
}

fn topologicalSort2<NodeType: Clone + 'static>(mut inStartNodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inRestNodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inAccumNodes: Arc<metamodelica::List<NodeType>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<NodeType>>, Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>)> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outNodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
    let mut outRemainingGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    (outNodes, outRemainingGraph) = (::match_deref::match_deref! { match &((inStartNodes.clone(), inRestNodes.clone(), inAccumNodes.clone(), inEqualFunc.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            (inAccumNodes.clone().reverse(), inRestNodes.clone())
        },
        (rest_start, Deref @ metamodelica::List::Nil, _, _) => {
            let mut node1: NodeType;
            let mut result: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
            result = inAccumNodes.clone();
            for mut n in &*rest_start.clone() {
                let mut n = n.clone();
                let __pa0 = ::match_deref::match_deref! { match &(n.clone()) {
                    (__pa0, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                node1 = __pa0.clone();
                result = cons(node1.clone(), result.clone());
            }
            result = result.clone().reverse();
            (result.clone(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: (node1, Deref @ metamodelica::List::Nil), tail: rest_start }, rest_rest, _, _) => {
            let mut rest_start_: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
            let mut new_start: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
            let mut result: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
            let mut rest_rest = (*rest_rest).clone();
            rest_rest = List::map2(rest_rest.clone(), Arc::new(removeEdge), node1.clone(), inEqualFunc.clone());
            (rest_rest, new_start) = List::splitOnTrue(rest_rest.clone(), Arc::new(fnptr!(hasOutgoingEdges, _)));
            rest_start_ = listAppend(rest_start.clone(), new_start.clone());
            (result, rest_rest) = topologicalSort2(rest_start_.clone(), rest_rest.clone(), cons(node1.clone(), inAccumNodes.clone()), inEqualFunc.clone())?;
            (result.clone(), rest_rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outNodes, outRemainingGraph))
}

fn hasOutgoingEdges<NodeType: Clone + 'static>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>)) -> bool {
    let mut outHasOutEdges: bool = false;
    outHasOutEdges = (::match_deref::match_deref! { match &(inNode.clone()) {
        (_, Deref @ metamodelica::List::Nil) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outHasOutEdges
}

fn removeEdge<NodeType: Clone + 'static>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>), mut inRemovedNode: NodeType, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<(NodeType, Arc<metamodelica::List<NodeType>>)> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outNode: (NodeType, Arc<metamodelica::List<NodeType>>);
    let mut node: NodeType;
    let mut edges: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
    (node, edges) = inNode.clone();
    (edges, _) = List::deleteMemberOnTrue(inRemovedNode.clone(), edges.clone(), inEqualFunc.clone())?;
    outNode = (node.clone(), edges.clone());
    Ok(outNode)
}

pub fn findCycles<NodeType: Clone + 'static>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outCycles: Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>> = metamodelica::nil();
    outCycles = findCycles2(inGraph.clone(), inGraph.clone(), inEqualFunc.clone())?;
    Ok(outCycles)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn findCycles2<NodeType: Clone + 'static>(mut inNodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outCycles: Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>> = metamodelica::nil();
    outCycles = 'mc: {
        let __mc_input = (inNodes.clone(), inGraph.clone(), inEqualFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: rest_nodes }, _, _) => {
                    let mut cycle: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    let mut rest_cycles: Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>> = metamodelica::nil();
                    let mut rest_nodes = (*rest_nodes).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(findCycleForNode(node.clone(), inGraph.clone(), metamodelica::nil(), inEqualFunc.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cycle = __pa0.clone();
                    rest_nodes = removeNodesFromGraph(cycle.clone(), rest_nodes.clone(), inEqualFunc.clone())?;
                    rest_cycles = findCycles2(rest_nodes.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    Ok(cons(cycle.clone(), rest_cycles.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest_nodes }, _, _) => {
                    let mut rest_cycles: Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>> = metamodelica::nil();
                    rest_cycles = findCycles2(rest_nodes.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    Ok(rest_cycles.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCycles)
}

fn findCycleForNode<NodeType: Clone + 'static>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>), mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inVisitedNodes: Arc<metamodelica::List<NodeType>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Option<Arc<metamodelica::List<NodeType>>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outCycle: Option<Arc<metamodelica::List<NodeType>>> = None;
    outCycle = 'mc: {
        let __mc_input = (inNode.clone(), inGraph.clone(), inVisitedNodes.clone(), inEqualFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((node, _), _, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
                    let mut start_node: NodeType;
                    let mut is_start_node: bool = false;
                    let mut opt_cycle: Option<Arc<metamodelica::List<NodeType>>> = None;
                    let true = (List::isMemberOnTrue(node.clone(), inVisitedNodes.clone(), inEqualFunc.clone())) else { bail!("pattern mismatch") };
                    start_node = List::last(inVisitedNodes.clone())?;
                    is_start_node = inEqualFunc(node.clone(), start_node.clone())?;
                    opt_cycle = if (is_start_node.clone()) {Some(inVisitedNodes.clone())} else {None};
                    Ok(opt_cycle.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((node, edges), _, _, _) => {
                    let mut visited_nodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    let mut cycle: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    visited_nodes = cons(node.clone(), inVisitedNodes.clone());
                    cycle = findCycleForNode2(edges.clone(), inGraph.clone(), visited_nodes.clone(), inEqualFunc.clone())?;
                    Ok(Some(cycle.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCycle)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findCycleForNode2<NodeType: Clone + 'static>(mut inNodes: Arc<metamodelica::List<NodeType>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inVisitedNodes: Arc<metamodelica::List<NodeType>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<NodeType>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outCycle: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
    outCycle = 'mc: {
        let __mc_input = (inNodes.clone(), inGraph.clone(), inVisitedNodes.clone(), inEqualFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: _ }, _, _, _) => {
                    let mut cycle: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    let mut graph_node: (NodeType, Arc<metamodelica::List<NodeType>>);
                    graph_node = findNodeInGraph(node.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(findCycleForNode(graph_node.clone(), inGraph.clone(), inVisitedNodes.clone(), inEqualFunc.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cycle = __pa0.clone();
                    Ok(cycle.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest_nodes }, _, _, _) => {
                    let mut cycle: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    cycle = findCycleForNode2(rest_nodes.clone(), inGraph.clone(), inVisitedNodes.clone(), inEqualFunc.clone())?;
                    Ok(cycle.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCycle)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findNodeInGraph<NodeType: Clone + 'static>(mut inNode: NodeType, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<(NodeType, Arc<metamodelica::List<NodeType>>)> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outNode: (NodeType, Arc<metamodelica::List<NodeType>>);
    outNode = 'mc: {
        let __mc_input = (inNode.clone(), inGraph.clone(), inEqualFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: graph_node @ (node, _), tail: _ }, _) => {
                    let true = (inEqualFunc(inNode.clone(), node.clone())?) else { bail!("pattern mismatch") };
                    Ok(graph_node.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest_graph }, _) => {
                    Ok(findNodeInGraph(inNode.clone(), rest_graph.clone(), inEqualFunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findIndexofNodeInGraph<NodeType: Clone + 'static>(mut inNode: NodeType, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inIndex: i32) -> Result<i32> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outIndex: i32 = 0;
    outIndex = 'mc: {
        let __mc_input = (inNode.clone(), inGraph.clone(), inEqualFunc.clone(), inIndex.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: (node, _), tail: _ }, _, _) => {
                    let true = (inEqualFunc(inNode.clone(), node.clone())?) else { bail!("pattern mismatch") };
                    Ok(inIndex.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest_graph }, _, _) => {
                    Ok(findIndexofNodeInGraph(inNode.clone(), rest_graph.clone(), inEqualFunc.clone(), inIndex.clone() + 1)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIndex)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeNodesFromGraph<NodeType: Clone + 'static>(mut inNodes: Arc<metamodelica::List<NodeType>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    outGraph = 'mc: {
        let __mc_input = (inNodes.clone(), inGraph.clone(), inEqualFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: (node, _), tail: rest_graph }, _) => {
                    let mut rest_nodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(List::deleteMemberOnTrue(node.clone(), inNodes.clone(), inEqualFunc.clone())?) {
                        (__pa0, Some(_)) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rest_nodes = __pa0.clone();
                    Ok(removeNodesFromGraph(rest_nodes.clone(), rest_graph.clone(), inEqualFunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: graph_node, tail: rest_graph }, _) => {
                    let mut rest_graph = (*rest_graph).clone();
                    rest_graph = removeNodesFromGraph(inNodes.clone(), rest_graph.clone(), inEqualFunc.clone())?;
                    Ok(cons(graph_node.clone(), rest_graph.clone()))
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
pub fn transposeGraph<NodeType: Clone + 'static + PartialEq>(mut intmpGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    outGraph = 'mc: {
        let __mc_input = (intmpGraph.clone(), inGraph.clone(), inEqualFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(intmpGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: (node, nodeList), tail: restGraph }, _) => {
                    let mut tmpGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
                    tmpGraph = List::fold2(nodeList.clone(), Arc::new(insertNodetoGraph), node.clone(), inEqualFunc.clone(), intmpGraph.clone());
                    tmpGraph = transposeGraph(tmpGraph.clone(), restGraph.clone(), inEqualFunc.clone())?;
                    Ok(tmpGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.transpose failed.")).clone()], metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

fn insertNodetoGraph<NodeType: Clone + 'static + PartialEq>(mut inNode: NodeType, mut inVertex: NodeType, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    outGraph = 'mc: {
        let __mc_input = (inNode.clone(), inVertex.clone(), inEqualFunc.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Cons { head: (node, rest), tail: restGraph }) => {
                    let mut rest = (*rest).clone();
                    let mut restGraph = (*restGraph).clone();
                    let true = (inEqualFunc(node.clone(), inNode.clone())?) else { bail!("pattern mismatch") };
                    rest = List::unionList(list![rest.clone(), list![inVertex.clone()]])?;
                    restGraph = insertNodetoGraph(inNode.clone(), inVertex.clone(), inEqualFunc.clone(), restGraph.clone())?;
                    Ok(cons((node.clone(), rest.clone()), restGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Cons { head: (node, rest), tail: restGraph }) => {
                    let mut restGraph = (*restGraph).clone();
                    let false = (inEqualFunc(node.clone(), inNode.clone())?) else { bail!("pattern mismatch") };
                    restGraph = insertNodetoGraph(inNode.clone(), inVertex.clone(), inEqualFunc.clone(), restGraph.clone())?;
                    Ok(cons((node.clone(), rest.clone()), restGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn allReachableNodes<NodeType: Clone + 'static>(mut intmpstorage: (Arc<metamodelica::List<NodeType>>, Arc<metamodelica::List<NodeType>>), mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<NodeType>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut reachableNodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(allReachableNodesWork(intmpstorage.clone(), inGraph.clone(), inEqualFunc.clone())?) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    reachableNodes = __pa0.clone();
    Ok(reachableNodes)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn allReachableNodesWork<NodeType: Clone + 'static>(mut intmpstorage: (Arc<metamodelica::List<NodeType>>, Arc<metamodelica::List<NodeType>>), mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Option<Arc<metamodelica::List<NodeType>>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut reachableNodes: Option<Arc<metamodelica::List<NodeType>>> = None;
    reachableNodes = 'mc: {
        let __mc_input = (intmpstorage.clone(), inGraph.clone(), inEqualFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ metamodelica::List::Nil, L), _, _) => {
                    let mut L = (*L).clone();
                    L = L.clone().reverse();
                    Ok(Some(L.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ metamodelica::List::Cons { head: node, tail: M }, L), _, _) => {
                    List::getMemberOnTrue(node.clone(), L.clone(), inEqualFunc.clone())?;
                    Ok(allReachableNodesWork((M.clone(), L.clone()), inGraph.clone(), inEqualFunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ metamodelica::List::Cons { head: node, tail: M }, L), _, _) => {
                    let mut edges: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    let mut M = (*M).clone();
                    let mut L = (*L).clone();
                    L = cons(node.clone(), L.clone());
                    (_, edges) = findNodeInGraph(node.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    M = listAppend(edges.clone(), M.clone());
                    Ok(allReachableNodesWork((M.clone(), L.clone()), inGraph.clone(), inEqualFunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.allReachableNodes failed.")).clone()], metamodelica::sourceInfo!())?;
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(reachableNodes)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn partialDistance2color<NodeType: Clone + 'static>(mut toColorNodes: Arc<metamodelica::List<NodeType>>, mut inforbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>, mut inColors: Arc<metamodelica::List<i32>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inGraphT: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inColored: metamodelica::Array<i32>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>) -> Result<metamodelica::Array<i32>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    pub type PrintFunc<NodeType: Clone> = fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()>;

    let mut outColored: metamodelica::Array<i32>;
    outColored = 'mc: {
        let __mc_input = (toColorNodes.clone(), inforbiddenColor.clone(), inColors.clone(), inGraph.clone(), inGraphT.clone(), inColored.clone(), inEqualFunc.clone(), inPrintFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    Ok(inColored.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, _, _, _, _, _, _) => {
                    let mut nodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    let mut forbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>;
                    let mut colored: metamodelica::Array<i32>;
                    let mut color: i32 = 0;
                    let mut index: i32 = 0;
                    index = (inColored.clone().borrow().len() as i32) - (rest.clone().len() as i32);
                    (_, nodes) = findNodeInGraph(node.clone(), inGraphT.clone(), inEqualFunc.clone())?;
                    forbiddenColor = addForbiddenColors(node.clone(), nodes.clone(), inColored.clone(), inforbiddenColor.clone(), inGraph.clone(), inEqualFunc.clone(), inPrintFunc.clone())?;
                    color = arrayFindMinColorIndex(forbiddenColor.clone(), node.clone(), 1, (inColored.clone().borrow().len() as i32) + 1, inEqualFunc.clone(), inPrintFunc.clone())?;
                    colored = {let _arr = inColored.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = color.clone(); _arr};
                    colored = partialDistance2color(rest.clone(), forbiddenColor.clone(), inColors.clone(), inGraph.clone(), inGraphT.clone(), colored.clone(), inEqualFunc.clone(), inPrintFunc.clone())?;
                    Ok(colored.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.partialDistance2color failed.")).clone()], metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outColored)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addForbiddenColors<NodeType: Clone + 'static>(mut inNode: NodeType, mut inNodes: Arc<metamodelica::List<NodeType>>, mut inColored: metamodelica::Array<i32>, mut inForbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>) -> Result<metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    pub type PrintFunc<NodeType: Clone> = fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()>;

    let mut outForbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>;
    outForbiddenColor = 'mc: {
        let __mc_input = (inNode.clone(), inNodes.clone(), inColored.clone(), inForbiddenColor.clone(), inGraph.clone(), inEqualFunc.clone(), inPrintFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok(inForbiddenColor.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, forbiddenColor, _, _, _) => {
                    let mut nodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
                    let mut indexes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut indexesColor: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut forbiddenColor1: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>;
                    (_, nodes) = findNodeInGraph(node.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    indexes = List::map3(nodes.clone(), Arc::new(findIndexofNodeInGraph), inGraph.clone(), inEqualFunc.clone(), 1);
                    indexes = List::select1(indexes.clone(), Arc::new(arrayElemetGtZero), inColored.clone());
                    indexesColor = List::map1(indexes.clone(), Arc::new(getArrayElem), inColored.clone());
                    List::map2_0(indexesColor.clone(), Arc::new(arrayUpdateListAppend), forbiddenColor.clone(), Some(list![inNode.clone()]));
                    forbiddenColor1 = addForbiddenColors(inNode.clone(), rest.clone(), inColored.clone(), forbiddenColor.clone(), inGraph.clone(), inEqualFunc.clone(), inPrintFunc.clone())?;
                    Ok(forbiddenColor1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.addForbiddenColors failed.")).clone()], metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outForbiddenColor)
}

fn getArrayElem<Type_a: Clone + 'static>(mut inIndex: i32, mut inArray: metamodelica::Array<Type_a>) -> Result<Type_a> {
    let mut outElem: Type_a;
    outElem = inArray.clone().borrow()[(inIndex.clone()-1) as usize].clone();
    Ok(outElem)
}

fn arrayUpdateListAppend<NodeType: Clone + 'static>(mut inIndex: i32, mut inArray: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>, mut inNode: Option<Arc<metamodelica::List<NodeType>>>) -> Result<()> {
    let mut arrayElem: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
    let _ = 'mc: {
        let __mc_input = (inIndex.clone(), inArray.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            {let _arr = inArray.clone(); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = inNode.clone(); _arr};
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.arrayUpdateListAppend failed.")).clone()], metamodelica::sourceInfo!())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn arrayElemetGtZero(mut inIndex: i32, mut inArray: metamodelica::Array<i32>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = intGt(inArray.clone().borrow()[(inIndex.clone()-1) as usize].clone(), 0);
    Ok(outBoolean)
}

fn arrayFindMinColorIndex<NodeType: Clone + 'static>(mut inForbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>, mut inNode: NodeType, mut inIndex: i32, mut inmaxIndex: i32, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>) -> Result<i32> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    pub type PrintFunc<NodeType: Clone> = fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()>;

    let mut outColor: i32 = 0;
    outColor = 'mc: {
        let __mc_input = (inForbiddenColor.clone(), inNode.clone(), inIndex.clone(), inmaxIndex.clone(), inEqualFunc.clone(), inPrintFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(inForbiddenColor.clone().borrow()[(inIndex.clone()-1) as usize].clone()) {
                None => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok(inIndex.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut nodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inForbiddenColor.clone().borrow()[(inIndex.clone()-1) as usize].clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            nodes = __pa0.clone();
            if '__try1: {
                let _ = unwrap_break_err!(List::getMemberOnTrue(inNode.clone(), nodes.clone(), inEqualFunc.clone()), '__try1);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            Ok(inIndex.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nodes: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
            let mut index: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(inForbiddenColor.clone().borrow()[(inIndex.clone()-1) as usize].clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            nodes = __pa0.clone();
            List::getMemberOnTrue(inNode.clone(), nodes.clone(), inEqualFunc.clone())?;
            index = arrayFindMinColorIndex(inForbiddenColor.clone(), inNode.clone(), inIndex.clone() + 1, inmaxIndex.clone(), inEqualFunc.clone(), inPrintFunc.clone())?;
            Ok(index.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outColor)
}

pub fn printGraph<NodeType: Clone + 'static>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>) -> ArcStr {
    pub type NodeToString<NodeType: Clone> = fn(NodeType) -> Result<ArcStr>;

    let mut outString: ArcStr = arcstr::literal!("");
    outString = stringDelimitList(List::map1(inGraph.clone(), Arc::new(fnptr!(printNode, _, _)), inPrintFunc.clone()), (literal!("\n")).clone());
    outString
}

pub fn printNode<NodeType: Clone + 'static>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>), mut inPrintFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>) -> ArcStr {
    pub type NodeToString<NodeType: Clone> = fn(NodeType) -> Result<ArcStr>;

    let mut outString: ArcStr = arcstr::literal!("");
    let mut node: NodeType;
    let mut edges: Arc<metamodelica::List<NodeType>> = metamodelica::nil();
    let mut node_str: ArcStr = arcstr::literal!("");
    let mut edges_str: ArcStr = arcstr::literal!("");
    (node, edges) = inNode.clone();
    node_str = (inPrintFunc(node.clone()).unwrap()).clone();
    edges_str = stringDelimitList(List::map(edges.clone(), inPrintFunc.clone()), (literal!(", ")).clone());
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*node_str.clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*edges_str.clone()); ArcStr::from(__mm_s) }).clone();
    outString
}

/* Functions for Integer graphs */
pub fn printGraphInt(mut inGraph: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>) -> () {
    let _ = (::match_deref::match_deref! { match &(inGraph.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (node, edges), tail: restGraph } => {
            let mut strEdges: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node : ")); __mm_s.push_str(&*intString(node.clone())); __mm_s.push_str(&*literal!(" Edges: ")); ArcStr::from(__mm_s) }).clone());
            strEdges = List::map(edges.clone(), Arc::new(fnptr!(intString, i32)));
            strEdges = List::map1(strEdges.clone(), Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)), (literal!(" ")).clone());
            List::map_0(strEdges.clone(), Arc::new(fnptr!(metamodelica::print, ArcStr)));
            println!("{}", (literal!("\n")).clone());
            printGraphInt(restGraph.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn printNodesInt(mut inListNodes: Arc<metamodelica::List<i32>>, mut inName: ArcStr) -> () {
    let _ = (::match_deref::match_deref! { match &((inListNodes.clone(), inName.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            ()
        },
        (_, _) => {
            let mut strNodes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!(" : ")); ArcStr::from(__mm_s) }).clone());
            strNodes = List::map(inListNodes.clone(), Arc::new(fnptr!(intString, i32)));
            strNodes = List::map1(strNodes.clone(), Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)), (literal!(" ")).clone());
            List::map_0(strNodes.clone(), Arc::new(fnptr!(metamodelica::print, ArcStr)));
            println!("{}", (literal!("\n")).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn allReachableNodesInt(mut intmpstorage: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), mut inGraph: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>, mut inMaxGraphNode: i32, mut inMaxNodexIndex: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut reachableNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    reachableNodes = 'mc: {
        let __mc_input = (intmpstorage.clone(), inGraph.clone(), inMaxGraphNode.clone(), inMaxNodexIndex.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ metamodelica::List::Nil, L), _, _, _) => {
                    Ok(L.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ metamodelica::List::Cons { head: node, tail: M }, L), _, _, _) => {
                    let mut edges: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut M = (*M).clone();
                    let mut L = (*L).clone();
                    let mut reachableNodes: Arc<metamodelica::List<i32>> = reachableNodes.clone();
                    L = List::union(L.clone(), list![node.clone()]);
                    let false = (intGe(node.clone(), inMaxGraphNode.clone())) else { bail!("pattern mismatch") };
                    (_, edges) = inGraph.clone().borrow()[(node.clone()-1) as usize].clone();
                    edges = List::filter1OnTrue(edges.clone(), Arc::new(fnptr!(List::notMember, _, _)), L.clone());
                    M = List::union(M.clone(), edges.clone());
                    reachableNodes = allReachableNodesInt((M.clone(), L.clone()), inGraph.clone(), inMaxGraphNode.clone(), inMaxNodexIndex.clone())?;
                    Ok(reachableNodes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ metamodelica::List::Cons { head: node, tail: M }, L), _, _, _) => {
                    let mut L = (*L).clone();
                    let mut reachableNodes: Arc<metamodelica::List<i32>> = reachableNodes.clone();
                    L = List::union(L.clone(), list![node.clone()]);
                    let true = (intGe(node.clone(), inMaxGraphNode.clone())) else { bail!("pattern mismatch") };
                    reachableNodes = allReachableNodesInt((M.clone(), L.clone()), inGraph.clone(), inMaxGraphNode.clone(), inMaxNodexIndex.clone())?;
                    Ok(reachableNodes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.allReachableNodesInt failed.")).clone()], metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(reachableNodes)
}

pub fn partialDistance2colorInt(mut inGraphT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, mut inforbiddenColor: metamodelica::Array<i32>, mut inColors: Arc<metamodelica::List<i32>>, mut inGraph: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>, mut inColored: metamodelica::Array<i32>) -> Result<()> {
    let mut node: i32 = 0;
    let mut color: i32 = 0;
    let mut nodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut forbiddenColor: metamodelica::Array<i32>;
    let mut color: i32 = 0;
    let mut restGraph: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    if '__try0: {
        for mut tpl in &*inGraphT.clone() {
            let mut tpl = tpl.clone();
            (node, nodes) = tpl.clone();
            unwrap_break_err!(addForbiddenColorsInt(node.clone(), nodes.clone(), inColored.clone(), inforbiddenColor.clone(), inGraph.clone()), '__try0);
            color = unwrap_break_err!(arrayFindMinColorIndexInt(inforbiddenColor.clone(), node.clone()), '__try0);
            {let _arr = inColored.clone(); _arr.borrow_mut()[(node.clone()-1) as usize] = color.clone(); _arr};
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.partialDistance2colorInt failed.")).clone()], metamodelica::sourceInfo!())?;
    }
    Ok(())
}

fn addForbiddenColorsInt(mut inNode: i32, mut nodes: Arc<metamodelica::List<i32>>, mut inColored: metamodelica::Array<i32>, mut forbiddenColor: metamodelica::Array<i32>, mut inGraph: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>) -> Result<()> {
    let mut indexes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if '__try0: {
        for mut node in &*nodes.clone() {
            let mut node = node.clone();
            (_, indexes) = inGraph.clone().borrow()[(node.clone()-1) as usize].clone();
            unwrap_break_err!(updateForbiddenColorArrayInt(indexes.clone(), inColored.clone(), forbiddenColor.clone(), inNode.clone()), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.addForbiddenColorsInt failed.")).clone()], metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok(())
}

fn updateForbiddenColorArrayInt(mut inIndexes: Arc<metamodelica::List<i32>>, mut inColored: metamodelica::Array<i32>, mut inForbiddenColor: metamodelica::Array<i32>, mut inNode: i32) -> Result<()> {
    let mut colorIndex: i32 = 0;
    for mut index in &*inIndexes.clone() {
        let mut index = index.clone();
        colorIndex = inColored.clone().borrow()[(index.clone()-1) as usize].clone();
        if colorIndex.clone() > 0 {
            {let _arr = inForbiddenColor.clone(); _arr.borrow_mut()[(colorIndex.clone()-1) as usize] = inNode.clone(); _arr};
        }
    }
    Ok(())
}

fn arrayFindMinColorIndexInt(mut inForbiddenColor: metamodelica::Array<i32>, mut inNode: i32) -> Result<i32> {
    let mut outColor: i32 = 1;
    while true {
        if inForbiddenColor.clone().borrow()[(outColor.clone()-1) as usize].clone() != inNode.clone() {
            return Ok(outColor);
        } else {
            outColor = outColor.clone() + 1;
        }
    }
    Ok(outColor)
}

pub fn filterGraph<NodeType: Clone + 'static>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inCondFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>) -> Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> {
    pub type CondFunc<NodeType: Clone> = fn(NodeType) -> Result<bool>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    outGraph = List::accumulateMapAccum(inGraph.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = inCondFunc.clone(); move |__pe_a0, __pe_a2| filterGraph2(__pe_a0, __pe_b1.clone(), __pe_a2) }));
    outGraph
}

fn filterGraph2<NodeType: Clone + 'static>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>), mut inCondFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>, mut inAccumGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type CondFunc<NodeType: Clone> = fn(NodeType) -> Result<bool>;

    let mut outNode: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    outNode = 'mc: {
        let __mc_input = (inNode.clone(), inCondFunc.clone(), inAccumGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((node, _), _, _) => {
                    let false = (inCondFunc(node.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAccumGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((node, edges), _, _) => {
                    let mut edges = (*edges).clone();
                    edges = List::filterOnTrue(edges.clone(), inCondFunc.clone());
                    Ok(cons((node.clone(), edges.clone()), inAccumGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

pub fn merge<NodeType: Clone + 'static>(mut graph1: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut graph2: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut eqFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut compareFunc: Arc<dyn ::std::ops::Fn((NodeType, Arc<metamodelica::List<NodeType>>), (NodeType, Arc<metamodelica::List<NodeType>>)) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    pub type CompareFunc<NodeType: Clone> = fn((NodeType, Arc<metamodelica::List<NodeType>>), (NodeType, Arc<metamodelica::List<NodeType>>)) -> Result<bool>;

    let mut graph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    graph = merge2(List::sort(listAppend(graph1.clone(), graph2.clone()), compareFunc.clone())?, eqFunc.clone(), metamodelica::nil())?;
    Ok(graph)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn merge2<NodeType: Clone + 'static>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut eqFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inAcc: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut graph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    graph = (::match_deref::match_deref! { match &((inGraph.clone(), eqFunc.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            inAcc.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: node, tail: Deref @ metamodelica::List::Nil }, _, _) => {
            cons(node.clone(), inAcc.clone()).reverse()
        },
        (Deref @ metamodelica::List::Cons { head: (n1, e1), tail: Deref @ metamodelica::List::Cons { head: (n2, e2), tail: rest } }, _, _) => {
            let mut node: (NodeType, Arc<metamodelica::List<NodeType>>);
            let mut b: bool = false;
            let mut rest = (*rest).clone();
            b = eqFunc(n1.clone(), n2.clone())?;
            (node, rest) = merge3(b.clone(), n1.clone(), e1.clone(), n2.clone(), e2.clone(), rest.clone(), eqFunc.clone())?;
            merge2(rest.clone(), eqFunc.clone(), cons(node.clone(), inAcc.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(graph)
}

fn merge3<NodeType: Clone + 'static>(mut b: bool, mut n1: NodeType, mut e1: Arc<metamodelica::List<NodeType>>, mut n2: NodeType, mut e2: Arc<metamodelica::List<NodeType>>, mut rest: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut eqFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<((NodeType, Arc<metamodelica::List<NodeType>>), Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>)> {
    pub type EqualFunc<NodeType: Clone> = fn(NodeType, NodeType) -> Result<bool>;

    let mut elt: (NodeType, Arc<metamodelica::List<NodeType>>);
    let mut outRest: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>> = metamodelica::nil();
    (elt, outRest) = (::match_deref::match_deref! { match &((b.clone(), n1.clone(), e1.clone(), n2.clone(), e2.clone(), rest.clone(), eqFunc.clone())) {
        (true, _, _, _, _, _, _) => ((n1.clone(), List::unionOnTrue(e1.clone(), e2.clone(), eqFunc.clone())), rest.clone()),
        (false, _, _, _, _, _, _) => ((n1.clone(), e1.clone()), cons((n2.clone(), e2.clone()), rest.clone())),
        _ => bail!("match: no arm matched"),
    } });
    Ok((elt, outRest))
}

