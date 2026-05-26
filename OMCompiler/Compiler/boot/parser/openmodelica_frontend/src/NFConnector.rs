// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE;
use crate::ElementSource;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Variability;
use crate::NFRestriction as Restriction;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use metamodelica::Dangerous::arrayGetNoBoundsChecking;
use metamodelica::Dangerous::listReverseInPlace;

pub struct CONNECTOR {
    pub name: Arc<ComponentRef::NFComponentRef>,
    pub ty: Arc<Type::NFType>,
    pub face: Face,
    pub cty: i32,
    pub source: Arc<DAE::ElementSource>,
}

pub type NFConnector = CONNECTOR;
pub enum Face {
    INSIDE,
    OUTSIDE,
}

pub fn fromCref(cref: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>, source: Arc<DAE::ElementSource>) -> Arc<Connector> {
    todo!()
}

pub fn fromFacedCref(cref: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>, face: Face, source: Arc<DAE::ElementSource>) -> Arc<Connector> {
    todo!()
}

pub fn fromExp(exp: Arc<Expression::NFExpression>, source: Arc<DAE::ElementSource>, conns: metamodelica::List<Arc<Connector>>) -> metamodelica::List<Arc<Connector>> {
    todo!()
}

pub fn getType(conn: Arc<Connector>) -> Arc<Type::NFType> {
    todo!()
}

pub fn getInfo(conn: Arc<Connector>) -> SourceInfo {
    todo!()
}

pub fn variability(conn: Arc<Connector>) -> Variability {
    todo!()
}

pub fn isEqual(conn1: Arc<Connector>, conn2: Arc<Connector>) -> bool {
    todo!()
}

pub fn isEqualNoSubs(conn1: Arc<Connector>, conn2: Arc<Connector>) -> bool {
    todo!()
}

pub fn isPrefix(conn1: Arc<Connector>, conn2: Arc<Connector>) -> bool {
    todo!()
}

pub fn isNodeNameEqual(conn1: Arc<Connector>, conn2: Arc<Connector>) -> bool {
    todo!()
}

pub fn isOutside(conn: Arc<Connector>) -> bool {
    todo!()
}

pub fn isInside(conn: Arc<Connector>) -> bool {
    todo!()
}

pub fn setOutside(conn: Arc<Connector>) -> Arc<Connector> {
    todo!()
}

pub fn isDeleted(conn: Arc<Connector>) -> bool {
    todo!()
}

pub fn isExpandable(conn: Arc<Connector>) -> bool {
    todo!()
}

pub fn isArray(conn: Arc<Connector>) -> bool {
    todo!()
}

pub fn name(conn: Arc<Connector>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

pub fn toString(conn: Arc<Connector>) -> String {
    todo!()
}

pub fn faceString(conn: Arc<Connector>) -> String {
    todo!()
}

pub fn hash(conn: Arc<Connector>) -> i32 {
    todo!()
}

pub fn hashNoSubs(conn: Arc<Connector>) -> i32 {
    todo!()
}

pub fn split(conn: Arc<Connector>) -> metamodelica::List<Arc<Connector>> {
    todo!()
}

pub fn scalarize(conn: Arc<Connector>) -> metamodelica::List<Arc<Connector>> {
    todo!()
}

pub fn scalarizePrefix(conn: Arc<Connector>) -> metamodelica::List<Arc<Connector>> {
    todo!()
}

pub fn addSubscripts(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, conn: Arc<Connector>) -> Arc<Connector> {
    todo!()
}

fn crefFace(cref: Arc<ComponentRef::NFComponentRef>) -> Face {
    todo!()
}

fn splitImpl(name: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>, face: Face, source: Arc<DAE::ElementSource>, cty: i32, dims: metamodelica::List<Arc<Dimension::NFDimension>>, conns: metamodelica::List<Arc<Connector>>) -> metamodelica::List<Arc<Connector>> {
    todo!()
}

fn splitImpl2(name: Arc<ComponentRef::NFComponentRef>, face: Face, source: Arc<DAE::ElementSource>, comps: metamodelica::List<Arc<InstNode::InstNode>>, dims: metamodelica::List<Arc<Dimension::NFDimension>>, conns: metamodelica::List<Arc<Connector>>) -> metamodelica::List<Arc<Connector>> {
    todo!()
}


