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

pub fn getVersionNr() -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Settings_getVersionNr"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 45, columnNumberStart: 67, lineNumberEnd: 45, columnNumberEnd: 80, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 45, columnNumberStart: 59, lineNumberEnd: 45, columnNumberEnd: 80, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outString
}

pub fn setTempDirectoryPath(mut inString: ArcStr) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SettingsImpl__setTempDirectoryPath"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inString", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 51, columnNumberStart: 80, lineNumberEnd: 51, columnNumberEnd: 93, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 51, columnNumberStart: 72, lineNumberEnd: 51, columnNumberEnd: 93, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn getTempDirectoryPath() -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Settings_getTempDirectoryPath"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 57, columnNumberStart: 77, lineNumberEnd: 57, columnNumberEnd: 90, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 57, columnNumberStart: 69, lineNumberEnd: 57, columnNumberEnd: 90, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outString
}

pub fn setInstallationDirectoryPath(mut inString: ArcStr) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SettingsImpl__setInstallationDirectoryPath"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inString", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 63, columnNumberStart: 88, lineNumberEnd: 63, columnNumberEnd: 101, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 63, columnNumberStart: 80, lineNumberEnd: 63, columnNumberEnd: 101, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn getInstallationDirectoryPath() -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Settings_getInstallationDirectoryPath"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 68, columnNumberStart: 86, lineNumberEnd: 68, columnNumberEnd: 99, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 68, columnNumberStart: 78, lineNumberEnd: 68, columnNumberEnd: 99, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outString)
}

pub fn setModelicaPath(mut inString: ArcStr) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SettingsImpl__setModelicaPath"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inString", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 73, columnNumberStart: 75, lineNumberEnd: 73, columnNumberEnd: 88, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 73, columnNumberStart: 67, lineNumberEnd: 73, columnNumberEnd: 88, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn getModelicaPath(mut runningTestsuite: bool) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Settings_getModelicaPath"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 79, columnNumberStart: 88, lineNumberEnd: 79, columnNumberEnd: 101, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 79, columnNumberStart: 80, lineNumberEnd: 79, columnNumberEnd: 101, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(outString)
}

pub fn getHomeDir(mut runningTestsuite: bool) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Settings_getHomeDir"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 85, columnNumberStart: 83, lineNumberEnd: 85, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 85, columnNumberStart: 75, lineNumberEnd: 85, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outString
}

pub fn getEcho() -> i32 {
    let mut echo: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Settings_getEcho"), lang: Some("C"), output_: Some(CREF_IDENT { name: "echo", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 90, columnNumberStart: 59, lineNumberEnd: 90, columnNumberEnd: 72, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 90, columnNumberStart: 51, lineNumberEnd: 90, columnNumberEnd: 72, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    echo
}

pub fn setEcho(mut echo: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Settings_setEcho"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "echo", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 95, columnNumberStart: 58, lineNumberEnd: 95, columnNumberEnd: 71, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/Settings.mo", isReadOnly: false, lineNumberStart: 95, columnNumberStart: 50, lineNumberEnd: 95, columnNumberEnd: 71, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

/* TODO: Implement an external C function for bootstrapped omc or remove me. DO NOT SIMPLY REMOVE THIS COMMENT
public function dumpSettings
  external "C" Settings_dumpSettings() annotation(Library = "omcruntime");
end dumpSettings;*/
