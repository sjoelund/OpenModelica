// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use openmodelica_util::Util;

pub fn isBindingNotFixed(binding: Arc<Binding::NFBinding>, requireFinal: bool, maxDepth: i32) -> bool {
    todo!()
}

pub fn isComponentBindingNotFixed(component: Arc<Component::NFComponent>, node: Arc<InstNode::InstNode>, requireFinal: bool, maxDepth: i32, isRecord: bool) -> bool {
    todo!()
}

pub fn isExpressionNotFixed(exp: Arc<Expression::NFExpression>, requireFinal: bool, maxDepth: i32) -> bool {
    todo!()
}

pub fn isStructuralComponent(component: Arc<Component::NFComponent>, compAttrs: Arc<Attributes::NFAttributes>, compBinding: Arc<Binding::NFBinding>, compNode: Arc<InstNode::InstNode>, compEval: bool, parentEval: bool, context: i32) -> bool {
    todo!()
}

pub fn markComponent(component: Arc<Component::NFComponent>, node: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn markDimension(dimension: Arc<Dimension::NFDimension>) -> () {
    todo!()
}

pub fn markExp(exp: Arc<Expression::NFExpression>) -> () {
    todo!()
}

pub fn markExpSize(exp: Arc<Expression::NFExpression>) -> () {
    todo!()
}

pub fn markExpSize_traverser(exp: Arc<Expression::NFExpression>) -> () {
    todo!()
}

pub fn markSubscript(sub: Arc<Subscript::NFSubscript>) -> () {
    todo!()
}

pub fn markSubscripts(exp: Arc<Expression::NFExpression>) -> () {
    todo!()
}

pub fn markSubscriptsInExp(exp: Arc<Expression::NFExpression>) -> () {
    todo!()
}

