// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFCeval as Ceval;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;
use openmodelica_util::Error;
use openmodelica_util::SBGraph::IncidenceList;
use openmodelica_util::SBGraph::VertexDescriptor;
use openmodelica_util::SBInterval;
use openmodelica_util::SBLinearMap;
use openmodelica_util::SBMultiInterval;
use openmodelica_util::SBPWLinearMap;
use openmodelica_util::SBSet;
use openmodelica_util::System;
use openmodelica_util::Vector;

pub fn evalCrefs(e: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn intervalFromBinaryExp(lhs: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, rhs: Arc<Expression::NFExpression>) -> Arc<SBInterval::SBInterval> {
    todo!()
}

pub fn intervalFromExp(e: Arc<Expression::NFExpression>) -> Arc<SBInterval::SBInterval> {
    todo!()
}

pub fn intervalFromRange(e: Arc<Expression::NFExpression>) -> Arc<SBInterval::SBInterval> {
    todo!()
}

pub fn intervalFromUnaryExp(e: Arc<Expression::NFExpression>) -> Arc<SBInterval::SBInterval> {
    todo!()
}

pub fn linearMapFromIntervals(d1: i32, d2: i32, mi1: Arc<SBMultiInterval::SBMultiInterval>, mi2: Arc<SBMultiInterval::SBMultiInterval>, eCount: Vector::Vector<i32>) -> (String, Arc<SBPWLinearMap::SBPWLinearMap>, Arc<SBPWLinearMap::SBPWLinearMap>) {
    todo!()
}

pub fn make_lo_interval(i: Arc<SBInterval::SBInterval>) -> Arc<SBInterval::SBInterval> {
    todo!()
}

pub fn multiIntervalFromDimensions(dims: metamodelica::List<Arc<Dimension::NFDimension>>, vCount: Vector::Vector<i32>) -> Arc<SBMultiInterval::SBMultiInterval> {
    todo!()
}

pub fn multiIntervalFromSubscripts(subs: metamodelica::List<Arc<Subscript::NFSubscript>>, vCount: Vector::Vector<i32>, multiInt: Arc<SBMultiInterval::SBMultiInterval>) -> Arc<SBMultiInterval::SBMultiInterval> {
    todo!()
}

