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

pub fn haveCorba() -> bool {
    let mut b: bool = false;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Corba_haveCorba"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "OpenModelicaCorba" }, tail: Nil } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 59, columnNumberStart: 55, lineNumberEnd: 59, columnNumberEnd: 91, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 59, columnNumberStart: 47, lineNumberEnd: 59, columnNumberEnd: 91, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn setObjectReferenceFilePath(mut inObjectReferenceFilePath: ArcStr) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Corba_setObjectReferenceFilePath"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inObjectReferenceFilePath", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "OpenModelicaCorba" }, tail: Nil } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 64, columnNumberStart: 95, lineNumberEnd: 64, columnNumberEnd: 131, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 64, columnNumberStart: 87, lineNumberEnd: 64, columnNumberEnd: 131, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn setSessionName(mut inSessionName: ArcStr) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Corba_setSessionName"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inSessionName", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "OpenModelicaCorba" }, tail: Nil } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 70, columnNumberStart: 71, lineNumberEnd: 70, columnNumberEnd: 107, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 70, columnNumberStart: 63, lineNumberEnd: 70, columnNumberEnd: 107, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn initialize() -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Corba_initialize"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "OpenModelicaCorba" }, tail: Nil } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 75, columnNumberStart: 54, lineNumberEnd: 75, columnNumberEnd: 89, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 75, columnNumberStart: 46, lineNumberEnd: 75, columnNumberEnd: 89, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn waitForCommand() -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Corba_waitForCommand"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "OpenModelicaCorba" }, tail: Nil } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 81, columnNumberStart: 68, lineNumberEnd: 81, columnNumberEnd: 103, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 81, columnNumberStart: 60, lineNumberEnd: 81, columnNumberEnd: 103, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outString)
}

pub fn sendreply(mut inString: ArcStr) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Corba_sendreply"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inString", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "OpenModelicaCorba" }, tail: Nil } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 87, columnNumberStart: 61, lineNumberEnd: 87, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 87, columnNumberStart: 53, lineNumberEnd: 87, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn close() -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Corba_close"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "OpenModelicaCorba" }, tail: Nil } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 92, columnNumberStart: 49, lineNumberEnd: 92, columnNumberEnd: 84, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Corba.mo", isReadOnly: false, lineNumberStart: 92, columnNumberStart: 41, lineNumberEnd: 92, columnNumberEnd: 84, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

