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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    INTEGER {
        integer: i32,
    },
    REAL {
        real: metamodelica::Real,
    },
    STRING {
        string: ArcStr,
    },
    BOOL {
        boolean: bool,
    },
    ENUM_LITERAL {
        name: Arc<Absyn::Path>,
        index: i32,
    },
    ARRAY {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
        dimLst: Arc<metamodelica::List<i32>>,
    },
    /// MetaModelica list
    LIST {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
    },
    /// MetaModelica array
    META_ARRAY {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
    },
    /// Modelica Tuple
    TUPLE {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
    },
    /// MetaModelica Tuple
    META_TUPLE {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
    },
    RECORD {
        /// record name
        record_: Arc<Absyn::Path>,
        /// orderd set of values
        orderd: Arc<metamodelica::List<Arc<Value>>>,
        /// comp names for each value
        comp: Arc<metamodelica::List<ArcStr>>,
        /// -1 for regular records, 0..n-1 for uniontypes containing n records
        index: i32,
    },
    OPTION {
        some: Option<Arc<Value>>,
    },
    CODE {
        /// A record consist of value  Ident pairs
        A: Arc<Absyn::CodeNode>,
    },
    NORETCALL,
    META_BOX {
        value: Arc<Value>,
    },
    /// If the result of constant evaluation of a MetaModelica function call is fail(),
    ///    we need to propagate this value in order to avoid running the code over and over again.
    ///    This is mostly an optimization.
    META_FAIL,
    /// an empty value, meaning a constant without a binding. is used to be able to continue the evaluation of a model even if there are constants with
    ///     no bindings. at the end, when we have the DAE we should have no EMPTY values or expressions in it when we need to simulate the model.
    ///     From Modelica specification: a package may we look inside should not be partial in a simulation model!
    EMPTY {
        /// the scope where we could not find the binding
        scope: ArcStr,
        /// the name of the variable
        name: ArcStr,
        /// the DAE.Type translated to Value using defaults
        ty: Arc<Value>,
        /// the type of the variable
        tyStr: ArcStr,
    },
}
impl Default for Value {
    fn default() -> Self { Self::NORETCALL }
}
pub use self::Value::{INTEGER,REAL,STRING,BOOL,ENUM_LITERAL,ARRAY,LIST,META_ARRAY,TUPLE,META_TUPLE,RECORD,OPTION,CODE,NORETCALL,META_BOX,META_FAIL,EMPTY};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntRealOp {
    MULOP,
    DIVOP,
    ADDOP,
    SUBOP,
    POWOP,
    LESSEQOP,
}
impl Default for IntRealOp {
    fn default() -> Self { Self::MULOP }
}
pub use self::IntRealOp::{MULOP,DIVOP,ADDOP,SUBOP,POWOP,LESSEQOP};

