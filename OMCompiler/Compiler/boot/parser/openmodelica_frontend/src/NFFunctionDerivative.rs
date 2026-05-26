// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::DAE;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunctionDerivative as FunctionDerivative;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use crate::SCode;
use crate::SCodeDump;
use crate::SCodeUtil;
use openmodelica_util::Util;

pub struct FUNCTION_DER {
    pub derivativeFn: Arc<InstNode::InstNode>,
    pub derivedFn: Arc<InstNode::InstNode>,
    pub order: Arc<Expression::NFExpression>,
    pub conditions: metamodelica::List<(Condition, String, i32)>,
    pub lowerOrderDerivatives: metamodelica::List<Arc<InstNode::InstNode>>,
}

pub type NFFunctionDerivative = FUNCTION_DER;
pub enum Condition {
    ZERO_DERIVATIVE,
    NO_DERIVATIVE,
}

pub fn instDerivatives(fnNode: Arc<InstNode::InstNode>, r#fn: Arc<Function::Function>) -> metamodelica::List<Arc<FunctionDerivative>> {
    todo!()
}

pub fn typeDerivative(fnDer: Arc<FunctionDerivative>) -> () {
    todo!()
}

pub fn toDAE(fnDer: Arc<FunctionDerivative>) -> DAE::FunctionDefinition {
    todo!()
}

pub fn conditionToDAE(cond: (Condition, String, i32)) -> (DAE::derivativeCond, i32) {
    todo!()
}

pub fn toSubMod(fnDer: Arc<FunctionDerivative>) -> Arc<SCode::SubMod> {
    todo!()
}

pub fn perfectFit(fnDer: Arc<FunctionDerivative>, interface_map: UnorderedMap::UnorderedMap<bool, String>) -> bool {
    todo!()
}

fn conditionToString(condition: Condition) -> String {
    todo!()
}

fn getDerivativeAnnotations(definition: Arc<SCode::Element>) -> metamodelica::List<Arc<SCode::Mod>> {
    todo!()
}

fn instDerivativeMod(r#mod: Arc<SCode::Mod>, fnNode: Arc<InstNode::InstNode>, r#fn: Arc<Function::Function>, scope: Arc<InstNode::InstNode>, fnDers: metamodelica::List<Arc<FunctionDerivative>>) -> metamodelica::List<Arc<FunctionDerivative>> {
    todo!()
}

fn getDerivativeAttributes(attrs: metamodelica::List<Arc<SCode::SubMod>>, r#fn: Arc<Function::Function>, scope: Arc<InstNode::InstNode>, info: SourceInfo) -> (Arc<Expression::NFExpression>, metamodelica::List<(Condition, String, i32)>) {
    todo!()
}

fn getInputIndex(name: String, r#fn: Arc<Function::Function>, info: SourceInfo) -> i32 {
    todo!()
}

fn addLowerOrderDerivative(fnNode: Arc<InstNode::InstNode>, lowerDerNode: Arc<InstNode::InstNode>) -> () {
    todo!()
}

fn addLowerOrderDerivative2(r#fn: Arc<Function::Function>, lowerDerNode: Arc<InstNode::InstNode>) -> Arc<Function::Function> {
    todo!()
}


