// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFConnection as Connection;
use crate::NFConnector as Connector;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Error;
use openmodelica_util::List;

pub struct CONNECTION {
    pub lhs: Arc<Connector::NFConnector>,
    pub rhs: Arc<Connector::NFConnector>,
}

pub type NFConnection = CONNECTION;
pub fn split(conn: Arc<Connection>) -> metamodelica::List<Arc<Connection>> {
    todo!()
}

pub fn scalarize(conn: Arc<Connection>) -> metamodelica::List<Arc<Connection>> {
    todo!()
}

pub fn scalarizePrefix(conn: Arc<Connection>) -> metamodelica::List<Arc<Connection>> {
    todo!()
}

pub fn toString(conn: Arc<Connection>) -> String {
    todo!()
}

fn checkBalance(leftConnectors: metamodelica::List<Arc<Connector::NFConnector>>, rightConnectors: metamodelica::List<Arc<Connector::NFConnector>>, conn: Arc<Connection>) -> () {
    todo!()
}


