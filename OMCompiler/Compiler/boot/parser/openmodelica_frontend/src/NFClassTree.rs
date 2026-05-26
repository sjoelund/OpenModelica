// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::NFBuiltin;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFDuplicateTree as DuplicateTree;
use crate::NFImport as Import;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFLookup as Lookup;
use crate::NFModifier::Modifier;
use crate::NFRestriction as Restriction;
use crate::NFType::Type;
use crate::SCode;
use crate::SCodeDump;
use crate::SCodeUtil;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::List;
use openmodelica_util::Mutable;
use openmodelica_util::NFLookupTree as LookupTree;
use openmodelica_util::UnorderedMap;

pub mod ClassTree {
    use super::*;
    pub enum ClassTree {
        PARTIAL_TREE {
            tree: Arc<NFLookupTree::Tree>,
            classes: Vec<Arc<InstNode::InstNode>>,
            components: Vec<Arc<InstNode::InstNode>>,
            exts: Vec<Arc<InstNode::InstNode>>,
            imports: Vec<Arc<Import::NFImport>>,
            duplicates: Arc<NFDuplicateTree::Tree>,
        },
        EXPANDED_TREE {
            tree: Arc<NFLookupTree::Tree>,
            classes: Vec<Arc<InstNode::InstNode>>,
            components: Vec<Arc<InstNode::InstNode>>,
            exts: Vec<Arc<InstNode::InstNode>>,
            imports: Vec<Arc<Import::NFImport>>,
            duplicates: Arc<NFDuplicateTree::Tree>,
        },
        INSTANTIATED_TREE {
            tree: Arc<NFLookupTree::Tree>,
            classes: Vec<Mutable::Mutable<Arc<InstNode::InstNode>>>,
            components: Vec<Mutable::Mutable<Arc<InstNode::InstNode>>>,
            localComponents: metamodelica::List<i32>,
            exts: Vec<Arc<InstNode::InstNode>>,
            imports: Vec<Arc<Import::NFImport>>,
            duplicates: Arc<NFDuplicateTree::Tree>,
        },
        FLAT_TREE {
            tree: Arc<NFLookupTree::Tree>,
            classes: Vec<Arc<InstNode::InstNode>>,
            components: Vec<Arc<InstNode::InstNode>>,
            imports: Vec<Arc<Import::NFImport>>,
            duplicates: Arc<NFDuplicateTree::Tree>,
        },
        EMPTY_TREE,
    }
    pub use ClassTree::*;
    pub fn fromSCode(elements: metamodelica::List<Arc<SCode::Element>>, isClassExtends: bool, parent: Arc<InstNode::InstNode>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn initImports(tree: Arc<ClassTree>, parent: Arc<InstNode::InstNode>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn fromEnumeration(literals: metamodelica::List<Arc<SCode::Enum>>, enumType: Arc<NFType::NFType>, enumClass: Arc<InstNode::InstNode>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn addElementsToFlatTree(elements: metamodelica::List<Arc<InstNode::InstNode>>, tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn expand(tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn instantiate(clsNode: Arc<InstNode::InstNode>, instance: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>) -> (Arc<InstNode::InstNode>, Arc<InstNode::InstNode>, i32, i32) {
        todo!()
    }

    pub fn fromRecordConstructor(fields: metamodelica::List<Arc<InstNode::InstNode>>, out: Arc<InstNode::InstNode>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn clone(tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn mapRedeclareChains(tree: Arc<ClassTree>, func: fn(metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>) -> ()) -> () {
        todo!()
    }

    pub fn replaceDuplicates(tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn appendComponentsToInstTree(components: metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>, tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn appendComponentsToFlatTree(components: metamodelica::List<Arc<InstNode::InstNode>>, tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn flatten(tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn flattenElements(elements: Vec<Mutable::Mutable<Arc<InstNode::InstNode>>>, flatElements: Vec<Arc<InstNode::InstNode>>) -> () {
        todo!()
    }

    pub fn flattenElementsWithOffset(elements: Vec<Mutable::Mutable<Arc<InstNode::InstNode>>>, flatElements: Vec<Arc<InstNode::InstNode>>, offsets: Vec<i32>) -> () {
        todo!()
    }

    pub fn createFlatOffsets(elementCount: i32, duplicates: metamodelica::List<i32>) -> Vec<i32> {
        todo!()
    }

    pub fn flattenLookupTree(tree: Arc<NFLookupTree::Tree>, offsets: Vec<i32>) -> Arc<NFLookupTree::Tree> {
        todo!()
    }

    pub fn flattenLookupTree2(key: String, entry: Arc<NFLookupTree::Entry::Entry>, offsets: Vec<i32>) -> Arc<NFLookupTree::Entry::Entry> {
        todo!()
    }

    pub fn lookupElement(name: String, tree: Arc<ClassTree>) -> (Arc<InstNode::InstNode>, bool) {
        todo!()
    }

    pub fn lookupElementPtr(name: String, tree: Arc<ClassTree>) -> Mutable::Mutable<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn lookupElementsPtr(name: String, tree: Arc<ClassTree>) -> metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>> {
        todo!()
    }

    pub fn lookupComponentIndex(name: String, tree: Arc<ClassTree>) -> i32 {
        todo!()
    }

    pub fn nthComponent(index: i32, tree: Arc<ClassTree>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    pub fn mapClasses(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode>) -> () {
        todo!()
    }

    pub fn foldClasses<ArgT>(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
        todo!()
    }

    pub fn applyExtends(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>) -> ()) -> () {
        todo!()
    }

    pub fn mapExtends(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode>) -> () {
        todo!()
    }

    pub fn foldExtends<ArgT>(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
        todo!()
    }

    pub fn mapFoldExtends<ArgT>(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>, ArgT) -> (Arc<InstNode::InstNode>, ArgT), arg: ArgT) -> ArgT {
        todo!()
    }

    pub fn applyLocalComponents(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>) -> ()) -> () {
        todo!()
    }

    pub fn applyComponents(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>) -> ()) -> () {
        todo!()
    }

    pub fn foldComponents<ArgT>(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
        todo!()
    }

    pub fn findComponent(tree: Arc<ClassTree>, func: fn(Arc<InstNode::InstNode>) -> bool) -> Option<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn classCount(tree: Arc<ClassTree>) -> i32 {
        todo!()
    }

    pub fn componentCount(tree: Arc<ClassTree>) -> i32 {
        todo!()
    }

    pub fn extendsCount(tree: Arc<ClassTree>) -> i32 {
        todo!()
    }

    pub fn recursiveElementCount(tree: Arc<ClassTree>) -> i32 {
        todo!()
    }

    pub fn checkDuplicates(tree: Arc<ClassTree>) -> () {
        todo!()
    }

    pub fn checkDuplicates2(name: String, entry: Arc<NFDuplicateTree::Entry>, tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn isIdentical(tree1: Arc<ClassTree>, tree2: Arc<ClassTree>) -> bool {
        todo!()
    }

    pub fn getRedeclaredNode(name: String, tree: Arc<ClassTree>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    pub fn setClassExtends(extNode: Arc<InstNode::InstNode>, tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn enumerateComponents(tree: Arc<ClassTree>) -> metamodelica::List<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn enumerateComponents2(name: String, entry: Arc<NFLookupTree::Entry::Entry>, comps: Vec<Arc<InstNode::InstNode>>, components: metamodelica::List<Arc<InstNode::InstNode>>) -> metamodelica::List<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getClasses(tree: Arc<ClassTree>) -> Vec<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getExtends(tree: Arc<ClassTree>) -> Vec<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getComponents(tree: Arc<ClassTree>) -> Vec<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getImports(tree: Arc<ClassTree>) -> Vec<Arc<Import::NFImport>> {
        todo!()
    }

    pub fn isEmptyTree(tree: Arc<ClassTree>) -> bool {
        todo!()
    }

    pub fn appendClasses(clsNodes: metamodelica::List<Arc<InstNode::InstNode>>, tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    pub fn appendClasses2(clsNodes: metamodelica::List<Arc<InstNode::InstNode>>, tree: Arc<NFLookupTree::Tree>, classes: Vec<Arc<InstNode::InstNode>>) -> (Arc<NFLookupTree::Tree>, Vec<Arc<InstNode::InstNode>>) {
        todo!()
    }

    pub fn replaceClass(node: Arc<InstNode::InstNode>, tree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    fn instExtendsComps(extNode: Arc<InstNode::InstNode>, comps: Vec<Mutable::Mutable<Arc<InstNode::InstNode>>>, index: i32) -> i32 {
        todo!()
    }

    fn getDuplicates(tree: Arc<ClassTree>) -> Arc<NFDuplicateTree::Tree> {
        todo!()
    }

    fn lookupTree(ctree: Arc<ClassTree>) -> Arc<NFLookupTree::Tree> {
        todo!()
    }

    fn setLookupTree(ltree: Arc<NFLookupTree::Tree>, ctree: Arc<ClassTree>) -> Arc<ClassTree> {
        todo!()
    }

    fn addLocalElement(name: String, entry: Arc<NFLookupTree::Entry::Entry>, classTree: Arc<ClassTree>, tree: Arc<NFLookupTree::Tree>) -> Arc<NFLookupTree::Tree> {
        todo!()
    }

    fn addLocalElementConflict(newEntry: Arc<NFLookupTree::Entry::Entry>, oldEntry: Arc<NFLookupTree::Entry::Entry>, name: String, classTree: Arc<ClassTree>) -> Arc<NFLookupTree::Entry::Entry> {
        todo!()
    }

    fn findLocalConflictElement(entry: Arc<NFLookupTree::Entry::Entry>, classTree: Arc<ClassTree>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    fn addEnumConflict(newEntry: Arc<NFLookupTree::Entry::Entry>, oldEntry: Arc<NFLookupTree::Entry::Entry>, name: String, literal: Arc<InstNode::InstNode>) -> Arc<NFLookupTree::Entry::Entry> {
        todo!()
    }

    fn addImport(imp: Arc<Import::NFImport>, index: i32, tree: Arc<NFLookupTree::Tree>, imports: Vec<Arc<Import::NFImport>>) -> Arc<NFLookupTree::Tree> {
        todo!()
    }

    fn addImportConflict(newEntry: Arc<NFLookupTree::Entry::Entry>, oldEntry: Arc<NFLookupTree::Entry::Entry>, name: String, imports: Vec<Arc<Import::NFImport>>) -> Arc<NFLookupTree::Entry::Entry> {
        todo!()
    }

    fn addDuplicate(name: String, duplicateEntry: Arc<NFLookupTree::Entry::Entry>, keptEntry: Arc<NFLookupTree::Entry::Entry>, duplicates: Mutable::Mutable<Arc<NFDuplicateTree::Tree>>) -> Mutable::Mutable<Arc<NFDuplicateTree::Tree>> {
        todo!()
    }

    fn addDuplicateConflict(newEntry: Arc<NFDuplicateTree::Entry>, oldEntry: Arc<NFDuplicateTree::Entry>, name: String) -> Arc<NFDuplicateTree::Entry> {
        todo!()
    }

    fn resolveEntry(entry: Arc<NFLookupTree::Entry::Entry>, tree: Arc<ClassTree>) -> (Arc<InstNode::InstNode>, bool) {
        todo!()
    }

    fn resolveEntryPtr(entry: Arc<NFLookupTree::Entry::Entry>, tree: Arc<ClassTree>) -> Mutable::Mutable<Arc<InstNode::InstNode>> {
        todo!()
    }

    fn resolveDuplicateEntriesPtr(entry: Arc<NFDuplicateTree::Entry>, tree: Arc<ClassTree>, elements: metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>) -> metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>> {
        todo!()
    }

    fn resolveClass(index: i32, tree: Arc<ClassTree>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    fn resolveComponent(index: i32, tree: Arc<ClassTree>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    fn resolveImport(index: i32, tree: Arc<ClassTree>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    fn countElements(elements: metamodelica::List<Arc<SCode::Element>>) -> (i32, i32, i32) {
        todo!()
    }

    fn countInheritedElements(extendsNode: Arc<InstNode::InstNode>, classCount: i32, componentCount: i32) -> (i32, i32) {
        todo!()
    }

    fn expandExtends(extendsNode: Arc<InstNode::InstNode>, tree: Arc<NFLookupTree::Tree>, classOffset: i32, componentOffset: i32, duplicates: Mutable::Mutable<Arc<NFDuplicateTree::Tree>>) -> Arc<NFLookupTree::Tree> {
        todo!()
    }

    fn addInheritedElement(name: String, entry: Arc<NFLookupTree::Entry::Entry>, classOffset: i32, componentOffset: i32, conflictFunc: fn(i32, i32, String) -> i32, tree: Arc<NFLookupTree::Tree>) -> Arc<NFLookupTree::Tree> {
        todo!()
    }

    fn addInheritedElementConflict(newEntry: Arc<NFLookupTree::Entry::Entry>, oldEntry: Arc<NFLookupTree::Entry::Entry>, name: String, duplicates: Mutable::Mutable<Arc<NFDuplicateTree::Tree>>, extDuplicates: Arc<NFDuplicateTree::Tree>) -> Arc<NFLookupTree::Entry::Entry> {
        todo!()
    }

    fn offsetDuplicates(name: String, entry: Arc<NFDuplicateTree::Entry>, classOffset: i32, componentOffset: i32) -> Arc<NFDuplicateTree::Entry> {
        todo!()
    }

    fn offsetDuplicate(entry: Arc<NFLookupTree::Entry::Entry>, classOffset: i32, componentOffset: i32) -> Arc<NFLookupTree::Entry::Entry> {
        todo!()
    }

    fn joinDuplicates(newEntry: Arc<NFDuplicateTree::Entry>, oldEntry: Arc<NFDuplicateTree::Entry>, name: String) -> Arc<NFDuplicateTree::Entry> {
        todo!()
    }

    fn enumerateDuplicates(duplicates: Arc<NFDuplicateTree::Tree>) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
        todo!()
    }

    fn enumerateDuplicates2(name: String, entry: Arc<NFDuplicateTree::Entry>, classes: metamodelica::List<i32>, components: metamodelica::List<i32>) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
        todo!()
    }

    fn enumerateDuplicates3(entry: Arc<NFDuplicateTree::Entry>, classes: metamodelica::List<i32>, components: metamodelica::List<i32>) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
        todo!()
    }

    fn enumerateDuplicates4(entry: Arc<NFLookupTree::Entry::Entry>, classes: metamodelica::List<i32>, components: metamodelica::List<i32>) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
        todo!()
    }

    fn mapRedeclareChain(name: String, entry: Arc<NFDuplicateTree::Entry>, func: fn(metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>) -> (), tree: Arc<ClassTree>) -> Arc<NFDuplicateTree::Entry> {
        todo!()
    }

    fn getRedeclareChain(entry: Arc<NFDuplicateTree::Entry>, tree: Arc<ClassTree>, chain: metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>) -> metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>> {
        todo!()
    }

    fn replaceDuplicates2(name: String, entry: Arc<NFDuplicateTree::Entry>, tree: Arc<ClassTree>) -> (Arc<NFDuplicateTree::Entry>, Arc<ClassTree>) {
        todo!()
    }

    fn replaceDuplicates3(entry: Arc<NFDuplicateTree::Entry>, node: Arc<InstNode::InstNode>) -> Arc<NFDuplicateTree::Entry> {
        todo!()
    }

    fn linkInnerOuter(outerNode: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    fn checkOuterClass(outerCls: Arc<InstNode::InstNode>) -> () {
        todo!()
    }

    fn getBreakModsInExtend(extendsNode: Arc<InstNode::InstNode>) -> metamodelica::List<Arc<SCode::SubMod>> {
        todo!()
    }

    fn breakComponents(node: Arc<InstNode::InstNode>, components: Vec<Mutable::Mutable<Arc<InstNode::InstNode>>>, tree: Arc<NFLookupTree::Tree>, duplicates: Arc<NFDuplicateTree::Tree>) -> () {
        todo!()
    }

    fn checkIsBreakable(node: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>, info: SourceInfo) -> () {
        todo!()
    }

}

pub type LookupEntry = Arc<NFLookupTree::Entry::Entry>;

pub type LookupTable = UnorderedMap::UnorderedMap<Arc<NFLookupTree::Entry::Entry>, String>;

