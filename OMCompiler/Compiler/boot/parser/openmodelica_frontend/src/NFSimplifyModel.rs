// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE;
use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use metamodelica::Dangerous::*;
use openmodelica_util::ExecStat::execStat;

pub fn combineBinaries(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn removeEmptyFunctionArguments(exp: Arc<Expression::NFExpression>, isArg: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn removeEmptyTupleElements(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn simplify(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn simplifyAlgorithm(alg: Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

pub fn simplifyAlgorithms(algs: metamodelica::List<Arc<Algorithm::NFAlgorithm>>) -> metamodelica::List<Arc<Algorithm::NFAlgorithm>> {
    todo!()
}

pub fn simplifyAssignment(stmt: Arc<Statement::NFStatement>, statements: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn simplifyBinding(binding: Arc<Binding::NFBinding>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn simplifyDimension(dim: Arc<Dimension::NFDimension>) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn simplifyEqualityEquation(eq: Arc<Equation::NFEquation>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn simplifyEquation(eq: Arc<Equation::NFEquation>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn simplifyEquations(eql: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn simplifyFunction(func: Arc<Function::Function>) -> () {
    todo!()
}

pub fn simplifyIfEqBranches(branches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, scope: Arc<InstNode::InstNode>, src: Arc<DAE::ElementSource>, elements: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn simplifyIfStmtBranches<ElemT>(branches: metamodelica::List<(metamodelica::List<ElemT>, Arc<Expression::NFExpression>)>, src: Arc<DAE::ElementSource>, makeFunc: fn(metamodelica::List<(metamodelica::List<ElemT>, Arc<Expression::NFExpression>)>, Arc<DAE::ElementSource>) -> ElemT, simplifyFunc: fn(metamodelica::List<ElemT>) -> metamodelica::List<ElemT>, elements: metamodelica::List<ElemT>) -> metamodelica::List<ElemT> {
    todo!()
}

pub fn simplifyStatement(stmt: Arc<Statement::NFStatement>, statements: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn simplifyStatements(stmts: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn simplifyTupleElement<ElementT>(lhsTuple: metamodelica::List<Arc<Expression::NFExpression>>, rhsTuple: metamodelica::List<Arc<Expression::NFExpression>>, ty: Arc<Type::NFType>, src: Arc<DAE::ElementSource>, makeFn: fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, Arc<DAE::ElementSource>) -> ElementT, statements: metamodelica::List<ElementT>) -> metamodelica::List<ElementT> {
    todo!()
}

pub fn simplifyTypeAttribute(attribute: (Arc<Binding::NFBinding>, String)) -> (Arc<Binding::NFBinding>, String) {
    todo!()
}

pub fn simplifyVariable(var: Arc<Variable::NFVariable>) -> Arc<Variable::NFVariable> {
    todo!()
}

pub fn simplifyWhenBranches(branches: metamodelica::List<(metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>)>) -> metamodelica::List<(metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>)> {
    todo!()
}

