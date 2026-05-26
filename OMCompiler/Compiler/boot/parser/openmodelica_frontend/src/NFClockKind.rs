// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::DAE;
use crate::NFClockKind as ClockKind;
use crate::NFExpression as Expression;
use openmodelica_util::JSON;

pub enum NFClockKind {
    INFERRED_CLOCK {
        idx: i32,
    },
    RATIONAL_CLOCK {
        intervalCounter: Arc<Expression::NFExpression>,
        resolution: Arc<Expression::NFExpression>,
    },
    REAL_CLOCK {
        interval: Arc<Expression::NFExpression>,
    },
    EVENT_CLOCK {
        condition: Arc<Expression::NFExpression>,
        startInterval: Arc<Expression::NFExpression>,
    },
    SOLVER_CLOCK {
        c: Arc<Expression::NFExpression>,
        solverMethod: Arc<Expression::NFExpression>,
    },
}
pub use NFClockKind::*;
pub fn isInferred(ck: Arc<ClockKind>) -> bool {
    todo!()
}

pub fn compare(ck1: Arc<ClockKind>, ck2: Arc<ClockKind>) -> i32 {
    todo!()
}

pub fn containsExp(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn containsExpShallow(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn applyExp(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExpShallow(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn foldExp<ArgT>(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn mapExp(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<ClockKind> {
    todo!()
}

pub fn mapExpShallow(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<ClockKind> {
    todo!()
}

pub fn mapFoldExp<ArgT>(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), arg: ArgT) -> (Arc<ClockKind>, ArgT) {
    todo!()
}

pub fn mapFoldExpShallow<ArgT>(ck: Arc<ClockKind>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), arg: ArgT) -> (Arc<ClockKind>, ArgT) {
    todo!()
}

pub fn toAbsyn(clk: Arc<ClockKind>) -> Arc<Absyn::Exp> {
    todo!()
}

pub fn toDAE(ick: Arc<ClockKind>) -> Arc<DAE::ClockKind> {
    todo!()
}

pub fn toDebugString(ick: Arc<ClockKind>) -> String {
    todo!()
}

pub fn toString(ck: Arc<ClockKind>) -> String {
    todo!()
}

pub fn toFlatString(ck: Arc<ClockKind>, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn toJSON(clk: Arc<ClockKind>) -> Arc<JSON::JSON> {
    todo!()
}

pub fn hashContinue(clk: Arc<ClockKind>, hash: i32) -> i32 {
    todo!()
}


