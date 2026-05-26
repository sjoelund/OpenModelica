// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFExpression;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;

pub type ConflictFunc = fn(Arc<NFExpression::NFExpression>, Arc<NFExpression::NFExpression>, String) -> Arc<NFExpression::NFExpression>;

pub type Key = i32;

pub enum Tree {
    NODE {
        key: String,
        value: Arc<NFExpression::NFExpression>,
        height: i32,
        left: Arc<Tree>,
        right: Arc<Tree>,
    },
    LEAF {
        key: String,
        value: Arc<NFExpression::NFExpression>,
    },
    EMPTY,
}
pub use Tree::*;

pub type Value = i32;

pub type ValueNode = String;

pub fn add(inTree: Arc<Tree>, inKey: String, inValue: Arc<NFExpression::NFExpression>, conflictFunc: fn(Arc<NFExpression::NFExpression>, Arc<NFExpression::NFExpression>, String) -> Arc<NFExpression::NFExpression>) -> Arc<Tree> {
    todo!()
}

pub fn addConflictFail(newValue: Arc<NFExpression::NFExpression>, oldValue: Arc<NFExpression::NFExpression>, key: String) -> Arc<NFExpression::NFExpression> {
    todo!()
}

pub fn addConflictKeep(newValue: Arc<NFExpression::NFExpression>, oldValue: Arc<NFExpression::NFExpression>, key: String) -> Arc<NFExpression::NFExpression> {
    todo!()
}

pub fn addConflictReplace(newValue: Arc<NFExpression::NFExpression>, oldValue: Arc<NFExpression::NFExpression>, key: String) -> Arc<NFExpression::NFExpression> {
    todo!()
}

pub fn addList(tree: Arc<Tree>, inValues: metamodelica::List<(Arc<NFExpression::NFExpression>, String)>, conflictFunc: fn(Arc<NFExpression::NFExpression>, Arc<NFExpression::NFExpression>, String) -> Arc<NFExpression::NFExpression>) -> Arc<Tree> {
    todo!()
}

pub fn addUpdate(tree: Arc<Tree>, key: String, r#fn: fn(Option<Arc<NFExpression::NFExpression>>) -> Arc<NFExpression::NFExpression>) -> Arc<Tree> {
    todo!()
}

fn balance(inTree: Arc<Tree>) -> Arc<Tree> {
    todo!()
}

fn calculateBalance(inNode: Arc<Tree>) -> i32 {
    todo!()
}

pub fn fold<FT>(inTree: Arc<Tree>, inFunc: fn(String, Arc<NFExpression::NFExpression>, FT) -> FT, inStartValue: FT) -> FT {
    todo!()
}

pub fn foldCond<FT>(tree: Arc<Tree>, foldFunc: fn(String, Arc<NFExpression::NFExpression>, FT) -> (FT, bool), value: FT) -> FT {
    todo!()
}

pub fn fold_2<FT1, FT2>(tree: Arc<Tree>, foldFunc: fn(String, Arc<NFExpression::NFExpression>, FT1, FT2) -> (FT1, FT2), foldArg1: FT1, foldArg2: FT2) -> (FT1, FT2) {
    todo!()
}

pub fn forEach(tree: Arc<Tree>, func: fn(String, Arc<NFExpression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn fromList(inValues: metamodelica::List<(Arc<NFExpression::NFExpression>, String)>, conflictFunc: fn(Arc<NFExpression::NFExpression>, Arc<NFExpression::NFExpression>, String) -> Arc<NFExpression::NFExpression>) -> Arc<Tree> {
    todo!()
}

pub fn get(tree: Arc<Tree>, key: String) -> Arc<NFExpression::NFExpression> {
    todo!()
}

pub fn getOpt(tree: Arc<Tree>, key: String) -> Option<Arc<NFExpression::NFExpression>> {
    todo!()
}

pub fn hasKey(inTree: Arc<Tree>, inKey: String) -> bool {
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

pub fn join(tree: Arc<Tree>, treeToJoin: Arc<Tree>, conflictFunc: fn(Arc<NFExpression::NFExpression>, Arc<NFExpression::NFExpression>, String) -> Arc<NFExpression::NFExpression>) -> Arc<Tree> {
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

pub fn listValues(tree: Arc<Tree>, lst: metamodelica::List<Arc<NFExpression::NFExpression>>) -> metamodelica::List<Arc<NFExpression::NFExpression>> {
    todo!()
}

pub fn map(inTree: Arc<Tree>, inFunc: fn(String, Arc<NFExpression::NFExpression>) -> Arc<NFExpression::NFExpression>) -> Arc<Tree> {
    todo!()
}

pub fn mapFold<FT>(inTree: Arc<Tree>, inFunc: fn(String, Arc<NFExpression::NFExpression>, FT) -> (Arc<NFExpression::NFExpression>, FT), inStartValue: FT) -> (Arc<Tree>, FT) {
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

pub fn smallestKey(tree: Arc<Tree>) -> String {
    todo!()
}

pub fn toList(inTree: Arc<Tree>, lst: metamodelica::List<(Arc<NFExpression::NFExpression>, String)>) -> metamodelica::List<(Arc<NFExpression::NFExpression>, String)> {
    todo!()
}

pub fn update(tree: Arc<Tree>, key: String, value: Arc<NFExpression::NFExpression>) -> Arc<Tree> {
    todo!()
}

pub fn valueStr(inValue: Arc<NFExpression::NFExpression>) -> String {
    todo!()
}

