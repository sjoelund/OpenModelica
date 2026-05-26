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

pub fn initialize(mut fileSuffix: ArcStr, mut listenToAll: bool, mut port: i32) -> Option<i32> {
    let mut zmqSocket: Option<i32> = None;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ZeroMQ_initialize"), lang: Some("C"), output_: Some(CREF_IDENT { name: "zmqSocket", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fileSuffix", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "listenToAll", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "port", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/ZeroMQ.mo", isReadOnly: false, lineNumberStart: 52, columnNumberStart: 96, lineNumberEnd: 52, columnNumberEnd: 109, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/ZeroMQ.mo", isReadOnly: false, lineNumberStart: 52, columnNumberStart: 88, lineNumberEnd: 52, columnNumberEnd: 109, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    zmqSocket
}

pub fn handleRequest(mut zmqSocket: Option<i32>) -> ArcStr {
    let mut request: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ZeroMQ_handleRequest"), lang: Some("C"), output_: Some(CREF_IDENT { name: "request", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "zmqSocket", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/ZeroMQ.mo", isReadOnly: false, lineNumberStart: 59, columnNumberStart: 77, lineNumberEnd: 59, columnNumberEnd: 90, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/ZeroMQ.mo", isReadOnly: false, lineNumberStart: 59, columnNumberStart: 69, lineNumberEnd: 59, columnNumberEnd: 90, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    request
}

pub fn sendReply(mut zmqSocket: Option<i32>, mut reply: ArcStr) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ZeroMQ_sendReply"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "zmqSocket", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "reply", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/ZeroMQ.mo", isReadOnly: false, lineNumberStart: 66, columnNumberStart: 70, lineNumberEnd: 66, columnNumberEnd: 83, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/ZeroMQ.mo", isReadOnly: false, lineNumberStart: 66, columnNumberStart: 62, lineNumberEnd: 66, columnNumberEnd: 83, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn close(mut zmqSocket: Option<i32>) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ZeroMQ_close"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "zmqSocket", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/ZeroMQ.mo", isReadOnly: false, lineNumberStart: 72, columnNumberStart: 59, lineNumberEnd: 72, columnNumberEnd: 72, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/ZeroMQ.mo", isReadOnly: false, lineNumberStart: 72, columnNumberStart: 51, lineNumberEnd: 72, columnNumberEnd: 72, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

