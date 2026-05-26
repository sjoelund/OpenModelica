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

use openmodelica_ast::Absyn;

/// - Machine states, the string contains the classname.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    UNKNOWN {
        path: Arc<Absyn::Path>,
    },
    OPTIMIZATION {
        path: Arc<Absyn::Path>,
    },
    MODEL {
        path: Arc<Absyn::Path>,
    },
    RECORD {
        path: Arc<Absyn::Path>,
    },
    BLOCK {
        path: Arc<Absyn::Path>,
    },
    CONNECTOR {
        path: Arc<Absyn::Path>,
        isExpandable: bool,
    },
    TYPE {
        path: Arc<Absyn::Path>,
    },
    PACKAGE {
        path: Arc<Absyn::Path>,
    },
    FUNCTION {
        path: Arc<Absyn::Path>,
        isImpure: bool,
    },
    ENUMERATION {
        path: Arc<Absyn::Path>,
    },
    HAS_RESTRICTIONS {
        path: Arc<Absyn::Path>,
        hasEquations: bool,
        hasAlgorithms: bool,
        hasConstraints: bool,
    },
    TYPE_INTEGER {
        path: Arc<Absyn::Path>,
    },
    TYPE_REAL {
        path: Arc<Absyn::Path>,
    },
    TYPE_STRING {
        path: Arc<Absyn::Path>,
    },
    TYPE_BOOL {
        path: Arc<Absyn::Path>,
    },
    TYPE_CLOCK {
        path: Arc<Absyn::Path>,
    },
    TYPE_ENUM {
        path: Arc<Absyn::Path>,
    },
    EXTERNAL_OBJ {
        path: Arc<Absyn::Path>,
    },
    META_TUPLE {
        path: Arc<Absyn::Path>,
    },
    META_LIST {
        path: Arc<Absyn::Path>,
    },
    META_OPTION {
        path: Arc<Absyn::Path>,
    },
    META_RECORD {
        path: Arc<Absyn::Path>,
    },
    META_UNIONTYPE {
        path: Arc<Absyn::Path>,
        typeVars: Arc<metamodelica::List<ArcStr>>,
    },
    META_ARRAY {
        path: Arc<Absyn::Path>,
    },
    META_POLYMORPHIC {
        path: Arc<Absyn::Path>,
    },
}
pub use self::State::{UNKNOWN,OPTIMIZATION,MODEL,RECORD,BLOCK,CONNECTOR,TYPE,PACKAGE,FUNCTION,ENUMERATION,HAS_RESTRICTIONS,TYPE_INTEGER,TYPE_REAL,TYPE_STRING,TYPE_BOOL,TYPE_CLOCK,TYPE_ENUM,EXTERNAL_OBJ,META_TUPLE,META_LIST,META_OPTION,META_RECORD,META_UNIONTYPE,META_ARRAY,META_POLYMORPHIC};

/// - Events
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event {
    /// There are equations inside the current definition
    FOUND_EQUATION,
    /// There are algorithms inside the current definition
    FOUND_ALGORITHM,
    /// There are constranit (equations) inside the current definition
    FOUND_CONSTRAINT,
    /// There is an external declaration inside the current definition
    FOUND_EXT_DECL,
    /// A definition with elements, i.e. a long definition
    NEWDEF,
    /// A Definition that contains components
    FOUND_COMPONENT {
        /// name of the component
        name: ArcStr,
    },
}
impl Default for Event {
    fn default() -> Self { Self::FOUND_EQUATION }
}
pub use self::Event::{FOUND_EQUATION,FOUND_ALGORITHM,FOUND_CONSTRAINT,FOUND_EXT_DECL,NEWDEF,FOUND_COMPONENT};

