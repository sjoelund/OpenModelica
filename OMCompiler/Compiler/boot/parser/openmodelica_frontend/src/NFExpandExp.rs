// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::NFCall as Call;
use crate::NFCallAttributes;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;

pub struct NFExpandExp;
pub fn expand(exp: Arc<Expression::NFExpression>, backend: bool, resize: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandArray(arr: Vec<Arc<Expression::NFExpression>>) -> (Vec<Arc<Expression::NFExpression>>, bool) {
    todo!()
}

pub fn expandList(expl: metamodelica::List<Arc<Expression::NFExpression>>, abortOnFailure: bool) -> (metamodelica::List<Arc<Expression::NFExpression>>, bool) {
    todo!()
}

pub fn expandCref(crefExp: Arc<Expression::NFExpression>, backend: bool, resize: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandCref2(cref: Arc<ComponentRef::NFComponentRef>, backend: bool, resize: bool, subs: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    todo!()
}

pub fn expandCref3(subs: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>, cref: Arc<ComponentRef::NFComponentRef>, crefType: Arc<Type::NFType>, accum: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandCref4(subs: metamodelica::List<Arc<Subscript::NFSubscript>>, comb: metamodelica::List<Arc<Subscript::NFSubscript>>, accum: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>, restSubs: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>, cref: Arc<ComponentRef::NFComponentRef>, crefType: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandTypename(ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandRange(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandCall(call: Arc<Call::NFCall>, exp: Arc<Expression::NFExpression>, resize: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBuiltinCall(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>, resize: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBuiltinCat(args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>, resize: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBuiltinPromote(args: metamodelica::List<Arc<Expression::NFExpression>>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBuiltinDiagonal(arg: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBuiltinFill(args: metamodelica::List<Arc<Expression::NFExpression>>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBuiltinTranspose(arg: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBuiltinGeneric(call: Arc<Call::NFCall>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBuiltinGeneric2(exp: Arc<Expression::NFExpression>, r#fn: Arc<Function::Function>, ty: Arc<Type::NFType>, var: Variability, pur: Purity, attr: Arc<NFCallAttributes::NFCallAttributes>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandArrayConstructor(exp: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>, iterators: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandArrayConstructor2(exp: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>, ranges: metamodelica::List<Arc<Expression::NFExpression>>, iterators: metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandSize(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBinary(exp: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, resize: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBinaryElementWise(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBinaryElementWise2(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, func: fn(Arc<Expression::NFExpression>, Arc<Operator::NFOperator>, Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandBinaryScalarArray(exp: Arc<Expression::NFExpression>, scalarOp: NFOperator::Op) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn makeScalarArrayBinary_traverser(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandBinaryArrayScalar(exp: Arc<Expression::NFExpression>, scalarOp: NFOperator::Op) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBinaryVectorMatrix(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBinaryMatrixVector(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBinaryDotProduct(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn makeScalarProduct(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandBinaryMatrixProduct(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn makeBinaryMatrixProduct(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn makeBinaryMatrixProduct2(row: Arc<Expression::NFExpression>, matrix: Vec<Arc<Expression::NFExpression>>) -> Vec<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn expandBinaryPowMatrix(exp: Arc<Expression::NFExpression>, resize: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandBinaryPowMatrix2(matrix: Arc<Expression::NFExpression>, n: i32) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandUnary(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandLogicalBinary(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn makeLBinaryOp(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandLogicalUnary(exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn makeLogicalUnaryOp(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandCast(castExp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandGeneric(exp: Arc<Expression::NFExpression>, resize: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn expandGeneric2(subs: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>, exp: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>, accum: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandCallArgs(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}


