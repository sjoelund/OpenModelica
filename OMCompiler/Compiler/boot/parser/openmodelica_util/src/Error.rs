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

use crate::ErrorExt;
use crate::ErrorTypes;
use crate::Flags;
use crate::Gettext;
use crate::Global;
use crate::System;
use crate::Testsuite;
use crate::Util;

pub static LOOKUP_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 3, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class %s not found in scope %s.") } };

pub static LOOKUP_ERROR_COMPNAME: ErrorTypes::Message = ErrorTypes::Message { id: 4, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class %s not found in scope %s while instantiating %s.") } };

pub static LOOKUP_VARIABLE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 5, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable %s not found in scope %s.") } };

pub static ASSIGN_CONSTANT_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 6, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Trying to assign to constant component in %s := %s") } };

pub static ASSIGN_PARAM_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Trying to assign to parameter component in %s := %s") } };

pub static ASSIGN_READONLY_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 8, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Trying to assign to %s component %s.") } };

pub static ASSIGN_TYPE_MISMATCH_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 9, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in assignment in %s := %s of %s := %s") } };

pub static IF_CONDITION_TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 10, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type error in conditional '%s'. Expected Boolean, got %s.") } };

pub static FOR_EXPRESSION_TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 11, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type error in iteration range '%s'. Expected array got %s.") } };

pub static WHEN_CONDITION_TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 12, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type error in when conditional '%s'. Expected Boolean scalar or vector, got %s.") } };

pub static WHILE_CONDITION_TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 13, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type error in while conditional '%s'. Expected Boolean got %s.") } };

pub static END_ILLEGAL_USE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 14, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'end' can not be used outside array subscripts.") } };

pub static DIVISION_BY_ZERO: ErrorTypes::Message = ErrorTypes::Message { id: 15, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Division by zero in %s / %s") } };

pub static MODULO_BY_ZERO: ErrorTypes::Message = ErrorTypes::Message { id: 16, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Modulo by zero in mod(%s,%s).") } };

pub static REM_ARG_ZERO: ErrorTypes::Message = ErrorTypes::Message { id: 17, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Second argument in rem is zero in rem(%s,%s).") } };

pub static SCRIPT_READ_SIM_RES_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 18, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Error reading simulation result.") } };

pub static EXTENDS_LOOP: ErrorTypes::Message = ErrorTypes::Message { id: 19, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("extends %s causes an instantiation loop.") } };

pub static LOAD_MODEL_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 20, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class %s not found.") } };

pub static WRITING_FILE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 21, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Error writing to file %s.") } };

pub static SIMULATOR_BUILD_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 22, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Error building simulator. Build log: %s") } };

pub static DIMENSION_NOT_KNOWN: ErrorTypes::Message = ErrorTypes::Message { id: 23, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Dimensions must be parameter or constant expression (in %s).") } };

pub static UNBOUND_VALUE: ErrorTypes::Message = ErrorTypes::Message { id: 24, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable %s has no value.") } };

pub static NEGATIVE_SQRT: ErrorTypes::Message = ErrorTypes::Message { id: 25, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Negative value as argument to sqrt.") } };

pub static NO_CONSTANT_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 26, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Constant '%s' has no value.") } };

pub static TYPE_NOT_FROM_PREDEFINED: ErrorTypes::Message = ErrorTypes::Message { id: 27, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("In class %s, class specialization 'type' can only be derived from predefined types.") } };

pub static INCOMPATIBLE_CONNECTOR_VARIABILITY: ErrorTypes::Message = ErrorTypes::Message { id: 28, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot connect %s %s to non-constant/parameter %s.") } };

pub static INVALID_CONNECTOR_PREFIXES: ErrorTypes::Message = ErrorTypes::Message { id: 29, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Connector element %s may not be both %s and %s.") } };

pub static INVALID_COMPLEX_CONNECTOR_VARIABILITY: ErrorTypes::Message = ErrorTypes::Message { id: 30, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is a composite connector element, and may not be declared as %s.") } };

pub static DIFFERENT_NO_EQUATION_IF_BRANCHES: ErrorTypes::Message = ErrorTypes::Message { id: 31, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Different number of equations in the branches of the if equation: %s") } };

pub static UNDERDET_EQN_SYSTEM: ErrorTypes::Message = ErrorTypes::Message { id: 32, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Too few equations, under-determined system. The model has %s equation(s) and %s variable(s).") } };

pub static OVERDET_EQN_SYSTEM: ErrorTypes::Message = ErrorTypes::Message { id: 33, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Too many equations, over-determined system. The model has %s equation(s) and %s variable(s).") } };

pub static STRUCT_SINGULAR_SYSTEM: ErrorTypes::Message = ErrorTypes::Message { id: 34, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Model is structurally singular, error found sorting equations\n%s\nfor variables\n%s") } };

pub static UNSUPPORTED_LANGUAGE_FEATURE: ErrorTypes::Message = ErrorTypes::Message { id: 35, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The language feature %s is not supported. Suggested workaround: %s") } };

pub static NON_EXISTING_DERIVATIVE: ErrorTypes::Message = ErrorTypes::Message { id: 36, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Derivative of expression \"%s\" w.r.t. \"%s\" is non-existent.") } };

pub static NO_CLASSES_LOADED: ErrorTypes::Message = ErrorTypes::Message { id: 37, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("No classes are loaded.") } };

pub static INST_PARTIAL_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 38, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal to instantiate partial class %s.") } };

pub static LOOKUP_BASECLASS_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 39, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Base class %s not found in scope %s.") } };

pub static INVALID_REDECLARE_AS: ErrorTypes::Message = ErrorTypes::Message { id: 40, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid redeclaration of %s %s as %s.") } };

pub static REDECLARE_NON_REPLACEABLE: ErrorTypes::Message = ErrorTypes::Message { id: 41, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Redeclaration with a new type requires '%s' to be replaceable.") } };

pub static COMPONENT_INPUT_OUTPUT_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 42, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component declared as %s when having the variable %s declared as %s.") } };

pub static ARRAY_DIMENSION_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 43, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Array dimension mismatch, expression %s has type %s, expected array dimensions [%s].") } };

pub static ARRAY_DIMENSION_INTEGER: ErrorTypes::Message = ErrorTypes::Message { id: 44, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Array dimension must be integer expression in %s which has type %s.") } };

pub static EQUATION_TYPE_MISMATCH_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 45, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in equation %s of type %s.") } };

pub static INST_ARRAY_EQ_UNKNOWN_SIZE: ErrorTypes::Message = ErrorTypes::Message { id: 46, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Array equation has unknown size in %s.") } };

pub static TUPLE_ASSIGN_FUNCALL_ONLY: ErrorTypes::Message = ErrorTypes::Message { id: 47, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Tuple assignment only allowed when rhs is function call (in %s).") } };

pub static INVALID_CONNECTOR_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 48, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is not a valid connector.") } };

pub static EXPANDABLE_NON_EXPANDABLE_CONNECTION: ErrorTypes::Message = ErrorTypes::Message { id: 49, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot connect expandable connector %s with non-expandable connector %s.") } };

pub static UNDECLARED_CONNECTION: ErrorTypes::Message = ErrorTypes::Message { id: 50, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot connect undeclared connectors %s with %s. At least one of them must be declared.") } };

pub static CONNECT_PREFIX_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 51, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot connect %1 component %2 to non-%1 component %3.") } };

pub static INVALID_CONNECTOR_VARIABLE: ErrorTypes::Message = ErrorTypes::Message { id: 52, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The type of variables %s and %s\nare inconsistent in connect equations.") } };

pub static TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 53, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Wrong type on %s, expected %s.") } };

pub static MODIFY_PROTECTED: ErrorTypes::Message = ErrorTypes::Message { id: 54, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Modification or redeclaration of protected elements is not allowed.\n  Element: %s, modification: %s.") } };

pub static INVALID_TUPLE_CONTENT: ErrorTypes::Message = ErrorTypes::Message { id: 55, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Tuple %s must contain component references only.") } };

pub static MISSING_REDECLARE_IN_CLASS_MOD: ErrorTypes::Message = ErrorTypes::Message { id: 56, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Missing redeclare keyword on attempted redeclaration of class %s.") } };

pub static IMPORT_SEVERAL_NAMES: ErrorTypes::Message = ErrorTypes::Message { id: 57, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s found in several unqualified import statements.") } };

pub static LOOKUP_TYPE_FOUND_COMP: ErrorTypes::Message = ErrorTypes::Message { id: 58, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found a component with same name when looking for type %s.") } };

pub static INHERITED_EXTENDS: ErrorTypes::Message = ErrorTypes::Message { id: 59, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The base class name %s was found in one or more base classes:") } };

pub static EXTEND_THROUGH_COMPONENT: ErrorTypes::Message = ErrorTypes::Message { id: 60, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Part %s of base class name %s is not a class.") } };

pub static PROTECTED_ACCESS: ErrorTypes::Message = ErrorTypes::Message { id: 61, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal access of protected element %s.") } };

pub static ILLEGAL_MODIFICATION: ErrorTypes::Message = ErrorTypes::Message { id: 62, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal modification %s (of %s).") } };

pub static INTERNAL_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 63, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Internal error %s") } };

pub static TYPE_MISMATCH_ARRAY_EXP: ErrorTypes::Message = ErrorTypes::Message { id: 64, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in array expression in component %s. %s is of type %s while the elements %s are of type %s.") } };

pub static TYPE_MISMATCH_MATRIX_EXP: ErrorTypes::Message = ErrorTypes::Message { id: 65, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in matrix rows in component %s. %s is a row of %s, the rest of the matrix is of type %s.") } };

pub static MATRIX_EXP_ROW_SIZE: ErrorTypes::Message = ErrorTypes::Message { id: 66, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Incompatible row length in matrix expression in component %s. %s is a row of size %s, the rest of the matrix rows are of size %s.") } };

pub static OPERAND_BUILTIN_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 67, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operand of %s in component %s must be builtin-type in %s.") } };

pub static WRONG_TYPE_OR_NO_OF_ARGS: ErrorTypes::Message = ErrorTypes::Message { id: 68, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Wrong type or wrong number of arguments to %s (in component %s).") } };

pub static DIFFERENT_DIM_SIZE_IN_ARGUMENTS: ErrorTypes::Message = ErrorTypes::Message { id: 69, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Different dimension sizes in arguments to %s in component %s.") } };

pub static LOOKUP_IMPORT_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 70, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Import %s not found in scope %s.") } };

pub static LOOKUP_SHADOWING: ErrorTypes::Message = ErrorTypes::Message { id: 71, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Import %s is shadowed by a local element.") } };

pub static ARGUMENT_MUST_BE_INTEGER: ErrorTypes::Message = ErrorTypes::Message { id: 72, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s argument to %s in component %s must be Integer expression.") } };

pub static ARGUMENT_MUST_BE_DISCRETE_VAR: ErrorTypes::Message = ErrorTypes::Message { id: 73, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s argument to %s in component %s must be discrete variable.") } };

pub static TYPE_MUST_BE_SIMPLE: ErrorTypes::Message = ErrorTypes::Message { id: 74, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type in %s must be simple type in component %s.") } };

pub static ARGUMENT_MUST_BE_VARIABLE: ErrorTypes::Message = ErrorTypes::Message { id: 75, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s argument to %s in component %s must be a variable.") } };

pub static NO_MATCHING_FUNCTION_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 76, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("No matching function found for %s in component %s\ncandidates are %s") } };

pub static NO_MATCHING_FUNCTION_FOUND_NO_CANDIDATE: ErrorTypes::Message = ErrorTypes::Message { id: 77, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("No matching function found for %s.") } };

pub static FUNCTION_COMPS_MUST_HAVE_DIRECTION: ErrorTypes::Message = ErrorTypes::Message { id: 78, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component %s in function is neither input nor output.") } };

pub static FUNCTION_SLOT_ALREADY_FILLED: ErrorTypes::Message = ErrorTypes::Message { id: 79, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Slot %s already filled in a function call in component %s.") } };

pub static NO_SUCH_PARAMETER: ErrorTypes::Message = ErrorTypes::Message { id: 80, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function %s has no parameter named %s.") } };

pub static CONSTANT_OR_PARAM_WITH_NONCONST_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 81, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is a constant or parameter with a non-constant initializer %s.") } };

pub static WRONG_DIMENSION_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 82, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Subscript %s of type %s is not a subtype of Integer, Boolean or enumeration.") } };

pub static TYPE_MISMATCH_IF_EXP: ErrorTypes::Message = ErrorTypes::Message { id: 83, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in if-expression in component %s. True branch: %s has type %s, false branch: %s has type %s.") } };

pub static UNRESOLVABLE_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 84, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot resolve type of expression %s. The operands have types %s in component %s.") } };

pub static INCOMPATIBLE_TYPES: ErrorTypes::Message = ErrorTypes::Message { id: 85, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Incompatible argument types to operation %s in component %s, left type: %s, right type: %s") } };

pub static NON_ENCAPSULATED_CLASS_ACCESS: ErrorTypes::Message = ErrorTypes::Message { id: 86, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class %s does not satisfy the requirements for a package. Lookup is therefore restricted to encapsulated elements, but %s is not encapsulated.") } };

pub static INHERIT_BASIC_WITH_COMPS: ErrorTypes::Message = ErrorTypes::Message { id: 87, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class %s inherits builtin type but has components.") } };

pub static MODIFIER_TYPE_MISMATCH_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 88, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in modifier of component %s, expected type %s, got modifier %s of type %s.") } };

pub static ERROR_FLATTENING: ErrorTypes::Message = ErrorTypes::Message { id: 89, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Error occurred while flattening model %s") } };

pub static DUPLICATE_ELEMENTS_NOT_IDENTICAL: ErrorTypes::Message = ErrorTypes::Message { id: 90, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Duplicate elements (due to inherited elements) not identical:\n  first element is:  %s\n  second element is: %s") } };

pub static PACKAGE_VARIABLE_NOT_CONSTANT: ErrorTypes::Message = ErrorTypes::Message { id: 91, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable %s in package %s is not constant.") } };

pub static RECURSIVE_DEFINITION: ErrorTypes::Message = ErrorTypes::Message { id: 92, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Declaration of element %s causes recursive definition of class %s.") } };

pub static NOT_ARRAY_TYPE_IN_FOR_STATEMENT: ErrorTypes::Message = ErrorTypes::Message { id: 93, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expression %s in for-statement must be an array type.") } };

pub static NON_CLASS_IN_COMP_FUNC_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 94, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found non-class %s while looking for function via component. The only valid form is c1..cN.C1..CN.f where c1..cN are scalar components and C1..CN are classes.") } };

pub static DIFFERENT_VARIABLES_SOLVED_IN_ELSEWHEN: ErrorTypes::Message = ErrorTypes::Message { id: 95, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The same variables must be solved in elsewhen clause as in the when clause.") } };

pub static CLASS_IN_COMPOSITE_COMP_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 96, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found class %s during lookup of composite component name '%s', expected component.") } };

pub static MODIFIER_DECLARATION_TYPE_MISMATCH_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 97, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in modifier of component %s, declared type %s, got modifier %s of type %s.") } };

pub static ASSERT_CONSTANT_FALSE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 98, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Assertion triggered during translation: %s.") } };

pub static ARRAY_INDEX_OUT_OF_BOUNDS: ErrorTypes::Message = ErrorTypes::Message { id: 99, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Subscript '%s' for dimension %s (size = %s) of %s is out of bounds.") } };

pub static COMPONENT_CONDITION_VARIABILITY: ErrorTypes::Message = ErrorTypes::Message { id: 100, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component condition must be parameter or constant expression (in %s).") } };

pub static FOUND_CLASS_NAME_VIA_COMPONENT: ErrorTypes::Message = ErrorTypes::Message { id: 101, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class name '%s' was found via a component (only component and function call names may be accessed in this way).") } };

pub static FOUND_FUNC_NAME_VIA_COMP_NONCALL: ErrorTypes::Message = ErrorTypes::Message { id: 102, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found function %s by name lookup via component, but this is only valid when the name is used as a function call.") } };

pub static DUPLICATE_MODIFICATIONS: ErrorTypes::Message = ErrorTypes::Message { id: 103, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Duplicate modification of element %s on %s.") } };

pub static ILLEGAL_SUBSCRIPT: ErrorTypes::Message = ErrorTypes::Message { id: 104, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal subscript %s for dimensions %s in component %s.") } };

pub static ILLEGAL_EQUATION_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 105, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal type in equation %s, only builtin types (Real, String, Integer, Boolean or enumeration) or record type allowed in equation.") } };

pub static EVAL_LOOP_LIMIT_REACHED: ErrorTypes::Message = ErrorTypes::Message { id: 106, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The loop iteration limit (--evalLoopLimit=%s) was exceeded during evaluation.") } };

pub static LOOKUP_IN_PARTIAL_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 107, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is partial, name lookup is not allowed in partial classes.") } };

pub static MISSING_INNER_PREFIX: ErrorTypes::Message = ErrorTypes::Message { id: 108, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("No corresponding 'inner' declaration found for component %s declared as '%s'.\n  The existing 'inner' components are:\n    %s\n  Check if you have not misspelled the 'outer' component name.\n  Please declare an 'inner' component with the same name in the top scope.\n  Continuing flattening by only considering the 'outer' component declaration.") } };

pub static NON_PARAMETER_ITERATOR_RANGE: ErrorTypes::Message = ErrorTypes::Message { id: 109, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The iteration range %s is not a constant or parameter expression.") } };

pub static IMPLICIT_ITERATOR_NOT_FOUND_IN_LOOP_BODY: ErrorTypes::Message = ErrorTypes::Message { id: 110, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Identifier %s of implicit for iterator must be present as array subscript in the loop body.") } };

pub static CONNECTOR_NON_PARAMETER_SUBSCRIPT: ErrorTypes::Message = ErrorTypes::Message { id: 111, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Connector '%s' has non-parameter subscript '%s'.") } };

pub static LOOKUP_CLASS_VIA_COMP_COMP: ErrorTypes::Message = ErrorTypes::Message { id: 112, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal access of class '%s' via a component when looking for '%s'.") } };

pub static SUBSCRIPTED_FUNCTION_CALL: ErrorTypes::Message = ErrorTypes::Message { id: 113, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function call %s contains subscripts.") } };

pub static IF_EQUATION_UNBALANCED: ErrorTypes::Message = ErrorTypes::Message { id: 114, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("In equation %s. If-equation with conditions that are not parameter expressions must have the same number of equations in each branch, equation count is %s for each respective branch.") } };

pub static IF_EQUATION_MISSING_ELSE: ErrorTypes::Message = ErrorTypes::Message { id: 115, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Missing else-clause in if-equation with non-parameter conditions.") } };

pub static CONNECT_IN_IF: ErrorTypes::Message = ErrorTypes::Message { id: 116, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("connect may not be used inside if-equations with non-parametric conditions (found connect(%s, %s)).") } };

pub static CONNECT_IN_WHEN: ErrorTypes::Message = ErrorTypes::Message { id: 117, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("connect may not be used inside when-equations (found connect(%s, %s)).") } };

pub static CONNECT_INCOMPATIBLE_TYPES: ErrorTypes::Message = ErrorTypes::Message { id: 118, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Incompatible components in connect statement: connect(%s, %s)\n- %s has components %s\n- %s has components %s") } };

pub static CONNECT_OUTER_OUTER: ErrorTypes::Message = ErrorTypes::Message { id: 119, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal connecting two outer connectors in statement connect(%s, %s).") } };

pub static CONNECTOR_ARRAY_NONCONSTANT: ErrorTypes::Message = ErrorTypes::Message { id: 120, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("in statement %s, subscript %s is not a parameter or constant.") } };

pub static CONNECTOR_ARRAY_DIFFERENT: ErrorTypes::Message = ErrorTypes::Message { id: 121, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unmatched dimension in equation connect(%s, %s), %s != %s.") } };

pub static MODIFIER_NON_ARRAY_TYPE_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 122, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Non-array modification '%s' for array component, possibly due to missing 'each'.") } };

pub static BUILTIN_VECTOR_INVALID_DIMENSIONS: ErrorTypes::Message = ErrorTypes::Message { id: 123, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("In scope %s, in component %s: Invalid dimensions %s in %s, no more than one dimension may have size > 1.") } };

pub static UNROLL_LOOP_CONTAINING_WHEN: ErrorTypes::Message = ErrorTypes::Message { id: 124, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unable to unroll for loop containing when statements or equations: %s.") } };

pub static CIRCULAR_PARAM: ErrorTypes::Message = ErrorTypes::Message { id: 125, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable '%s' has a cyclic dependency and has variability %s.") } };

pub static NESTED_WHEN: ErrorTypes::Message = ErrorTypes::Message { id: 126, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Nested when statements are not allowed.") } };

pub static INVALID_ENUM_LITERAL: ErrorTypes::Message = ErrorTypes::Message { id: 127, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid use of reserved attribute name %s as enumeration literal.") } };

pub static UNEXPECTED_FUNCTION_INPUTS_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 128, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function %s has not the expected inputs. Expected inputs are %s.") } };

pub static DUPLICATE_CLASSES_NOT_EQUIVALENT: ErrorTypes::Message = ErrorTypes::Message { id: 129, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Duplicate class definitions (due to inheritance) not equivalent, first definition is: %s, second definition is: %s.") } };

pub static HIGHER_VARIABILITY_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 130, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component %s of variability %s has binding %s of higher variability %s.") } };

pub static IF_EQUATION_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 131, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("If-equations are only partially supported. Ignoring %s.") } };

pub static IF_EQUATION_UNBALANCED_2: ErrorTypes::Message = ErrorTypes::Message { id: 132, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("If-equation with conditions that are not parameter expressions must have the same number of equations in each branch, equation count is %s for each respective branch:\n%s") } };

pub static EQUATION_GENERIC_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 133, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to instantiate equation %s.") } };

pub static INST_PARTIAL_CLASS_CHECK_MODEL_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 134, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Forcing full instantiation of partial class %s during checkModel.") } };

pub static VARIABLE_BINDING_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 135, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in binding %s = %s, expected subtype of %s, got type %s.") } };

pub static COMPONENT_NAME_SAME_AS_TYPE_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 136, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component %s has the same name as its type %s.\n\tThis is forbidden by Modelica specification and may lead to lookup errors.") } };

pub static CONDITIONAL_EXP_WITHOUT_VALUE: ErrorTypes::Message = ErrorTypes::Message { id: 137, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The conditional expression %s could not be evaluated.") } };

pub static INCOMPATIBLE_IMPLICIT_RANGES: ErrorTypes::Message = ErrorTypes::Message { id: 138, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Dimension %s of %s and %s of %s differs when trying to deduce implicit iteration range.") } };

pub static INITIAL_WHEN: ErrorTypes::Message = ErrorTypes::Message { id: 139, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("when-clause is not allowed in initial section.") } };

pub static MODIFICATION_INDEX_NOT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 140, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Instantiation of array component: %s failed because index modification: %s is invalid.\n\tArray component: %s has more dimensions than binding %s.") } };

pub static DUPLICATE_MODIFICATIONS_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 141, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Duplicate modifications for attribute: %s in modifier: %s.\n\tConsidering only the first modification: %s and ignoring the rest %s.") } };

pub static GENERATECODE_INVARS_HAS_FUNCTION_PTR: ErrorTypes::Message = ErrorTypes::Message { id: 142, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s has a function pointer as input. OpenModelica does not support this feature in the interactive environment. Suggested workaround: Call this function with the arguments you want from another function (that does not have function pointer input). Then call that function from the interactive environment instead.") } };

pub static LOOKUP_FOUND_WRONG_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 143, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expected %s to be a %s, but found %s instead.") } };

pub static DUPLICATE_ELEMENTS_NOT_SYNTACTICALLY_IDENTICAL: ErrorTypes::Message = ErrorTypes::Message { id: 144, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Duplicate elements (due to inherited elements) not syntactically identical but semantically identical:\n\tfirst element is:  %s\tsecond element is: %s\tModelica specification requires that elements are exactly identical.") } };

pub static GENERIC_INST_FUNCTION: ErrorTypes::Message = ErrorTypes::Message { id: 145, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to instantiate function %s in scope %s.") } };

pub static WRONG_NO_OF_ARGS: ErrorTypes::Message = ErrorTypes::Message { id: 146, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Wrong number of arguments to %s.") } };

pub static TUPLE_ASSIGN_CREFS_ONLY: ErrorTypes::Message = ErrorTypes::Message { id: 147, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Tuple assignment only allowed for tuple of component references in lhs (in %s).") } };

pub static LOOKUP_FUNCTION_GOT_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 148, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Looking for a function %s but found a %s.") } };

pub static NON_STREAM_OPERAND_IN_STREAM_OPERATOR: ErrorTypes::Message = ErrorTypes::Message { id: 149, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operand '%s' to operator '%s' is not a stream variable.") } };

pub static UNBALANCED_CONNECTOR: ErrorTypes::Message = ErrorTypes::Message { id: 150, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Connector %s is not balanced: The number of potential variables (%s) is not equal to the number of flow variables (%s).") } };

pub static RESTRICTION_VIOLATION: ErrorTypes::Message = ErrorTypes::Message { id: 151, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class specialization violation: %s is a %s, not a %s.") } };

pub static ZERO_STEP_IN_ARRAY_CONSTRUCTOR: ErrorTypes::Message = ErrorTypes::Message { id: 152, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Step equals 0 in array constructor %s.") } };

pub static RECURSIVE_SHORT_CLASS_DEFINITION: ErrorTypes::Message = ErrorTypes::Message { id: 153, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Recursive short class definition of %s in terms of %s.") } };

pub static WRONG_NUMBER_OF_SUBSCRIPTS: ErrorTypes::Message = ErrorTypes::Message { id: 154, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Wrong number of subscripts in %s (%s subscripts for %s dimensions).") } };

pub static FUNCTION_ELEMENT_WRONG_KIND: ErrorTypes::Message = ErrorTypes::Message { id: 155, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Element is not allowed in function context: %s") } };

pub static MISSING_BINDING_PROTECTED_RECORD_VAR: ErrorTypes::Message = ErrorTypes::Message { id: 156, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Protected record member %s has no binding and is not modifiable by a record constructor.") } };

pub static DUPLICATE_CLASSES_TOP_LEVEL: ErrorTypes::Message = ErrorTypes::Message { id: 157, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Duplicate classes on top level is not allowed (got %s).") } };

pub static WHEN_EQ_LHS: ErrorTypes::Message = ErrorTypes::Message { id: 158, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid left-hand side of when-equation: %s.") } };

pub static GENERIC_ELAB_EXPRESSION: ErrorTypes::Message = ErrorTypes::Message { id: 159, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to elaborate expression: %s.") } };

pub static EXTENDS_EXTERNAL: ErrorTypes::Message = ErrorTypes::Message { id: 160, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Ignoring external declaration of the extended class: %s.") } };

pub static DOUBLE_DECLARATION_OF_ELEMENTS: ErrorTypes::Message = ErrorTypes::Message { id: 161, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("An element with name %s is already declared in this scope.") } };

pub static INVALID_REDECLARATION_OF_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 162, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid redeclaration of class %s, class extends only allowed on inherited classes.") } };

pub static MULTIPLE_QUALIFIED_IMPORTS_WITH_SAME_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 163, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Qualified import name %s already exists in this scope.") } };

pub static EXTENDS_INHERITED_FROM_LOCAL_EXTENDS: ErrorTypes::Message = ErrorTypes::Message { id: 164, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s was found in base class %s.") } };

pub static LOOKUP_FUNCTION_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 165, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function %s not found in scope %s.") } };

pub static ELAB_CODE_EXP_FAILED: ErrorTypes::Message = ErrorTypes::Message { id: 166, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to elaborate %s as a code expression of type %s.") } };

pub static EQUATION_TRANSITION_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 167, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Equations are not allowed in %s.") } };

pub static METARECORD_CONTAINS_METARECORD_MEMBER: ErrorTypes::Message = ErrorTypes::Message { id: 168, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The called uniontype record (%s) contains a member (%s) that has a uniontype record as its type instead of a uniontype.") } };

pub static INVALID_EXTERNAL_OBJECT: ErrorTypes::Message = ErrorTypes::Message { id: 169, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid external object %s, %s.") } };

pub static CIRCULAR_COMPONENTS: ErrorTypes::Message = ErrorTypes::Message { id: 170, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cyclically dependent constants or parameters found in scope %s: %s (ignore with -d=ignoreCycles).") } };

pub static FAILURE_TO_DEDUCE_DIMS_FROM_MOD: ErrorTypes::Message = ErrorTypes::Message { id: 171, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to deduce dimensions of %s due to unknown dimensions of modifier %s.") } };

pub static REPLACEABLE_BASE_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 172, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class '%s' in 'extends %s' is replaceable, the base class name must be transitively non-replaceable.") } };

pub static NON_REPLACEABLE_CLASS_EXTENDS: ErrorTypes::Message = ErrorTypes::Message { id: 173, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Non-replaceable base class %s in class extends.") } };

pub static ERROR_FROM_HERE: ErrorTypes::Message = ErrorTypes::Message { id: 174, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("From here:") } };

pub static EXTERNAL_FUNCTION_RESULT_NOT_CREF: ErrorTypes::Message = ErrorTypes::Message { id: 175, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The lhs (result) of the external function declaration is not a component reference: %s.") } };

pub static EXTERNAL_FUNCTION_RESULT_NOT_VAR: ErrorTypes::Message = ErrorTypes::Message { id: 176, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The lhs (result) of the external function declaration is not a variable.") } };

pub static EXTERNAL_FUNCTION_RESULT_ARRAY_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 177, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The lhs (result) of the external function declaration has array type (%s), but this is not allowed in the specification. You need to pass it as an input to the function (preferably also with a size()-expression to avoid out-of-bounds errors in the external call).") } };

pub static INVALID_REDECLARE: ErrorTypes::Message = ErrorTypes::Message { id: 178, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Redeclaration of %s %s %s is not allowed.") } };

pub static INVALID_TYPE_PREFIX: ErrorTypes::Message = ErrorTypes::Message { id: 179, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid type prefix '%s' on %s %s, due to existing type prefix '%s'.") } };

pub static LINEAR_SYSTEM_INVALID: ErrorTypes::Message = ErrorTypes::Message { id: 180, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Linear solver (%s) returned invalid input for linear system %s.") } };

pub static LINEAR_SYSTEM_SINGULAR: ErrorTypes::Message = ErrorTypes::Message { id: 181, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The linear system: %1\n might be structurally or numerically singular for variable %3 since U(%2,%2) = 0.0. It might be hard to solve. Compilation continues anyway.") } };

pub static EMPTY_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 182, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Array constructor may not be empty.") } };

pub static LOAD_MODEL_DIFFERENT_VERSIONS: ErrorTypes::Message = ErrorTypes::Message { id: 183, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Requested package %s of version %s, but this package was already loaded with version %s. OpenModelica cannot reason about compatibility between the two packages since they are not semantic versions.") } };

pub static LOAD_MODEL_FAILED: ErrorTypes::Message = ErrorTypes::Message { id: 184, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to load package %s (%s) using MODELICAPATH %s.") } };

pub static LOAD_FILE_FAILED: ErrorTypes::Message = ErrorTypes::Message { id: 185, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to load file %s: %s.") } };

pub static INVALID_SIZE_INDEX: ErrorTypes::Message = ErrorTypes::Message { id: 186, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid index %s in call to size of %s, valid index interval is [1,%s].") } };

pub static ALGORITHM_TRANSITION_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 187, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Algorithm sections are not allowed in %s.") } };

pub static FAILURE_TO_DEDUCE_DIMS_NO_MOD: ErrorTypes::Message = ErrorTypes::Message { id: 188, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to deduce dimension %s of %s due to missing binding equation.") } };

pub static FUNCTION_MULTIPLE_ALGORITHM: ErrorTypes::Message = ErrorTypes::Message { id: 189, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The behavior of multiple algorithm sections in function %s is not standard Modelica. OpenModelica will execute the sections in the order in which they were declared or inherited (same ordering as inherited input/output arguments, which also are not standardized).") } };

pub static STATEMENT_GENERIC_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 190, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to instantiate statement:\n%s") } };

pub static EXTERNAL_NOT_SINGLE_RESULT: ErrorTypes::Message = ErrorTypes::Message { id: 191, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is an unbound output in external function %s. Either add it to the external declaration or add a default binding.") } };

pub static FUNCTION_UNUSED_INPUT: ErrorTypes::Message = ErrorTypes::Message { id: 192, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unused input variable %s in function %s.") } };

pub static ARRAY_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 193, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Array types mismatch: %s and %s.") } };

pub static VECTORIZE_TWO_UNKNOWN: ErrorTypes::Message = ErrorTypes::Message { id: 194, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Could not vectorize call with unknown dimensions due to finding two for-iterators: %s and %s.") } };

pub static FUNCTION_SLOT_VARIABILITY: ErrorTypes::Message = ErrorTypes::Message { id: 195, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function argument %s=%s in call to %s has variability %s which is not a %s expression.") } };

pub static INVALID_ARRAY_DIM_IN_CONVERSION_OP: ErrorTypes::Message = ErrorTypes::Message { id: 196, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid dimension %s of argument to %s, expected dimension size %s but got %s.") } };

pub static DUPLICATE_REDECLARATION: ErrorTypes::Message = ErrorTypes::Message { id: 197, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is already redeclared in this scope.") } };

pub static INVALID_FUNCTION_VAR_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 198, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid type %s for function component %s.") } };

pub static IMBALANCED_EQUATIONS: ErrorTypes::Message = ErrorTypes::Message { id: 199, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("An independent subset of the model has imbalanced number of equations (%s) and variables (%s).\nvariables:\n%s\nequations:\n%s") } };

pub static EQUATIONS_VAR_NOT_DEFINED: ErrorTypes::Message = ErrorTypes::Message { id: 200, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable %s is not referenced in any equation (possibly after symbolic manipulations).") } };

pub static NON_FORMAL_PUBLIC_FUNCTION_VAR: ErrorTypes::Message = ErrorTypes::Message { id: 201, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid public variable %s, function variables that are not input/output must be protected.") } };

pub static PROTECTED_FORMAL_FUNCTION_VAR: ErrorTypes::Message = ErrorTypes::Message { id: 202, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid protected variable %s, function variables that are input/output must be public.") } };

pub static UNFILLED_SLOT: ErrorTypes::Message = ErrorTypes::Message { id: 203, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function parameter %s was not given by the function call, and does not have a default value.") } };

pub static SAME_CONNECT_INSTANCE: ErrorTypes::Message = ErrorTypes::Message { id: 204, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("connect(%s, %s) connects the same connector instance! The connect equation will be ignored.") } };

pub static STACK_OVERFLOW: ErrorTypes::Message = ErrorTypes::Message { id: 205, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Stack overflow occurred while evaluating %s.") } };

pub static UNKNOWN_DEBUG_FLAG: ErrorTypes::Message = ErrorTypes::Message { id: 206, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unknown debug flag %s.") } };

pub static INVALID_FLAG_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 207, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid type of flag %s, expected %s but got %s.") } };

pub static CHANGED_STD_VERSION: ErrorTypes::Message = ErrorTypes::Message { id: 208, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Modelica language version set to %s due to loading of MSL %s.") } };

pub static SIMPLIFY_FIXPOINT_MAXIMUM: ErrorTypes::Message = ErrorTypes::Message { id: 209, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expression simplification iterated to the fix-point maximum, which may be a performance bottleneck. The last two iterations were: %s, and %s.") } };

pub static UNKNOWN_OPTION: ErrorTypes::Message = ErrorTypes::Message { id: 210, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unknown option %s.") } };

pub static SUBSCRIPTED_MODIFIER: ErrorTypes::Message = ErrorTypes::Message { id: 211, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Subscripted modifier is illegal.") } };

pub static TRANS_VIOLATION: ErrorTypes::Message = ErrorTypes::Message { id: 212, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class specialization violation: %s is a %s, which may not contain an %s.") } };

pub static INSERT_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 213, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to insert class %s %s the available classes were:%s") } };

pub static MISSING_MODIFIED_ELEMENT: ErrorTypes::Message = ErrorTypes::Message { id: 214, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Modified element %s not found in class %s.") } };

pub static INVALID_REDECLARE_IN_BASIC_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 215, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid redeclaration of %s, attributes of basic types may not be redeclared.") } };

pub static MISMATCHED_FLOW_IN_STREAM_CONNECTOR: ErrorTypes::Message = ErrorTypes::Message { id: 216, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid stream connector %s: A stream connector must have exactly one flow variable, this connector has %s flow variables.") } };

pub static CONDITION_TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 217, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in condition '%s' of component %s. Expected a Boolean expression, but got an expression of type %s.") } };

pub static SIMPLIFY_CONSTANT_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 218, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("The compiler failed to perform constant folding on expression %s. Please report this bug to the developers and we will fix it as soon as possible (using the +t compiler option if possible).") } };

pub static SUM_EXPECTED_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 219, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("In sum(%s), the expression is of type %s, but is required to be of builtin array type (of any number of dimensions).") } };

pub static INVALID_CLASS_RESTRICTION: ErrorTypes::Message = ErrorTypes::Message { id: 220, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid specialized class type '%s' for component %s.") } };

pub static CONNECT_IN_INITIAL_EQUATION: ErrorTypes::Message = ErrorTypes::Message { id: 221, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Connect equations are not allowed in initial equation sections.") } };

pub static FINAL_COMPONENT_OVERRIDE: ErrorTypes::Message = ErrorTypes::Message { id: 222, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Trying to override final element %s with modifier '%s'.") } };

pub static NOTIFY_LOAD_MODEL_DUE_TO_USES: ErrorTypes::Message = ErrorTypes::Message { id: 223, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Automatically loaded package %s %s due to uses annotation from %s.") } };

pub static REINIT_MUST_BE_REAL: ErrorTypes::Message = ErrorTypes::Message { id: 224, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument to reinit must be a subtype of Real, but %s has type %s.") } };

pub static REINIT_MUST_BE_VAR: ErrorTypes::Message = ErrorTypes::Message { id: 225, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument to reinit must be a continuous time variable, but %s is %s.") } };

pub static CONNECT_TWO_SOURCES: ErrorTypes::Message = ErrorTypes::Message { id: 226, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Connecting two signal sources while connecting %s to %s.") } };

pub static INNER_OUTER_FORMAL_PARAMETER: ErrorTypes::Message = ErrorTypes::Message { id: 227, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid prefix %s on formal parameter %s.") } };

pub static REDECLARE_NONEXISTING_ELEMENT: ErrorTypes::Message = ErrorTypes::Message { id: 228, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal redeclare of element %s, no inherited element with that name exists.") } };

pub static INVALID_ARGUMENT_TYPE_FIRST_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 229, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument of %s must be an array expression.") } };

pub static INVALID_ARGUMENT_TYPE_BRANCH_FIRST: ErrorTypes::Message = ErrorTypes::Message { id: 230, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument '%s' of %s must have the form A.R, where A is a connector and R an over-determined type/record.") } };

pub static INVALID_ARGUMENT_TYPE_BRANCH_SECOND: ErrorTypes::Message = ErrorTypes::Message { id: 231, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The second argument '%s' of %s must have the form A.R, where A is a connector and R an over-determined type/record.") } };

pub static INVALID_ARGUMENT_TYPE_OVERDET_FIRST: ErrorTypes::Message = ErrorTypes::Message { id: 232, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument of %s must be an over-determined type or record.") } };

pub static INVALID_ARGUMENT_TYPE_OVERDET_SECOND: ErrorTypes::Message = ErrorTypes::Message { id: 233, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The second argument of %s must be an over-determined type or record.") } };

pub static LIBRARY_ONE_PACKAGE_PER_FILE: ErrorTypes::Message = ErrorTypes::Message { id: 234, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Modelica library files should contain exactly one package, but found the following classes: %s.") } };

pub static LIBRARY_UNEXPECTED_WITHIN: ErrorTypes::Message = ErrorTypes::Message { id: 235, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expected the package to have %s but got %s.") } };

pub static LIBRARY_UNEXPECTED_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 236, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expected the package to have name %s, but got %s.") } };

pub static PACKAGE_MO_NOT_IN_ORDER: ErrorTypes::Message = ErrorTypes::Message { id: 237, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Elements in the package.mo-file need to be in the same relative order as the package.order file. Got element named %s but it was already added because it was not the next element in the list at that time.") } };

pub static LIBRARY_EXPECTED_PARTS: ErrorTypes::Message = ErrorTypes::Message { id: 238, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is a package.mo-file and needs to be based on class parts (i.e. not class extends, derived class, or enumeration).") } };

pub static PACKAGE_ORDER_FILE_NOT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 239, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("%1 was referenced in the package.order file, but was not found in package.mo, %1/package.mo or %1.mo.") } };

pub static FOUND_ELEMENT_NOT_IN_ORDER_FILE: ErrorTypes::Message = ErrorTypes::Message { id: 240, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Got element %1 that was not referenced in the package.order file.") } };

pub static ORDER_FILE_COMPONENTS: ErrorTypes::Message = ErrorTypes::Message { id: 241, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Components referenced in the package.order file must be moved in full chunks. Either split the constants to different lines or make them subsequent in the package.order file.") } };

pub static GUARD_EXPRESSION_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 242, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Guard expressions need to be Boolean, got expression of type %s.") } };

pub static FUNCTION_RETURNS_META_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 243, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("User-defined function calls that return Array<...> are not supported: %s.") } };

pub static ASSIGN_UNKNOWN_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 244, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to elaborate assignment for some unknown reason: %1 := %2. File a bug report and we will make sure this error gets a better message in the future.") } };

pub static WARNING_DEF_USE: ErrorTypes::Message = ErrorTypes::Message { id: 245, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s was used before it was defined (given a value). Additional such uses may exist for the variable, but some messages were suppressed.") } };

pub static EXP_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 246, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expression '%1' has type %3, expected type %2.") } };

pub static PACKAGE_ORDER_DUPLICATES: ErrorTypes::Message = ErrorTypes::Message { id: 247, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found duplicate names in package.order file: %s.") } };

pub static ERRONEOUS_TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 248, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Got type mismatch error, but matching types %s.\nThis is a ***COMPILER BUG***, please report it to https://trac.openmodelica.org/OpenModelica.") } };

pub static REINIT_MUST_BE_VAR_OR_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 249, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument to reinit must be a variable of type Real or an array of such variables.") } };

pub static SLICE_ASSIGN_NON_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 250, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot assign slice to non-initialized array %s.") } };

pub static EXTERNAL_ARG_WRONG_EXP: ErrorTypes::Message = ErrorTypes::Message { id: 251, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expression %s cannot be an external argument. Only identifiers, scalar constants, and size-expressions are allowed.") } };

pub static OPERATOR_FUNCTION_NOT_EXPECTED: ErrorTypes::Message = ErrorTypes::Message { id: 252, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Only classes of type 'operator record' may contain elements of type 'operator function'; %s was found in a class that has restriction '%s'.") } };

pub static OPERATOR_FUNCTION_EXPECTED: ErrorTypes::Message = ErrorTypes::Message { id: 253, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'operator record' classes may only contain elements of type 'operator function'; %s has restriction '%s'.") } };

pub static STRUCTURAL_SINGULAR_INITIAL_SYSTEM: ErrorTypes::Message = ErrorTypes::Message { id: 254, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Initialization problem is structurally singular, error found sorting equations\n %s for variables\n %s") } };

pub static UNFIXED_PARAMETER_WITH_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 255, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The parameter %s has fixed = false and a binding equation %s = %s, which is probably redundant.\nSetting fixed = false usually means there is an additional initial equation to determine the parameter value. The binding was ignored by old Modelica tools, but this is not according to the Modelica specification. Please remove the parameter binding, or bind the parameter to another parameter with fixed = false and no binding.") } };

pub static UNFIXED_PARAMETER_WITH_BINDING_31: ErrorTypes::Message = ErrorTypes::Message { id: 256, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The parameter %s has fixed = false and a binding equation %s = %s, which is probably redundant. The binding equation will be ignored, as it is expected for Modelica 3.1.") } };

pub static UNFIXED_PARAMETER_WITH_BINDING_AND_START_VALUE_31: ErrorTypes::Message = ErrorTypes::Message { id: 257, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The parameter %s has fixed = false, a start value, start = %s and a binding equation %s = %s, which is probably redundant. The binding equation will be ignored, as it is expected for Modelica 3.1.") } };

pub static BACKENDDAEINFO_LOWER: ErrorTypes::Message = ErrorTypes::Message { id: 258, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Model statistics after passing the front-end and creating the data structures used by the back-end:\n * Number of equations: %s\n * Number of variables: %s") } };

pub static BACKENDDAEINFO_STATISTICS: ErrorTypes::Message = ErrorTypes::Message { id: 259, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Model statistics after passing the back-end for %s:\n * Number of independent subsystems: %s\n * Number of states: %s\n * Number of discrete variables: %s\n * Number of discrete states: %s\n * Number of clocked states: %s\n * Top-level inputs: %s") } };

pub static BACKENDDAEINFO_MIXED: ErrorTypes::Message = ErrorTypes::Message { id: 260, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Mixed equation statistics:\n * Mixed systems with single equation: %s\n * Mixed systems with array equation: %s\n * Mixed systems with algorithm: %s\n * Mixed systems with complex equation: %s\n * Mixed systems with constant Jacobian: %s\n * Mixed systems with linear Jacobian: %s\n * Mixed systems with non-linear Jacobian: %s\n * Mixed systems with analytic Jacobian: %s\n * Mixed systems with linear tearing system: %s\n * Mixed systems with nonlinear tearing system: %s") } };

pub static BACKENDDAEINFO_STRONGCOMPONENT_STATISTICS: ErrorTypes::Message = ErrorTypes::Message { id: 261, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Strong component statistics for %s (%s):\n * Single equations (assignments): %s\n * Array equations: %s\n * Algorithm blocks: %s\n * Record equations: %s\n * When equations: %s\n * If-equations: %s\n * Equation systems (not torn): %s\n * Torn equation systems: %s\n * Mixed (continuous/discrete) equation systems: %s") } };

pub static BACKENDDAEINFO_SYSTEMS: ErrorTypes::Message = ErrorTypes::Message { id: 262, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Equation system details (not torn):\n * Constant Jacobian (size): %s\n * Linear Jacobian (size,density): %s\n * Non-linear Jacobian (size): %s\n * Without analytic Jacobian (size): %s") } };

pub static BACKENDDAEINFO_TORN: ErrorTypes::Message = ErrorTypes::Message { id: 263, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Torn system details for %s tearing set:\n * Linear torn systems (#iteration vars, #inner vars, density): %s\n * Non-linear torn systems (#iteration vars, #inner vars): %s") } };

pub static BACKEND_DAE_TO_MODELICA: ErrorTypes::Message = ErrorTypes::Message { id: 264, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("The following Modelica-like model represents the back-end DAE for the '%s' stage:\n%s") } };

pub static NEGATIVE_DIMENSION_INDEX: ErrorTypes::Message = ErrorTypes::Message { id: 265, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Negative dimension index (%s) for component %s.") } };

pub static GENERATE_SEPARATE_CODE_DEPENDENCIES_FAILED: ErrorTypes::Message = ErrorTypes::Message { id: 266, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to get dependencies for package %s. Perhaps there is an import to a non-existing package.") } };

pub static CYCLIC_DEFAULT_VALUE: ErrorTypes::Message = ErrorTypes::Message { id: 267, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The default value of %s causes a cyclic dependency.") } };

pub static NAMED_ARG_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 268, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch for named argument in %s(%s=%s). The argument has type:\n  %s\nexpected type:\n  %s") } };

pub static ARG_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 269, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch for positional argument %s in %s(%s=%s). The argument has type:\n  %s\nexpected type:\n  %s") } };

pub static OP_OVERLOAD_MULTIPLE_VALID: ErrorTypes::Message = ErrorTypes::Message { id: 270, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operator overloading requires exactly one matching expression, but found %s expressions: %s") } };

pub static OP_OVERLOAD_OPERATOR_NOT_INPUT: ErrorTypes::Message = ErrorTypes::Message { id: 271, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operator %s is not an input to the overloaded function: %s") } };

pub static NOTIFY_FRONTEND_STRUCTURAL_PARAMETERS: ErrorTypes::Message = ErrorTypes::Message { id: 272, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("The following structural parameters were evaluated in the front-end: %s\nStructural parameters are parameters used to calculate array dimensions or branch selection in certain if-equations or if-expressions among other things.") } };

pub static SIMPLIFICATION_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 273, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expression simplification '%s' → '%s' changed the type from %s to %s.") } };

pub static VECTORIZE_CALL_DIM_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 274, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to vectorize function call because arguments %s=%s and %s=%s have mismatched dimensions %s and %s.") } };

pub static TCOMPLEX_MULTIPLE_NAMES: ErrorTypes::Message = ErrorTypes::Message { id: 275, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Non-tuple complex type specifiers need to have exactly one type name: %s.") } };

pub static TCOMPLEX_TUPLE_ONE_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 276, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Tuple complex type specifiers need to have more than one type name: %s.") } };

pub static ENUM_DUPLICATES: ErrorTypes::Message = ErrorTypes::Message { id: 277, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Enumeration has duplicate names: %s in list of names %s.") } };

pub static RESERVED_IDENTIFIER: ErrorTypes::Message = ErrorTypes::Message { id: 278, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Identifier %s is reserved for the built-in element with the same name.") } };

pub static NOTIFY_PKG_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 279, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("You can install the requested package using one of the commands:\n%s.") } };

pub static DERIVATIVE_FUNCTION_CONTEXT: ErrorTypes::Message = ErrorTypes::Message { id: 280, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The der() operator is not allowed in function context (possible solutions: pass the derivative as an explicit input; use a block instead of function).") } };

pub static RETURN_OUTSIDE_FUNCTION: ErrorTypes::Message = ErrorTypes::Message { id: 281, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'return' may not be used outside function.") } };

pub static EXT_LIBRARY_NOT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 282, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Could not find library %s in either of:%s") } };

pub static EXT_LIBRARY_NOT_FOUND_DESPITE_COMPILATION_SUCCESS: ErrorTypes::Message = ErrorTypes::Message { id: 283, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Could not find library %s despite compilation command %s in directory %s returning success.") } };

pub static GENERATE_SEPARATE_CODE_DEPENDENCIES_FAILED_UNKNOWN_PACKAGE: ErrorTypes::Message = ErrorTypes::Message { id: 284, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to get dependencies for package %s. %s contains an import to non-existing package %s.") } };

pub static USE_OF_PARTIAL_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 285, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("component %s contains the definition of a partial class %s.\nPlease redeclare it to any package compatible with %s.") } };

pub static SCANNER_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 286, ty: crate::ErrorTypes::MessageType::SYNTAX, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Syntax error, unrecognized input: %s.") } };

pub static SCANNER_ERROR_LIMIT: ErrorTypes::Message = ErrorTypes::Message { id: 287, ty: crate::ErrorTypes::MessageType::SYNTAX, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Additional syntax errors were suppressed.") } };

pub static INVALID_TIME_SCOPE: ErrorTypes::Message = ErrorTypes::Message { id: 288, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Built-in variable 'time' may only be used in a model or block.") } };

pub static NO_JACONIAN_TORNLINEAR_SYSTEM: ErrorTypes::Message = ErrorTypes::Message { id: 289, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("A torn linear system has no symbolic jacobian and currently there are no means to solve that numerically. Please compile with the module \"calculateStrongComponentJacobians\" to provide symbolic jacobians for torn linear systems.") } };

pub static EXT_FN_SINGLE_RETURN_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 290, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("An external declaration with a single output without explicit mapping is defined as having the output as the lhs, but language %s does not support this for array variables. OpenModelica will put the output as an input (as is done when there is more than 1 output), but this is not according to the Modelica Specification. Use an explicit mapping instead of the implicit one to suppress this warning.") } };

pub static RHS_TUPLE_EXPRESSION: ErrorTypes::Message = ErrorTypes::Message { id: 291, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Tuple expressions may only occur on the left side of an assignment or equation with a single function call on the right side. Got the following expression: %s.") } };

pub static EACH_ON_NON_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 292, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("'each' used when modifying non-array element %s.") } };

pub static BUILTIN_EXTENDS_INVALID_ELEMENTS: ErrorTypes::Message = ErrorTypes::Message { id: 293, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("A class extending from builtin type %s may not have other elements.") } };

pub static INITIAL_CALL_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 294, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("initial() may only be used as a when condition (when initial() or when {..., initial(), ...}), but got condition '%s'.") } };

pub static RANGE_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 295, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in range: '%s' of type\n  %s\nis not type compatible with '%s' of type\n  %s") } };

pub static RANGE_TOO_SMALL_STEP: ErrorTypes::Message = ErrorTypes::Message { id: 296, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Step size %s in range is too small.") } };

pub static RANGE_INVALID_STEP: ErrorTypes::Message = ErrorTypes::Message { id: 297, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Range of type %s may not specify a step size.") } };

pub static RANGE_INVALID_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 298, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Range has invalid type %s.") } };

pub static CLASS_EXTENDS_MISSING_REDECLARE: ErrorTypes::Message = ErrorTypes::Message { id: 299, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Missing redeclare prefix on class extends %s, treating like redeclare anyway.") } };

pub static CYCLIC_DIMENSIONS: ErrorTypes::Message = ErrorTypes::Message { id: 300, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Dimension %s of %s, '%s', could not be evaluated due to a cyclic dependency.") } };

pub static INVALID_DIMENSION_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 301, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Dimension '%s' of type %s is not an integer expression or an enumeration or Boolean type name.") } };

pub static NON_PARAMETER_EXPRESSION_DIMENSION: ErrorTypes::Message = ErrorTypes::Message { id: 302, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expression '%s' that determines the size of dimension '%s' of '%s' is not an evaluable parameter expression.") } };

pub static INVALID_TYPENAME_USE: ErrorTypes::Message = ErrorTypes::Message { id: 303, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type name '%s' is not allowed in this context.") } };

pub static FOUND_WRONG_INNER_ELEMENT: ErrorTypes::Message = ErrorTypes::Message { id: 305, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found inner %s %s instead of expected %s.") } };

pub static FOUND_OTHER_BASECLASS: ErrorTypes::Message = ErrorTypes::Message { id: 306, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found other base class for extends %s after instantiating extends.") } };

pub static OUTER_ELEMENT_MOD: ErrorTypes::Message = ErrorTypes::Message { id: 307, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Modifier '%s' found on outer element %s.") } };

pub static OUTER_LONG_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 308, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Illegal outer class %s, outer classes may only be declared using short-class definitions.") } };

pub static MISSING_INNER_ADDED: ErrorTypes::Message = ErrorTypes::Message { id: 309, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("An inner declaration for outer %s %s could not be found and was automatically generated.") } };

pub static MISSING_INNER_MESSAGE: ErrorTypes::Message = ErrorTypes::Message { id: 310, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("The diagnostics message for the missing inner is: %s") } };

pub static INVALID_CONNECTOR_FORM: ErrorTypes::Message = ErrorTypes::Message { id: 311, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is not a valid form for a connector, connectors must be either c1.c2...cn or m.c (where c is a connector and m is a non-connector).") } };

pub static CONNECTOR_PREFIX_OUTSIDE_CONNECTOR: ErrorTypes::Message = ErrorTypes::Message { id: 312, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Prefix '%s' used outside connector declaration.") } };

pub static EXTERNAL_OBJECT_INVALID_ELEMENT: ErrorTypes::Message = ErrorTypes::Message { id: 313, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("External object %s contains invalid element '%s'.") } };

pub static EXTERNAL_OBJECT_MISSING_STRUCTOR: ErrorTypes::Message = ErrorTypes::Message { id: 314, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("External object %s is missing a %s.") } };

pub static MULTIPLE_SECTIONS_IN_FUNCTION: ErrorTypes::Message = ErrorTypes::Message { id: 315, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function %s has more than one algorithm section or external declaration.") } };

pub static INVALID_EXTERNAL_LANGUAGE: ErrorTypes::Message = ErrorTypes::Message { id: 316, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%s' is not a valid language for an external function.") } };

pub static SUBSCRIPT_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 317, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Subscript '%s' has type %s, expected type %s.") } };

pub static EXP_INVALID_IN_FUNCTION: ErrorTypes::Message = ErrorTypes::Message { id: 318, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is not allowed in a function.") } };

pub static NO_MATCHING_FUNCTION_FOUND_NFINST: ErrorTypes::Message = ErrorTypes::Message { id: 319, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("No matching function found for %s.\nCandidates are:\n  %s") } };

pub static ARGUMENT_OUT_OF_RANGE: ErrorTypes::Message = ErrorTypes::Message { id: 320, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Argument %s of %s is out of range (%s)") } };

pub static UNBOUND_CONSTANT: ErrorTypes::Message = ErrorTypes::Message { id: 321, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Constant %s is used without having been given a value.") } };

pub static INVALID_ARGUMENT_VARIABILITY: ErrorTypes::Message = ErrorTypes::Message { id: 322, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Argument %s of %s must be a %s expression, but %s is %s.") } };

pub static AMBIGUOUS_MATCHING_FUNCTIONS_NFINST: ErrorTypes::Message = ErrorTypes::Message { id: 323, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Ambiguous matching functions found for %s.\nCandidates are:\n  %s") } };

pub static AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST: ErrorTypes::Message = ErrorTypes::Message { id: 324, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Ambiguous matching overloaded operator functions found for %s.\nCandidates are:\n  %s") } };

pub static REDECLARE_CONDITION: ErrorTypes::Message = ErrorTypes::Message { id: 325, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid redeclaration of %s, a redeclare may not have a condition attribute.") } };

pub static REDECLARE_OF_CONSTANT: ErrorTypes::Message = ErrorTypes::Message { id: 326, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is constant and may not be redeclared.") } };

pub static REDECLARE_MISMATCHED_PREFIX: ErrorTypes::Message = ErrorTypes::Message { id: 327, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid redeclaration '%s %s', original element is declared '%s'.") } };

pub static EXTERNAL_ARG_NONCONSTANT_SIZE_INDEX: ErrorTypes::Message = ErrorTypes::Message { id: 328, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid external argument '%s', the dimension index must be a constant expression.") } };

pub static FAILURE_TO_DEDUCE_DIMS_EACH: ErrorTypes::Message = ErrorTypes::Message { id: 329, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to deduce dimension %s of '%s' due to 'each' prefix on binding equation.") } };

pub static MISSING_TYPE_BASETYPE: ErrorTypes::Message = ErrorTypes::Message { id: 330, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type '%s' does not extend a basic type.") } };

pub static ASSERT_TRIGGERED_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 331, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("assert triggered: %s") } };

pub static ASSERT_TRIGGERED_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 332, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("assert triggered: %s") } };

pub static TERMINATE_TRIGGERED: ErrorTypes::Message = ErrorTypes::Message { id: 333, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("terminate triggered: %s") } };

pub static EVAL_RECURSION_LIMIT_REACHED: ErrorTypes::Message = ErrorTypes::Message { id: 334, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The recursion limit (--evalRecursionLimit=%s) was exceeded during evaluation of %s.") } };

pub static UNASSIGNED_FUNCTION_OUTPUT: ErrorTypes::Message = ErrorTypes::Message { id: 335, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Output parameter %s was not assigned a value") } };

pub static INVALID_WHEN_STATEMENT_CONTEXT: ErrorTypes::Message = ErrorTypes::Message { id: 336, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("A when-statement may not be used inside a function or a while, if, or for-clause.") } };

pub static MISSING_FUNCTION_DERIVATIVE_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 337, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Derivative annotation for function '%s' does not specify a derivative function.") } };

pub static INVALID_FUNCTION_ANNOTATION_ATTR: ErrorTypes::Message = ErrorTypes::Message { id: 338, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%s' is not a valid function %s attribute.") } };

pub static INVALID_FUNCTION_ANNOTATION_INPUT: ErrorTypes::Message = ErrorTypes::Message { id: 339, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%s' is not an input of function '%s'.") } };

pub static OPERATOR_OVERLOADING_ONE_OUTPUT_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 340, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operator %s must have exactly one output.") } };

pub static OPERATOR_OVERLOADING_INVALID_OUTPUT_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 341, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Output '%s' in operator %s must be of type %s, got type %s.") } };

pub static OPERATOR_NOT_ENCAPSULATED: ErrorTypes::Message = ErrorTypes::Message { id: 342, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operator %s is not encapsulated.") } };

pub static NO_SUCH_INPUT_PARAMETER: ErrorTypes::Message = ErrorTypes::Message { id: 343, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function %s has no input parameter named %s.") } };

pub static INVALID_REDUCTION_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 344, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid expression '%s' of type %s in %s reduction, expected %s.") } };

pub static INVALID_COMPONENT_PREFIX: ErrorTypes::Message = ErrorTypes::Message { id: 345, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Prefix '%s' on component '%s' not allowed in class specialization '%s'.") } };

pub static INVALID_CARDINALITY_CONTEXT: ErrorTypes::Message = ErrorTypes::Message { id: 346, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("cardinality may only be used in the condition of an if-statement/equation or an assert.") } };

pub static VARIABLE_BINDING_DIMS_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 347, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in binding '%s = %s', expected array dimensions %s, got %s.") } };

pub static MODIFIER_NON_ARRAY_TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 348, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Non-array modification '%s' for array component '%s', possibly due to missing 'each'.") } };

pub static INST_RECURSION_LIMIT_REACHED: ErrorTypes::Message = ErrorTypes::Message { id: 349, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Recursion limit reached while instantiating '%s'.") } };

pub static WHEN_IF_VARIABLE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 350, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The branches of an if-equation inside a when-equation must have the same set of component references on the left-hand side.") } };

pub static DIMENSION_DEDUCTION_FROM_BINDING_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 351, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Dimension %s of '%s' could not be deduced from the component's binding equation '%s'.") } };

pub static NON_REAL_FLOW_OR_STREAM: ErrorTypes::Message = ErrorTypes::Message { id: 352, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid prefix '%s' on non-Real component '%s'.") } };

pub static LIBRARY_UNEXPECTED_NAME_CASE_SENSITIVE: ErrorTypes::Message = ErrorTypes::Message { id: 353, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expected the package to have name %s, but got %s. Proceeding since only the case of the names are different.") } };

pub static PACKAGE_ORDER_CASE_SENSITIVE: ErrorTypes::Message = ErrorTypes::Message { id: 354, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The package.order file contains a class %s, which is expected to be stored in file %s, but seems to be named %s. Proceeding since only the case of the names are different.") } };

pub static REDECLARE_CLASS_NON_SUBTYPE: ErrorTypes::Message = ErrorTypes::Message { id: 355, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Redeclaration of %s '%s' is not a subtype of the redeclared element.") } };

pub static REDECLARE_ENUM_NON_SUBTYPE: ErrorTypes::Message = ErrorTypes::Message { id: 356, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Redeclaration of enumeration '%s' is not a subtype of the redeclared element (use enumeration(:) for a generic replaceable enumeration).") } };

pub static CONDITIONAL_COMPONENT_INVALID_CONTEXT: ErrorTypes::Message = ErrorTypes::Message { id: 357, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Conditional component '%s' is used in a non-connect context.") } };

pub static OPERATOR_RECORD_MISSING_OPERATOR: ErrorTypes::Message = ErrorTypes::Message { id: 358, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type '%s' of expression '%s' in '%s' does not implement the required operator '%s'") } };

pub static IMPORT_IN_COMPOSITE_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 359, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found imported name '%s' while looking up composite name '%s'.") } };

pub static SHADOWED_ITERATOR: ErrorTypes::Message = ErrorTypes::Message { id: 360, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("An iterator named '%s' is already declared in this scope.") } };

pub static W_INVALID_ARGUMENT_TYPE_BRANCH_FIRST: ErrorTypes::Message = ErrorTypes::Message { id: 361, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument '%s' of %s must have the form A.R, where A is a connector and R an over-determined type/record.") } };

pub static W_INVALID_ARGUMENT_TYPE_BRANCH_SECOND: ErrorTypes::Message = ErrorTypes::Message { id: 362, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The second argument '%s' of %s must have the form A.R, where A is a connector and R an over-determined type/record.") } };

pub static LIBRARY_WITHIN_WRONG_CASE: ErrorTypes::Message = ErrorTypes::Message { id: 363, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expected the package to have %s but got %s (ignoring the potential error; the class might have been inserted at an unexpected location).") } };

pub static INVALID_FLAG_CONDITION: ErrorTypes::Message = ErrorTypes::Message { id: 364, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Flag %s ignored: %s.") } };

pub static EXPERIMENTAL_REQUIRED: ErrorTypes::Message = ErrorTypes::Message { id: 365, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is an experimental feature and requires the --std=experimental flag.") } };

pub static INVALID_NUMBER_OF_DIMENSIONS_FOR_PROMOTE: ErrorTypes::Message = ErrorTypes::Message { id: 366, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The second argument '%s' of promote may not be smaller than the number of dimensions (%s) of the first argument.") } };

pub static PURE_FUNCTION_WITH_IMPURE_CALLS: ErrorTypes::Message = ErrorTypes::Message { id: 367, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Pure function '%s' contains a call to impure function '%s'.") } };

pub static DISCRETE_REAL_UNDEFINED: ErrorTypes::Message = ErrorTypes::Message { id: 368, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Following variable is discrete, but does not appear on the LHS of a when-statement: '%s'.") } };

pub static DER_OF_NONDIFFERENTIABLE_EXP: ErrorTypes::Message = ErrorTypes::Message { id: 369, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Argument '%s' of der is not differentiable.") } };

pub static LOAD_MODEL_DIFFERENT_VERSIONS_WITHOUT_CONVERSION: ErrorTypes::Message = ErrorTypes::Message { id: 370, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("%1 requested package %2 of version %3. %2 %4 is used instead which states that it is fully compatible without conversion script needed.") } };

// The following errors (371, 372, 373) are used by OMEdit. Do not change them.
pub static LOAD_MODEL_DIFFERENT_VERSIONS_WITH_CONVERSION: ErrorTypes::Message = ErrorTypes::Message { id: 371, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("%1 requested package %2 of version %3. %2 %4 is used instead which states that it is only compatible with a conversion script. Use convertPackageToLibrary(%1, %2, \"%4\") to run the conversion script or proceed with potential issues as a result.") } };

pub static LOAD_MODEL_DIFFERENT_VERSIONS_OLDER: ErrorTypes::Message = ErrorTypes::Message { id: 372, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Requested package %1 of version %2, but this package was already loaded with version %3. There are no conversion annotations and %2 is older than %3, so the libraries are probably incompatible.") } };

pub static LOAD_MODEL_DIFFERENT_VERSIONS_NEWER: ErrorTypes::Message = ErrorTypes::Message { id: 373, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Requested package %1 of version %2, but this package was already loaded with version %3. There are no conversion annotations for this version but %2 is newer than %3. There is a possibility that %2 remains backwards compatible, but it is not loaded so OpenModelica cannot verify this.") } };

pub static EQUATION_NOT_SOLVABLE_DIFFERENT_COUNT: ErrorTypes::Message = ErrorTypes::Message { id: 374, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s has size %s but %s variables (%s)") } };

pub static PARTIAL_COMPONENT_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 375, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component '%s' has partial type '%s'.") } };

pub static PARTIAL_FUNCTION_CALL: ErrorTypes::Message = ErrorTypes::Message { id: 376, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Called function '%s' is partial.") } };

pub static TOO_MANY_TYPE_VARS_IN_CALL: ErrorTypes::Message = ErrorTypes::Message { id: 377, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Too many type variables given in call to '%s'.") } };

pub static BREAK_OUTSIDE_LOOP: ErrorTypes::Message = ErrorTypes::Message { id: 378, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'break' may only be used in a while- or for-loop.") } };

pub static TOP_LEVEL_OUTER: ErrorTypes::Message = ErrorTypes::Message { id: 379, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The model can't be instantiated due to top-level outer element '%s', it may only be used as part of a simulation model.") } };

pub static MISSING_INNER_NAME_CONFLICT: ErrorTypes::Message = ErrorTypes::Message { id: 380, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("An inner declaration for outer element '%s' could not be found, and could not be automatically generated due to an existing declaration of that name.") } };

pub static TOP_LEVEL_INPUT_WITH_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 381, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Top-level input '%s' has a binding equation and will not be accessible as an input of the model.") } };

pub static NON_DISCRETE_WHEN_CONDITION: ErrorTypes::Message = ErrorTypes::Message { id: 382, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("When-condition '%s' is not a discrete-time expression.") } };

pub static CYCLIC_FUNCTION_COMPONENTS: ErrorTypes::Message = ErrorTypes::Message { id: 383, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cyclically dependent function components found: %s") } };

pub static EXTERNAL_FUNCTION_NOT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 384, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("External function '%s' could not be found in any of the given shared libraries:\n%s") } };

pub static INVALID_CONVERSION_RULE: ErrorTypes::Message = ErrorTypes::Message { id: 385, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid conversion rule '%s'.") } };

pub static CONVERSION_MESSAGE: ErrorTypes::Message = ErrorTypes::Message { id: 386, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s") } };

pub static CONVERSION_MISMATCHED_PLACEHOLDER: ErrorTypes::Message = ErrorTypes::Message { id: 387, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Mismatched % in conversion modifier '%s'.") } };

pub static CONVERSION_MISSING_PLACEHOLDER_VALUE: ErrorTypes::Message = ErrorTypes::Message { id: 388, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("No replacement value for placeholder '%s' could be found.") } };

pub static UNSUPPORTED_RECORD_REORDERING: ErrorTypes::Message = ErrorTypes::Message { id: 389, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The record constructor for '%s' requires reordering of local fields to initialize them in the correct order, which is not yet supported.") } };

pub static FUNCTION_INVALID_OUTPUTS_FOR_INVERSE: ErrorTypes::Message = ErrorTypes::Message { id: 390, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid inverse annotation for '%s', only functions with exactly one output may have an inverse.") } };

pub static NOTIFY_IMPLICIT_LOAD: ErrorTypes::Message = ErrorTypes::Message { id: 391, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Automatically loaded package %s %s due to usage.") } };

pub static CONVERSION_MISSING_USES: ErrorTypes::Message = ErrorTypes::Message { id: 392, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot convert '%s' since it has no uses-annotation for '%s'.") } };

pub static CONVERSION_NO_COMPATIBLE_SCRIPT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 393, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("No compatible conversion script for converting from %s %s to %s could be found.") } };

pub static FUNCTION_CALL_EXPRESSION: Gettext::TranslatableContent = Gettext::TranslatableContent::gettext { msgid: literal!("a function call expression") };

pub static FUNCTION_ARGUMENT_MUST_BE: ErrorTypes::Message = ErrorTypes::Message { id: 394, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The argument to '%s' must be %s.") } };

pub static UNEXPECTED_COMPONENT_IN_COMPOSITE_NAME: ErrorTypes::Message = ErrorTypes::Message { id: 395, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found component '%s' in composite name '%s', expected class.") } };

pub static NF_MODIFY_PROTECTED: ErrorTypes::Message = ErrorTypes::Message { id: 396, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Protected element '%s' may not be modified, got '%s'.") } };

pub static PROTECTED_TRANSITION_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 397, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Protected sections are not allowed in %s.") } };

pub static DEPRECATED_TRANSITION_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 398, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s are deprecated in %s.") } };

pub static INITIAL_ALGORITHM_TRANSITION_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 399, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Initial algorithm sections are not allowed in %s.") } };

pub static INVALID_SPECIALIZATION_FOR_BINDING_EQUATION: ErrorTypes::Message = ErrorTypes::Message { id: 400, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component '%s' may not have a binding equation due to class specialization '%s'.") } };

pub static INVALID_SPECIALIZATION_IN_ASSIGNMENT: ErrorTypes::Message = ErrorTypes::Message { id: 401, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component '%s' may not be assigned to due to class specialization '%s'.") } };

pub static NF_PDE_NOT_IMPLEMENTED: ErrorTypes::Message = ErrorTypes::Message { id: 402, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("PDEModelica is not yet supported by the new front-end, using the old front-end instead.") } };

pub static NON_CONSTANT_IN_ENCLOSING_SCOPE: ErrorTypes::Message = ErrorTypes::Message { id: 403, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component '%s' was found in an enclosing scope but is not a constant.") } };

pub static PARTIAL_DERIVATIVE_INPUT_NOT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 404, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%s' in partial derivative of '%s' does not name an input parameter of the function.") } };

pub static PARTIAL_DERIVATIVE_INPUT_INVALID_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 405, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%s' in partial derivative of '%s' is not a scalar Real input parameter of the function.") } };

pub static CONNECT_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 406, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The connectors in connect(%s, %s) are not type compatible.") } };

pub static UNSPECIFIED_ENUM_COMPONENT: ErrorTypes::Message = ErrorTypes::Message { id: 407, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component '%s' has an unspecified enumeration type (enumeration(:)).") } };

pub static ELEMENT_REPLACEABLE_NOT_ALLOWED: ErrorTypes::Message = ErrorTypes::Message { id: 408, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%s' may not be replaceable.") } };

pub static INVALID_NEGATIVE_POW: ErrorTypes::Message = ErrorTypes::Message { id: 409, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid operation %s ^ %s, exponent must be an Integer when the base is negative.") } };

pub static DEPRECATED_EXPRESSION: ErrorTypes::Message = ErrorTypes::Message { id: 411, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%s' is deprecated, use '%s' instead.") } };

pub static UNKNOWN_ANNOTATION_VALUE: ErrorTypes::Message = ErrorTypes::Message { id: 412, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unknown value '%s' for annotation '%s'") } };

pub static NON_FIXED_CONSTANT: ErrorTypes::Message = ErrorTypes::Message { id: 413, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Constant '%s' must be fixed but has 'fixed = false'") } };

pub static CONFLICTING_INHERITED_ANNOTATIONS: ErrorTypes::Message = ErrorTypes::Message { id: 414, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Conflicting '%s' annotations inherited by class '%s':\n  %s from 'extends %s'\n  %s from 'extends %s'") } };

pub static ASSIGN_ITERATOR_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 415, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Assignment to iterator '%s'.") } };

pub static INVALID_CONNECTOR_VARIABILITY: ErrorTypes::Message = ErrorTypes::Message { id: 416, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid variability %s on connector '%s'.") } };

pub static NON_BREAKABLE_ELEMENT: ErrorTypes::Message = ErrorTypes::Message { id: 417, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid use of break on non-component '%s'.") } };

pub static NON_BREAKABLE_COMPONENT: ErrorTypes::Message = ErrorTypes::Message { id: 418, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid use of break on component '%s', component must be a model, block, or connector.") } };

pub static UNMATCHED_BREAK_CONNECT: ErrorTypes::Message = ErrorTypes::Message { id: 419, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("No matching element found for 'break connect(%s, %s)'.") } };

pub static INVALID_DELETED_COMPONENT_CONTEXT: ErrorTypes::Message = ErrorTypes::Message { id: 420, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%s' refers to a component with a false condition.") } };

pub static UNROLL_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 421, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("For loop could not be unrolled:\n%s") } };

pub static INITIALIZATION_NOT_FULLY_SPECIFIED: ErrorTypes::Message = ErrorTypes::Message { id: 496, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The initial conditions are not fully specified. %s.") } };

pub static INITIALIZATION_OVER_SPECIFIED: ErrorTypes::Message = ErrorTypes::Message { id: 497, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The initial conditions are over specified. %s.") } };

pub static INITIALIZATION_ITERATION_VARIABLES: ErrorTypes::Message = ErrorTypes::Message { id: 498, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("There are nonlinear iteration variables with default zero start attribute found in %s. %s.") } };

pub static UNBOUND_PARAMETER_WITH_START_VALUE_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 499, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Parameter %s has no value, and is fixed during initialization (fixed=true), using available start value (start=%s) as default value.") } };

pub static UNBOUND_PARAMETER_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 500, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Parameter %s has neither value nor start value, and is fixed during initialization (fixed=true).") } };

pub static BUILTIN_FUNCTION_PRODUCT_HAS_SCALAR_PARAMETER: ErrorTypes::Message = ErrorTypes::Message { id: 502, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function \"product\" has scalar as argument in %s in component %s.") } };

pub static SETTING_FIXED_ATTRIBUTE: ErrorTypes::Message = ErrorTypes::Message { id: 503, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Using over-determined solver for initialization. Setting fixed=false to the following variables: %s.") } };

pub static FAILED_TO_EVALUATE_FUNCTION: ErrorTypes::Message = ErrorTypes::Message { id: 506, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to evaluate function: %s.") } };

pub static WARNING_RELATION_ON_REAL: ErrorTypes::Message = ErrorTypes::Message { id: 509, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("In relation %s, %s on Real operands is deprecated in non-function contexts.") } };

pub static OUTER_MODIFICATION: ErrorTypes::Message = ErrorTypes::Message { id: 512, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Ignoring the modification on outer element: %s.") } };

pub static DERIVATIVE_NON_REAL: ErrorTypes::Message = ErrorTypes::Message { id: 514, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Argument '%s' to der has illegal type %s, must be a subtype of Real.") } };

pub static UNUSED_MODIFIER: ErrorTypes::Message = ErrorTypes::Message { id: 515, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("In modifier %s.") } };

pub static MULTIPLE_MODIFIER: ErrorTypes::Message = ErrorTypes::Message { id: 516, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Multiple modifiers in same scope for element %s.") } };

pub static INCONSISTENT_UNITS: ErrorTypes::Message = ErrorTypes::Message { id: 517, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The system of units is inconsistent in term %s with the units %s and %s respectively.") } };

pub static CONSISTENT_UNITS: ErrorTypes::Message = ErrorTypes::Message { id: 518, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("The system of units is consistent.") } };

pub static INCOMPLETE_UNITS: ErrorTypes::Message = ErrorTypes::Message { id: 519, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("The system of units is incomplete. Please provide unit information to the model by e.g. using types from the SIunits package.") } };

pub static INVALID_UNIT: ErrorTypes::Message = ErrorTypes::Message { id: 520, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid unit expression '%s'.") } };

pub static ASSIGN_RHS_ELABORATION: ErrorTypes::Message = ErrorTypes::Message { id: 521, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to elaborate rhs of %s.") } };

pub static FAILED_TO_EVALUATE_EXPRESSION: ErrorTypes::Message = ErrorTypes::Message { id: 522, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Could not evaluate expression: %s") } };

pub static WARNING_JACOBIAN_EQUATION_SOLVE: ErrorTypes::Message = ErrorTypes::Message { id: 523, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Jacobian equation %s could not solve proper for %s. Assume %s=0.") } };

pub static SIMPLIFICATION_COMPLEXITY: ErrorTypes::Message = ErrorTypes::Message { id: 523, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Simplification produced a higher complexity (%s) than the original (%s). The simplification was: %s => %s.") } };

pub static ITERATOR_NON_ARRAY: ErrorTypes::Message = ErrorTypes::Message { id: 524, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Iterator %s, has type %s, but expected a 1D array expression.") } };

pub static INST_INVALID_RESTRICTION: ErrorTypes::Message = ErrorTypes::Message { id: 525, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot instantiate %s due to class specialization %s.") } };

pub static INST_NON_LOADED: ErrorTypes::Message = ErrorTypes::Message { id: 526, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Library %s was not loaded but is marked as used by model %s.") } };

pub static RECURSION_DEPTH_REACHED: ErrorTypes::Message = ErrorTypes::Message { id: 527, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The maximum recursion depth of %s was reached, probably due to mutual recursion. The current scope: %s.") } };

pub static DERIVATIVE_INPUT: ErrorTypes::Message = ErrorTypes::Message { id: 528, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The model requires derivatives of some inputs as listed below:\n%s") } };

pub static UTF8_COMMAND_LINE_ARGS: ErrorTypes::Message = ErrorTypes::Message { id: 529, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The compiler was sent command-line arguments that were not UTF-8 encoded and will abort the current execution.") } };

pub static PACKAGE_ORDER_FILE_NOT_COMPLETE: ErrorTypes::Message = ErrorTypes::Message { id: 530, ty: crate::ErrorTypes::MessageType::GRAMMAR, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The package.order file does not list all .mo files and directories (containing package.mo) present in its directory.\nMissing names are:\n\t%s") } };

pub static REINIT_IN_WHEN_INITIAL: ErrorTypes::Message = ErrorTypes::Message { id: 531, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Using reinit in when with condition initial() is not allowed. Use assignment or equality equation instead.") } };

pub static MISSING_INNER_CLASS: ErrorTypes::Message = ErrorTypes::Message { id: 532, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("No corresponding 'inner' declaration found for class %s declared as '%s'.\n Continuing flattening by only considering the 'outer' class declaration.") } };

pub static RECURSION_DEPTH_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 533, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The maximum recursion depth of %s was reached when evaluating expression %s in scope %s. Translation may still succeed but you are recommended to fix the problem.") } };

pub static RECURSION_DEPTH_DERIVED: ErrorTypes::Message = ErrorTypes::Message { id: 534, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The maximum recursion depth of was reached when instantiating a derived class. Current class %s in scope %s.") } };

pub static EVAL_EXTERNAL_OBJECT_CONSTRUCTOR: ErrorTypes::Message = ErrorTypes::Message { id: 535, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("OpenModelica requires that all external objects input arguments are possible to evaluate before initialization in order to avoid odd run-time failures, but %s is a variable.") } };

pub static CLASS_ANNOTATION_DOES_NOT_EXIST: ErrorTypes::Message = ErrorTypes::Message { id: 536, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Could not find class annotation %s in class %s.") } };

pub static SEPARATE_COMPILATION_PACKAGE_FAILED: ErrorTypes::Message = ErrorTypes::Message { id: 537, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to compile all functions in package %s.") } };

pub static INVALID_ARRAY_DIM_IN_SCALAR_OP: ErrorTypes::Message = ErrorTypes::Message { id: 538, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The operator scalar requires all dimension size to be 1, but the input has type %s.") } };

pub static NON_STANDARD_OPERATOR_CLASS_DIRECTORY: ErrorTypes::Message = ErrorTypes::Message { id: 539, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("classDirectory() is a non-standard operator that was replaced by Modelica.Utilities.Files.loadResource(uri) before it was added to the language specification.") } };

pub static PACKAGE_DUPLICATE_CHILDREN: ErrorTypes::Message = ErrorTypes::Message { id: 540, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The same class is defined in multiple files: %s.") } };

pub static INTEGER_ENUMERATION_CONVERSION_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 541, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Integer (%s) to enumeration (%s) conversion is not valid Modelica, please use enumeration constant (%s) instead.") } };

pub static INTEGER_ENUMERATION_OUT_OF_RANGE: ErrorTypes::Message = ErrorTypes::Message { id: 542, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The Integer to %s conversion failed, as the Integer %s is outside the range (1, ..., %s) of values corresponding to enumeration constants.") } };

pub static INTEGER_TO_UNKNOWN_ENUMERATION: ErrorTypes::Message = ErrorTypes::Message { id: 543, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::INTERNAL, message: Gettext::TranslatableContent::gettext { msgid: literal!("The Integer (%s) to enumeration conversion failed because information about the the enumeration type is missing.") } };

pub static NORETCALL_INVALID_EXP: ErrorTypes::Message = ErrorTypes::Message { id: 544, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expression %s is not a valid statement - only function calls are allowed.") } };

pub static INVALID_FLAG_TYPE_STRINGS: ErrorTypes::Message = ErrorTypes::Message { id: 545, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid type of flag %s, expected one of %s but got %s.") } };

pub static FUNCTION_RETURN_EXT_OBJ: ErrorTypes::Message = ErrorTypes::Message { id: 546, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Function %s returns an external object, but the only function allowed to return this object is %s.") } };

pub static NON_STANDARD_OPERATOR: ErrorTypes::Message = ErrorTypes::Message { id: 547, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Usage of non-standard operator (not specified in the Modelica specification): %s. Functionality might be partially supported but is not guaranteed.") } };

pub static CONNECT_ARRAY_SIZE_ZERO: ErrorTypes::Message = ErrorTypes::Message { id: 548, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Ignoring connection of array components having size zero: %s and %s.") } };

pub static ILLEGAL_RECORD_COMPONENT: ErrorTypes::Message = ErrorTypes::Message { id: 549, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Ignoring record component:\n%swhen building the record constructor. Records are allowed to contain only components of basic types, arrays of basic types or other records.") } };

pub static EQ_WITHOUT_TIME_DEP_VARS: ErrorTypes::Message = ErrorTypes::Message { id: 550, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Found equation without time-dependent variables: %s = %s") } };

pub static OVERCONSTRAINED_OPERATOR_SIZE_ZERO: ErrorTypes::Message = ErrorTypes::Message { id: 551, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Ignoring overconstrained operator applied to array components having size zero: %s.") } };

pub static OVERCONSTRAINED_OPERATOR_SIZE_ZERO_RETURN_FALSE: ErrorTypes::Message = ErrorTypes::Message { id: 552, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Returning false from overconstrained operator applied to array components having size zero: %s.") } };

pub static MISMATCHING_INTERFACE_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 553, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("__OpenModelica_Interface types are incompatible. Got interface type '%s', expected something compatible with '%s'.") } };

pub static MISSING_INTERFACE_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 554, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Annotation __OpenModelica_Interface is missing or the string is not in the input list.") } };

pub static CLASS_NOT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 555, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Class %s not found inside class %s.") } };

pub static NOTIFY_LOAD_MODEL_FAILED: ErrorTypes::Message = ErrorTypes::Message { id: 556, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Skipped loading package %s (%s) using MODELICAPATH %s (uses-annotation may be wrong).") } };

pub static ROOT_USER_INTERACTIVE: ErrorTypes::Message = ErrorTypes::Message { id: 557, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("You are trying to run OpenModelica as a server using the root user.\nThis is a very bad idea:\n* The socket interface does not authenticate the user.\n* OpenModelica allows execution of arbitrary commands.") } };

pub static USES_MISSING_VERSION: ErrorTypes::Message = ErrorTypes::Message { id: 558, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Uses-annotation is missing version for library %s. Assuming the tool-specific version=\"default\".") } };

pub static CLOCK_PREFIX_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 559, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Clock variable can not be declared with prefixes flow, stream, discrete, parameter, or constant.") } };

pub static DEFAULT_CLOCK_USED: ErrorTypes::Message = ErrorTypes::Message { id: 560, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Default inferred clock is used.") } };

pub static CONT_CLOCKED_PARTITION_CONFLICT_VAR: ErrorTypes::Message = ErrorTypes::Message { id: 561, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable %s belongs to clocked and continuous partitions.") } };

pub static ELSE_WHEN_CLOCK: ErrorTypes::Message = ErrorTypes::Message { id: 562, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Clocked when equation can not contain elsewhen part.") } };

pub static REINIT_NOT_IN_WHEN: ErrorTypes::Message = ErrorTypes::Message { id: 563, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operator reinit may only be used in the body of a when equation.") } };

pub static NESTED_CLOCKED_WHEN: ErrorTypes::Message = ErrorTypes::Message { id: 564, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Nested clocked when statements are not allowed.") } };

pub static CLOCKED_WHEN_BRANCH: ErrorTypes::Message = ErrorTypes::Message { id: 565, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Clocked when branch in when equation.") } };

pub static CLOCKED_WHEN_IN_WHEN_EQ: ErrorTypes::Message = ErrorTypes::Message { id: 566, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Clocked when equation inside the body of when equation.") } };

pub static CONT_CLOCKED_PARTITION_CONFLICT_EQ: ErrorTypes::Message = ErrorTypes::Message { id: 567, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Equation belongs to clocked and continuous partitions.") } };

pub static CLOCK_SOLVERMETHOD: ErrorTypes::Message = ErrorTypes::Message { id: 568, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Applying clock solverMethod %s instead of specified %s. Supported are: ImplicitEuler, SemiImplicitEuler, ExplicitEuler and ImplicitTrapezoid.") } };

pub static INVALID_CLOCK_EQUATION: ErrorTypes::Message = ErrorTypes::Message { id: 569, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid form of clock equation") } };

pub static SUBCLOCK_CONFLICT: ErrorTypes::Message = ErrorTypes::Message { id: 570, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Partition has different sub-clock %ss (%s) and (%s).") } };

pub static CLOCK_CONFLICT: ErrorTypes::Message = ErrorTypes::Message { id: 571, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Partitions have different base clocks.") } };

pub static EXEC_STAT: ErrorTypes::Message = ErrorTypes::Message { id: 572, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Performance of %s: time %s/%s, allocations: %s / %s, free: %s / %s") } };

pub static EXEC_STAT_GC: ErrorTypes::Message = ErrorTypes::Message { id: 573, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Performance of %s: time %s/%s, GC stats:%s") } };

pub static MAX_TEARING_SIZE: ErrorTypes::Message = ErrorTypes::Message { id: 574, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Tearing is skipped for strong component %s because system size of %s exceeds maximum system size for tearing of %s systems (%s).\nTo adjust the maximum system size for tearing use --%s=<size>.\n") } };

pub static NO_TEARING_FOR_COMPONENT: ErrorTypes::Message = ErrorTypes::Message { id: 575, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Tearing is skipped for strong component %s because of activated compiler flag 'noTearingForComponent=%1'.\n") } };

pub static WRONG_VALUE_OF_ARG: ErrorTypes::Message = ErrorTypes::Message { id: 576, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Wrong value of argument to %s: %s = %s %s.") } };

pub static USER_DEFINED_TEARING_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 577, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Wrong usage of user defined tearing: %s Make sure you use user defined tearing as stated in the flag description.") } };

pub static USER_TEARING_VARS: ErrorTypes::Message = ErrorTypes::Message { id: 578, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Following iteration variables are selected by the user for strong component %s (DAE kind: %s):\n%s") } };

pub static CLASS_EXTENDS_TARGET_NOT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 579, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Base class targeted by class extends %s not found in the inherited classes.") } };

pub static ASSIGN_PARAM_FIXED_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 580, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Trying to assign to parameter component %s(fixed=true) in %s := %s") } };

pub static EQN_NO_SPACE_TO_SOLVE: ErrorTypes::Message = ErrorTypes::Message { id: 581, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Equation %s (size: %s) %s is not big enough to solve for enough variables.\n  Remaining unsolved variables are:%s\n  Already solved:%s\n  Equations used to solve those variables:%s") } };

pub static VAR_NO_REMAINING_EQN: ErrorTypes::Message = ErrorTypes::Message { id: 582, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable %s does not have any remaining equation to be solved in.\n  The original equations were:%s") } };

pub static MOVING_PARAMETER_BINDING_TO_INITIAL_EQ_SECTION: ErrorTypes::Message = ErrorTypes::Message { id: 583, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Moving binding to initial equation section and setting fixed attribute of %s to false.") } };

pub static MIXED_DETERMINED: ErrorTypes::Message = ErrorTypes::Message { id: 584, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The initialization problem of given system is mixed-determined. It is under- as well as overdetermined and the mixed-determination-index is too high. [index > %s]\nPlease checkout the option \"--maxMixedDeterminedIndex\" to simulate with a higher threshold or consider changing some initial equations, fixed variables and start values. Use -d=initialization for more information.") } };

pub static STACK_OVERFLOW_DETAILED: ErrorTypes::Message = ErrorTypes::Message { id: 585, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Stack overflow occurred while evaluating %s:\n%s") } };

pub static NF_VECTOR_INVALID_DIMENSIONS: ErrorTypes::Message = ErrorTypes::Message { id: 586, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid dimensions %s in %s, no more than one dimension may have size > 1.") } };

pub static NF_ARRAY_TYPE_MISMATCH: ErrorTypes::Message = ErrorTypes::Message { id: 587, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Array types mismatch. Argument %s (%s) has type %s whereas previous arguments have type %s.") } };

pub static NF_DIFFERENT_NUM_DIM_IN_ARGUMENTS: ErrorTypes::Message = ErrorTypes::Message { id: 588, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Different number of dimensions (%s) in arguments to %s.") } };

pub static NF_CAT_WRONG_DIMENSION: ErrorTypes::Message = ErrorTypes::Message { id: 589, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument of cat characterizes an existing dimension in the other arguments (1..%s), but got dimension %s.") } };

pub static NF_CAT_FIRST_ARG_EVAL: ErrorTypes::Message = ErrorTypes::Message { id: 590, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The first argument of cat must be possible to evaluate during compile-time. Expression %s has variability %s.") } };

pub static COMMA_OPERATOR_DIFFERENT_SIZES: ErrorTypes::Message = ErrorTypes::Message { id: 591, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Arguments of concatenation comma operator have different sizes for the first dimension: %s has dimension %s and %s has dimension %s.") } };

pub static NON_STATE_STATESELECT_ALWAYS: ErrorTypes::Message = ErrorTypes::Message { id: 592, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable %s has attribute stateSelect=StateSelect.always, but can't be selected as a state.") } };

pub static STATE_STATESELECT_NEVER: ErrorTypes::Message = ErrorTypes::Message { id: 593, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Variable %s has attribute stateSelect=StateSelect.never, but was selected as a state") } };

pub static FUNCTION_HIGHER_VARIABILITY_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 594, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Component '%s' of variability %s has binding %s of higher variability %s.") } };

pub static OCG_MISSING_BRANCH: ErrorTypes::Message = ErrorTypes::Message { id: 595, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Connections.rooted(%s) needs exactly one statement Connections.branch(%s, B.R) involving %s but we found none in the graph. Run with -d=cgraphGraphVizFile to debug") } };

pub static UNBOUND_PARAMETER_EVALUATE_TRUE: ErrorTypes::Message = ErrorTypes::Message { id: 596, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Parameter %s has annotation(Evaluate=true) and no binding.") } };

pub static FMI_URI_RESOLVE: ErrorTypes::Message = ErrorTypes::Message { id: 597, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Could not resolve URI (%s) at compile-time; copying all loaded packages into the FMU") } };

pub static PATTERN_MIXED_POS_NAMED: ErrorTypes::Message = ErrorTypes::Message { id: 598, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Call to %s contains mixed positional and named arguments.") } };

pub static STATE_STATESELECT_NEVER_FORCED: ErrorTypes::Message = ErrorTypes::Message { id: 599, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Following variables have attribute stateSelect=StateSelect.never, but cant be statically chosen. %s") } };

pub static STATE_STATESELECT_PREFER_REVERT: ErrorTypes::Message = ErrorTypes::Message { id: 600, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Some equations could not be differentiated for following variables having attribute stateSelect=StateSelect.prefer. %s") } };

pub static ERROR_PKG_NOT_IDENT: ErrorTypes::Message = ErrorTypes::Message { id: 601, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The package manager only accepts simple identifiers (%s has a dot in it).") } };

pub static ERROR_PKG_NOT_FOUND_VERSION: ErrorTypes::Message = ErrorTypes::Message { id: 602, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The package index did not contain an entry for package %s that provides version %s. The following versions are available:\n%s") } };

pub static ERROR_PKG_NOT_EXACT_MATCH: ErrorTypes::Message = ErrorTypes::Message { id: 603, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The package index did not contain an entry for package %s of version %s. There are other versions that claim to be compatible: %s.") } };

pub static ERROR_PKG_INDEX_NOT_ON_PATH: ErrorTypes::Message = ErrorTypes::Message { id: 604, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The MODELICAPATH (%s) does not contain %s, so the package index cannot be used.") } };

pub static ERROR_PKG_INDEX_NOT_FOUND: ErrorTypes::Message = ErrorTypes::Message { id: 605, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The package index does not exist: %s.") } };

pub static ERROR_PKG_INDEX_NOT_PARSED: ErrorTypes::Message = ErrorTypes::Message { id: 606, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The package index %s could not be parsed.") } };

pub static ERROR_PKG_INDEX_FAILED_DOWNLOAD: ErrorTypes::Message = ErrorTypes::Message { id: 607, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to download package index %s to file %s.") } };

pub static NOTIFY_PKG_INDEX_DOWNLOAD: ErrorTypes::Message = ErrorTypes::Message { id: 608, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Downloaded package index from URL %s.") } };

pub static NOTIFY_PKG_INSTALL_DONE: ErrorTypes::Message = ErrorTypes::Message { id: 609, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Package installed successfully (SHA %s).") } };

pub static NOTIFY_PKG_UPGRADE_DONE: ErrorTypes::Message = ErrorTypes::Message { id: 609, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Package upgraded successfully (SHA %s from %s).") } };

pub static ERROR_PKG_INSTALL_NO_PACKAGE_MO: ErrorTypes::Message = ErrorTypes::Message { id: 611, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("After extracting %s, %s does not exist. Removing the failed installation.") } };

pub static WARNING_PKG_CONFLICTING_VERSIONS: ErrorTypes::Message = ErrorTypes::Message { id: 612, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Conflicting versions for loading package %s: %s is to be installed, but another package requires version %s which is not provided by this version.") } };

pub static NOTIFY_PKG_NO_INSTALL: ErrorTypes::Message = ErrorTypes::Message { id: 613, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s %s will not be installed since version %s is installed.") } };

pub static DEPRECATED_FLAG: ErrorTypes::Message = ErrorTypes::Message { id: 614, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The flag '%s' is deprecated. Please use '%s' instead.") } };

pub static UNKNOWN_ERROR_INST_FUNCTION: ErrorTypes::Message = ErrorTypes::Message { id: 615, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::INTERNAL, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unknown error trying to instantiate function: %s.") } };

pub static NOTIFY_INITIALIZING_USER_LIBRARIES: ErrorTypes::Message = ErrorTypes::Message { id: 616, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cached libraries were found and will be installed into %s.") } };

pub static NOTIFY_PKG_ALREADY_INSTALLED: ErrorTypes::Message = ErrorTypes::Message { id: 617, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s %s is already installed, skipping.") } };

pub static REINIT_IN_ALGORITHM: ErrorTypes::Message = ErrorTypes::Message { id: 618, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operator reinit may not be used in an algorithm section (use translation flag --allowNonStandardModelica=reinitInAlgorithms to ignore).") } };

pub static HIDE_RESULT_NOT_EVALUATED: ErrorTypes::Message = ErrorTypes::Message { id: 619, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Ignoring the hideResult annotation on '%s' which could not be evaluated, probably due to missing annotation(Evaluate=true).") } };

pub static MISPLACED_EXTERNAL_ANNOTATION: ErrorTypes::Message = ErrorTypes::Message { id: 620, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("External function annotation should occur on the external-clause, not on the function.") } };

pub static MATCH_SHADOWING: ErrorTypes::Message = ErrorTypes::Message { id: 5001, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Local variable '%s' shadows another variable.") } };

pub static META_POLYMORPHIC: ErrorTypes::Message = ErrorTypes::Message { id: 5002, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s uses invalid subtypeof syntax. Only subtypeof Any is supported.") } };

pub static META_FUNCTION_TYPE_NO_PARTIAL_PREFIX: ErrorTypes::Message = ErrorTypes::Message { id: 5003, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s is used as a function reference, but doesn't specify the partial prefix.") } };

pub static META_MATCH_EQUATION_FORBIDDEN: ErrorTypes::Message = ErrorTypes::Message { id: 5004, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Match expression equation sections forbid the use of %s-equations.") } };

pub static META_UNIONTYPE_ALIAS_MODS: ErrorTypes::Message = ErrorTypes::Message { id: 5005, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Uniontype %s was not generated correctly. One possible cause is modifications, which are not allowed.") } };

pub static META_COMPLEX_TYPE_MOD: ErrorTypes::Message = ErrorTypes::Message { id: 5006, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("MetaModelica complex types may not have modifiers.") } };

pub static META_CEVAL_FUNCTION_REFERENCE: ErrorTypes::Message = ErrorTypes::Message { id: 5008, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot evaluate function pointers (got %s).") } };

pub static NON_INSTANTIATED_FUNCTION: ErrorTypes::Message = ErrorTypes::Message { id: 5009, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Tried to use function %s, but it was not instantiated.") } };

pub static META_UNSOLVED_POLYMORPHIC_BINDINGS: ErrorTypes::Message = ErrorTypes::Message { id: 5010, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Could not solve the polymorphism in the function call to %s\n  Input bindings:\n%s\n  Solved bindings:\n%s\n  Unsolved bindings:\n%s") } };

pub static META_RECORD_FOUND_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 5011, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("In record constructor %s: %s") } };

pub static META_INVALID_PATTERN: ErrorTypes::Message = ErrorTypes::Message { id: 5012, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid pattern: %s") } };

pub static META_MATCH_GENERAL_FAILURE: ErrorTypes::Message = ErrorTypes::Message { id: 5014, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to elaborate match expression %s") } };

pub static META_CONS_TYPE_MATCH: ErrorTypes::Message = ErrorTypes::Message { id: 5015, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Failed to match types of cons expression %s. The head has type %s and the tail %s.") } };

pub static META_NONE_CREF: ErrorTypes::Message = ErrorTypes::Message { id: 5017, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("NONE is not acceptable syntax. Use NONE() instead.") } };

pub static META_INVALID_PATTERN_NAMED_FIELD: ErrorTypes::Message = ErrorTypes::Message { id: 5018, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid named fields: %s. Valid field names: %s.") } };

pub static META_INVALID_LOCAL_ELEMENT: ErrorTypes::Message = ErrorTypes::Message { id: 5019, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Only components without direction are allowed in local declarations, got: %s") } };

pub static META_INVALID_COMPLEX_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 5020, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Invalid complex type name: %s") } };

pub static META_CONSTRUCTOR_NOT_PART_OF_UNIONTYPE: ErrorTypes::Message = ErrorTypes::Message { id: 5021, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("In pattern %s: %s is not part of uniontype %s") } };

pub static META_TYPE_MISMATCH_PATTERN: ErrorTypes::Message = ErrorTypes::Message { id: 5022, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Type mismatch in pattern %s\nexpression type:\n  %s\npattern type:\n  %s") } };

pub static META_CONSTRUCTOR_NOT_RECORD: ErrorTypes::Message = ErrorTypes::Message { id: 5023, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Call pattern is not a record constructor %s") } };

pub static META_MATCHEXP_RESULT_TYPES: ErrorTypes::Message = ErrorTypes::Message { id: 5024, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Match expression has mismatched result types:%s") } };

pub static MATCHCONTINUE_TO_MATCH_OPTIMIZATION: ErrorTypes::Message = ErrorTypes::Message { id: 5025, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("This matchcontinue expression has no overlapping patterns and should be using match instead of matchcontinue.") } };

pub static META_DEAD_CODE: ErrorTypes::Message = ErrorTypes::Message { id: 5026, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Dead code elimination: %s.") } };

pub static META_UNUSED_DECL: ErrorTypes::Message = ErrorTypes::Message { id: 5027, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unused local variable: %s.") } };

pub static META_UNUSED_AS_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 5028, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Removing unused as-binding: %s.") } };

pub static MATCH_TO_SWITCH_OPTIMIZATION: ErrorTypes::Message = ErrorTypes::Message { id: 5029, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Converted match expression to switch of type %s.") } };

pub static REDUCTION_TYPE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 5030, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Reductions require the types of the %s and %s to be %s, but got: %s and %s.") } };

pub static UNSUPPORTED_REDUCTION_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 5031, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expected a reduction function with type signature ('A,'B) => 'B, but got %s.") } };

pub static FOUND_NON_NUMERIC_TYPES: ErrorTypes::Message = ErrorTypes::Message { id: 5032, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Operator %s expects numeric types as operands, but got '%s and %s'.") } };

pub static STRUCTURAL_PARAMETER_OR_CONSTANT_WITH_NO_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 5033, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Could not evaluate structural parameter (or constant): %s which gives dimensions of array: %s. Array dimensions must be known at compile time.") } };

pub static META_UNUSED_ASSIGNMENT: ErrorTypes::Message = ErrorTypes::Message { id: 5034, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Removing unused assignment to: %s.") } };

pub static META_EMPTY_CALL_PATTERN: ErrorTypes::Message = ErrorTypes::Message { id: 5035, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Removing empty call named pattern argument: %s.") } };

pub static META_ALL_EMPTY: ErrorTypes::Message = ErrorTypes::Message { id: 5036, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("All patterns in call were empty: %s.") } };

pub static DUPLICATE_DEFINITION: ErrorTypes::Message = ErrorTypes::Message { id: 5037, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The same variable is being defined twice: %s.") } };

pub static PATTERN_VAR_NOT_VARIABLE: ErrorTypes::Message = ErrorTypes::Message { id: 5038, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Identifiers need to point to local or output variables. Variable %s is %s.") } };

pub static LIST_REVERSE_WRONG_ORDER: ErrorTypes::Message = ErrorTypes::Message { id: 5039, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("%1:=listAppend(%1, _) has the first argument in the \"wrong\" order.\n  It is very slow to keep appending a linked list (scales like O(N²)).\n  Consider building the list in the reverse order in order to improve performance (scales like O(N) even if you need to reverse a lot of lists). Use annotation __OpenModelica_DisableListAppendWarning=true to disable this message for a certain assignment.") } };

pub static IS_PRESENT_WRONG_SCOPE: ErrorTypes::Message = ErrorTypes::Message { id: 5040, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("isPresent needs to be called from a function scope, got %s.") } };

pub static IS_PRESENT_WRONG_DIRECTION: ErrorTypes::Message = ErrorTypes::Message { id: 5041, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("isPresent needs to be called on an input or output formal parameter.") } };

pub static IS_PRESENT_INVALID_EXP: ErrorTypes::Message = ErrorTypes::Message { id: 5042, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("isPresent needs to be called on an input or output formal parameter, but got a non-identifier expression: %s.") } };

pub static METARECORD_WITH_TYPEVARS: ErrorTypes::Message = ErrorTypes::Message { id: 5043, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Records inside uniontypes must not contain type variables (got: %s). Put them on the uniontype instead.") } };

pub static UNIONTYPE_MISSING_TYPEVARS: ErrorTypes::Message = ErrorTypes::Message { id: 5044, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Uniontype %s has type variables, but they were not given in the declaration.") } };

pub static UNIONTYPE_WRONG_NUM_TYPEVARS: ErrorTypes::Message = ErrorTypes::Message { id: 5045, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Uniontype %s has %s type variables, but got %s.") } };

pub static SERIALIZED_SIZE: ErrorTypes::Message = ErrorTypes::Message { id: 5046, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("%s uses %s of memory (%s without GC overhead; %s is consumed by not performing String sharing).") } };

pub static META_MATCH_CONSTANT: ErrorTypes::Message = ErrorTypes::Message { id: 5047, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Match input %s is a constant value.") } };

pub static CONVERSION_MISSING_FROM_VERSION: ErrorTypes::Message = ErrorTypes::Message { id: 5048, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Conversion-annotation is missing version for from-conversion: %s.") } };

pub static CONVERSION_UNKNOWN_ANNOTATION: ErrorTypes::Message = ErrorTypes::Message { id: 5049, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Conversion-annotation contains unknown element: %s.") } };

pub static CONVERSION_MISSING_NONE_FROM_VERSION: ErrorTypes::Message = ErrorTypes::Message { id: 5048, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Conversion-annotation is missing version for noneFromVersion: %s.") } };

pub static UNPATCHED_MODELICA_SERVICES: ErrorTypes::Message = ErrorTypes::Message { id: 5049, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("This version of ModelicaServices does not appear to be patched for use with OpenModelica, consider using a version distributed by OpenModelica instead to avoid compatibility issues.") } };

pub static META_MATCH_UNUSED_INPUT: ErrorTypes::Message = ErrorTypes::Message { id: 5050, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Match input %s is not used by any case and could be removed.") } };

pub static META_PATTERN_INFALLIBLE_NO_BINDING: ErrorTypes::Message = ErrorTypes::Message { id: 5051, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Pattern %s is infallible and binds no variables; it could be replaced with a wildcard.") } };

pub static META_PATTERN_AS_ONLY: ErrorTypes::Message = ErrorTypes::Message { id: 5052, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Pattern only renames the match input %s; the match expression could be rewritten without this input and the body could use %s directly.") } };

pub static MATCHCONTINUE_TO_TRY_OPTIMIZATION: ErrorTypes::Message = ErrorTypes::Message { id: 5053, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("This matchcontinue has a single case and an else and could be rewritten as a try/else.") } };

pub static MATCH_SINGLE_INFALLIBLE_CASE: ErrorTypes::Message = ErrorTypes::Message { id: 5054, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("This match expression has a single case with an infallible pattern; it could be rewritten as a destructuring assignment of the input(s).") } };

pub static COMPILER_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 5999, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::notrans { r#str: literal!("%s") } };

pub static COMPILER_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 6000, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::notrans { r#str: literal!("%s") } };

pub static COMPILER_NOTIFICATION: ErrorTypes::Message = ErrorTypes::Message { id: 6001, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::notrans { r#str: literal!("%s") } };

pub static COMPILER_NOTIFICATION_SCRIPTING: ErrorTypes::Message = ErrorTypes::Message { id: 6002, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::notrans { r#str: literal!("%s") } };

pub static SUSAN_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7000, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::notrans { r#str: literal!("%s") } };

pub static TEMPLATE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7001, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Template error: %s.") } };

pub static PARMODELICA_WARNING: ErrorTypes::Message = ErrorTypes::Message { id: 7004, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::notrans { r#str: literal!("ParModelica: %s.") } };

pub static PARMODELICA_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7005, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::notrans { r#str: literal!("ParModelica: %s.") } };

pub static OPTIMICA_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7006, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::notrans { r#str: literal!("Optimica: %s.") } };

pub static FILE_NOT_FOUND_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7007, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("File not Found: %s.") } };

pub static UNKNOWN_FMU_VERSION: ErrorTypes::Message = ErrorTypes::Message { id: 7008, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unknown FMU version %s. Only version 1.0 & 2.0 are supported.") } };

pub static UNKNOWN_FMU_TYPE: ErrorTypes::Message = ErrorTypes::Message { id: 7009, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unknown FMU type %s. Supported types are me (model exchange), cs (co-simulation) & me_cs (model exchange & co-simulation).") } };

pub static FMU_EXPORT_NOT_SUPPORTED: ErrorTypes::Message = ErrorTypes::Message { id: 7010, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Export of FMU type %s for version %s is not supported. Supported combinations are me (model exchange) for versions 1.0 & 2.0, cs (co-simulation) & me_cs (model exchange & co-simulation) for version 2.0.") } };

// FIGARO_ERROR added by Alexander Carlqvist
pub static FIGARO_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7011, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::notrans { r#str: literal!("Figaro: %s.") } };

pub static SUSAN_NOTIFY: ErrorTypes::Message = ErrorTypes::Message { id: 7012, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::notrans { r#str: literal!("%s") } };

pub static PDEModelica_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7013, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("PDEModelica: %s") } };

pub static TEMPLATE_ERROR_FUNC: ErrorTypes::Message = ErrorTypes::Message { id: 7014, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Template error: A template call failed (%s). One possible reason could be that a template imported function call failed (which should not happen for functions called from within template code; templates assert pure 'match'/non-failing semantics).") } };

pub static FMU_EXPORT_NOT_SUPPORTED_CPP: ErrorTypes::Message = ErrorTypes::Message { id: 7015, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("Export of FMU type %s is not supported with Cpp target. FMU will be for Model Exchange (me).") } };

pub static DEPRECATED_API_CALL: ErrorTypes::Message = ErrorTypes::Message { id: 7016, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("'%1' is deprecated. It is recommended to use '%2' instead.") } };

pub static REDUNDANT_ALIAS_SET: ErrorTypes::Message = ErrorTypes::Message { id: 7017, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::WARNING, message: Gettext::TranslatableContent::gettext { msgid: literal!("The model contains alias variables with redundant start and/or conflicting nominal values. It is recommended to resolve the conflicts, because otherwise the system could be hard to solve. To print the conflicting alias sets and the chosen candidates please use -d=aliasConflicts.") } };

pub static CONFLICTING_ALIAS_SET: ErrorTypes::Message = ErrorTypes::Message { id: 7018, ty: crate::ErrorTypes::MessageType::SYMBOLIC, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("The model contains alias variables with conflicting fixed start values. It is necessary to resolve the conflicts, because otherwise the initial system is impossible to solve. To print the conflicting alias sets and the chosen candidates please use -d=aliasConflicts.") } };

pub static PACKAGE_FILE_NOT_FOUND_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7019, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unable to find the package definition file. Looked for \"%s\", \"%s\", \"%s\", \"%s\", \"%s\", \"%s\", \"%s\" and \"%s\".") } };

pub static UNABLE_TO_UNZIP_FILE: ErrorTypes::Message = ErrorTypes::Message { id: 7020, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Unable to unzip the file: %s.") } };

pub static EXPECTED_ENCRYPTED_PACKAGE: ErrorTypes::Message = ErrorTypes::Message { id: 7021, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Expected encrypted package with .mol extension got: %s.") } };

pub static SAVE_ENCRYPTED_CLASS_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7022, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot save the encrypted class. Encrypted classes are read-only.") } };

pub static ACCESS_ENCRYPTED_PROTECTED_CONTENTS: ErrorTypes::Message = ErrorTypes::Message { id: 7023, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::NOTIFICATION, message: Gettext::TranslatableContent::gettext { msgid: literal!("Cannot access encrypted and protected class contents.") } };

pub static INVALID_NONLINEAR_JACOBIAN_COMPONENT: ErrorTypes::Message = ErrorTypes::Message { id: 7024, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Jacobian %s contains non-linear components. This indicates a singular system or internal generation errors.") } };

pub static DUPLICATE_VARIABLE_ERROR: ErrorTypes::Message = ErrorTypes::Message { id: 7025, ty: crate::ErrorTypes::MessageType::TRANSLATION, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("Duplicate elements:\n %s.") } };

pub static ENCRYPTION_NOT_SUPPORTED: ErrorTypes::Message = ErrorTypes::Message { id: 7026, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("File not Found: %s. Compile OpenModelica with Encryption support.") } };

pub static FMU_EXPORT_DAE_MODE_NOT_SUPPORTED: ErrorTypes::Message = ErrorTypes::Message { id: 7027, ty: crate::ErrorTypes::MessageType::SCRIPTING, severity: crate::ErrorTypes::Severity::ERROR, message: Gettext::TranslatableContent::gettext { msgid: literal!("DAE mode (--daeMode) is not supported for FMU export. Please remove the --daeMode flag.") } };

pub static dummyInfo: SourceInfo = SourceInfo { fileName: literal!(""), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };

pub fn clearCurrentComponent() -> Result<()> {
    updateCurrentComponent((literal!("")).clone(), dummyInfo.clone(), (std::sync::Arc::new(fnptr!(dummy, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))?;
    Ok(())
}

fn dummy(mut r#str: ArcStr) -> ArcStr {
    let mut r#str: ArcStr = r#str;
    r#str
}

pub fn updateCurrentComponent(mut component: ArcStr, mut info: SourceInfo, mut func: Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>) -> Result<()> {
    pub type prefixToStr = std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>;

    let mut tpl: Option<(metamodelica::Array<ArcStr>, metamodelica::Array<SourceInfo>, metamodelica::Array<Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>>)> = None;
    let mut astr: metamodelica::Array<ArcStr>;
    let mut ainfo: metamodelica::Array<SourceInfo>;
    let mut afunc: metamodelica::Array<Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>>;
    tpl = crate::Globals::currentInstVar.with(|__root| __root.borrow().clone());
    let () = (match tpl.clone() {
        None => {
            { let __v = Some((arrayCreate(1, (component.clone()).clone()), arrayCreate(1, info.clone()), arrayCreate(1, func.clone()))); crate::Globals::currentInstVar.with(|__root| *__root.borrow_mut() = __v) };
            ()
        },
        Some((mut astr, mut ainfo, mut afunc)) => {
            {let _arr = astr.clone(); _arr.borrow_mut()[(1-1) as usize] = (component.clone()).clone(); _arr};
            {let _arr = ainfo.clone(); _arr.borrow_mut()[(1-1) as usize] = info.clone(); _arr};
            {let _arr = afunc.clone(); _arr.borrow_mut()[(1-1) as usize] = func.clone(); _arr};
            ()
        },
    });
    Ok(())
}

pub fn getCurrentComponent() -> Result<(ArcStr, i32, i32, i32, i32, bool, ArcStr)> {
    type prefixToStr = std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr = arcstr::literal!("");
    let mut sline: i32 = 0;
    let mut scol: i32 = 0;
    let mut eline: i32 = 0;
    let mut ecol: i32 = 0;
    let mut read_only: bool = false;
    let mut filename: ArcStr = literal!("");
    let mut tpl: Option<(metamodelica::Array<ArcStr>, metamodelica::Array<SourceInfo>, metamodelica::Array<Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>>)> = None;
    let mut astr: metamodelica::Array<ArcStr>;
    let mut ainfo: metamodelica::Array<SourceInfo>;
    let mut afunc: metamodelica::Array<Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>>;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut func: Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>;
    tpl = crate::Globals::currentInstVar.with(|__root| __root.borrow().clone());
    r#str = ((match tpl.clone() {
        None => literal!(""),
        Some((mut astr, mut ainfo, mut afunc)) => {
            r#str = (astr.clone().borrow()[(1-1) as usize].clone()).clone();
            if r#str.clone() != literal!("") {
                func = afunc.clone().borrow()[(1-1) as usize].clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*func((r#str.clone()).clone())?); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone();
                info = ainfo.clone().borrow()[(1-1) as usize].clone();
                sline = info.lineNumberStart.clone();
                scol = info.columnNumberStart.clone();
                eline = info.lineNumberEnd.clone();
                ecol = info.columnNumberEnd.clone();
                read_only = info.isReadOnly.clone();
                filename = info.fileName.clone();
            }
            r#str.clone()
        },
    })).clone();
    Ok((r#str, sline, scol, eline, ecol, read_only, filename))
}

pub fn addMessage(mut inErrorMsg: ErrorTypes::Message, mut inMessageTokens: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let mut msg_type: ErrorTypes::MessageType = ErrorTypes::MessageType::GRAMMAR;
    let mut severity: ErrorTypes::Severity = ErrorTypes::Severity::ERROR;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut msg_str: ArcStr = arcstr::literal!("");
    let mut file: ArcStr = arcstr::literal!("");
    let mut error_id: i32 = 0;
    let mut sline: i32 = 0;
    let mut scol: i32 = 0;
    let mut eline: i32 = 0;
    let mut ecol: i32 = 0;
    let mut isReadOnly: bool = false;
    let mut msg: Gettext::TranslatableContent;
    if !(Flags::getConfigBool(Flags::DEMO_MODE.clone())?) {
        (r#str, sline, scol, eline, ecol, isReadOnly, file) = getCurrentComponent()?;
        let ErrorTypes::MESSAGE { id: __pa0, ty: __pa1, severity: __pa2, message: __pa3 } = (inErrorMsg.clone()) else { bail!("pattern mismatch") };
        error_id = __pa0.clone();
        msg_type = __pa1.clone();
        severity = __pa2.clone();
        msg = __pa3.clone();
        msg_str = (Gettext::translateContent(msg.clone())?).clone();
        ErrorExt::addSourceMessage(error_id.clone(), msg_type.clone(), severity.clone(), sline.clone(), scol.clone(), eline.clone(), ecol.clone(), isReadOnly.clone(), (Testsuite::friendly((file.clone()).clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*msg_str.clone()); ArcStr::from(__mm_s) }).clone(), inMessageTokens.clone());
    }
    Ok(())
}

pub fn addSourceMessage(mut inErrorMsg: ErrorTypes::Message, mut inMessageTokens: Arc<metamodelica::List<ArcStr>>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inErrorMsg.clone(), inMessageTokens.clone(), inInfo.clone())) {
        (ErrorTypes::Message { id: error_id, ty: msg_type, severity, message: msg }, tokens, SourceInfo { columnNumberEnd: ecol, lineNumberEnd: eline, columnNumberStart: scol, lineNumberStart: sline, isReadOnly, fileName: file, .. }) => {
            let mut msg_str: ArcStr = arcstr::literal!("");
            msg_str = (Gettext::translateContent(msg.clone())?).clone();
            ErrorExt::addSourceMessage(error_id.clone(), msg_type.clone(), severity.clone(), sline.clone(), scol.clone(), eline.clone(), ecol.clone(), isReadOnly.clone(), (Testsuite::friendly((file.clone()).clone())?).clone(), (msg_str.clone()).clone(), tokens.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn addSourceMessageAsError(mut msg: ErrorTypes::Message, mut tokens: Arc<metamodelica::List<ArcStr>>, mut info: SourceInfo) -> Result<()> {
    let mut m: ErrorTypes::Message = msg.clone();
    m.severity = crate::ErrorTypes::Severity::ERROR;
    addSourceMessage(m.clone(), tokens.clone(), info.clone())?;
    Ok(())
}

pub fn addStrictMessage(mut errorMsg: ErrorTypes::Message, mut tokens: Arc<metamodelica::List<ArcStr>>, mut info: SourceInfo) -> Result<()> {
    let mut msg: ErrorTypes::Message = errorMsg.clone();
    if Flags::getConfigBool(Flags::STRICT.clone())? {
        msg.severity = crate::ErrorTypes::Severity::ERROR;
        addSourceMessageAndFail(msg.clone(), tokens.clone(), info.clone())?;
        unreachable!("addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    } else {
        addSourceMessage(msg.clone(), tokens.clone(), info.clone())?;
    }
    Ok(())
}

pub fn addSourceMessageAndFail(mut inErrorMsg: ErrorTypes::Message, mut inMessageTokens: Arc<metamodelica::List<ArcStr>>, mut inInfo: SourceInfo) -> Result<()> {
    addSourceMessage(inErrorMsg.clone(), inMessageTokens.clone(), inInfo.clone())?;
    bail!("fail");
    Ok(())
}

pub fn addMultiSourceMessage(mut inErrorMsg: ErrorTypes::Message, mut inMessageTokens: Arc<metamodelica::List<ArcStr>>, mut inInfo: Arc<metamodelica::List<SourceInfo>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inInfo.clone()) {
        Deref @ metamodelica::List::Cons { head: info, tail: Deref @ metamodelica::List::Nil } => {
            addSourceMessage(inErrorMsg.clone(), inMessageTokens.clone(), info.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: info, tail: rest_info } => {
            if !(listMember(info.clone(), rest_info.clone())) {
                addSourceMessage(ERROR_FROM_HERE.clone(), metamodelica::nil(), info.clone())?;
            }
            addMultiSourceMessage(inErrorMsg.clone(), inMessageTokens.clone(), rest_info.clone())?;
            ()
        },
        Deref @ metamodelica::List::Nil => {
            addMessage(inErrorMsg.clone(), inMessageTokens.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn addMessageOrSourceMessage(mut inErrorMsg: ErrorTypes::Message, mut inMessageTokens: Arc<metamodelica::List<ArcStr>>, mut inInfoOpt: Option<SourceInfo>) -> Result<()> {
    let () = (match inInfoOpt.clone() {
        None => {
            addMessage(inErrorMsg.clone(), inMessageTokens.clone())?;
            ()
        },
        Some(mut info) => {
            addSourceMessage(inErrorMsg.clone(), inMessageTokens.clone(), info.clone())?;
            ()
        },
    });
    Ok(())
}

pub fn addTotalMessage(mut message: ErrorTypes::TotalMessage) -> Result<()> {
    let mut msg: ErrorTypes::Message = <ErrorTypes::Message as ::std::default::Default>::default();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let ErrorTypes::TOTALMESSAGE { info: __pa0, msg: __pa1 } = (message.clone()) else { bail!("pattern mismatch") };
    info = __pa0.clone();
    msg = __pa1.clone();
    addSourceMessage(msg.clone(), metamodelica::nil(), info.clone())?;
    Ok(())
}

pub fn addTotalMessages(mut messages: Arc<metamodelica::List<ErrorTypes::TotalMessage>>) -> Result<()> {
    for mut msg in &*messages.clone() {
        let mut msg = msg.clone();
        addTotalMessage(msg.clone())?;
    }
    Ok(())
}

pub fn printMessagesStr(mut warningsAsErrors: bool) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    res = (ErrorExt::printMessagesStr(warningsAsErrors.clone())).clone();
    res
}

pub fn printErrorsNoWarning() -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    res = (ErrorExt::printErrorsNoWarning()).clone();
    res
}

pub fn printMessagesStrLst() -> Arc<metamodelica::List<ArcStr>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = (match () {
        () => list![(literal!("Not impl. yet")).clone()],
    });
    outStringLst
}

pub fn printMessagesStrLstType(mut inMessageType: ErrorTypes::MessageType) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = (match inMessageType.clone() {
        _ => list![(literal!("Not impl. yet")).clone()],
    });
    outStringLst
}

pub fn printMessagesStrLstSeverity(mut inSeverity: ErrorTypes::Severity) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = (match inSeverity.clone() {
        _ => list![(literal!("Not impl. yet")).clone()],
    });
    outStringLst
}

pub fn clearMessages() -> () {
    ErrorExt::clearMessages();
    ()
}

pub fn getNumMessages() -> i32 {
    let mut num: i32 = 0;
    num = ErrorExt::getNumMessages();
    num
}

pub fn getNumErrorMessages() -> i32 {
    let mut num: i32 = 0;
    num = ErrorExt::getNumErrorMessages();
    num
}

pub fn getMessages() -> Arc<metamodelica::List<ErrorTypes::TotalMessage>> {
    let mut res: Arc<metamodelica::List<ErrorTypes::TotalMessage>> = metamodelica::nil();
    res = ErrorExt::getMessages();
    res
}

pub fn getMessagesStrType(mut inMessageType: ErrorTypes::MessageType) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (literal!("not impl yet.")).clone();
    outString
}

pub fn getMessagesStrSeverity(mut inSeverity: ErrorTypes::Severity) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (literal!("not impl yet.")).clone();
    outString
}

pub fn messageTypeStr(mut inMessageType: ErrorTypes::MessageType) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inMessageType.clone() {
        ErrorTypes::MessageType::SYNTAX => literal!("SYNTAX"),
        ErrorTypes::MessageType::GRAMMAR => literal!("GRAMMAR"),
        ErrorTypes::MessageType::TRANSLATION => literal!("TRANSLATION"),
        ErrorTypes::MessageType::SYMBOLIC => literal!("SYMBOLIC"),
        ErrorTypes::MessageType::SIMULATION => literal!("SIMULATION"),
        ErrorTypes::MessageType::SCRIPTING => literal!("SCRIPTING"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn severityStr(mut inSeverity: ErrorTypes::Severity) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inSeverity.clone() {
        ErrorTypes::Severity::INTERNAL => literal!("Internal error"),
        ErrorTypes::Severity::ERROR => literal!("Error"),
        ErrorTypes::Severity::WARNING => literal!("Warning"),
        ErrorTypes::Severity::NOTIFICATION => literal!("Notification"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn infoStr(mut info: SourceInfo) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match info.clone() {
        SourceInfo { columnNumberEnd: mut col_end, lineNumberEnd: mut line_end, columnNumberStart: mut col_start, lineNumberStart: mut line_start, fileName: mut filename, .. } => {
            let mut info_str: ArcStr = arcstr::literal!("");
            info_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*Testsuite::friendly((filename.clone()).clone())?); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(line_start.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(col_start.clone())); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*intString(line_end.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(col_end.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
            info_str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn assertion(mut b: bool, mut message: ArcStr, mut info: SourceInfo) -> Result<()> {
    let () = (match b.clone() {
        true => (),
        _ => {
            addSourceMessage(INTERNAL_ERROR.clone(), list![(message.clone()).clone()], info.clone())?;
            bail!("fail")
        },
    });
    Ok(())
}

pub fn assertionOrAddSourceMessage(mut inCond: bool, mut inErrorMsg: ErrorTypes::Message, mut inMessageTokens: Arc<metamodelica::List<ArcStr>>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (match inCond.clone() {
        true => (),
        _ => {
            addSourceMessage(inErrorMsg.clone(), inMessageTokens.clone(), inInfo.clone())?;
            failOnErrorMsg(inErrorMsg.clone())?;
            ()
        },
    });
    Ok(())
}

fn failOnErrorMsg(mut inMessage: ErrorTypes::Message) -> Result<()> {
    let () = (match inMessage.clone() {
        ErrorTypes::Message { severity: ErrorTypes::Severity::ERROR, .. } => bail!("fail"),
        _ => (),
    });
    Ok(())
}

pub fn addCompilerError(mut message: ArcStr) -> Result<()> {
    addMessage(COMPILER_ERROR.clone(), list![(message.clone()).clone()])?;
    Ok(())
}

pub fn addCompilerWarning(mut message: ArcStr) -> Result<()> {
    addMessage(COMPILER_WARNING.clone(), list![(message.clone()).clone()])?;
    Ok(())
}

pub fn addCompilerNotification(mut message: ArcStr) -> Result<()> {
    addMessage(COMPILER_NOTIFICATION.clone(), list![(message.clone()).clone()])?;
    Ok(())
}

pub fn addInternalError(mut message: ArcStr, mut info: SourceInfo) -> Result<()> {
    let mut filename: ArcStr = arcstr::literal!("");
    if Testsuite::isRunning()? {
        let SourceInfo { fileName: __pa0, .. } = (info.clone()) else { bail!("pattern mismatch") };
        filename = __pa0.clone();
        addSourceMessage(INTERNAL_ERROR.clone(), list![(message.clone()).clone()], SourceInfo { fileName: (filename.clone()).clone(), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat((0) as f64) })?;
    } else {
        addSourceMessage(INTERNAL_ERROR.clone(), list![(message.clone()).clone()], info.clone())?;
    }
    Ok(())
}

pub fn terminateError(mut message: ArcStr, mut info: SourceInfo) -> Result<()> {
    ErrorExt::addSourceMessage(0, crate::ErrorTypes::MessageType::TRANSLATION, crate::ErrorTypes::Severity::INTERNAL, info.lineNumberStart.clone(), info.columnNumberStart.clone(), info.lineNumberEnd.clone(), info.columnNumberEnd.clone(), info.isReadOnly.clone(), info.fileName.clone(), (literal!("%s")).clone(), list![(message.clone()).clone()]);
    println!("{}", (ErrorExt::printMessagesStr(false)).clone());
    System::exit(-1)?;
    Ok(())
}

