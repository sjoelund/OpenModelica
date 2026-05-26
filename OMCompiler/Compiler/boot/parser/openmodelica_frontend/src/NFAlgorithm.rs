// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE;
use crate::NFAlgorithm as Algorithm;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedSet;

pub struct ALGORITHM {
    pub statements: metamodelica::List<Arc<Statement::NFStatement>>,
    pub inputs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>,
    pub outputs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>,
    pub stmtDiffInfo: Option<UnorderedSet::UnorderedSet<Arc<Statement::NFStatement>>>,
    pub scope: Arc<InstNode::InstNode>,
    pub source: Arc<DAE::ElementSource>,
}

pub type NFAlgorithm = ALGORITHM;
pub type ApplyFn = fn(Arc<Statement::NFStatement>) -> ();

pub fn applyList(algs: metamodelica::List<Arc<Algorithm>>, func: fn(Arc<Statement::NFStatement>) -> ()) -> () {
    todo!()
}

pub fn apply(alg: Arc<Algorithm>, func: fn(Arc<Statement::NFStatement>) -> ()) -> () {
    todo!()
}

pub fn applyExp(alg: Arc<Algorithm>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExpList(algs: metamodelica::List<Arc<Algorithm>>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn map(alg: Arc<Algorithm>, r#fn: fn(Arc<Statement::NFStatement>) -> Arc<Statement::NFStatement>) -> Arc<Algorithm> {
    todo!()
}

pub fn mapExp(alg: Arc<Algorithm>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Algorithm> {
    todo!()
}

pub fn mapExpList(algs: metamodelica::List<Arc<Algorithm>>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> metamodelica::List<Arc<Algorithm>> {
    todo!()
}

pub fn foldExp<ArgT>(alg: Arc<Algorithm>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn foldExpList<ArgT>(algs: metamodelica::List<Arc<Algorithm>>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn toString(alg: Arc<Algorithm>, indent: String) -> String {
    todo!()
}

pub fn setInputsOutputs(alg: Arc<Algorithm>) -> Arc<Algorithm> {
    todo!()
}

pub fn getInputsOutputs(statements: metamodelica::List<Arc<Statement::NFStatement>>) -> (metamodelica::List<Arc<ComponentRef::NFComponentRef>>, metamodelica::List<Arc<ComponentRef::NFComponentRef>>) {
    todo!()
}

pub fn isEqual(alg1: Arc<Algorithm>, alg2: Arc<Algorithm>) -> bool {
    todo!()
}

pub fn isEmpty(alg: Arc<Algorithm>) -> bool {
    todo!()
}

pub fn isDiscrete(alg: Arc<Algorithm>) -> bool {
    todo!()
}

fn statementInputsOutputs(statement: Arc<Statement::NFStatement>, inputs_set: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, outputs_set: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn expressionInputs(exp: Arc<Expression::NFExpression>, inputs_set: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, outputs_set: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn expressionOutput(exp: Arc<Expression::NFExpression>, inputs_set: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, outputs_set: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}


