// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::DAE;
use crate::ElementSource;
use crate::FFI;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunctionExt as EvalFunctionExt;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRecord as Record;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::SCode;
use crate::SCodeUtil;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;
use openmodelica_util::Autoconf;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;

pub type ArgumentMap = UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>;

pub enum FlowControl {
    NEXT,
    CONTINUE,
    BREAK,
    RETURN,
    ASSERTION,
}

fn addImmutableArgument(node: Arc<InstNode::InstNode>, map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, buildArrayBinding: bool) -> UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>> {
    todo!()
}

fn addMutableArgument(node: Arc<InstNode::InstNode>, map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, buildArrayBinding: bool) -> UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>> {
    todo!()
}

fn applyBindingReplacement(exp: Arc<Expression::NFExpression>, map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn applyReplacementCall(map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, call: Arc<Call::NFCall>, exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn applyReplacementCref(map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, cref: Arc<ComponentRef::NFComponentRef>, exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn applyReplacements(map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, fnBody: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

fn applyReplacements2(map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn applyReplacementsDim(map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, dim: Arc<Dimension::NFDimension>) -> Arc<Dimension::NFDimension> {
    todo!()
}

fn assertAssignedOutput(outputNode: Arc<InstNode::InstNode>, value: Arc<Expression::NFExpression>) -> () {
    todo!()
}

fn assignArrayElement(arrayExp: Arc<Expression::NFExpression>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, value: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn assignExp(lhs: Arc<Expression::NFExpression>, rhs: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn assignRecord(lhs: Arc<Expression::NFExpression>, rhs: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn assignSubscriptedVariable(variable: Mutable::Mutable<Arc<Expression::NFExpression>>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, value: Arc<Expression::NFExpression>) -> () {
    todo!()
}

pub fn assignVariable(variable: Arc<Expression::NFExpression>, value: Arc<Expression::NFExpression>) -> () {
    todo!()
}

fn buildBinding(node: Arc<InstNode::InstNode>, map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, mutableParams: bool, buildArrayBinding: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

fn buildRecordBinding(recordNode: Arc<InstNode::InstNode>, map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, mutableParams: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

fn cacheLibrary(libName: String, libHandle: i32) -> () {
    todo!()
}

fn callExternalFunction(extName: String, r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, extArgs: metamodelica::List<Arc<Expression::NFExpression>>, outputRef: Arc<ComponentRef::NFComponentRef>, extAnnotation: Option<Arc<SCode::Annotation>>, debug: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

fn checkExtReturnValue(cref: Arc<ComponentRef::NFComponentRef>, info: SourceInfo) -> () {
    todo!()
}

pub fn clearLibraryCache() -> () {
    todo!()
}

fn createArgumentMap(inputs: metamodelica::List<Arc<InstNode::InstNode>>, outputs: metamodelica::List<Arc<InstNode::InstNode>>, locals: metamodelica::List<Arc<InstNode::InstNode>>, args: metamodelica::List<Arc<Expression::NFExpression>>, mutableParams: bool, buildArrayBinding: bool) -> UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>> {
    todo!()
}

fn createResult(map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, outputs: metamodelica::List<Arc<InstNode::InstNode>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evalTargetFromSource(source: Arc<DAE::ElementSource>, context: i32, currentContext: i32) -> Arc<EvalTarget::EvalTarget> {
    todo!()
}

pub fn evaluate(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateAssert(condition: Arc<Expression::NFExpression>, assertStmt: Arc<Statement::NFStatement>, source: Arc<DAE::ElementSource>, context: i32) -> FlowControl {
    todo!()
}

fn evaluateAssignment(lhsExp: Arc<Expression::NFExpression>, rhsExp: Arc<Expression::NFExpression>, source: Arc<DAE::ElementSource>, context: i32) -> FlowControl {
    todo!()
}

pub fn evaluateExternal(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateExternal2(name: String, r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, extArgs: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateExternal3(name: String, args: metamodelica::List<Arc<Expression::NFExpression>>) -> () {
    todo!()
}

fn evaluateFor(iterator: Arc<InstNode::InstNode>, range: Option<Arc<Expression::NFExpression>>, forBody: metamodelica::List<Arc<Statement::NFStatement>>, source: Arc<DAE::ElementSource>, context: i32) -> FlowControl {
    todo!()
}

fn evaluateIf(branches: metamodelica::List<(metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>)>, source: Arc<DAE::ElementSource>, context: i32) -> FlowControl {
    todo!()
}

fn evaluateNoRetCall(callExp: Arc<Expression::NFExpression>, source: Arc<DAE::ElementSource>, context: i32) -> FlowControl {
    todo!()
}

pub fn evaluateNormal(r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, context: i32) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evaluateRecordConstructor(r#fn: Arc<Function::Function>, ty: Arc<Type::NFType>, args: metamodelica::List<Arc<Expression::NFExpression>>, evaluate: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateReplacement(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateReplacement2(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateStatement(stmt: Arc<Statement::NFStatement>, context: i32) -> FlowControl {
    todo!()
}

fn evaluateStatements(stmts: metamodelica::List<Arc<Statement::NFStatement>>, context: i32) -> FlowControl {
    todo!()
}

fn evaluateWhile(condition: Arc<Expression::NFExpression>, body: metamodelica::List<Arc<Statement::NFStatement>>, source: Arc<DAE::ElementSource>, context: i32) -> FlowControl {
    todo!()
}

fn freeLibraryFunction(fnHandle: i32, debug: bool) -> () {
    todo!()
}

fn getBindingExp(node: Arc<InstNode::InstNode>, map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, mutableParams: bool, buildArrayBinding: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

fn getExternalOutputResult(outputNode: Arc<InstNode::InstNode>, map: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn loadLibraryFunction(libName: String, fnName: String, extAnnotation: Option<Arc<SCode::Annotation>>, debug: bool, info: SourceInfo) -> i32 {
    todo!()
}

fn lookupLibraryInCache(libName: String) -> i32 {
    todo!()
}

fn makeExternalArg(arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeExternalResult(values: metamodelica::List<Arc<Expression::NFExpression>>, outputRef: Arc<ComponentRef::NFComponentRef>, extArgs: metamodelica::List<Arc<Expression::NFExpression>>, outputs: metamodelica::List<Arc<InstNode::InstNode>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn mapExternalArg(extArg: Arc<Expression::NFExpression>, argMap: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<InstNode::InstNode>>, r#fn: Arc<Function::Function>) -> (Arc<Expression::NFExpression>, FFI::ArgSpec) {
    todo!()
}

fn mapExternalArgs(r#fn: Arc<Function::Function>, inputArgs: metamodelica::List<Arc<Expression::NFExpression>>, extArgs: metamodelica::List<Arc<Expression::NFExpression>>) -> (Vec<Arc<Expression::NFExpression>>, Vec<FFI::ArgSpec>) {
    todo!()
}

fn mergeFunctionApplicationArgs(oldFn: Arc<Function::Function>, oldArgs: metamodelica::List<Arc<Expression::NFExpression>>, newFn: Arc<Function::Function>, newArgs: metamodelica::List<Arc<Expression::NFExpression>>, argNames: metamodelica::List<String>) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

fn optimizeBody(body: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

fn optimizeStatement(stmt: Arc<Statement::NFStatement>) -> Arc<Statement::NFStatement> {
    todo!()
}

fn parseExternalAnnotation(name: String, ann: Arc<SCode::Annotation>) -> metamodelica::List<String> {
    todo!()
}

fn parseExternalAnnotationExp(exp: Arc<Absyn::Exp>, strl: metamodelica::List<String>) -> metamodelica::List<String> {
    todo!()
}

