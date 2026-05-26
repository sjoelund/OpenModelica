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

use crate::NFInstNode::InstNode;
use crate::NFRecord as Record;
use openmodelica_util::UnorderedMap;

#[derive(Clone, Debug, PartialEq)]
pub enum NFComplexType {
    CLASS,
    /// Used for long class declarations extending from a type, e.g.:
    ///       type SomeType
    ///         extends Real;
    ///       end SomeType;
    EXTENDS_TYPE {
        baseClass: Arc<InstNode::InstNode>,
    },
    CONNECTOR {
        potentials: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
        flows: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
        streams: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
    },
    EXPANDABLE_CONNECTOR {
        potentiallyPresents: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
        expandableConnectors: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
    },
    RECORD {
        constructor: Arc<InstNode::InstNode>,
        fields: metamodelica::Array<Arc<Record::Field::Field>>,
        indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    },
    EXTERNAL_OBJECT {
        constructor: Arc<InstNode::InstNode>,
        destructor: Arc<InstNode::InstNode>,
    },
}
impl Default for NFComplexType {
    fn default() -> Self { Self::CLASS }
}
pub use self::NFComplexType::{CLASS,EXTENDS_TYPE,CONNECTOR,EXPANDABLE_CONNECTOR,RECORD,EXTERNAL_OBJECT};

