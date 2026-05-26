// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_ast::Absyn;

#[derive(Clone, Debug, PartialEq)]
pub enum IntRealOp {
    MULOP,
    DIVOP,
    ADDOP,
    SUBOP,
    POWOP,
    LESSEQOP,
}
pub use self::IntRealOp::{MULOP,DIVOP,ADDOP,SUBOP,POWOP,LESSEQOP};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    INTEGER {
        integer: i32,
    },
    REAL {
        real: f64,
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
    LIST {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
    },
    META_ARRAY {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
    },
    TUPLE {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
    },
    META_TUPLE {
        valueLst: Arc<metamodelica::List<Arc<Value>>>,
    },
    RECORD {
        record_: Arc<Absyn::Path>,
        orderd: Arc<metamodelica::List<Arc<Value>>>,
        comp: Arc<metamodelica::List<ArcStr>>,
        index: i32,
    },
    OPTION {
        some: Option<Arc<Value>>,
    },
    CODE {
        A: Arc<Absyn::CodeNode>,
    },
    NORETCALL,
    META_BOX {
        value: Arc<Value>,
    },
    META_FAIL,
    EMPTY {
        scope: ArcStr,
        name: ArcStr,
        ty: Arc<Value>,
        tyStr: ArcStr,
    },
}
pub use self::Value::{INTEGER,REAL,STRING,BOOL,ENUM_LITERAL,ARRAY,LIST,META_ARRAY,TUPLE,META_TUPLE,RECORD,OPTION,CODE,NORETCALL,META_BOX,META_FAIL,EMPTY};

