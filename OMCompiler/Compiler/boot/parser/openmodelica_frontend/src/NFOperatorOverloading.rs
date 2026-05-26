// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFRecord as Record;
use crate::NFType as Type;
use crate::SCodeUtil;

fn checkOperatorConstructorOutput(r#fn: Arc<Function::Function>, recordNode: Arc<InstNode::InstNode>, path: Arc<Absyn::Path>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkOperatorRestrictions(operatorNode: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn instConstructor(path: Arc<Absyn::Path>, recordNode: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn instOperatorFunctions(node: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn lookupOperatorFunctionsInType(operatorName: String, ty: Arc<Type::NFType>) -> metamodelica::List<Arc<Function::Function>> {
    todo!()
}

pub fn patchOperatorRecordConstructorBinding(r#fn: Arc<Function::Function>) -> Arc<Function::Function> {
    todo!()
}

fn patchOperatorRecordConstructorBinding_traverser(exp: Arc<Expression::NFExpression>, constructorFn: Arc<Function::Function>) -> Arc<Expression::NFExpression> {
    todo!()
}

