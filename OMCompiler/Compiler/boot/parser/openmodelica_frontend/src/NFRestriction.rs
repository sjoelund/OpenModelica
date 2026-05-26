// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::ClassInf;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFRestriction as Restriction;
use crate::SCode;
use crate::SCodeUtil;

pub enum NFRestriction {
    BLOCK,
    CLASS,
    CLOCK,
    CONNECTOR {
        isExpandable: bool,
    },
    ENUMERATION,
    EXTERNAL_OBJECT,
    FUNCTION,
    MODEL,
    PACKAGE,
    OPERATOR,
    RECORD {
        isOperator: bool,
        usedExternally: bool,
    },
    RECORD_CONSTRUCTOR,
    TYPE,
    UNKNOWN,
}
pub use NFRestriction::*;
pub fn fromSCode(sres: SCode::Restriction) -> Arc<Restriction> {
    todo!()
}

pub fn toDAE(res: Arc<Restriction>, path: Arc<Absyn::Path>) -> ClassInf::State {
    todo!()
}

pub fn isConnector(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isExpandableConnector(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isNonexpandableConnector(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isExternalObject(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isFunction(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isRecordConstructor(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isRecord(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isExternalRecord(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn setExternalRecord(res: Arc<Restriction>) -> Arc<Restriction> {
    todo!()
}

pub fn isOperatorRecord(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isOperator(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isType(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isClock(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn isModel(res: Arc<Restriction>) -> bool {
    todo!()
}

pub fn toString(res: Arc<Restriction>) -> String {
    todo!()
}

pub fn assertNoEquations(equations: metamodelica::List<Arc<SCode::Equation>>, initialEquations: metamodelica::List<Arc<SCode::Equation>>, res: Arc<Restriction>, onlyDeprecated: bool) -> () {
    todo!()
}

pub fn assertNoAlgorithms(algorithms: metamodelica::List<Arc<SCode::AlgorithmSection>>, initialAlgorithms: metamodelica::List<Arc<SCode::AlgorithmSection>>, res: Arc<Restriction>, onlyDeprecated: bool) -> () {
    todo!()
}

pub fn assertNoInitialAlgorithms(algs: metamodelica::List<Arc<SCode::AlgorithmSection>>, res: Arc<Restriction>) -> () {
    todo!()
}

pub fn assertNoProtected(elements: metamodelica::List<Arc<SCode::Element>>, res: Arc<Restriction>) -> () {
    todo!()
}

pub fn assertNoComponents(elements: metamodelica::List<Arc<SCode::Element>>, res: Arc<Restriction>) -> () {
    todo!()
}

pub fn assertOnlyConstantComponents(elements: metamodelica::List<Arc<SCode::Element>>, clsNode: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn assertOnlyFunctions(elements: metamodelica::List<Arc<SCode::Element>>, res: Arc<Restriction>) -> () {
    todo!()
}

pub fn checkClass(node: Arc<InstNode::InstNode>, restriction: Arc<Restriction>, context: i32) -> () {
    todo!()
}


