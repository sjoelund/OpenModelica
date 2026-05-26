// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::DAE;
use crate::NFBinding as Binding;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFCall as Call;
use crate::NFCallAttributes;
use crate::NFCallParameterTree;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::NamedArg;
use crate::NFFunction::TypedArg;
use crate::NFInline as Inline;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use crate::SCodeUtil;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::ErrorExt;
use openmodelica_util::JSON;
use openmodelica_util::List;
use openmodelica_util::Util;

pub enum NFCall {
    UNTYPED_CALL {
        r#ref: Arc<ComponentRef::NFComponentRef>,
        arguments: metamodelica::List<Arc<Expression::NFExpression>>,
        named_args: metamodelica::List<(Arc<Expression::NFExpression>, String)>,
        call_scope: Arc<InstNode::InstNode>,
    },
    ARG_TYPED_CALL {
        r#ref: Arc<ComponentRef::NFComponentRef>,
        positional_args: metamodelica::List<Arc<TypedArg>>,
        named_args: metamodelica::List<Arc<TypedArg>>,
        call_scope: Arc<InstNode::InstNode>,
    },
    TYPED_CALL {
        r#fn: Arc<Function::Function>,
        ty: Arc<Type::NFType>,
        var: Variability,
        purity: Purity,
        arguments: metamodelica::List<Arc<Expression::NFExpression>>,
        attributes: Arc<NFCallAttributes::NFCallAttributes>,
    },
    UNTYPED_ARRAY_CONSTRUCTOR {
        exp: Arc<Expression::NFExpression>,
        iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>,
    },
    TYPED_ARRAY_CONSTRUCTOR {
        ty: Arc<Type::NFType>,
        var: Variability,
        purity: Purity,
        exp: Arc<Expression::NFExpression>,
        iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>,
    },
    UNTYPED_REDUCTION {
        r#ref: Arc<ComponentRef::NFComponentRef>,
        exp: Arc<Expression::NFExpression>,
        iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>,
    },
    TYPED_REDUCTION {
        r#fn: Arc<Function::Function>,
        ty: Arc<Type::NFType>,
        var: Variability,
        purity: Purity,
        exp: Arc<Expression::NFExpression>,
        iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>,
        defaultExp: Option<Arc<Expression::NFExpression>>,
        foldExp: (String, String, Option<Arc<Expression::NFExpression>>),
    },
}
pub use NFCall::*;
pub type ParameterTree = Arc<NFCallParameterTree::Tree>;

pub fn instantiate(functionName: Arc<Absyn::ComponentRef>, functionArgs: Arc<Absyn::FunctionArgs>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn typeCall(callExp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo, retype: bool) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

pub fn checkNotPartial(fnRef: Arc<ComponentRef::NFComponentRef>, context: i32, info: SourceInfo) -> () {
    todo!()
}

pub fn typeCallExp(ty_call: Arc<Call>) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeNormalCall(call: Arc<Call>, context: i32, info: SourceInfo) -> Arc<Call> {
    todo!()
}

pub fn makeTypedCall(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, variability: Variability, purity: Purity, returnType: Arc<Type::NFType>) -> Arc<Call> {
    todo!()
}

pub fn unboxArgs(call: Arc<Call>) -> Arc<Call> {
    todo!()
}

pub fn typeMatchNormalCall(call: Arc<Call>, context: i32, info: SourceInfo, vectorize: bool) -> Arc<Call> {
    todo!()
}

pub fn matchTypedNormalCall(call: Arc<Call>, context: i32, info: SourceInfo, vectorize: bool) -> Arc<Call> {
    todo!()
}

pub fn retypeCall(call: Arc<Call>, context: i32, info: SourceInfo) -> Arc<Call> {
    todo!()
}

pub fn typeOf(call: Arc<Call>) -> Arc<Type::NFType> {
    todo!()
}

pub fn setType(call: Arc<Call>, ty: Arc<Type::NFType>) -> Arc<Call> {
    todo!()
}

pub fn variability(call: Arc<Call>) -> Variability {
    todo!()
}

pub fn purity(call: Arc<Call>) -> Purity {
    todo!()
}

pub fn compare(call1: Arc<Call>, call2: Arc<Call>) -> i32 {
    todo!()
}

pub fn compareIterator(iter1: (Arc<Expression::NFExpression>, Arc<InstNode::InstNode>), iter2: (Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)) -> i32 {
    todo!()
}

pub fn isExternal(call: Arc<Call>) -> bool {
    todo!()
}

pub fn isImpure(call: Arc<Call>) -> bool {
    todo!()
}

pub fn isRecordConstructor(call: Arc<Call>) -> bool {
    todo!()
}

pub fn isExternalObjectConstructor(call: Arc<Call>) -> bool {
    todo!()
}

pub fn isLiteral(call: Arc<Call>) -> bool {
    todo!()
}

pub fn isKnownSizeFill(call: Arc<Call>) -> bool {
    todo!()
}

pub fn isReduction(call: Arc<Call>) -> bool {
    todo!()
}

pub fn inlineType(call: Arc<Call>) -> DAE::InlineType {
    todo!()
}

pub fn typedFunction(call: Arc<Call>) -> Arc<Function::Function> {
    todo!()
}

pub fn functionName(call: Arc<Call>) -> Arc<Absyn::Path> {
    todo!()
}

pub fn functionNameLast(call: Arc<Call>) -> String {
    todo!()
}

pub fn functionNameFirst(call: Arc<Call>) -> String {
    todo!()
}

pub fn isNamed(call: Arc<Call>, name: String) -> bool {
    todo!()
}

pub fn arguments(call: Arc<Call>) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn setArguments(call: Arc<Call>, arguments: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Call> {
    todo!()
}

pub fn iterators(call: Arc<Call>) -> metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)> {
    todo!()
}

pub fn toRecordExpression(call: Arc<Call>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn toString(call: Arc<Call>) -> String {
    todo!()
}

pub fn toFlatString(call: Arc<Call>, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn toFlatStringArgs(args: metamodelica::List<Arc<Expression::NFExpression>>, fnName: String, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn typedString(call: Arc<Call>) -> String {
    todo!()
}

pub fn toJSON(call: Arc<Call>) -> Arc<JSON::JSON> {
    todo!()
}

pub fn toJSONStringArgs(args: metamodelica::List<Arc<Expression::NFExpression>>, json: Arc<JSON::JSON>) -> Arc<JSON::JSON> {
    todo!()
}

pub fn toAbsyn(call: Arc<Call>) -> Arc<Absyn::Exp> {
    todo!()
}

pub fn toAbsynIterators(iterExp: Arc<Expression::NFExpression>, iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>) -> Arc<Absyn::FunctionArgs> {
    todo!()
}

pub fn toDAE(call: Arc<Call>) -> Arc<DAE::Exp> {
    todo!()
}

pub fn toDAE_work(call: Arc<Call>) -> Arc<DAE::Exp> {
    todo!()
}

pub fn expandReduction(call: Arc<Call>) -> Arc<Call> {
    todo!()
}

pub fn isVectorizeable(call: Arc<Call>) -> bool {
    todo!()
}

pub fn retype(call: Arc<Call>) -> Arc<Call> {
    todo!()
}

pub fn typeCast(callExp: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn containsExp(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn containsExpShallow(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn applyExp(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExpShallow(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn foldExp<ArgT>(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, foldArg: ArgT) -> ArgT {
    todo!()
}

pub fn mapExp(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Call> {
    todo!()
}

pub fn mapIteratorsExp(iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)> {
    todo!()
}

pub fn mapExpShallow(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Call> {
    todo!()
}

pub fn mapIteratorsExpShallow(iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)> {
    todo!()
}

pub fn mapFoldExp<ArgT>(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), foldArg: ArgT) -> (Arc<Call>, ArgT) {
    todo!()
}

pub fn mapFoldIteratorsExp<ArgT>(iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), arg: ArgT) -> (metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>, ArgT) {
    todo!()
}

pub fn mapFoldExpShallow<ArgT>(call: Arc<Call>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), foldArg: ArgT) -> (Arc<Call>, ArgT) {
    todo!()
}

pub fn mapFoldIteratorsExpShallow<ArgT>(iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), arg: ArgT) -> (metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>, ArgT) {
    todo!()
}

pub fn updateExternalRecordArgs(args: metamodelica::List<Arc<Expression::NFExpression>>) -> () {
    todo!()
}

pub fn updateExternalRecordArgsInType(ty: Arc<Type::NFType>) -> () {
    todo!()
}

pub fn toArrayConstructor(iCall: Arc<Call>, index_ptr: Pointer::Pointer<i32>) -> Arc<Call> {
    todo!()
}

pub fn isConnectionsOperator(call: Arc<Call>) -> bool {
    todo!()
}

pub fn isStreamOperator(call: Arc<Call>) -> bool {
    todo!()
}

pub fn isCardinality(call: Arc<Call>) -> bool {
    todo!()
}

fn instNormalCall(functionName: Arc<Absyn::ComponentRef>, functionArgs: Arc<Absyn::FunctionArgs>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

fn instArgs(args: Arc<Absyn::FunctionArgs>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (metamodelica::List<Arc<Expression::NFExpression>>, metamodelica::List<(Arc<Expression::NFExpression>, String)>) {
    todo!()
}

fn instNamedArg(absynArg: Arc<Absyn::NamedArg>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, String) {
    todo!()
}

fn instIteratorCall(functionName: Arc<Absyn::ComponentRef>, functionArgs: Arc<Absyn::FunctionArgs>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

fn instIteratorCallArgs(args: Arc<Absyn::FunctionArgs>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>) {
    todo!()
}

fn instIterators(inIters: metamodelica::List<Arc<Absyn::ForIterator>>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<InstNode::InstNode>, metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>) {
    todo!()
}

fn typeArrayConstructor(call: Arc<Call>, context: i32, info: SourceInfo) -> (Arc<Call>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

fn typeReduction(call: Arc<Call>, context: i32, info: SourceInfo) -> (Arc<Call>, Arc<Type::NFType>, Variability, Purity) {
    todo!()
}

pub fn makeTypedReduction(r#fn: Arc<Function::Function>, ty: Arc<Type::NFType>, var: Variability, purity: Purity, arg: Arc<Expression::NFExpression>, iters: metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>, info: SourceInfo) -> Arc<Call> {
    todo!()
}

fn reductionDefaultValue(r#fn: Arc<Function::Function>, ty: Arc<Type::NFType>) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

fn reductionFoldExpression(reductionFn: Arc<Function::Function>, reductionType: Arc<Type::NFType>, reductionVar: Variability, reductionPurity: Purity, foldId: String, resultId: String, info: SourceInfo) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

fn reductionFoldIterator(name: String, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn typeArgs(call: Arc<Call>, context: i32, info: SourceInfo) -> Arc<Call> {
    todo!()
}

fn checkMatchingFunctions(call: Arc<Call>, context: i32, info: SourceInfo, vectorize: bool) -> Arc<MatchedFunction::MatchedFunction> {
    todo!()
}

fn iteratorToDAE(iter: (Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)) -> Arc<DAE::ReductionIterator> {
    todo!()
}

fn vectorizeCall(base_call: Arc<Call>, mk: Arc<FunctionMatchKind::FunctionMatchKind>, scope: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<Call> {
    todo!()
}

fn isVectorized(call: Arc<Call>) -> bool {
    todo!()
}

fn devectorizeCall(call: Arc<Call>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateCallType(ty: Arc<Type::NFType>, r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, outputIndex: i32, ptree: Arc<NFCallParameterTree::Tree>) -> (Arc<Type::NFType>, Arc<NFCallParameterTree::Tree>) {
    todo!()
}

fn evaluateCallTypeDim(dim: Arc<Dimension::NFDimension>, r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, ptree: Arc<NFCallParameterTree::Tree>) -> (Arc<Dimension::NFDimension>, Arc<NFCallParameterTree::Tree>) {
    todo!()
}

fn buildParameterTree(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, ptree: Arc<NFCallParameterTree::Tree>) -> Arc<NFCallParameterTree::Tree> {
    todo!()
}

fn evaluateCallTypeDimExp(exp: Arc<Expression::NFExpression>, ptree: Arc<NFCallParameterTree::Tree>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn resolvePolymorphicReturnType(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<TypedArg>>, ty: Arc<Type::NFType>) -> Arc<Type::NFType> {
    todo!()
}


