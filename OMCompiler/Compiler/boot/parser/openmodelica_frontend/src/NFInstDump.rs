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

use crate::NFInstDumpTpl;
use crate::NFInstPrefix;
use crate::NFInstTypes;
use openmodelica_susan::Tpl;
use openmodelica_util_datatypes_basic::Array;

//public import NFConnect2;
pub fn modelStr(mut inName: ArcStr, mut inClass: Arc<NFInstTypes::Class>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString2(Arc::new(NFInstDumpTpl::dumpModel), (inName.clone()).clone(), inClass.clone())?).clone();
    Ok(outString)
}

pub fn elementStr(mut inElement: Arc<NFInstTypes::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(NFInstDumpTpl::dumpElement), inElement.clone())?).clone();
    Ok(outString)
}

pub fn componentStr(mut inComponent: Arc<NFInstTypes::Component>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(NFInstDumpTpl::dumpComponent), inComponent.clone())?).clone();
    Ok(outString)
}

pub fn bindingStr(mut inBinding: NFInstTypes::Binding) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(NFInstDumpTpl::dumpBinding), inBinding.clone())?).clone();
    Ok(outString)
}

pub fn prefixStr(mut inPrefix: Arc<NFInstPrefix::Prefix>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(NFInstDumpTpl::dumpPrefix), inPrefix.clone())?).clone();
    Ok(outString)
}

pub fn equationStr(mut inEquation: Arc<NFInstTypes::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(NFInstDumpTpl::dumpEquation), inEquation.clone())?).clone();
    Ok(outString)
}

//public function connectionsStr
//  input NFConnect2.Connections inConnections;
//  output String outString;
//algorithm
//  outString := Tpl.tplString(NFInstDumpTpl.dumpConnections, inConnections);
//end connectionsStr;
pub fn dimensionStr(mut inDimension: NFInstTypes::Dimension) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(NFInstDumpTpl::dumpDimension), inDimension.clone())?).clone();
    Ok(outString)
}

pub fn dumpUntypedComponentDims(mut inComponent: Arc<NFInstTypes::Component>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inComponent.clone()) {
        Deref @ NFInstTypes::Component::UNTYPED_COMPONENT { dimensions: dims, .. } => {
            let mut dims_str: ArcStr = arcstr::literal!("");
            dims_str = (Array::toString(dims.clone(), Arc::new(dimensionStr), (literal!("")).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), false, 0)?).clone();
            dims_str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

