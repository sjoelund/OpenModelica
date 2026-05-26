// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFInstNode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::List;
use openmodelica_util::NFLookupTree;

pub type ConflictFunc = fn(i32, i32, String) -> i32;

pub struct ENTRY {
    pub entry: Arc<NFLookupTree::Entry::Entry>,
    pub node: Option<Arc<NFInstNode::InstNode::InstNode>>,
    pub children: metamodelica::List<Arc<Entry>>,
    pub ty: EntryType,
}

pub type Entry = ENTRY;

pub enum EntryType {
    DUPLICATE,
    REDECLARE,
    ENTRY,
}

pub type Key = i32;

pub enum Tree {
    NODE {
        key: String,
        value: i32,
        height: i32,
        left: Arc<Tree>,
        right: Arc<Tree>,
    },
    LEAF {
        key: String,
        value: i32,
    },
    EMPTY,
}
pub use Tree::*;

pub type Value = i32;

pub type ValueNode = String;

pub fn add(inTree: Arc<Tree>, inKey: String, inValue: i32, conflictFunc: fn(i32, i32, String) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn addConflictFail(newValue: i32, oldValue: i32, key: String) -> i32 {
    todo!()
}

pub fn addConflictKeep(newValue: i32, oldValue: i32, key: String) -> i32 {
    todo!()
}

pub fn addConflictReplace(newValue: i32, oldValue: i32, key: String) -> i32 {
    todo!()
}

pub fn addList(tree: Arc<Tree>, inValues: metamodelica::List<(i32, i32)>, conflictFunc: fn(i32, i32, String) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn addUpdate(tree: Arc<Tree>, key: String, r#fn: fn(Option<i32>) -> i32) -> Arc<Tree> {
    todo!()
}

fn balance(inTree: Arc<Tree>) -> Arc<Tree> {
    todo!()
}

fn calculateBalance(inNode: Arc<Tree>) -> i32 {
    todo!()
}

pub fn entryToList(entry: Arc<Entry>) -> metamodelica::List<Arc<Entry>> {
    todo!()
}

pub fn fold<FT>(inTree: Arc<Tree>, inFunc: fn(String, i32, FT) -> FT, inStartValue: FT) -> FT {
    todo!()
}

pub fn foldCond<FT>(tree: Arc<Tree>, foldFunc: fn(String, i32, FT) -> (FT, bool), value: FT) -> FT {
    todo!()
}

pub fn fold_2<FT1, FT2>(tree: Arc<Tree>, foldFunc: fn(String, i32, FT1, FT2) -> (FT1, FT2), foldArg1: FT1, foldArg2: FT2) -> (FT1, FT2) {
    todo!()
}

pub fn forEach(tree: Arc<Tree>, func: fn(String, i32) -> ()) -> () {
    todo!()
}

pub fn fromList(inValues: metamodelica::List<(i32, i32)>, conflictFunc: fn(i32, i32, String) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn get(tree: Arc<Tree>, key: String) -> i32 {
    todo!()
}

pub fn getLookupEntries(entry: Arc<Entry>) -> metamodelica::List<Arc<NFLookupTree::Entry::Entry>> {
    todo!()
}

pub fn getOpt(tree: Arc<Tree>, key: String) -> Option<i32> {
    todo!()
}

pub fn hasKey(inTree: Arc<Tree>, inKey: String) -> bool {
    todo!()
}

fn height(inNode: Arc<Tree>) -> i32 {
    todo!()
}

pub fn idExistsInEntry(id: Arc<NFLookupTree::Entry::Entry>, entry: Arc<Entry>) -> bool {
    todo!()
}

pub fn intersection() -> () {
    todo!()
}

pub fn isEmpty(tree: Arc<Tree>) -> bool {
    todo!()
}

pub fn join(tree: Arc<Tree>, treeToJoin: Arc<Tree>, conflictFunc: fn(i32, i32, String) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn keyCompare(inKey1: String, inKey2: String) -> i32 {
    todo!()
}

pub fn keyStr(inKey: String) -> String {
    todo!()
}

pub fn listKeys(tree: Arc<Tree>, lst: metamodelica::List<String>) -> metamodelica::List<String> {
    todo!()
}

pub fn listKeysReverse(inTree: Arc<Tree>, lst: metamodelica::List<String>) -> metamodelica::List<String> {
    todo!()
}

pub fn listValues(tree: Arc<Tree>, lst: metamodelica::List<i32>) -> metamodelica::List<i32> {
    todo!()
}

pub fn map(inTree: Arc<Tree>, inFunc: fn(String, i32) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn mapFold<FT>(inTree: Arc<Tree>, inFunc: fn(String, i32, FT) -> (i32, FT), inStartValue: FT) -> (Arc<Tree>, FT) {
    todo!()
}

pub fn new() -> Arc<Tree> {
    todo!()
}

pub fn newDuplicate(kept: Arc<NFLookupTree::Entry::Entry>, duplicate: Arc<NFLookupTree::Entry::Entry>) -> Arc<Entry> {
    todo!()
}

pub fn newEntry(lentry: Arc<NFLookupTree::Entry::Entry>) -> Arc<Entry> {
    todo!()
}

pub fn newRedeclare(entry: Arc<NFLookupTree::Entry::Entry>) -> Arc<Entry> {
    todo!()
}

pub fn printNodeStr(inNode: Arc<Tree>) -> String {
    todo!()
}

pub fn printTreeStr(inTree: Arc<Tree>) -> String {
    todo!()
}

fn printTreeStr2(inTree: Arc<Tree>, isLeft: bool, inIndent: String) -> String {
    todo!()
}

fn referenceEqOrEmpty(t1: Arc<Tree>, t2: Arc<Tree>) -> bool {
    todo!()
}

fn rotateLeft(inNode: Arc<Tree>) -> Arc<Tree> {
    todo!()
}

fn rotateRight(inNode: Arc<Tree>) -> Arc<Tree> {
    todo!()
}

pub fn setTreeLeftRight(orig: Arc<Tree>, left: Arc<Tree>, right: Arc<Tree>) -> Arc<Tree> {
    todo!()
}

pub fn smallestKey(tree: Arc<Tree>) -> String {
    todo!()
}

pub fn toList(inTree: Arc<Tree>, lst: metamodelica::List<(i32, i32)>) -> metamodelica::List<(i32, i32)> {
    todo!()
}

pub fn update(tree: Arc<Tree>, key: String, value: i32) -> Arc<Tree> {
    todo!()
}

pub fn valueStr(inValue: i32) -> String {
    todo!()
}

