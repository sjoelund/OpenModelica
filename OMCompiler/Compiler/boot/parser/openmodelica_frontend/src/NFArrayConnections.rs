// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE;
use crate::ElementSource;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Purity;
use crate::NFSBGraphUtil as SBGraphUtil;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;
use openmodelica_util::SBFunctions;
use openmodelica_util::SBGraph::IncidenceList;
use openmodelica_util::SBGraph::VertexDescriptor;
use openmodelica_util::SBPWLinearMap;
use openmodelica_util::SBSet;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;

pub type NameVertexTable = UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>;

pub type SBGraph = IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>;

pub mod SetEdge {
    use super::*;
    pub struct SET_EDGE {
        pub name: String,
        pub es1: Arc<SBPWLinearMap::SBPWLinearMap>,
        pub es2: Arc<SBPWLinearMap::SBPWLinearMap>,
    }

    pub type SetEdge = SET_EDGE;
    pub fn isEqual(e1: Arc<SetEdge>, e2: Arc<SetEdge>) -> bool {
        todo!()
    }

    pub fn toString(e: Arc<SetEdge>) -> String {
        todo!()
    }

}

pub mod SetVertex {
    use super::*;
    pub struct SET_VERTEX {
        pub name: Arc<Connector::NFConnector>,
        pub vs: Arc<SBSet::SBSet>,
    }

    pub type SetVertex = SET_VERTEX;
    pub fn isEqual(v1: Arc<SetVertex>, v2: Arc<SetVertex>) -> bool {
        todo!()
    }

    pub fn isNamed(v: Arc<SetVertex>, name: Arc<Connector::NFConnector>) -> bool {
        todo!()
    }

    pub fn toString(v: Arc<SetVertex>) -> String {
        todo!()
    }

}

fn addConnectionsToGraph(equations: metamodelica::List<Arc<Equation::NFEquation>>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, vCount: Vector::Vector<i32>, eCount: Vector::Vector<i32>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>) -> () {
    todo!()
}

fn addFlowsToGraph(variables: metamodelica::List<Arc<Variable::NFVariable>>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, vCount: Vector::Vector<i32>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>) -> () {
    todo!()
}

fn applyOffset(mi: Arc<SBMultiInterval::SBMultiInterval>, off: Vec<i32>) -> Arc<SBMultiInterval::SBMultiInterval> {
    todo!()
}

fn collect(flatModel: Arc<FlatModel::NFFlatModel>) -> (Arc<FlatModel::NFFlatModel>, metamodelica::List<Arc<Equation::NFEquation>>) {
    todo!()
}

fn createConnection(lhs: Arc<Expression::NFExpression>, rhs: Arc<Expression::NFExpression>, source: Arc<DAE::ElementSource>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, vCount: Vector::Vector<i32>, eCount: Vector::Vector<i32>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>) -> () {
    todo!()
}

fn createGraph(variables: metamodelica::List<Arc<Variable::NFVariable>>, equations: metamodelica::List<Arc<Equation::NFEquation>>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, vCount: Vector::Vector<i32>, eCount: Vector::Vector<i32>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>) -> () {
    todo!()
}

fn createMaps(graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>) -> (Arc<SBSet::SBSet>, Arc<SBPWLinearMap::SBPWLinearMap>, Arc<SBPWLinearMap::SBPWLinearMap>) {
    todo!()
}

fn createVertex(conn: Arc<Connector::NFConnector>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, vCount: Vector::Vector<i32>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>) -> (Arc<SBMultiInterval::SBMultiInterval>, i32) {
    todo!()
}

fn crefDims(cr: Arc<ComponentRef::NFComponentRef>) -> metamodelica::List<Arc<Dimension::NFDimension>> {
    todo!()
}

fn generateConnector(cr: Arc<ComponentRef::NFComponentRef>, indices: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn generateEquations(pw: Arc<SBPWLinearMap::SBPWLinearMap>, flatModel: Arc<FlatModel::NFFlatModel>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, vCount: Vector::Vector<i32>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn generateFlowEquation(aset: Arc<SBAtomicSet::SBAtomicSet>, dom: Arc<SBSet::SBSet>, iterators: Vec<Arc<InstNode::InstNode>>, flowVars: metamodelica::List<Arc<Variable::NFVariable>>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn generateForLoop(connects: metamodelica::List<Arc<Equation::NFEquation>>, iterators: Vec<Arc<InstNode::InstNode>>, ranges: Vec<Arc<Expression::NFExpression>>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn generatePotentialEquations(aset: Arc<SBAtomicSet::SBAtomicSet>, dom: Arc<SBSet::SBSet>, vars: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, iterators: Vec<Arc<InstNode::InstNode>>, iterExps: metamodelica::List<Arc<Expression::NFExpression>>, potVars: metamodelica::List<Arc<Variable::NFVariable>>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn generatePotentialEquations2(vars1: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, vars2: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, inds1: metamodelica::List<Arc<Expression::NFExpression>>, inds2: metamodelica::List<Arc<Expression::NFExpression>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn getConnectIntervals(conn: Arc<Connector::NFConnector>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, vCount: Vector::Vector<i32>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>) -> (Arc<SBMultiInterval::SBMultiInterval>, i32) {
    todo!()
}

fn getConnectors(flatModel: Arc<FlatModel::NFFlatModel>) -> (metamodelica::List<Arc<Variable::NFVariable>>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

fn getOffset(mi: Arc<SBMultiInterval::SBMultiInterval>, nmvTable: UnorderedMap::UnorderedMap<Arc<SBMultiInterval::SBMultiInterval>, String>) -> Vec<i32> {
    todo!()
}

fn getVars(vars: metamodelica::List<Arc<Variable::NFVariable>>, sauxi: Arc<SBSet::SBSet>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn intervalToRange(interval: Arc<SBInterval::SBInterval>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn isConnection(eq: Arc<Equation::NFEquation>) -> bool {
    todo!()
}

pub fn resolve(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

fn separate(cref: Arc<ComponentRef::NFComponentRef>) -> (Arc<ComponentRef::NFComponentRef>, metamodelica::List<Arc<Subscript::NFSubscript>>) {
    todo!()
}

fn transMulti(mi1: Arc<SBMultiInterval::SBMultiInterval>, mi2: Arc<SBMultiInterval::SBMultiInterval>, iterators: Vec<Arc<InstNode::InstNode>>, forFlow: bool) -> (metamodelica::List<Arc<Expression::NFExpression>>, bool) {
    todo!()
}

fn updateGraph(d1: i32, d2: i32, mi1: Arc<SBMultiInterval::SBMultiInterval>, mi2: Arc<SBMultiInterval::SBMultiInterval>, graph: IncidenceList::IncidenceList<Arc<SetEdge::SetEdge>, Arc<SetVertex::SetVertex>>, eCount: Vector::Vector<i32>) -> () {
    todo!()
}

