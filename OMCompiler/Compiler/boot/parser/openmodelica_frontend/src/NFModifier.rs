// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::NFBinding as Binding;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFSubscript as Subscript;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::List;

pub mod ModTable {
    use super::*;
    pub type ConflictFunc = fn(i32, i32, String) -> i32;

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

    pub fn getOpt(tree: Arc<Tree>, key: String) -> Option<i32> {
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

}

pub mod Modifier {
    use super::*;
    pub enum Modifier {
        MODIFIER {
            name: String,
            finalPrefix: SCode::Final,
            eachPrefix: SCode::Each,
            binding: Arc<Binding::NFBinding>,
            subModifiers: Arc<BaseAvlTree::Tree>,
            info: SourceInfo,
        },
        REDECLARE {
            finalPrefix: SCode::Final,
            eachPrefix: SCode::Each,
            element: Arc<InstNode::InstNode>,
            innerMod: Arc<Modifier>,
            outerMod: Arc<Modifier>,
            constrainingMod: Arc<Modifier>,
            propagatedSubs: metamodelica::List<Arc<Subscript::NFSubscript>>,
        },
        NOMOD,
    }
    pub use Modifier::*;
    pub fn create(r#mod: Arc<SCode::Mod>, name: String, modScope: Arc<ModifierScope::ModifierScope>, scope: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        todo!()
    }

    pub fn createConstrainingMod(element: Arc<SCode::Element>, scope: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        todo!()
    }

    pub fn stripSCodeMod(elem: Arc<SCode::Element>) -> (Arc<SCode::Element>, Arc<SCode::Mod>) {
        todo!()
    }

    pub fn fromElement(element: Arc<SCode::Element>, scope: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        todo!()
    }

    pub fn patchElementModFinal(prefixes: Arc<SCode::Prefixes>, info: SourceInfo, r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
        todo!()
    }

    pub fn lookupModifier(modName: String, modifier: Arc<Modifier>) -> Arc<Modifier> {
        todo!()
    }

    pub fn name(modifier: Arc<Modifier>) -> String {
        todo!()
    }

    pub fn info(modifier: Arc<Modifier>) -> SourceInfo {
        todo!()
    }

    pub fn hasBinding(modifier: Arc<Modifier>) -> bool {
        todo!()
    }

    pub fn binding(modifier: Arc<Modifier>) -> Arc<Binding::NFBinding> {
        todo!()
    }

    pub fn setBinding(binding: Arc<Binding::NFBinding>, modifier: Arc<Modifier>) -> Arc<Modifier> {
        todo!()
    }

    pub fn merge(outerMod: Arc<Modifier>, innerMod: Arc<Modifier>, name: String) -> Arc<Modifier> {
        todo!()
    }

    pub fn propagate(r#mod: Arc<Modifier>, origin: Arc<InstNode::InstNode>, parent: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        todo!()
    }

    pub fn propagateSubs(r#mod: Arc<Modifier>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Modifier> {
        todo!()
    }

    pub fn propagateBinding(r#mod: Arc<Modifier>, origin: Arc<InstNode::InstNode>, parent: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        todo!()
    }

    pub fn propagateSubMod(name: String, submod: Arc<Modifier>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Modifier> {
        todo!()
    }

    pub fn isEmpty(r#mod: Arc<Modifier>) -> bool {
        todo!()
    }

    pub fn isRedeclare(r#mod: Arc<Modifier>) -> bool {
        todo!()
    }

    pub fn toList(r#mod: Arc<Modifier>) -> metamodelica::List<Arc<Modifier>> {
        todo!()
    }

    pub fn isEach(r#mod: Arc<Modifier>) -> bool {
        todo!()
    }

    pub fn isFinal(r#mod: Arc<Modifier>) -> bool {
        todo!()
    }

    pub fn map(r#mod: Arc<Modifier>, func: fn(String, Arc<Modifier>) -> Arc<Modifier>) -> Arc<Modifier> {
        todo!()
    }

    pub fn toString(r#mod: Arc<Modifier>, printName: bool) -> String {
        todo!()
    }

    pub fn toFlatStreamList(modifiers: metamodelica::List<Arc<Modifier>>, format: BaseModelica::OutputFormat, s: IOStream::IOStream, delimiter: String) -> IOStream::IOStream {
        todo!()
    }

    pub fn toFlatStream(r#mod: Arc<Modifier>, format: BaseModelica::OutputFormat, s: IOStream::IOStream, printName: bool) -> IOStream::IOStream {
        todo!()
    }

    pub fn toFlatString(r#mod: Arc<Modifier>, format: BaseModelica::OutputFormat, printName: bool) -> String {
        todo!()
    }

    fn createSubMod(subMod: Arc<SCode::SubMod>, modScope: Arc<ModifierScope::ModifierScope>, scope: Arc<InstNode::InstNode>) -> Arc<Modifier> {
        todo!()
    }

    fn checkFinalOverride(innerFinal: SCode::Final, outerMod: Arc<Modifier>, innerInfo: SourceInfo) -> () {
        todo!()
    }

    fn mergeLocal(mod1: Arc<Modifier>, mod2: Arc<Modifier>, name: String, scope: Arc<ModifierScope::ModifierScope>, prefix: metamodelica::List<String>) -> Arc<Modifier> {
        todo!()
    }

}

pub mod ModifierScope {
    use super::*;
    pub enum ModifierScope {
        COMPONENT {
            name: String,
        },
        CLASS {
            name: String,
        },
        EXTENDS {
            path: Arc<Absyn::Path>,
        },
    }
    pub use ModifierScope::*;
    pub fn fromElement(element: Arc<SCode::Element>) -> Arc<ModifierScope> {
        todo!()
    }

    pub fn name(scope: Arc<ModifierScope>) -> String {
        todo!()
    }

    pub fn isClass(scope: Arc<ModifierScope>) -> bool {
        todo!()
    }

    pub fn toString(scope: Arc<ModifierScope>) -> String {
        todo!()
    }

}

