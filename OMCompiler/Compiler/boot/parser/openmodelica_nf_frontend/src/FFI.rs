// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::NFExpression as Expression;
use crate::NFType as Type;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ArgSpec {
    INPUT = 1,
    OUTPUT = 2,
    LOCAL = 3,
}
impl PartialOrd for ArgSpec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ArgSpec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for ArgSpec {
    fn default() -> Self { Self::INPUT }
}

pub fn callFunction(mut fnHandle: i32, mut args: metamodelica::Array<Arc<Expression::NFExpression>>, mut specs: metamodelica::Array<ArgSpec>, mut returnType: Arc<Type::NFType>) -> Result<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>)> {
    let mut returnValue: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outputArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("FFI_callFunction"), lang: Some("C"), output_: Some(CREF_IDENT { name: "returnValue", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fnHandle", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "args", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "specs", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "returnType", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outputArgs", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/FFI.mo", isReadOnly: false, lineNumberStart: 60, columnNumberStart: 24, lineNumberEnd: 60, columnNumberEnd: 37, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/FFI.mo", isReadOnly: false, lineNumberStart: 60, columnNumberStart: 16, lineNumberEnd: 60, columnNumberEnd: 37, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok((returnValue, outputArgs))
}

