// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::NFCall as Call;
use crate::NFCallAttributes;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClockKind as ClockKind;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::NamedArg;
use crate::NFFunction::TypedArg;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use crate::NFTyping::InstContext;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Array;
use openmodelica_util::Config;
use openmodelica_util::Global;
use openmodelica_util::List;
use openmodelica_util::System;
use openmodelica_util::Util;

fn assertNoNamedParams(fnName: String, namedArgs: metamodelica::List<(Arc<Expression::NFExpression>, String)>, info: SourceInfo) -> () {
    todo!()
}

fn checkConnectionsArgument(arg: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>, fnRef: Arc<ComponentRef::NFComponentRef>, argIndex: i32, info: SourceInfo) -> () {
    todo!()
}

pub fn makeArrayExp(posArgs: metamodelica::List<Arc<Expression::NFExpression>>, namedArgs: metamodelica::List<(Arc<Expression::NFExpression>, String)>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn makeCatExp(n: i32, args: metamodelica::List<Arc<Expression::NFExpression>>, tys: metamodelica::List<Arc<Type::NFType>>, variability: Variability, purity: Purity, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn makeSizeExp(posArgs: metamodelica::List<Arc<Expression::NFExpression>>, namedArgs: metamodelica::List<(Arc<Expression::NFExpression>, String)>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn needSpecialHandling(call: Arc<Call::NFCall>) -> bool {
    todo!()
}

fn typeActualInStreamCall(name: String, call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeActualInStreamCall2(name: String, r#fn: Arc<Function::Function>, arg: Arc<Expression::NFExpression>, var: Variability, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

fn typeBackSampleCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeBranchCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeBuiltinCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo, vectorize: bool) -> (Arc<Call::NFCall>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeBuiltinCallExp(call: Arc<Call::NFCall>, context: i32, info: SourceInfo, vectorize: bool) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeBuiltinStringCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeCardinalityCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeCatCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeChangeCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeClockCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeConnectionsArg(arg: Arc<Expression::NFExpression>, context: i32, info: SourceInfo, fnRef: Arc<ComponentRef::NFComponentRef>, index: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn typeConnectionsArgs(args: metamodelica::List<Arc<Expression::NFExpression>>, context: i32, info: SourceInfo, fnRef: Arc<ComponentRef::NFComponentRef>) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

fn typeDerCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeDiscreteCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeDynamicSelectCall(name: String, call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeEdgeCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeFillCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeFillCall2(fnRef: Arc<ComponentRef::NFComponentRef>, fillType: Arc<Type::NFType>, fillArg: Arc<Expression::NFExpression>, fillVariability: Variability, fillPurity: Purity, dimensionArgs: metamodelica::List<Arc<Expression::NFExpression>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeGetInstanceName(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeIsRootCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeMatrixCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeMinMaxCall(name: String, call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeNdimsCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeNoEventCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeOverloadedStringCall(overloadedType: Arc<Type::NFType>, args: metamodelica::List<Arc<TypedArg>>, namedArgs: metamodelica::List<Arc<TypedArg>>, call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typePotentialRootCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typePreCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typePreChangeCall(name: String, call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typePromoteCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typePureCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeRootCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeRootedCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeSampleCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeScalarCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeShiftSampleCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeSmoothCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeSpecial(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeStringCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeSubSampleCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeSuperSampleCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeSymmetricCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeTransposeCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeUniqueRootCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeUniqueRootIndicesCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeVectorCall(call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeZerosOnesCall(name: String, call: Arc<Call::NFCall>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

