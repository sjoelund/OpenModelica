// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::ElementSource;
use crate::ExpressionDump;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFUnit as Unit;
use crate::NFVariable as Variable;
use openmodelica_util::ExecStat::execStat;

fn Errorfunction(inexpList: metamodelica::List<(NFUnit::Unit, Arc<Expression::NFExpression>)>, inEq: Arc<Equation::NFEquation>, inHtU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> () {
    todo!()
}

fn Errorfunction2(inexpList: metamodelica::List<(NFUnit::Unit, Arc<Expression::NFExpression>)>, inHtU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> String {
    todo!()
}

pub type FunctionUnitCache = UnorderedMap::UnorderedMap<Functionargs, String>;

pub struct FUNCTIONUNITS {
    pub name: String,
    pub invars: metamodelica::List<String>,
    pub outvars: metamodelica::List<String>,
    pub inunits: metamodelica::List<String>,
    pub outunits: metamodelica::List<String>,
}

pub type Functionargs = FUNCTIONUNITS;

fn addUnit2HtS2U(name: String, unit: NFUnit::Unit, inHtS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>) -> () {
    todo!()
}

fn addUnit2HtU2S(name: String, unit: NFUnit::Unit, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> () {
    todo!()
}

fn checkModelConsistency(variables: metamodelica::List<Arc<Variable::NFVariable>>, equations: metamodelica::List<Arc<Equation::NFEquation>>, initialEquations: metamodelica::List<Arc<Equation::NFEquation>>, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>) -> UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

pub fn checkUnits(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

fn convertUnitStringToUnit(var: Arc<Variable::NFVariable>, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> () {
    todo!()
}

fn foldBindingExp(var: Arc<Variable::NFVariable>, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>, dumpEqInitStruct: bool) -> () {
    todo!()
}

fn foldCallArg(args: metamodelica::List<Arc<Expression::NFExpression>>, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>) -> metamodelica::List<metamodelica::List<(NFUnit::Unit, Arc<Expression::NFExpression>)>> {
    todo!()
}

fn foldCallArg1(args: metamodelica::List<Arc<Expression::NFExpression>>, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>, inUnit: NFUnit::Unit, units: metamodelica::List<String>, vars: metamodelica::List<String>, fnName: String) -> metamodelica::List<metamodelica::List<(NFUnit::Unit, Arc<Expression::NFExpression>)>> {
    todo!()
}

fn foldEquation(eq: Arc<Equation::NFEquation>, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>, dumpEqInitStruct: bool) -> () {
    todo!()
}

fn foldEquation2(eq: Arc<Equation::NFEquation>, dumpEqInitStruct: bool, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>) -> metamodelica::List<metamodelica::List<(NFUnit::Unit, Arc<Expression::NFExpression>)>> {
    todo!()
}

fn getCallUnits(fnName: String, call: Arc<Call::NFCall>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>) -> (metamodelica::List<String>, metamodelica::List<String>, metamodelica::List<String>, metamodelica::List<String>) {
    todo!()
}

fn getUnitStringFromExp(unitExp: Arc<Expression::NFExpression>) -> String {
    todo!()
}

fn insertUnitInEquation(eq: Arc<Expression::NFExpression>, unit: NFUnit::Unit, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>) -> (NFUnit::Unit, metamodelica::List<metamodelica::List<(NFUnit::Unit, Arc<Expression::NFExpression>)>>) {
    todo!()
}

fn insertUnitInEquationCall(call: Arc<Call::NFCall>, unit: NFUnit::Unit, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, fnCache: UnorderedMap::UnorderedMap<Functionargs, String>) -> (NFUnit::Unit, metamodelica::List<metamodelica::List<(NFUnit::Unit, Arc<Expression::NFExpression>)>>) {
    todo!()
}

fn insertUnitString(unit: NFUnit::Unit, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> () {
    todo!()
}

fn makeNewCref(paramName: String, fnName: String) -> Arc<Expression::NFExpression> {
    todo!()
}

fn notification(inHtCr2U1: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, inHtCr2U2: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, inHtU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> () {
    todo!()
}

fn notification2(inLt1: metamodelica::List<(NFUnit::Unit, Arc<ComponentRef::NFComponentRef>)>, inHtCr2U2: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, inHtU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> String {
    todo!()
}

fn parse(unitString: String, cref: Arc<ComponentRef::NFComponentRef>, htS2U: UnorderedMap::UnorderedMap<NFUnit::Unit, String>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>, info: SourceInfo) -> NFUnit::Unit {
    todo!()
}

fn parseFunctionUnits(funcName: String, func: Arc<Function::Function>) -> Functionargs {
    todo!()
}

fn unitTypesEqual(unit1: NFUnit::Unit, unit2: NFUnit::Unit, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>) -> (bool, NFUnit::Unit) {
    todo!()
}

fn updateHtCr2U(cref: Arc<ComponentRef::NFComponentRef>, unit: NFUnit::Unit, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn updateModel(flatModel: Arc<FlatModel::NFFlatModel>, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

fn updateVariable(var: Arc<Variable::NFVariable>, htCr2U: UnorderedMap::UnorderedMap<NFUnit::Unit, Arc<ComponentRef::NFComponentRef>>, htU2S: UnorderedMap::UnorderedMap<String, NFUnit::Unit>) -> Arc<Variable::NFVariable> {
    todo!()
}

