// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::DAE;
use crate::NFType;

pub struct CALL_ATTR {
    pub tuple_: bool,
    pub builtin: bool,
    pub isImpure: bool,
    pub isFunctionPointerCall: bool,
    pub inlineType: DAE::InlineType,
    pub tailCall: DAE::TailCall,
}

pub type NFCallAttributes = CALL_ATTR;
pub fn toDAE(attr: Arc<NFCallAttributes>, returnType: Arc<NFType::NFType>) -> Arc<DAE::CallAttributes> {
    todo!()
}


