// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use openmodelica_util::Flags;

pub struct OUTPUT_FORMAT {
    pub scalarizeMode: ScalarizeMode,
    pub recordMode: RecordMode,
    pub moveBindings: bool,
}

pub type OutputFormat = OUTPUT_FORMAT;

pub enum RecordMode {
    WITH_RECORDS,
    WITHOUT_RECORDS,
}

pub enum ScalarizeMode {
    SCALARIZED,
    PARTIALLY_SCALARIZED,
    NOT_SCALARIZED,
}

pub fn formatFromFlags() -> OutputFormat {
    todo!()
}

pub fn inlineFunctions() -> bool {
    todo!()
}

