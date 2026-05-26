// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

pub fn checkModel(flatModel: Arc<FlatModel::NFFlatModel>) -> (i32, i32) {
    todo!()
}

pub fn countAlgorithmSize(alg: Arc<Algorithm::NFAlgorithm>) -> i32 {
    todo!()
}

pub fn countVariableSize(var: Arc<Variable::NFVariable>, variables: i32, equations: i32) -> (i32, i32) {
    todo!()
}

fn statementOutputCrefFinder(exp: Arc<Expression::NFExpression>, crefs: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn statementOutputCrefFinder2(exp: Arc<Expression::NFExpression>, crefs: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn statementOutputs(stmt: Arc<Statement::NFStatement>, crefs: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

