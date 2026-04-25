//! Translation of FrontEnd/SCodeSimplify.mo
//!
//! SCodeSimplify is used to further simplify SCode.
//! Currently it:
//! - removes extends *Icons*
//!
//! This is a direct translation of the SCodeSimplify package from MetaModelica.

use anyhow::Result;
use im::Vector;

use crate::absyn::Path;
use crate::scode::{ClassDef, Element, Program};

// ============================================================================
// Helper: pathContains
// Checks if a Path contains the given identifier name
// ============================================================================

fn path_contains(path: &Path, name: &str) -> bool {
    match path {
        Path::QUALIFIED { name: n, path: rest } => n == name || path_contains(rest, name),
        Path::IDENT { name: n } => n == name,
        Path::FULLYQUALIFIED { path: rest } => path_contains(rest, name),
    }
}

// ============================================================================
// simplifyProgram
// transforms scode to scode simplified
// ============================================================================

/// Simplifies a program (list of elements) by recursively simplifying each element.
/// Corresponds to the `simplifyProgram` function in SCodeSimplify.mo.
pub fn simplify_program(in_scode_program: Program) -> Result<Program> {
    matchcontinue_simplify_elements(in_scode_program)
}

// ============================================================================
// simplifyClass
// simplifies a class
// ============================================================================

/// Simplifies a single CLASS element by simplifying its class definition.
/// Corresponds to the `simplifyClass` function in SCodeSimplify.mo.
pub fn simplify_class(in_class: &Element) -> Result<Element> {
    match in_class {
        Element::CLASS {
            name,
            prefixes,
            encapsulated_prefix,
            partial_prefix,
            restriction,
            class_def,
            cmt,
            info,
        } => {
            let nc_def = simplify_class_def(class_def)?;
            Ok(Element::CLASS {
                name: name.clone(),
                prefixes: prefixes.clone(),
                encapsulated_prefix: encapsulated_prefix.clone(),
                partial_prefix: partial_prefix.clone(),
                restriction: restriction.clone(),
                class_def: nc_def,
                cmt: cmt.clone(),
                info: info.clone(),
            })
        }
        _ => Ok(in_class.clone()),
    }
}

// ============================================================================
// simplifyClassDef
// simplifies a classdef
// ============================================================================

/// Simplifies a ClassDef by simplifying its constituent parts.
/// Most variants (DERIVED, ENUMERATION, OVERLOAD, PDER) are passed through unchanged.
/// PARTS and CLASS_EXTENDS variants are recursively processed.
/// Corresponds to the `simplifyClassDef` function in SCodeSimplify.mo.
pub fn simplify_class_def(in_class_def: &ClassDef) -> Result<ClassDef> {
    match in_class_def {
        ClassDef::PARTS {
            element_lst,
            normal_equation_lst,
            initial_equation_lst,
            normal_algorithm_lst,
            initial_algorithm_lst,
            constraint_lst,
            clsattrs,
            external_decl,
        } => {
            let els = simplify_elements(element_lst)?;
            Ok(ClassDef::PARTS {
                element_lst: els,
                normal_equation_lst: normal_equation_lst.clone(),
                initial_equation_lst: initial_equation_lst.clone(),
                normal_algorithm_lst: normal_algorithm_lst.clone(),
                initial_algorithm_lst: initial_algorithm_lst.clone(),
                constraint_lst: constraint_lst.clone(),
                clsattrs: clsattrs.clone(),
                external_decl: external_decl.clone(),
            })
        }
        ClassDef::CLASS_EXTENDS {
            modifications,
            composition,
        } => {
            let c_def = simplify_class_def(composition)?;
            Ok(ClassDef::CLASS_EXTENDS {
                modifications: modifications.clone(),
                composition: Box::new(c_def),
            })
        }
        // DERIVED, ENUMERATION, OVERLOAD, PDER: return unchanged
        _ => Ok(in_class_def.clone()),
    }
}

// ============================================================================
// simplifyElements
// simplifies elements (matchcontinue version)
// ============================================================================

/// Simplifies a list of elements using matchcontinue semantics.
/// - Removes EXTENDS elements whose base class path contains "Icons"
/// - Recursively simplifies CLASS elements
/// - Passes through all other elements unchanged
/// Corresponds to the `simplifyElements` function in SCodeSimplify.mo.
fn simplify_elements(in_elements: &Vector<Element>) -> Result<Vector<Element>> {
    matchcontinue_simplify_elements(in_elements.clone())
}

/// Internal matchcontinue-style recursive helper for simplifyElements.
/// This simulates the MetaModelica matchcontinue control flow:
///   case ({}) -> {}
///   case (SCode.EXTENDS(Icons))::rest -> els  (skip Icons extends)
///   case (CLASS())::rest -> el2::els           (simplify CLASS)
///   case el::rest -> el::els                    (pass through)
fn matchcontinue_simplify_elements(mut elements: Vector<Element>) -> Result<Vector<Element>> {
    // Case 1: empty list
    if elements.is_empty() {
        return Ok(Vector::new());
    }

    // Remove first element; elements now contains the rest (tail)
    let head = elements.remove(0);
    let tail = elements;

    // Case: EXTENDS with "Icons" in path -> skip this element
    if let Element::EXTENDS { base_class_path, .. } = &head {
        if path_contains(base_class_path, "Icons") {
            // Skip this element, continue with rest
            return matchcontinue_simplify_elements(tail);
        }
    }

    // Case: CLASS -> simplify it
    if let Element::CLASS { .. } = &head {
        let el2 = simplify_class(&head)?;
        let els = matchcontinue_simplify_elements(tail)?;
        // Prepend el2 to els
        let mut result = els;
        result.push_back(el2);
        return Ok(result);
    }

    // Case (el::rest) -> el::els (pass through)
    let els = matchcontinue_simplify_elements(tail)?;
    let mut result = els;
    result.push_back(head);
    Ok(result)
}
