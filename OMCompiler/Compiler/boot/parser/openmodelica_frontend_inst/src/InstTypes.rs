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

use openmodelica_frontend_types::DAE;

///
/// Calling scope is used to determine when unconnected flow variables should be set to zero.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallingScope {
    /// this is a top call
    TOP_CALL,
    /// this is an inner call
    INNER_CALL,
    /// a call to determine type of a class
    TYPE_CALL,
}
pub use self::CallingScope::{TOP_CALL,INNER_CALL,TYPE_CALL};

pub type PolymorphicBindings = Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>;

pub const alwaysUnroll: bool = true;

pub const neverUnroll: bool = false;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SearchStrategy {
    /// this one searches only in the local scope, it won't find *time* variable
    SEARCH_LOCAL_ONLY,
    /// this one searches also in the builtin scope, it will find *time* variable
    SEARCH_ALSO_BUILTIN,
}
pub use self::SearchStrategy::{SEARCH_LOCAL_ONLY,SEARCH_ALSO_BUILTIN};

/// data for 'spliced expression' (typically a component reference) returned in lookupVar
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SplicedExpData {
    /// the spliced expression
    pub splicedExp: Option<Arc<DAE::Exp>>,
    /// the type of the variable without subscripts, needed for vectorization
    pub identType: Arc<DAE::Type>,
}

impl Default for SplicedExpData {
    fn default() -> Self {
        Self {
            splicedExp: Default::default(),
            identType: Default::default(),
        }
    }
}

pub type SPLICEDEXPDATA = SplicedExpData;


pub type TypeMemoryEntry = (Arc<DAE::Type>, Arc<DAE::Type>);

pub type TypeMemoryEntryList = Arc<metamodelica::List<(Arc<DAE::Type>, Arc<DAE::Type>)>>;

pub type TypeMemoryEntryListArray = metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Type>, Arc<DAE::Type>)>>>;

/// Changed from list<Subscript> to list<list<Subscript>>. One list for each scope.
/// This so when instantiating classes extending from primitive types can collect the dimension of -one- surrounding scope to create type.
/// E.g. RealInput p[3]; gives the list {3} for this scope and other lists for outer (in instance hierachy) scopes
pub type InstDims = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>;

pub fn callingScopeStr(mut inCallingScope: CallingScope) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inCallingScope.clone() {
        CallingScope::TOP_CALL { .. } => literal!("topCall"),
        CallingScope::INNER_CALL { .. } => literal!("innerCall"),
        CallingScope::TYPE_CALL { .. } => literal!("typeCall"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

