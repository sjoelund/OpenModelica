// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::DAE;
use crate::DAEDump;
use crate::Dump;
use crate::ElementSource;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::Util;

pub enum ElementType {
    ROOT_CLASS,
    CLASS,
    FUNCTION,
    COMPONENT,
    EQUATION,
    ALGORITHM,
    OTHER,
}

pub fn appendAnnotationMod(r#mod: Arc<SCode::Mod>, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendAnnotationSubMod(r#mod: Arc<SCode::SubMod>, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendComment(comment: Arc<SCode::Comment>, elementType: ElementType, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendCommentAnnotation(comment: Arc<SCode::Comment>, elementType: ElementType, indent: String, ending: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendCommentOpt(comment: Option<Arc<SCode::Comment>>, elementType: ElementType, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendCommentString(comment: Arc<SCode::Comment>, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendElementSourceComment(source: Arc<DAE::ElementSource>, elementType: ElementType, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendElementSourceCommentAnnotation(source: Arc<DAE::ElementSource>, elementType: ElementType, indent: String, ending: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendElementSourceCommentString(source: Arc<DAE::ElementSource>, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn appendExp(exp: Arc<Absyn::Exp>, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn filterRootClassAnnotations(r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    todo!()
}

pub fn quoteCref(exp: Arc<Absyn::Exp>, dummy: i32) -> (Arc<Absyn::Exp>, i32) {
    todo!()
}

