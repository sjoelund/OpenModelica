// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEquation::Branch;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPackage as Package;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use metamodelica::Dangerous::*;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;

pub struct SETTINGS {
    pub scalarize: bool,
}

pub type EvalSettings = SETTINGS;

pub fn evaluate(flatModel: Arc<FlatModel::NFFlatModel>, context: i32) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn evaluateAlgorithm(alg: Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

pub fn evaluateAlgorithms(algs: metamodelica::List<Arc<Algorithm::NFAlgorithm>>) -> metamodelica::List<Arc<Algorithm::NFAlgorithm>> {
    todo!()
}

pub fn evaluateBinding(binding: Arc<Binding::NFBinding>, prefix: Arc<ComponentRef::NFComponentRef>, structural: bool, variability: Variability, context: i32) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn evaluateDimension(dim: Arc<Dimension::NFDimension>, info: SourceInfo) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn evaluateEqBranch(branch: Arc<Branch::Branch>, info: SourceInfo) -> Arc<Branch::Branch> {
    todo!()
}

pub fn evaluateEquation(eq: Arc<Equation::NFEquation>) -> Arc<Equation::NFEquation> {
    todo!()
}

pub fn evaluateEquations(eql: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn evaluateExp(exp: Arc<Expression::NFExpression>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evaluateExpTraverser(exp: Arc<Expression::NFExpression>, info: SourceInfo, changed: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn evaluateFuncExp(exp: Arc<Expression::NFExpression>, fnNode: Arc<InstNode::InstNode>, evaluateAll: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evaluateFuncExpTraverser(exp: Arc<Expression::NFExpression>, fnNode: Arc<InstNode::InstNode>, evaluateAll: bool, changed: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn evaluateFunction(func: Arc<Function::Function>) -> Arc<Function::Function> {
    todo!()
}

pub fn evaluateIfExp(exp: Arc<Expression::NFExpression>, info: SourceInfo) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

pub fn evaluateRecordDeclaration(recordNode: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn evaluateRecordDeclarationField(fieldNode: Arc<InstNode::InstNode>, recordNode: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn evaluateStatement(stmt: Arc<Statement::NFStatement>) -> Arc<Statement::NFStatement> {
    todo!()
}

pub fn evaluateStatements(stmts: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn evaluateStmtBranch(branch: (metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>), info: SourceInfo) -> (metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>) {
    todo!()
}

pub fn evaluateType(ty: Arc<Type::NFType>, info: SourceInfo) -> Arc<Type::NFType> {
    todo!()
}

pub fn evaluateTypeAttribute(attribute: (Arc<Binding::NFBinding>, String), prefix: Arc<ComponentRef::NFComponentRef>, context: i32) -> (Arc<Binding::NFBinding>, String) {
    todo!()
}

pub fn evaluateVariable(var: Arc<Variable::NFVariable>, context: i32, settings: EvalSettings) -> Arc<Variable::NFVariable> {
    todo!()
}

pub fn isLocalFunctionVariable(cref: Arc<ComponentRef::NFComponentRef>, fnNode: Arc<InstNode::InstNode>) -> bool {
    todo!()
}

