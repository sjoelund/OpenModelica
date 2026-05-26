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

use crate::FMI;

pub fn initializeFMIImport(mut inFileName: ArcStr, mut inWorkingDirectory: ArcStr, mut inFMILogLevel: i32, mut inInputConnectors: bool, mut inOutputConnectors: bool, mut inIsModelDescriptionImport: bool) -> Result<(bool, Option<i32>, Option<i32>, FMI::Info, Arc<metamodelica::List<FMI::TypeDefinitions>>, FMI::ExperimentAnnotation, Option<i32>, Arc<metamodelica::List<FMI::ModelVariables>>)> {
    let mut result: bool = false;
    let mut outFMIContext: Option<i32> = None;
    let mut outFMIInstance: Option<i32> = None;
    let mut outFMIInfo: FMI::Info;
    let mut outTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>> = metamodelica::nil();
    let mut outExperimentAnnotation: FMI::ExperimentAnnotation;
    let mut outModelVariablesInstance: Option<i32> = None;
    let mut outModelVariablesList: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("FMIImpl__initializeFMIImport"), lang: Some("C"), output_: Some(CREF_IDENT { name: "result", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inFileName", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWorkingDirectory", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inFMILogLevel", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inInputConnectors", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inOutputConnectors", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inIsModelDescriptionImport", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outFMIContext", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outFMIInstance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outFMIInfo", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outTypeDefinitionsList", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outExperimentAnnotation", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outModelVariablesInstance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outModelVariablesList", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcbackendruntime" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "fmilib" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/FMIExt.mo", isReadOnly: false, lineNumberStart: 60, columnNumberStart: 24, lineNumberEnd: 60, columnNumberEnd: 68, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/FMIExt.mo", isReadOnly: false, lineNumberStart: 60, columnNumberStart: 16, lineNumberEnd: 60, columnNumberEnd: 68, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok((result, outFMIContext, outFMIInstance, outFMIInfo, outTypeDefinitionsList, outExperimentAnnotation, outModelVariablesInstance, outModelVariablesList))
}

pub fn releaseFMIImport(mut inFMIModelVariablesInstance: Option<i32>, mut inFMIInstance: Option<i32>, mut inFMIContext: Option<i32>, mut inFMIVersion: ArcStr) -> Result<()> {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("FMIImpl__releaseFMIImport"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inFMIModelVariablesInstance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inFMIInstance", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inFMIContext", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inFMIVersion", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcbackendruntime" }, tail: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "fmilib" }, tail: Nil } } } }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/FMIExt.mo", isReadOnly: false, lineNumberStart: 68, columnNumberStart: 133, lineNumberEnd: 68, columnNumberEnd: 177, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/FMIExt.mo", isReadOnly: false, lineNumberStart: 68, columnNumberStart: 125, lineNumberEnd: 68, columnNumberEnd: 177, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(())
}

