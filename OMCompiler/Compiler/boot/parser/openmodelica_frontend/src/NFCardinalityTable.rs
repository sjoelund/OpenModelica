// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFExpression as Expression;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub type Table = UnorderedMap::UnorderedMap<i32, String>;

pub fn addConnector(conn: Arc<Connector::NFConnector>, table: UnorderedMap::UnorderedMap<i32, String>) -> () {
    todo!()
}

pub fn emptyCardinalityTable(size: i32) -> UnorderedMap::UnorderedMap<i32, String> {
    todo!()
}

pub fn evaluateCardinality(arg: Arc<Expression::NFExpression>, table: UnorderedMap::UnorderedMap<i32, String>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn fromConnections(conns: Arc<Connections::NFConnections>) -> UnorderedMap::UnorderedMap<i32, String> {
    todo!()
}

pub fn print(table: UnorderedMap::UnorderedMap<i32, String>) -> () {
    todo!()
}

