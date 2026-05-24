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

use crate::Absyn;
use crate::GlobalScript;

pub fn parse(mut filename: ArcStr, mut infoFilename: ArcStr, mut acceptedGram: i32, mut encoding: ArcStr, mut languageStandardInt: i32, mut strict: bool, mut runningTestsuite: bool, mut libraryPath: ArcStr, mut lveInstance: Option<i32>) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_parse"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outProgram", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "infoFilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "acceptedGram", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "languageStandardInt", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "strict", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "encoding", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "libraryPath", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "lveInstance", subscripts: Nil } }, tail: Nil } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 61, columnNumberStart: 183, lineNumberEnd: 61, columnNumberEnd: 219, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 61, columnNumberStart: 175, lineNumberEnd: 61, columnNumberEnd: 219, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outProgram)
}

pub fn parseexp(mut filename: ArcStr, mut infoFilename: ArcStr, mut acceptedGram: i32, mut languageStandardInt: i32, mut runningTestsuite: bool) -> Result<GlobalScript::Statements> {
    let mut outStatements: GlobalScript::Statements;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_parseexp"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outStatements", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "infoFilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "acceptedGram", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "languageStandardInt", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 72, columnNumberStart: 145, lineNumberEnd: 72, columnNumberEnd: 181, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 72, columnNumberStart: 137, lineNumberEnd: 72, columnNumberEnd: 181, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outStatements)
}

pub fn parsestring(mut r#str: ArcStr, mut infoFilename: ArcStr, mut acceptedGram: i32, mut languageStandardInt: i32, mut strict: bool, mut runningTestsuite: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_parsestring"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outProgram", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "str", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "infoFilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "acceptedGram", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "languageStandardInt", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "strict", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 83, columnNumberStart: 147, lineNumberEnd: 83, columnNumberEnd: 183, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 83, columnNumberStart: 139, lineNumberEnd: 83, columnNumberEnd: 183, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outProgram)
}

pub fn parsestringexp(mut r#str: ArcStr, mut infoFilename: ArcStr, mut acceptedGram: i32, mut languageStandardInt: i32, mut runningTestsuite: bool) -> Result<GlobalScript::Statements> {
    let mut outStatements: GlobalScript::Statements;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_parsestringexp"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outStatements", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "str", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "infoFilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "acceptedGram", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "languageStandardInt", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 93, columnNumberStart: 145, lineNumberEnd: 93, columnNumberEnd: 181, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 93, columnNumberStart: 137, lineNumberEnd: 93, columnNumberEnd: 181, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outStatements)
}

pub fn stringPath(mut r#str: ArcStr, mut infoFilename: ArcStr, mut acceptedGram: i32, mut languageStandardInt: i32, mut runningTestsuite: bool) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_stringPath"), lang: Some("C"), output_: Some(CREF_IDENT { name: "path", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "str", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "infoFilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "acceptedGram", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "languageStandardInt", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 103, columnNumberStart: 133, lineNumberEnd: 103, columnNumberEnd: 169, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 103, columnNumberStart: 125, lineNumberEnd: 103, columnNumberEnd: 169, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(path)
}

pub fn stringCref(mut r#str: ArcStr, mut infoFilename: ArcStr, mut acceptedGram: i32, mut languageStandardInt: i32, mut runningTestsuite: bool) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef>;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_stringCref"), lang: Some("C"), output_: Some(CREF_IDENT { name: "cref", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "str", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "infoFilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "acceptedGram", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "languageStandardInt", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 113, columnNumberStart: 133, lineNumberEnd: 113, columnNumberEnd: 169, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 113, columnNumberStart: 125, lineNumberEnd: 113, columnNumberEnd: 169, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(cref)
}

pub fn stringMod(mut r#str: ArcStr, mut infoFilename: ArcStr, mut acceptedGram: i32, mut languageStandardInt: i32, mut runningTestsuite: bool) -> Result<Arc<Absyn::ElementArg>> {
    let mut cref: Arc<Absyn::ElementArg>;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_stringMod"), lang: Some("C"), output_: Some(CREF_IDENT { name: "cref", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "str", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "infoFilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "acceptedGram", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "languageStandardInt", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 123, columnNumberStart: 132, lineNumberEnd: 123, columnNumberEnd: 168, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 123, columnNumberStart: 124, lineNumberEnd: 123, columnNumberEnd: 168, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(cref)
}

pub fn stringEq(mut r#str: ArcStr, mut infoFilename: ArcStr, mut acceptedGram: i32, mut languageStandardInt: i32, mut runningTestsuite: bool) -> Arc<Absyn::EquationItem> {
    let mut eq: Arc<Absyn::EquationItem>;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_stringEq"), lang: Some("C"), output_: Some(CREF_IDENT { name: "eq", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "str", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "infoFilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "acceptedGram", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "languageStandardInt", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 133, columnNumberStart: 129, lineNumberEnd: 133, columnNumberEnd: 165, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 133, columnNumberStart: 121, lineNumberEnd: 133, columnNumberEnd: 165, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    eq
}

pub fn startLibraryVendorExecutable(mut lvePath: ArcStr) -> (bool, Option<i32>) {
    let mut success: bool;
    let mut lveInstance: Option<i32>;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_startLibraryVendorExecutable"), lang: Some("C"), output_: Some(CREF_IDENT { name: "success", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "lvePath", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "lveInstance", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 141, columnNumberStart: 104, lineNumberEnd: 141, columnNumberEnd: 140, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 141, columnNumberStart: 96, lineNumberEnd: 141, columnNumberEnd: 140, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (success, lveInstance)
}

pub fn checkLVEToolLicense(mut lveInstance: Option<i32>, mut packageName: ArcStr) -> bool {
    let mut status: bool;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_checkLVEToolLicense"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "lveInstance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "packageName", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 149, columnNumberStart: 98, lineNumberEnd: 149, columnNumberEnd: 134, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 149, columnNumberStart: 90, lineNumberEnd: 149, columnNumberEnd: 134, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn checkLVEToolFeature(mut lveInstance: Option<i32>, mut feature: ArcStr) -> bool {
    let mut status: bool;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_checkLVEToolFeature"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "lveInstance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "feature", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 157, columnNumberStart: 94, lineNumberEnd: 157, columnNumberEnd: 130, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 157, columnNumberStart: 86, lineNumberEnd: 157, columnNumberEnd: 130, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn stopLibraryVendorExecutable(mut lveInstance: Option<i32>) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("ParserExt_stopLibraryVendorExecutable"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "lveInstance", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omparse" }, tail: Cons { head: STRING { value: "omantlr3" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 163, columnNumberStart: 86, lineNumberEnd: 163, columnNumberEnd: 122, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/ParserExt.mo", isReadOnly: false, lineNumberStart: 163, columnNumberStart: 78, lineNumberEnd: 163, columnNumberEnd: 122, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

