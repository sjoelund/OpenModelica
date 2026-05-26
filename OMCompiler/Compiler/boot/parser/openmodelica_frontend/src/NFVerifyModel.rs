// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE;
use crate::ElementSource;
use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::List;
use openmodelica_util::Util;

pub struct NFVerifyModel;
pub fn verify(flatModel: Arc<FlatModel::NFFlatModel>, isPartial: bool) -> () {
    todo!()
}

fn verifyVariable(var: Arc<Variable::NFVariable>, isPartial: bool) -> () {
    todo!()
}

fn verifyBinding(binding: Arc<Binding::NFBinding>, isPartial: bool) -> () {
    todo!()
}

fn verifyEquation(eq: Arc<Equation::NFEquation>, isPartial: bool) -> () {
    todo!()
}

fn verifyWhenEquation(branches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, source: Arc<DAE::ElementSource>) -> () {
    todo!()
}

fn whenEquationBranchCrefs(eql: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn whenEquationEqualityCrefs(lhsExp: Arc<Expression::NFExpression>, crefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn whenEquationIfCrefs(branches: metamodelica::List<Arc<NFEquation::Branch::Branch>>, source: Arc<DAE::ElementSource>, crefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn checkCrefSetEquality(crefs1: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, crefs2: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, errMsg: ErrorTypes::Message, source: Arc<DAE::ElementSource>) -> () {
    todo!()
}

fn expandCrefSet(crefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn verifyAlgorithm(alg: Arc<Algorithm::NFAlgorithm>, isPartial: bool) -> () {
    todo!()
}

fn verifyStatement(stmt: Arc<Statement::NFStatement>, isPartial: bool) -> () {
    todo!()
}

fn checkSubscriptBounds(exp: Arc<Expression::NFExpression>, isPartial: bool, info: SourceInfo) -> () {
    todo!()
}

fn checkSubscriptBounds_traverser(exp: Arc<Expression::NFExpression>, isPartial: bool, info: SourceInfo) -> () {
    todo!()
}

fn checkSubscriptBoundsCref(cref: Arc<ComponentRef::NFComponentRef>, isPartial: bool, info: SourceInfo) -> () {
    todo!()
}

fn checkDiscreteReal(flatModel: Arc<FlatModel::NFFlatModel>) -> () {
    todo!()
}

fn checkDiscreteRealBranch(branch: Arc<NFEquation::Branch::Branch>, discreteReals: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, when_found: bool) -> UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn checkDiscreteRealEquation(body_eqn: Arc<Equation::NFEquation>, discreteReals: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, when_found: bool) -> () {
    todo!()
}

fn checkDiscreteRealStatement(statement: Arc<Statement::NFStatement>, discreteReals: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>, when_found: bool) -> () {
    todo!()
}

fn checkDiscreteRealExp(exp: Arc<Expression::NFExpression>, discreteReals: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn checkDiscreteRealRecord(cref: Arc<ComponentRef::NFComponentRef>, cls: Arc<InstNode::InstNode>, discreteReals: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}


