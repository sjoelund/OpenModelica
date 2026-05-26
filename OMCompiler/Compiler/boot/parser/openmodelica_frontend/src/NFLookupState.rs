// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::Dump;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFRestriction as Restriction;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_util::Error;
use openmodelica_util::System;

pub mod LookupState {
    use super::*;
    pub enum LookupState {
        BEGIN,
        COMP,
        CLASS_COMP,
        COMP_CLASS,
        COMP_FUNC,
        PACKAGE,
        CLASS,
        FUNC,
        PREDEF_COMP,
        PREDEF_CLASS,
        IMPORT,
        PARTIAL_CLASS,
        NON_CONSTANT,
        NON_ENCAPSULATED,
        ERROR {
            errorState: Arc<LookupState>,
        },
    }
    pub use LookupState::*;
    pub fn assertClass(endState: Arc<LookupState>, node: Arc<InstNode::InstNode>, name: Arc<Absyn::Path>, context: i32, info: SourceInfo) -> () {
        todo!()
    }

    pub fn assertFunction(endState: Arc<LookupState>, node: Arc<InstNode::InstNode>, name: Arc<Absyn::ComponentRef>, context: i32, info: SourceInfo) -> () {
        todo!()
    }

    pub fn assertComponent(endState: Arc<LookupState>, node: Arc<InstNode::InstNode>, name: Arc<Absyn::ComponentRef>, context: i32, info: SourceInfo) -> () {
        todo!()
    }

    pub fn assertImport(endState: Arc<LookupState>, node: Arc<InstNode::InstNode>, name: Arc<Absyn::Path>, info: SourceInfo) -> () {
        todo!()
    }

    pub fn isCallableType(node: Arc<InstNode::InstNode>) -> bool {
        todo!()
    }

    pub fn isCallableComponent(node: Arc<InstNode::InstNode>) -> bool {
        todo!()
    }

    pub fn isFunction(state: Arc<LookupState>, node: Arc<InstNode::InstNode>) -> bool {
        todo!()
    }

    pub fn isClass(state: Arc<LookupState>) -> bool {
        todo!()
    }

    pub fn assertState(endState: Arc<LookupState>, expectedState: Arc<LookupState>, node: Arc<InstNode::InstNode>, name: Arc<LookupStateName::LookupStateName>, context: i32, info: SourceInfo) -> () {
        todo!()
    }

    pub fn isError(state: Arc<LookupState>) -> bool {
        todo!()
    }

    pub fn lookupStateString(state: Arc<LookupState>) -> String {
        todo!()
    }

    pub fn printFoundWrongTypeError(foundState: Arc<LookupState>, expectedState: Arc<LookupState>, name: Arc<LookupStateName::LookupStateName>, info: SourceInfo) -> () {
        todo!()
    }

    pub fn next(node: Arc<InstNode::InstNode>, currentState: Arc<LookupState>, context: i32, checkAccessViolations: bool) -> Arc<LookupState> {
        todo!()
    }

    pub fn checkProtection(node: Arc<InstNode::InstNode>, currentState: Arc<LookupState>) -> () {
        todo!()
    }

    pub fn nodeState(node: Arc<InstNode::InstNode>) -> Arc<LookupState> {
        todo!()
    }

    pub fn elementState(element: Arc<SCode::Element>) -> Arc<LookupState> {
        todo!()
    }

    pub fn next2(elementState: Arc<LookupState>, currentState: Arc<LookupState>, node: Arc<InstNode::InstNode>) -> Arc<LookupState> {
        todo!()
    }

    pub fn checkCrefVariability(cref: Arc<ComponentRef::NFComponentRef>, inEnclosingScope: bool, context: i32, state: Arc<LookupState>) -> Arc<LookupState> {
        todo!()
    }

    pub fn isNonConstantComponent(node: Arc<InstNode::InstNode>) -> bool {
        todo!()
    }

}

pub mod LookupStateName {
    use super::*;
    pub enum LookupStateName {
        PATH {
            path: Arc<Absyn::Path>,
        },
        CREF {
            cref: Arc<Absyn::ComponentRef>,
        },
    }
    pub use LookupStateName::*;
    pub fn toString(name: Arc<LookupStateName>) -> String {
        todo!()
    }

    pub fn firstIdent(name: Arc<LookupStateName>) -> String {
        todo!()
    }

    pub fn secondIdent(name: Arc<LookupStateName>) -> String {
        todo!()
    }

}

