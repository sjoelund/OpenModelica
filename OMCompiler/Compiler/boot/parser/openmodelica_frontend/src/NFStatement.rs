// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::BaseModelica;
use crate::DAE;
use crate::ElementSource;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInstNode::InstNode;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use openmodelica_util::IOStream;
use openmodelica_util::Util;

pub enum NFStatement {
    ASSIGNMENT {
        lhs: Arc<Expression::NFExpression>,
        rhs: Arc<Expression::NFExpression>,
        ty: Arc<Type::NFType>,
        source: Arc<DAE::ElementSource>,
    },
    FUNCTION_ARRAY_INIT {
        name: String,
        ty: Arc<Type::NFType>,
        source: Arc<DAE::ElementSource>,
    },
    FOR {
        iterator: Arc<InstNode::InstNode>,
        range: Option<Arc<Expression::NFExpression>>,
        body: metamodelica::List<Arc<Statement>>,
        forType: Arc<ForType>,
        source: Arc<DAE::ElementSource>,
    },
    IF {
        branches: metamodelica::List<(metamodelica::List<Arc<Statement>>, Arc<Expression::NFExpression>)>,
        source: Arc<DAE::ElementSource>,
    },
    WHEN {
        branches: metamodelica::List<(metamodelica::List<Arc<Statement>>, Arc<Expression::NFExpression>)>,
        source: Arc<DAE::ElementSource>,
    },
    ASSERT {
        condition: Arc<Expression::NFExpression>,
        message: Arc<Expression::NFExpression>,
        level: Arc<Expression::NFExpression>,
        source: Arc<DAE::ElementSource>,
    },
    TERMINATE {
        message: Arc<Expression::NFExpression>,
        source: Arc<DAE::ElementSource>,
    },
    REINIT {
        cref: Arc<Expression::NFExpression>,
        reinitExp: Arc<Expression::NFExpression>,
        source: Arc<DAE::ElementSource>,
    },
    NORETCALL {
        exp: Arc<Expression::NFExpression>,
        source: Arc<DAE::ElementSource>,
    },
    WHILE {
        condition: Arc<Expression::NFExpression>,
        body: metamodelica::List<Arc<Statement>>,
        source: Arc<DAE::ElementSource>,
    },
    RETURN {
        source: Arc<DAE::ElementSource>,
    },
    BREAK {
        source: Arc<DAE::ElementSource>,
    },
    FAILURE {
        body: metamodelica::List<Arc<Statement>>,
        source: Arc<DAE::ElementSource>,
    },
}
pub use NFStatement::*;
pub fn isDiscrete(stmt: Arc<Statement>) -> bool {
    todo!()
}

pub fn filterDiscrete(stmts: metamodelica::List<Arc<Statement>>, out_stmts: metamodelica::List<Arc<Statement>>) -> metamodelica::List<Arc<Statement>> {
    todo!()
}

pub fn hash(stmt: Arc<Statement>) -> i32 {
    todo!()
}

pub fn isEqual(stmt1: Arc<Statement>, stmt2: Arc<Statement>) -> bool {
    todo!()
}

pub fn makeAssignment(lhs: Arc<Expression::NFExpression>, rhs: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>, src: Arc<DAE::ElementSource>) -> Arc<Statement> {
    todo!()
}

pub fn isAssignment(stmt: Arc<Statement>) -> bool {
    todo!()
}

pub fn isFor(stmt: Arc<Statement>) -> bool {
    todo!()
}

pub fn isReturn(stmt: Arc<Statement>) -> bool {
    todo!()
}

pub fn makeIf(branches: metamodelica::List<(metamodelica::List<Arc<Statement>>, Arc<Expression::NFExpression>)>, src: Arc<DAE::ElementSource>) -> Arc<Statement> {
    todo!()
}

pub fn source(stmt: Arc<Statement>) -> Arc<DAE::ElementSource> {
    todo!()
}

pub fn setSource(source: Arc<DAE::ElementSource>, stmt: Arc<Statement>) -> Arc<Statement> {
    todo!()
}

pub fn info(stmt: Arc<Statement>) -> SourceInfo {
    todo!()
}

pub type ApplyFn = fn(Arc<Statement>) -> ();

pub fn apply(stmt: Arc<Statement>, func: fn(Arc<Statement>) -> ()) -> () {
    todo!()
}

pub fn map(stmt: Arc<Statement>, func: fn(Arc<Statement>) -> Arc<Statement>) -> Arc<Statement> {
    todo!()
}

pub fn fold<ArgT>(stmt: Arc<Statement>, func: fn(Arc<Statement>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn applyExpList(stmt: metamodelica::List<Arc<Statement>>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExp(stmt: Arc<Statement>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn mapExpList(stmtl: metamodelica::List<Arc<Statement>>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> metamodelica::List<Arc<Statement>> {
    todo!()
}

pub fn mapExp(stmt: Arc<Statement>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Statement> {
    todo!()
}

pub fn mapExpShallow(stmt: Arc<Statement>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Statement> {
    todo!()
}

pub fn foldExpList<ArgT>(stmt: metamodelica::List<Arc<Statement>>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn foldExp<ArgT>(stmt: Arc<Statement>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn contains(stmt: Arc<Statement>, r#fn: fn(Arc<Statement>) -> bool) -> bool {
    todo!()
}

pub fn containsList(eql: metamodelica::List<Arc<Statement>>, func: fn(Arc<Statement>) -> bool) -> bool {
    todo!()
}

pub fn replaceIteratorList(stmtl: metamodelica::List<Arc<Statement>>, iterator: Arc<InstNode::InstNode>, value: Arc<Expression::NFExpression>) -> metamodelica::List<Arc<Statement>> {
    todo!()
}

pub fn toString(stmt: Arc<Statement>, indent: String) -> String {
    todo!()
}

pub fn toStringList(stmtl: metamodelica::List<Arc<Statement>>, indent: String) -> String {
    todo!()
}

pub fn toStream(stmt: Arc<Statement>, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toStreamList(stmtl: metamodelica::List<Arc<Statement>>, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatStream(stmt: Arc<Statement>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatStreamList(stmtl: metamodelica::List<Arc<Statement>>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn isMultiLine(stmt: Arc<Statement>) -> bool {
    todo!()
}


