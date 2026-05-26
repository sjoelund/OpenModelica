// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::DAE;
use crate::Expression as DAEExpression;
use crate::NFBinding as Binding;
use crate::NFBuiltin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::Slot;
use crate::NFFunction::TypedArg;
use crate::NFInline as Inline;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRestriction as Restriction;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::Types;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::List;
use openmodelica_util::Util;

pub enum MatchKind {
    EXACT,
    CAST,
    UNKNOWN_EXPECTED,
    UNKNOWN_ACTUAL,
    GENERIC,
    PLUG_COMPATIBLE,
    NOT_COMPATIBLE,
}

pub type MatchOptions = i32;

pub fn checkBinaryOperation(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, operator: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, context: i32, info: SourceInfo, retype: bool) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkBinaryOperationAdd(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn checkBinaryOperationBoxed(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, context: i32, info: SourceInfo, retype: bool) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkBinaryOperationDiv(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, info: SourceInfo, isElementWise: bool) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkBinaryOperationEW(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, elemOp: Op, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkBinaryOperationMul(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkBinaryOperationPow(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkBinaryOperationPowEW(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkBinaryOperationSub(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkConditionalBinaryOperator(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, context: i32, info: SourceInfo, retype: bool) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn checkDimensionType(exp: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkLogicalBinaryOperation(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, operator: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn checkLogicalUnaryOperation(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, operator: Arc<Operator::NFOperator>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryArrayAddSub(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryArrayAddSub2(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryArrayDiv(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryArrayEW(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryArrayEW2(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryArrayMul(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryArrayScalar(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryArrayScalar2(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn checkOverloadedBinaryOperator(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryScalarArray(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn checkOverloadedBinaryScalarArray2(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn checkOverloadedUnaryOperator(inExp1: Arc<Expression::NFExpression>, inType1: Arc<Type::NFType>, var: Variability, inOp: Arc<Operator::NFOperator>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn checkReductionType(ty: Arc<Type::NFType>, name: Arc<Absyn::Path>, exp: Arc<Expression::NFExpression>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkRelationOperation(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, operator: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, index: i32, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn checkSumComplexType(ty: Arc<Type::NFType>, exp: Arc<Expression::NFExpression>, info: SourceInfo) -> bool {
    todo!()
}

pub fn checkUnaryOperation(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, operator: Arc<Operator::NFOperator>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn elaborateBindingType(bindingExp: Arc<Expression::NFExpression>, component: Arc<InstNode::InstNode>, bindingType: Arc<Type::NFType>, componentType: Arc<Type::NFType>) -> (Arc<Type::NFType>, Arc<Type::NFType>) {
    todo!()
}

pub fn getOption(options: i32, option: i32) -> bool {
    todo!()
}

pub fn getRangeType(startExp: Arc<Expression::NFExpression>, stepExp: Option<Arc<Expression::NFExpression>>, stopExp: Arc<Expression::NFExpression>, rangeElemType: Arc<Type::NFType>, info: SourceInfo) -> Arc<Type::NFType> {
    todo!()
}

pub fn getRangeTypeBool(startExp: Arc<Expression::NFExpression>, stopExp: Arc<Expression::NFExpression>) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn getRangeTypeEnum(startExp: Arc<Expression::NFExpression>, stopExp: Arc<Expression::NFExpression>) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn getRangeTypeInt(startExp: Arc<Expression::NFExpression>, stepExp: Option<Arc<Expression::NFExpression>>, stopExp: Arc<Expression::NFExpression>, info: SourceInfo) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn getRangeTypeReal(startExp: Arc<Expression::NFExpression>, stepExp: Option<Arc<Expression::NFExpression>>, stopExp: Arc<Expression::NFExpression>, info: SourceInfo) -> Arc<Dimension::NFDimension> {
    todo!()
}

fn implicitConstructAndMatch(candidates: metamodelica::List<Arc<Function::Function>>, inExp1: Arc<Expression::NFExpression>, inType1: Arc<Type::NFType>, op: Arc<Operator::NFOperator>, inExp2: Arc<Expression::NFExpression>, inType2: Arc<Type::NFType>, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

fn implicitConstructAndMatch2(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, paramType1: Arc<Type::NFType>, paramInfo1: SourceInfo, paramType2: Arc<Type::NFType>, paramInfo2: SourceInfo, scope: Arc<InstNode::InstNode>, r#fn: Arc<Function::Function>, reverseArgs: bool, matchedFns: metamodelica::List<(Variability, metamodelica::List<Arc<Expression::NFExpression>>, Arc<Function::Function>)>) -> (metamodelica::List<(Variability, metamodelica::List<Arc<Expression::NFExpression>>, Arc<Function::Function>)>, bool) {
    todo!()
}

pub fn isCastMatch(kind: MatchKind) -> bool {
    todo!()
}

pub fn isCompatibleMatch(kind: MatchKind) -> bool {
    todo!()
}

pub fn isExactMatch(kind: MatchKind) -> bool {
    todo!()
}

pub fn isGenericMatch(kind: MatchKind) -> bool {
    todo!()
}

pub fn isIncompatibleMatch(kind: MatchKind) -> bool {
    todo!()
}

pub fn isValidArgumentMatch(kind: MatchKind) -> bool {
    todo!()
}

pub fn isValidAssignmentMatch(kind: MatchKind) -> bool {
    todo!()
}

pub fn isValidPlugCompatibleMatch(kind: MatchKind) -> bool {
    todo!()
}

pub fn matchArrayDims(dims1: metamodelica::List<Arc<Dimension::NFDimension>>, dims2: metamodelica::List<Arc<Dimension::NFDimension>>, ty: Arc<Type::NFType>, matchKind: MatchKind, options: i32) -> (Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchArrayExpressions(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchArrayTypes(arrayType1: Arc<Type::NFType>, arrayType2: Arc<Type::NFType>, expression: Arc<Expression::NFExpression>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchBinding(binding: Arc<Binding::NFBinding>, componentType: Arc<Type::NFType>, name: String, component: Arc<InstNode::InstNode>, context: i32) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn matchBoxedExpressions(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchComplexComponents(actualComponents: Vec<Arc<InstNode::InstNode>>, expectedComponents: Vec<Arc<InstNode::InstNode>>, expressions: Vec<Arc<Expression::NFExpression>>, classTree: Arc<ClassTree::ClassTree>, options: i32) -> (metamodelica::List<Arc<Expression::NFExpression>>, MatchKind) {
    todo!()
}

pub fn matchComplexTypes(actualType: Arc<Type::NFType>, expectedType: Arc<Type::NFType>, expression: Arc<Expression::NFExpression>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchComponentList(comps1: metamodelica::List<Arc<InstNode::InstNode>>, comps2: metamodelica::List<Arc<InstNode::InstNode>>, options: i32) -> MatchKind {
    todo!()
}

pub fn matchConditionalArrayExp(condExp: Arc<Expression::NFExpression>, condType: Arc<Type::NFType>, otherExp: Arc<Expression::NFExpression>, otherType: Arc<Type::NFType>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchConditionalArrayTypes(actualType: Arc<Type::NFType>, expectedType: Arc<Type::NFType>, exp: Arc<Expression::NFExpression>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchConditionalArrayTypes_cast(condType: Arc<Type::NFType>, expectedType: Arc<Type::NFType>, exp: Arc<Expression::NFExpression>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchDimensions(dim1: Arc<Dimension::NFDimension>, dim2: Arc<Dimension::NFDimension>) -> (Arc<Dimension::NFDimension>, bool) {
    todo!()
}

pub fn matchEnumerationTypes(type1: Arc<Type::NFType>, type2: Arc<Type::NFType>) -> MatchKind {
    todo!()
}

pub fn matchExpressions(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchExpressions_cast(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchFunctionParameters(params1: metamodelica::List<Arc<InstNode::InstNode>>, params2: metamodelica::List<Arc<InstNode::InstNode>>, options: i32) -> bool {
    todo!()
}

pub fn matchFunctionTypes(actualType: Arc<Type::NFType>, expectedType: Arc<Type::NFType>, expression: Arc<Expression::NFExpression>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchIfBranches(trueBranch: Arc<Expression::NFExpression>, trueType: Arc<Type::NFType>, falseBranch: Arc<Expression::NFExpression>, falseType: Arc<Type::NFType>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchOverloadedBinaryOperator(exp1: Arc<Expression::NFExpression>, type1: Arc<Type::NFType>, var1: Variability, op: Arc<Operator::NFOperator>, exp2: Arc<Expression::NFExpression>, type2: Arc<Type::NFType>, var2: Variability, candidates: metamodelica::List<Arc<Function::Function>>, context: i32, info: SourceInfo, showErrors: bool) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>) {
    todo!()
}

pub fn matchPolymorphic(polymorphicName: String, actualType: Arc<Type::NFType>, exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchTupleTypes(tupleType1: Arc<Type::NFType>, tupleType2: Arc<Type::NFType>, expression: Arc<Expression::NFExpression>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchTypes(actualType: Arc<Type::NFType>, expectedType: Arc<Type::NFType>, expression: Arc<Expression::NFExpression>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn matchTypes_cast(actualType: Arc<Type::NFType>, expectedType: Arc<Type::NFType>, expression: Arc<Expression::NFExpression>, options: i32) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind) {
    todo!()
}

pub fn printBindingTypeError(name: String, binding: Arc<Binding::NFBinding>, componentType: Arc<Type::NFType>, bindingType: Arc<Type::NFType>, component: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn printUnresolvableTypeError(exp: Arc<Expression::NFExpression>, types: metamodelica::List<Arc<Type::NFType>>, info: SourceInfo, printError: bool) -> () {
    todo!()
}

pub fn setOption(currentOptions: i32, newOption: i32) -> i32 {
    todo!()
}

pub fn typeCastRecord(expressions: metamodelica::List<Arc<Expression::NFExpression>>, node: Arc<InstNode::InstNode>, expectedType: Arc<Type::NFType>, expression: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

