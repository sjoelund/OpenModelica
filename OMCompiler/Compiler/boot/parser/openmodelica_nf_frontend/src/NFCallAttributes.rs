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

use crate::NFType;
use openmodelica_frontend_types::DAE;

#[derive(Clone, Debug, PartialEq)]
pub struct NFCallAttributes {
    /// tuple
    pub tuple_: bool,
    /// builtin Function call
    pub builtin: bool,
    /// if the function has prefix *impure* is true, else false
    pub isImpure: bool,
    pub isFunctionPointerCall: bool,
    pub inlineType: DAE::InlineType,
    /// Input variables of the function if the call is tail-recursive
    pub tailCall: DAE::TailCall,
}

pub type CALL_ATTR = NFCallAttributes;

pub fn toDAE(mut attr: Arc<NFCallAttributes>, mut returnType: Arc<NFType::NFType>) -> Result<Arc<DAE::CallAttributes>> {
    let mut fattr: Arc<DAE::CallAttributes>;
    fattr = Arc::new(DAE::CallAttributes { ty: NFType::toDAE(returnType.clone(), true)?, tuple_: attr.tuple_.clone(), builtin: attr.builtin.clone(), isImpure: attr.isImpure.clone(), isFunctionPointerCall: attr.isFunctionPointerCall.clone(), inlineType: attr.inlineType.clone(), tailCall: attr.tailCall.clone() });
    Ok(fattr)
}


