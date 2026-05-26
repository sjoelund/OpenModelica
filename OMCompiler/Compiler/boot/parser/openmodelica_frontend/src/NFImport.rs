// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFImport as Import;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;

pub enum NFImport {
    UNRESOLVED_IMPORT {
        imp: Absyn::Import,
        scope: Arc<InstNode::InstNode>,
        info: SourceInfo,
    },
    RESOLVED_IMPORT {
        node: Arc<InstNode::InstNode>,
        shortName: String,
        info: SourceInfo,
    },
    CONFLICTING_IMPORT {
        imp1: Arc<Import>,
        imp2: Arc<Import>,
    },
}
pub use NFImport::*;
pub fn name(imp: Arc<Import>) -> String {
    todo!()
}

pub fn info(imp: Arc<Import>) -> SourceInfo {
    todo!()
}

pub fn resolve(imp: Arc<Import>) -> (Arc<InstNode::InstNode>, bool, Arc<Import>) {
    todo!()
}

pub fn resolveList(imps: Vec<Arc<Import>>) -> metamodelica::List<Arc<Import>> {
    todo!()
}

pub fn instQualified(imp: Absyn::Import, scope: Arc<InstNode::InstNode>, info: SourceInfo) -> (Arc<Import>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn instUnqualified(imp: Arc<Import>, imps: metamodelica::List<Arc<Import>>) -> metamodelica::List<Arc<Import>> {
    todo!()
}

pub fn printImportError(imp1: Arc<Import>, imp2: Arc<Import>) -> () {
    todo!()
}


