//! Translation of Script/MMToJuliaUtil.mo
//!
//! This module provides context types and utility functions for the
//! MetaModelica-to-Julia code generator.

use im::Vector;

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = Vector<T>;

// Re-export Absyn types used by this module
use crate::absyn;

// ============================================================================
// Context uniontype
// ============================================================================

/// Context - the uniontype carrying different context variants used during
/// Julia code generation (function context, package context, match context, etc.)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Context {
    FUNCTION { ret_vals_str: String },
    FUNCTION_RETURN_CONTEXT { ret_vals_str: String, ty_str: String },
    PACKAGE,
    UNIONTYPE { name: String },
    NO_CONTEXT,
    INPUT_CONTEXT { ty_str: String },
    MATCH_CONTEXT { input_exp: absyn::Exp },
}

// ============================================================================
// Context constants
// ============================================================================

pub const PACKAGE_CONTEXT: Context = Context::PACKAGE;
pub const NO_CONTEXT: Context = Context::NO_CONTEXT;
pub const FUNCTION_CONTEXT: Context = Context::FUNCTION {
    ret_vals_str: String::new(),
};
pub const RETURN_CONTEXT: Context = Context::FUNCTION_RETURN_CONTEXT {
    ret_vals_str: String::new(),
    ty_str: String::new(),
};
pub const INPUT_CONTEXT: Context = Context::INPUT_CONTEXT {
    ty_str: String::new(),
};

// ============================================================================
// Context constructor functions
// ============================================================================

/// Create a UNIONTYPE context with the given name.
pub fn make_uniontype_context(name: &str) -> Context {
    Context::UNIONTYPE {
        name: name.to_string(),
    }
}

/// Create an INPUT_CONTEXT with the given type string.
pub fn make_input_context(ty_str: &str) -> Context {
    Context::INPUT_CONTEXT {
        ty_str: ty_str.to_string(),
    }
}

/// Create a FUNCTION context with the given return values string.
pub fn make_function_context(return_values_str: &str) -> Context {
    Context::FUNCTION {
        ret_vals_str: return_values_str.to_string(),
    }
}

/// Create a FUNCTION_RETURN_CONTEXT with the given return values string and type string.
pub fn make_function_return_context(return_values_str: &str, ty_str: &str) -> Context {
    Context::FUNCTION_RETURN_CONTEXT {
        ret_vals_str: return_values_str.to_string(),
        ty_str: ty_str.to_string(),
    }
}

/// Create a MATCH_CONTEXT with the given input expression.
pub fn make_match_context(i_exp: absyn::Exp) -> Context {
    Context::MATCH_CONTEXT { input_exp: i_exp }
}

// ============================================================================
// Direction constructor functions
// ============================================================================

/// Create an Absyn INPUT direction.
pub fn make_input_direction() -> absyn::Direction {
    absyn::Direction::INPUT
}

/// Create an Absyn OUTPUT direction.
pub fn make_output_direction() -> absyn::Direction {
    absyn::Direction::OUTPUT
}

/// Create an Absyn INPUT_OUTPUT direction.
pub fn make_input_output_direction() -> absyn::Direction {
    absyn::Direction::INPUT_OUTPUT
}

/// Create an Absyn BIDIR direction.
pub fn make_bdirection() -> absyn::Direction {
    absyn::Direction::BIDIR
}

/// Check if the given context is a FUNCTION context.
pub fn is_function_context(given_ctx: &Context) -> bool {
    matches!(given_ctx, Context::FUNCTION { .. })
}

/// Compare two directions for equality.
pub fn direction_equal(d1: &absyn::Direction, d2: &absyn::Direction) -> bool {
    matches!((d1, d2),
        (absyn::Direction::BIDIR, absyn::Direction::BIDIR)
        | (absyn::Direction::INPUT, absyn::Direction::INPUT)
        | (absyn::Direction::OUTPUT, absyn::Direction::OUTPUT)
        | (absyn::Direction::INPUT_OUTPUT, absyn::Direction::INPUT_OUTPUT)
    )
}

/// Get the direction from an ElementItem, defaulting to BIDIR if no direction is found.
pub fn get_direction(element_item: &absyn::ElementItem) -> absyn::Direction {
    match element_item {
        absyn::ElementItem::ELEMENTITEM {
            element: absyn::Element::ELEMENT {
                specification: absyn::ElementSpec::COMPONENTS {
                    attributes: absyn::ElementAttributes::ATTR { direction, .. },
                    ..
                },
                ..
            },
        } => direction.clone(),
        _ => absyn::Direction::BIDIR,
    }
}

/// Filter a list of ElementItem, keeping only those whose direction matches
/// the supplied direction (or INPUT_OUTPUT).
pub fn filter_on_direction(
    inputs: &[absyn::ElementItem],
    direction: &absyn::Direction,
) -> Vec<absyn::ElementItem> {
    let io_direction = absyn::Direction::INPUT_OUTPUT;
    let mut outputs = Vec::new();
    for item in inputs {
        let item_dir = get_direction(item);
        if direction_equal(direction, &item_dir)
            || direction_equal(&io_direction, &item_dir)
        {
            outputs.push(item.clone());
        }
    }
    outputs
}

/// Check if an ElementSpec is BIDIR.
pub fn element_spec_is_bidir(spec: &absyn::ElementSpec) -> bool {
    match spec {
        absyn::ElementSpec::COMPONENTS {
            attributes: absyn::ElementAttributes::ATTR { direction, .. },
            ..
        } => *direction == absyn::Direction::BIDIR,
        _ => false,
    }
}

/// Check if an ElementSpec is OUTPUT.
pub fn element_spec_is_output(spec: &absyn::ElementSpec) -> bool {
    match spec {
        absyn::ElementSpec::COMPONENTS {
            attributes: absyn::ElementAttributes::ATTR { direction, .. },
            ..
        } => *direction == absyn::Direction::OUTPUT,
        _ => false,
    }
}

/// Check if an ElementSpec is OUTPUT or BIDIR.
pub fn element_spec_is_output_or_bidir(spec: &absyn::ElementSpec) -> bool {
    element_spec_is_output(spec) || element_spec_is_bidir(spec)
}

/// Check if any class part contains an explicit return statement.
/// Only works for ALGORITHMS class parts.
pub fn explicit_return_in_class_part(class_parts: &[absyn::ClassPart]) -> bool {
    for cp in class_parts {
        if algorithm_items_contains_return(cp) {
            return true;
        }
    }
    false
}

/// Check if a class part (typically ALGORITHMS) contains a return statement.
fn algorithm_items_contains_return(class_part: &absyn::ClassPart) -> bool {
    match class_part {
        absyn::ClassPart::ALGORITHMS { contents } => {
            for item in contents {
                if matches!(
                    item,
                    absyn::AlgorithmItem::ALGORITHMITEM {
                        algorithm_: absyn::Algorithm::ALG_RETURN,
                        ..
                    }
                ) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}
