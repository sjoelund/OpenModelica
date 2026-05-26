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

pub fn initSIUnits() -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__initSIUnits"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 39, columnNumberStart: 68, lineNumberEnd: 39, columnNumberEnd: 81, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 39, columnNumberStart: 60, lineNumberEnd: 39, columnNumberEnd: 81, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn unit2str(mut noms: Arc<metamodelica::List<i32>>, mut denoms: Arc<metamodelica::List<i32>>, mut tpnoms: Arc<metamodelica::List<i32>>, mut tpdenoms: Arc<metamodelica::List<i32>>, mut tpstrs: Arc<metamodelica::List<ArcStr>>, mut scaleFactor: metamodelica::Real, mut offset: metamodelica::Real) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExt_unit2str"), lang: Some("C"), output_: Some(CREF_IDENT { name: "res", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "noms", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "denoms", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "tpnoms", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "tpdenoms", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "tpstrs", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "scaleFactor", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "offset", subscripts: Nil } }, tail: Nil } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 51, columnNumberStart: 117, lineNumberEnd: 51, columnNumberEnd: 130, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 51, columnNumberStart: 109, lineNumberEnd: 51, columnNumberEnd: 130, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    res
}

pub fn str2unit(mut res: ArcStr) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>, metamodelica::Real, metamodelica::Real)> {
    let mut noms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut denoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpnoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpdenoms: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tpstrs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut scaleFactor: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut offset: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExt_str2unit"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "res", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "noms", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "denoms", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "tpnoms", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "tpdenoms", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "tpstrs", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "scaleFactor", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "offset", subscripts: Nil } }, tail: Nil } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 63, columnNumberStart: 117, lineNumberEnd: 63, columnNumberEnd: 130, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 63, columnNumberStart: 109, lineNumberEnd: 63, columnNumberEnd: 130, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok((noms, denoms, tpnoms, tpdenoms, tpstrs, scaleFactor, offset))
}

pub fn allUnitSymbols() -> Arc<metamodelica::List<ArcStr>> {
    let mut unitSymbols: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__allUnitSymbols"), lang: Some("C"), output_: Some(CREF_IDENT { name: "unitSymbols", subscripts: Nil }), args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 68, columnNumberStart: 85, lineNumberEnd: 68, columnNumberEnd: 98, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 68, columnNumberStart: 77, lineNumberEnd: 68, columnNumberEnd: 98, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    unitSymbols
}

pub fn addBase(mut name: ArcStr) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__addBase"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "name", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 73, columnNumberStart: 68, lineNumberEnd: 73, columnNumberEnd: 81, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 73, columnNumberStart: 60, lineNumberEnd: 73, columnNumberEnd: 81, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn registerWeight(mut name: ArcStr, mut weight: metamodelica::Real) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__registerWeight"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "name", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "weight", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 79, columnNumberStart: 82, lineNumberEnd: 79, columnNumberEnd: 95, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 79, columnNumberStart: 74, lineNumberEnd: 79, columnNumberEnd: 95, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn addDerived(mut name: ArcStr, mut exp: ArcStr) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__addDerived"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "name", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "exp", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 86, columnNumberStart: 75, lineNumberEnd: 86, columnNumberEnd: 88, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 86, columnNumberStart: 67, lineNumberEnd: 86, columnNumberEnd: 88, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn addDerivedWeight(mut name: ArcStr, mut exp: ArcStr, mut weight: metamodelica::Real) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__addDerivedWeight"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "name", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "exp", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "weight", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 93, columnNumberStart: 88, lineNumberEnd: 93, columnNumberEnd: 101, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 93, columnNumberStart: 80, lineNumberEnd: 93, columnNumberEnd: 101, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn checkpoint() -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__checkpoint"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 97, columnNumberStart: 68, lineNumberEnd: 97, columnNumberEnd: 81, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 97, columnNumberStart: 60, lineNumberEnd: 97, columnNumberEnd: 81, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn rollback() -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__rollback"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 101, columnNumberStart: 65, lineNumberEnd: 101, columnNumberEnd: 78, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 101, columnNumberStart: 57, lineNumberEnd: 101, columnNumberEnd: 78, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn clear() -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__clear"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 105, columnNumberStart: 62, lineNumberEnd: 105, columnNumberEnd: 75, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 105, columnNumberStart: 54, lineNumberEnd: 105, columnNumberEnd: 75, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn commit() -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("UnitParserExtImpl__commit"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 110, columnNumberStart: 63, lineNumberEnd: 110, columnNumberEnd: 76, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/FrontEnd/UnitParserExt.mo", isReadOnly: false, lineNumberStart: 110, columnNumberStart: 55, lineNumberEnd: 110, columnNumberEnd: 76, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

