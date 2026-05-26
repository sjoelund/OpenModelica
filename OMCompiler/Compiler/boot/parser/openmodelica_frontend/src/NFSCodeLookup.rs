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

use crate::NFEnvExtends;
use crate::NFInstPrefix;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeEnv;
use crate::NFSCodeFlattenImports;
use crate::NFSCodeFlattenRedeclare;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

pub type Env = Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>;

pub type Item = Arc<NFSCodeEnv::Item>;

pub type Extends = Arc<NFSCodeEnv::Extends>;

pub type Frame = Arc<NFSCodeEnv::Frame>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RedeclareReplaceStrategy {
    INSERT_REDECLARES,
    IGNORE_REDECLARES,
}
pub use self::RedeclareReplaceStrategy::{INSERT_REDECLARES,IGNORE_REDECLARES};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LookupStrategy {
    NO_BUILTIN_TYPES,
    LOOKUP_ANY,
}
pub use self::LookupStrategy::{NO_BUILTIN_TYPES,LOOKUP_ANY};

// Default parts of the declarations for builtin elements and types.
pub static BUILTIN_PREFIXES: std::sync::LazyLock<Arc<SCode::Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Prefixes { visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(openmodelica_frontend_types::SCode::Replaceable::NOT_REPLACEABLE) }) });

pub static BUILTIN_ATTRIBUTES: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| { SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static BUILTIN_CONST_ATTRIBUTES: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| { SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static BUILTIN_EMPTY_CLASS: std::sync::LazyLock<Arc<SCode::ClassDef>> = std::sync::LazyLock::new(|| { Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }) });

// Metatypes used to define the builtin types.
pub static BUILTIN_REALTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$RealType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_INTEGERTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$IntegerType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_BOOLEANTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$BooleanType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_STRINGTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$StringType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_ENUMTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$EnumType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_REALTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REALTYPE.clone(), isUsed: None }) });

pub static BUILTIN_INTEGERTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_INTEGERTYPE.clone(), isUsed: None }) });

pub static BUILTIN_BOOLEANTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_BOOLEANTYPE.clone(), isUsed: None }) });

pub static BUILTIN_STRINGTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STRINGTYPE.clone(), isUsed: None }) });

pub static BUILTIN_ENUMTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ENUMTYPE.clone(), isUsed: None }) });

pub static BUILTIN_REALTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$RealType")).clone() }), arrayDim: None }) });

pub static BUILTIN_INTEGERTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$IntegerType")).clone() }), arrayDim: None }) });

pub static BUILTIN_BOOLEANTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$BooleanType")).clone() }), arrayDim: None }) });

pub static BUILTIN_STRINGTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$StringType")).clone() }), arrayDim: None }) });

pub static BUILTIN_ENUMTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$EnumType")).clone() }), arrayDim: None }) });

pub static BUILTIN_STATESELECT_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("StateSelect")).clone() }), arrayDim: None }) });

// Parts of the builtin types.
// Generic elements:
pub static BUILTIN_ATTR_QUANTITY: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("quantity")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_ATTR_UNIT: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("unit")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_ATTR_DISPLAYUNIT: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("displayUnit")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_ATTR_FIXED: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("fixed")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_BOOLEANTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_ATTR_STATESELECT: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("stateSelect")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STATESELECT_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Real-specific elements:
pub static BUILTIN_REAL_MIN: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("min")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_REALTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_REAL_MAX: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("max")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_REALTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_REAL_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_REALTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_REAL_NOMINAL: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("nominal")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_REALTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Integer-specific elements:
pub static BUILTIN_INTEGER_MIN: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("min")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_INTEGER_MAX: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("max")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_INTEGER_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Boolean-specific elements:
pub static BUILTIN_BOOLEAN_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_BOOLEANTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// String-specific elements:
pub static BUILTIN_STRING_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// StateSelect-specific elements:
pub static BUILTIN_ENUM_MIN: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("min")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_ENUM_MAX: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("max")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_ENUM_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_STATESELECT_NEVER: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("never")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_STATESELECT_AVOID: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("avoid")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_STATESELECT_DEFAULT: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("default")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_STATESELECT_PREFER: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("prefer")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static BUILTIN_STATESELECT_ALWAYS: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("always")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Environments for the builtin types:
pub static BUILTIN_REAL_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("Real")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("nominal")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REAL_NOMINAL.clone(), isUsed: None }), height: 3, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("max")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REAL_MAX.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("fixed")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_FIXED.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("displayUnit")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_DISPLAYUNIT.clone(), isUsed: None }) }), right: Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("min")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REAL_MIN.clone(), isUsed: None }) }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REAL_START.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("stateSelect")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_STATESELECT.clone(), isUsed: None }), height: 1, left: Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("unit")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_UNIT.clone(), isUsed: None }) }) }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

pub static BUILTIN_INTEGER_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("Integer")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("max")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_INTEGER_MAX.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("fixed")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_FIXED.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("min")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_INTEGER_MIN.clone(), isUsed: None }) }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_INTEGER_START.clone(), isUsed: None }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

pub static BUILTIN_BOOLEAN_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("Boolean")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("fixed")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_FIXED.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_BOOLEAN_START.clone(), isUsed: None }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

pub static BUILTIN_STRING_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("String")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }), height: 2, left: Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STRING_START.clone(), isUsed: None }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

pub static BUILTIN_STATESELECT_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("StateSelect")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("max")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ENUM_MAX.clone(), isUsed: None }), height: 3, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("default")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_DEFAULT.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("avoid")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_AVOID.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("always")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_ALWAYS.clone(), isUsed: None }) }), right: Arc::new(crate::NFSCodeEnv::EnvTree::Tree::EMPTY) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("fixed")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_FIXED.clone(), isUsed: None }) }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("never")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_NEVER.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("min")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ENUM_MIN.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("prefer")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_PREFER.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ENUM_START.clone(), isUsed: None }) }) }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

// The builtin types:
pub static BUILTIN_REAL: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("Real")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_REAL_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub static BUILTIN_INTEGER: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("Integer")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_INTEGER_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub static BUILTIN_BOOLEAN: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("Boolean")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_BOOLEAN_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub static BUILTIN_STRING: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("String")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_STRING_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub static BUILTIN_STATESELECT: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("StateSelect")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS, classDef: Arc::new(SCode::ClassDef::ENUMERATION { enumLst: list![Arc::new(SCode::Enum { literal: (literal!("never")).clone(), comment: SCode::noComment.clone() }), Arc::new(SCode::Enum { literal: (literal!("avoid")).clone(), comment: SCode::noComment.clone() }), Arc::new(SCode::Enum { literal: (literal!("default")).clone(), comment: SCode::noComment.clone() }), Arc::new(SCode::Enum { literal: (literal!("prefer")).clone(), comment: SCode::noComment.clone() }), Arc::new(SCode::Enum { literal: (literal!("always")).clone(), comment: SCode::noComment.clone() })] }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_STATESELECT_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub static BUILTIN_EXTERNALOBJECT: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("ExternalObject")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub static BUILTIN_CLOCK: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("Clock")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub fn lookupSimpleName(mut inName: ArcStr, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupSimpleName2((inName.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
        (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outItem = __pa0.clone();
    outPath = __pa1.clone();
    outEnv = __pa2.clone();
    Ok((outItem, outPath, outEnv))
}

fn lookupSimpleName2(mut inName: ArcStr, mut inEnv: Env, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inEnv.clone(), inVisitedScopes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (opt_item, opt_path, opt_env) = lookupInLocalScope((inName.clone()).clone(), inEnv.clone(), inVisitedScopes.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: frame_type, name: Some(scope_name), .. }, tail: rest_env }, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    frameNotEncapsulated(frame_type.clone())?;
                    (opt_item, opt_path, opt_env) = lookupSimpleName2((inName.clone()).clone(), rest_env.clone(), cons(scope_name.clone(), inVisitedScopes.clone()))?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE, .. }, tail: rest_env }, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    let mut rest_env = (*rest_env).clone();
                    rest_env = NFSCodeEnv::getEnvTopScope(rest_env.clone())?;
                    (opt_item, opt_path, opt_env) = lookupSimpleName2((inName.clone()).clone(), rest_env.clone(), metamodelica::nil())?;
                    checkBuiltinItem(opt_item.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub fn frameNotEncapsulated(mut frameType: FrameType) -> Result<()> {
    let _ = (match frameType.clone() {
        NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE => bail!("fail"),
        _ => (),
    });
    Ok(())
}

fn checkBuiltinItem(mut inItem: Option<Arc<NFSCodeEnv::Item>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inItem.clone()) {
        Some(Deref @ NFSCodeEnv::Item::CLASS { classType: NFSCodeEnv::ClassType::BUILTIN, .. }) => (),
        None => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn lookupInLocalScope(mut inName: ArcStr, mut inEnv: Env, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inEnv.clone(), inVisitedScopes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut env: Env = metamodelica::nil();
                    let mut item: Item;
                    (item, env) = lookupInClass((inName.clone()).clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() })), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (opt_item, opt_path, opt_env) = lookupInBaseClasses((inName.clone()).clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::INSERT_REDECLARES, inVisitedScopes.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { qualifiedImports: imps, hidden: false, .. }, .. }, tail: _ }, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (opt_item, opt_path, opt_env) = lookupInQualifiedImports((inName.clone()).clone(), imps.clone(), inEnv.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { unqualifiedImports: imps, hidden: false, .. }, .. }, tail: _ }, _) => {
                    let mut env: Env = metamodelica::nil();
                    let mut item: Item;
                    let mut path: Arc<Absyn::Path>;
                    (item, path, env) = lookupInUnqualifiedImports((inName.clone()).clone(), imps.clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: rest_env }, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (opt_item, opt_path, opt_env) = lookupInLocalScope((inName.clone()).clone(), rest_env.clone(), inVisitedScopes.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub fn lookupInClass(mut inName: ArcStr, mut inEnv: Env) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    let mut tree: Arc<NFSCodeEnv::EnvTree::Tree> = Arc::new(NFSCodeEnv::EnvTree::Tree::EMPTY);
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tree = __pa0.clone();
    outItem = NFSCodeEnv::EnvTree::get(tree.clone(), (inName.clone()).clone())?;
    (outItem, outEnv) = resolveAlias(outItem.clone(), inEnv.clone())?;
    Ok((outItem, outEnv))
}

pub fn resolveAlias(mut inItem: Item, mut inEnv: Env) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outEnv) = (::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
        (Deref @ NFSCodeEnv::Item::ALIAS { path: None, name, .. }, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ }) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            item = NFSCodeEnv::EnvTree::get(tree.clone(), (name.clone()).clone())?;
            (item, env) = resolveAlias(item.clone(), inEnv.clone())?;
            (item.clone(), env.clone())
        },
        (Deref @ NFSCodeEnv::Item::ALIAS { path: Some(path), name, .. }, _) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            let mut tree: Arc<NFSCodeEnv::EnvTree::Tree> = Arc::new(NFSCodeEnv::EnvTree::Tree::EMPTY);
            env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
            env = NFSCodeEnv::enterScopePath(env.clone(), path.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
                Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            tree = __pa0.clone();
            item = NFSCodeEnv::EnvTree::get(tree.clone(), (name.clone()).clone())?;
            (item, env) = resolveAlias(item.clone(), env.clone())?;
            (item.clone(), env.clone())
        },
        _ => {
            (inItem.clone(), inEnv.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outItem, outEnv))
}

fn lookupInBaseClasses(mut inName: ArcStr, mut inEnv: Env, mut inReplaceRedeclares: RedeclareReplaceStrategy, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    let mut env: Env = metamodelica::nil();
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    bcl = __pa0.clone();
    env = NFSCodeEnv::removeExtendsFromLocalScope(inEnv.clone())?;
    env = NFSCodeEnv::setImportTableHidden(env.clone(), false)?;
    (outItem, outPath, outEnv) = lookupInBaseClasses2((inName.clone()).clone(), bcl.clone(), env.clone(), inEnv.clone(), inReplaceRedeclares.clone(), inVisitedScopes.clone())?;
    Ok((outItem, outPath, outEnv))
}

fn lookupInBaseClasses2(mut inName: ArcStr, mut inBaseClasses: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>, mut inEnv: Env, mut inEnvWithExtends: Env, mut inReplaceRedeclares: RedeclareReplaceStrategy, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inBaseClasses.clone(), inEnv.clone(), inEnvWithExtends.clone(), inReplaceRedeclares.clone(), inVisitedScopes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: ext, tail: _ }, _, _, _, _) => {
                    let mut item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut path: Option<Arc<Absyn::Path>> = None;
                    let mut env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (item, path, env) = lookupInBaseClasses3((inName.clone()).clone(), ext.clone(), inEnv.clone(), inEnvWithExtends.clone(), inReplaceRedeclares.clone(), inVisitedScopes.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest_ext }, _, _, _, _) => {
                    let mut item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut path: Option<Arc<Absyn::Path>> = None;
                    let mut env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (item, path, env) = lookupInBaseClasses2((inName.clone()).clone(), rest_ext.clone(), inEnv.clone(), inEnvWithExtends.clone(), inReplaceRedeclares.clone(), inVisitedScopes.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub fn lookupInBaseClasses3(mut inName: ArcStr, mut inBaseClass: Extends, mut inEnv: Env, mut inEnvWithExtends: Env, mut inReplaceRedeclares: RedeclareReplaceStrategy, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inBaseClass.clone(), inEnv.clone(), inEnvWithExtends.clone(), inReplaceRedeclares.clone(), inVisitedScopes.clone())) {
        (_, Deref @ NFSCodeEnv::Extends { info, baseClass: bc @ Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", .. }, .. }, _, _, _, _) => {
            NFEnvExtends::printExtendsError(bc.clone(), inEnvWithExtends.clone(), info.clone())?;
            (None, None, None)
        },
        (_, Deref @ NFSCodeEnv::Extends { info, redeclareModifiers: redecls, baseClass: bc, .. }, _, _, _, _) => {
            let mut path: Arc<Absyn::Path>;
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            let mut opt_path: Option<Arc<Absyn::Path>> = None;
            let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
            let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
            (item, path, env) = lookupBaseClassName(bc.clone(), inEnv.clone(), info.clone())?;
            let true = (checkVisitedScopes(inVisitedScopes.clone(), inEnv.clone(), path.clone())?) else { bail!("pattern mismatch") };
            item = NFSCodeEnv::setImportsInItemHidden(item.clone(), true)?;
            (opt_item, opt_env) = NFSCodeFlattenRedeclare::replaceRedeclares(redecls.clone(), item.clone(), env.clone(), inEnvWithExtends.clone(), inReplaceRedeclares.clone())?;
            (opt_item, opt_path, opt_env) = lookupInBaseClasses4(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), opt_item.clone(), opt_env.clone())?;
            (opt_item.clone(), opt_path.clone(), opt_env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outPath, outEnv))
}

fn checkVisitedScopes(mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>, mut inEnv: Env, mut inBaseClass: Arc<Absyn::Path>) -> Result<bool> {
    let mut outRes: bool = false;
    outRes = 'mc: {
        let __mc_input = (inVisitedScopes.clone(), inEnv.clone(), inBaseClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut env_path: Arc<Absyn::Path>;
                    let mut visited_path: Arc<Absyn::Path>;
                    let mut bc_path: Arc<Absyn::Path>;
                    env_path = NFSCodeEnv::getEnvPath(inEnv.clone())?;
                    bc_path = AbsynUtil::removePrefix(env_path.clone(), inBaseClass.clone())?;
                    visited_path = AbsynUtil::stringListPath(inVisitedScopes.clone());
                    let true = (AbsynUtil::pathPrefixOf(visited_path.clone(), bc_path.clone())) else { bail!("pattern mismatch") };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRes)
}

fn lookupInBaseClasses4(mut inName: Arc<Absyn::Path>, mut inItem: Option<Arc<NFSCodeEnv::Item>>, mut inEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inItem.clone(), inEnv.clone())) {
        (_, None, None) => {
            (None, None, None)
        },
        (_, Some(item), Some(env)) => {
            let mut path: Arc<Absyn::Path>;
            let mut item = (*item).clone();
            let mut env = (*env).clone();
            (item, path, env) = lookupNameInItem(inName.clone(), item.clone(), env.clone())?;
            (Some(item.clone()), Some(path.clone()), Some(env.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outPath, outEnv))
}

pub fn lookupInQualifiedImports(mut inName: ArcStr, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: Env) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inImports.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: rest_imps }, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    let false = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (opt_item, opt_path, opt_env) = lookupInQualifiedImports((inName.clone()).clone(), rest_imps.clone(), inEnv.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { path, name }, tail: _ }, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut path = (*path).clone();
                    let true = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (item, path, env) = lookupFullyQualified(path.clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: _ }, _) => {
                    let true = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((None, None, None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub fn lookupInUnqualifiedImports(mut inName: ArcStr, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inImports.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ }, _) => {
                    let mut item: Item;
                    let mut path2: Arc<Absyn::Path>;
                    let mut env: Env = metamodelica::nil();
                    let mut path = (*path).clone();
                    (item, path, env) = lookupFullyQualified(path.clone(), inEnv.clone())?;
                    (item, path2, env) = lookupNameInItem(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), item.clone(), env.clone())?;
                    path = joinPaths(path.clone(), path2.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest_imps }, _) => {
                    let mut item: Item;
                    let mut path: Arc<Absyn::Path>;
                    let mut env: Env = metamodelica::nil();
                    (item, path, env) = lookupInUnqualifiedImports((inName.clone()).clone(), rest_imps.clone(), inEnv.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub fn lookupFullyQualified(mut inName: Arc<Absyn::Path>, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    let mut env: Env = metamodelica::nil();
    env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
    (outItem, outPath, outEnv) = lookupNameInPackage(inName.clone(), env.clone())?;
    outPath = AbsynUtil::makeFullyQualified(outPath.clone());
    Ok((outItem, outPath, outEnv))
}

pub fn lookupNameInPackage(mut inName: Arc<Absyn::Path>, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inEnv.clone())) {
        (Deref @ Absyn::Path::IDENT { name }, _) => {
            let mut path: Arc<Absyn::Path>;
            let mut env: Env = metamodelica::nil();
            let mut item: Item;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
                (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            item = __pa0.clone();
            path = __pa1.clone();
            env = __pa2.clone();
            env = NFSCodeEnv::setImportTableHidden(env.clone(), false)?;
            (item.clone(), path.clone(), env.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { path, name }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut new_path: Arc<Absyn::Path>;
            let mut env: Env = metamodelica::nil();
            let mut item: Item;
            let mut path = (*path).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
                (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            item = __pa0.clone();
            new_path = __pa1.clone();
            env = __pa2.clone();
            env = NFSCodeEnv::setImportTableHidden(env.clone(), false)?;
            (item, path, env) = lookupNameInItem(path.clone(), item.clone(), env.clone())?;
            path = joinPaths(new_path.clone(), path.clone())?;
            (item.clone(), path.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outPath, outEnv))
}

pub fn lookupCrefInPackage(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env) -> Result<(Item, Arc<Absyn::ComponentRef>)> {
    let mut outItem: Item;
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    (outItem, outCref) = 'mc: {
        let __mc_input = (inCref.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: subs, name }, _) => {
                    let mut new_path: Arc<Absyn::Path>;
                    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut item: Item;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
                        (Some(__pa0), Some(__pa1), _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    new_path = __pa1.clone();
                    cref = AbsynUtil::pathToCrefWithSubs(new_path.clone(), subs.clone())?;
                    Ok((item.clone(), cref.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cref_rest, subscripts: subs, name }, _) => {
                    let mut new_path: Arc<Absyn::Path>;
                    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut cref_rest = (*cref_rest).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
                        (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    new_path = __pa1.clone();
                    env = __pa2.clone();
                    (item, cref_rest) = lookupCrefInItem(cref_rest.clone(), item.clone(), env.clone())?;
                    if '__try3: {
                        ::match_deref::match_deref! { match &(cref_rest.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: _ } => (),
                            _ => break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    cref = AbsynUtil::pathToCrefWithSubs(new_path.clone(), subs.clone())?;
                    cref = AbsynUtil::joinCrefs(cref.clone(), cref_rest.clone())?;
                    Ok((item.clone(), cref.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cref_rest, name, .. }, _) => {
                    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut cref_rest = (*cref_rest).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
                        (Some(__pa0), Some(_), Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    env = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(lookupCrefInItem(cref_rest.clone(), item.clone(), env.clone())?) {
                        (__pa2, __pa3 @ Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: _ }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa2.clone();
                    cref_rest = __pa3.clone();
                    cref = cref_rest.clone();
                    Ok((item.clone(), cref.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outCref))
}

pub fn lookupNameInItem(mut inName: Arc<Absyn::Path>, mut inItem: Item, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inItem.clone(), inEnv.clone())) {
        (_, Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { info, modifications: mods, typeSpec: type_spec, .. }, .. }, env) => {
            let mut item: Item;
            let mut path: Arc<Absyn::Path>;
            let mut type_env: Env = metamodelica::nil();
            let mut redeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
            let mut env = (*env).clone();
            (item, _, type_env) = lookupTypeSpec(type_spec.clone(), env.clone(), info.clone())?;
            redeclares = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(mods.clone());
            (item, type_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redeclares.clone(), item.clone(), type_env.clone(), inEnv.clone(), NFInstPrefix::emptyPrefix.clone())?;
            (item, path, env) = lookupNameInItem(inName.clone(), item.clone(), type_env.clone())?;
            (item.clone(), path.clone(), env.clone())
        },
        (_, Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::List::Cons { head: class_env, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut item: Item;
            let mut path: Arc<Absyn::Path>;
            let mut env: Env = metamodelica::nil();
            env = NFSCodeEnv::enterFrame(class_env.clone(), inEnv.clone());
            (item, path, env) = lookupNameInPackage(inName.clone(), env.clone())?;
            (item.clone(), path.clone(), env.clone())
        },
        (_, Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { declaredEnv: env, item }, _) => {
            let mut path: Arc<Absyn::Path>;
            let mut env = (*env).clone();
            let mut item = (*item).clone();
            (item, path, env) = lookupNameInItem(inName.clone(), item.clone(), env.clone())?;
            (item.clone(), path.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outPath, outEnv))
}

pub fn lookupCrefInItem(mut inCref: Arc<Absyn::ComponentRef>, mut inItem: Item, mut inEnv: Env) -> Result<(Item, Arc<Absyn::ComponentRef>)> {
    let mut outItem: Item;
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    (outItem, outCref) = (::match_deref::match_deref! { match &((inCref.clone(), inItem.clone(), inEnv.clone())) {
        (_, Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { info, modifications: mods, typeSpec: type_spec, .. }, .. }, _) => {
            let mut item: Item;
            let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut type_env: Env = metamodelica::nil();
            let mut redeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
            (item, _, type_env) = lookupTypeSpec(type_spec.clone(), inEnv.clone(), info.clone())?;
            redeclares = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(mods.clone());
            (item, type_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redeclares.clone(), item.clone(), type_env.clone(), inEnv.clone(), NFInstPrefix::emptyPrefix.clone())?;
            (item, cref) = lookupCrefInItem(inCref.clone(), item.clone(), type_env.clone())?;
            (item.clone(), cref.clone())
        },
        (_, Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::List::Cons { head: class_env, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut item: Item;
            let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut env: Env = metamodelica::nil();
            env = NFSCodeEnv::enterFrame(class_env.clone(), inEnv.clone());
            (item, cref) = lookupCrefInPackage(inCref.clone(), env.clone())?;
            (item.clone(), cref.clone())
        },
        (_, Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { declaredEnv: env, item }, _) => {
            let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut item = (*item).clone();
            (item, cref) = lookupCrefInItem(inCref.clone(), item.clone(), env.clone())?;
            (item.clone(), cref.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outCref))
}

pub fn lookupBaseClasses(mut inName: ArcStr, mut inEnv: Env) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    bcl = __pa0.clone();
    (_, outBaseClasses) = List::fold22(bcl.clone(), Arc::new(lookupBaseClasses2), (inName.clone()).clone(), inEnv.clone(), metamodelica::nil(), metamodelica::nil());
    let false = (outBaseClasses.clone().is_empty()) else { bail!("pattern mismatch") };
    outBaseClasses = outBaseClasses.clone().reverse();
    Ok(outBaseClasses)
}

fn lookupBaseClasses2(mut inBaseClass: Extends, mut inName: ArcStr, mut inEnv: Env, mut items: Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, mut bcl: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> {
    let mut items: Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>> = items;
    let mut bcl: Arc<metamodelica::List<Arc<Absyn::Path>>> = bcl;
    (items, bcl) = 'mc: {
        let __mc_input = (inBaseClass.clone(), inName.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Extends { info, baseClass: bc, .. }, _, _) => {
                    let mut env: Env = metamodelica::nil();
                    let mut item: Item;
                    (item, _, env) = lookupBaseClassName(bc.clone(), inEnv.clone(), info.clone())?;
                    item = NFSCodeEnv::setImportsInItemHidden(item.clone(), true)?;
                    (item, _, _) = lookupNameInItem(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), item.clone(), env.clone())?;
                    Ok((cons(item.clone(), items.clone()), cons(bc.clone(), bcl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((items.clone(), bcl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((items, bcl))
}

pub fn lookupInheritedName(mut inName: ArcStr, mut inEnv: Env) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses((inName.clone()).clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::INSERT_REDECLARES, metamodelica::nil())?) {
        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outItem = __pa0.clone();
    outEnv = __pa1.clone();
    Ok((outItem, outEnv))
}

pub fn lookupInheritedNameAndBC(mut inName: ArcStr, mut inEnv: Env) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> {
    let mut outItems: Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>> = metamodelica::nil();
    let mut outBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    bcl = __pa0.clone();
    (outItems, outBaseClasses) = List::fold22(bcl.clone(), Arc::new(lookupBaseClasses2), (inName.clone()).clone(), inEnv.clone(), metamodelica::nil(), metamodelica::nil());
    outBaseClasses = outBaseClasses.clone().reverse();
    outItems = outItems.clone().reverse();
    Ok((outItems, outBaseClasses))
}

pub fn lookupRedeclaredClassByItem(mut inItem: Item, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outEnv) = 'mc: {
        let __mc_input = (inItem.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, .. }, .. }, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut rdp: SCode::Redeclare = SCode::Redeclare::NOT_REDECLARE;
                    let mut rpp: Arc<SCode::Replaceable> = Arc::new(SCode::Replaceable::NOT_REPLACEABLE);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses((name.clone()).clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES, metamodelica::nil())?) {
                        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    env = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(NFSCodeEnv::getItemPrefixes(item.clone())?) {
                        Deref @ SCode::Prefixes { replaceablePrefix: __pa2, redeclarePrefix: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rpp = __pa2.clone();
                    rdp = __pa3.clone();
                    (item, env) = lookupRedeclaredClass2(item.clone(), rdp.clone(), rpp.clone(), env.clone(), inInfo.clone())?;
                    Ok((item.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.lookupRedeclaredClassByItem failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outEnv))
}

fn lookupRedeclaredClass2(mut inItem: Item, mut inRedeclarePrefix: SCode::Redeclare, mut inReplaceablePrefix: Arc<SCode::Replaceable>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outEnv) = 'mc: {
        let __mc_input = (inItem.clone(), inRedeclarePrefix.clone(), inReplaceablePrefix.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Redeclare::NOT_REDECLARE, Deref @ SCode::Replaceable::REPLACEABLE { .. }, _, _) => {
                    Ok((inItem.clone(), inEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, .. }, .. }, SCode::Redeclare::REDECLARE, Deref @ SCode::Replaceable::REPLACEABLE { .. }, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut rdp: SCode::Redeclare = SCode::Redeclare::NOT_REDECLARE;
                    let mut rpp: Arc<SCode::Replaceable> = Arc::new(SCode::Replaceable::NOT_REPLACEABLE);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses((name.clone()).clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES, metamodelica::nil())?) {
                        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    env = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(NFSCodeEnv::getItemPrefixes(item.clone())?) {
                        Deref @ SCode::Prefixes { replaceablePrefix: __pa2, redeclarePrefix: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rpp = __pa2.clone();
                    rdp = __pa3.clone();
                    (item, env) = lookupRedeclaredClass2(item.clone(), rdp.clone(), rpp.clone(), env.clone(), inInfo.clone())?;
                    Ok((item.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, declaredEnv: env }, _, _, _, _) => {
                    let mut item = (*item).clone();
                    let mut env = (*env).clone();
                    (item, env) = lookupRedeclaredClass2(item.clone(), inRedeclarePrefix.clone(), inReplaceablePrefix.clone(), env.clone(), inInfo.clone())?;
                    Ok((item.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { info, name, .. }, .. }, _, Deref @ SCode::Replaceable::NOT_REPLACEABLE, _, _) => {
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), inInfo.clone())?;
                    Error::addSourceMessage(Error::REDECLARE_NON_REPLACEABLE.clone(), list![(name.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { info, name, .. }, .. }, _, _, _, _) => {
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), inInfo.clone())?;
                    Error::addSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(literal!("component")).clone(), (name.clone()).clone(), (literal!("a class")).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.lookupRedeclaredClass2 failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outEnv))
}

pub fn lookupBuiltinType(mut inName: ArcStr) -> Result<Item> {
    let mut outItem: Item;
    outItem = (::match_deref::match_deref! { match &(inName.clone()) {
        Deref @ "Real" => BUILTIN_REAL.clone(),
        Deref @ "Integer" => BUILTIN_INTEGER.clone(),
        Deref @ "Boolean" => BUILTIN_BOOLEAN.clone(),
        Deref @ "String" => BUILTIN_STRING.clone(),
        Deref @ "StateSelect" => BUILTIN_STATESELECT.clone(),
        Deref @ "ExternalObject" => BUILTIN_EXTERNALOBJECT.clone(),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            BUILTIN_CLOCK.clone()
        },
        Deref @ "$RealType" => BUILTIN_REALTYPE_ITEM.clone(),
        Deref @ "$IntegerType" => BUILTIN_INTEGERTYPE_ITEM.clone(),
        Deref @ "$BooleanType" => BUILTIN_BOOLEANTYPE_ITEM.clone(),
        Deref @ "$StringType" => BUILTIN_STRINGTYPE_ITEM.clone(),
        Deref @ "$EnumType" => BUILTIN_ENUMTYPE_ITEM.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outItem)
}

fn lookupBuiltinName(mut inName: Arc<Absyn::Path>) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outEnv) = (::match_deref::match_deref! { match &(inName.clone()) {
        Deref @ Absyn::Path::IDENT { name: id } => {
            let mut item: Item;
            item = lookupBuiltinType((id.clone()).clone())?;
            (item.clone(), NFSCodeEnv::emptyEnv.clone())
        },
        Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: id }, name: Deref @ "StateSelect" } => {
            let mut item: Item;
            (item, _) = lookupInClass((id.clone()).clone(), BUILTIN_STATESELECT_ENV.clone())?;
            (item.clone(), BUILTIN_STATESELECT_ENV.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outEnv))
}

fn lookupName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inLookupStrategy: LookupStrategy, mut inInfo: SourceInfo, mut inErrorType: Option<ErrorTypes::Message>) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outName, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inEnv.clone(), inLookupStrategy.clone(), inInfo.clone(), inErrorType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, LookupStrategy::LOOKUP_ANY { .. }, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    (item, env) = lookupBuiltinName(inName.clone())?;
                    Ok((item.clone(), inName.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: id }, _, _, _, _) => {
                    let mut item: Item;
                    let mut new_path: Arc<Absyn::Path>;
                    let mut env: Env = metamodelica::nil();
                    (item, new_path, env) = lookupSimpleName((id.clone()).clone(), inEnv.clone())?;
                    Ok((item.clone(), new_path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::QUALIFIED { path, name: id }, _, _, _, _) => {
                    let mut item: Item;
                    let mut new_path: Arc<Absyn::Path>;
                    let mut env: Env = metamodelica::nil();
                    let mut path = (*path).clone();
                    (item, new_path, env) = lookupSimpleName((id.clone()).clone(), inEnv.clone())?;
                    (item, path, env) = lookupNameInItem(path.clone(), item.clone(), env.clone())?;
                    path = joinPaths(new_path.clone(), path.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _, _, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut path = (*path).clone();
                    (item, path, env) = lookupFullyQualified(path.clone(), inEnv.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Some(error_id)) => {
                    let mut name_str: ArcStr = arcstr::literal!("");
                    let mut env_str: ArcStr = arcstr::literal!("");
                    name_str = (AbsynUtil::pathString(inName.clone(), (literal!(".")).clone(), true, false)?).clone();
                    env_str = (NFSCodeEnv::getEnvName(inEnv.clone())?).clone();
                    Error::addSourceMessage(error_id.clone(), list![(name_str.clone()).clone(), (env_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outName, outEnv))
}

fn joinPaths(mut inPath1: Arc<Absyn::Path>, mut inPath2: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath1.clone(), inPath2.clone())) {
        (_, Deref @ Absyn::Path::FULLYQUALIFIED { .. }) => {
            inPath2.clone()
        },
        (Deref @ Absyn::Path::IDENT { name: id }, _) => {
            Arc::new(Absyn::Path::QUALIFIED { name: (id.clone()).clone(), path: inPath2.clone() })
        },
        (Deref @ Absyn::Path::QUALIFIED { path, name: id }, _) => {
            let mut path = (*path).clone();
            path = joinPaths(path.clone(), inPath2.clone())?;
            Arc::new(Absyn::Path::QUALIFIED { name: (id.clone()).clone(), path: path.clone() })
        },
        (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _) => {
            let mut path = (*path).clone();
            path = joinPaths(path.clone(), inPath2.clone())?;
            AbsynUtil::makeFullyQualified(path.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn lookupNameSilent(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outName, outEnv) = lookupName(inName.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY, inInfo.clone(), None)?;
    Ok((outItem, outName, outEnv))
}

pub fn lookupNameSilentNoBuiltin(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outName, outEnv) = lookupName(inName.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo.clone(), None)?;
    Ok((outItem, outName, outEnv))
}

pub fn lookupClassName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outName, outEnv) = lookupName(inName.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY, inInfo.clone(), Some(Error::LOOKUP_ERROR.clone()))?;
    Ok((outItem, outName, outEnv))
}

pub fn lookupBaseClassName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outName, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ Absyn::Path::QUALIFIED { path: path @ Deref @ Absyn::Path::IDENT { name: id }, name: Deref @ "$ce" }, Deref @ metamodelica::List::Cons { head: _, tail: env }, _) => {
            let mut item: Item;
            let mut env = (*env).clone();
            (item, env) = lookupInheritedName((id.clone()).clone(), env.clone())?;
            (item.clone(), path.clone(), env.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", .. }, _, _) => {
            NFEnvExtends::printExtendsError(inName.clone(), inEnv.clone(), inInfo.clone())?;
            bail!("fail")
        },
        _ => {
            let mut env: Env = metamodelica::nil();
            let mut item: Item;
            let mut path: Arc<Absyn::Path>;
            (item, path, env) = lookupName(inName.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY, inInfo.clone(), Some(Error::LOOKUP_BASECLASS_ERROR.clone()))?;
            (item.clone(), path.clone(), env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outItem, outName, outEnv))
}

pub fn lookupVariableName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outName, outEnv) = lookupName(inName.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo.clone(), Some(Error::LOOKUP_VARIABLE_ERROR.clone()))?;
    Ok((outItem, outName, outEnv))
}

pub fn lookupFunctionName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outName, outEnv) = lookupName(inName.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo.clone(), Some(Error::LOOKUP_FUNCTION_ERROR.clone()))?;
    Ok((outItem, outName, outEnv))
}

fn crefStripEnvPrefix(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else { bail!("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut env_path: Arc<Absyn::Path>;
                    let mut cref1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut cref2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let false = (Flags::isSet(Flags::SCODE_INST.clone())?) else { bail!("pattern mismatch") };
                    env_path = NFSCodeEnv::getEnvPath(inEnv.clone())?;
                    cref1 = AbsynUtil::unqualifyCref(inCref.clone());
                    cref2 = crefStripEnvPrefix2(cref1.clone(), env_path.clone())?;
                    let false = (AbsynUtil::crefEqual(cref1.clone(), cref2.clone())) else { bail!("pattern mismatch") };
                    Ok(cref2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn crefStripEnvPrefix2(mut inCref: Arc<Absyn::ComponentRef>, mut inEnvPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), inEnvPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cref, subscripts: Deref @ metamodelica::List::Nil, name: id1 }, Deref @ Absyn::Path::QUALIFIED { path: env_path, name: id2 }) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(crefStripEnvPrefix2(cref.clone(), env_path.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cref, subscripts: Deref @ metamodelica::List::Nil, name: id1 }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { subscripts: Deref @ metamodelica::List::Nil, name: id1, .. }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
                    let false = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

pub fn lookupComponentRef(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, subscripts: Deref @ metamodelica::List::Nil, name: Deref @ "StateSelect" }, _, _) => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::WILD, _, _) => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    cref = NFSCodeFlattenImports::flattenComponentRefSubs(inCref.clone(), inEnv.clone(), inInfo.clone())?;
                    (cref, _) = lookupComponentRef2(cref.clone(), inEnv.clone())?;
                    cref = crefStripEnvPrefix(cref.clone(), inEnv.clone())?;
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

fn lookupComponentRef2(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env) -> Result<(Arc<Absyn::ComponentRef>, Env)> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut outEnv: Env = metamodelica::nil();
    (outCref, outEnv) = (::match_deref::match_deref! { match &((inCref.clone(), inEnv.clone())) {
        (Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs }, _) => {
            let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut path: Arc<Absyn::Path>;
            let mut env: Env = metamodelica::nil();
            (_, path, env) = lookupSimpleName((name.clone()).clone(), inEnv.clone())?;
            cref = AbsynUtil::pathToCrefWithSubs(path.clone(), subs.clone())?;
            (cref.clone(), env.clone())
        },
        (Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts: subs, componentRef: rest_cref }, _) => {
            let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut new_path: Arc<Absyn::Path>;
            let mut env: Env = metamodelica::nil();
            let mut item: Item;
            let mut rest_cref = (*rest_cref).clone();
            (item, new_path, env) = lookupSimpleName((name.clone()).clone(), inEnv.clone())?;
            cref = AbsynUtil::pathToCrefWithSubs(new_path.clone(), subs.clone())?;
            (item, rest_cref) = lookupCrefInItem(rest_cref.clone(), item.clone(), env.clone())?;
            cref = joinCrefs(cref.clone(), rest_cref.clone())?;
            (cref.clone(), env.clone())
        },
        (Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref }, _) => {
            let mut env: Env = metamodelica::nil();
            let mut cref = (*cref).clone();
            cref = lookupCrefFullyQualified(cref.clone(), inEnv.clone())?;
            env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
            (cref.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCref, outEnv))
}

pub fn lookupCrefFullyQualified(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut env: Env = metamodelica::nil();
    env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
    (_, outCref) = lookupCrefInPackage(inCref.clone(), inEnv.clone())?;
    outCref = AbsynUtil::crefMakeFullyQualified(outCref.clone());
    Ok(outCref)
}

pub fn joinCrefs(mut inCref1: Arc<Absyn::ComponentRef>, mut inCref2: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCref = (::match_deref::match_deref! { match &((inCref1.clone(), inCref2.clone())) {
        (_, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }) => inCref2.clone(),
        _ => AbsynUtil::joinCrefs(inCref1.clone(), inCref2.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

pub fn lookupTypeSpec(mut inTypeSpec: Arc<Absyn::TypeSpec>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::TypeSpec>, Env)> {
    let mut outItem: Item;
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    let mut outTypeEnv: Env = metamodelica::nil();
    (outItem, outTypeSpec, outTypeEnv) = (::match_deref::match_deref! { match &((inTypeSpec.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: ad }, _, _) => {
            let mut newpath: Arc<Absyn::Path>;
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            (item, newpath, env) = lookupClassName(path.clone(), inEnv.clone(), inInfo.clone())?;
            (item.clone(), Arc::new(Absyn::TypeSpec::TPATH { path: newpath.clone(), arrayDim: ad.clone() }), env.clone())
        },
        (Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name }, .. }, _, _) => {
            let mut cls: Arc<SCode::Element>;
            cls = makeDummyMetaType((name.clone()).clone());
            (Arc::new(NFSCodeEnv::Item::CLASS { cls: cls.clone(), env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }), inTypeSpec.clone(), NFSCodeEnv::emptyEnv.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outTypeSpec, outTypeEnv))
}

fn makeDummyMetaType(mut inTypeName: ArcStr) -> Arc<SCode::Element> {
    let mut outClass: Arc<SCode::Element>;
    outClass = Arc::new(SCode::Element::CLASS { name: (inTypeName.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() });
    outClass
}

pub fn qualifyPath(mut inPath: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo, mut inErrorType: Option<ErrorTypes::Message>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = (inPath.clone(), inEnv.clone(), inInfo.clone(), inErrorType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: id }, _, _, _) => {
                    let _ = lookupBuiltinType((id.clone()).clone())?;
                    Ok(inPath.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut env: Env = metamodelica::nil();
                    (_, path, env) = lookupName(inPath.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo.clone(), inErrorType.clone())?;
                    path = NFSCodeEnv::mergePathWithEnvPath(path.clone(), env.clone())?;
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.qualifyPath failed on ")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

