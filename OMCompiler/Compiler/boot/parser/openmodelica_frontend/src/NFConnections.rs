// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE;
use crate::ElementSource;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::ConnectorType;
use crate::NFType as Type;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Flags;

pub struct CONNECTIONS {
    pub connections: metamodelica::List<Arc<Connection::NFConnection>>,
    pub flows: metamodelica::List<Arc<Connector::NFConnector>>,
    pub broken: metamodelica::List<BrokenEdge>,
}

pub type NFConnections = CONNECTIONS;
pub type BrokenEdges = metamodelica::List<BrokenEdge>;

pub fn new() -> Arc<Connections> {
    todo!()
}

pub fn fromConnectionList(connl: metamodelica::List<Arc<Connection::NFConnection>>) -> Arc<Connections> {
    todo!()
}

pub fn addConnection(conn: Arc<Connection::NFConnection>, conns: Arc<Connections>) -> Arc<Connections> {
    todo!()
}

pub fn addFlow(conn: Arc<Connector::NFConnector>, conns: Arc<Connections>) -> Arc<Connections> {
    todo!()
}

pub fn addBroken(broken: metamodelica::List<BrokenEdge>, conns: Arc<Connections>) -> Arc<Connections> {
    todo!()
}

pub fn collectConnections(flatModel: Arc<FlatModel::NFFlatModel>, isDeleted: fn(Arc<ComponentRef::NFComponentRef>) -> bool) -> (Arc<FlatModel::NFFlatModel>, Arc<Connections>) {
    todo!()
}

pub fn collectFlows(flatModel: Arc<FlatModel::NFFlatModel>, conns: Arc<Connections>) -> Arc<Connections> {
    todo!()
}

pub fn makeConnections(lhsCref: Arc<ComponentRef::NFComponentRef>, lhsType: Arc<Type::NFType>, rhsCref: Arc<ComponentRef::NFComponentRef>, rhsType: Arc<Type::NFType>, source: Arc<DAE::ElementSource>, isDeleted: fn(Arc<ComponentRef::NFComponentRef>) -> bool, connections: metamodelica::List<Arc<Connection::NFConnection>>) -> metamodelica::List<Arc<Connection::NFConnection>> {
    todo!()
}

pub fn makeConnectors(cref: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>, source: Arc<DAE::ElementSource>) -> metamodelica::List<Arc<Connector::NFConnector>> {
    todo!()
}

pub fn split(conns: Arc<Connections>) -> Arc<Connections> {
    todo!()
}

pub fn connectCount(conn: Arc<Connector::NFConnector>, connectCounts: UnorderedMap::UnorderedMap<i32, Arc<Connector::NFConnector>>) -> i32 {
    todo!()
}

pub fn scalarize(conns: Arc<Connections>, keepSingleConnectedArrays: bool) -> Arc<Connections> {
    todo!()
}

pub fn analyseArrayConnections(conns: Arc<Connections>) -> UnorderedMap::UnorderedMap<i32, Arc<Connector::NFConnector>> {
    todo!()
}

pub fn analyseArrayConnector(conn: Arc<Connector::NFConnector>, connectCounts: UnorderedMap::UnorderedMap<i32, Arc<Connector::NFConnector>>) -> () {
    todo!()
}

pub fn toString(conns: Arc<Connections>) -> String {
    todo!()
}

pub fn toStringList(conns: Arc<Connections>) -> metamodelica::List<metamodelica::List<String>> {
    todo!()
}


