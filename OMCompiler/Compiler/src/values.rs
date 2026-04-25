//! Translation of FrontEnd/Values.mo
//!
//! This module defines data structures for representing constant Modelica values.
//! These include integer, real, string and boolean values, and also arrays
//! of any dimensionality and type. Multidimensional arrays are represented
//! as arrays of arrays.
//!
//! This is a direct translation of the Values package from MetaModelica.
//! All uniontypes are translated to Rust enums with struct variants.
//! All records are translated to Rust structs.
//!
//! Note: The original MetaModelica uses 1-based indexing; Rust uses 0-based indexing.

use crate::absyn::{CodeNode, Path};

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = im::Vector<T>;

// ============================================================================
// Value - Evaluated expression values
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Value {
    /// Integer value
    INTEGER { integer: i32 },

    /// Real (floating-point) value
    REAL { real: f64 },

    /// String value
    STRING { string: String },

    /// Boolean value
    BOOL { boolean: bool },

    /// Enumeration literal
    ENUM_LITERAL {
        name: Path,
        index: i32,
    },

    /// Array value (multidimensional arrays are represented as arrays of arrays)
    ARRAY {
        value_lst: List<Value>,
        dim_lst: List<i32>,
    },

    /// MetaModelica list
    LIST {
        value_lst: List<Value>,
    },

    /// MetaModelica array
    META_ARRAY {
        value_lst: List<Value>,
    },

    /// Modelica Tuple
    TUPLE {
        value_lst: List<Value>,
    },

    /// MetaModelica Tuple
    META_TUPLE {
        value_lst: List<Value>,
    },

    /// Record value
    RECORD {
        record_: Path,
        orderd: List<Value>,
        comp: List<String>,
        index: i32,
    },

    /// Optional value
    OPTION {
        some: Option<Box<Value>>,
    },

    /// Code node (a record consisting of value/ident pairs)
    CODE {
        a: CodeNode,
    },

    /// No return call
    NORETCALL,

    /// Boxed MetaModelica value (wrapper)
    META_BOX {
        value: Box<Value>,
    },

    /// MetaModelica fail marker
    /// If the result of constant evaluation of a MetaModelica function call is fail(),
    /// we need to propagate this value in order to avoid running the code over and over again.
    /// This is mostly an optimization.
    META_FAIL,

    /// Empty value - represents a constant without a binding.
    /// Used to be able to continue the evaluation of a model even if there are
    /// constants with no bindings. At the end, when we have the DAE, we should
    /// have no EMPTY values or expressions in it when we need to simulate the model.
    /// From Modelica specification: a package we look inside should not be partial
    /// in a simulation model!
    EMPTY {
        scope: String,
        name: String,
        ty: Box<Value>,
        ty_str: String,
    },
}

// ============================================================================
// IntRealOp - Integer/Real operators
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntRealOp {
    MULOP,
    DIVOP,
    ADDOP,
    SUBOP,
    POWOP,
    LESSEQOP,
}
