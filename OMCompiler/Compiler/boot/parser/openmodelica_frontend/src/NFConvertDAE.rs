// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::ComponentReference;
use crate::DAE;
use crate::ElementSource;
use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use crate::SCode;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Util;

pub struct VARIABLE_CONVERSION_SETTINGS {
    pub isFunctionParameter: bool,
    pub addTypeToSource: bool,
}

pub type VariableConversionSettings = VARIABLE_CONVERSION_SETTINGS;

fn addComponentTypeToSource(cref: Arc<ComponentRef::NFComponentRef>, source: Arc<DAE::ElementSource>) -> Arc<DAE::ElementSource> {
    todo!()
}

pub fn convert(flatModel: Arc<FlatModel::NFFlatModel>, functions: Arc<BaseAvlTree::Tree>) -> (DAE::DAElist, Arc<BaseAvlTree::Tree>) {
    todo!()
}

fn convertAlgorithm(alg: Arc<Algorithm::NFAlgorithm>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertAlgorithms(algorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertAssignment(stmt: Arc<Statement::NFStatement>) -> Arc<DAE::Statement> {
    todo!()
}

fn convertBoolVarAttributes(attrs: metamodelica::List<(Arc<Binding::NFBinding>, String)>, isFinal: Option<bool>) -> Option<Arc<DAE::VariableAttributes>> {
    todo!()
}

fn convertEnumVarAttributes(attrs: metamodelica::List<(Arc<Binding::NFBinding>, String)>, isFinal: Option<bool>) -> Option<Arc<DAE::VariableAttributes>> {
    todo!()
}

fn convertEquation(eq: Arc<Equation::NFEquation>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertEquations(equations: metamodelica::List<Arc<Equation::NFEquation>>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertExternalDecl(extDecl: Arc<Sections::NFSections>, parameters: metamodelica::List<Arc<DAE::Element>>) -> DAE::FunctionDefinition {
    todo!()
}

fn convertExternalDeclArg(exp: Arc<Expression::NFExpression>) -> DAE::ExtArg {
    todo!()
}

fn convertExternalDeclOutput(cref: Arc<ComponentRef::NFComponentRef>) -> DAE::ExtArg {
    todo!()
}

fn convertForEquation(forEquation: Arc<Equation::NFEquation>, isInitial: bool) -> Arc<DAE::Element> {
    todo!()
}

fn convertForStatement(forStmt: Arc<Statement::NFStatement>) -> Arc<DAE::Statement> {
    todo!()
}

fn convertForStatementParallelVar(var: (SourceInfo, Arc<ComponentRef::NFComponentRef>)) -> (SourceInfo, Arc<DAE::ComponentRef>) {
    todo!()
}

fn convertFunction(func: Arc<Function::Function>) -> DAE::Function {
    todo!()
}

fn convertFunctionParam(node: Arc<InstNode::InstNode>) -> Arc<DAE::Element> {
    todo!()
}

fn convertFunctionParams(params: metamodelica::List<Arc<InstNode::InstNode>>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

pub fn convertFunctionTree(funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

fn convertIfEquation(ifBranches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, source: Arc<DAE::ElementSource>, isInitial: bool) -> Arc<DAE::Element> {
    todo!()
}

fn convertIfStatement(ifBranches: metamodelica::List<(metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>)>, source: Arc<DAE::ElementSource>) -> Arc<DAE::Statement> {
    todo!()
}

fn convertInitialAlgorithm(alg: Arc<Algorithm::NFAlgorithm>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertInitialAlgorithms(algorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertInitialEquation(eq: Arc<Equation::NFEquation>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertInitialEquations(equations: metamodelica::List<Arc<Equation::NFEquation>>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertIntVarAttributes(attrs: metamodelica::List<(Arc<Binding::NFBinding>, String)>, isFinal: Option<bool>) -> Option<Arc<DAE::VariableAttributes>> {
    todo!()
}

pub fn convertModel(flatModel: Arc<FlatModel::NFFlatModel>) -> DAE::DAElist {
    todo!()
}

fn convertRealVarAttributes(attrs: metamodelica::List<(Arc<Binding::NFBinding>, String)>, isFinal: Option<bool>) -> Option<Arc<DAE::VariableAttributes>> {
    todo!()
}

fn convertStartOrigin(binding: Arc<Binding::NFBinding>) -> Option<Arc<DAE::Exp>> {
    todo!()
}

fn convertStateSelectAttribute(binding: Arc<Binding::NFBinding>) -> Option<DAE::StateSelect> {
    todo!()
}

fn convertStatement(stmt: Arc<Statement::NFStatement>) -> Arc<DAE::Statement> {
    todo!()
}

pub fn convertStatements(statements: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<DAE::Statement>> {
    todo!()
}

fn convertStringVarAttributes(attrs: metamodelica::List<(Arc<Binding::NFBinding>, String)>, isFinal: Option<bool>) -> Option<Arc<DAE::VariableAttributes>> {
    todo!()
}

fn convertUncertaintyAttribute(binding: Arc<Binding::NFBinding>) -> Option<DAE::Uncertainty> {
    todo!()
}

fn convertVarAttribute(binding: Arc<Binding::NFBinding>) -> Option<Arc<DAE::Exp>> {
    todo!()
}

fn convertVarAttributes(attrs: metamodelica::List<(Arc<Binding::NFBinding>, String)>, ty: Arc<Type::NFType>, compAttrs: Arc<Attributes::NFAttributes>) -> Option<Arc<DAE::VariableAttributes>> {
    todo!()
}

fn convertVariable(var: Arc<Variable::NFVariable>, settings: VariableConversionSettings) -> Arc<DAE::Element> {
    todo!()
}

fn convertVariables(variables: metamodelica::List<Arc<Variable::NFVariable>>, elements: metamodelica::List<Arc<DAE::Element>>) -> metamodelica::List<Arc<DAE::Element>> {
    todo!()
}

fn convertWhenEquation(whenBranches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, source: Arc<DAE::ElementSource>) -> Arc<DAE::Element> {
    todo!()
}

fn convertWhenStatement(whenBranches: metamodelica::List<(metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>)>, source: Arc<DAE::ElementSource>) -> Arc<DAE::Statement> {
    todo!()
}

fn getStateSelectName(exp: Arc<Expression::NFExpression>) -> String {
    todo!()
}

fn lookupStateSelectMember(name: String) -> DAE::StateSelect {
    todo!()
}

fn lookupUncertaintyMember(name: String) -> DAE::Uncertainty {
    todo!()
}

fn makeDAEVar(cref: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>, binding: Option<Arc<DAE::Exp>>, attr: Arc<Attributes::NFAttributes>, vis: Visibility, vattr: Option<Arc<DAE::VariableAttributes>>, comment: Arc<SCode::Comment>, settings: VariableConversionSettings, info: SourceInfo, encrypted: bool) -> Arc<DAE::Element> {
    todo!()
}

pub fn makeTypeRecordVar(component: Arc<InstNode::InstNode>) -> Arc<DAE::Var> {
    todo!()
}

pub fn makeTypeVar(component: Arc<InstNode::InstNode>) -> Arc<DAE::Var> {
    todo!()
}

pub fn makeTypeVars(complexCls: Arc<InstNode::InstNode>) -> metamodelica::List<Arc<DAE::Var>> {
    todo!()
}

fn stripScopePrefixCref(cref: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

fn stripScopePrefixCrefExp(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn stripScopePrefixExp(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn stripScopePrefixFromDim(dim: Arc<Dimension::NFDimension>) -> Arc<Dimension::NFDimension> {
    todo!()
}

