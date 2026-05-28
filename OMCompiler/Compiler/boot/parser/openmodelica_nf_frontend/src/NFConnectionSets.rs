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

use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnections::BrokenEdges;
use crate::NFConnector as Connector;
use openmodelica_util::DisjointSets;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub mod ConnectionSets {
    use super::*;
    pub fn EntryHash(mut entry: Entry) -> i32 {
        let mut hash: i32 = 0;
        hash = Connector::hash(entry.clone());
        hash
    }

    pub fn EntryEqual(mut entry1: Entry, mut entry2: Entry) -> bool {
        let mut isEqual: bool = false;
        isEqual = Connector::isEqual(entry1.clone(), entry2.clone());
        isEqual
    }

    pub fn EntryString(mut entry: Entry) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (Connector::toString(entry.clone())).clone();
        r#str
    }

    pub fn fromConnections(mut connections: Arc<Connections::NFConnections>) -> Result<Sets> {
        let mut sets: Sets = <Sets as ::std::default::Default>::default();
        sets = emptySets((connections.connections.clone().len() as i32) + (connections.flows.clone().len() as i32));
        if !(Flags::isSet(Flags::DISABLE_SINGLE_FLOW_EQ.clone())?) {
            sets = List::fold(connections.flows.clone(), (std::sync::Arc::new(addSingleConnector) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Sets) -> Result<Sets> + 'static>), sets.clone());
        }
        sets = List::fold1(connections.connections.clone(), (std::sync::Arc::new(addConnection) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connection::NFConnection>, Arc<metamodelica::List<Connections::BrokenEdge>>, Sets) -> Result<Sets> + 'static>), connections.broken.clone(), sets.clone());
        Ok(sets)
    }

    pub fn addScalarConnector(mut conn: Arc<Connector::NFConnector>, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        (sets, _) = add(conn.clone(), sets.clone())?;
        Ok(sets)
    }

    pub fn addConnector(mut conn: Arc<Connector::NFConnector>, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        sets = addList(Connector::scalarize(conn.clone())?, sets.clone())?;
        Ok(sets)
    }

    pub fn addSingleConnector(mut conn: Arc<Connector::NFConnector>, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        (sets, _) = find(conn.clone(), sets.clone())?;
        Ok(sets)
    }

    pub fn addConnection(mut connection: Arc<Connection::NFConnection>, mut broken: Arc<metamodelica::List<Connections::BrokenEdge>>, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        if !(broken.clone().is_empty()) && isBroken(connection.lhs.clone(), connection.rhs.clone(), broken.clone())? {
            return Ok(sets.clone());
        }
        sets = merge(connection.lhs.clone(), connection.rhs.clone(), sets.clone())?;
        Ok(sets)
    }

    pub fn isBroken(mut c1: Arc<Connector::NFConnector>, mut c2: Arc<Connector::NFConnector>, mut broken: Arc<metamodelica::List<Connections::BrokenEdge>>) -> Result<bool> {
        let mut b: bool = false;
        let mut cr1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut cr2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        cr1 = Connector::name(c1.clone());
        cr2 = Connector::name(c2.clone());
        for mut c in &*broken.clone() {
            let mut c = c.clone();
            if ComponentRef::isPrefix(c.lhs.clone(), cr1.clone())? && ComponentRef::isPrefix(c.rhs.clone(), cr2.clone())? || ComponentRef::isPrefix(c.lhs.clone(), cr2.clone())? && ComponentRef::isPrefix(c.rhs.clone(), cr1.clone())? {
                b = true;
                break;
            }
        }
        Ok(b)
    }

    pub type Entry = Arc<Connector::NFConnector>;

    pub type IndexTable = Arc<UnorderedMap::UnorderedMap<Arc<Connector::NFConnector>, i32>>;

    /// This is a disjoint sets data structure. The nodes are stored in an array of
    ///   Integers. The root elements of a set is given a negative value that
    ///   corresponds to its rank, while other elements are given positive values that
    ///   corresponds to the index of their parent in the array. The hashtable is used
    ///   to look up the array index of a entry, and is also used to store the entries.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn addList(mut entries: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut sets: Sets) -> Result<Sets> {
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
        let mut sets: Sets = <Sets as ::std::default::Default>::default();
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut sz: i32 = 0;
        sz = std::cmp::max(setCount.clone(), 3);
        nodes = arrayCreate(sz.clone(), -1);
        elements = UnorderedMap::new((std::sync::Arc::new(fnptr!(EntryHash, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(EntryEqual, Arc<Connector::NFConnector>, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Connector::NFConnector>) -> Result<bool> + 'static>), 1);
        sets = Sets { nodes: nodes.clone(), elements: elements.clone(), nodeCount: 0 };
        sets
    }

    pub fn extractSets(mut sets: Sets) -> (metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>, Sets) {
        let mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>;
        let mut assignedSets: Sets = <Sets as ::std::default::Default>::default();
        let mut nodes: metamodelica::Array<i32>;
        let mut set_idx: i32 = 0;
        let mut idx: i32 = 0;
        let mut entries: metamodelica::Array<(Arc<Connector::NFConnector>, i32)>;
        let mut e: Entry = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
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
            let __pa0 = ::match_deref::match_deref! { match &(oindex.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
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
        let mut updatedSets: Sets = <Sets as ::std::default::Default>::default();
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

    pub fn getEntry(mut entry: Entry, mut sets: Sets) -> Option<Arc<Connector::NFConnector>> {
        let mut outEntry: Option<Arc<Connector::NFConnector>> = None;
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

    pub fn printSets(mut sets: Sets) -> () {
        let mut nodes: metamodelica::Array<i32>;
        let mut entries: Arc<metamodelica::List<(Arc<Connector::NFConnector>, i32)>> = metamodelica::nil();
        let mut e: Entry = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
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
            println!("{}", (EntryString(e.clone())).clone());
            println!("{}", (literal!(" -> ")).clone());
            println!("{}", ArcStr::from(::std::format!("{}", nodes.borrow()[(i.clone()-1) as usize].clone())));
            println!("{}", (literal!("\n")).clone());
        }
        ()
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

