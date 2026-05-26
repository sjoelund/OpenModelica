// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::DAE;
use crate::ElementSource;
use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFConnection as Connection;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFModifier::ModTable;
use crate::NFModifier::Modifier;
use crate::NFOperator as Operator;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPackage as Package;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections::Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType::Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::Types;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Array;
use openmodelica_util::Config;
use openmodelica_util::ErrorExt;
use openmodelica_util::ErrorTypes;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::List;
use openmodelica_util::System;

pub mod TypingError {
    use super::*;
    pub enum TypingError {
        NO_ERROR,
        OUT_OF_BOUNDS {
            upperBound: i32,
        },
    }
    pub use TypingError::*;
    pub fn isError(error: Arc<TypingError>) -> bool {
        todo!()
    }

}

pub fn checkAssignment(lhsExp: Arc<Expression::NFExpression>, rhsExp: Arc<Expression::NFExpression>, lhsVar: Variability, context: i32, info: SourceInfo) -> () {
    todo!()
}

pub fn checkComponentBindingVariability(name: String, component: Arc<Component::NFComponent>, binding: Arc<Binding::NFBinding>, context: i32) -> Variability {
    todo!()
}

pub fn checkComponentStreamAttribute(cty: i32, ty: Arc<NFType::NFType>, component: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn checkConnector(connExp: Arc<Expression::NFExpression>, info: SourceInfo) -> bool {
    todo!()
}

pub fn checkConnectorForm(cref: Arc<ComponentRef::NFComponentRef>, isConnector: bool) -> bool {
    todo!()
}

pub fn checkConnectorTypeBalance(component: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn checkCyclicDimension(dim: Arc<Dimension::NFDimension>, component: Arc<InstNode::InstNode>, index: i32, info: SourceInfo) -> () {
    todo!()
}

pub fn checkExternalCallResult(result: Arc<ComponentRef::NFComponentRef>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkLhsInWhen(exp: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn checkSizeTypingError(typingError: Arc<TypingError::TypingError>, exp: Arc<Expression::NFExpression>, index: i32, info: SourceInfo) -> () {
    todo!()
}

pub fn checkSubscriptType(subscriptExp: Arc<Expression::NFExpression>, subscriptType: Arc<NFType::NFType>, dimension: Arc<Dimension::NFDimension>, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>) {
    todo!()
}

pub fn checkWhenInitial(condition: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

pub fn collectIteratorCrefs(exp: Arc<Expression::NFExpression>, iterator: Arc<InstNode::InstNode>, crefs: metamodelica::List<(i32, Arc<ComponentRef::NFComponentRef>)>) -> metamodelica::List<(i32, Arc<ComponentRef::NFComponentRef>)> {
    todo!()
}

pub fn collectIteratorCrefs2(exp: Arc<Expression::NFExpression>, iterator: Arc<InstNode::InstNode>, crefs: metamodelica::List<(i32, Arc<ComponentRef::NFComponentRef>)>) -> metamodelica::List<(i32, Arc<ComponentRef::NFComponentRef>)> {
    todo!()
}

pub fn deduceDimensionFromExp(exp: Arc<Expression::NFExpression>, ty: Option<Arc<NFType::NFType>>, index: i32, parentDims: i32, component: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>) {
    todo!()
}

pub fn deduceIterationRange(crefs: metamodelica::List<(i32, Arc<ComponentRef::NFComponentRef>)>, iterator: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn deduceIterationRange2(range1: (i32, Arc<ComponentRef::NFComponentRef>), range2: (i32, Arc<ComponentRef::NFComponentRef>), info: SourceInfo) -> (i32, Arc<ComponentRef::NFComponentRef>) {
    todo!()
}

pub fn deduceIterationRangeEq(eq: Arc<Equation::NFEquation>, iterator: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn deduceIterationRangeExp(exp: Arc<Expression::NFExpression>, iterator: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn deduceIterationRangeStmt(stmt: Arc<Statement::NFStatement>, iterator: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evaluateArrayIf(exp: Arc<Expression::NFExpression>, target: Arc<NFCeval::EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evaluateEnd(exp: Arc<Expression::NFExpression>, dim: Arc<Dimension::NFDimension>, subscriptedExp: Arc<Expression::NFExpression>, index: i32, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn expandProxySubscripts(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, context: i32) -> (metamodelica::List<Arc<Subscript::NFSubscript>>, metamodelica::List<Arc<Expression::NFExpression>>) {
    todo!()
}

pub fn getRecordElementBinding(component: Arc<InstNode::InstNode>, context: i32) -> (Arc<Binding::NFBinding>, i32) {
    todo!()
}

pub fn makeConnectorType(ctree: Arc<ClassTree::ClassTree>, isExpandable: bool) -> Arc<ComplexType::NFComplexType> {
    todo!()
}

pub fn makeDefaultExternalCall(extDecl: Arc<NFSections::NFSections>, fnNode: Arc<InstNode::InstNode>) -> Arc<NFSections::NFSections> {
    todo!()
}

pub fn makeDimension(dimExp: Arc<Expression::NFExpression>, unevaledExp: Arc<Expression::NFExpression>, variability: Variability) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn makeRecordType(constructor: Arc<InstNode::InstNode>) -> Arc<ComplexType::NFComplexType> {
    todo!()
}

pub fn nthDimensionBoundsChecked(ty: Arc<NFType::NFType>, dimIndex: i32, offset: i32) -> (Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>) {
    todo!()
}

pub fn printRangeTypeError(exp1: Arc<Expression::NFExpression>, ty1: Arc<NFType::NFType>, exp2: Arc<Expression::NFExpression>, ty2: Arc<NFType::NFType>, info: SourceInfo) -> () {
    todo!()
}

pub fn simplifyDimExp(dimExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn subscriptDimExp(dimExp: Arc<Expression::NFExpression>, component: Arc<InstNode::InstNode>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn typeAlgorithm(alg: Arc<Algorithm::NFAlgorithm>, context: i32) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

pub fn typeArray(elements: Vec<Arc<Expression::NFExpression>>, isLiteral: bool, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeArrayDim(arrayExp: Arc<Expression::NFExpression>, dimIndex: i32) -> (Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>) {
    todo!()
}

pub fn typeArrayDim2(arrayExp: Arc<Expression::NFExpression>, dimIndex: i32, dimCount: i32) -> (Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>) {
    todo!()
}

pub fn typeAssert(condition: Arc<Expression::NFExpression>, message: Arc<Expression::NFExpression>, level: Arc<Expression::NFExpression>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) {
    todo!()
}

pub fn typeBinding(binding: Arc<Binding::NFBinding>, context: i32) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn typeBindings(cls: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn typeClass(cls: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn typeClassSections(classNode: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn typeClassType(clsNode: Arc<InstNode::InstNode>, componentBinding: Arc<Binding::NFBinding>, context: i32, instanceNode: Arc<InstNode::InstNode>) -> Arc<NFType::NFType> {
    todo!()
}

pub fn typeComponent(component: Arc<InstNode::InstNode>, context: i32, typeChildren: bool) -> Arc<NFType::NFType> {
    todo!()
}

pub fn typeComponentBinding(component: Arc<InstNode::InstNode>, context: i32, typeChildren: bool) -> () {
    todo!()
}

pub fn typeComponentCondition(condition: Arc<Binding::NFBinding>, context: i32, evaluate: bool) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn typeComponentSections(component: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn typeComponentTry(componentNode: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn typeComponents(cls: Arc<InstNode::InstNode>, context: i32, preserveDerived: bool) -> () {
    todo!()
}

pub fn typeCondition(condition: Arc<Expression::NFExpression>, context: i32, source: Arc<DAE::ElementSource>, errorMsg: ErrorTypes::Message, allowVector: bool, allowClock: bool) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability) {
    todo!()
}

pub fn typeConnect(lhsConn: Arc<Expression::NFExpression>, rhsConn: Arc<Expression::NFExpression>, context: i32, scope: Arc<InstNode::InstNode>, source: Arc<DAE::ElementSource>) -> Arc<Equation::NFEquation> {
    todo!()
}

pub fn typeConnector(connExp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, bool) {
    todo!()
}

pub fn typeCref(cref: Arc<ComponentRef::NFComponentRef>, context: i32, info: SourceInfo) -> (Arc<ComponentRef::NFComponentRef>, Arc<NFType::NFType>, Variability, Variability) {
    todo!()
}

pub fn typeCref2(cref: Arc<ComponentRef::NFComponentRef>, context: i32, info: SourceInfo, firstPart: bool) -> (Arc<ComponentRef::NFComponentRef>, Variability) {
    todo!()
}

pub fn typeCrefDim(cref: Arc<ComponentRef::NFComponentRef>, dimIndex: i32, context: i32, info: SourceInfo) -> (Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>) {
    todo!()
}

pub fn typeCrefExp(cref: Arc<ComponentRef::NFComponentRef>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeDimension(dimensions: Vec<Arc<Dimension::NFDimension>>, index: i32, component: Arc<InstNode::InstNode>, binding: Arc<Binding::NFBinding>, context: i32, info: SourceInfo) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn typeDimensions(dimensions: Vec<Arc<Dimension::NFDimension>>, component: Arc<InstNode::InstNode>, binding: Arc<Binding::NFBinding>, context: i32, info: SourceInfo) -> Vec<Arc<Dimension::NFDimension>> {
    todo!()
}

pub fn typeEqualityEquation(lhsExp: Arc<Expression::NFExpression>, rhsExp: Arc<Expression::NFExpression>, context: i32, scope: Arc<InstNode::InstNode>, source: Arc<DAE::ElementSource>) -> Arc<Equation::NFEquation> {
    todo!()
}

pub fn typeEquation(eq: Arc<Equation::NFEquation>, context: i32) -> Arc<Equation::NFEquation> {
    todo!()
}

pub fn typeExp(exp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo, retype: bool) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeExpDim(exp: Arc<Expression::NFExpression>, dimIndex: i32, context: i32, info: SourceInfo) -> (Arc<Dimension::NFDimension>, Option<Arc<Expression::NFExpression>>, Arc<TypingError::TypingError>) {
    todo!()
}

pub fn typeExpl(expl: metamodelica::List<Arc<Expression::NFExpression>>, context: i32, info: SourceInfo) -> (metamodelica::List<Arc<Expression::NFExpression>>, metamodelica::List<Arc<NFType::NFType>>, metamodelica::List<Variability>) {
    todo!()
}

pub fn typeExternalArg(arg: Arc<Expression::NFExpression>, info: SourceInfo, node: Arc<InstNode::InstNode>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn typeFunctionSections(classNode: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn typeIfEquation(branches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, context: i32, scope: Arc<InstNode::InstNode>, source: Arc<DAE::ElementSource>) -> Arc<Equation::NFEquation> {
    todo!()
}

pub fn typeIfExpression(ifExp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeIterator(iterator: Arc<InstNode::InstNode>, range: Arc<Expression::NFExpression>, context: i32, structural: bool) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeMatrix(elements: metamodelica::List<metamodelica::List<Arc<Expression::NFExpression>>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeMatrixComma(elements: metamodelica::List<Arc<Expression::NFExpression>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeOperatorArg(arg: Arc<Expression::NFExpression>, expectedType: Arc<NFType::NFType>, context: i32, operatorName: String, argName: String, argIndex: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Variability) {
    todo!()
}

pub fn typeRange(rangeExp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeRecordExp(exp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeReinit(crefExp: Arc<Expression::NFExpression>, exp: Arc<Expression::NFExpression>, context: i32, source: Arc<DAE::ElementSource>) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) {
    todo!()
}

pub fn typeSize(sizeExp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo, evaluate: bool) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeStatement(st: Arc<Statement::NFStatement>, context: i32) -> Arc<Statement::NFStatement> {
    todo!()
}

pub fn typeStatements(alg: metamodelica::List<Arc<Statement::NFStatement>>, context: i32) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn typeStructor(node: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn typeSubscript(subscript: Arc<Subscript::NFSubscript>, dimension: Arc<Dimension::NFDimension>, subscriptedExp: Arc<Expression::NFExpression>, index: i32, context: i32, info: SourceInfo, checkSubscript: bool) -> (Arc<Subscript::NFSubscript>, Variability) {
    todo!()
}

pub fn typeSubscriptedExp(exp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeSubscriptedExp2(exp: Arc<Expression::NFExpression>, splitSubs: metamodelica::List<Arc<Subscript::NFSubscript>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeSubscripts(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, crefType: Arc<NFType::NFType>, subscriptedExp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo, checkSubscripts: bool) -> (metamodelica::List<Arc<Subscript::NFSubscript>>, Variability) {
    todo!()
}

pub fn typeTuple(elements: metamodelica::List<Arc<Expression::NFExpression>>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability, Purity) {
    todo!()
}

pub fn typeTypeAttribute(attribute: Arc<Modifier::Modifier>, attrType: Arc<NFType::NFType>, component: Arc<InstNode::InstNode>, context: i32) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn typeWhenCondition(condition: Arc<Expression::NFExpression>, context: i32, source: Arc<DAE::ElementSource>, allowClock: bool) -> (Arc<Expression::NFExpression>, Arc<NFType::NFType>, Variability) {
    todo!()
}

pub fn typeWhenEquation(branches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, context: i32, scope: Arc<InstNode::InstNode>, source: Arc<DAE::ElementSource>) -> Arc<Equation::NFEquation> {
    todo!()
}

