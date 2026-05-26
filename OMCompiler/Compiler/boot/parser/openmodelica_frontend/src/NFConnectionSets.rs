// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnections::BrokenEdges;
use crate::NFConnector as Connector;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;
use openmodelica_util::DisjointSets;
use openmodelica_util::Flags;
use openmodelica_util::List;
use openmodelica_util::UnorderedMap;

pub mod ConnectionSets {
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

    pub fn addConnection(connection: Arc<Connection::NFConnection>, broken: metamodelica::List<NFConnections::BrokenEdge>, sets: DisjointSets::Sets) -> DisjointSets::Sets {
        todo!()
    }

    pub fn addConnector(conn: Arc<Connector::NFConnector>, sets: DisjointSets::Sets) -> DisjointSets::Sets {
        todo!()
    }

    pub fn addList(entries: metamodelica::List<i32>, sets: Sets) -> Sets {
        todo!()
    }

    pub fn addScalarConnector(conn: Arc<Connector::NFConnector>, sets: DisjointSets::Sets) -> DisjointSets::Sets {
        todo!()
    }

    pub fn addSingleConnector(conn: Arc<Connector::NFConnector>, sets: DisjointSets::Sets) -> DisjointSets::Sets {
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

    pub fn fromConnections(connections: Arc<Connections::NFConnections>) -> DisjointSets::Sets {
        todo!()
    }

    pub fn getNodeCount(sets: Sets) -> i32 {
        todo!()
    }

    pub fn isBroken(c1: Arc<Connector::NFConnector>, c2: Arc<Connector::NFConnector>, broken: metamodelica::List<NFConnections::BrokenEdge>) -> bool {
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

