// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::DAE::Connect;
use crate::DAE;
use crate::ElementSource;
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
use metamodelica::Dangerous::*;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Array;
use openmodelica_util::Debug;
use openmodelica_util::DisjointSets;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::List;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub enum ConnectionsOperator {
    BRANCH,
    ROOT,
    POTENTIAL_ROOT,
    IS_ROOT,
    ROOTED,
    UNIQUE_ROOT,
    UNIQUE_ROOT_INDICES,
    NOT_OPERATOR,
}

pub type CrefCrefTable = UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>;

pub type CrefIndexTable = UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>>;

pub type CrefRootsTable = UnorderedMap::UnorderedMap<metamodelica::List<Arc<ComponentRef::NFComponentRef>>, Arc<ComponentRef::NFComponentRef>>;

pub mod CrefSets {
    use super::*;
    pub type Entry = i32;

    pub fn EntryEqual(entry1: i32, entry2: i32) -> bool {
        todo!()
    }

    pub fn EntryHash(entry: i32) -> i32 {
        todo!()
    }

    pub fn EntryString(entry: i32) -> String {
        todo!()
    }

    pub type IndexTable = UnorderedMap::UnorderedMap<i32, i32>;

    pub struct DISJOINT_SETS {
        pub nodes: Vec<i32>,
        pub elements: UnorderedMap::UnorderedMap<i32, i32>,
        pub nodeCount: i32,
    }

    pub type Sets = DISJOINT_SETS;

    pub fn add(entry: i32, sets: Sets) -> (Sets, i32) {
        todo!()
    }

    pub fn addList(entries: metamodelica::List<i32>, sets: Sets) -> Sets {
        todo!()
    }

    pub fn contains(entry: i32, sets: Sets) -> bool {
        todo!()
    }

    pub fn emptySets(setCount: i32) -> Sets {
        todo!()
    }

    pub fn extractSets(sets: Sets) -> (Vec<metamodelica::List<i32>>, Sets) {
        todo!()
    }

    pub fn find(entry: i32, sets: Sets) -> (Sets, i32) {
        todo!()
    }

    pub fn findRoot(nodeIndex: i32, nodes: Vec<i32>) -> i32 {
        todo!()
    }

    pub fn findSet(entry: i32, sets: Sets) -> (i32, Sets) {
        todo!()
    }

    pub fn findSetArrayIndex(entry: i32, sets: Sets) -> i32 {
        todo!()
    }

    pub fn getNodeCount(sets: Sets) -> i32 {
        todo!()
    }

    pub fn merge(entry1: i32, entry2: i32, sets: Sets) -> Sets {
        todo!()
    }

    pub fn printSets(sets: Sets) -> () {
        todo!()
    }

    pub fn union(set1: i32, set2: i32, sets: Sets) -> Sets {
        todo!()
    }

}

pub type DefiniteRoot = Arc<ComponentRef::NFComponentRef>;

pub type DefiniteRoots = metamodelica::List<Arc<ComponentRef::NFComponentRef>>;

pub type Edge = (Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>);

pub type Edges = metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>;

pub type FlatEdge = NFConnections::BrokenEdge;

fn FlatEdgeIsEqual(inEdge1: NFConnections::BrokenEdge, inEdge2: NFConnections::BrokenEdge) -> bool {
    todo!()
}

pub type FlatEdges = metamodelica::List<NFConnections::BrokenEdge>;

pub type IsDeletedFn = fn(Arc<ComponentRef::NFComponentRef>) -> bool;

pub struct GRAPH {
    pub updateGraph: bool,
    pub definiteRoots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>,
    pub potentialRoots: metamodelica::List<(f64, Arc<ComponentRef::NFComponentRef>)>,
    pub uniqueRoots: metamodelica::List<(Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>)>,
    pub branches: metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>,
    pub connections: metamodelica::List<NFConnections::BrokenEdge>,
}

pub type NFOCConnectionGraph = GRAPH;

pub type PotentialRoot = (f64, Arc<ComponentRef::NFComponentRef>);

pub type PotentialRoots = metamodelica::List<(f64, Arc<ComponentRef::NFComponentRef>)>;

pub type UniqueRoots = metamodelica::List<(Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>)>;

fn addBranch(ref1: Arc<ComponentRef::NFComponentRef>, ref2: Arc<ComponentRef::NFComponentRef>, printTrace: bool, graph: NFOCConnectionGraph) -> NFOCConnectionGraph {
    todo!()
}

fn addBranches(edge: (Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>), table: UnorderedMap::UnorderedMap<metamodelica::List<Arc<ComponentRef::NFComponentRef>>, Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn addBranchesToTable(table: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, branches: metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>) -> () {
    todo!()
}

fn addBreakableBranches(connections: metamodelica::List<Arc<Connection::NFConnection>>, isDeleted: fn(Arc<ComponentRef::NFComponentRef>) -> bool, printTrace: bool, graph: NFOCConnectionGraph) -> NFOCConnectionGraph {
    todo!()
}

fn addConnection(ref1: Arc<ComponentRef::NFComponentRef>, ref2: Arc<ComponentRef::NFComponentRef>, source: Arc<DAE::ElementSource>, printTrace: bool, graph: NFOCConnectionGraph) -> NFOCConnectionGraph {
    todo!()
}

fn addConnectionRooted(cref1: Arc<ComponentRef::NFComponentRef>, cref2: Arc<ComponentRef::NFComponentRef>, table: UnorderedMap::UnorderedMap<metamodelica::List<Arc<ComponentRef::NFComponentRef>>, Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn addConnections(table: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, inConnections: metamodelica::List<NFConnections::BrokenEdge>) -> (metamodelica::List<NFConnections::BrokenEdge>, metamodelica::List<NFConnections::BrokenEdge>) {
    todo!()
}

fn addConnectionsRooted(connection: NFConnections::BrokenEdge, table: UnorderedMap::UnorderedMap<metamodelica::List<Arc<ComponentRef::NFComponentRef>>, Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn addDefiniteRoot(root: Arc<ComponentRef::NFComponentRef>, printTrace: bool, graph: NFOCConnectionGraph) -> NFOCConnectionGraph {
    todo!()
}

fn addPotentialRoot(root: Arc<ComponentRef::NFComponentRef>, priority: f64, printTrace: bool, graph: NFOCConnectionGraph) -> NFOCConnectionGraph {
    todo!()
}

fn addPotentialRootsToTable(table: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, potentialRoots: metamodelica::List<(f64, Arc<ComponentRef::NFComponentRef>)>, roots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, firstRoot: Arc<ComponentRef::NFComponentRef>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn addRootsAndBranches(equations: metamodelica::List<Arc<Equation::NFEquation>>, printTrace: bool, graph: NFOCConnectionGraph) -> (metamodelica::List<Arc<Equation::NFEquation>>, NFOCConnectionGraph) {
    todo!()
}

fn addRootsToTable(table: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, roots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, firstRoot: Arc<ComponentRef::NFComponentRef>) -> () {
    todo!()
}

fn addUniqueRoots(roots: Arc<Expression::NFExpression>, message: Arc<Expression::NFExpression>, printTrace: bool, graph: NFOCConnectionGraph) -> NFOCConnectionGraph {
    todo!()
}

fn areInSameComponent(partition: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, ref1: Arc<ComponentRef::NFComponentRef>, ref2: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn buildRootedTable(roots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, graph: NFOCConnectionGraph) -> UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn canonical(inPartition: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, inRef: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

fn connectBranchComponents(partition: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, ref1: Arc<ComponentRef::NFComponentRef>, ref2: Arc<ComponentRef::NFComponentRef>) -> () {
    todo!()
}

fn connectCanonicalComponents(inPartition: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, inRef1: Arc<ComponentRef::NFComponentRef>, inRef2: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn connectComponents(partition: UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>, edge: NFConnections::BrokenEdge) -> (metamodelica::List<NFConnections::BrokenEdge>, metamodelica::List<NFConnections::BrokenEdge>) {
    todo!()
}

fn evalConnectionsOperatorsEqs(inRoots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, rooted: UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>>, graph: NFOCConnectionGraph, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn evalConnectionsOperatorsHelper(exp: Arc<Expression::NFExpression>, rooted: UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>>, roots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, graph: NFOCConnectionGraph, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalConnectionsOperatorsVar(roots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, rooted: UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>>, graph: NFOCConnectionGraph, var: Arc<Variable::NFVariable>) -> Arc<Variable::NFVariable> {
    todo!()
}

fn evaluateOperators(exp: Arc<Expression::NFExpression>, rooted: UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>>, roots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, graph: NFOCConnectionGraph, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

fn findResultGraph(inGraph: NFOCConnectionGraph, modelNameQualified: String) -> (metamodelica::List<Arc<ComponentRef::NFComponentRef>>, metamodelica::List<NFConnections::BrokenEdge>, metamodelica::List<NFConnections::BrokenEdge>) {
    todo!()
}

fn generateEqualityConstraintEquation(lhs: Arc<ComponentRef::NFComponentRef>, rhs: Arc<ComponentRef::NFComponentRef>, source: Arc<DAE::ElementSource>) -> Arc<Equation::NFEquation> {
    todo!()
}

fn generateGraphViz(modelNameQualified: String, definiteRoots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, potentialRoots: metamodelica::List<(f64, Arc<ComponentRef::NFComponentRef>)>, uniqueRoots: metamodelica::List<(Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>)>, branches: metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, connections: metamodelica::List<NFConnections::BrokenEdge>, finalRoots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, broken: metamodelica::List<NFConnections::BrokenEdge>) -> String {
    todo!()
}

fn getBranches(inGraph: NFOCConnectionGraph) -> metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)> {
    todo!()
}

fn getConnections(inGraph: NFOCConnectionGraph) -> metamodelica::List<NFConnections::BrokenEdge> {
    todo!()
}

fn getDefiniteRoots(inGraph: NFOCConnectionGraph) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn getEdge(cr: Arc<ComponentRef::NFComponentRef>, edges: metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

fn getOverconstrainedCref(cref: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

fn getOverconstrainedCrefs(conn: Arc<Connector::NFConnector>, isDeleted: fn(Arc<ComponentRef::NFComponentRef>) -> bool) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn getPotentialRoots(inGraph: NFOCConnectionGraph) -> metamodelica::List<(f64, Arc<ComponentRef::NFComponentRef>)> {
    todo!()
}

fn getRooted(cref1: Arc<ComponentRef::NFComponentRef>, cref2: Arc<ComponentRef::NFComponentRef>, rooted: UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

fn getUniqueRoots(inGraph: NFOCConnectionGraph) -> metamodelica::List<(Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>)> {
    todo!()
}

fn graphVizDefiniteRoot(inDefiniteRoot: Arc<ComponentRef::NFComponentRef>, inFinalRoots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> String {
    todo!()
}

fn graphVizEdge(inEdge: (Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)) -> String {
    todo!()
}

fn graphVizFlatEdge(edge: NFConnections::BrokenEdge, inBrokenFlatEdges: metamodelica::List<NFConnections::BrokenEdge>) -> String {
    todo!()
}

fn graphVizPotentialRoot(inPotentialRoot: (f64, Arc<ComponentRef::NFComponentRef>), inFinalRoots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> String {
    todo!()
}

pub fn handleOverconstrainedConnections(flatModel: Arc<FlatModel::NFFlatModel>, conns: Arc<NFConnections::NFConnections>, isDeleted: fn(Arc<ComponentRef::NFComponentRef>) -> bool) -> (Arc<FlatModel::NFFlatModel>, metamodelica::List<NFConnections::BrokenEdge>) {
    todo!()
}

fn handleOverconstrainedConnections_dispatch(graph: NFOCConnectionGraph, flatModel: Arc<FlatModel::NFFlatModel>) -> (Arc<FlatModel::NFFlatModel>, metamodelica::List<NFConnections::BrokenEdge>, metamodelica::List<NFConnections::BrokenEdge>) {
    todo!()
}

fn identifyConnectionsOperator(functionName: Arc<Absyn::Path>) -> ConnectionsOperator {
    todo!()
}

fn isOverconstrainedCref(cref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn makeTuple(inLstLst: metamodelica::List<metamodelica::List<String>>) -> metamodelica::List<(String, String)> {
    todo!()
}

fn merge(inGraph1: NFOCConnectionGraph, inGraph2: NFOCConnectionGraph) -> NFOCConnectionGraph {
    todo!()
}

fn newCrefCrefTable() -> UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn ord(inEl1: (f64, Arc<ComponentRef::NFComponentRef>), inEl2: (f64, Arc<ComponentRef::NFComponentRef>)) -> bool {
    todo!()
}

fn orderConnectsGuidedByUser(inConnections: metamodelica::List<NFConnections::BrokenEdge>, inUserSelectedBreaking: metamodelica::List<(String, String)>) -> metamodelica::List<NFConnections::BrokenEdge> {
    todo!()
}

fn printConnectionStr(edge: NFConnections::BrokenEdge, ty: String) -> String {
    todo!()
}

fn printEdges(inEdges: metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>) -> () {
    todo!()
}

fn printFlatEdges(inEdges: metamodelica::List<NFConnections::BrokenEdge>) -> () {
    todo!()
}

fn printNFOCConnectionGraph(inGraph: NFOCConnectionGraph) -> () {
    todo!()
}

fn printPotentialRootTuple(potentialRoot: (f64, Arc<ComponentRef::NFComponentRef>)) -> String {
    todo!()
}

fn printTupleStr(inTpl: (String, String)) -> String {
    todo!()
}

fn removeBrokenConnects(inEquations: metamodelica::List<Arc<Equation::NFEquation>>, inConnected: metamodelica::List<NFConnections::BrokenEdge>, inBroken: metamodelica::List<NFConnections::BrokenEdge>, isDeleted: fn(Arc<ComponentRef::NFComponentRef>) -> bool) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn resultGraphWithRoots(roots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn setRootDistance(finalRoots: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, table: UnorderedMap::UnorderedMap<metamodelica::List<Arc<ComponentRef::NFComponentRef>>, Arc<ComponentRef::NFComponentRef>>, distance: i32, nextLevel: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, rooted: UnorderedMap::UnorderedMap<i32, Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn showGraphViz(fileNameGraphViz: String, modelNameQualified: String) -> String {
    todo!()
}

