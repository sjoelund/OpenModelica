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

pub fn createFile(mut fileName: ArcStr) -> Result<i32> {
    let mut fileID: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_createFile"), lang: Some("C"), output_: Some(CREF_IDENT { name: "fileID", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fileName", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 51, columnNumberStart: 75, lineNumberEnd: 51, columnNumberEnd: 88, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 51, columnNumberStart: 67, lineNumberEnd: 51, columnNumberEnd: 88, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(fileID)
}

pub fn closeFile(mut fileID: i32) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_closeFile"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fileID", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 57, columnNumberStart: 65, lineNumberEnd: 57, columnNumberEnd: 78, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 57, columnNumberStart: 57, lineNumberEnd: 57, columnNumberEnd: 78, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn deleteFile(mut fileID: i32) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_deleteFile"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fileID", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 63, columnNumberStart: 66, lineNumberEnd: 63, columnNumberEnd: 79, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 63, columnNumberStart: 58, lineNumberEnd: 63, columnNumberEnd: 79, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn clearFile(mut fileID: i32) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_clearFile"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fileID", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 69, columnNumberStart: 65, lineNumberEnd: 69, columnNumberEnd: 78, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 69, columnNumberStart: 57, lineNumberEnd: 69, columnNumberEnd: 78, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn appendFile(mut fileID: i32, mut inString: ArcStr) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_appendFile"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fileID", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inString", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 76, columnNumberStart: 75, lineNumberEnd: 76, columnNumberEnd: 88, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 76, columnNumberStart: 67, lineNumberEnd: 76, columnNumberEnd: 88, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn readFile(mut fileID: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_readFile"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fileID", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 83, columnNumberStart: 74, lineNumberEnd: 83, columnNumberEnd: 87, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 83, columnNumberStart: 66, lineNumberEnd: 83, columnNumberEnd: 87, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outString)
}

pub fn printFile(mut fileID: i32, mut whereToPrint: i32) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_printFile"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "fileID", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "whereToPrint", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 90, columnNumberStart: 78, lineNumberEnd: 90, columnNumberEnd: 91, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 90, columnNumberStart: 70, lineNumberEnd: 90, columnNumberEnd: 91, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn createBuffer() -> Result<i32> {
    let mut bufferID: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_createBuffer"), lang: Some("C"), output_: Some(CREF_IDENT { name: "bufferID", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 96, columnNumberStart: 73, lineNumberEnd: 96, columnNumberEnd: 86, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 96, columnNumberStart: 65, lineNumberEnd: 96, columnNumberEnd: 86, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(bufferID)
}

pub fn appendBuffer(mut bufferID: i32, mut inString: ArcStr) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_appendBuffer"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "bufferID", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inString", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 103, columnNumberStart: 79, lineNumberEnd: 103, columnNumberEnd: 92, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 103, columnNumberStart: 71, lineNumberEnd: 103, columnNumberEnd: 92, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn deleteBuffer(mut bufferID: i32) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_deleteBuffer"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "bufferID", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 109, columnNumberStart: 70, lineNumberEnd: 109, columnNumberEnd: 83, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 109, columnNumberStart: 62, lineNumberEnd: 109, columnNumberEnd: 83, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn clearBuffer(mut bufferID: i32) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_clearBuffer"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "bufferID", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 115, columnNumberStart: 69, lineNumberEnd: 115, columnNumberEnd: 82, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 115, columnNumberStart: 61, lineNumberEnd: 115, columnNumberEnd: 82, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn readBuffer(mut bufferID: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_readBuffer"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "bufferID", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 122, columnNumberStart: 78, lineNumberEnd: 122, columnNumberEnd: 91, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 122, columnNumberStart: 70, lineNumberEnd: 122, columnNumberEnd: 91, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outString)
}

pub fn printBuffer(mut bufferID: i32, mut whereToPrint: i32) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_printBuffer"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "bufferID", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "whereToPrint", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 129, columnNumberStart: 82, lineNumberEnd: 129, columnNumberEnd: 95, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 129, columnNumberStart: 74, lineNumberEnd: 129, columnNumberEnd: 95, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn appendReversedList(mut inStringLst: Arc<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_appendReversedList"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inStringLst", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 136, columnNumberStart: 91, lineNumberEnd: 136, columnNumberEnd: 104, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 136, columnNumberStart: 83, lineNumberEnd: 136, columnNumberEnd: 104, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outString
}

pub fn printReversedList(mut inStringLst: Arc<metamodelica::List<ArcStr>>, mut whereToPrint: i32) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("IOStreamExt_printReversedList"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inStringLst", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "whereToPrint", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 143, columnNumberStart: 92, lineNumberEnd: 143, columnNumberEnd: 105, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/IOStreamExt.mo", isReadOnly: false, lineNumberStart: 143, columnNumberStart: 84, lineNumberEnd: 143, columnNumberEnd: 105, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

