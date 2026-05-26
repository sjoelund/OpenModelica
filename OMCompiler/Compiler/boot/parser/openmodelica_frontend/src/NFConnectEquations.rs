// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::ComponentReference;
use crate::DAE;
use crate::ElementSource;
use crate::NFBinding as Binding;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCardinalityTable as CardinalityTable;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnector as Connector;
use crate::NFConnector::Face;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatten as Flatten;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Config;
use openmodelica_util::Global;
use openmodelica_util::List;
use openmodelica_util::UnorderedMap;

fn associatedFlowCref(streamCref: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

fn compareCrefStreamSet(cref: Arc<ComponentRef::NFComponentRef>, element: Arc<Connector::NFConnector>) -> bool {
    todo!()
}

fn evaluateActualStream(streamCref: Arc<ComponentRef::NFComponentRef>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> (Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>) {
    todo!()
}

fn evaluateActualStreamMul(crefExp: Arc<Expression::NFExpression>, actualStreamArg: Arc<Expression::NFExpression>, op: Arc<Operator::NFOperator>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateFlowDirection(flowCref: Arc<ComponentRef::NFComponentRef>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> i32 {
    todo!()
}

fn evaluateInStream(cref: Arc<ComponentRef::NFComponentRef>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateOperatorArrayConstructorExp(exp: Arc<Expression::NFExpression>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn evaluateOperatorReductionExp(exp: Arc<Expression::NFExpression>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn evaluateOperators(exp: Arc<Expression::NFExpression>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn flowExp(element: Arc<Connector::NFConnector>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn generateEquations(sets: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, metamodelica::List<metamodelica::List<Arc<Connector::NFConnector>>>) {
    todo!()
}

fn generateFlowEquations(elements: metamodelica::List<Arc<Connector::NFConnector>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn generateInStreamExp(streamCref: Arc<ComponentRef::NFComponentRef>, streams: metamodelica::List<Arc<Connector::NFConnector>>, sets: DisjointSets::Sets, setsArray: Vec<metamodelica::List<Arc<Connector::NFConnector>>>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>, ctable: UnorderedMap::UnorderedMap<i32, String>, flowThreshold: f64) -> Arc<Expression::NFExpression> {
    todo!()
}

fn generatePotentialEquations(elements: metamodelica::List<Arc<Connector::NFConnector>>, connectedLocalIOs: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) {
    todo!()
}

fn generateStreamEquations(elements: metamodelica::List<Arc<Connector::NFConnector>>, flowThreshold: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn getSetType(set: metamodelica::List<Arc<Connector::NFConnector>>) -> i32 {
    todo!()
}

fn isNoFlow(element: Arc<Connector::NFConnector>, attr: String, pred: fn(Arc<Expression::NFExpression>) -> bool, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

fn isNoFlowInside(conn: Arc<Connector::NFConnector>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

fn isNoFlowMinMax(conn: Arc<Connector::NFConnector>, streamCref: Arc<ComponentRef::NFComponentRef>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

fn isNoFlowOutside(conn: Arc<Connector::NFConnector>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

fn isStreamCall(exp: Arc<Expression::NFExpression>) -> bool {
    todo!()
}

fn lookupVarAttr(varName: Arc<ComponentRef::NFComponentRef>, attrName: String, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

fn makeEqualityAssert(lhsCref: Arc<ComponentRef::NFComponentRef>, lhsSource: Arc<DAE::ElementSource>, rhsCref: Arc<ComponentRef::NFComponentRef>, rhsSource: Arc<DAE::ElementSource>) -> Arc<Equation::NFEquation> {
    todo!()
}

fn makeEqualityEquation(lhsCref: Arc<ComponentRef::NFComponentRef>, lhsSource: Arc<DAE::ElementSource>, rhsCref: Arc<ComponentRef::NFComponentRef>, rhsSource: Arc<DAE::ElementSource>) -> Arc<Equation::NFEquation> {
    todo!()
}

fn makeFlowExp(element: Arc<Connector::NFConnector>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeInStreamCall(streamExp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeInStreamDivCall(sum_exp: Arc<Expression::NFExpression>, fallback: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makePositiveMaxCall(flowExp: Arc<Expression::NFExpression>, streamExp: Arc<Expression::NFExpression>, element: Arc<Connector::NFConnector>, flowThreshold: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeSmoothCall(arg: Arc<Expression::NFExpression>, order: i32) -> Arc<Expression::NFExpression> {
    todo!()
}

fn removeStreamSetElement(cref: Arc<ComponentRef::NFComponentRef>, elements: metamodelica::List<Arc<Connector::NFConnector>>) -> metamodelica::List<Arc<Connector::NFConnector>> {
    todo!()
}

fn streamEquationGeneral(outsideElements: metamodelica::List<Arc<Connector::NFConnector>>, insideElements: metamodelica::List<Arc<Connector::NFConnector>>, flowThreshold: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

fn streamFlowExp(element: Arc<Connector::NFConnector>) -> (Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) {
    todo!()
}

fn streamSumEquationExp(outsideElements: metamodelica::List<Arc<Connector::NFConnector>>, insideElements: metamodelica::List<Arc<Connector::NFConnector>>, flowThreshold: Arc<Expression::NFExpression>, fallback: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn sumInside1(element: Arc<Connector::NFConnector>, flowThreshold: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn sumInside2(element: Arc<Connector::NFConnector>, flowThreshold: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn sumMap(elements: metamodelica::List<Arc<Connector::NFConnector>>, func: fn(Arc<Connector::NFConnector>, Arc<Expression::NFExpression>, UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Expression::NFExpression>, flowThreshold: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn sumOutside1(element: Arc<Connector::NFConnector>, flowThreshold: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn sumOutside2(element: Arc<Connector::NFConnector>, flowThreshold: Arc<Expression::NFExpression>, variables: UnorderedMap::UnorderedMap<Arc<Variable::NFVariable>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Expression::NFExpression> {
    todo!()
}

