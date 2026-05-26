// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFExpression as Expression;
use crate::NFType as Type;

pub enum ArgSpec {
    INPUT,
    OUTPUT,
    LOCAL,
}

pub fn callFunction(fnHandle: i32, args: Vec<Arc<Expression::NFExpression>>, specs: Vec<ArgSpec>, returnType: Arc<Type::NFType>) -> (Arc<Expression::NFExpression>, metamodelica::List<Arc<Expression::NFExpression>>) {
    todo!()
}

