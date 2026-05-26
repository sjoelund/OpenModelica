// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::Dump;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Mutable;
use openmodelica_util::UnorderedMap;

pub type ConflictFunc = fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32;

pub struct ENTRY {
    pub hasMatch: bool,
    pub r#mod: Arc<SCode::Mod>,
}

pub type Entry = ENTRY;

pub type EntryTable = UnorderedMap::UnorderedMap<Entry, String>;

pub mod EntryTree {
    use super::*;
    pub type ConflictFunc = fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32;

    pub type Key = i32;

    pub enum Tree {
        NODE {
            key: Arc<Absyn::ComponentRef>,
            value: i32,
            height: i32,
            left: Arc<Tree>,
            right: Arc<Tree>,
        },
        LEAF {
            key: Arc<Absyn::ComponentRef>,
            value: i32,
        },
        EMPTY,
    }
    pub use Tree::*;

    pub type Value = i32;

    pub type ValueNode = Arc<Absyn::ComponentRef>;

    pub fn add(inTree: Arc<Tree>, inKey: Arc<Absyn::ComponentRef>, inValue: i32, conflictFunc: fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32) -> Arc<Tree> {
        todo!()
    }

    pub fn addConflictFail(newValue: i32, oldValue: i32, key: Arc<Absyn::ComponentRef>) -> i32 {
        todo!()
    }

    pub fn addConflictKeep(newValue: i32, oldValue: i32, key: Arc<Absyn::ComponentRef>) -> i32 {
        todo!()
    }

    pub fn addConflictReplace(newValue: i32, oldValue: i32, key: Arc<Absyn::ComponentRef>) -> i32 {
        todo!()
    }

    pub fn addList(tree: Arc<Tree>, inValues: metamodelica::List<(i32, i32)>, conflictFunc: fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32) -> Arc<Tree> {
        todo!()
    }

    pub fn addUpdate(tree: Arc<Tree>, key: Arc<Absyn::ComponentRef>, r#fn: fn(Option<i32>) -> i32) -> Arc<Tree> {
        todo!()
    }

    fn balance(inTree: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    fn calculateBalance(inNode: Arc<Tree>) -> i32 {
        todo!()
    }

    pub fn fold<FT>(inTree: Arc<Tree>, inFunc: fn(Arc<Absyn::ComponentRef>, i32, FT) -> FT, inStartValue: FT) -> FT {
        todo!()
    }

    pub fn foldCond<FT>(tree: Arc<Tree>, foldFunc: fn(Arc<Absyn::ComponentRef>, i32, FT) -> (FT, bool), value: FT) -> FT {
        todo!()
    }

    pub fn fold_2<FT1, FT2>(tree: Arc<Tree>, foldFunc: fn(Arc<Absyn::ComponentRef>, i32, FT1, FT2) -> (FT1, FT2), foldArg1: FT1, foldArg2: FT2) -> (FT1, FT2) {
        todo!()
    }

    pub fn forEach(tree: Arc<Tree>, func: fn(Arc<Absyn::ComponentRef>, i32) -> ()) -> () {
        todo!()
    }

    pub fn fromList(inValues: metamodelica::List<(i32, i32)>, conflictFunc: fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32) -> Arc<Tree> {
        todo!()
    }

    pub fn get(tree: Arc<Tree>, key: Arc<Absyn::ComponentRef>) -> i32 {
        todo!()
    }

    pub fn getOpt(tree: Arc<Tree>, key: Arc<Absyn::ComponentRef>) -> Option<i32> {
        todo!()
    }

    pub fn hasKey(inTree: Arc<Tree>, inKey: Arc<Absyn::ComponentRef>) -> bool {
        todo!()
    }

    fn height(inNode: Arc<Tree>) -> i32 {
        todo!()
    }

    pub fn intersection() -> () {
        todo!()
    }

    pub fn isEmpty(tree: Arc<Tree>) -> bool {
        todo!()
    }

    pub fn join(tree: Arc<Tree>, treeToJoin: Arc<Tree>, conflictFunc: fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32) -> Arc<Tree> {
        todo!()
    }

    pub fn keyCompare(inKey1: Arc<Absyn::ComponentRef>, inKey2: Arc<Absyn::ComponentRef>) -> i32 {
        todo!()
    }

    pub fn keyStr(inKey: Arc<Absyn::ComponentRef>) -> String {
        todo!()
    }

    pub fn listKeys(tree: Arc<Tree>, lst: metamodelica::List<Arc<Absyn::ComponentRef>>) -> metamodelica::List<Arc<Absyn::ComponentRef>> {
        todo!()
    }

    pub fn listKeysReverse(inTree: Arc<Tree>, lst: metamodelica::List<Arc<Absyn::ComponentRef>>) -> metamodelica::List<Arc<Absyn::ComponentRef>> {
        todo!()
    }

    pub fn listValues(tree: Arc<Tree>, lst: metamodelica::List<i32>) -> metamodelica::List<i32> {
        todo!()
    }

    pub fn map(inTree: Arc<Tree>, inFunc: fn(Arc<Absyn::ComponentRef>, i32) -> i32) -> Arc<Tree> {
        todo!()
    }

    pub fn mapFold<FT>(inTree: Arc<Tree>, inFunc: fn(Arc<Absyn::ComponentRef>, i32, FT) -> (i32, FT), inStartValue: FT) -> (Arc<Tree>, FT) {
        todo!()
    }

    pub fn new() -> Arc<Tree> {
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

    pub fn smallestKey(tree: Arc<Tree>) -> Arc<Absyn::ComponentRef> {
        todo!()
    }

    pub fn toList(inTree: Arc<Tree>, lst: metamodelica::List<(i32, i32)>) -> metamodelica::List<(i32, i32)> {
        todo!()
    }

    pub fn update(tree: Arc<Tree>, key: Arc<Absyn::ComponentRef>, value: i32) -> Arc<Tree> {
        todo!()
    }

    pub fn valueStr(inValue: i32) -> String {
        todo!()
    }

}

pub type Key = i32;

pub enum Tree {
    NODE {
        key: Arc<Absyn::ComponentRef>,
        value: i32,
        height: i32,
        left: Arc<Tree>,
        right: Arc<Tree>,
    },
    LEAF {
        key: Arc<Absyn::ComponentRef>,
        value: i32,
    },
    EMPTY,
}
pub use Tree::*;

pub type Value = i32;

pub type ValueNode = Arc<Absyn::ComponentRef>;

pub fn add(inTree: Arc<Tree>, inKey: Arc<Absyn::ComponentRef>, inValue: i32, conflictFunc: fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn addConflictFail(newValue: i32, oldValue: i32, key: Arc<Absyn::ComponentRef>) -> i32 {
    todo!()
}

pub fn addConflictKeep(newValue: i32, oldValue: i32, key: Arc<Absyn::ComponentRef>) -> i32 {
    todo!()
}

pub fn addConflictReplace(newValue: i32, oldValue: i32, key: Arc<Absyn::ComponentRef>) -> i32 {
    todo!()
}

pub fn addList(tree: Arc<Tree>, inValues: metamodelica::List<(i32, i32)>, conflictFunc: fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn addUpdate(tree: Arc<Tree>, key: Arc<Absyn::ComponentRef>, r#fn: fn(Option<i32>) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn appendBreaksInNode(node: Arc<InstNode::InstNode>, tree: Arc<Tree>) -> (Arc<Tree>, metamodelica::List<Mutable::Mutable<Entry>>) {
    todo!()
}

fn balance(inTree: Arc<Tree>) -> Arc<Tree> {
    todo!()
}

fn calculateBalance(inNode: Arc<Tree>) -> i32 {
    todo!()
}

pub fn checkUnmatchedBreaks(entries: metamodelica::List<Mutable::Mutable<Entry>>) -> () {
    todo!()
}

pub fn fold<FT>(inTree: Arc<Tree>, inFunc: fn(Arc<Absyn::ComponentRef>, i32, FT) -> FT, inStartValue: FT) -> FT {
    todo!()
}

pub fn foldCond<FT>(tree: Arc<Tree>, foldFunc: fn(Arc<Absyn::ComponentRef>, i32, FT) -> (FT, bool), value: FT) -> FT {
    todo!()
}

pub fn fold_2<FT1, FT2>(tree: Arc<Tree>, foldFunc: fn(Arc<Absyn::ComponentRef>, i32, FT1, FT2) -> (FT1, FT2), foldArg1: FT1, foldArg2: FT2) -> (FT1, FT2) {
    todo!()
}

pub fn forEach(tree: Arc<Tree>, func: fn(Arc<Absyn::ComponentRef>, i32) -> ()) -> () {
    todo!()
}

pub fn fromList(inValues: metamodelica::List<(i32, i32)>, conflictFunc: fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn get(tree: Arc<Tree>, key: Arc<Absyn::ComponentRef>) -> i32 {
    todo!()
}

pub fn getOpt(tree: Arc<Tree>, key: Arc<Absyn::ComponentRef>) -> Option<i32> {
    todo!()
}

pub fn hasKey(inTree: Arc<Tree>, inKey: Arc<Absyn::ComponentRef>) -> bool {
    todo!()
}

fn height(inNode: Arc<Tree>) -> i32 {
    todo!()
}

pub fn intersection() -> () {
    todo!()
}

pub fn isConnectBroken(lhs: Arc<Absyn::ComponentRef>, rhs: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, connectBreaks: Arc<Tree>) -> bool {
    todo!()
}

pub fn isEmpty(tree: Arc<Tree>) -> bool {
    todo!()
}

pub fn join(tree: Arc<Tree>, treeToJoin: Arc<Tree>, conflictFunc: fn(i32, i32, Arc<Absyn::ComponentRef>) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn keyCompare(inKey1: Arc<Absyn::ComponentRef>, inKey2: Arc<Absyn::ComponentRef>) -> i32 {
    todo!()
}

pub fn keyStr(inKey: Arc<Absyn::ComponentRef>) -> String {
    todo!()
}

pub fn listKeys(tree: Arc<Tree>, lst: metamodelica::List<Arc<Absyn::ComponentRef>>) -> metamodelica::List<Arc<Absyn::ComponentRef>> {
    todo!()
}

pub fn listKeysReverse(inTree: Arc<Tree>, lst: metamodelica::List<Arc<Absyn::ComponentRef>>) -> metamodelica::List<Arc<Absyn::ComponentRef>> {
    todo!()
}

pub fn listValues(tree: Arc<Tree>, lst: metamodelica::List<i32>) -> metamodelica::List<i32> {
    todo!()
}

pub fn map(inTree: Arc<Tree>, inFunc: fn(Arc<Absyn::ComponentRef>, i32) -> i32) -> Arc<Tree> {
    todo!()
}

pub fn mapFold<FT>(inTree: Arc<Tree>, inFunc: fn(Arc<Absyn::ComponentRef>, i32, FT) -> (i32, FT), inStartValue: FT) -> (Arc<Tree>, FT) {
    todo!()
}

pub fn new() -> Arc<Tree> {
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

pub fn smallestKey(tree: Arc<Tree>) -> Arc<Absyn::ComponentRef> {
    todo!()
}

pub fn toList(inTree: Arc<Tree>, lst: metamodelica::List<(i32, i32)>) -> metamodelica::List<(i32, i32)> {
    todo!()
}

pub fn update(tree: Arc<Tree>, key: Arc<Absyn::ComponentRef>, value: i32) -> Arc<Tree> {
    todo!()
}

pub fn valueStr(inValue: i32) -> String {
    todo!()
}

