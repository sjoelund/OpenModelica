// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn::Exp;
use crate::Absyn::Path;
use crate::Absyn::Subscript;
use crate::Absyn;
use crate::BaseModelica;
use crate::DAE;
use crate::Dump;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Variability;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFType as Type;
use openmodelica_util::List;

pub enum NFDimension {
    RAW_DIM {
        dim: Arc<Subscript>,
        scope: Arc<InstNode::InstNode>,
    },
    UNTYPED {
        dimension: Arc<Expression::NFExpression>,
        isProcessing: bool,
    },
    INTEGER {
        size: i32,
        var: Variability,
    },
    BOOLEAN,
    ENUM {
        enumType: Arc<Type::NFType>,
    },
    EXP {
        exp: Arc<Expression::NFExpression>,
        var: Variability,
    },
    RESIZABLE {
        size: i32,
        opt_size: Option<i32>,
        exp: Arc<Expression::NFExpression>,
        var: Variability,
    },
    UNKNOWN,
}
pub use NFDimension::*;
pub fn fromExp(exp: Arc<Expression::NFExpression>, var: Variability) -> Arc<Dimension> {
    todo!()
}

pub fn fromRange(range: Arc<Expression::NFExpression>) -> Arc<Dimension> {
    todo!()
}

pub fn fromInteger(n: i32, var: Variability) -> Arc<Dimension> {
    todo!()
}

pub fn fromExpArray(expl: Vec<Arc<Expression::NFExpression>>) -> Arc<Dimension> {
    todo!()
}

pub fn fromExpList(expl: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Dimension> {
    todo!()
}

pub fn toRange(dim: Arc<Dimension>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn toDAE(dim: Arc<Dimension>) -> Arc<DAE::Dimension> {
    todo!()
}

pub fn add(a: Arc<Dimension>, b: Arc<Dimension>) -> Arc<Dimension> {
    todo!()
}

pub fn size(dim: Arc<Dimension>, resize: bool) -> i32 {
    todo!()
}

pub fn sizes(dims: metamodelica::List<Arc<Dimension>>, resize: bool) -> metamodelica::List<i32> {
    todo!()
}

pub fn sizesProduct(dims: metamodelica::List<Arc<Dimension>>, resize: bool) -> i32 {
    todo!()
}

pub fn isEqual(dim1: Arc<Dimension>, dim2: Arc<Dimension>) -> bool {
    todo!()
}

pub fn isEqualKnown(dim1: Arc<Dimension>, dim2: Arc<Dimension>) -> bool {
    todo!()
}

pub fn isEqualKnownSize(dim1: Arc<Dimension>, node1: Arc<InstNode::InstNode>, index1: i32, dim2: Arc<Dimension>, node2: Arc<InstNode::InstNode>, index2: i32) -> bool {
    todo!()
}

pub fn isSizeOf(dim: Arc<Dimension>, node: Arc<InstNode::InstNode>, index: i32) -> bool {
    todo!()
}

pub fn isResizable(dim: Arc<Dimension>) -> bool {
    todo!()
}

pub fn allEqualKnown(dims1: metamodelica::List<Arc<Dimension>>, dims2: metamodelica::List<Arc<Dimension>>) -> bool {
    todo!()
}

pub fn isKnown(dim: Arc<Dimension>, allowExp: bool) -> bool {
    todo!()
}

pub fn isUnknown(dim: Arc<Dimension>) -> bool {
    todo!()
}

pub fn isZero(dim: Arc<Dimension>) -> bool {
    todo!()
}

pub fn isOne(dim: Arc<Dimension>) -> bool {
    todo!()
}

pub fn subscriptType(dim: Arc<Dimension>) -> Arc<Type::NFType> {
    todo!()
}

pub fn toString(dim: Arc<Dimension>) -> String {
    todo!()
}

pub fn hashList(dims: metamodelica::List<Arc<Dimension>>) -> i32 {
    todo!()
}

pub fn toStringList(dims: metamodelica::List<Arc<Dimension>>, brackets: bool) -> String {
    todo!()
}

pub fn toFlatString(dim: Arc<Dimension>, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn toFlatStringList(dims: metamodelica::List<Arc<Dimension>>, format: BaseModelica::OutputFormat, name: String) -> String {
    todo!()
}

pub fn endExp(dim: Arc<Dimension>, subscriptedExp: Arc<Expression::NFExpression>, index: i32) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn sizeExp(dim: Arc<Dimension>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn lowerBoundExp(dim: Arc<Dimension>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expIsLowerBound(exp: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn upperBoundExp(dim: Arc<Dimension>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expIsUpperBound(exp: Arc<Expression::NFExpression>, dim: Arc<Dimension>) -> bool {
    todo!()
}

pub fn variability(dim: Arc<Dimension>) -> Variability {
    todo!()
}

pub fn mapExp(dim: Arc<Dimension>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Dimension> {
    todo!()
}

pub fn foldExp<ArgT>(dim: Arc<Dimension>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn foldExpList<ArgT>(dims: metamodelica::List<Arc<Dimension>>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn eval(dim: Arc<Dimension>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Dimension> {
    todo!()
}

pub fn simplify(dim: Arc<Dimension>) -> Arc<Dimension> {
    todo!()
}

pub fn typeOf(dim: Arc<Dimension>) -> Arc<Type::NFType> {
    todo!()
}


