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

use crate::NFBinding as Binding;
use crate::NFBuiltin;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::DisjointSets;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

/// a tuple with two crefs and equation(s) for calling the equalityConstraint function call
pub type FlatEdge = NFConnections::BrokenEdge;

/// a lit of broken edges
pub type FlatEdges = Arc<metamodelica::List<NFConnections::BrokenEdge>>;

/// an edge is a tuple with two component references
pub type Edge = (Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>);

/// A list of edges
pub type Edges = Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>;

/// root defined with Connection.root
pub type DefiniteRoot = Arc<ComponentRef::NFComponentRef>;

/// roots defined with Connection.root
pub type DefiniteRoots = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

/// roots defined with Connection.uniqueRoot
pub type UniqueRoots = Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)>>;

/// potential root defined with Connections.potentialRoot
pub type PotentialRoot = (Arc<ComponentRef::NFComponentRef>, metamodelica::Real);

/// potential roots defined with Connections.potentialRoot
pub type PotentialRoots = Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, metamodelica::Real)>>;

/// Input structure for connection breaking algorithm. It is collected during instantiation phase.
#[derive(Clone, Debug, PartialEq)]
pub struct NFOCConnectionGraph {
    pub updateGraph: bool,
    /// Roots defined with Connection.root
    pub definiteRoots: DefiniteRoots,
    /// Roots defined with Connection.potentialRoot
    pub potentialRoots: PotentialRoots,
    /// Roots defined with Connection.uniqueRoot
    pub uniqueRoots: UniqueRoots,
    /// Edges defined with Connection.branch
    pub branches: Edges,
    /// Edges defined with connect statement
    pub connections: FlatEdges,
}

impl Default for NFOCConnectionGraph {
    fn default() -> Self {
        Self {
            updateGraph: Default::default(),
            definiteRoots: Default::default(),
            potentialRoots: Default::default(),
            uniqueRoots: Default::default(),
            branches: Default::default(),
            connections: Default::default(),
        }
    }
}

pub type GRAPH = NFOCConnectionGraph;


// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: NFOCConnectionGraph
// Expr: Constructor { name: 'NFOCConnectionGraph.NFOCConnectionGraph.GRAPH', args: [Lit(Bool(true)), Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }], named_args: [], ty: RustStruct('NFOCConnectionGraph.NFOCConnectionGraph'), field_names: ['updateGraph', 'definiteRoots', 'potentialRoots', 'uniqueRoots', 'branches', 'connections'] }
pub fn EMPTY() -> NFOCConnectionGraph { todo!("non-Sync, non-const-emittable constant EMPTY — extend codegen") }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ConnectionsOperator {
    BRANCH = 1,
    ROOT = 2,
    POTENTIAL_ROOT = 3,
    IS_ROOT = 4,
    ROOTED = 5,
    UNIQUE_ROOT = 6,
    UNIQUE_ROOT_INDICES = 7,
    NOT_OPERATOR = 8,
}
impl PartialOrd for ConnectionsOperator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ConnectionsOperator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub type CrefCrefTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;

pub type CrefIndexTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;

pub type CrefRootsTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>;

pub mod CrefSets {
    use super::*;
    pub fn EntryHash(mut entry: Entry) -> i32 {
        let mut hash: i32 = 0;
        hash = ComponentRef::hash(entry.clone());
        hash
    }

    pub fn EntryEqual(mut entry1: Entry, mut entry2: Entry) -> Result<bool> {
        let mut isEqual: bool = false;
        isEqual = ComponentRef::isEqual(entry1.clone(), entry2.clone())?;
        Ok(isEqual)
    }

    pub fn EntryString(mut entry: Entry) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (ComponentRef::toString(entry.clone())?).clone();
        Ok(r#str)
    }

    pub type Entry = Arc<ComponentRef::NFComponentRef>;

    pub type IndexTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;

    /// This is a disjoint sets data structure. The nodes are stored in an array of
    ///   Integers. The root elements of a set is given a negative value that
    ///   corresponds to its rank, while other elements are given positive values that
    ///   corresponds to the index of their parent in the array. The hashtable is used
    ///   to look up the array index of a entry, and is also used to store the entries.
#[derive(Clone, Debug, PartialEq)]
    pub struct Sets {
        /// An array of nodes
        pub nodes: metamodelica::Array<i32>,
        /// An Entry->Integer table.
        pub elements: IndexTable,
        /// The number of nodes stored in the sets.
        pub nodeCount: i32,
    }

    impl Default for Sets {
        fn default() -> Self {
            Self {
                nodes: Default::default(),
                elements: Default::default(),
                nodeCount: Default::default(),
            }
        }
    }

    pub type DISJOINT_SETS = Sets;


    pub fn add(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
        let mut sets: Sets = sets;
        let mut index: i32 = 0;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut node_count: i32 = 0;
        let Sets { nodes: __pa0, elements: __pa1, nodeCount: __pa2 } = (sets.clone()) else { bail!("pattern mismatch") };
        nodes = __pa0.clone();
        elements = __pa1.clone();
        node_count = __pa2.clone();
        index = node_count.clone() + 1;
        if index.clone() > (nodes.clone().borrow().len() as i32) {
            nodes = Array::expand(((intReal(index.clone()) * metamodelica::OrderedFloat(1.4_f64)).0 as i32), nodes.clone(), -1)?;
        }
        UnorderedMap::addNew(entry.clone(), index.clone(), elements.clone())?;
        sets = Sets { nodes: nodes.clone(), elements: elements.clone(), nodeCount: index.clone() };
        Ok((sets, index))
    }

    pub fn addList(mut entries: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut node_count: i32 = 0;
        let mut sz: i32 = 0;
        let mut index: i32 = 0;
        let Sets { nodes: __pa0, elements: __pa1, nodeCount: __pa2 } = (sets.clone()) else { bail!("pattern mismatch") };
        nodes = __pa0.clone();
        elements = __pa1.clone();
        node_count = __pa2.clone();
        sz = (entries.clone().len() as i32);
        index = node_count.clone() + 1;
        node_count = node_count.clone() + sz.clone();
        if node_count.clone() > (nodes.clone().borrow().len() as i32) {
            nodes = Array::expand(((intReal(node_count.clone()) * metamodelica::OrderedFloat(1.4_f64)).0 as i32), nodes.clone(), -1)?;
        }
        for mut e in &*entries.clone() {
            let mut e = e.clone();
            UnorderedMap::addNew(e.clone(), index.clone(), elements.clone())?;
            index = index.clone() + 1;
        }
        sets = Sets { nodes: nodes.clone(), elements: elements.clone(), nodeCount: node_count.clone() };
        Ok(sets)
    }

    pub fn contains(mut entry: Entry, mut sets: Sets) -> bool {
        let mut found: bool = false;
        found = isSome(UnorderedMap::get(entry.clone(), sets.elements.clone()));
        found
    }

    pub fn emptySets(mut setCount: i32) -> Sets {
        let mut sets: Sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut sz: i32 = 0;
        sz = std::cmp::max(setCount.clone(), 3);
        nodes = arrayCreate(sz.clone(), -1);
        elements = UnorderedMap::new(fnptr!(EntryHash, Arc<ComponentRef::NFComponentRef>), EntryEqual, 1);
        sets = Sets { nodes: nodes.clone(), elements: elements.clone(), nodeCount: 0 };
        sets
    }

    pub fn extractSets(mut sets: Sets) -> (metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>, Sets) {
        let mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut assignedSets: Sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut set_idx: i32 = 0;
        let mut idx: i32 = 0;
        let mut entries: metamodelica::Array<(Arc<ComponentRef::NFComponentRef>, i32)>;
        let mut e: Entry = Arc::new(ComponentRef::EMPTY);
        nodes = sets.nodes.clone();
        for mut i in 1..=sets.nodeCount.clone() {
            if nodes.borrow()[(i.clone()-1) as usize].clone() < 0 {
                set_idx = set_idx.clone() + 1;
                {
                    let __cell0 = -(set_idx.clone());
                    nodes.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
                }
            }
        }
        setsArray = arrayCreate(set_idx.clone(), metamodelica::nil());
        entries = UnorderedMap::toArray(sets.elements.clone());
        let __range1 = (1..=(entries.clone().borrow().len() as i32)).rev();
        for mut i in __range1 {
            (e, idx) = entries.clone().borrow()[(i.clone()-1) as usize].clone();
            set_idx = nodes.borrow()[(idx.clone()-1) as usize].clone();
            while set_idx.clone() > 0 {
                set_idx = nodes.borrow()[(set_idx.clone()-1) as usize].clone();
            }
            set_idx = -(set_idx.clone());
            {
                let __cell2 = cons(e.clone(), setsArray.borrow()[(set_idx.clone()-1) as usize].clone());
                setsArray.clone().borrow_mut()[(set_idx.clone()-1) as usize] = __cell2;
            }
        }
        assignedSets = Sets { nodes: nodes.clone(), elements: sets.elements.clone(), nodeCount: sets.nodeCount.clone() };
        (setsArray, assignedSets)
    }

    pub fn find(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
        let mut sets: Sets = sets;
        let mut index: i32 = 0;
        let mut oindex: Option<i32> = None;
        oindex = UnorderedMap::get(entry.clone(), sets.elements.clone());
        if isSome(oindex.clone()) {
            let Some(__pa0) = (oindex.clone()) else { bail!("pattern mismatch") };
            index = __pa0.clone();
        } else {
            (sets, index) = add(entry.clone(), sets.clone())?;
        }
        Ok((sets, index))
    }

    pub fn findRoot(mut nodeIndex: i32, mut nodes: metamodelica::Array<i32>) -> Result<i32> {
        let mut rootIndex: i32 = nodeIndex.clone();
        let mut parent: i32 = nodes.borrow()[(nodeIndex.clone()-1) as usize].clone();
        let mut idx: i32 = nodeIndex.clone();
        while parent.clone() > 0 {
            rootIndex = parent.clone();
            parent = nodes.borrow()[(parent.clone()-1) as usize].clone();
        }
        parent = nodes.borrow()[(nodeIndex.clone()-1) as usize].clone();
        while parent.clone() > 0 {
            {let _arr = nodes.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = rootIndex.clone(); _arr};
            idx = parent.clone();
            parent = nodes.borrow()[(parent.clone()-1) as usize].clone();
        }
        Ok(rootIndex)
    }

    pub fn findSet(mut entry: Entry, mut sets: Sets) -> Result<(i32, Sets)> {
        let mut set: i32 = 0;
        let mut updatedSets: Sets;
        let mut index: i32 = 0;
        (updatedSets, index) = find(entry.clone(), sets.clone())?;
        set = findRoot(index.clone(), updatedSets.nodes.clone())?;
        Ok((set, updatedSets))
    }

    pub fn findSetArrayIndex(mut entry: Entry, mut sets: Sets) -> i32 {
        let mut set: i32 = 0;
        set = UnorderedMap::getOrFail(entry.clone(), sets.elements.clone());
        while set.clone() > 0 {
            set = sets.nodes.borrow()[(set.clone()-1) as usize].clone();
        }
        set = -(set.clone());
        set
    }

    pub fn getEntry(mut entry: Entry, mut sets: Sets) -> Option<Arc<ComponentRef::NFComponentRef>> {
        let mut outEntry: Option<Arc<ComponentRef::NFComponentRef>> = None;
        outEntry = UnorderedMap::getKey(entry.clone(), sets.elements.clone());
        outEntry
    }

    pub fn getNodeCount(mut sets: Sets) -> i32 {
        let mut nodeCount: i32 = sets.nodeCount.clone();
        nodeCount
    }

    pub fn merge(mut entry1: Entry, mut entry2: Entry, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut set1: i32 = 0;
        let mut set2: i32 = 0;
        (set1, sets) = findSet(entry1.clone(), sets.clone())?;
        (set2, sets) = findSet(entry2.clone(), sets.clone())?;
        sets = union(set1.clone(), set2.clone(), sets.clone())?;
        Ok(sets)
    }

    pub fn printSets(mut sets: Sets) -> Result<()> {
        let mut nodes: metamodelica::Array<i32>;
        let mut entries: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>> = metamodelica::nil();
        let mut e: Entry = Arc::new(ComponentRef::EMPTY);
        let mut i: i32 = 0;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(sets.nodeCount.clone())); __mm_s.push_str(&*literal!(" sets:\n")); ArcStr::from(__mm_s) }).clone());
        nodes = sets.nodes.clone();
        entries = UnorderedMap::toList(sets.elements.clone());
        for mut p in &*entries.clone() {
            let mut p = p.clone();
            (e, i) = p.clone();
            println!("{}", (literal!("[")).clone());
            println!("{}", ArcStr::from(::std::format!("{}", i.clone())));
            println!("{}", (literal!("]")).clone());
            println!("{}", (EntryString(e.clone())?).clone());
            println!("{}", (literal!(" -> ")).clone());
            println!("{}", ArcStr::from(::std::format!("{}", nodes.borrow()[(i.clone()-1) as usize].clone())));
            println!("{}", (literal!("\n")).clone());
        }
        Ok(())
    }

    pub fn union(mut set1: i32, mut set2: i32, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut rank1: i32 = 0;
        let mut rank2: i32 = 0;
        if set1.clone() != set2.clone() {
            rank1 = sets.nodes.borrow()[(set1.clone()-1) as usize].clone();
            rank2 = sets.nodes.borrow()[(set2.clone()-1) as usize].clone();
            if rank1.clone() > rank2.clone() {
                {let _arr = sets.nodes.clone(); _arr.borrow_mut()[(set2.clone()-1) as usize] = set1.clone(); _arr};
            } else if rank1.clone() < rank2.clone() {
                {let _arr = sets.nodes.clone(); _arr.borrow_mut()[(set1.clone()-1) as usize] = set2.clone(); _arr};
            } else {
                {let _arr = sets.nodes.clone(); let _val = sets.nodes.borrow()[(set1.clone()-1) as usize].clone() - 1; _arr.borrow_mut()[(set1.clone()-1) as usize] = _val; _arr};
                {let _arr = sets.nodes.clone(); _arr.borrow_mut()[(set2.clone()-1) as usize] = set1.clone(); _arr};
            }
        }
        Ok(sets)
    }

}

pub type IsDeletedFn = fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool>;

pub fn handleOverconstrainedConnections(mut flatModel: Arc<FlatModel::NFFlatModel>, mut conns: Arc<NFConnections::NFConnections>, mut isDeleted: IsDeletedFn) -> Result<(Arc<FlatModel::NFFlatModel>, FlatEdges)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut broken: FlatEdges = metamodelica::nil();
    let mut graph: NFOCConnectionGraph = EMPTY().clone();
    let mut connected: FlatEdges = metamodelica::nil();
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut print_trace: bool = Flags::isSet(Flags::CGRAPH.clone())?;
    graph = addBreakableBranches(conns.connections.clone(), isDeleted, print_trace.clone(), graph.clone())?;
    (eql, graph) = addRootsAndBranches(flatModel.equations.clone(), print_trace.clone(), graph.clone())?;
    assign_field!(flatModel.equations = eql.clone());
    (flatModel, connected, broken) = handleOverconstrainedConnections_dispatch(graph.clone(), flatModel.clone())?;
    assign_field!(flatModel.equations = removeBrokenConnects(flatModel.equations.clone(), connected.clone(), broken.clone(), isDeleted)?);
    Ok((flatModel, broken))
}

fn addBreakableBranches(mut connections: Arc<metamodelica::List<Arc<Connection::NFConnection>>>, mut isDeleted: IsDeletedFn, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    let mut breakable: CrefSets::Sets;
    let mut c1: Arc<Connector::NFConnector>;
    let mut c2: Arc<Connector::NFConnector>;
    let mut lhs_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut rhs_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut lhs_set: i32 = 0;
    let mut rhs_set: i32 = 0;
    breakable = CrefSets::emptySets(3);
    for mut conn in &*connections.clone() {
        let mut conn = conn.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(conn.clone()) {
            Deref @ Connection::CONNECTION { rhs: __pa0, lhs: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        c2 = __pa0.clone();
        c1 = __pa1.clone();
        lhs_crefs = getOverconstrainedCrefs(c1.clone(), isDeleted)?;
        rhs_crefs = getOverconstrainedCrefs(c2.clone(), isDeleted)?;
        for mut lhs in &*lhs_crefs.clone() {
            let mut lhs = lhs.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rhs_crefs.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rhs = __pa2.clone();
            rhs_crefs = __pa3.clone();
            (lhs_set, breakable) = CrefSets::findSet(lhs.clone(), breakable.clone())?;
            (rhs_set, breakable) = CrefSets::findSet(rhs.clone(), breakable.clone())?;
            if lhs_set.clone() != rhs_set.clone() {
                graph = addConnection(lhs.clone(), rhs.clone(), c1.source.clone(), printTrace.clone(), graph.clone())?;
                breakable = CrefSets::union(lhs_set.clone(), rhs_set.clone(), breakable.clone())?;
            }
        }
    }
    Ok(graph)
}

fn addRootsAndBranches(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, NFOCConnectionGraph)> {
    let mut outEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut graph: NFOCConnectionGraph = graph;
    let mut call: Arc<Call::NFCall>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut root: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut msg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut priority: i32 = 0;
    for mut eq in &*equations.clone() {
        let mut eq = eq.clone();
        outEquations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: args, .. } }, .. } => (match identifyConnectionsOperator(Function::name(call.r#fn.clone())) {
        ConnectionsOperator::ROOT => {
            let __pa0 = ::match_deref::match_deref! { match &(args.clone()) {
                Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cref = __pa0.clone();
            graph = addDefiniteRoot(cref.clone(), printTrace.clone(), graph.clone())?;
            outEquations.clone()
        },
        ConnectionsOperator::POTENTIAL_ROOT => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg1 = __pa0.clone();
            arg2 = __pa1.clone();
            let __pa3 = ::match_deref::match_deref! { match &(arg1.clone()) {
                Deref @ Expression::CREF { cref: __pa3, .. } => __pa3.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cref = __pa3.clone();
            let __pa4 = ::match_deref::match_deref! { match &(Ceval::evalExp(arg2.clone(), Ceval::noTarget().clone())?) {
                Deref @ Expression::INTEGER { value: __pa4 } => __pa4.clone(),
                _ => bail!("pattern mismatch"),
            } };
            priority = __pa4.clone();
            graph = addPotentialRoot(cref.clone(), metamodelica::OrderedFloat((priority.clone()) as f64), printTrace.clone(), graph.clone())?;
            outEquations.clone()
        },
        ConnectionsOperator::UNIQUE_ROOT => {
            graph = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: root @ Deref @ Expression::CREF { cref, .. }, tail: Deref @ metamodelica::List::Nil } => addUniqueRoots(root.clone(), Arc::new(Expression::NFExpression::STRING { value: (literal!("")).clone() }), printTrace.clone(), graph.clone())?,
        Deref @ metamodelica::List::Cons { head: root @ Deref @ Expression::CREF { cref, .. }, tail: Deref @ metamodelica::List::Cons { head: msg, tail: Deref @ metamodelica::List::Nil } } => addUniqueRoots(root.clone(), msg.clone(), printTrace.clone(), graph.clone())?,
        _ => bail!("match: no arm matched"),
    } });
            outEquations.clone()
        },
        ConnectionsOperator::BRANCH { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa1, .. }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            lhs = __pa0.clone();
            rhs = __pa1.clone();
            graph = addBranch(lhs.clone(), rhs.clone(), printTrace.clone(), graph.clone())?;
            outEquations.clone()
        },
        _ => cons(eq.clone(), outEquations.clone()),
    }),
        _ => cons(eq.clone(), outEquations.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outEquations = outEquations.clone().reverse();
    Ok((outEquations, graph))
}

fn generateEqualityConstraintEquation(mut lhs: Arc<ComponentRef::NFComponentRef>, mut rhs: Arc<ComponentRef::NFComponentRef>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut equalityConstraintEq: Arc<Equation::NFEquation>;
    let mut context: i32 = 0;
    let mut fcref_rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut fcref_lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut fn_node_rhs: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut fn_node_lhs: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exp_rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp_lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut info: SourceInfo = ElementSource::getInfo(source.clone());
    context = intBitOr(InstContext::EQUATION.clone(), InstContext::CONNECT.clone());
    fcref_rhs = Function::lookupFunctionSimple((literal!("equalityConstraint")).clone(), InstNode::classScope(ComponentRef::node(lhs.clone())?), context.clone())?;
    (fcref_rhs, fn_node_rhs, _) = Function::instFunctionRef(fcref_rhs.clone(), context.clone(), Absyn::dummyInfo.clone())?;
    exp_rhs = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::UNTYPED_CALL { r#ref: fcref_rhs.clone(), arguments: list![Expression::fromCref(lhs.clone(), false)?, Expression::fromCref(rhs.clone(), false)?], named_args: metamodelica::nil(), call_scope: fn_node_rhs.clone() }) });
    (exp_rhs, ty, _, _) = Typing::typeExp(exp_rhs.clone(), context.clone(), info.clone(), false)?;
    fcref_lhs = Function::lookupFunctionSimple((literal!("fill")).clone(), InstNode::topScope(ComponentRef::node(lhs.clone())?), context.clone())?;
    (fcref_lhs, fn_node_lhs, _) = Function::instFunctionRef(fcref_lhs.clone(), context.clone(), Absyn::dummyInfo.clone())?;
    exp_lhs = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::UNTYPED_CALL { r#ref: fcref_lhs.clone(), arguments: cons(Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }), {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut d in (Type::arrayDims(ty.clone())).into_iter().cloned() {
            let __x = Dimension::sizeExp(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), named_args: metamodelica::nil(), call_scope: fn_node_lhs.clone() }) });
    (exp_lhs, ty, _, _) = Typing::typeExp(exp_lhs.clone(), context.clone(), info.clone(), false)?;
    equalityConstraintEq = Equation::makeEquality(exp_rhs.clone(), exp_lhs.clone(), ty.clone(), source.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone());
    Ok(equalityConstraintEq)
}

fn getOverconstrainedCrefs(mut conn: Arc<Connector::NFConnector>, mut isDeleted: IsDeletedFn) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut conns: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    conns = Connector::split(conn.clone())?;
    conns = List::mapFlat(conns.clone(), Arc::new(Connector::scalarizePrefix));
    crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut c in (conns.clone()).into_iter().cloned() {
            if !(!(isDeleted(c.name.clone())?) && isOverconstrainedCref(c.name.clone())?) { continue; }
            let __x = getOverconstrainedCref(c.name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    crefs = List::uniqueOnTrue(crefs.clone(), Arc::new(ComponentRef::isEqual));
    Ok(crefs)
}

fn isOverconstrainedCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut rest: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    b = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { restCref: rest, origin: ComponentRef::Origin::CREF, node, .. } => Class::isOverdetermined(InstNode::getClass(node.clone())?) || isOverconstrainedCref(rest.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getOverconstrainedCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut c: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut rest: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    c = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { restCref: rest, origin: ComponentRef::Origin::CREF, node, .. } => if (Class::isOverdetermined(InstNode::getClass(node.clone())?)) {cref.clone()} else {getOverconstrainedCref(rest.clone())?},
        _ => bail!("match: no arm matched"),
    } });
    Ok(c)
}

fn handleOverconstrainedConnections_dispatch(mut graph: NFOCConnectionGraph, mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<(Arc<FlatModel::NFFlatModel>, FlatEdges, FlatEdges)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut connected: FlatEdges = metamodelica::nil();
    let mut broken: FlatEdges = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ieqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut rooted: CrefIndexTable;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Summary:\n\t")); __mm_s.push_str(&*literal!("Nr Roots:           ")); __mm_s.push_str(&*intString((getDefiniteRoots(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Potential Roots: ")); __mm_s.push_str(&*intString((getPotentialRoots(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Unique Roots:    ")); __mm_s.push_str(&*intString((getUniqueRoots(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Branches:        ")); __mm_s.push_str(&*intString((getBranches(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Connections:     ")); __mm_s.push_str(&*intString((getConnections(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        (roots, connected, broken) = unwrap_break_err!(findResultGraph(graph.clone(), (FlatModel::fullName(flatModel.clone())).clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Roots: ")); __mm_s.push_str(&*stringDelimitList(List::map(roots.clone(), Arc::new(ComponentRef::toString)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Broken connections: ")); __mm_s.push_str(&*stringDelimitList(List::map1(broken.clone(), Arc::new(printConnectionStr), (literal!("broken")).clone()), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowed connections: ")); __mm_s.push_str(&*stringDelimitList(List::map1(connected.clone(), Arc::new(printConnectionStr), (literal!("allowed")).clone()), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        rooted = unwrap_break_err!(buildRootedTable(roots.clone(), graph.clone()), '__try0);
        assign_field!(
            flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = evalConnectionsOperatorsVar(roots.clone(), rooted.clone(), graph.clone(), v.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
            flatModel.equations = evalConnectionsOperatorsEqs(roots.clone(), rooted.clone(), graph.clone(), flatModel.equations.clone()),
            flatModel.initialEquations = evalConnectionsOperatorsEqs(roots.clone(), rooted.clone(), graph.clone(), flatModel.initialEquations.clone())
        );
        Ok::<_, anyhow::Error>((broken.clone(), connected.clone(), flatModel.clone(), rooted.clone(), roots.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            broken = __try0_o0;
            connected = __try0_o1;
            flatModel = __try0_o2;
            rooted = __try0_o3;
            roots = __try0_o4;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::CGRAPH.clone())?) else { bail!("pattern mismatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.handleOverconstrainedConnections failed for model: ")); __mm_s.push_str(&*FlatModel::fullName(flatModel.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail");
        }
    }
    Ok((flatModel, connected, broken))
}

fn addDefiniteRoot(mut root: Arc<ComponentRef::NFComponentRef>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addDefiniteRoot(")); __mm_s.push_str(&*ComponentRef::toString(root.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    graph.definiteRoots = cons(root.clone(), graph.definiteRoots.clone());
    Ok(graph)
}

fn addPotentialRoot(mut root: Arc<ComponentRef::NFComponentRef>, mut priority: metamodelica::Real, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addPotentialRoot(")); __mm_s.push_str(&*ComponentRef::toString(root.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    graph.potentialRoots = cons((root.clone(), priority.clone()), graph.potentialRoots.clone());
    Ok(graph)
}

fn addUniqueRoots(mut roots: Arc<Expression::NFExpression>, mut message: Arc<Expression::NFExpression>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    let mut unique_roots: UniqueRoots = graph.uniqueRoots.clone();
    for mut root in &*Expression::arrayScalarElements(roots.clone()) {
        let mut root = root.clone();
        unique_roots = (::match_deref::match_deref! { match &(root.clone()) {
        Deref @ Expression::CREF { .. } => {
            if printTrace.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addUniqueRoots(")); __mm_s.push_str(&*Expression::toString(root.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(message.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
            }
            cons((var_field!((*root).cref, Expression::NFExpression::CREF).clone(), message.clone()), unique_roots.clone())
        },
        _ => unique_roots.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(graph)
}

fn addBranch(mut ref1: Arc<ComponentRef::NFComponentRef>, mut ref2: Arc<ComponentRef::NFComponentRef>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addBranch(")); __mm_s.push_str(&*ComponentRef::toString(ref1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(ref2.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    graph.branches = cons((ref1.clone(), ref2.clone()), graph.branches.clone());
    Ok(graph)
}

fn addConnection(mut ref1: Arc<ComponentRef::NFComponentRef>, mut ref2: Arc<ComponentRef::NFComponentRef>, mut source: Arc<DAE::ElementSource>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addConnection(")); __mm_s.push_str(&*ComponentRef::toString(ref1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(ref2.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    graph.connections = cons(Arc::new(NFConnections::BrokenEdge { lhs: ref1.clone(), rhs: ref2.clone(), source: source.clone(), brokenEquations: metamodelica::nil() }), graph.connections.clone());
    Ok(graph)
}

// ************************************* //
// ********* protected section ********* //
// ************************************* //
fn canonical(mut inPartition: CrefCrefTable, mut inRef: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    let mut outCanonical: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cref_opt: Option<Arc<ComponentRef::NFComponentRef>> = None;
    let mut parent: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut parentCanonical: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    cref_opt = UnorderedMap::get(inRef.clone(), inPartition.clone());
    outCanonical = (::match_deref::match_deref! { match &(cref_opt.clone()) {
        Some(outCanonical) => canonical(inPartition.clone(), outCanonical.clone()),
        _ => inRef.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCanonical
}

fn areInSameComponent(mut partition: CrefCrefTable, mut ref1: Arc<ComponentRef::NFComponentRef>, mut ref2: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut outResult: bool = false;
    outResult = ComponentRef::isEqual(canonical(partition.clone(), ref1.clone()), canonical(partition.clone(), ref2.clone()))?;
    Ok(outResult)
}

fn connectBranchComponents(mut partition: CrefCrefTable, mut ref1: Arc<ComponentRef::NFComponentRef>, mut ref2: Arc<ComponentRef::NFComponentRef>) -> Result<()> {
    connectCanonicalComponents(partition.clone(), canonical(partition.clone(), ref1.clone()), canonical(partition.clone(), ref2.clone()))?;
    Ok(())
}

fn connectComponents(mut partition: CrefCrefTable, mut edge: FlatEdge) -> (FlatEdges, FlatEdges) {
    let mut outConnectedConnections: FlatEdges = metamodelica::nil();
    let mut outBrokenConnections: FlatEdges = metamodelica::nil();
    let mut canon1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut canon2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut eq: Arc<Equation::NFEquation>;
    match '__try0: {
        canon1 = canonical(partition.clone(), edge.lhs.clone());
        canon2 = canonical(partition.clone(), edge.rhs.clone());
        let false = (unwrap_break_err!(connectCanonicalComponents(partition.clone(), canon1.clone(), canon2.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            unwrap_break_err!(Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.connectComponents: should remove equations generated from: connect(")); __mm_s.push_str(&*ComponentRef::toString(edge.lhs.clone()).unwrap()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(edge.rhs.clone()).unwrap()); __mm_s.push_str(&*literal!(") and add {0, ..., 0} = equalityConstraint(cr1, cr2) instead.\n")); ArcStr::from(__mm_s) }).clone()), '__try0);
        }
        outConnectedConnections = metamodelica::nil();
        eq = unwrap_break_err!(generateEqualityConstraintEquation(edge.lhs.clone(), edge.rhs.clone(), edge.source.clone()), '__try0);
        outBrokenConnections = list![Arc::new(NFConnections::BrokenEdge { lhs: edge.lhs.clone(), rhs: edge.rhs.clone(), source: edge.source.clone(), brokenEquations: list![eq.clone()] })];
        Ok::<_, anyhow::Error>((outBrokenConnections.clone(), outConnectedConnections.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outBrokenConnections = __try0_o0;
            outConnectedConnections = __try0_o1;
        }
        Err(_) => {
            outConnectedConnections = list![edge.clone()];
            outBrokenConnections = metamodelica::nil();
        }
    }
    (outConnectedConnections, outBrokenConnections)
}

fn connectCanonicalComponents(mut inPartition: CrefCrefTable, mut inRef1: Arc<ComponentRef::NFComponentRef>, mut inRef2: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut outReallyConnected: bool = false;
    outReallyConnected = !(ComponentRef::isEqual(inRef1.clone(), inRef2.clone())?);
    if outReallyConnected.clone() {
        UnorderedMap::add(inRef1.clone(), inRef2.clone(), inPartition.clone())?;
    }
    Ok(outReallyConnected)
}

fn addRootsToTable(mut table: CrefCrefTable, mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut firstRoot: Arc<ComponentRef::NFComponentRef>) -> Result<()> {
    let mut root: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut rest_roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    for mut root in &*roots.clone() {
        let mut root = root.clone();
        UnorderedMap::add(root.clone(), firstRoot.clone(), table.clone())?;
    }
    Ok(())
}

fn resultGraphWithRoots(mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<CrefCrefTable> {
    let mut outTable: CrefCrefTable;
    let mut dummyRoot: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    dummyRoot = NFBuiltin::TIME_CREF().clone();
    outTable = newCrefCrefTable();
    addRootsToTable(outTable.clone(), roots.clone(), dummyRoot.clone())?;
    Ok(outTable)
}

fn addBranchesToTable(mut table: CrefCrefTable, mut branches: Edges) -> Result<()> {
    let mut ref1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ref2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        (ref1, ref2) = branch.clone();
        connectBranchComponents(table.clone(), ref1.clone(), ref2.clone())?;
    }
    Ok(())
}

fn ord(mut inEl1: PotentialRoot, mut inEl2: PotentialRoot) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (inEl1.clone(), inEl2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((c1, r1), (c2, r2)) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let true = (realEq(r1.clone(), r2.clone())) else { bail!("pattern mismatch") };
                    s1 = (ComponentRef::toString(c1.clone())?).clone();
                    s2 = (ComponentRef::toString(c2.clone())?).clone();
                    let 1 = (stringCompare((s1.clone()).clone(), (s2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((_, r1), (_, r2)) => {
                    Ok(r1.clone() > r2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBoolean)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addPotentialRootsToTable(mut table: CrefCrefTable, mut potentialRoots: PotentialRoots, mut roots: DefiniteRoots, mut firstRoot: Arc<ComponentRef::NFComponentRef>) -> Result<DefiniteRoots> {
    let mut outRoots: DefiniteRoots = metamodelica::nil();
    outRoots = 'mc: {
        let __mc_input = potentialRoots.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(roots.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (potentialRoot, _), tail: tail } => {
                    let mut canon1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
                    let mut canon2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
                    let mut finalRoots: DefiniteRoots = metamodelica::nil();
                    canon1 = canonical(table.clone(), potentialRoot.clone());
                    canon2 = canonical(table.clone(), firstRoot.clone());
                    let true = (connectCanonicalComponents(table.clone(), canon1.clone(), canon2.clone())?) else { bail!("pattern mismatch") };
                    finalRoots = addPotentialRootsToTable(table.clone(), tail.clone(), cons(potentialRoot.clone(), roots.clone()), firstRoot.clone())?;
                    Ok(finalRoots.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: tail } => {
                    let mut finalRoots: DefiniteRoots = metamodelica::nil();
                    finalRoots = addPotentialRootsToTable(table.clone(), tail.clone(), roots.clone(), firstRoot.clone())?;
                    Ok(finalRoots.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRoots)
}

fn addConnections(mut table: CrefCrefTable, mut inConnections: FlatEdges) -> (FlatEdges, FlatEdges) {
    let mut outConnectedConnections: FlatEdges = metamodelica::nil();
    let mut outBrokenConnections: FlatEdges = metamodelica::nil();
    let mut connected: FlatEdges = metamodelica::nil();
    let mut broken: FlatEdges = metamodelica::nil();
    for mut c in &*inConnections.clone() {
        let mut c = c.clone();
        (connected, broken) = connectComponents(table.clone(), c.clone());
        outConnectedConnections = listAppend(connected.clone(), outConnectedConnections.clone());
        outBrokenConnections = listAppend(broken.clone(), outBrokenConnections.clone());
    }
    (outConnectedConnections, outBrokenConnections)
}

fn findResultGraph(mut inGraph: NFOCConnectionGraph, mut modelNameQualified: ArcStr) -> Result<(DefiniteRoots, FlatEdges, FlatEdges)> {
    let mut outRoots: DefiniteRoots = metamodelica::nil();
    let mut outConnectedConnections: FlatEdges = metamodelica::nil();
    let mut outBrokenConnections: FlatEdges = metamodelica::nil();
    (outRoots, outConnectedConnections, outBrokenConnections) = (::match_deref::match_deref! { match &((inGraph.clone(), modelNameQualified.clone())) {
        (NFOCConnectionGraph { connections: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, definiteRoots: Deref @ metamodelica::List::Nil, .. }, _) => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        (NFOCConnectionGraph { connections, branches, uniqueRoots, potentialRoots, definiteRoots, .. }, _) => {
            let mut finalRoots: DefiniteRoots = metamodelica::nil();
            let mut orderedPotentialRoots: PotentialRoots = metamodelica::nil();
            let mut broken: FlatEdges = metamodelica::nil();
            let mut connected: FlatEdges = metamodelica::nil();
            let mut table: CrefCrefTable;
            let mut dummyRoot: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut brokenConnectsViaGraphViz: ArcStr = arcstr::literal!("");
            let mut userBrokenLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut userBrokenLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            let mut userBrokenTplLst: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
            let mut connections = (*connections).clone();
            connections = connections.clone().reverse();
            table = resultGraphWithRoots(definiteRoots.clone())?;
            addBranchesToTable(table.clone(), branches.clone())?;
            orderedPotentialRoots = List::sort(potentialRoots.clone(), Arc::new(ord))?;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Ordered Potential Roots: ")); __mm_s.push_str(&*stringDelimitList(List::map(orderedPotentialRoots.clone(), Arc::new(printPotentialRootTuple)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (connected, broken) = addConnections(table.clone(), connections.clone());
            dummyRoot = NFBuiltin::TIME_CREF().clone();
            finalRoots = addPotentialRootsToTable(table.clone(), orderedPotentialRoots.clone(), definiteRoots.clone(), dummyRoot.clone())?;
            brokenConnectsViaGraphViz = (generateGraphViz((modelNameQualified.clone()).clone(), definiteRoots.clone(), potentialRoots.clone(), uniqueRoots.clone(), branches.clone(), connections.clone(), finalRoots.clone(), broken.clone())?).clone();
            if stringEq((brokenConnectsViaGraphViz.clone()).clone(), (literal!("")).clone()) {
            } else {
                userBrokenLst = Util::stringSplitAtChar((brokenConnectsViaGraphViz.clone()).clone(), (literal!("#")).clone())?;
                userBrokenLstLst = List::map1(userBrokenLst.clone(), Arc::new(Util::stringSplitAtChar), (literal!("|")).clone());
                userBrokenTplLst = makeTuple(userBrokenLstLst.clone())?;
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("User selected the following connect edges for breaking:\n\t")); __mm_s.push_str(&*stringDelimitList(List::map(userBrokenTplLst.clone(), Arc::new(fnptr!(printTupleStr, (ArcStr, ArcStr)))), (literal!("\n\t")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                printFlatEdges(connections.clone())?;
                connections = orderConnectsGuidedByUser(connections.clone(), userBrokenTplLst.clone())?;
                connections = connections.clone().reverse();
                println!("{}", (literal!("\nAfer ordering:\n")).clone());
                (finalRoots, connected, broken) = findResultGraph(NFOCConnectionGraph { updateGraph: false, definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() }, (modelNameQualified.clone()).clone())?;
            }
            (finalRoots.clone(), connected.clone(), broken.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outRoots, outConnectedConnections, outBrokenConnections))
}

fn orderConnectsGuidedByUser(mut inConnections: FlatEdges, mut inUserSelectedBreaking: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<FlatEdges> {
    let mut outOrderedConnections: FlatEdges = metamodelica::nil();
    let mut front: FlatEdges = metamodelica::nil();
    let mut back: FlatEdges = metamodelica::nil();
    let mut sc1: ArcStr = arcstr::literal!("");
    let mut sc2: ArcStr = arcstr::literal!("");
    for mut e in &*inConnections.clone() {
        let mut e = e.clone();
        sc1 = (ComponentRef::toString(e.lhs.clone())?).clone();
        sc2 = (ComponentRef::toString(e.rhs.clone())?).clone();
        if listMember((sc1.clone(), sc2.clone()), inUserSelectedBreaking.clone()) || listMember((sc2.clone(), sc1.clone()), inUserSelectedBreaking.clone()) {
            back = cons(e.clone(), back.clone());
        } else {
            front = cons(e.clone(), front.clone());
        }
    }
    outOrderedConnections = List::append_reverse(front.clone(), back.clone());
    Ok(outOrderedConnections)
}

fn printTupleStr(mut inTpl: (ArcStr, ArcStr)) -> ArcStr {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((match inTpl.clone() {
        (mut c1, mut c2) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*c1.clone()); __mm_s.push_str(&*literal!(" -- ")); __mm_s.push_str(&*c2.clone()); ArcStr::from(__mm_s) }
        },
    })).clone();
    out
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeTuple(mut inLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Result<Arc<metamodelica::List<(ArcStr, ArcStr)>>> {
    let mut outLst: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    outLst = 'mc: {
        let __mc_input = inLstLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: c1, tail: Deref @ metamodelica::List::Cons { head: c2, tail: Deref @ metamodelica::List::Nil } }, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    lst = makeTuple(rest.clone())?;
                    Ok(cons((c1.clone(), c2.clone()), lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Nil }, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    lst = makeTuple(rest.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Nil, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    lst = makeTuple(rest.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: bad, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following output from GraphViz OpenModelica assistant cannot be parsed:")); __mm_s.push_str(&*stringDelimitList(bad.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\nExpected format from GrapViz: cref1|cref2#cref3|cref4#. Ignoring malformed input.\n")); ArcStr::from(__mm_s) }).clone());
                    lst = makeTuple(rest.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLst)
}

fn printPotentialRootTuple(mut potentialRoot: PotentialRoot) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ((::match_deref::match_deref! { match &(potentialRoot.clone()) {
        (cr, priority) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(cr.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStr)
}

fn buildRootedTable(mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut graph: NFOCConnectionGraph) -> Result<CrefIndexTable> {
    let mut rooted: CrefIndexTable;
    let mut table: CrefRootsTable;
    table = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
    List::map1_0(getBranches(graph.clone())?, Arc::new(addBranches), table.clone());
    List::map1_0(getConnections(graph.clone())?, Arc::new(addConnectionsRooted), table.clone());
    rooted = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
    setRootDistance(roots.clone(), table.clone(), 0, metamodelica::nil(), rooted.clone())?;
    Ok(rooted)
}

fn setRootDistance(mut finalRoots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut table: CrefRootsTable, mut distance: i32, mut nextLevel: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut rooted: CrefIndexTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((finalRoots.clone(), nextLevel.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            setRootDistance(nextLevel.clone(), table.clone(), distance.clone() + 1, metamodelica::nil(), rooted.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, _) if (!(UnorderedMap::contains(cr.clone(), rooted.clone()))) => {
            let mut next: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            UnorderedMap::addNew(cr.clone(), distance.clone(), rooted.clone())?;
            next = (::match_deref::match_deref! { match &(UnorderedMap::get(cr.clone(), table.clone())) {
        Some(next) => listAppend(nextLevel.clone(), next.clone()),
        _ => nextLevel.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            setRootDistance(rest.clone(), table.clone(), distance.clone(), next.clone(), rooted.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            setRootDistance(rest.clone(), table.clone(), distance.clone(), nextLevel.clone(), rooted.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn addBranches(mut edge: Edge, mut table: CrefRootsTable) -> Result<()> {
    let mut cref1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cref2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    (cref1, cref2) = edge.clone();
    addConnectionRooted(cref1.clone(), cref2.clone(), table.clone())?;
    addConnectionRooted(cref2.clone(), cref1.clone(), table.clone())?;
    Ok(())
}

fn addConnectionsRooted(mut connection: FlatEdge, mut table: CrefRootsTable) -> Result<()> {
    addConnectionRooted(connection.lhs.clone(), connection.rhs.clone(), table.clone())?;
    addConnectionRooted(connection.rhs.clone(), connection.lhs.clone(), table.clone())?;
    Ok(())
}

fn addConnectionRooted(mut cref1: Arc<ComponentRef::NFComponentRef>, mut cref2: Arc<ComponentRef::NFComponentRef>, mut table: CrefRootsTable) -> Result<()> {
    pub fn updateRooted(mut roots: Option<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>, mut newRoot: Arc<ComponentRef::NFComponentRef>) -> DefiniteRoots {
        let mut outRoots: DefiniteRoots = metamodelica::nil();
        outRoots = (::match_deref::match_deref! { match &(roots.clone()) {
        Some(outRoots) => cons(newRoot.clone(), outRoots.clone()),
        _ => list![newRoot.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outRoots
    }

    UnorderedMap::addUpdate(cref1.clone(), Arc::new({ let __pe_b1 = cref2.clone(); move |__pe_a0| Ok(updateRooted(__pe_a0, __pe_b1.clone())) }), table.clone())?;
    Ok(())
}

fn evalConnectionsOperatorsEqs(mut inRoots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut rooted: CrefIndexTable, mut graph: NFOCConnectionGraph, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Arc<metamodelica::List<Arc<Equation::NFEquation>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    equations = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (equations.clone()).into_iter().cloned() {
            let __x = Equation::mapExpShallow(eq.clone(), { let __pe_b1 = rooted.clone(); let __pe_b2 = inRoots.clone(); let __pe_b3 = graph.clone(); let __pe_b4 = Equation::info(eq.clone()); move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    equations
}

fn evalConnectionsOperatorsVar(mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut rooted: CrefIndexTable, mut graph: NFOCConnectionGraph, mut var: Arc<Variable::NFVariable>) -> Arc<Variable::NFVariable> {
    let mut var: Arc<Variable::NFVariable> = var;
    assign_field!(var.binding = Binding::mapExpShallow(var.binding.clone(), Arc::new({ let __pe_b1 = rooted.clone(); let __pe_b2 = roots.clone(); let __pe_b3 = graph.clone(); let __pe_b4 = var.info.clone(); move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) })));
    var
}

fn evaluateOperators(mut exp: Arc<Expression::NFExpression>, mut rooted: CrefIndexTable, mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut graph: NFOCConnectionGraph, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = rooted.clone(); let __pe_b2 = roots.clone(); let __pe_b3 = graph.clone(); let __pe_b4 = info.clone(); move |__pe_a0| evalConnectionsOperatorsHelper(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
    Ok(exp)
}

fn evalConnectionsOperatorsHelper(mut exp: Arc<Expression::NFExpression>, mut rooted: CrefIndexTable, mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut graph: NFOCConnectionGraph, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut uroots: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut nodes: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut message: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut cref1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut result: bool = false;
            let mut branches: Edges = metamodelica::nil();
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            (match identifyConnectionsOperator(Function::name(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())) {
        ConnectionsOperator::ROOTED => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        _ if (Expression::isEmptyArray(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?)) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(" = false\n")); ArcStr::from(__mm_s) }).clone());
            }
            Arc::new(Expression::NFExpression::BOOLEAN { value: false })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref, .. }, tail: Deref @ metamodelica::List::Nil } => {
            let mut cref = (*cref).clone();
            branches = getBranches(graph.clone())?;
            cref = ComponentRef::stripIteratorSubscripts(cref.clone())?;
            match '__try0: {
                cref1 = unwrap_break_err!(getEdge(cref.clone(), branches.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: Found Branche Partner ")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(cref.clone()), '__try0)); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(cref1.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                result = unwrap_break_err!(getRooted(cref.clone(), cref1.clone(), rooted.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*unwrap_break_err!(Expression::toString(exp.clone()), '__try0)); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                Ok::<_, anyhow::Error>((result.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    result = __try0_o0;
                }
                Err(_) => {
                    r#str = (ComponentRef::toString(cref.clone())?).clone();
                    Error::addSourceMessage(Error::OCG_MISSING_BRANCH.clone(), list![(r#str.clone()).clone(), (r#str.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    result = false;
                }
            }
            Arc::new(Expression::NFExpression::BOOLEAN { value: result.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            res.clone()
        },
        ConnectionsOperator::IS_ROOT => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        _ if (Expression::isEmptyArray(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?)) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(" = false\n")); ArcStr::from(__mm_s) }).clone());
            }
            Arc::new(Expression::NFExpression::BOOLEAN { value: false })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref, .. }, tail: Deref @ metamodelica::List::Nil } => {
            let mut cref = (*cref).clone();
            cref = ComponentRef::stripIteratorSubscripts(cref.clone())?;
            result = List::isMemberOnTrue(cref.clone(), roots.clone(), Arc::new(ComponentRef::isEqual));
            if Flags::isSet(Flags::CGRAPH.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            Arc::new(Expression::NFExpression::BOOLEAN { value: result.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            res.clone()
        },
        ConnectionsOperator::UNIQUE_ROOT_INDICES => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        Deref @ metamodelica::List::Cons { head: uroots, tail: Deref @ metamodelica::List::Cons { head: nodes, tail: Deref @ metamodelica::List::Cons { head: message, tail: Deref @ metamodelica::List::Nil } } } => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: Connections.uniqueRootsIndices(")); __mm_s.push_str(&*Expression::toString(uroots.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*Expression::toString(nodes.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*Expression::toString(message.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
            }
            dim = Type::nthDimension(Expression::typeOf(uroots.clone()), 1)?;
            if !(Dimension::isKnown(dim.clone(), false)) {
                Error::addSourceMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![(Expression::toString(exp.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            Expression::fillArray(Dimension::size(dim.clone(), false)?, Arc::new(Expression::NFExpression::INTEGER { value: 1 }))
        },
        _ => bail!("match: no arm matched"),
    } });
            res.clone()
        },
        _ => exp.clone(),
    })
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn getRooted(mut cref1: Arc<ComponentRef::NFComponentRef>, mut cref2: Arc<ComponentRef::NFComponentRef>, mut rooted: CrefIndexTable) -> Result<bool> {
    let mut result: bool = false;
    result = 'mc: {
        let __mc_input = (cref1.clone(), cref2.clone(), rooted.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    i1 = UnorderedMap::getOrFail(cref1.clone(), rooted.clone());
                    i2 = UnorderedMap::getOrFail(cref2.clone(), rooted.clone());
                    Ok(intLt(i1.clone(), i2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(result)
}

fn getEdge(mut cr: Arc<ComponentRef::NFComponentRef>, mut edges: Edges) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut ocr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cref1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cref2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    for mut edge in &*edges.clone() {
        let mut edge = edge.clone();
        (cref1, cref2) = edge.clone();
        if ComponentRef::isEqual(cr.clone(), cref1.clone())? {
            ocr = cref2.clone();
            return Ok(ocr);
        } else if ComponentRef::isEqual(cr.clone(), cref2.clone())? {
            ocr = cref1.clone();
            return Ok(ocr);
        }
    }
    bail!("fail");
    Ok(ocr)
}

fn printConnectionStr(mut edge: FlatEdge, mut ty: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ty.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentRef::toString(edge.lhs.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(edge.rhs.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(outStr)
}

fn printEdges(mut inEdges: Edges) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inEdges.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (c1, c2), tail: tail } => {
            println!("{}", (literal!("    ")).clone());
            println!("{}", (ComponentRef::toString(c1.clone())?).clone());
            println!("{}", (literal!(" -- ")).clone());
            println!("{}", (ComponentRef::toString(c2.clone())?).clone());
            println!("{}", (literal!("\n")).clone());
            printEdges(tail.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printFlatEdges(mut inEdges: FlatEdges) -> Result<()> {
    for mut edge in &*inEdges.clone() {
        let mut edge = edge.clone();
        println!("{}", (literal!("    ")).clone());
        println!("{}", (ComponentRef::toString(edge.lhs.clone())?).clone());
        println!("{}", (literal!(" -- ")).clone());
        println!("{}", (ComponentRef::toString(edge.rhs.clone())?).clone());
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

fn printNFOCConnectionGraph(mut inGraph: NFOCConnectionGraph) -> Result<()> {
    let _ = (match inGraph.clone() {
        NFOCConnectionGraph { branches: mut branches, connections: mut connections, .. } => {
            println!("{}", (literal!("Connections:\n")).clone());
            printFlatEdges(connections.clone())?;
            println!("{}", (literal!("Branches:\n")).clone());
            printEdges(branches.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn getDefiniteRoots(mut inGraph: NFOCConnectionGraph) -> Result<DefiniteRoots> {
    let mut outResult: DefiniteRoots = metamodelica::nil();
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph { definiteRoots: ref result, .. } => {
            result.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outResult)
}

fn getUniqueRoots(mut inGraph: NFOCConnectionGraph) -> Result<UniqueRoots> {
    let mut outResult: UniqueRoots = metamodelica::nil();
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph { uniqueRoots: ref result, .. } => {
            result.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outResult)
}

fn getPotentialRoots(mut inGraph: NFOCConnectionGraph) -> Result<PotentialRoots> {
    let mut outResult: PotentialRoots = metamodelica::nil();
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph { potentialRoots: ref result, .. } => {
            result.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outResult)
}

fn getBranches(mut inGraph: NFOCConnectionGraph) -> Result<Edges> {
    let mut outResult: Edges = metamodelica::nil();
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph { branches: ref result, .. } => {
            result.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outResult)
}

fn getConnections(mut inGraph: NFOCConnectionGraph) -> Result<FlatEdges> {
    let mut outResult: FlatEdges = metamodelica::nil();
    outResult = (match inGraph.clone() {
        NFOCConnectionGraph { connections: ref result, .. } => {
            result.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outResult)
}

fn merge(mut inGraph1: NFOCConnectionGraph, mut inGraph2: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut outGraph: NFOCConnectionGraph;
    outGraph = 'mc: {
        let __mc_input = (inGraph1.clone(), inGraph2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, NFOCConnectionGraph { connections: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, definiteRoots: Deref @ metamodelica::List::Nil, .. }) => {
                    Ok(inGraph1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (NFOCConnectionGraph { connections: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, definiteRoots: Deref @ metamodelica::List::Nil, .. }, _) => {
                    Ok(inGraph2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    if !((inGraph1.clone() == inGraph2.clone())) { bail!("guard") }
                    Ok(inGraph1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (NFOCConnectionGraph { connections: connections1, branches: branches1, uniqueRoots: uniqueRoots1, potentialRoots: potentialRoots1, definiteRoots: definiteRoots1, updateGraph: updateGraph1 }, NFOCConnectionGraph { connections: connections2, branches: branches2, uniqueRoots: uniqueRoots2, potentialRoots: potentialRoots2, definiteRoots: definiteRoots2, updateGraph: updateGraph2 }) => {
                    let mut updateGraph: bool = false;
                    let mut definiteRoots: DefiniteRoots = metamodelica::nil();
                    let mut uniqueRoots: UniqueRoots = metamodelica::nil();
                    let mut potentialRoots: PotentialRoots = metamodelica::nil();
                    let mut branches: Edges = metamodelica::nil();
                    let mut connections: FlatEdges = metamodelica::nil();
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::trace((literal!("- NFOCConnectionGraph.merge()\n")).clone())?;
                    }
                    updateGraph = boolOr(updateGraph1.clone(), updateGraph2.clone());
                    definiteRoots = List::union(definiteRoots1.clone(), definiteRoots2.clone());
                    potentialRoots = List::union(potentialRoots1.clone(), potentialRoots2.clone());
                    uniqueRoots = List::union(uniqueRoots1.clone(), uniqueRoots2.clone());
                    branches = List::union(branches1.clone(), branches2.clone());
                    connections = List::union(connections1.clone(), connections2.clone());
                    Ok(NFOCConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

/* **********************************************************************************************************************/
/* ****************************************** GraphViz generation *******************************************************/
/* **********************************************************************************************************************/
fn graphVizEdge(mut inEdge: Edge) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((::match_deref::match_deref! { match &(inEdge.clone()) {
        (c1, c2) => {
            let mut strEdge: ArcStr = arcstr::literal!("");
            strEdge = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c1.clone())?); __mm_s.push_str(&*literal!("\" -- \"")); __mm_s.push_str(&*ComponentRef::toString(c2.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [color = blue, dir = \"none\", fontcolor=blue, label = \"branch\"];\n\t")); ArcStr::from(__mm_s) }).clone();
            strEdge.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn graphVizFlatEdge(mut edge: FlatEdge, mut inBrokenFlatEdges: FlatEdges) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    let mut sc1: ArcStr = arcstr::literal!("");
    let mut sc2: ArcStr = arcstr::literal!("");
    let mut label: ArcStr = arcstr::literal!("");
    let mut labelFontSize: ArcStr = arcstr::literal!("");
    let mut decorate: ArcStr = arcstr::literal!("");
    let mut color: ArcStr = arcstr::literal!("");
    let mut style: ArcStr = arcstr::literal!("");
    let mut fontColor: ArcStr = arcstr::literal!("");
    let mut isBroken: bool = false;
    isBroken = List::isMemberOnTrue(edge.clone(), inBrokenFlatEdges.clone(), Arc::new(FlatEdgeIsEqual));
    label = (if (isBroken.clone()) {literal!("[[broken connect]]")} else {literal!("connect")}).clone();
    color = (if (isBroken.clone()) {literal!("red")} else {literal!("green")}).clone();
    style = (if (isBroken.clone()) {literal!("\"bold, dashed\"")} else {literal!("solid")}).clone();
    decorate = (boolString(isBroken.clone())).clone();
    fontColor = (if (isBroken.clone()) {literal!("red")} else {literal!("green")}).clone();
    labelFontSize = (if (isBroken.clone()) {literal!("labelfontsize = 20.0, ")} else {literal!("")}).clone();
    sc1 = (ComponentRef::toString(edge.lhs.clone())?).clone();
    sc2 = (ComponentRef::toString(edge.rhs.clone())?).clone();
    out = stringAppendList(list![(literal!("\"")).clone(), (sc1.clone()).clone(), (literal!("\" -- \"")).clone(), (sc2.clone()).clone(), (literal!("\" [")).clone(), (literal!("dir = \"none\", ")).clone(), (literal!("style = ")).clone(), (style.clone()).clone(), (literal!(", ")).clone(), (literal!("decorate = ")).clone(), (decorate.clone()).clone(), (literal!(", ")).clone(), (literal!("color = ")).clone(), (color.clone()).clone(), (literal!(", ")).clone(), (labelFontSize.clone()).clone(), (literal!("fontcolor = ")).clone(), (fontColor.clone()).clone(), (literal!(", ")).clone(), (literal!("label = \"")).clone(), (label.clone()).clone(), (literal!("\"")).clone(), (literal!("];\n\t")).clone()]);
    Ok(out)
}

fn FlatEdgeIsEqual(mut inEdge1: FlatEdge, mut inEdge2: FlatEdge) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = ComponentRef::isEqual(inEdge1.lhs.clone(), inEdge2.lhs.clone())? && ComponentRef::isEqual(inEdge1.rhs.clone(), inEdge2.rhs.clone())?;
    Ok(isEqual)
}

fn graphVizDefiniteRoot(mut inDefiniteRoot: DefiniteRoot, mut inFinalRoots: DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((::match_deref::match_deref! { match &((inDefiniteRoot.clone(), inFinalRoots.clone())) {
        (c, _) => {
            let mut strDefiniteRoot: ArcStr = arcstr::literal!("");
            let mut isSelectedRoot: bool = false;
            isSelectedRoot = List::isMemberOnTrue(c.clone(), inFinalRoots.clone(), Arc::new(ComponentRef::isEqual));
            strDefiniteRoot = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = red, rank = \"source\", label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c.clone())?); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot.clone()) {literal!("shape=polygon, sides=8, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) }).clone();
            strDefiniteRoot.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn graphVizPotentialRoot(mut inPotentialRoot: PotentialRoot, mut inFinalRoots: DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((::match_deref::match_deref! { match &((inPotentialRoot.clone(), inFinalRoots.clone())) {
        ((c, priority), _) => {
            let mut strPotentialRoot: ArcStr = arcstr::literal!("");
            let mut isSelectedRoot: bool = false;
            isSelectedRoot = List::isMemberOnTrue(c.clone(), inFinalRoots.clone(), Arc::new(ComponentRef::isEqual));
            strPotentialRoot = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = orangered, rank = \"min\" label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c.clone())?); __mm_s.push_str(&*literal!("\\n")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot.clone()) {literal!("shape=ploygon, sides=7, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) }).clone();
            strPotentialRoot.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn generateGraphViz(mut modelNameQualified: ArcStr, mut definiteRoots: DefiniteRoots, mut potentialRoots: PotentialRoots, mut uniqueRoots: UniqueRoots, mut branches: Edges, mut connections: FlatEdges, mut finalRoots: DefiniteRoots, mut broken: FlatEdges) -> Result<ArcStr> {
    let mut brokenConnectsViaGraphViz: ArcStr = arcstr::literal!("");
    brokenConnectsViaGraphViz = ('mc: {
        let __mc_input = (modelNameQualified.clone(), definiteRoots.clone(), potentialRoots.clone(), uniqueRoots.clone(), branches.clone(), connections.clone(), finalRoots.clone(), broken.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let false = (boolOr(Flags::isSet(Flags::CGRAPH_GRAPHVIZ_FILE.clone())?, Flags::isSet(Flags::CGRAPH_GRAPHVIZ_SHOW.clone())?)) else { bail!("pattern mismatch") };
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut i: ArcStr = arcstr::literal!("");
                    let mut nrDR: ArcStr = arcstr::literal!("");
                    let mut nrPR: ArcStr = arcstr::literal!("");
                    let mut nrUR: ArcStr = arcstr::literal!("");
                    let mut nrBR: ArcStr = arcstr::literal!("");
                    let mut nrCO: ArcStr = arcstr::literal!("");
                    let mut nrFR: ArcStr = arcstr::literal!("");
                    let mut nrBC: ArcStr = arcstr::literal!("");
                    let mut timeStr: ArcStr = arcstr::literal!("");
                    let mut infoNodeStr: ArcStr = arcstr::literal!("");
                    let mut brokenConnects: ArcStr = arcstr::literal!("");
                    let mut tStart: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut tEnd: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut t: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut graphVizStream: IOStream::IOStream;
                    let mut infoNode: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    tStart = clock();
                    i = (literal!("\t")).clone();
                    fileName = (stringAppend((modelNameQualified.clone()).clone(), (literal!(".gv")).clone())).clone();
                    graphVizStream = IOStream::create((fileName.clone()).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
                    nrDR = (intString((definiteRoots.clone().len() as i32))).clone();
                    nrPR = (intString((potentialRoots.clone().len() as i32))).clone();
                    nrUR = (intString((uniqueRoots.clone().len() as i32))).clone();
                    nrBR = (intString((branches.clone().len() as i32))).clone();
                    nrCO = (intString((connections.clone().len() as i32))).clone();
                    nrFR = (intString((finalRoots.clone().len() as i32))).clone();
                    nrBC = (intString((broken.clone().len() as i32))).clone();
                    infoNode = list![(literal!("// Generated by OpenModelica.\n")).clone(), (literal!("// Overconstrained connection graph for model:\n//    ")).clone(), (modelNameQualified.clone()).clone(), (literal!("\n")).clone(), (literal!("//\n")).clone(), (literal!("// Summary:\n")).clone(), (literal!("//   Roots:                      ")).clone(), (nrDR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Potential Roots:    ")).clone(), (nrPR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Unique Roots:       ")).clone(), (nrUR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Branches:           ")).clone(), (nrBR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Connections:        ")).clone(), (nrCO.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Final Roots:        ")).clone(), (nrFR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Broken Connections: ")).clone(), (nrBC.clone()).clone(), (literal!("\n")).clone()];
                    infoNodeStr = stringAppendList(infoNode.clone());
                    infoNodeStr = (System::stringReplace((infoNodeStr.clone()).clone(), (literal!("\n")).clone(), (literal!("\\l")).clone())?).clone();
                    infoNodeStr = (System::stringReplace((infoNodeStr.clone()).clone(), (literal!("\t")).clone(), (literal!(" ")).clone())?).clone();
                    infoNodeStr = (System::stringReplace((infoNodeStr.clone()).clone(), (literal!("/")).clone(), (literal!("")).clone())?).clone();
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), infoNode.clone());
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n\n")).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("graph \"")).clone(), (modelNameQualified.clone()).clone(), (literal!("\"\n{\n\n")).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("overlap=false;\n")).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("layout=dot;\n\n")).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("node [")).clone(), (literal!("fillcolor = \"lightsteelblue1\", ")).clone(), (literal!("shape = box, ")).clone(), (literal!("style = \"bold, filled\", ")).clone(), (literal!("rank = \"max\"")).clone(), (literal!("]\n\n")).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("edge [")).clone(), (literal!("color = \"black\", ")).clone(), (literal!("style = bold")).clone(), (literal!("]\n\n")).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("graph [fontsize=20, fontname = \"Courier Bold\" label= \"\\n\\n")).clone(), (infoNodeStr.clone()).clone(), (literal!("\", size=\"6,6\"];\n")).clone(), (i.clone()).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Definite Roots (Connections.root)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(definiteRoots.clone(), Arc::new(graphVizDefiniteRoot), finalRoots.clone()));
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Potential Roots (Connections.potentialRoot)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(potentialRoots.clone(), Arc::new(graphVizPotentialRoot), finalRoots.clone()));
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Branches (Connections.branch)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map(branches.clone(), Arc::new(graphVizEdge)));
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Connections (connect)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()]);
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(connections.clone(), Arc::new(graphVizFlatEdge), broken.clone()));
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n}\n")).clone()]);
                    tEnd = clock();
                    t = tEnd.clone() - tStart.clone();
                    timeStr = (realString(t.clone())).clone();
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n\n\n// graph generation took: ")).clone(), (timeStr.clone()).clone(), (literal!(" seconds\n")).clone()]);
                    System::writeFile((fileName.clone()).clone(), (IOStream::string(graphVizStream.clone())?).clone())?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz with connection graph for model: ")); __mm_s.push_str(&*modelNameQualified.clone()); __mm_s.push_str(&*literal!(" was writen to file: ")); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    brokenConnects = (showGraphViz((fileName.clone()).clone(), (modelNameQualified.clone()).clone())?).clone();
                    Ok(brokenConnects.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(brokenConnectsViaGraphViz)
}

fn showGraphViz(mut fileNameGraphViz: ArcStr, mut modelNameQualified: ArcStr) -> Result<ArcStr> {
    let mut brokenConnectsViaGraphViz: ArcStr = arcstr::literal!("");
    brokenConnectsViaGraphViz = ('mc: {
        let __mc_input = (fileNameGraphViz.clone(), modelNameQualified.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (Flags::isSet(Flags::CGRAPH_GRAPHVIZ_SHOW.clone())?) else { bail!("pattern mismatch") };
            Ok(literal!(""))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut leftyCMD: ArcStr = arcstr::literal!("");
            let mut fileNameTraceRemovedConnections: ArcStr = arcstr::literal!("");
            let mut omhome: ArcStr = arcstr::literal!("");
            let mut brokenConnects: ArcStr = arcstr::literal!("");
            let mut leftyExitStatus: i32 = 0;
            fileNameTraceRemovedConnections = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelNameQualified.clone()); __mm_s.push_str(&*literal!("_removed_connections.txt")); ArcStr::from(__mm_s) }).clone();
            println!("{}", (literal!("Tyring to start GraphViz *lefty* to visualize the graph. You need to have lefty in your PATH variable\n")).clone());
            println!("{}", (literal!("Make sure you quit GraphViz *lefty* via Right Click->quit to be sure the process will be exited.\n")).clone());
            println!("{}", (literal!("If you quit the GraphViz *lefty* window via X, please kill the process in task manager to continue.\n")).clone());
            omhome = (Settings::getInstallationDirectoryPath()?).clone();
            omhome = (System::stringReplace((omhome.clone()).clone(), (literal!("\"")).clone(), (literal!("")).clone())?).clone();
            leftyCMD = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("load('")); __mm_s.push_str(&*omhome.clone()); __mm_s.push_str(&*literal!("/share/omc/scripts/openmodelica.lefty');")); __mm_s.push_str(&*literal!("openmodelica.init();openmodelica.createviewandgraph('")); __mm_s.push_str(&*fileNameGraphViz.clone()); __mm_s.push_str(&*literal!("','file',null,null);txtview('off');")); ArcStr::from(__mm_s) }).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Running command: ")); __mm_s.push_str(&*literal!("lefty -e ")); __mm_s.push_str(&*leftyCMD.clone()); __mm_s.push_str(&*literal!(" > ")); __mm_s.push_str(&*fileNameTraceRemovedConnections.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            leftyExitStatus = System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lefty -e ")); __mm_s.push_str(&*leftyCMD.clone()); ArcStr::from(__mm_s) }).clone(), (fileNameTraceRemovedConnections.clone()).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz *lefty* exited with status:")); __mm_s.push_str(&*intString(leftyExitStatus.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            brokenConnects = (System::readFile((fileNameTraceRemovedConnections.clone()).clone())?).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz OpenModelica assistant returned the following broken connects: ")); __mm_s.push_str(&*brokenConnects.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(brokenConnects.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(brokenConnectsViaGraphViz)
}

fn removeBrokenConnects(mut inEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut inConnected: FlatEdges, mut inBroken: FlatEdges, mut isDeleted: IsDeletedFn) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut outEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    outEquations = (::match_deref::match_deref! { match &((inEquations.clone(), inConnected.clone(), inBroken.clone())) {
        (_, _, Deref @ metamodelica::List::Nil) => {
            let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            inEquations.clone()
        },
        (_, _, _) => {
            let mut lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut isThere: bool = false;
            let mut r#str: ArcStr = arcstr::literal!("");
            for mut eq in &*inEquations.clone() {
                let mut eq = eq.clone();
                eql = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::CONNECT { source, rhs: Deref @ Expression::CREF { cref: rhs, ty: ty2 }, lhs: Deref @ Expression::CREF { cref: lhs, ty: ty1 }, .. } => {
            if !(isDeleted(lhs.clone())? || isDeleted(rhs.clone())?) {
                isThere = false;
                for mut b in &*inBroken.clone() {
                    let mut b = b.clone();
                    if ComponentRef::isEqual(b.lhs.clone(), lhs.clone())? && ComponentRef::isEqual(b.rhs.clone(), rhs.clone())? || ComponentRef::isEqual(b.rhs.clone(), lhs.clone())? && ComponentRef::isEqual(b.lhs.clone(), rhs.clone())? {
                        isThere = true;
                        break;
                    }
                }
            }
            if !(isThere.clone()) {
                eql = cons(eq.clone(), eql.clone());
            }
            eql.clone()
        },
        _ => cons(eq.clone(), eql.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            eql = eql.clone().reverse();
            if Flags::isSet(Flags::CGRAPH.clone())? {
                r#str = (literal!("")).clone();
                for mut b in &*inBroken.clone() {
                    let mut b = b.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*ComponentRef::toString(b.lhs.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(b.rhs.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                }
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.removeBrokenConnects:\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            eql.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEquations)
}

fn identifyConnectionsOperator(mut functionName: Arc<Absyn::Path>) -> ConnectionsOperator {
    let mut call: ConnectionsOperator = ConnectionsOperator::BRANCH;
    call = (::match_deref::match_deref! { match &(functionName.clone()) {
        Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name }, name: Deref @ "Connections" } => {
            (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "branch" => ConnectionsOperator::BRANCH.clone(),
        Deref @ "root" => ConnectionsOperator::ROOT.clone(),
        Deref @ "potentialRoot" => ConnectionsOperator::POTENTIAL_ROOT.clone(),
        Deref @ "isRoot" => ConnectionsOperator::IS_ROOT.clone(),
        Deref @ "rooted" => ConnectionsOperator::ROOTED.clone(),
        Deref @ "uniqueRoot" => ConnectionsOperator::UNIQUE_ROOT.clone(),
        Deref @ "uniqueRootIndices" => ConnectionsOperator::UNIQUE_ROOT_INDICES.clone(),
        _ => ConnectionsOperator::NOT_OPERATOR.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "rooted" } => {
            ConnectionsOperator::ROOTED.clone()
        },
        _ => {
            ConnectionsOperator::NOT_OPERATOR.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    call
}

fn newCrefCrefTable() -> CrefCrefTable {
    let mut table: CrefCrefTable;
    table = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
    table
}

