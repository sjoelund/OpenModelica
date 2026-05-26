// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::DAE;
use crate::NFCall as Call;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunctionInverse as FunctionInverse;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::SCode;
use crate::SCodeUtil;

pub struct FUNCTION_INV {
    pub inputParam: Arc<ComponentRef::NFComponentRef>,
    pub inverseCall: Arc<Expression::NFExpression>,
    pub info: SourceInfo,
}

pub type NFFunctionInverse = FUNCTION_INV;
pub fn instInverses(fnNode: Arc<InstNode::InstNode>, r#fn: Arc<Function::Function>) -> Vec<Arc<FunctionInverse>> {
    todo!()
}

pub fn typeInverse(fnInv: Arc<FunctionInverse>) -> Arc<FunctionInverse> {
    todo!()
}

pub fn toDAE(fnInv: Arc<FunctionInverse>) -> DAE::FunctionDefinition {
    todo!()
}

pub fn toSubMod(fnInv: Arc<FunctionInverse>) -> Arc<SCode::SubMod> {
    todo!()
}

pub fn getFunction(fnInv: Arc<FunctionInverse>) -> Arc<Function::Function> {
    todo!()
}

fn getInverseAnnotations(definition: Arc<SCode::Element>) -> metamodelica::List<Arc<SCode::Mod>> {
    todo!()
}

fn instInverseMod(r#mod: Arc<SCode::Mod>, fnNode: Arc<InstNode::InstNode>, r#fn: Arc<Function::Function>, fnInvs: metamodelica::List<Arc<FunctionInverse>>) -> metamodelica::List<Arc<FunctionInverse>> {
    todo!()
}

fn instInverseSubMod(submod: Arc<SCode::SubMod>, fnNode: Arc<InstNode::InstNode>, r#fn: Arc<Function::Function>, info: SourceInfo, fnInvs: metamodelica::List<Arc<FunctionInverse>>) -> metamodelica::List<Arc<FunctionInverse>> {
    todo!()
}


