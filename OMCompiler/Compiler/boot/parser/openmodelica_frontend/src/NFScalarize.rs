// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE;
use crate::ElementSource;
use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableAttributes;
use crate::NFBinding as Binding;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use crate::SCode;
use metamodelica::Dangerous::arrayCreateNoInit;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::List;

pub fn expandComplexCref(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandComplexCref_traverser(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn nextTypeAttributes(names: metamodelica::List<String>, iters: Vec<Arc<ExpressionIterator::NFExpressionIterator>>) -> metamodelica::List<(Arc<Binding::NFBinding>, String)> {
    todo!()
}

pub fn scalarize(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn scalarizeAlgorithm(alg: Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

pub fn scalarizeBackendVariable(var: Arc<Variable::NFVariable>, indices: metamodelica::List<i32>) -> metamodelica::List<Arc<Variable::NFVariable>> {
    todo!()
}

pub fn scalarizeComplexVariable(var: Arc<Variable::NFVariable>, vars: metamodelica::List<Arc<Variable::NFVariable>>) -> metamodelica::List<Arc<Variable::NFVariable>> {
    todo!()
}

pub fn scalarizeEquation(eq: Arc<Equation::NFEquation>, equations: metamodelica::List<Arc<Equation::NFEquation>>, forceScalarize: bool) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn scalarizeEquations(eql: metamodelica::List<Arc<Equation::NFEquation>>, forceScalarize: bool) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn scalarizeIfEquation(branches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, scope: Arc<InstNode::InstNode>, source: Arc<DAE::ElementSource>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn scalarizeIfStatement(branches: metamodelica::List<(metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>)>, source: Arc<DAE::ElementSource>, statements: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn scalarizeStatement(stmt: Arc<Statement::NFStatement>, statements: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn scalarizeStatements(stmts: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn scalarizeTypeAttributes(attrs: metamodelica::List<(Arc<Binding::NFBinding>, String)>) -> (metamodelica::List<String>, Vec<Arc<ExpressionIterator::NFExpressionIterator>>) {
    todo!()
}

pub fn scalarizeVariable(var: Arc<Variable::NFVariable>, vars: metamodelica::List<Arc<Variable::NFVariable>>, forceScalarize: bool) -> metamodelica::List<Arc<Variable::NFVariable>> {
    todo!()
}

pub fn scalarizeVariables(vars: metamodelica::List<Arc<Variable::NFVariable>>, forceScalarize: bool) -> metamodelica::List<Arc<Variable::NFVariable>> {
    todo!()
}

pub fn scalarizeWhenEquation(branches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, scope: Arc<InstNode::InstNode>, source: Arc<DAE::ElementSource>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn scalarizeWhenStatement(branches: metamodelica::List<(metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>)>, source: Arc<DAE::ElementSource>, statements: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn variableHasForcedScalarAttribute(var: Arc<Variable::NFVariable>) -> bool {
    todo!()
}

