// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::DAE;
use crate::Dump;
use crate::ElementSource;
use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use crate::SCode;
use crate::SCodeDump;
use crate::SCodeUtil;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;

pub type MergeNameMap = UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>;

pub fn addTrailingWholeIndices(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn collectExtractorModelVariables(vars: metamodelica::List<Arc<Variable::NFVariable>>) -> (metamodelica::List<Arc<Variable::NFVariable>>, metamodelica::List<Arc<Variable::NFVariable>>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

pub fn combineSubscripts(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn combineSubscriptsExp(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn createExtractorModel(flatModel: Arc<FlatModel::NFFlatModel>, funcs: Arc<BaseAvlTree::Tree>) -> (Arc<FlatModel::NFFlatModel>, Arc<BaseAvlTree::Tree>) {
    todo!()
}

pub fn createExtractorModelDummyEq(var: Arc<Variable::NFVariable>, varType: String, r#fn: Arc<Function::Function>, args: metamodelica::List<Arc<Expression::NFExpression>>, funcs: Arc<BaseAvlTree::Tree>, index: i32) -> (Arc<Equation::NFEquation>, Arc<BaseAvlTree::Tree>, i32) {
    todo!()
}

pub fn createExtractorModelDummyFn(connectors: metamodelica::List<Arc<Variable::NFVariable>>) -> Arc<Function::Function> {
    todo!()
}

pub fn createExtractorModelDummyFnInput(var: Arc<Variable::NFVariable>) -> Arc<SCode::Element> {
    todo!()
}

pub fn dumpFlatModel(flatModel: Arc<FlatModel::NFFlatModel>, functions: Arc<BaseAvlTree::Tree>) -> String {
    todo!()
}

pub fn dumpFlatModelDebug(stage: String, flatModel: Arc<FlatModel::NFFlatModel>, functions: Arc<BaseAvlTree::Tree>) -> () {
    todo!()
}

pub fn expandSlicedCrefs(flatModel: Arc<FlatModel::NFFlatModel>, functions: Arc<BaseAvlTree::Tree>) -> (Arc<FlatModel::NFFlatModel>, Arc<BaseAvlTree::Tree>) {
    todo!()
}

pub fn expandSlicedCrefsAlg(alg: Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

pub fn expandSlicedCrefsEq(eq: Arc<Equation::NFEquation>) -> Arc<Equation::NFEquation> {
    todo!()
}

pub fn expandSlicedCrefsExp(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandSlicedCrefsExp2(cref: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandSlicedCrefsFunction(fnPath: Arc<Absyn::Path>, r#fn: Arc<Function::Function>) -> Arc<Function::Function> {
    todo!()
}

pub fn expandSlicedCrefsStmt(stmt: Arc<Statement::NFStatement>) -> Arc<Statement::NFStatement> {
    todo!()
}

pub fn getComponentSignature(element: Arc<SCode::Element>) -> String {
    todo!()
}

pub fn getModBindings(r#mod: Arc<SCode::Mod>, names: metamodelica::List<Arc<Absyn::Path>>, bindings: metamodelica::List<metamodelica::List<Arc<Absyn::Exp>>>) -> metamodelica::List<metamodelica::List<Arc<Absyn::Exp>>> {
    todo!()
}

pub fn getModNames(r#mod: Arc<SCode::Mod>, name: metamodelica::List<String>, names: metamodelica::List<Arc<Absyn::Path>>) -> metamodelica::List<Arc<Absyn::Path>> {
    todo!()
}

pub fn getModSignature(r#mod: Arc<SCode::Mod>, name: String) -> String {
    todo!()
}

pub fn isMergeableComponent(element: Arc<SCode::Element>) -> bool {
    todo!()
}

pub fn isMergeableMod(r#mod: Arc<SCode::Mod>) -> bool {
    todo!()
}

pub fn isMergeableType(ty: Arc<Absyn::TypeSpec>) -> bool {
    todo!()
}

pub fn lookupMod(name: Arc<Absyn::Path>, r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    todo!()
}

pub fn lookupModBinding(name: Arc<Absyn::Path>, r#mod: Arc<SCode::Mod>) -> Arc<Absyn::Exp> {
    todo!()
}

pub fn makeMergeMap(elements: metamodelica::List<Arc<SCode::Element>>) -> (metamodelica::List<metamodelica::List<Arc<SCode::Element>>>, metamodelica::List<Arc<SCode::Element>>) {
    todo!()
}

pub fn makeMergeNameMap() -> UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String> {
    todo!()
}

pub fn makeModPath(name: metamodelica::List<String>) -> Arc<Absyn::Path> {
    todo!()
}

pub fn mergeComponents(components: metamodelica::List<Arc<SCode::Element>>, prefix: String, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<SCode::Element> {
    todo!()
}

pub fn mergeMods(mods: metamodelica::List<Arc<SCode::Mod>>) -> Arc<SCode::Mod> {
    todo!()
}

pub fn mergeMods2(r#mod: Arc<SCode::Mod>, bindingMap: UnorderedMap::UnorderedMap<Arc<Absyn::Exp>, Arc<Absyn::Path>>, name: metamodelica::List<String>) -> Arc<SCode::Mod> {
    todo!()
}

pub fn mergeScalars(node: Arc<InstNode::InstNode>, classPath: Arc<Absyn::Path>, isRootClass: bool, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn mergeScalars2(cls: Arc<SCode::Element>, classPath: Arc<Absyn::Path>, isRootClass: bool, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<SCode::Element> {
    todo!()
}

pub fn mergeScalars3(elements: metamodelica::List<Arc<SCode::Element>>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> metamodelica::List<Arc<SCode::Element>> {
    todo!()
}

pub fn mergeScalarsAlgs(algs: metamodelica::List<Arc<SCode::AlgorithmSection>>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> metamodelica::List<Arc<SCode::AlgorithmSection>> {
    todo!()
}

pub fn mergeScalarsComponentBinding(node: Arc<InstNode::InstNode>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> () {
    todo!()
}

pub fn mergeScalarsComponentBindings(node: Arc<InstNode::InstNode>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> () {
    todo!()
}

pub fn mergeScalarsCref(cref: Arc<Absyn::ComponentRef>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<Absyn::ComponentRef> {
    todo!()
}

pub fn mergeScalarsElement(element: Arc<SCode::Element>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<SCode::Element> {
    todo!()
}

pub fn mergeScalarsEq(eq: Arc<SCode::Equation>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<SCode::Equation> {
    todo!()
}

pub fn mergeScalarsEql(eql: metamodelica::List<Arc<SCode::Equation>>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> metamodelica::List<Arc<SCode::Equation>> {
    todo!()
}

pub fn mergeScalarsExp(exp: Arc<Absyn::Exp>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> (Arc<Absyn::Exp>, UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) {
    todo!()
}

pub fn mergeScalarsExps(exp: Arc<Absyn::Exp>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<Absyn::Exp> {
    todo!()
}

pub fn mergeScalarsMod(r#mod: Arc<SCode::Mod>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<SCode::Mod> {
    todo!()
}

pub fn mergeScalarsStmt(stmt: Arc<SCode::Statement>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<SCode::Statement> {
    todo!()
}

pub fn mergeScalarsSubMod(r#mod: Arc<SCode::SubMod>, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> Arc<SCode::SubMod> {
    todo!()
}

pub fn printStructuralParameters(flatModel: Arc<FlatModel::NFFlatModel>) -> () {
    todo!()
}

pub fn replaceEmptyArrays(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn replaceEmptyArraysExp(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

