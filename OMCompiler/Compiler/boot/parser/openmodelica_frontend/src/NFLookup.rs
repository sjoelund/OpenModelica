// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynToSCode;
use crate::AbsynUtil;
use crate::BackendInterface;
use crate::Dump;
use crate::NFAttributes as Attributes;
use crate::NFBuiltin;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFInstNode::NodeTree;
use crate::NFLookupState::LookupState;
use crate::NFModifier as Modifier;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ErrorTypes;
use openmodelica_util::Global;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;

pub enum MatchType {
    FOUND,
    NOT_FOUND,
    PARTIAL,
}

pub fn fixExternalObjectCall(node: Arc<InstNode::InstNode>, cref: Arc<ComponentRef::NFComponentRef>, state: Arc<LookupState::LookupState>) -> (Arc<ComponentRef::NFComponentRef>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn fixTypenameState(component: Arc<InstNode::InstNode>, state: Arc<LookupState::LookupState>, context: i32) -> Arc<LookupState::LookupState> {
    todo!()
}

pub fn generateInner(outerNode: Arc<InstNode::InstNode>, topScope: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn loadLibrary(name: String, scope: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn loadLibrary_work(name: String, scope: Arc<InstNode::InstNode>) -> String {
    todo!()
}

pub fn lookupBaseClassName(name: Arc<Absyn::Path>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> metamodelica::List<Arc<InstNode::InstNode>> {
    todo!()
}

pub fn lookupClassName(name: Arc<Absyn::Path>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo, checkAccessViolations: bool) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn lookupComponent(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn lookupConnector(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn lookupCref(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupCrefInNode(cref: Arc<Absyn::ComponentRef>, node: Arc<InstNode::InstNode>, foundCref: Arc<ComponentRef::NFComponentRef>, foundScope: Arc<InstNode::InstNode>, state: Arc<LookupState::LookupState>, context: i32) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupCrefWithError(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo, errMsg: ErrorTypes::Message) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupFirstIdent(name: String, scope: Arc<InstNode::InstNode>, context: i32) -> (Arc<InstNode::InstNode>, Arc<LookupState::LookupState>, bool) {
    todo!()
}

pub fn lookupFunctionName(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn lookupFunctionNameSilent(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn lookupImport(name: Arc<Absyn::Path>, scope: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn lookupInner(outerNode: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn lookupIterator(name: String, iterators: metamodelica::List<Arc<InstNode::InstNode>>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn lookupLocalComponent(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn lookupLocalCref(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupLocalName(name: Arc<Absyn::Path>, node: Arc<InstNode::InstNode>, state: Arc<LookupState::LookupState>, context: i32, checkAccessViolations: bool, selfReference: bool) -> (Arc<InstNode::InstNode>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupLocalNames(name: Arc<Absyn::Path>, scope: Arc<InstNode::InstNode>, nodes: metamodelica::List<Arc<InstNode::InstNode>>, state: Arc<LookupState::LookupState>, context: i32, selfReference: bool) -> (metamodelica::List<Arc<InstNode::InstNode>>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupLocalSimpleCref(name: String, scope: Arc<InstNode::InstNode>) -> (Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn lookupLocalSimpleName(name: String, scope: Arc<InstNode::InstNode>) -> (Arc<InstNode::InstNode>, bool) {
    todo!()
}

pub fn lookupName(name: Arc<Absyn::Path>, scope: Arc<InstNode::InstNode>, context: i32, checkAccessViolations: bool) -> (Arc<InstNode::InstNode>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupNameWithError(name: Arc<Absyn::Path>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo, errorType: ErrorTypes::Message, checkAccessViolations: bool) -> (Arc<InstNode::InstNode>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupNames(name: Arc<Absyn::Path>, scope: Arc<InstNode::InstNode>, context: i32) -> (metamodelica::List<Arc<InstNode::InstNode>>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupSimpleBuiltinCref(name: String, subs: metamodelica::List<Arc<Absyn::Subscript>>) -> (Arc<InstNode::InstNode>, Arc<ComponentRef::NFComponentRef>, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupSimpleBuiltinName(name: String) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn lookupSimpleCref(name: String, subs: metamodelica::List<Arc<Absyn::Subscript>>, scope: Arc<InstNode::InstNode>, context: i32) -> (Arc<InstNode::InstNode>, Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, bool, Arc<LookupState::LookupState>) {
    todo!()
}

pub fn lookupSimpleName(name: String, scope: Arc<InstNode::InstNode>, context: i32) -> (Arc<InstNode::InstNode>, bool) {
    todo!()
}

pub fn lookupSimpleNameRootPath(name: String, scope: Arc<InstNode::InstNode>, context: i32) -> Arc<Absyn::Path> {
    todo!()
}

pub fn makeInnerNode(node: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn resolveInnerCref(node: Arc<InstNode::InstNode>, cref: Arc<ComponentRef::NFComponentRef>, foundScope: Arc<InstNode::InstNode>) -> (Arc<InstNode::InstNode>, Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>) {
    todo!()
}

