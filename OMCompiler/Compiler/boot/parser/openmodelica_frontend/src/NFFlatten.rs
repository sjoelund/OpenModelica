// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn::Path;
use crate::Absyn;
use crate::DAE;
use crate::ElementSource;
use crate::NFAlgorithm as Algorithm;
use crate::NFArrayConnections as ArrayConnections;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFCardinalityTable as CardinalityTable;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectEquations as ConnectEquations;
use crate::NFConnection as Connection;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFConnector::Face;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEvalConstants as EvalConstants;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpandableConnectors as ExpandableConnectors;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFlatModel as FlatModel;
use crate::NFFunction::Function;
use crate::NFInline as Inline;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFInstUtil as InstUtil;
use crate::NFModifier::Modifier;
use crate::NFOCConnectionGraph;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Parallelism;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSimplifyModel as SimplifyModel;
use crate::NFStatement as Statement;
use crate::NFStreamFlowAlias as StreamFlowAlias;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use crate::SCode;
use crate::SCodeUtil;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::List;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

pub enum ComponentType {
    NORMAL,
    COMPLEX,
    RECORD,
}

pub type DeletedVariables = UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>;

pub struct SETTINGS {
    pub scalarize: bool,
    pub arrayConnect: bool,
    pub nfAPI: bool,
    pub relaxedErrorChecking: bool,
    pub newBackend: bool,
    pub vectorizeBindings: bool,
    pub implicitStartAttribute: bool,
    pub minimalEval: bool,
}

pub type FlattenSettings = SETTINGS;

pub type FunctionTree = Arc<BaseAvlTree::Tree>;

pub mod FunctionTreeImpl {
    use super::*;
    pub type ConflictFunc = fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Arc<Function::Function>;

    pub type Key = Arc<Path>;

    pub enum Tree {
        NODE {
            key: Arc<Path>,
            value: Arc<Function::Function>,
            height: i32,
            left: Arc<Tree>,
            right: Arc<Tree>,
        },
        LEAF {
            key: Arc<Path>,
            value: Arc<Function::Function>,
        },
        EMPTY,
    }
    pub use Tree::*;

    pub type Value = Arc<Function::Function>;

    pub type ValueNode = Arc<Path>;

    pub fn add(inTree: Arc<Tree>, inKey: Arc<Path>, inValue: Arc<Function::Function>, conflictFunc: fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Arc<Function::Function>) -> Arc<Tree> {
        todo!()
    }

    pub fn addConflictFail(newValue: Arc<Function::Function>, oldValue: Arc<Function::Function>, key: Arc<Path>) -> Arc<Function::Function> {
        todo!()
    }

    pub fn addConflictKeep(newValue: Arc<Function::Function>, oldValue: Arc<Function::Function>, key: Arc<Path>) -> Arc<Function::Function> {
        todo!()
    }

    pub fn addConflictReplace(newValue: Arc<Function::Function>, oldValue: Arc<Function::Function>, key: Arc<Path>) -> Arc<Function::Function> {
        todo!()
    }

    pub fn addList(tree: Arc<Tree>, inValues: metamodelica::List<(Arc<Function::Function>, Arc<Path>)>, conflictFunc: fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Arc<Function::Function>) -> Arc<Tree> {
        todo!()
    }

    pub fn addUpdate(tree: Arc<Tree>, key: Arc<Path>, r#fn: fn(Option<Arc<Function::Function>>) -> Arc<Function::Function>) -> Arc<Tree> {
        todo!()
    }

    fn balance(inTree: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    fn calculateBalance(inNode: Arc<Tree>) -> i32 {
        todo!()
    }

    pub fn fold<FT>(inTree: Arc<Tree>, inFunc: fn(Arc<Path>, Arc<Function::Function>, FT) -> FT, inStartValue: FT) -> FT {
        todo!()
    }

    pub fn foldCond<FT>(tree: Arc<Tree>, foldFunc: fn(Arc<Path>, Arc<Function::Function>, FT) -> (FT, bool), value: FT) -> FT {
        todo!()
    }

    pub fn fold_2<FT1, FT2>(tree: Arc<Tree>, foldFunc: fn(Arc<Path>, Arc<Function::Function>, FT1, FT2) -> (FT1, FT2), foldArg1: FT1, foldArg2: FT2) -> (FT1, FT2) {
        todo!()
    }

    pub fn forEach(tree: Arc<Tree>, func: fn(Arc<Path>, Arc<Function::Function>) -> ()) -> () {
        todo!()
    }

    pub fn fromList(inValues: metamodelica::List<(Arc<Function::Function>, Arc<Path>)>, conflictFunc: fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Arc<Function::Function>) -> Arc<Tree> {
        todo!()
    }

    pub fn get(tree: Arc<Tree>, key: Arc<Path>) -> Arc<Function::Function> {
        todo!()
    }

    pub fn getOpt(tree: Arc<Tree>, key: Arc<Path>) -> Option<Arc<Function::Function>> {
        todo!()
    }

    pub fn hasKey(inTree: Arc<Tree>, inKey: Arc<Path>) -> bool {
        todo!()
    }

    fn height(inNode: Arc<Tree>) -> i32 {
        todo!()
    }

    pub fn intersection() -> () {
        todo!()
    }

    pub fn isEmpty(tree: Arc<Tree>) -> bool {
        todo!()
    }

    pub fn join(tree: Arc<Tree>, treeToJoin: Arc<Tree>, conflictFunc: fn(Arc<Function::Function>, Arc<Function::Function>, Arc<Path>) -> Arc<Function::Function>) -> Arc<Tree> {
        todo!()
    }

    pub fn keyCompare(inKey1: Arc<Path>, inKey2: Arc<Path>) -> i32 {
        todo!()
    }

    pub fn keyStr(inKey: Arc<Path>) -> String {
        todo!()
    }

    pub fn listKeys(tree: Arc<Tree>, lst: metamodelica::List<Arc<Path>>) -> metamodelica::List<Arc<Path>> {
        todo!()
    }

    pub fn listKeysReverse(inTree: Arc<Tree>, lst: metamodelica::List<Arc<Path>>) -> metamodelica::List<Arc<Path>> {
        todo!()
    }

    pub fn listValues(tree: Arc<Tree>, lst: metamodelica::List<Arc<Function::Function>>) -> metamodelica::List<Arc<Function::Function>> {
        todo!()
    }

    pub fn map(inTree: Arc<Tree>, inFunc: fn(Arc<Path>, Arc<Function::Function>) -> Arc<Function::Function>) -> Arc<Tree> {
        todo!()
    }

    pub fn mapFold<FT>(inTree: Arc<Tree>, inFunc: fn(Arc<Path>, Arc<Function::Function>, FT) -> (Arc<Function::Function>, FT), inStartValue: FT) -> (Arc<Tree>, FT) {
        todo!()
    }

    pub fn new() -> Arc<Tree> {
        todo!()
    }

    pub fn printNodeStr(inNode: Arc<Tree>) -> String {
        todo!()
    }

    pub fn printTreeStr(inTree: Arc<Tree>) -> String {
        todo!()
    }

    fn printTreeStr2(inTree: Arc<Tree>, isLeft: bool, inIndent: String) -> String {
        todo!()
    }

    fn referenceEqOrEmpty(t1: Arc<Tree>, t2: Arc<Tree>) -> bool {
        todo!()
    }

    fn rotateLeft(inNode: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    fn rotateRight(inNode: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    pub fn setTreeLeftRight(orig: Arc<Tree>, left: Arc<Tree>, right: Arc<Tree>) -> Arc<Tree> {
        todo!()
    }

    pub fn smallestKey(tree: Arc<Tree>) -> Arc<Path> {
        todo!()
    }

    pub fn toList(inTree: Arc<Tree>, lst: metamodelica::List<(Arc<Function::Function>, Arc<Path>)>) -> metamodelica::List<(Arc<Function::Function>, Arc<Path>)> {
        todo!()
    }

    pub fn update(tree: Arc<Tree>, key: Arc<Path>, value: Arc<Function::Function>) -> Arc<Tree> {
        todo!()
    }

    pub fn valueStr(inValue: Arc<Function::Function>) -> String {
        todo!()
    }

}

pub mod Prefix {
    use super::*;
    pub enum Prefix {
        PREFIX {
            root: Arc<InstNode::InstNode>,
            prefix: Arc<ComponentRef::NFComponentRef>,
        },
        INDEXED_PREFIX {
            root: Arc<InstNode::InstNode>,
            prefix: Arc<ComponentRef::NFComponentRef>,
            indexedPrefix: Arc<ComponentRef::NFComponentRef>,
        },
    }
    pub use Prefix::*;
    pub fn new(root: Arc<InstNode::InstNode>, indexed: bool) -> Arc<Prefix> {
        todo!()
    }

    pub fn isEmpty(prefix: Arc<Prefix>) -> bool {
        todo!()
    }

    pub fn isIndexed(prefix: Arc<Prefix>) -> bool {
        todo!()
    }

    pub fn push(node: Arc<InstNode::InstNode>, ty: Arc<Type::NFType>, dims: metamodelica::List<Arc<Dimension::NFDimension>>, prefix: Arc<Prefix>) -> Arc<Prefix> {
        todo!()
    }

    pub fn pop(prefix: Arc<Prefix>) -> Arc<Prefix> {
        todo!()
    }

    pub fn prefix(prefix: Arc<Prefix>) -> Arc<ComponentRef::NFComponentRef> {
        todo!()
    }

    pub fn indexedPrefix(prefix: Arc<Prefix>) -> Arc<ComponentRef::NFComponentRef> {
        todo!()
    }

    pub fn toNonIndexedPrefix(prefix: Arc<Prefix>) -> Arc<Prefix> {
        todo!()
    }

    pub fn apply(prefix: Arc<Prefix>, cref: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
        todo!()
    }

    pub fn subscript(subs: metamodelica::List<Arc<Subscript::NFSubscript>>, prefix: Arc<Prefix>) -> Arc<Prefix> {
        todo!()
    }

    pub fn toString(pre: Arc<Prefix>) -> String {
        todo!()
    }

    pub fn rootNode(pre: Arc<Prefix>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    pub fn instanceName(pre: Arc<Prefix>) -> String {
        todo!()
    }

}

pub fn addElementSourceArrayPrefix(source: Arc<DAE::ElementSource>, prefix: Arc<Prefix::Prefix>) -> Arc<DAE::ElementSource> {
    todo!()
}

fn addIterator(exp: Arc<Expression::NFExpression>, prefix: Arc<Prefix::Prefix>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn addIterator_traverse(exp: Arc<Expression::NFExpression>, prefix: Arc<Prefix::Prefix>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn checkDeletedVarRefs(flatModel: Arc<FlatModel::NFFlatModel>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> () {
    todo!()
}

pub fn checkDeletedVarRefsInAlg(alg: Arc<Algorithm::NFAlgorithm>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> () {
    todo!()
}

pub fn checkDeletedVarRefsInEq(eq: Arc<Equation::NFEquation>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> () {
    todo!()
}

pub fn checkDeletedVarRefsInExp(exp: Arc<Expression::NFExpression>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings, info: SourceInfo) -> () {
    todo!()
}

pub fn checkDeletedVarRefsInExp_traverser(exp: Arc<Expression::NFExpression>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings, info: SourceInfo) -> () {
    todo!()
}

pub fn checkDeletedVarRefsInVar(var: Arc<Variable::NFVariable>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> () {
    todo!()
}

pub fn checkParGlobalCref(crefInfo: (SourceInfo, Arc<ComponentRef::NFComponentRef>)) -> () {
    todo!()
}

fn checkUnspecifiedEnumType(ty: Arc<Type::NFType>, node: Arc<InstNode::InstNode>, info: SourceInfo) -> () {
    todo!()
}

pub fn collectAlgorithmFuncs(alg: Arc<Algorithm::NFAlgorithm>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectBindingFuncs(binding: Arc<Binding::NFBinding>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectClassFunctions(clsNode: Arc<InstNode::InstNode>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectComponentFuncs(var: Arc<Variable::NFVariable>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectEqBranchFuncs(branch: Arc<NFEquation::Branch::Branch>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectEquationFuncs(eq: Arc<Equation::NFEquation>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectExpFuncs(exp: Arc<Expression::NFExpression>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectExpFuncs_traverse(exp: Arc<Expression::NFExpression>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectFunctions(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectParallelVariables(stmt: Arc<Statement::NFStatement>, vars: UnorderedMap::UnorderedMap<SourceInfo, Arc<ComponentRef::NFComponentRef>>) -> UnorderedMap::UnorderedMap<SourceInfo, Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

pub fn collectParallelVariablesExp(exp: Arc<Expression::NFExpression>, info: SourceInfo, vars: UnorderedMap::UnorderedMap<SourceInfo, Arc<ComponentRef::NFComponentRef>>) -> UnorderedMap::UnorderedMap<SourceInfo, Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

pub fn collectStatementFuncs(stmt: Arc<Statement::NFStatement>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectStmtBranchFuncs(branch: (metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>), funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectStructor(node: Arc<InstNode::InstNode>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn collectTypeFuncs(ty: Arc<Type::NFType>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

fn containsPrefix(exp: Arc<Expression::NFExpression>, prefix: Arc<Prefix::Prefix>) -> bool {
    todo!()
}

fn containsPrefix_traverse(exp: Arc<Expression::NFExpression>, contains: bool, prefix: Arc<Prefix::Prefix>) -> bool {
    todo!()
}

fn deleteComponent(node: Arc<InstNode::InstNode>, prefix: Arc<Prefix::Prefix>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

pub fn evaluateBindingConnOp(var: Arc<Variable::NFVariable>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> Arc<Variable::NFVariable> {
    todo!()
}

pub fn evaluateConnectionOperators(flatModel: Arc<FlatModel::NFFlatModel>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn evaluateEquationConnOp(eq: Arc<Equation::NFEquation>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> Arc<Equation::NFEquation> {
    todo!()
}

pub fn evaluateEquationsConnOp(equations: metamodelica::List<Arc<Equation::NFEquation>>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn evaluateIfWithConnects(eql: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn evaluateIfWithConnects2(eq: Arc<Equation::NFEquation>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn fillVectorizedBinding(binding: Arc<Binding::NFBinding>, varType: Arc<Type::NFType>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn fillVectorizedVariableBinding(var: Arc<Variable::NFVariable>) -> Arc<Variable::NFVariable> {
    todo!()
}

pub fn flatten(classInst: Arc<InstNode::InstNode>, classPath: Arc<Path>, getConnectionResolved: bool) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn flattenAlgorithms(algorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, prefix: Arc<Prefix::Prefix>) -> metamodelica::List<Arc<Algorithm::NFAlgorithm>> {
    todo!()
}

fn flattenArray(cls: Arc<Class::NFClass>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, prefix: Arc<Prefix::Prefix>, visibility: Visibility, binding: Option<Arc<Binding::NFBinding>>, vars: metamodelica::List<Arc<Variable::NFVariable>>, sections: Arc<Sections::NFSections>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, info: SourceInfo, settings: FlattenSettings) -> (metamodelica::List<Arc<Variable::NFVariable>>, Arc<Sections::NFSections>) {
    todo!()
}

pub fn flattenBinding(binding: Arc<Binding::NFBinding>, prefix: Arc<Prefix::Prefix>, isTypeAttribute: bool) -> Arc<Binding::NFBinding> {
    todo!()
}

fn flattenClass(cls: Arc<Class::NFClass>, prefix: Arc<Prefix::Prefix>, visibility: Visibility, binding: Option<Arc<Binding::NFBinding>>, vars: metamodelica::List<Arc<Variable::NFVariable>>, sections: Arc<Sections::NFSections>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> (metamodelica::List<Arc<Variable::NFVariable>>, Arc<Sections::NFSections>) {
    todo!()
}

fn flattenComplexComponent(node: Arc<InstNode::InstNode>, comp: Arc<Component::NFComponent>, cls: Arc<Class::NFClass>, nodeTy: Arc<Type::NFType>, visibility: Visibility, outerBinding: Option<Arc<Binding::NFBinding>>, prefix: Arc<Prefix::Prefix>, vars: metamodelica::List<Arc<Variable::NFVariable>>, sections: Arc<Sections::NFSections>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> (metamodelica::List<Arc<Variable::NFVariable>>, Arc<Sections::NFSections>) {
    todo!()
}

fn flattenComponent(component: Arc<InstNode::InstNode>, prefix: Arc<Prefix::Prefix>, visibility: Visibility, outerBinding: Option<Arc<Binding::NFBinding>>, vars: metamodelica::List<Arc<Variable::NFVariable>>, sections: Arc<Sections::NFSections>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> (metamodelica::List<Arc<Variable::NFVariable>>, Arc<Sections::NFSections>) {
    todo!()
}

pub fn flattenConditionalArrayIfExp(exp: Arc<Expression::NFExpression>, prefix: Arc<Prefix::Prefix>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn flattenConnection(classInst: Arc<InstNode::InstNode>, classPath: Arc<Path>) -> Arc<Connections::NFConnections> {
    todo!()
}

pub fn flattenCref(cref: Arc<ComponentRef::NFComponentRef>, prefix: Arc<Prefix::Prefix>, info: SourceInfo) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

pub fn flattenCrefSplitSubscripts(cref: Arc<ComponentRef::NFComponentRef>, prefix: Arc<Prefix::Prefix>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

pub fn flattenCrefSplitSubscripts2(sub: Arc<Subscript::NFSubscript>, subMap: UnorderedMap::UnorderedMap<metamodelica::List<Arc<Subscript::NFSubscript>>, Arc<InstNode::InstNode>>) -> Arc<Subscript::NFSubscript> {
    todo!()
}

pub fn flattenDimension(dim: Arc<Dimension::NFDimension>, prefix: Arc<Prefix::Prefix>, info: SourceInfo) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn flattenEqBranch(branch: Arc<NFEquation::Branch::Branch>, prefix: Arc<Prefix::Prefix>, info: SourceInfo, settings: FlattenSettings) -> Arc<NFEquation::Branch::Branch> {
    todo!()
}

pub fn flattenEquation(eq: Arc<Equation::NFEquation>, prefix: Arc<Prefix::Prefix>, equations: metamodelica::List<Arc<Equation::NFEquation>>, settings: FlattenSettings) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn flattenEquations(eql: metamodelica::List<Arc<Equation::NFEquation>>, prefix: Arc<Prefix::Prefix>, settings: FlattenSettings) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn flattenExp(exp: Arc<Expression::NFExpression>, prefix: Arc<Prefix::Prefix>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn flattenExpType(exp: Arc<Expression::NFExpression>, prefix: Arc<Prefix::Prefix>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn flattenFunction(func: Arc<Function::Function>, funcs: Arc<BaseAvlTree::Tree>) -> Arc<BaseAvlTree::Tree> {
    todo!()
}

pub fn flattenIfEquation(eq: Arc<Equation::NFEquation>, prefix: Arc<Prefix::Prefix>, equations: metamodelica::List<Arc<Equation::NFEquation>>, settings: FlattenSettings) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn flattenSections(sections: Arc<Sections::NFSections>, prefix: Arc<Prefix::Prefix>, accumSections: Arc<Sections::NFSections>, settings: FlattenSettings) -> Arc<Sections::NFSections> {
    todo!()
}

fn flattenSimpleComponent(node: Arc<InstNode::InstNode>, comp: Arc<Component::NFComponent>, visibility: Visibility, outerBinding: Option<Arc<Binding::NFBinding>>, typeAttrs: metamodelica::List<Arc<Modifier::Modifier>>, prefix: Arc<Prefix::Prefix>, vars: metamodelica::List<Arc<Variable::NFVariable>>, sections: Arc<Sections::NFSections>, settings: FlattenSettings, children: metamodelica::List<Arc<Variable::NFVariable>>) -> (metamodelica::List<Arc<Variable::NFVariable>>, Arc<Sections::NFSections>) {
    todo!()
}

pub fn flattenStatement(stmt: Arc<Statement::NFStatement>, prefix: Arc<Prefix::Prefix>) -> Arc<Statement::NFStatement> {
    todo!()
}

pub fn flattenStatements(stmts: metamodelica::List<Arc<Statement::NFStatement>>, prefix: Arc<Prefix::Prefix>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn flattenStmtBranch(branch: (metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>), prefix: Arc<Prefix::Prefix>, info: SourceInfo) -> (metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>) {
    todo!()
}

pub fn flattenType(ty: Arc<Type::NFType>, prefix: Arc<Prefix::Prefix>, info: SourceInfo) -> Arc<Type::NFType> {
    todo!()
}

fn flattenTypeAttribute(attr: Arc<Modifier::Modifier>, prefix: Arc<Prefix::Prefix>) -> (Arc<Binding::NFBinding>, String) {
    todo!()
}

pub fn generateTopLevelIOs(variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, connectedLocalIOs: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, exposeLocalIOs: i32) -> (metamodelica::List<Arc<Variable::NFVariable>>, metamodelica::List<Arc<Equation::NFEquation>>) {
    todo!()
}

fn getComponentType(ty: Arc<Type::NFType>, settings: FlattenSettings) -> ComponentType {
    todo!()
}

fn getRecordBindings(binding: Arc<Binding::NFBinding>, comps: Vec<Arc<InstNode::InstNode>>, prefix: Arc<Prefix::Prefix>) -> metamodelica::List<Arc<Binding::NFBinding>> {
    todo!()
}

fn isDeletedComponent(condition: Arc<Binding::NFBinding>, prefix: Arc<Prefix::Prefix>) -> bool {
    todo!()
}

pub fn isDeletedCref(cref: Arc<ComponentRef::NFComponentRef>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

fn isTypeAttributeNamed(name: String, attr: (Arc<Binding::NFBinding>, String)) -> bool {
    todo!()
}

fn makeBindingIterators(prefix: Arc<ComponentRef::NFComponentRef>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>) -> metamodelica::List<Arc<Subscript::NFSubscript>> {
    todo!()
}

pub fn makeIterators(prefix: Arc<ComponentRef::NFComponentRef>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>) -> (metamodelica::List<Arc<InstNode::InstNode>>, metamodelica::List<Arc<Expression::NFExpression>>, metamodelica::List<Arc<Subscript::NFSubscript>>) {
    todo!()
}

fn mergeIterator(cref: Arc<ComponentRef::NFComponentRef>, r#ref: Arc<ComponentRef::NFComponentRef>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

pub fn replaceSplitIndices(exp: Arc<Expression::NFExpression>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, prefix: Arc<Prefix::Prefix>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn replaceSplitIndices2(sub: Arc<Subscript::NFSubscript>, node: Arc<InstNode::InstNode>, index: i32) -> bool {
    todo!()
}

pub fn resolveArrayConnections(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn resolveConnections(flatModel: Arc<FlatModel::NFFlatModel>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

pub fn splitForLoop(forLoop: Arc<Equation::NFEquation>, prefix: Arc<Prefix::Prefix>, equations: metamodelica::List<Arc<Equation::NFEquation>>, settings: FlattenSettings) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn splitForLoop2(forBody: metamodelica::List<Arc<Equation::NFEquation>>, settings: FlattenSettings) -> (metamodelica::List<Arc<Equation::NFEquation>>, metamodelica::List<Arc<Equation::NFEquation>>) {
    todo!()
}

fn splitRecordCref(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn subscriptBindingOpt(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, binding: Option<Arc<Binding::NFBinding>>) -> Option<Arc<Binding::NFBinding>> {
    todo!()
}

pub fn unrollForLoop(forLoop: Arc<Equation::NFEquation>, prefix: Arc<Prefix::Prefix>, equations: metamodelica::List<Arc<Equation::NFEquation>>, settings: FlattenSettings) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn unrollForStatement(stmt: Arc<Statement::NFStatement>, statements: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn unrollForStatements(stmts: metamodelica::List<Arc<Statement::NFStatement>>) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn unrollForStatementsInAlg(alg: Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

pub fn updateForType(forType: Arc<NFStatement::ForType>, forBody: metamodelica::List<Arc<Statement::NFStatement>>) -> Arc<NFStatement::ForType> {
    todo!()
}

pub fn updateVariability(var: Arc<Variable::NFVariable>) -> Arc<Variable::NFVariable> {
    todo!()
}

fn vectorizeAlgorithm(alg: Arc<Algorithm::NFAlgorithm>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, prefix: Arc<Prefix::Prefix>) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

fn vectorizeAlgorithms(algs: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, prefix: Arc<Prefix::Prefix>) -> metamodelica::List<Arc<Algorithm::NFAlgorithm>> {
    todo!()
}

fn vectorizeArray(cls: Arc<Class::NFClass>, cls_ty: Arc<Type::NFType>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, prefix: Arc<Prefix::Prefix>, visibility: Visibility, binding: Option<Arc<Binding::NFBinding>>, vars: metamodelica::List<Arc<Variable::NFVariable>>, sections: Arc<Sections::NFSections>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, deletedVars: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, settings: FlattenSettings) -> (metamodelica::List<Arc<Variable::NFVariable>>, Arc<Sections::NFSections>) {
    todo!()
}

fn vectorizeBinding(binding: Arc<Binding::NFBinding>, prefix: Arc<Prefix::Prefix>) -> Arc<Binding::NFBinding> {
    todo!()
}

fn vectorizeEquation(eqn: Arc<Equation::NFEquation>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, prefix: Arc<Prefix::Prefix>, settings: FlattenSettings, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn vectorizeEquationGeneric(eqn: Arc<Equation::NFEquation>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, prefix: Arc<Prefix::Prefix>) -> Arc<Equation::NFEquation> {
    todo!()
}

fn vectorizeEquations(eql: metamodelica::List<Arc<Equation::NFEquation>>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, prefix: Arc<Prefix::Prefix>, settings: FlattenSettings) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn verifyBinding(var: Arc<Variable::NFVariable>, variability: Variability, binding: Arc<Binding::NFBinding>, settings: FlattenSettings) -> Arc<Variable::NFVariable> {
    todo!()
}

pub fn verifyDimension(dimension: Arc<Dimension::NFDimension>, component: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn verifyDimensions(dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, component: Arc<InstNode::InstNode>) -> () {
    todo!()
}

