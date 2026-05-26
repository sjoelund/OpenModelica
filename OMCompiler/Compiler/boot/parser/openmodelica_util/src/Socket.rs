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

pub fn waitforconnect(mut inInteger: i32) -> i32 {
    let mut outInteger: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Socket_waitforconnect"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outInteger", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 51, columnNumberStart: 79, lineNumberEnd: 51, columnNumberEnd: 92, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 51, columnNumberStart: 71, lineNumberEnd: 51, columnNumberEnd: 92, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outInteger
}

pub fn handlerequest(mut inInteger: i32) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Socket_handlerequest"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 58, columnNumberStart: 77, lineNumberEnd: 58, columnNumberEnd: 90, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 58, columnNumberStart: 69, lineNumberEnd: 58, columnNumberEnd: 90, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outString
}

pub fn sendreply(mut inInteger: i32, mut inString: ArcStr) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Socket_sendreply"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inString", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 65, columnNumberStart: 72, lineNumberEnd: 65, columnNumberEnd: 85, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 65, columnNumberStart: 64, lineNumberEnd: 65, columnNumberEnd: 85, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn close(mut inInteger: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Socket_close"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 71, columnNumberStart: 59, lineNumberEnd: 71, columnNumberEnd: 72, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 71, columnNumberStart: 51, lineNumberEnd: 71, columnNumberEnd: 72, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn cleanup() -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Socket_cleanup"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 76, columnNumberStart: 52, lineNumberEnd: 76, columnNumberEnd: 65, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Socket.mo", isReadOnly: false, lineNumberStart: 76, columnNumberStart: 44, lineNumberEnd: 76, columnNumberEnd: 65, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

