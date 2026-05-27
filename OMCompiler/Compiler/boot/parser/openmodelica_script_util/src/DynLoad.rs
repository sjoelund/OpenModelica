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

use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::StackOverflow;

pub fn executeFunction(mut handle: i32, mut values: Arc<metamodelica::List<Arc<Values::Value>>>, mut debug: bool) -> Result<Arc<Values::Value>> {
    fn executeFunction_internal(mut handle: i32, mut values: Arc<metamodelica::List<Arc<Values::Value>>>, mut debug: bool) -> Result<Arc<Values::Value>> {
        let mut outVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("DynLoad_executeFunction"), lang: Some("C"), output_: Some(CREF_IDENT { name: "outVal", subscripts: Nil }), args: Cons { head: CALL { function_: CREF_QUAL { name: "OpenModelica", subscripts: Nil, componentRef: CREF_IDENT { name: "threadData", subscripts: Nil } }, functionArgs: FUNCTIONARGS { args: Nil, argNames: Nil }, typeVars: Nil }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "handle", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "values", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "debug", subscripts: Nil } }, tail: Nil } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "omcruntime" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/DynLoad.mo", isReadOnly: false, lineNumberStart: 66, columnNumberStart: 113, lineNumberEnd: 66, columnNumberEnd: 126, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/DynLoad.mo", isReadOnly: false, lineNumberStart: 66, columnNumberStart: 105, lineNumberEnd: 66, columnNumberEnd: 126, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
        Ok(outVal)
    }

    let mut outVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    StackOverflow::clearStacktraceMessages();
    outVal = executeFunction_internal(handle.clone(), values.clone(), debug.clone())?;
    if StackOverflow::hasStacktraceMessages() {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow when evaluating function:\n")); __mm_s.push_str(&*stringDelimitList(StackOverflow::readableStacktraceMessages()?, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    Ok(outVal)
}

