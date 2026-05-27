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

use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::Values;
use openmodelica_util_datatypes_basic::List;

pub fn val(mut filename: ArcStr, mut varname: ArcStr, mut timeStamp: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_val"), lang: Some("C"), output_: Some(CREF_IDENT { name: "val", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "varname", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "timeStamp", subscripts: Nil } }, tail: Nil } } }, annotation_: None }, annotation: None }
    Ok(val)
}

pub fn readVariables(mut filename: ArcStr, mut readParameters: bool, mut openmodelicaStyle: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut vars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_readVariables"), lang: Some("C"), output_: Some(CREF_IDENT { name: "vars", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "readParameters", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "openmodelicaStyle", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 65, columnNumberStart: 117, lineNumberEnd: 65, columnNumberEnd: 130, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 65, columnNumberStart: 109, lineNumberEnd: 65, columnNumberEnd: 130, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(vars)
}

pub fn readDataset(mut filename: ArcStr, mut vars: Arc<metamodelica::List<ArcStr>>, mut dimsize: i32) -> Result<Arc<Values::Value>> {
    fn readDataset_work(mut filename: ArcStr, mut vars: Arc<metamodelica::List<ArcStr>>, mut dimsize: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>> {
        let mut outMatrix: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
        todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_readDataset"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outMatrix", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "vars", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "dimsize", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 83, columnNumberStart: 100, lineNumberEnd: 83, columnNumberEnd: 113, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 83, columnNumberStart: 92, lineNumberEnd: 83, columnNumberEnd: 113, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
        Ok(outMatrix)
    }

    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut rvals: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut vals: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
    let mut rows: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    rvals = readDataset_work((filename.clone()).clone(), vars.clone(), dimsize.clone())?;
    vals = List::mapListReverse(rvals.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeReal, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<Arc<Values::Value>> + 'static>));
    rows = List::mapReverse(vals.clone(), (std::sync::Arc::new(ValuesMake::makeArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> + 'static>));
    val = ValuesMake::makeArray(rows.clone())?;
    Ok(val)
}

pub fn readSimulationResultSize(mut filename: ArcStr) -> Result<i32> {
    let mut size: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_readSimulationResultSize"), lang: Some("C"), output_: Some(CREF_IDENT { name: "size", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Nil }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 96, columnNumberStart: 93, lineNumberEnd: 96, columnNumberEnd: 106, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 96, columnNumberStart: 85, lineNumberEnd: 96, columnNumberEnd: 106, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(size)
}

pub fn close() -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_close"), lang: Some("C"), output_: None, args: Nil, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 100, columnNumberStart: 61, lineNumberEnd: 100, columnNumberEnd: 74, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 100, columnNumberStart: 53, lineNumberEnd: 100, columnNumberEnd: 74, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

pub fn cmpSimulationResults(mut runningTestsuite: bool, mut filename: ArcStr, mut reffilename: ArcStr, mut logfilename: ArcStr, mut refTol: metamodelica::Real, mut absTol: metamodelica::Real, mut vars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_cmpSimulationResults"), lang: Some("C"), output_: Some(CREF_IDENT { name: "res", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "reffilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "logfilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "refTol", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "absTol", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "vars", subscripts: Nil } }, tail: Nil } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 112, columnNumberStart: 148, lineNumberEnd: 112, columnNumberEnd: 161, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 112, columnNumberStart: 140, lineNumberEnd: 112, columnNumberEnd: 161, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(res)
}

pub fn deltaSimulationResults(mut filename: ArcStr, mut reffilename: ArcStr, mut method: ArcStr, mut vars: Arc<metamodelica::List<ArcStr>>) -> Result<metamodelica::Real> {
    let mut res: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_deltaSimulationResults"), lang: Some("C"), output_: Some(CREF_IDENT { name: "res", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "reffilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "method", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "vars", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 121, columnNumberStart: 114, lineNumberEnd: 121, columnNumberEnd: 127, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 121, columnNumberStart: 106, lineNumberEnd: 121, columnNumberEnd: 127, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(res)
}

pub fn diffSimulationResults(mut runningTestsuite: bool, mut filename: ArcStr, mut reffilename: ArcStr, mut prefix: ArcStr, mut refTol: metamodelica::Real, mut relTolDiffMaxMin: metamodelica::Real, mut rangeDelta: metamodelica::Real, mut vars: Arc<metamodelica::List<ArcStr>>, mut keepEqualResults: bool) -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> {
    let mut success: bool = false;
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_diffSimulationResults"), lang: Some("C"), output_: Some(CREF_IDENT { name: "res", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "reffilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "prefix", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "refTol", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "relTolDiffMaxMin", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "rangeDelta", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "vars", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "keepEqualResults", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "success", subscripts: Nil } }, tail: Nil } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 136, columnNumberStart: 190, lineNumberEnd: 136, columnNumberEnd: 203, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 136, columnNumberStart: 182, lineNumberEnd: 136, columnNumberEnd: 203, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok((success, res))
}

pub fn diffSimulationResultsHtml(mut runningTestsuite: bool, mut filename: ArcStr, mut reffilename: ArcStr, mut refTol: metamodelica::Real, mut relTolDiffMaxMin: metamodelica::Real, mut rangeDelta: metamodelica::Real, mut var: ArcStr) -> Result<ArcStr> {
    let mut html: ArcStr = arcstr::literal!("");
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_diffSimulationResultsHtml"), lang: Some("C"), output_: Some(CREF_IDENT { name: "html", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "runningTestsuite", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "var", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "reffilename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "refTol", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "relTolDiffMaxMin", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "rangeDelta", subscripts: Nil } }, tail: Nil } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 148, columnNumberStart: 162, lineNumberEnd: 148, columnNumberEnd: 175, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 148, columnNumberStart: 154, lineNumberEnd: 148, columnNumberEnd: 175, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(html)
}

pub fn filterSimulationResults(mut inFile: ArcStr, mut outFile: ArcStr, mut vars: Arc<metamodelica::List<ArcStr>>, mut numberOfIntervals: i32, mut removeDescription: bool, mut hintReadAllVars: bool) -> Result<bool> {
    let mut result: bool = false;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("SimulationResults_filterSimulationResults"), lang: Some("C"), output_: Some(CREF_IDENT { name: "result", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inFile", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outFile", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "vars", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "numberOfIntervals", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "removeDescription", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "hintReadAllVars", subscripts: Nil } }, tail: Nil } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 159, columnNumberStart: 157, lineNumberEnd: 159, columnNumberEnd: 170, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/SimulationResults.mo", isReadOnly: false, lineNumberStart: 159, columnNumberStart: 149, lineNumberEnd: 159, columnNumberEnd: 170, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(result)
}

