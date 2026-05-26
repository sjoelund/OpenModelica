// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFBinding as Binding;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFInstNode::InstNode;
use crate::NFSimplifyExp as SimplifyExp;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Array;

pub enum NFExpressionIterator {
    ARRAY_ITERATOR {
        arr: Vec<Arc<Expression::NFExpression>>,
        index: i32,
        arrays: metamodelica::List<Vec<Arc<Expression::NFExpression>>>,
    },
    SCALAR_ITERATOR {
        exp: Arc<Expression::NFExpression>,
    },
    EACH_ITERATOR {
        exp: Arc<Expression::NFExpression>,
    },
    NONE_ITERATOR,
    REPEAT_ITERATOR {
        current: metamodelica::List<Arc<Expression::NFExpression>>,
        all: metamodelica::List<Arc<Expression::NFExpression>>,
    },
}
pub use NFExpressionIterator::*;
pub fn toString(iter: Arc<ExpressionIterator>) -> String {
    todo!()
}

pub fn fromExp(exp: Arc<Expression::NFExpression>, backend: bool, resize: bool) -> Arc<ExpressionIterator> {
    todo!()
}

pub fn fromExpOpt(optExp: Option<Arc<Expression::NFExpression>>) -> Arc<ExpressionIterator> {
    todo!()
}

pub fn fromBinding(binding: Arc<Binding::NFBinding>) -> Arc<ExpressionIterator> {
    todo!()
}

pub fn hasNext(iterator: Arc<ExpressionIterator>) -> bool {
    todo!()
}

pub fn next(iterator: Arc<ExpressionIterator>) -> (Arc<ExpressionIterator>, Arc<Expression::NFExpression>) {
    todo!()
}

pub fn nextOpt(iterator: Arc<ExpressionIterator>) -> (Arc<ExpressionIterator>, Option<Arc<Expression::NFExpression>>) {
    todo!()
}

pub fn toList(iterator: Arc<ExpressionIterator>) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn isSubscriptedArrayCall(iterator: Arc<ExpressionIterator>, trySimplify: bool) -> bool {
    todo!()
}

fn makeArrayIterator(exp: Arc<Expression::NFExpression>) -> Arc<ExpressionIterator> {
    todo!()
}

fn flattenArray(exp: Arc<Expression::NFExpression>, arrays: metamodelica::List<Vec<Arc<Expression::NFExpression>>>) -> metamodelica::List<Vec<Arc<Expression::NFExpression>>> {
    todo!()
}

fn flattenArray_impl(exp: Arc<Expression::NFExpression>, arrays: metamodelica::List<Vec<Arc<Expression::NFExpression>>>) -> metamodelica::List<Vec<Arc<Expression::NFExpression>>> {
    todo!()
}


