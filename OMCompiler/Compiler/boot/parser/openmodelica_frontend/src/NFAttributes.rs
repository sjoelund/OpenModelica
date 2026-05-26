// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::DAE;
use crate::NFAttributes as Attributes;
use crate::NFClass as Class;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRestriction as Restriction;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_util::IOStream;

pub struct ATTRIBUTES {
    pub connectorType: i32,
    pub parallelism: Parallelism,
    pub variability: Variability,
    pub direction: Direction,
    pub innerOuter: InnerOuter,
    pub isFinal: bool,
    pub isRedeclare: bool,
    pub isReplaceable: Arc<Replaceable>,
    pub isResizable: bool,
}

pub type NFAttributes = ATTRIBUTES;
pub fn fromSCode(compAttr: SCode::Attributes, compPrefs: Arc<SCode::Prefixes>) -> Arc<Attributes> {
    todo!()
}

pub fn fromDerivedSCode(scodeAttr: SCode::Attributes) -> Arc<Attributes> {
    todo!()
}

pub fn mergeComponentAttributes(outerAttr: Arc<Attributes>, innerAttr: Arc<Attributes>, node: Arc<InstNode::InstNode>, parentRestriction: Arc<Restriction::NFRestriction>) -> Arc<Attributes> {
    todo!()
}

pub fn mergeDerivedAttributes(outerAttr: Arc<Attributes>, innerAttr: Arc<Attributes>, node: Arc<InstNode::InstNode>) -> Arc<Attributes> {
    todo!()
}

pub fn mergeRedeclaredComponentAttributes(origAttr: Arc<Attributes>, redeclAttr: Arc<Attributes>, node: Arc<InstNode::InstNode>) -> Arc<Attributes> {
    todo!()
}

pub fn mergeRedeclaredClassPrefixes(origPrefs: Arc<NFClass::Prefixes::Prefixes>, redeclPrefs: Arc<NFClass::Prefixes::Prefixes>, node: Arc<InstNode::InstNode>) -> Arc<NFClass::Prefixes::Prefixes> {
    todo!()
}

pub fn printRedeclarePrefixError(node: Arc<InstNode::InstNode>, prefix1: String, prefix2: String) -> () {
    todo!()
}

pub fn checkDeclaredComponentAttributes(attr: Arc<Attributes>, parentRestriction: Arc<Restriction::NFRestriction>, component: Arc<InstNode::InstNode>) -> Arc<Attributes> {
    todo!()
}

pub fn invalidComponentPrefixError(prefix: String, node: Arc<InstNode::InstNode>, restriction: Arc<Restriction::NFRestriction>) -> () {
    todo!()
}

pub fn assertNotInputOutput(dir: Direction, node: Arc<InstNode::InstNode>, restriction: Arc<Restriction::NFRestriction>) -> () {
    todo!()
}

pub fn assertNotInnerOuter(io: InnerOuter, node: Arc<InstNode::InstNode>, restriction: Arc<Restriction::NFRestriction>) -> () {
    todo!()
}

pub fn assertNotFlowStream(cty: i32, node: Arc<InstNode::InstNode>, restriction: Arc<Restriction::NFRestriction>) -> () {
    todo!()
}

pub fn updateComponentConnectorType(attributes: Arc<Attributes>, restriction: Arc<Restriction::NFRestriction>, context: i32, component: Arc<InstNode::InstNode>) -> Arc<Attributes> {
    todo!()
}

pub fn updateClassConnectorType(res: Arc<Restriction::NFRestriction>, attrs: Arc<Attributes>) -> Arc<Attributes> {
    todo!()
}

pub fn updateVariability(attr: Arc<Attributes>, cls: Arc<Class::NFClass>, clsNode: Arc<InstNode::InstNode>, compNode: Arc<InstNode::InstNode>, context: i32) -> Arc<Attributes> {
    todo!()
}

pub fn setConnectorType(cty: i32, attr: Arc<Attributes>) -> Arc<Attributes> {
    todo!()
}

pub fn setVariability(var: Variability, attr: Arc<Attributes>) -> Arc<Attributes> {
    todo!()
}

pub fn setDirection(dir: Direction, attr: Arc<Attributes>) -> Arc<Attributes> {
    todo!()
}

pub fn setInnerOuter(io: InnerOuter, attr: Arc<Attributes>) -> Arc<Attributes> {
    todo!()
}

pub fn setFinal(fin: bool, attr: Arc<Attributes>) -> Arc<Attributes> {
    todo!()
}

pub fn setRedeclare(redecl: bool, attr: Arc<Attributes>) -> Arc<Attributes> {
    todo!()
}

pub fn setReplaceable(repl: Arc<Replaceable>, attr: Arc<Attributes>) -> Arc<Attributes> {
    todo!()
}

pub fn toDAE(ina: Arc<Attributes>, vis: Visibility) -> Arc<DAE::Attributes> {
    todo!()
}

pub fn toString(attr: Arc<Attributes>, ty: Arc<NFType::NFType>) -> String {
    todo!()
}

pub fn toFlatStream(attr: Arc<Attributes>, ty: Arc<NFType::NFType>, s: IOStream::IOStream, isTopLevel: bool) -> IOStream::IOStream {
    todo!()
}


