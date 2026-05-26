// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::ElementSource;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Visibility;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Array;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

fn addConnectionToSets(c1: Arc<Connector::NFConnector>, c2: Arc<Connector::NFConnector>, csets: DisjointSets::Sets) -> DisjointSets::Sets {
    todo!()
}

fn addExpandableConnectorsToSets(conns: metamodelica::List<Arc<Connection::NFConnection>>, csets: DisjointSets::Sets) -> DisjointSets::Sets {
    todo!()
}

fn addNestedExpandableConnectorsToSets(c1: Arc<Connector::NFConnector>, c2: Arc<Connector::NFConnector>, csets: DisjointSets::Sets) -> DisjointSets::Sets {
    todo!()
}

fn addUndeclaredConnectorToSets(conn: Arc<Connection::NFConnection>, csets: DisjointSets::Sets) -> (Arc<Connection::NFConnection>, DisjointSets::Sets) {
    todo!()
}

fn augmentExpandableConnector(conn: Arc<Connector::NFConnector>, expandableSet: metamodelica::List<Arc<Connector::NFConnector>>, vars: metamodelica::List<Arc<Variable::NFVariable>>) -> metamodelica::List<Arc<Variable::NFVariable>> {
    todo!()
}

fn createVirtualVariables(connectorName: Arc<ComponentRef::NFComponentRef>, connectorType: Arc<Type::NFType>, info: SourceInfo, vars: metamodelica::List<Arc<Variable::NFVariable>>) -> metamodelica::List<Arc<Variable::NFVariable>> {
    todo!()
}

pub fn elaborate(flatModel: Arc<FlatModel::NFFlatModel>, connections: Arc<Connections::NFConnections>) -> (Arc<FlatModel::NFFlatModel>, Arc<Connections::NFConnections>) {
    todo!()
}

fn elaborateExpandableSet(set: metamodelica::List<Arc<Connector::NFConnector>>, vars: metamodelica::List<Arc<Variable::NFVariable>>) -> metamodelica::List<Arc<Variable::NFVariable>> {
    todo!()
}

fn getExpandableConnectorsInConnector(c1: Arc<Connector::NFConnector>) -> metamodelica::List<Arc<Connector::NFConnector>> {
    todo!()
}

fn hashConnector(conn: Arc<Connector::NFConnector>) -> i32 {
    todo!()
}

fn makeVirtualConnector(virtualConnector: Arc<Connector::NFConnector>, normalConnector: Arc<Connector::NFConnector>) -> Arc<Connector::NFConnector> {
    todo!()
}

fn markComponentPresent(node: Arc<InstNode::InstNode>) -> () {
    todo!()
}

fn sortConnections(conns: metamodelica::List<Arc<Connection::NFConnection>>) -> (metamodelica::List<Arc<Connection::NFConnection>>, metamodelica::List<Arc<Connection::NFConnection>>, metamodelica::List<Arc<Connection::NFConnection>>) {
    todo!()
}

fn updateExpandableConnection(conn: Arc<Connection::NFConnection>, conns: metamodelica::List<Arc<Connection::NFConnection>>) -> metamodelica::List<Arc<Connection::NFConnection>> {
    todo!()
}

fn updateExpandableConnector(conn: Arc<Connector::NFConnector>) -> (Arc<Connector::NFConnector>, Arc<Type::NFType>) {
    todo!()
}

fn updatePotentiallyPresentVariable(var: Arc<Variable::NFVariable>) -> Arc<Variable::NFVariable> {
    todo!()
}

fn updateUndeclaredConnection(conn: Arc<Connection::NFConnection>, conns: metamodelica::List<Arc<Connection::NFConnection>>) -> metamodelica::List<Arc<Connection::NFConnection>> {
    todo!()
}

