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

pub fn checkTaskGraph(mut filename: ArcStr, mut reffilename: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("TaskGraphResults_checkTaskGraph"), lang: Some("C"), output_: Some(CREF_IDENT { name: "res", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "filename", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "reffilename", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/TaskGraphResults.mo", isReadOnly: false, lineNumberStart: 48, columnNumberStart: 93, lineNumberEnd: 48, columnNumberEnd: 106, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/TaskGraphResults.mo", isReadOnly: false, lineNumberStart: 48, columnNumberStart: 85, lineNumberEnd: 48, columnNumberEnd: 106, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(res)
}

pub fn checkCodeGraph(mut graphfile: ArcStr, mut codefile: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("TaskGraphResults_checkCodeGraph"), lang: Some("C"), output_: Some(CREF_IDENT { name: "res", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "graphfile", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "codefile", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/TaskGraphResults.mo", isReadOnly: false, lineNumberStart: 55, columnNumberStart: 91, lineNumberEnd: 55, columnNumberEnd: 104, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/TaskGraphResults.mo", isReadOnly: false, lineNumberStart: 55, columnNumberStart: 83, lineNumberEnd: 55, columnNumberEnd: 104, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    Ok(res)
}

