// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::BaseModelica;
use crate::DAE;
use crate::ElementSource;
use crate::NFCall as Call;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::IOStream;
use openmodelica_util::Util;

pub enum NFEquation {
    EQUALITY {
        lhs: Arc<Expression::NFExpression>,
        rhs: Arc<Expression::NFExpression>,
        ty: Arc<Type::NFType>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
        scalarizeMode: ScalarizeMode,
    },
    CONNECT {
        lhs: Arc<Expression::NFExpression>,
        rhs: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    FOR {
        iterator: Arc<InstNode::InstNode>,
        range: Option<Arc<Expression::NFExpression>>,
        body: metamodelica::List<Arc<Equation>>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    IF {
        branches: metamodelica::List<Arc<Branch::Branch>>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    WHEN {
        branches: metamodelica::List<Arc<Branch::Branch>>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    ASSERT {
        condition: Arc<Expression::NFExpression>,
        message: Arc<Expression::NFExpression>,
        level: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    TERMINATE {
        message: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    REINIT {
        cref: Arc<Expression::NFExpression>,
        reinitExp: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    NORETCALL {
        exp: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
}
pub use NFEquation::*;
pub enum ScalarizeMode {
    DONT_SCALARIZE,
    SCALARIZE,
    NO_PREFERENCE,
}

pub fn makeEquality(lhs: Arc<Expression::NFExpression>, rhs: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>, src: Arc<DAE::ElementSource>, scope: Arc<InstNode::InstNode>, scalarizeMode: ScalarizeMode) -> Arc<Equation> {
    todo!()
}

pub fn makeCrefEquality(lhsCref: Arc<ComponentRef::NFComponentRef>, rhsCref: Arc<ComponentRef::NFComponentRef>, scope: Arc<InstNode::InstNode>, src: Arc<DAE::ElementSource>) -> Arc<Equation> {
    todo!()
}

pub fn makeBranch(condition: Arc<Expression::NFExpression>, body: metamodelica::List<Arc<Equation>>, condVar: Variability) -> Arc<Branch::Branch> {
    todo!()
}

pub fn makeIf(branches: metamodelica::List<Arc<Branch::Branch>>, scope: Arc<InstNode::InstNode>, src: Arc<DAE::ElementSource>) -> Arc<Equation> {
    todo!()
}

pub fn source(eq: Arc<Equation>) -> Arc<DAE::ElementSource> {
    todo!()
}

pub fn setSource(source: Arc<DAE::ElementSource>, eq: Arc<Equation>) -> Arc<Equation> {
    todo!()
}

pub fn scope(eq: Arc<Equation>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn info(eq: Arc<Equation>) -> SourceInfo {
    todo!()
}

pub type ApplyFn = fn(Arc<Equation>) -> ();

pub fn applyList(eql: metamodelica::List<Arc<Equation>>, func: fn(Arc<Equation>) -> ()) -> () {
    todo!()
}

pub fn apply(eq: Arc<Equation>, func: fn(Arc<Equation>) -> ()) -> () {
    todo!()
}

pub type MapFn = fn(Arc<Equation>) -> Arc<Equation>;

pub fn map(eq: Arc<Equation>, func: fn(Arc<Equation>) -> Arc<Equation>) -> Arc<Equation> {
    todo!()
}

pub fn applyExpList(eq: metamodelica::List<Arc<Equation>>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExp(eq: Arc<Equation>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExpShallow(eq: Arc<Equation>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub type MapExpFn = fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>;

pub fn mapExpList(eql: metamodelica::List<Arc<Equation>>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> metamodelica::List<Arc<Equation>> {
    todo!()
}

pub fn mapExp(eq: Arc<Equation>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Equation> {
    todo!()
}

pub fn mapExpShallow(eq: Arc<Equation>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Equation> {
    todo!()
}

pub fn foldExpList<ArgT>(eq: metamodelica::List<Arc<Equation>>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn foldExp<ArgT>(eq: Arc<Equation>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn contains(eq: Arc<Equation>, func: fn(Arc<Equation>) -> bool) -> bool {
    todo!()
}

pub fn containsList(eql: metamodelica::List<Arc<Equation>>, func: fn(Arc<Equation>) -> bool) -> bool {
    todo!()
}

pub fn containsExp(eq: Arc<Equation>, r#fn: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn containsExpList(eql: metamodelica::List<Arc<Equation>>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn replaceIteratorList(eql: metamodelica::List<Arc<Equation>>, iterator: Arc<InstNode::InstNode>, value: Arc<Expression::NFExpression>) -> metamodelica::List<Arc<Equation>> {
    todo!()
}

pub fn isArrayEquality(eq: Arc<Equation>) -> bool {
    todo!()
}

pub fn isConnect(eq: Arc<Equation>) -> bool {
    todo!()
}

pub fn isConnection(eq: Arc<Equation>) -> bool {
    todo!()
}

pub fn sizeOfList(eqs: metamodelica::List<Arc<Equation>>) -> i32 {
    todo!()
}

pub fn sizeOf(eq: Arc<Equation>) -> i32 {
    todo!()
}

pub fn toString(eq: Arc<Equation>, indent: String) -> String {
    todo!()
}

pub fn toStringList(eql: metamodelica::List<Arc<Equation>>, indent: String) -> String {
    todo!()
}

pub fn toStream(eq: Arc<Equation>, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toStreamList(eql: metamodelica::List<Arc<Equation>>, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatStream(eq: Arc<Equation>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatStreamList(eql: metamodelica::List<Arc<Equation>>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn isMultiLine(eq: Arc<Equation>) -> bool {
    todo!()
}

pub fn splitRecordEquations(equations: metamodelica::List<Arc<Equation>>) -> metamodelica::List<Arc<Equation>> {
    todo!()
}

pub fn splitRecordEquation(eq: Arc<Equation>, equations: metamodelica::List<Arc<Equation>>) -> metamodelica::List<Arc<Equation>> {
    todo!()
}

pub fn splitRecordEquationBranch(branch: Arc<Branch::Branch>) -> Arc<Branch::Branch> {
    todo!()
}


