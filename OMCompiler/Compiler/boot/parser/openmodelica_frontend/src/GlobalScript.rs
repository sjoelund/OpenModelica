// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::Absyn;
use crate::DAE;
use crate::Values;

#[derive(Clone, Debug, PartialEq)]
pub enum Component {
    COMPONENTITEM {
        the1: Arc<Absyn::Path>,
        the2: Arc<Absyn::Path>,
        the3: Arc<Absyn::ComponentRef>,
    },
    EXTENDSITEM {
        the1: Arc<Absyn::Path>,
        the2: Arc<Absyn::Path>,
    },
}
pub use self::Component::{COMPONENTITEM,EXTENDSITEM};

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentReplacement {
    pub which1: Arc<Absyn::Path>,
    pub the2: Arc<Absyn::ComponentRef>,
    pub the3: Arc<Absyn::ComponentRef>,
}

pub type COMPONENTREPLACEMENT = ComponentReplacement;


#[derive(Clone, Debug, PartialEq)]
pub struct ComponentReplacementRules {
    pub componentReplacementLst: Arc<metamodelica::List<ComponentReplacement>>,
    pub the: i32,
}

pub type COMPONENTREPLACEMENTRULES = ComponentReplacementRules;


#[derive(Clone, Debug, PartialEq)]
pub struct Components {
    pub componentLst: Arc<metamodelica::List<Component>>,
    pub the: i32,
}

pub type COMPONENTS = Components;


#[derive(Clone, Debug, PartialEq)]
pub struct SimulationOptions {
    pub startTime: Arc<DAE::Exp>,
    pub stopTime: Arc<DAE::Exp>,
    pub numberOfIntervals: Arc<DAE::Exp>,
    pub stepSize: Arc<DAE::Exp>,
    pub tolerance: Arc<DAE::Exp>,
    pub method: Arc<DAE::Exp>,
    pub fileNamePrefix: Arc<DAE::Exp>,
    pub options: Arc<DAE::Exp>,
    pub outputFormat: Arc<DAE::Exp>,
    pub variableFilter: Arc<DAE::Exp>,
    pub cflags: Arc<DAE::Exp>,
    pub simflags: Arc<DAE::Exp>,
}

pub type SIMULATION_OPTIONS = SimulationOptions;


#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    IALG {
        algItem: Arc<Absyn::AlgorithmItem>,
    },
    IEXP {
        exp: Arc<Absyn::Exp>,
        info: SourceInfo,
    },
}
pub use self::Statement::{IALG,IEXP};

#[derive(Clone, Debug, PartialEq)]
pub struct Statements {
    pub interactiveStmtLst: Arc<metamodelica::List<Statement>>,
    pub semicolon: bool,
}

pub type ISTMTS = Statements;


#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub varIdent: ArcStr,
    pub value: Arc<Values::Value>,
    pub type_: Arc<DAE::Type>,
}

pub type IVAR = Variable;


