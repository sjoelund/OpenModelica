// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE::InlineType;
use crate::DAE;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_util::Flags;

fn convertIfToAssignment(stmt: Arc<Statement::NFStatement>) -> Arc<Statement::NFStatement> {
    todo!()
}

fn convertToAssignment(stmt: Arc<Statement::NFStatement>) -> Arc<Statement::NFStatement> {
    todo!()
}

fn getOutputExp(stmt: Arc<Statement::NFStatement>, outputNode: Arc<InstNode::InstNode>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn inlineCall(callExp: Arc<Expression::NFExpression>, forceInline: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn inlineCallExp(callExp: Arc<Expression::NFExpression>, forceInline: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeOutputStatement(outputNode: Arc<InstNode::InstNode>) -> Arc<Statement::NFStatement> {
    todo!()
}

fn removeDeadCode(body: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

fn replaceCrefNode(exp: Arc<Expression::NFExpression>, node: Arc<InstNode::InstNode>, value: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn replaceCrefNode2(cref: Arc<ComponentRef::NFComponentRef>, node: Arc<InstNode::InstNode>, value: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn replaceDimExp(dim: Arc<Dimension::NFDimension>, node: Arc<InstNode::InstNode>, value: Arc<Expression::NFExpression>) -> Arc<Dimension::NFDimension> {
    todo!()
}

