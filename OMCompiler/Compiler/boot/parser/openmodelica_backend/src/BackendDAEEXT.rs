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

pub fn initMarks(mut inInteger1: i32, mut inInteger2: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_initMarks"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger2", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 52, columnNumberStart: 82, lineNumberEnd: 52, columnNumberEnd: 95, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 52, columnNumberStart: 74, lineNumberEnd: 52, columnNumberEnd: 95, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn eMark(mut inInteger: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_eMark"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 58, columnNumberStart: 66, lineNumberEnd: 58, columnNumberEnd: 79, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 58, columnNumberStart: 58, lineNumberEnd: 58, columnNumberEnd: 79, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

/* TODO: Implement an external C function for bootstrapped omc or remove me. DO NOT SIMPLY REMOVE THIS COMMENT
public function getEMark
  input Integer inInteger;
  output Boolean outBoolean;

  external "C" outBoolean=BackendDAEEXT_getEMark(inInteger) annotation(Library = "omcruntime");
end getEMark;*/
pub fn vMark(mut inInteger: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_vMark"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 72, columnNumberStart: 66, lineNumberEnd: 72, columnNumberEnd: 79, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 72, columnNumberStart: 58, lineNumberEnd: 72, columnNumberEnd: 79, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn getVMark(mut inInteger: i32) -> bool {
    let mut outBoolean: bool = false;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_getVMark"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outBoolean", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 79, columnNumberStart: 80, lineNumberEnd: 79, columnNumberEnd: 93, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 79, columnNumberStart: 72, lineNumberEnd: 79, columnNumberEnd: 93, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outBoolean
}

pub fn getMarkedEqns() -> Arc<metamodelica::List<i32>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_getMarkedEqns"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outIntegerLst", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 85, columnNumberStart: 79, lineNumberEnd: 85, columnNumberEnd: 92, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 85, columnNumberStart: 71, lineNumberEnd: 85, columnNumberEnd: 92, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outIntegerLst
}

pub fn getDifferentiatedEqns() -> Arc<metamodelica::List<i32>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_getDifferentiatedEqns"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outIntegerLst", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 91, columnNumberStart: 87, lineNumberEnd: 91, columnNumberEnd: 100, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 91, columnNumberStart: 79, lineNumberEnd: 91, columnNumberEnd: 100, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outIntegerLst
}

pub fn clearDifferentiated() -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_clearDifferentiated"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 96, columnNumberStart: 71, lineNumberEnd: 96, columnNumberEnd: 84, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 96, columnNumberStart: 63, lineNumberEnd: 96, columnNumberEnd: 84, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn markDifferentiated(mut inInteger: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_markDifferentiated"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 102, columnNumberStart: 79, lineNumberEnd: 102, columnNumberEnd: 92, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 102, columnNumberStart: 71, lineNumberEnd: 102, columnNumberEnd: 92, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn getMarkedVariables() -> Arc<metamodelica::List<i32>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_getMarkedVariables"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outIntegerLst", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 108, columnNumberStart: 84, lineNumberEnd: 108, columnNumberEnd: 97, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 108, columnNumberStart: 76, lineNumberEnd: 108, columnNumberEnd: 97, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outIntegerLst
}

pub fn initLowLink(mut inInteger: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_initLowLink"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 114, columnNumberStart: 72, lineNumberEnd: 114, columnNumberEnd: 85, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 114, columnNumberStart: 64, lineNumberEnd: 114, columnNumberEnd: 85, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn initNumber(mut inInteger: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_initNumber"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 120, columnNumberStart: 71, lineNumberEnd: 120, columnNumberEnd: 84, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 120, columnNumberStart: 63, lineNumberEnd: 120, columnNumberEnd: 84, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn setLowLink(mut inInteger1: i32, mut inInteger2: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_setLowLink"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger2", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 127, columnNumberStart: 83, lineNumberEnd: 127, columnNumberEnd: 96, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 127, columnNumberStart: 75, lineNumberEnd: 127, columnNumberEnd: 96, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn getLowLink(mut inInteger: i32) -> i32 {
    let mut outInteger: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_getLowLink"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outInteger", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 134, columnNumberStart: 82, lineNumberEnd: 134, columnNumberEnd: 95, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 134, columnNumberStart: 74, lineNumberEnd: 134, columnNumberEnd: 95, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outInteger
}

pub fn setNumber(mut inInteger1: i32, mut inInteger2: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_setNumber"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger2", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 141, columnNumberStart: 82, lineNumberEnd: 141, columnNumberEnd: 95, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 141, columnNumberStart: 74, lineNumberEnd: 141, columnNumberEnd: 95, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn getNumber(mut inInteger: i32) -> i32 {
    let mut outInteger: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_getNumber"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outInteger", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInteger", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 148, columnNumberStart: 81, lineNumberEnd: 148, columnNumberEnd: 94, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 148, columnNumberStart: 73, lineNumberEnd: 148, columnNumberEnd: 94, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outInteger
}

/* TODO: Implement an external C function for bootstrapped omc or remove me. DO NOT SIMPLY REMOVE THIS COMMENT
public function dumpMarkedEquations
  input Integer inInteger;

  external "C" BackendDAEEXT_dumpMarkedEquations(inInteger) annotation(Library = "omcruntime");
end dumpMarkedEquations;

public function dumpMarkedVariables
  input Integer inInteger;

  external "C" BackendDAEEXT_dumpMarkedVariables(inInteger) annotation(Library = "omcruntime");
end dumpMarkedVariables;

public function initV
  input Integer inInteger;

  external "C" BackendDAEEXT_initV(inInteger) annotation(Library = "omcruntime");
end initV;

public function initF
  input Integer inInteger;

  external "C" BackendDAEEXT_initF(inInteger) annotation(Library = "omcruntime");
end initF;

public function setV
  input Integer inInteger1;
  input Integer inInteger2;

  external "C" BackendDAEEXT_setV(inInteger1,inInteger2) annotation(Library = "omcruntime");
end setV;

public function getV
  input Integer inInteger;
  output Integer outInteger;

  external "C" outInteger=BackendDAEEXT_getV(inInteger) annotation(Library = "omcruntime");
end getV;

public function setF
  input Integer inInteger1;
  input Integer inInteger2;

  external "C" BackendDAEEXT_setF(inInteger1,inInteger2) annotation(Library = "omcruntime");
end setF;

public function getF
  input Integer inInteger;
  output Integer outInteger;

  external "C" outInteger=BackendDAEEXT_getF(inInteger) annotation(Library = "omcruntime");
end getF;
*/
/* *****************************************
 C-Implementation Stuff from
 Kamer Kaya, Johannes Langguth and Bora Ucar
 see: http://bmi.osu.edu/~kamer/index.html
 *****************************************/
pub fn setAdjacencyMatrix(mut nv: i32, mut ne: i32, mut nz: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_setAdjacencyMatrix"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "nv", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "ne", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "nz", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "m", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 217, columnNumberStart: 80, lineNumberEnd: 217, columnNumberEnd: 93, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 217, columnNumberStart: 72, lineNumberEnd: 217, columnNumberEnd: 93, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

/* TODO: Implement an external C function for bootstrapped omc or remove me. DO NOT SIMPLY REMOVE THIS COMMENT
public function cheapmatching
"author: Frenkel TUD 2012-04
  calls cheapmatching algorithms
  cheapID: id of cheap algo (1-4)
      1: Simple Greedy
      2: Karp-Sipser
      3: Random Karp-Sipser (DEFAULT)
      4: Minimum Degree (two-sided)

     Other than these two, non-positive values are not allowed.
  "
  input Integer nv;
  input Integer ne;
  input Integer cheapID;
  input Integer clear_match;
  external "C" BackendDAEEXT_cheapmatching(nv,ne,cheapID,clear_match) annotation(Library = "omcruntime");
end cheapmatching;*/
pub fn matching(mut nv: i32, mut ne: i32, mut matchingID: i32, mut cheapID: i32, mut relabel_period: metamodelica::Real, mut clear_match: i32) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_matching"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "nv", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "ne", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "matchingID", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "cheapID", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "relabel_period", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "clear_match", subscripts: Nil } }, tail: Nil } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 275, columnNumberStart: 111, lineNumberEnd: 275, columnNumberEnd: 124, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 275, columnNumberStart: 103, lineNumberEnd: 275, columnNumberEnd: 124, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn getAssignment(mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_getAssignment"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "ass1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "ass2", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 281, columnNumberStart: 75, lineNumberEnd: 281, columnNumberEnd: 88, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 281, columnNumberStart: 67, lineNumberEnd: 281, columnNumberEnd: 88, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

pub fn setAssignment(mut lenass1: i32, mut lenass2: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> bool {
    let mut outBoolean: bool = false;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("BackendDAEEXT_setAssignment"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outBoolean", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "lenass1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "lenass2", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "ass1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "ass2", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 292, columnNumberStart: 101, lineNumberEnd: 292, columnNumberEnd: 114, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/BackEnd/BackendDAEEXT.mo", isReadOnly: false, lineNumberStart: 292, columnNumberStart: 93, lineNumberEnd: 292, columnNumberEnd: 114, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    outBoolean
}

