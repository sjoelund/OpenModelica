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

use crate::HashTable;
use crate::MMath;
use openmodelica_frontend_types::DAE;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitCheckResult {
    CONSISTENT,
    INCONSISTENT {
        u1: SpecUnit,
        u2: SpecUnit,
    },
}
pub use self::UnitCheckResult::{CONSISTENT,INCONSISTENT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecUnit {
    /// A type parameter also has an exponent.
    pub typeParameters: Arc<metamodelica::List<(MMath::Rational, TypeParameter)>>,
    /// first seven elements are the SI base units
    pub units: Arc<metamodelica::List<MMath::Rational>>,
}

pub type SPECUNIT = SpecUnit;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeParameter {
    /// a type parameter name has the form identifier followed by a apostrophe, e.g. p'
    pub name: ArcStr,
    /// indx in Store
    pub indx: i32,
}

pub type TYPEPARAMETER = TypeParameter;


/// A unit is either specified (including type parameters) or unspecified
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Unit {
    /// A specified unit
    SPECIFIED {
        specified: SpecUnit,
    },
    /// Unpspecified unit means that the unit is unknown and should be inferred
    UNSPECIFIED,
}
pub use self::Unit::{SPECIFIED,UNSPECIFIED};

/// A unit term is either
/// - a binary operation, e.g. multiplication, addition, etc.
/// - an equation (equality)
/// - a location with unique id
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitTerm {
    /// addition ut1+ut2
    ADD {
        /// left
        ut1: Arc<UnitTerm>,
        /// right
        ut2: Arc<UnitTerm>,
        /// for proper error reporting
        origExp: Arc<DAE::Exp>,
    },
    /// subtraction ut1-ut2
    SUB {
        /// left
        ut1: Arc<UnitTerm>,
        /// right
        ut2: Arc<UnitTerm>,
        /// for proper error reporting
        origExp: Arc<DAE::Exp>,
    },
    /// multiplication, ut1*ut2
    MUL {
        /// left
        ut1: Arc<UnitTerm>,
        /// right
        ut2: Arc<UnitTerm>,
        /// for proper error reporting
        origExp: Arc<DAE::Exp>,
    },
    /// division nominator/denominator
    DIV {
        /// nominator
        ut1: Arc<UnitTerm>,
        /// denominator
        ut2: Arc<UnitTerm>,
        /// for proper error reporting
        origExp: Arc<DAE::Exp>,
    },
    /// equation
    EQN {
        ut1: Arc<UnitTerm>,
        ut2: Arc<UnitTerm>,
        /// for proper error reporting
        origExp: Arc<DAE::Exp>,
    },
    /// location
    LOC {
        /// location is an integer(index in vector)
        loc: i32,
        /// for proper error reporting
        origExp: Arc<DAE::Exp>,
    },
    /// exponentiation
    POW {
        ut1: Arc<UnitTerm>,
        /// ut^exponent
        exponent: MMath::Rational,
        /// for proper error reporting
        origExp: Arc<DAE::Exp>,
    },
}
pub use self::UnitTerm::{ADD,SUB,MUL,DIV,EQN,LOC,POW};

pub type UnitTerms = Arc<metamodelica::List<Arc<UnitTerm>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Store {
    pub storeVector: metamodelica::Array<Option<Unit>>,
    /// Number of elements stored in vector
    pub numElts: i32,
}

pub type STORE = Store;


/// A store used in Inst.mo
/// requires a mapping from variable names to locations. Unit checking can be turned off using NOSTORE
#[derive(Clone, Debug, PartialEq)]
pub enum InstStore {
    INSTSTORE {
        store: Store,
        ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)),
        /// when a check is done the result is stored here
        checkResult: Option<UnitCheckResult>,
    },
    /// used to skip unit checking
    NOSTORE,
}
pub use self::InstStore::{INSTSTORE,NOSTORE};

pub const fn noStore() -> InstStore { crate::UnitAbsyn::InstStore::NOSTORE }

