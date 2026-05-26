// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Array;
use openmodelica_util::Debug;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;

fn addArgument(exp: Arc<Expression::NFExpression>, arg: Arc<Expression::NFExpression>, inverse: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

fn cancelTermsInMultary(inArguments: metamodelica::List<Arc<Expression::NFExpression>>, inInv_arguments: metamodelica::List<Arc<Expression::NFExpression>>) -> (metamodelica::List<Arc<Expression::NFExpression>>, metamodelica::List<Arc<Expression::NFExpression>>) {
    todo!()
}

pub fn combineBinaries(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn combineBinariesExp(exp: Arc<Expression::NFExpression>, optOperator: Option<Arc<Operator::NFOperator>>, result: Arc<Expression::NFExpression>, inverse: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

fn combineBinariesSubscript(subscript: Arc<Subscript::NFSubscript>) -> Arc<Subscript::NFSubscript> {
    todo!()
}

pub fn combineConstantNumbers(r#const: metamodelica::List<Arc<Expression::NFExpression>>, inv_const: metamodelica::List<Arc<Expression::NFExpression>>, mcl: NFOperator::MathClassification, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn getConstantValue(exp: Arc<Expression::NFExpression>) -> f64 {
    todo!()
}

pub fn isIteratorSubscriptedArray(exp: Arc<Expression::NFExpression>, iterator: Arc<InstNode::InstNode>) -> bool {
    todo!()
}

pub fn removeInStreamDiv(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn removePositiveMax(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn removeStream(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn removeTrivialScalarProduct(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplify(exp: Arc<Expression::NFExpression>, includeScope: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyArrayConstructor(call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBinary(binaryExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBinaryAdd(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBinaryDiv(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBinaryEW(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBinaryMul(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, switched: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBinaryOp(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBinaryPow(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBinarySub(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyBuiltinCall(name: Arc<Absyn::Path>, args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>, expand: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyCall(callExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyCall2(call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyCast(exp: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyCat(args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyDelay(args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyDer(arg: Arc<Expression::NFExpression>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyDump(exp: Arc<Expression::NFExpression>, includeScope: bool, name: String, indent: String) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyFill(fillArg: Arc<Expression::NFExpression>, dimArgs: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>, expand: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyHomotopy(args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyIf(ifExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyInStreamDiv(args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>, removeStream: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyLogicBinary(binaryExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyLogicBinaryAnd(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyLogicBinaryOr(exp1: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyLogicUnary(unaryExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyMinMax(args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>, isMin: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyMultary(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyMultarySigns(arguments: metamodelica::List<Arc<Expression::NFExpression>>, inv_arguments: metamodelica::List<Arc<Expression::NFExpression>>, mcl: NFOperator::MathClassification) -> (metamodelica::List<Arc<Expression::NFExpression>>, metamodelica::List<Arc<Expression::NFExpression>>, bool) {
    todo!()
}

pub fn simplifyPositiveMax(args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyRange(range: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyRecordElement(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyReducedArrayConstructor(arg: Arc<Expression::NFExpression>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyReduction(call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyReduction2(name: String, exp: Arc<Expression::NFExpression>, iterators: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyRelation(relationExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifySemiLinear(args: metamodelica::List<Arc<Expression::NFExpression>>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifySize(sizeExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifySubscriptedExp(subscriptedExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifySumProduct(arg: Arc<Expression::NFExpression>, call: Arc<Call::NFCall>, expand: bool, isSum: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyTranspose(arg: Arc<Expression::NFExpression>, call: Arc<Call::NFCall>, expand: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyTupleElement(tupleExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn simplifyURIToFilename(arg: Arc<Expression::NFExpression>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyUnary(unaryExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyUnaryOp(exp: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyUnarySign(unaryExp: Arc<Expression::NFExpression>, isNegative: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplifyVector(arg: Arc<Expression::NFExpression>, call: Arc<Call::NFCall>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn splitMultary(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

