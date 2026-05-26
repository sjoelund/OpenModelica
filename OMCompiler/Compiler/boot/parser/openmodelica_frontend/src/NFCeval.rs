// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::ExpressionSimplify;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use crate::NFTyping::TypingError;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Global;
use openmodelica_util::List;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Vector;

pub mod EvalTarget {
    use super::*;
    pub struct EVAL_TARGET {
        pub info: SourceInfo,
        pub context: i32,
        pub extra: Option<Arc<EvalTargetData>>,
    }

    pub type EvalTarget = EVAL_TARGET;
    pub fn new(info: SourceInfo, context: i32, extra: Option<Arc<EvalTargetData>>) -> Arc<EvalTarget> {
        todo!()
    }

    pub fn hasInfo(target: Arc<EvalTarget>) -> bool {
        todo!()
    }

    pub fn getInfo(target: Arc<EvalTarget>) -> SourceInfo {
        todo!()
    }

}

pub struct DIMENSION_DATA {
    pub component: Arc<InstNode::InstNode>,
    pub index: i32,
    pub exp: Arc<Expression::NFExpression>,
}

pub type EvalTargetData = DIMENSION_DATA;

type ReductionFn = fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>;

fn evalArrayConstructor(callExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalArrayConstructor2(exp: Arc<Expression::NFExpression>, ranges: metamodelica::List<Arc<Expression::NFExpression>>, iterators: metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryAdd(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryArrayScalar(arrayExp: Arc<Expression::NFExpression>, scalarExp: Arc<Expression::NFExpression>, opFunc: fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryDiv(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryExp(binaryExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryMatrixProduct(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryMul(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryMulMatrixVector(matrixExp: Arc<Expression::NFExpression>, vectorExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryMulVectorMatrix(vectorExp: Arc<Expression::NFExpression>, matrixExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryOp(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryOp_dispatch(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryPow(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryPowMatrix(matrixExp: Arc<Expression::NFExpression>, nExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryPowMatrix2(matrix: Arc<Expression::NFExpression>, n: i32) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryScalarArray(scalarExp: Arc<Expression::NFExpression>, arrayExp: Arc<Expression::NFExpression>, opFunc: fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinaryScalarProduct(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBinarySub(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBooleanClock(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinAbs(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinAcos(arg: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinArray(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinAsin(arg: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinAtan(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinAtan2(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinCall(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinCallExp(callExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinCat(argN: Arc<Expression::NFExpression>, args: metamodelica::List<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinCeil(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinCos(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinCosh(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinDer(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinDiagonal(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinDiv(args: metamodelica::List<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinExp(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalBuiltinFill(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinFloor(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinIdentity(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinInteger(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinIntegerEnum(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinLog(arg: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinLog10(arg: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinMatrix(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinMatrix2(arg: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinMax(args: metamodelica::List<Arc<Expression::NFExpression>>, r#fn: Arc<Function::Function>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinMax2(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinMin(args: metamodelica::List<Arc<Expression::NFExpression>>, r#fn: Arc<Function::Function>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinMin2(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinMod(args: metamodelica::List<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinOnes(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinProduct(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinProductInt(exp: Arc<Expression::NFExpression>, result: i32) -> i32 {
    todo!()
}

fn evalBuiltinProductReal(exp: Arc<Expression::NFExpression>, result: f64) -> f64 {
    todo!()
}

fn evalBuiltinPromote(arg: Arc<Expression::NFExpression>, argN: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinRem(args: metamodelica::List<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinScalar(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinSign(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinSin(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinSinh(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinSkew(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinSqrt(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinString(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinSum(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinSumInt(exp: Arc<Expression::NFExpression>, result: i32) -> i32 {
    todo!()
}

fn evalBuiltinSumReal(exp: Arc<Expression::NFExpression>, result: f64) -> f64 {
    todo!()
}

fn evalBuiltinSymmetric(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinTan(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinTanh(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinTranspose(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinVector(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalBuiltinZeros(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalCall(call: Arc<Call::NFCall>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalCast(castExp: Arc<Expression::NFExpression>, castTy: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalComponentBinding(node: Arc<InstNode::InstNode>, cref: Arc<ComponentRef::NFComponentRef>, defaultExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>, evalSubscripts: bool, liftExp: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalComponentStartBinding(node: Arc<InstNode::InstNode>, comp: Arc<Component::NFComponent>, cref: Arc<ComponentRef::NFComponentRef>, target: Arc<EvalTarget::EvalTarget>, evalSubscripts: bool) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn evalCref(cref: Arc<ComponentRef::NFComponentRef>, defaultExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>, evalSubscripts: bool, liftExp: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalExp(exp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalExpPartial(exp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>, evaluated: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn evalExpPartialDefault(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalGetInstanceName(scope: Arc<InstNode::InstNode>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalIfExp(ifExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalIfExp2(ifExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalInferredClock(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalIntBitAnd(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalIntBitLShift(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalIntBitOr(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalIntBitRShift(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalIntBitXor(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalLogicBinaryAnd(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalLogicBinaryExp(binaryExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalLogicBinaryOp(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalLogicBinaryOp_dispatch(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalLogicBinaryOr(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalLogicUnaryNot(exp1: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalLogicUnaryOp(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalMultaryAddSub(arguments: metamodelica::List<Arc<Expression::NFExpression>>, inv_arguments: metamodelica::List<Arc<Expression::NFExpression>>, operator_ty: Arc<Type::NFType>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn evalMultaryMulDiv(arguments: metamodelica::List<Arc<Expression::NFExpression>>, inv_arguments: metamodelica::List<Arc<Expression::NFExpression>>, operator_ty: Arc<Type::NFType>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn evalNormalCall(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalNormalCallExp(callExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalPositiveMax(flow_exp: Arc<Expression::NFExpression>, eps: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalRange(rangeExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalRangeExp(rangeExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalRangeReal(start: f64, step: f64, stop: f64) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

fn evalRationalClock(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalRealClock(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalRecordElement(exp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalRecordElement2(exp: Arc<Expression::NFExpression>, index: i32) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalReduction(callExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalRelationEqual(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn evalRelationExp(relationExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalRelationGreater(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn evalRelationGreaterEq(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn evalRelationLess(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn evalRelationLessEq(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn evalRelationNotEqual(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn evalRelationOp(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalRelationOp_dispatch(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalSize(exp: Arc<Expression::NFExpression>, optIndex: Option<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalSolverClock(args: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalSubscriptedExp(exp: Arc<Expression::NFExpression>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalTypename(ty: Arc<Type::NFType>, originExp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalUnaryMinus(exp1: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evalUnaryOp(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalUriToFilename(r#fn: Arc<Function::Function>, arg: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn isFlatCref(cref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

pub fn makeComponentBinding(component: Arc<Component::NFComponent>, node: Arc<InstNode::InstNode>, cref: Arc<ComponentRef::NFComponentRef>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn makeRecordBindingExp(typeNode: Arc<InstNode::InstNode>, recordNode: Arc<InstNode::InstNode>, recordType: Arc<Type::NFType>, cref: Arc<ComponentRef::NFComponentRef>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn makeRecordFieldBindingFromParent(cref: Arc<ComponentRef::NFComponentRef>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn printFailedEvalError(name: String, exp: Arc<Expression::NFExpression>, info: SourceInfo) -> () {
    todo!()
}

fn printUnboundError(component: Arc<Component::NFComponent>, target: Arc<EvalTarget::EvalTarget>, exp: Arc<Expression::NFExpression>) -> () {
    todo!()
}

fn printWrongArgsError(evalFunc: String, args: metamodelica::List<Arc<Expression::NFExpression>>, info: SourceInfo) -> () {
    todo!()
}

pub fn subscriptBinding(exp: Arc<Expression::NFExpression>, cref: Arc<ComponentRef::NFComponentRef>, evalSubscripts: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn subscriptBinding2(exp: Arc<Expression::NFExpression>, cref: Arc<ComponentRef::NFComponentRef>, evalSubscripts: bool, subMap: Option<UnorderedMap::UnorderedMap<metamodelica::List<Arc<Subscript::NFSubscript>>, Arc<InstNode::InstNode>>>) -> (Arc<Expression::NFExpression>, Option<UnorderedMap::UnorderedMap<metamodelica::List<Arc<Subscript::NFSubscript>>, Arc<InstNode::InstNode>>>) {
    todo!()
}

pub fn subscriptBinding3(subscript: Arc<Subscript::NFSubscript>, subMap: UnorderedMap::UnorderedMap<metamodelica::List<Arc<Subscript::NFSubscript>>, Arc<InstNode::InstNode>>) -> Arc<Subscript::NFSubscript> {
    todo!()
}

pub fn tryEvalExp(exp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn tryEvalExpPartial(exp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn tryEvalExpResizable(exp: Arc<Expression::NFExpression>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

