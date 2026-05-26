// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFRangeIterator as RangeIterator;
use crate::NFType as Type;
use openmodelica_util::Util;

pub enum NFRangeIterator {
    INT_RANGE {
        current: i32,
        last: i32,
    },
    INT_STEP_RANGE {
        current: i32,
        stepsize: i32,
        last: i32,
    },
    REAL_RANGE {
        start: f64,
        stepsize: f64,
        current: i32,
        steps: i32,
    },
    ARRAY_RANGE {
        values: Vec<Arc<Expression::NFExpression>>,
        index: i32,
    },
    INVALID_RANGE {
        exp: Arc<Expression::NFExpression>,
    },
}
pub use NFRangeIterator::*;
pub fn isValid(iterator: Arc<RangeIterator>) -> bool {
    todo!()
}

pub fn fromExp(exp: Arc<Expression::NFExpression>) -> Arc<RangeIterator> {
    todo!()
}

pub fn fromDim(dim: Arc<Dimension::NFDimension>, resizable: bool) -> Arc<RangeIterator> {
    todo!()
}

pub fn next(iterator: Arc<RangeIterator>) -> (Arc<RangeIterator>, Arc<Expression::NFExpression>) {
    todo!()
}

pub fn hasNext(iterator: Arc<RangeIterator>) -> bool {
    todo!()
}

pub fn toList(iterator: Arc<RangeIterator>) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn toListReverse(iterator: Arc<RangeIterator>) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn map<T>(iterator: Arc<RangeIterator>, func: fn(Arc<Expression::NFExpression>) -> T) -> metamodelica::List<T> {
    todo!()
}

pub fn fold<ArgT>(iterator: Arc<RangeIterator>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}


