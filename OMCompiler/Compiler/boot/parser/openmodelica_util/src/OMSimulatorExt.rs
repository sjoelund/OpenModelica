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

pub fn statusToString(mut status: i32) -> ArcStr {
    let mut outstring: ArcStr = arcstr::literal!("");
    if status.clone() == 0 {
        outstring = (literal!("ok")).clone();
    } else if status.clone() == 1 {
        outstring = (literal!("warning")).clone();
    } else if status.clone() == 2 {
        outstring = (literal!("discard")).clone();
    } else if status.clone() == 3 {
        outstring = (literal!("error")).clone();
    } else if status.clone() == 4 {
        outstring = (literal!("fatal")).clone();
    } else if status.clone() == 5 {
        outstring = (literal!("pending")).clone();
    } else {
        outstring = (literal!("unknown_status")).clone();
    }
    outstring
}

pub fn loadOMSimulator() -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_loadDLL"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 64, columnNumberStart: 66, lineNumberEnd: 64, columnNumberEnd: 79, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 64, columnNumberStart: 58, lineNumberEnd: 64, columnNumberEnd: 79, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn unloadOMSimulator() -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_unloadDLL"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 69, columnNumberStart: 68, lineNumberEnd: 69, columnNumberEnd: 81, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 69, columnNumberStart: 60, lineNumberEnd: 69, columnNumberEnd: 81, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_getVersion() -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getVersion"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outString", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 74, columnNumberStart: 76, lineNumberEnd: 74, columnNumberEnd: 89, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 74, columnNumberStart: 68, lineNumberEnd: 74, columnNumberEnd: 89, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outString
}

pub fn oms_addBus(mut cref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addBus"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 80, columnNumberStart: 73, lineNumberEnd: 80, columnNumberEnd: 86, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 80, columnNumberStart: 65, lineNumberEnd: 80, columnNumberEnd: 86, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addConnection(mut crefA: ArcStr, mut crefB: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addConnection"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "crefA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "crefB", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 87, columnNumberStart: 87, lineNumberEnd: 87, columnNumberEnd: 100, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 87, columnNumberStart: 79, lineNumberEnd: 87, columnNumberEnd: 100, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addConnector(mut cref: ArcStr, mut causality: i32, mut type_: i32) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addConnector"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "causality", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "type_", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 95, columnNumberStart: 95, lineNumberEnd: 95, columnNumberEnd: 108, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 95, columnNumberStart: 87, lineNumberEnd: 95, columnNumberEnd: 108, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addConnectorToBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addConnectorToBus"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "busCref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "connectorCref", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 102, columnNumberStart: 101, lineNumberEnd: 102, columnNumberEnd: 114, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 102, columnNumberStart: 93, lineNumberEnd: 102, columnNumberEnd: 114, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addConnectorToTLMBus(mut busCref: ArcStr, mut connectorCref: ArcStr, mut type_: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addConnectorToTLMBus"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "busCref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "connectorCref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "type_", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 110, columnNumberStart: 110, lineNumberEnd: 110, columnNumberEnd: 123, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 110, columnNumberStart: 102, lineNumberEnd: 110, columnNumberEnd: 123, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addDynamicValueIndicator(mut signal: ArcStr, mut lower: ArcStr, mut upper: ArcStr, mut stepSize: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addDynamicValueIndicator"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "signal", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "lower", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "upper", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "stepSize", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 119, columnNumberStart: 114, lineNumberEnd: 119, columnNumberEnd: 127, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 119, columnNumberStart: 106, lineNumberEnd: 119, columnNumberEnd: 127, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addEventIndicator(mut signal: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addEventIndicator"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "signal", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 125, columnNumberStart: 86, lineNumberEnd: 125, columnNumberEnd: 99, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 125, columnNumberStart: 78, lineNumberEnd: 125, columnNumberEnd: 99, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addExternalModel(mut cref: ArcStr, mut path: ArcStr, mut startscript: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addExternalModel"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "path", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "startscript", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 133, columnNumberStart: 100, lineNumberEnd: 133, columnNumberEnd: 113, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 133, columnNumberStart: 92, lineNumberEnd: 133, columnNumberEnd: 113, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addSignalsToResults(mut cref: ArcStr, mut regex: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addSignalsToResults"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "regex", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 140, columnNumberStart: 92, lineNumberEnd: 140, columnNumberEnd: 105, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 140, columnNumberStart: 84, lineNumberEnd: 140, columnNumberEnd: 105, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addStaticValueIndicator(mut signal: ArcStr, mut lower: metamodelica::Real, mut upper: metamodelica::Real, mut stepSize: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addStaticValueIndicator"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "signal", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "lower", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "upper", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "stepSize", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 149, columnNumberStart: 113, lineNumberEnd: 149, columnNumberEnd: 126, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 149, columnNumberStart: 105, lineNumberEnd: 149, columnNumberEnd: 126, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addSubModel(mut cref: ArcStr, mut fmuPath: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addSubModel"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "fmuPath", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 156, columnNumberStart: 86, lineNumberEnd: 156, columnNumberEnd: 99, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 156, columnNumberStart: 78, lineNumberEnd: 156, columnNumberEnd: 99, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addSystem(mut cref: ArcStr, mut type_: i32) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addSystem"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "type_", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 163, columnNumberStart: 82, lineNumberEnd: 163, columnNumberEnd: 95, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 163, columnNumberStart: 74, lineNumberEnd: 163, columnNumberEnd: 95, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addTimeIndicator(mut signal: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addTimeIndicator"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "signal", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 169, columnNumberStart: 85, lineNumberEnd: 169, columnNumberEnd: 98, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 169, columnNumberStart: 77, lineNumberEnd: 169, columnNumberEnd: 98, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addTLMBus(mut cref: ArcStr, mut domain: i32, mut dimensions: i32, mut interpolation: i32) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addTLMBus"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "domain", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "dimensions", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "interpolation", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 178, columnNumberStart: 108, lineNumberEnd: 178, columnNumberEnd: 121, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 178, columnNumberStart: 100, lineNumberEnd: 178, columnNumberEnd: 121, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_addTLMConnection(mut crefA: ArcStr, mut crefB: ArcStr, mut delay: metamodelica::Real, mut alpha: metamodelica::Real, mut linearimpedance: metamodelica::Real, mut angularimpedance: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_addTLMConnection"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "crefA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "crefB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "delay", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "alpha", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "linearimpedance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "angularimpedance", subscripts: Nil } }, tail: Nil } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 189, columnNumberStart: 135, lineNumberEnd: 189, columnNumberEnd: 148, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 189, columnNumberStart: 127, lineNumberEnd: 189, columnNumberEnd: 148, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_compareSimulationResults(mut filenameA: ArcStr, mut filenameB: ArcStr, mut var: ArcStr, mut relTol: metamodelica::Real, mut absTol: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_compareSimulationResults"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filenameA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "filenameB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "var", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "relTol", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "absTol", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 199, columnNumberStart: 124, lineNumberEnd: 199, columnNumberEnd: 137, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 199, columnNumberStart: 116, lineNumberEnd: 199, columnNumberEnd: 137, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_copySystem(mut source: ArcStr, mut target: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_copySystem"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "source", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "target", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 206, columnNumberStart: 86, lineNumberEnd: 206, columnNumberEnd: 99, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 206, columnNumberStart: 78, lineNumberEnd: 206, columnNumberEnd: 99, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_delete(mut cref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_delete"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 212, columnNumberStart: 73, lineNumberEnd: 212, columnNumberEnd: 86, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 212, columnNumberStart: 65, lineNumberEnd: 212, columnNumberEnd: 86, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_deleteConnection(mut crefA: ArcStr, mut crefB: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_deleteConnection"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "crefA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "crefB", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 219, columnNumberStart: 90, lineNumberEnd: 219, columnNumberEnd: 103, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 219, columnNumberStart: 82, lineNumberEnd: 219, columnNumberEnd: 103, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_deleteConnectorFromBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_deleteConnectorFromBus"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "busCref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "connectorCref", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 226, columnNumberStart: 106, lineNumberEnd: 226, columnNumberEnd: 119, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 226, columnNumberStart: 98, lineNumberEnd: 226, columnNumberEnd: 119, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_deleteConnectorFromTLMBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_deleteConnectorFromTLMBus"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "busCref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "connectorCref", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 233, columnNumberStart: 109, lineNumberEnd: 233, columnNumberEnd: 122, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 233, columnNumberStart: 101, lineNumberEnd: 233, columnNumberEnd: 122, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_export(mut cref: ArcStr, mut filename: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_export"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 240, columnNumberStart: 82, lineNumberEnd: 240, columnNumberEnd: 95, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 240, columnNumberStart: 74, lineNumberEnd: 240, columnNumberEnd: 95, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_exportDependencyGraphs(mut cref: ArcStr, mut initialization: ArcStr, mut event: ArcStr, mut simulation: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_exportDependencyGraphs"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "initialization", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "event", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "simulation", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 249, columnNumberStart: 121, lineNumberEnd: 249, columnNumberEnd: 134, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 249, columnNumberStart: 113, lineNumberEnd: 249, columnNumberEnd: 134, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_exportSnapshot(mut cref: ArcStr) -> (ArcStr, i32) {
    let mut contents: ArcStr = arcstr::literal!("");
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_exportSnapshot"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "contents", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 256, columnNumberStart: 90, lineNumberEnd: 256, columnNumberEnd: 103, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 256, columnNumberStart: 82, lineNumberEnd: 256, columnNumberEnd: 103, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (contents, status)
}

pub fn oms_extractFMIKind(mut filename: ArcStr) -> (i32, i32) {
    let mut kind: i32 = 0;
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_extractFMIKind"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "kind", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 263, columnNumberStart: 90, lineNumberEnd: 263, columnNumberEnd: 103, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 263, columnNumberStart: 82, lineNumberEnd: 263, columnNumberEnd: 103, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (kind, status)
}

pub fn oms_getBoolean(mut cref: ArcStr) -> (bool, i32) {
    let mut value: bool = false;
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getBoolean"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "value", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 270, columnNumberStart: 83, lineNumberEnd: 270, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 270, columnNumberStart: 75, lineNumberEnd: 270, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (value, status)
}

pub fn oms_getFixedStepSize(mut cref: ArcStr) -> (metamodelica::Real, i32) {
    let mut stepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getFixedStepSize"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "stepSize", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 277, columnNumberStart: 92, lineNumberEnd: 277, columnNumberEnd: 105, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 277, columnNumberStart: 84, lineNumberEnd: 277, columnNumberEnd: 105, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (stepSize, status)
}

pub fn oms_getInteger(mut cref: ArcStr) -> (i32, i32) {
    let mut value: i32 = 0;
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getInteger"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "value", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 284, columnNumberStart: 83, lineNumberEnd: 284, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 284, columnNumberStart: 75, lineNumberEnd: 284, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (value, status)
}

pub fn oms_getModelState(mut cref: ArcStr) -> (i32, i32) {
    let mut modelState: i32 = 0;
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getModelState"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "modelState", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 291, columnNumberStart: 91, lineNumberEnd: 291, columnNumberEnd: 104, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 291, columnNumberStart: 83, lineNumberEnd: 291, columnNumberEnd: 104, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (modelState, status)
}

pub fn oms_getReal(mut cref: ArcStr) -> (metamodelica::Real, i32) {
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getReal"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "value", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 298, columnNumberStart: 80, lineNumberEnd: 298, columnNumberEnd: 93, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 298, columnNumberStart: 72, lineNumberEnd: 298, columnNumberEnd: 93, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (value, status)
}

pub fn oms_getSolver(mut cref: ArcStr) -> (i32, i32) {
    let mut solver: i32 = 0;
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getSolver"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "solver", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 305, columnNumberStart: 83, lineNumberEnd: 305, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 305, columnNumberStart: 75, lineNumberEnd: 305, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (solver, status)
}

pub fn oms_getStartTime(mut cref: ArcStr) -> (metamodelica::Real, i32) {
    let mut startTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getStartTime"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "startTime", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 312, columnNumberStart: 89, lineNumberEnd: 312, columnNumberEnd: 102, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 312, columnNumberStart: 81, lineNumberEnd: 312, columnNumberEnd: 102, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (startTime, status)
}

pub fn oms_getStopTime(mut cref: ArcStr) -> (metamodelica::Real, i32) {
    let mut stopTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getStopTime"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "stopTime", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 319, columnNumberStart: 87, lineNumberEnd: 319, columnNumberEnd: 100, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 319, columnNumberStart: 79, lineNumberEnd: 319, columnNumberEnd: 100, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (stopTime, status)
}

pub fn oms_getSubModelPath(mut cref: ArcStr) -> (ArcStr, i32) {
    let mut path: ArcStr = arcstr::literal!("");
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getSubModelPath"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "path", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 326, columnNumberStart: 87, lineNumberEnd: 326, columnNumberEnd: 100, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 326, columnNumberStart: 79, lineNumberEnd: 326, columnNumberEnd: 100, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (path, status)
}

pub fn oms_getSystemType(mut cref: ArcStr) -> (i32, i32) {
    let mut type_: i32 = 0;
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getSystemType"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "type_", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 333, columnNumberStart: 86, lineNumberEnd: 333, columnNumberEnd: 99, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 333, columnNumberStart: 78, lineNumberEnd: 333, columnNumberEnd: 99, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (type_, status)
}

pub fn oms_getTolerance(mut cref: ArcStr) -> (metamodelica::Real, metamodelica::Real, i32) {
    let mut absoluteTolerance: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut relativeTolerance: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getTolerance"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "absoluteTolerance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "relativeTolerance", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 341, columnNumberStart: 115, lineNumberEnd: 341, columnNumberEnd: 128, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 341, columnNumberStart: 107, lineNumberEnd: 341, columnNumberEnd: 128, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (absoluteTolerance, relativeTolerance, status)
}

pub fn oms_getVariableStepSize(mut cref: ArcStr) -> (metamodelica::Real, metamodelica::Real, metamodelica::Real, i32) {
    let mut initialStepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut minimumStepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut maximumStepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_getVariableStepSize"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "initialStepSize", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "minimumStepSize", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "maximumStepSize", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 350, columnNumberStart: 134, lineNumberEnd: 350, columnNumberEnd: 147, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 350, columnNumberStart: 126, lineNumberEnd: 350, columnNumberEnd: 147, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (initialStepSize, minimumStepSize, maximumStepSize, status)
}

pub fn oms_faultInjection(mut signal: ArcStr, mut faultType: i32, mut faultValue: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_faultInjection"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "signal", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "faultType", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "faultValue", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 358, columnNumberStart: 104, lineNumberEnd: 358, columnNumberEnd: 117, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 358, columnNumberStart: 96, lineNumberEnd: 358, columnNumberEnd: 117, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_importFile(mut filename: ArcStr) -> (ArcStr, i32) {
    let mut cref: ArcStr = arcstr::literal!("");
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_importFile"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 365, columnNumberStart: 86, lineNumberEnd: 365, columnNumberEnd: 99, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 365, columnNumberStart: 78, lineNumberEnd: 365, columnNumberEnd: 99, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (cref, status)
}

pub fn oms_importSnapshot(mut cref: ArcStr, mut snapshot: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_importSnapshot"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "snapshot", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 372, columnNumberStart: 90, lineNumberEnd: 372, columnNumberEnd: 103, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 372, columnNumberStart: 82, lineNumberEnd: 372, columnNumberEnd: 103, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_initialize(mut cref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_initialize"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 378, columnNumberStart: 77, lineNumberEnd: 378, columnNumberEnd: 90, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 378, columnNumberStart: 69, lineNumberEnd: 378, columnNumberEnd: 90, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_instantiate(mut cref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_instantiate"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 384, columnNumberStart: 78, lineNumberEnd: 384, columnNumberEnd: 91, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 384, columnNumberStart: 70, lineNumberEnd: 384, columnNumberEnd: 91, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_list(mut cref: ArcStr) -> (ArcStr, i32) {
    let mut contents: ArcStr = arcstr::literal!("");
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_list"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "contents", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 391, columnNumberStart: 80, lineNumberEnd: 391, columnNumberEnd: 93, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 391, columnNumberStart: 72, lineNumberEnd: 391, columnNumberEnd: 93, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (contents, status)
}

pub fn oms_listUnconnectedConnectors(mut cref: ArcStr) -> (ArcStr, i32) {
    let mut contents: ArcStr = arcstr::literal!("");
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_listUnconnectedConnectors"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "contents", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 398, columnNumberStart: 101, lineNumberEnd: 398, columnNumberEnd: 114, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 398, columnNumberStart: 93, lineNumberEnd: 398, columnNumberEnd: 114, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (contents, status)
}

pub fn oms_loadSnapshot(mut cref: ArcStr, mut snapshot: ArcStr) -> (ArcStr, i32) {
    let mut newCref: ArcStr = arcstr::literal!("");
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_loadSnapshot"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "snapshot", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "newCref", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 406, columnNumberStart: 96, lineNumberEnd: 406, columnNumberEnd: 109, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 406, columnNumberStart: 88, lineNumberEnd: 406, columnNumberEnd: 109, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (newCref, status)
}

pub fn oms_newModel(mut cref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_newModel"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 412, columnNumberStart: 75, lineNumberEnd: 412, columnNumberEnd: 88, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 412, columnNumberStart: 67, lineNumberEnd: 412, columnNumberEnd: 88, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_removeSignalsFromResults(mut cref: ArcStr, mut regex: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_removeSignalsFromResults"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "regex", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 419, columnNumberStart: 97, lineNumberEnd: 419, columnNumberEnd: 110, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 419, columnNumberStart: 89, lineNumberEnd: 419, columnNumberEnd: 110, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_rename(mut cref: ArcStr, mut newCref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_rename"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "newCref", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 426, columnNumberStart: 81, lineNumberEnd: 426, columnNumberEnd: 94, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 426, columnNumberStart: 73, lineNumberEnd: 426, columnNumberEnd: 94, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_reset(mut cref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_reset"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 432, columnNumberStart: 72, lineNumberEnd: 432, columnNumberEnd: 85, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 432, columnNumberStart: 64, lineNumberEnd: 432, columnNumberEnd: 85, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_RunFile(mut filename: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_RunFile"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 438, columnNumberStart: 78, lineNumberEnd: 438, columnNumberEnd: 91, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 438, columnNumberStart: 70, lineNumberEnd: 438, columnNumberEnd: 91, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setBoolean(mut cref: ArcStr, mut value: bool) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setBoolean"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "value", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 445, columnNumberStart: 83, lineNumberEnd: 445, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 445, columnNumberStart: 75, lineNumberEnd: 445, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setCommandLineOption(mut cmd: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setCommandLineOption"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cmd", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 451, columnNumberStart: 86, lineNumberEnd: 451, columnNumberEnd: 99, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 451, columnNumberStart: 78, lineNumberEnd: 451, columnNumberEnd: 99, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setFixedStepSize(mut cref: ArcStr, mut stepSize: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setFixedStepSize"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "stepSize", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 458, columnNumberStart: 92, lineNumberEnd: 458, columnNumberEnd: 105, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 458, columnNumberStart: 84, lineNumberEnd: 458, columnNumberEnd: 105, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setInteger(mut cref: ArcStr, mut value: i32) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setInteger"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "value", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 465, columnNumberStart: 83, lineNumberEnd: 465, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 465, columnNumberStart: 75, lineNumberEnd: 465, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setLogFile(mut filename: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setLogFile"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 471, columnNumberStart: 81, lineNumberEnd: 471, columnNumberEnd: 94, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 471, columnNumberStart: 73, lineNumberEnd: 471, columnNumberEnd: 94, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setLoggingInterval(mut cref: ArcStr, mut loggingInterval: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setLoggingInterval"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "loggingInterval", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 478, columnNumberStart: 101, lineNumberEnd: 478, columnNumberEnd: 114, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 478, columnNumberStart: 93, lineNumberEnd: 478, columnNumberEnd: 114, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setLoggingLevel(mut logLevel: i32) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setLoggingLevel"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "logLevel", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 484, columnNumberStart: 86, lineNumberEnd: 484, columnNumberEnd: 99, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 484, columnNumberStart: 78, lineNumberEnd: 484, columnNumberEnd: 99, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setReal(mut cref: ArcStr, mut value: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setReal"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "value", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 491, columnNumberStart: 80, lineNumberEnd: 491, columnNumberEnd: 93, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 491, columnNumberStart: 72, lineNumberEnd: 491, columnNumberEnd: 93, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setRealInputDerivative(mut cref: ArcStr, mut value: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setRealInputDerivative"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "value", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 498, columnNumberStart: 95, lineNumberEnd: 498, columnNumberEnd: 108, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 498, columnNumberStart: 87, lineNumberEnd: 498, columnNumberEnd: 108, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setResultFile(mut cref: ArcStr, mut filename: ArcStr, mut bufferSize: i32) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setResultFile"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "bufferSize", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 506, columnNumberStart: 100, lineNumberEnd: 506, columnNumberEnd: 113, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 506, columnNumberStart: 92, lineNumberEnd: 506, columnNumberEnd: 113, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setSignalFilter(mut cref: ArcStr, mut regex: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setSignalFilter"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "regex", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 513, columnNumberStart: 88, lineNumberEnd: 513, columnNumberEnd: 101, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 513, columnNumberStart: 80, lineNumberEnd: 513, columnNumberEnd: 101, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setSolver(mut cref: ArcStr, mut solver: i32) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setSolver"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "solver", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 520, columnNumberStart: 83, lineNumberEnd: 520, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 520, columnNumberStart: 75, lineNumberEnd: 520, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setStartTime(mut cref: ArcStr, mut startTime: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setStartTime"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "startTime", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 527, columnNumberStart: 89, lineNumberEnd: 527, columnNumberEnd: 102, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 527, columnNumberStart: 81, lineNumberEnd: 527, columnNumberEnd: 102, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setStopTime(mut cref: ArcStr, mut stopTime: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setStopTime"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "stopTime", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 534, columnNumberStart: 87, lineNumberEnd: 534, columnNumberEnd: 100, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 534, columnNumberStart: 79, lineNumberEnd: 534, columnNumberEnd: 100, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setTempDirectory(mut newTempDir: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setTempDirectory"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "newTempDir", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 540, columnNumberStart: 89, lineNumberEnd: 540, columnNumberEnd: 102, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 540, columnNumberStart: 81, lineNumberEnd: 540, columnNumberEnd: 102, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setTLMPositionAndOrientation(mut cref: ArcStr, mut x1: metamodelica::Real, mut x2: metamodelica::Real, mut x3: metamodelica::Real, mut A11: metamodelica::Real, mut A12: metamodelica::Real, mut A13: metamodelica::Real, mut A21: metamodelica::Real, mut A22: metamodelica::Real, mut A23: metamodelica::Real, mut A31: metamodelica::Real, mut A32: metamodelica::Real, mut A33: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setTLMPositionAndOrientation"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "x1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "x2", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "x3", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A11", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A12", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A13", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A21", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A22", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A23", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A31", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A32", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "A33", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 558, columnNumberStart: 140, lineNumberEnd: 558, columnNumberEnd: 153, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 558, columnNumberStart: 132, lineNumberEnd: 558, columnNumberEnd: 153, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setTLMSocketData(mut cref: ArcStr, mut address: ArcStr, mut managerPort: i32, mut monitorPort: i32) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setTLMSocketData"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "address", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "managerPort", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "monitorPort", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 567, columnNumberStart: 115, lineNumberEnd: 567, columnNumberEnd: 128, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 567, columnNumberStart: 107, lineNumberEnd: 567, columnNumberEnd: 128, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setTolerance(mut cref: ArcStr, mut absoluteTolerance: metamodelica::Real, mut relativeTolerance: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setTolerance"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "absoluteTolerance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "relativeTolerance", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 575, columnNumberStart: 115, lineNumberEnd: 575, columnNumberEnd: 128, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 575, columnNumberStart: 107, lineNumberEnd: 575, columnNumberEnd: 128, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setVariableStepSize(mut cref: ArcStr, mut initialStepSize: metamodelica::Real, mut minimumStepSize: metamodelica::Real, mut maximumStepSize: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setVariableStepSize"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "initialStepSize", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "minimumStepSize", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "maximumStepSize", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 584, columnNumberStart: 134, lineNumberEnd: 584, columnNumberEnd: 147, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 584, columnNumberStart: 126, lineNumberEnd: 584, columnNumberEnd: 147, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_setWorkingDirectory(mut newWorkingDir: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_setWorkingDirectory"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "newWorkingDir", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 590, columnNumberStart: 95, lineNumberEnd: 590, columnNumberEnd: 108, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 590, columnNumberStart: 87, lineNumberEnd: 590, columnNumberEnd: 108, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_simulate(mut cref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_simulate"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 596, columnNumberStart: 75, lineNumberEnd: 596, columnNumberEnd: 88, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 596, columnNumberStart: 67, lineNumberEnd: 596, columnNumberEnd: 88, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_stepUntil(mut cref: ArcStr, mut stopTime: metamodelica::Real) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_stepUntil"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "stopTime", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 603, columnNumberStart: 85, lineNumberEnd: 603, columnNumberEnd: 98, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 603, columnNumberStart: 77, lineNumberEnd: 603, columnNumberEnd: 98, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

pub fn oms_terminate(mut cref: ArcStr) -> i32 {
    let mut status: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("OMSimulator_oms_terminate"), lang: Some("C"), output_: Some(CREF_IDENT { name: "status", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "cref", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 609, columnNumberStart: 76, lineNumberEnd: 609, columnNumberEnd: 89, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/OMSimulatorExt.mo", isReadOnly: false, lineNumberStart: 609, columnNumberStart: 68, lineNumberEnd: 609, columnNumberEnd: 89, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    status
}

