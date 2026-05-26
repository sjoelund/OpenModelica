// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::BaseModelica;
use crate::DAE::ElementSource;
use crate::DAE;
use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef::ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFFunctionInverse as FunctionInverse;
use crate::NFInline as Inline;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Visibility;
use crate::NFScalarize as Scalarize;
use crate::NFStatement as Statement;
use crate::NFSubscript::Subscript;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use crate::SCode;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::ErrorExt;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub struct FLAT_MODEL {
    pub name: Arc<Absyn::Path>,
    pub variables: metamodelica::List<Arc<Variable::NFVariable>>,
    pub equations: metamodelica::List<Arc<Equation::NFEquation>>,
    pub initialEquations: metamodelica::List<Arc<Equation::NFEquation>>,
    pub algorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>,
    pub initialAlgorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>,
    pub source: Arc<ElementSource>,
}

pub type NFFlatModel = FLAT_MODEL;
pub type TypeMap = UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>;

pub fn mapExp(flatModel: Arc<FlatModel>, r#fn: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<FlatModel> {
    todo!()
}

pub fn mapEquations(flatModel: Arc<FlatModel>, r#fn: fn(Arc<Equation::NFEquation>) -> Arc<Equation::NFEquation>) -> Arc<FlatModel> {
    todo!()
}

pub fn mapAlgorithms(flatModel: Arc<FlatModel>, r#fn: fn(Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm>) -> Arc<FlatModel> {
    todo!()
}

pub fn fullName(flatModel: Arc<FlatModel>) -> String {
    todo!()
}

pub fn className(flatModel: Arc<FlatModel>) -> String {
    todo!()
}

pub fn toString(flatModel: Arc<FlatModel>, functions: Arc<BaseAvlTree::Tree>, printBindingTypes: bool) -> String {
    todo!()
}

pub fn printString(flatModel: Arc<FlatModel>, functions: Arc<BaseAvlTree::Tree>, printBindingTypes: bool) -> () {
    todo!()
}

pub fn toStream(flatModel: Arc<FlatModel>, functions: Arc<BaseAvlTree::Tree>, printBindingTypes: bool) -> IOStream::IOStream {
    todo!()
}

pub fn appendStream(flatModel: Arc<FlatModel>, functions: Arc<BaseAvlTree::Tree>, printBindingTypes: bool, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatString(flatModel: Arc<FlatModel>, functions: Arc<BaseAvlTree::Tree>, printBindingTypes: bool) -> String {
    todo!()
}

pub fn printFlatString(flatModel: Arc<FlatModel>, functions: Arc<BaseAvlTree::Tree>, printBindingTypes: bool) -> () {
    todo!()
}

pub fn toFlatStream(flatModel: Arc<FlatModel>, functions: Arc<BaseAvlTree::Tree>, printBindingTypes: bool) -> IOStream::IOStream {
    todo!()
}

pub fn appendFlatStream(flatModel: Arc<FlatModel>, functions: Arc<BaseAvlTree::Tree>, printBindingTypes: bool, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn inlineFunctions(flatModel: Arc<FlatModel>) -> (Arc<FlatModel>, metamodelica::List<Arc<Function::Function>>) {
    todo!()
}

pub fn inlineFunctions_traverser(exp: Arc<Expression::NFExpression>, funcs: UnorderedSet::UnorderedSet<Arc<Function::Function>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn collectFunctions(exp: Arc<Expression::NFExpression>, funcs: UnorderedSet::UnorderedSet<Arc<Function::Function>>) -> () {
    todo!()
}

pub fn collectFunction(r#fn: Arc<Function::Function>, funcs: UnorderedSet::UnorderedSet<Arc<Function::Function>>) -> () {
    todo!()
}

pub fn collectFlatTypes(flatModel: Arc<FlatModel>, functions: metamodelica::List<Arc<Function::Function>>) -> metamodelica::List<Arc<Type::NFType>> {
    todo!()
}

pub fn collectVariableFlatTypes(var: Arc<Variable::NFVariable>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectFlatType(ty: Arc<Type::NFType>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectBindingFlatTypes(binding: Arc<Binding::NFBinding>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectEquationFlatTypes(eq: Arc<Equation::NFEquation>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectEqBranchFlatTypes(branch: Arc<NFEquation::Branch::Branch>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectAlgorithmFlatTypes(alg: Arc<Algorithm::NFAlgorithm>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectStatementsFlatTypes(statements: metamodelica::List<Arc<Statement::NFStatement>>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectStatementFlatTypes(stmt: Arc<Statement::NFStatement>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectStmtBranchFlatTypes(branch: (metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>), types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectExpFlatTypes(exp: Arc<Expression::NFExpression>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectExpFlatTypes_traverse(exp: Arc<Expression::NFExpression>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>> {
    todo!()
}

pub fn collectFunctionFlatTypes(r#fn: Arc<Function::Function>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn collectComponentFlatTypes(component: Arc<InstNode::InstNode>, types: UnorderedMap::UnorderedMap<Arc<Type::NFType>, Arc<Absyn::Path>>) -> () {
    todo!()
}

pub fn reconstructRecordInstances(variables: metamodelica::List<Arc<Variable::NFVariable>>) -> metamodelica::List<Arc<Variable::NFVariable>> {
    todo!()
}

pub fn reconstructRecordInstance(recordName: Arc<NFComponentRef::NFComponentRef>, variables: metamodelica::List<Arc<Variable::NFVariable>>) -> Arc<Variable::NFVariable> {
    todo!()
}

pub fn typeFlatType(ty: Arc<Type::NFType>) -> Arc<Type::NFType> {
    todo!()
}

pub type ObfuscationMap = UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>;

pub fn obfuscate(flatModel: Arc<FlatModel>) -> Arc<FlatModel> {
    todo!()
}

pub fn addObfuscatedVariable(var: Arc<Variable::NFVariable>, onlyEncrypted: bool, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> () {
    todo!()
}

pub fn obfuscateVariable(var: Arc<Variable::NFVariable>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Variable::NFVariable> {
    todo!()
}

pub fn obfuscateCref(cref: Arc<NFComponentRef::NFComponentRef>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> (Arc<NFComponentRef::NFComponentRef>, bool) {
    todo!()
}

pub fn obfuscateExp(exp: Arc<Expression::NFExpression>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn obfuscateExpOpt(exp: Option<Arc<Expression::NFExpression>>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn obfuscateExp_impl(exp: Arc<Expression::NFExpression>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn obfuscateEquation(eq: Arc<Equation::NFEquation>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Equation::NFEquation> {
    todo!()
}

pub fn obfuscateAlgorithm(alg: Arc<Algorithm::NFAlgorithm>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

pub fn obfuscateStatement(stmt: Arc<Statement::NFStatement>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Statement::NFStatement> {
    todo!()
}

pub fn obfuscateSource(source: Arc<ElementSource>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<ElementSource> {
    todo!()
}

pub fn obfuscateCommentOpt(comment: Option<Arc<SCode::Comment>>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>, stripComment: bool) -> Option<Arc<SCode::Comment>> {
    todo!()
}

pub fn obfuscateComment(comment: Arc<SCode::Comment>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>, stripComment: bool) -> Arc<SCode::Comment> {
    todo!()
}

pub fn obfuscateAnnotationOpt(ann: Option<Arc<SCode::Annotation>>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Option<Arc<SCode::Annotation>> {
    todo!()
}

pub fn obfuscateAnnotation(ann: Arc<SCode::Annotation>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<SCode::Annotation> {
    todo!()
}

pub fn obfuscateAnnotationMod(r#mod: Arc<SCode::Mod>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<SCode::Mod> {
    todo!()
}

pub fn isAllowedAnnotation(r#mod: Arc<SCode::SubMod>) -> bool {
    todo!()
}

pub fn obfuscateAnnotationSubMod(r#mod: Arc<SCode::SubMod>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<SCode::SubMod> {
    todo!()
}

pub fn obfuscateAbsynExpOpt(exp: Option<Arc<Absyn::Exp>>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Option<Arc<Absyn::Exp>> {
    todo!()
}

pub fn obfuscateAbsynExp(exp: Arc<Absyn::Exp>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Absyn::Exp> {
    todo!()
}

pub fn obfuscateAbsynExpTraverse(exp: Arc<Absyn::Exp>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> (Arc<Absyn::Exp>, UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) {
    todo!()
}

pub fn obfuscateAbsynCref(cref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Absyn::ComponentRef> {
    todo!()
}

pub fn obfuscateAbsynCref2(cref: Arc<Absyn::ComponentRef>, nodes: metamodelica::List<Arc<InstNode::InstNode>>, obfuscationMap: UnorderedMap::UnorderedMap<String, Arc<InstNode::InstNode>>) -> Arc<Absyn::ComponentRef> {
    todo!()
}

pub fn hasArrayConnections(flatModel: Arc<FlatModel>, minSize: i32) -> bool {
    todo!()
}

pub fn removeNonTopLevelDirections(flatModel: Arc<FlatModel>) -> Arc<FlatModel> {
    todo!()
}

pub fn moveBindings(flatModel: Arc<FlatModel>) -> Arc<FlatModel> {
    todo!()
}


