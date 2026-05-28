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

use crate::FCore;
use crate::FGraph;
use crate::FGraphBuild;
use crate::Parser;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::MetaUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;

// protected imports
/* These imports were used in e.g. MSL 1.6. They should not be here anymore...
   If you need them, add them to the initial environment and recompile; they are not standard Modelica.
  import arcsin = asin;
  import arccos = acos;
  import arctan = atan;
  import ln = log;
*/
// Predefined DAE.Types
// Real arrays
thread_local! { static __T_REAL_ARRAY_DEFAULT_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }); }
pub fn T_REAL_ARRAY_DEFAULT() -> Arc<DAE::Type> { __T_REAL_ARRAY_DEFAULT_TLS.with(|__t| __t.clone()) }

thread_local! { static __T_REAL_ARRAY_1_DEFAULT_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] }); }
pub fn T_REAL_ARRAY_1_DEFAULT() -> Arc<DAE::Type> { __T_REAL_ARRAY_1_DEFAULT_TLS.with(|__t| __t.clone()) }

// Integer arrays
thread_local! { static __T_INT_ARRAY_1_DEFAULT_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] }); }
pub fn T_INT_ARRAY_1_DEFAULT() -> Arc<DAE::Type> { __T_INT_ARRAY_1_DEFAULT_TLS.with(|__t| __t.clone()) }

pub static commonPrefixes: std::sync::LazyLock<Arc<SCode::Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Prefixes { visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE, finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(openmodelica_frontend_types::SCode::Replaceable::NOT_REPLACEABLE) }) });

pub static commonPrefixesNotFinal: std::sync::LazyLock<Arc<SCode::Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Prefixes { visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(openmodelica_frontend_types::SCode::Replaceable::NOT_REPLACEABLE) }) });

pub static attrConst: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| { SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static attrParam: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| { SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static attrParamVectorNoDim: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| { SCode::Attributes { arrayDims: list![Arc::new(openmodelica_ast::Absyn::Subscript::NOSUB)], connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

//
// The primitive types
// These are the primitive types that are used to build the types
// Real, Integer etc.
pub static rlType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("RealType")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static intType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("IntegerType")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static strType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("StringType")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static boolType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("BooleanType")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static enumType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("EnumType")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static unit: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("unit")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("StringType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static quantity: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("quantity")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("StringType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static displayUnit: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("displayUnit")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("StringType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static min: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("min")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("RealType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::REAL { value: (literal!("-1e+099")).clone() })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static max: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("max")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("RealType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::REAL { value: (literal!("1e+099")).clone() })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static startOrigin: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("startOrigin")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("StringType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::STRING { value: (literal!("undefined")).clone() })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static realStart: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("RealType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::REAL { value: (literal!("0.0")).clone() })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static integerStart: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("IntegerType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::INTEGER { value: 0 })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static stringStart: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("StringType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static booleanStart: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("BooleanType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::BOOL { value: false })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static fixed: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("fixed")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("BooleanType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::BOOL { value: false })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static nominal: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("nominal")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("RealType")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: None, comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static stateSelect: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("stateSelect")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("StateSelect")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("StateSelect")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("default")).clone(), subscripts: metamodelica::nil() }) }) })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Extensions for uncertainties
pub static uncertainty: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("uncertain")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Uncertainty")).clone() }), arrayDim: None }), modifications: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("Uncertainty")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("given")).clone(), subscripts: metamodelica::nil() }) }) })), comment: None, info: Absyn::dummyInfo.clone() }), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static distribution: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("distribution")).clone(), prefixes: commonPrefixes.clone(), attributes: attrParam.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Distribution")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Distribution is declared in ModelicaBuiltin.mo
// END Extensions for uncertainties
pub static stateSelectComps: std::sync::LazyLock<Arc<metamodelica::List<Arc<SCode::Element>>>> = std::sync::LazyLock::new(|| { list![Arc::new(SCode::Element::COMPONENT { name: (literal!("never")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(SCode::Element::COMPONENT { name: (literal!("avoid")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(SCode::Element::COMPONENT { name: (literal!("default")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(SCode::Element::COMPONENT { name: (literal!("prefer")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(SCode::Element::COMPONENT { name: (literal!("always")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() })] });

pub static uncertaintyComps: std::sync::LazyLock<Arc<metamodelica::List<Arc<SCode::Element>>>> = std::sync::LazyLock::new(|| { list![Arc::new(SCode::Element::COMPONENT { name: (literal!("given")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(SCode::Element::COMPONENT { name: (literal!("sought")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(SCode::Element::COMPONENT { name: (literal!("refine")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(SCode::Element::COMPONENT { name: (literal!("propagate")).clone(), prefixes: commonPrefixes.clone(), attributes: attrConst.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() })] });

pub static stateSelectType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("StateSelect")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: stateSelectComps.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static uncertaintyType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("Uncertainty")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: uncertaintyComps.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub static ExternalObjectType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("ExternalObject")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

// The Real type
pub static realType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("Real")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: list![unit.clone(), quantity.clone(), displayUnit.clone(), (std::sync::Arc::new(fnptr!(min)) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<()> + 'static>), (std::sync::Arc::new(fnptr!(max)) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<()> + 'static>), realStart.clone(), fixed.clone(), nominal.clone(), stateSelect.clone(), uncertainty.clone(), distribution.clone(), startOrigin.clone()], normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

// The Integer type
pub static integerType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("Integer")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: list![quantity.clone(), (std::sync::Arc::new(fnptr!(min)) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<()> + 'static>), (std::sync::Arc::new(fnptr!(max)) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<()> + 'static>), integerStart.clone(), fixed.clone(), uncertainty.clone(), distribution.clone(), startOrigin.clone()], normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

// The String type
pub static stringType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("String")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: list![quantity.clone(), stringStart.clone(), startOrigin.clone()], normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

// The Boolean type
pub static booleanType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("Boolean")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: list![quantity.clone(), booleanStart.clone(), fixed.clone(), startOrigin.clone()], normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

// BTH The Clock type
pub static clockType: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("Clock")).clone(), prefixes: commonPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_CLOCK, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

// The builtin variable time. See also variableIsBuiltin
thread_local! { static __timeVar_TLS: Arc<DAE::Var> = Arc::new(DAE::Var { name: (literal!("time")).clone(), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }); }
pub fn timeVar() -> Arc<DAE::Var> { __timeVar_TLS.with(|__t| __t.clone()) }

/* Optimica Extensions. Theses variables are considered builtin for Optimica: startTime, finalTime, objectiveIntegrand and objective */
/* Optimica Extensions. The builtin variable startTime. */
thread_local! { static __startTimeVar_TLS: Arc<DAE::Var> = Arc::new(DAE::Var { name: (literal!("startTime")).clone(), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }); }
pub fn startTimeVar() -> Arc<DAE::Var> { __startTimeVar_TLS.with(|__t| __t.clone()) }

/* Optimica Extensions. The builtin variable finalTime. */
thread_local! { static __finalTimeVar_TLS: Arc<DAE::Var> = Arc::new(DAE::Var { name: (literal!("finalTime")).clone(), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }); }
pub fn finalTimeVar() -> Arc<DAE::Var> { __finalTimeVar_TLS.with(|__t| __t.clone()) }

/* Optimica Extensions. The builtin variable objectiveIntegrand. */
thread_local! { static __objectiveIntegrandVar_TLS: Arc<DAE::Var> = Arc::new(DAE::Var { name: (literal!("objectiveIntegrand")).clone(), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }); }
pub fn objectiveIntegrandVar() -> Arc<DAE::Var> { __objectiveIntegrandVar_TLS.with(|__t| __t.clone()) }

/* Optimica Extensions. The builtin variable objective. */
thread_local! { static __objectiveVar_TLS: Arc<DAE::Var> = Arc::new(DAE::Var { name: (literal!("objective")).clone(), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }); }
pub fn objectiveVar() -> Arc<DAE::Var> { __objectiveVar_TLS.with(|__t| __t.clone()) }

thread_local! { static __argRealX_TLS: Arc<DAE::FuncArg> = Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }); }
pub fn argRealX() -> Arc<DAE::FuncArg> { __argRealX_TLS.with(|__t| __t.clone()) }

thread_local! { static __argRealY_TLS: Arc<DAE::FuncArg> = Arc::new(DAE::FuncArg { name: (literal!("y")).clone(), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }); }
pub fn argRealY() -> Arc<DAE::FuncArg> { __argRealY_TLS.with(|__t| __t.clone()) }

thread_local! { static __argRealZ_TLS: Arc<DAE::FuncArg> = Arc::new(DAE::FuncArg { name: (literal!("z")).clone(), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }); }
pub fn argRealZ() -> Arc<DAE::FuncArg> { __argRealZ_TLS.with(|__t| __t.clone()) }

thread_local! { static __argsRealX_TLS: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = list![argRealX().clone()]; }
pub fn argsRealX() -> Arc<metamodelica::List<Arc<DAE::FuncArg>>> { __argsRealX_TLS.with(|__t| __t.clone()) }

thread_local! { static __argsRealXY_TLS: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = list![argRealX().clone(), argRealY().clone()]; }
pub fn argsRealXY() -> Arc<metamodelica::List<Arc<DAE::FuncArg>>> { __argsRealXY_TLS.with(|__t| __t.clone()) }

thread_local! { static __argsRealXYZ_TLS: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = list![argRealX().clone(), argRealY().clone(), argRealZ().clone()]; }
pub fn argsRealXYZ() -> Arc<metamodelica::List<Arc<DAE::FuncArg>>> { __argsRealXYZ_TLS.with(|__t| __t.clone()) }

pub static timeComp: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("time")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static startTimeComp: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("startTime")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static finalTimeComp: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("finalTime")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static objectiveIntegrandComp: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("objectiveIntegrand")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static objectiveVarComp: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("objectiveVar")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub static basicTypes: std::sync::LazyLock<Arc<metamodelica::List<Arc<SCode::Element>>>> = std::sync::LazyLock::new(|| { list![clockType.clone(), rlType.clone(), intType.clone(), strType.clone(), boolType.clone(), enumType.clone(), ExternalObjectType.clone(), realType.clone(), integerType.clone(), stringType.clone(), booleanType.clone(), uncertaintyType.clone()] });

pub static basicTypesNF: std::sync::LazyLock<Arc<metamodelica::List<Arc<SCode::Element>>>> = std::sync::LazyLock::new(|| { list![rlType.clone(), intType.clone(), strType.clone(), boolType.clone(), enumType.clone(), realType.clone(), integerType.clone(), stringType.clone(), booleanType.clone()] });

pub fn getBasicTypes() -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut tys: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    tys = if (Flags::isSet(Flags::SCODE_INST.clone())?) {basicTypesNF.clone()} else {basicTypes.clone()};
    Ok(tys)
}

pub fn variableIsBuiltin(mut cref: Arc<DAE::ComponentRef>, mut useOptimica: bool) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((cref.clone(), useOptimica.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, _) => true,
        (_, false) => false,
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "startTime", .. }, true) => true,
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "finalTime", .. }, true) => true,
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "objective", .. }, true) => true,
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "objectiveIntegrand", .. }, true) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isDer(mut inPath: Arc<Absyn::Path>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "der" } => {
            ()
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path } => {
            isDer(path.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn mergePrograms(mut program1: Absyn::Program, mut program2: Absyn::Program) -> Absyn::Program {
    let mut outProgram: Absyn::Program = program1.clone();
    outProgram.classes = listAppend(program1.classes.clone(), program2.classes.clone());
    outProgram
}

pub fn getInitialFunctions() -> Result<(Absyn::Program, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut initialProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut initialSCodeProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut fileModelicaNF: ArcStr = arcstr::literal!("");
    let mut fileModelicaCF: ArcStr = arcstr::literal!("");
    let mut fileMetaModelica: ArcStr = arcstr::literal!("");
    let mut fileParModelica: ArcStr = arcstr::literal!("");
    let mut filePDEModelica: ArcStr = arcstr::literal!("");
    let mut assocLst: Arc<metamodelica::List<((i32, bool), (Absyn::Program, Arc<metamodelica::List<Arc<SCode::Element>>>))>> = metamodelica::nil();
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut pNF: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut pCF: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut pMM: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut spNF: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut spCF: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    fileModelicaNF = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/omc/NFModelicaBuiltin.mo")); ArcStr::from(__mm_s) }).clone();
    fileModelicaCF = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/omc/ModelicaBuiltin.mo")); ArcStr::from(__mm_s) }).clone();
    fileMetaModelica = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/omc/MetaModelicaBuiltin.mo")); ArcStr::from(__mm_s) }).clone();
    fileParModelica = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/omc/ParModelicaBuiltin.mo")); ArcStr::from(__mm_s) }).clone();
    filePDEModelica = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/omc/PDEModelicaBuiltin.mo")); ArcStr::from(__mm_s) }).clone();
    (initialProgram, initialSCodeProgram) = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                let _ = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = metamodelica::nil());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = sp.clone();
            let mut p: Absyn::Program = p.clone();
            let mut assocLst: Arc<metamodelica::List<((i32, bool), (Absyn::Program, Arc<metamodelica::List<Arc<SCode::Element>>>))>> = assocLst.clone();
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            (p, sp) = Util::assoc(Util::makeTuple(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::isSet(Flags::SCODE_INST.clone())?), assocLst.clone())?;
            Ok((p.clone(), sp.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut spNF: Arc<metamodelica::List<Arc<SCode::Element>>> = spNF.clone();
            let mut assocLst: Arc<metamodelica::List<((i32, bool), (Absyn::Program, Arc<metamodelica::List<Arc<SCode::Element>>>))>> = assocLst.clone();
            let mut spCF: Arc<metamodelica::List<Arc<SCode::Element>>> = spCF.clone();
            let mut pNF: Absyn::Program = pNF.clone();
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = sp.clone();
            let mut p: Absyn::Program = p.clone();
            let mut pCF: Absyn::Program = pCF.clone();
            let mut pMM: Absyn::Program = pMM.clone();
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::METAMODELICA.clone())) else { bail!("pattern mismatch") };
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileModelicaNF.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileModelicaNF.clone()).clone()], Absyn::dummyInfo.clone())?;
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileModelicaCF.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileModelicaCF.clone()).clone()], Absyn::dummyInfo.clone())?;
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileMetaModelica.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileMetaModelica.clone()).clone()], Absyn::dummyInfo.clone())?;
            pNF = Parser::parse((fileModelicaNF.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pCF = Parser::parse((fileModelicaCF.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pMM = Parser::parse((fileMetaModelica.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pNF = mergePrograms(pNF.clone(), pMM.clone());
            pCF = mergePrograms(pCF.clone(), pMM.clone());
            pNF = MetaUtil::createMetaClassesInProgram(pNF.clone())?;
            pCF = MetaUtil::createMetaClassesInProgram(pCF.clone())?;
            spNF = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (pNF.classes.clone()).into_iter().cloned() {
            let __x = AbsynToSCode::translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            spCF = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (pCF.classes.clone()).into_iter().cloned() {
            let __x = AbsynToSCode::translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = cons(((Flags::METAMODELICA.clone(), true), (pNF.clone(), spNF.clone())), cons(((Flags::METAMODELICA.clone(), false), (pCF.clone(), spCF.clone())), assocLst.clone())));
            (p, sp) = if (Flags::isSet(Flags::SCODE_INST.clone())?) {(pNF.clone(), spNF.clone())} else {(pCF.clone(), spCF.clone())};
            Ok((p.clone(), sp.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut spNF: Arc<metamodelica::List<Arc<SCode::Element>>> = spNF.clone();
            let mut pMM: Absyn::Program = pMM.clone();
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = sp.clone();
            let mut p: Absyn::Program = p.clone();
            let mut spCF: Arc<metamodelica::List<Arc<SCode::Element>>> = spCF.clone();
            let mut pNF: Absyn::Program = pNF.clone();
            let mut pCF: Absyn::Program = pCF.clone();
            let mut assocLst: Arc<metamodelica::List<((i32, bool), (Absyn::Program, Arc<metamodelica::List<Arc<SCode::Element>>>))>> = assocLst.clone();
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PARMODELICA.clone())) else { bail!("pattern mismatch") };
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileModelicaNF.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileModelicaNF.clone()).clone()], Absyn::dummyInfo.clone())?;
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileModelicaCF.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileModelicaCF.clone()).clone()], Absyn::dummyInfo.clone())?;
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileMetaModelica.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileMetaModelica.clone()).clone()], Absyn::dummyInfo.clone())?;
            pNF = Parser::parse((fileModelicaNF.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pCF = Parser::parse((fileModelicaCF.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pMM = Parser::parse((fileParModelica.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pNF = mergePrograms(pNF.clone(), pMM.clone());
            pCF = mergePrograms(pCF.clone(), pMM.clone());
            spNF = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (pNF.classes.clone()).into_iter().cloned() {
            let __x = AbsynToSCode::translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            spCF = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (pCF.classes.clone()).into_iter().cloned() {
            let __x = AbsynToSCode::translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = cons(((Flags::PARMODELICA.clone(), true), (pNF.clone(), spNF.clone())), cons(((Flags::PARMODELICA.clone(), false), (pCF.clone(), spCF.clone())), assocLst.clone())));
            (p, sp) = if (Flags::isSet(Flags::SCODE_INST.clone())?) {(pNF.clone(), spNF.clone())} else {(pCF.clone(), spCF.clone())};
            Ok((p.clone(), sp.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut pNF: Absyn::Program = pNF.clone();
            let mut pCF: Absyn::Program = pCF.clone();
            let mut spCF: Arc<metamodelica::List<Arc<SCode::Element>>> = spCF.clone();
            let mut spNF: Arc<metamodelica::List<Arc<SCode::Element>>> = spNF.clone();
            let mut assocLst: Arc<metamodelica::List<((i32, bool), (Absyn::Program, Arc<metamodelica::List<Arc<SCode::Element>>>))>> = assocLst.clone();
            let mut p: Absyn::Program = p.clone();
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = sp.clone();
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::MODELICA.clone()) || intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::OPTIMICA.clone())) else { bail!("pattern mismatch") };
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileModelicaNF.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileModelicaNF.clone()).clone()], Absyn::dummyInfo.clone())?;
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileModelicaCF.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileModelicaCF.clone()).clone()], Absyn::dummyInfo.clone())?;
            pNF = Parser::parse((fileModelicaNF.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pCF = Parser::parse((fileModelicaCF.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            spNF = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (pNF.classes.clone()).into_iter().cloned() {
            let __x = AbsynToSCode::translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            spCF = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (pCF.classes.clone()).into_iter().cloned() {
            let __x = AbsynToSCode::translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = cons(((Flags::MODELICA.clone(), true), (pNF.clone(), spNF.clone())), cons(((Flags::MODELICA.clone(), false), (pCF.clone(), spCF.clone())), assocLst.clone())));
            (p, sp) = if (Flags::isSet(Flags::SCODE_INST.clone())?) {(pNF.clone(), spNF.clone())} else {(pCF.clone(), spCF.clone())};
            Ok((p.clone(), sp.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut spNF: Arc<metamodelica::List<Arc<SCode::Element>>> = spNF.clone();
            let mut assocLst: Arc<metamodelica::List<((i32, bool), (Absyn::Program, Arc<metamodelica::List<Arc<SCode::Element>>>))>> = assocLst.clone();
            let mut p: Absyn::Program = p.clone();
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = sp.clone();
            let mut pNF: Absyn::Program = pNF.clone();
            let mut pCF: Absyn::Program = pCF.clone();
            let mut spCF: Arc<metamodelica::List<Arc<SCode::Element>>> = spCF.clone();
            let mut pMM: Absyn::Program = pMM.clone();
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone())) else { bail!("pattern mismatch") };
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileModelicaNF.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileModelicaNF.clone()).clone()], Absyn::dummyInfo.clone())?;
            Error::assertionOrAddSourceMessage(System::regularFileExists((fileModelicaCF.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileModelicaCF.clone()).clone()], Absyn::dummyInfo.clone())?;
            Error::assertionOrAddSourceMessage(System::regularFileExists((filePDEModelica.clone()).clone()), Error::FILE_NOT_FOUND_ERROR.clone(), list![(filePDEModelica.clone()).clone()], Absyn::dummyInfo.clone())?;
            pNF = Parser::parse((fileModelicaNF.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pCF = Parser::parse((fileModelicaCF.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pMM = Parser::parse((filePDEModelica.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Flags::METAMODELICA.clone(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            pNF = mergePrograms(pNF.clone(), pMM.clone());
            pCF = mergePrograms(pCF.clone(), pMM.clone());
            spNF = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (pNF.classes.clone()).into_iter().cloned() {
            let __x = AbsynToSCode::translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            spCF = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (pCF.classes.clone()).into_iter().cloned() {
            let __x = AbsynToSCode::translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = cons(((Flags::PDEMODELICA.clone(), true), (pNF.clone(), spNF.clone())), cons(((Flags::PDEMODELICA.clone(), false), (pCF.clone(), spCF.clone())), assocLst.clone())));
            (p, sp) = if (Flags::isSet(Flags::SCODE_INST.clone())?) {(pNF.clone(), spNF.clone())} else {(pCF.clone(), spCF.clone())};
            Ok((p.clone(), sp.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("FBuiltin.getInitialFunctions failed.")).clone(), metamodelica::sourceInfo!())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((initialProgram, initialSCodeProgram))
}

pub fn initialGraph(mut inCache: FCore::Cache) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut graph: FCore::Graph;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    (outCache, graph) = 'mc: {
        let __mc_input = inCache.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else { bail!("nomatch") };
            let mut graph: FCore::Graph;
            graph = FCore::getCachedInitialGraph(cache.clone())?;
            Ok((cache.clone(), graph.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else { bail!("nomatch") };
            let mut graph: FCore::Graph;
            graph = getSetInitialGraph(None)?;
            Ok((cache.clone(), graph.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else { bail!("nomatch") };
            let mut initialProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut graph: FCore::Graph;
            graph = FGraph::new((literal!("graph")).clone(), FCore::dummyTopModel.clone())?;
            graph = FGraphBuild::mkProgramGraph(basicTypes.clone(), crate::FCore::Kind::BASIC_TYPE, graph.clone())?;
            graph = initialGraphOptimica(graph.clone(), (std::sync::Arc::new(FGraphBuild::mkCompNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>))?;
            graph = initialGraphMetaModelica(graph.clone(), (std::sync::Arc::new(FGraphBuild::mkTypeNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, metamodelica::Array<FCore::Node>, ArcStr, FCore::Graph) -> Result<FCore::Graph> + 'static>))?;
            graph = initialGraphModelica(graph.clone(), (std::sync::Arc::new(FGraphBuild::mkTypeNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, metamodelica::Array<FCore::Node>, ArcStr, FCore::Graph) -> Result<FCore::Graph> + 'static>), (std::sync::Arc::new(FGraphBuild::mkCompNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>))?;
            (_, initialProgram) = getInitialFunctions()?;
            graph = FGraphBuild::mkProgramGraph(initialProgram.clone(), crate::FCore::Kind::BUILTIN, graph.clone())?;
            cache = FCore::setCachedInitialGraph(cache.clone(), graph.clone());
            let _ = getSetInitialGraph(Some(graph.clone()))?;
            Ok((cache.clone(), graph.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, graph))
}

fn getSetInitialGraph(mut inEnvOpt: Option<FCore::Graph>) -> Result<FCore::Graph> {
    let mut initialEnv: FCore::Graph;
    initialEnv = 'mc: {
        let __mc_input = inEnvOpt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                let _ = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = metamodelica::nil());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let None = __mc_input.clone() else { bail!("nomatch") };
            let mut assocLst: Arc<metamodelica::List<(i32, FCore::Graph)>> = metamodelica::nil();
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            Ok(Util::assoc(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, assocLst.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let Some(mut graph) = __mc_input.clone() else { bail!("nomatch") };
            let mut assocLst: Arc<metamodelica::List<(i32, FCore::Graph)>> = metamodelica::nil();
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::METAMODELICA.clone())) else { bail!("pattern mismatch") };
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = cons((Flags::METAMODELICA.clone(), graph.clone()), assocLst.clone()));
            Ok(graph.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let Some(mut graph) = __mc_input.clone() else { bail!("nomatch") };
            let mut assocLst: Arc<metamodelica::List<(i32, FCore::Graph)>> = metamodelica::nil();
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PARMODELICA.clone())) else { bail!("pattern mismatch") };
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = cons((Flags::PARMODELICA.clone(), graph.clone()), assocLst.clone()));
            Ok(graph.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let Some(mut graph) = __mc_input.clone() else { bail!("nomatch") };
            let mut assocLst: Arc<metamodelica::List<(i32, FCore::Graph)>> = metamodelica::nil();
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::MODELICA.clone()) || intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::OPTIMICA.clone())) else { bail!("pattern mismatch") };
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = cons((Flags::MODELICA.clone(), graph.clone()), assocLst.clone()));
            Ok(graph.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(initialEnv)
}

pub type MakeTypeNode = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, metamodelica::Array<FCore::Node>, ArcStr, FCore::Graph) -> Result<FCore::Graph> + 'static>;

pub type MakeCompNode = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>;

pub fn initialGraphModelica(mut graph: FCore::Graph, mut mkTypeNode: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, metamodelica::Array<FCore::Node>, ArcStr, FCore::Graph) -> Result<FCore::Graph> + 'static>, mut mkCompNode: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>) -> Result<FCore::Graph> {
    let mut graph: FCore::Graph = graph;
    let enumeration2int: Arc<DAE::Type> = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Integer")).clone() }) });
    graph = mkCompNode(timeComp.clone(), FGraph::top(graph.clone())?, crate::FCore::Kind::BUILTIN, graph.clone())?;
    graph = FGraph::updateComp(graph.clone(), timeVar().clone(), crate::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
    graph = mkTypeNode(list![Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: Arc::new(DAE::Type::T_ANYTYPE { anyClassType: Some(ClassInf::State::CONNECTOR { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$dummy$")).clone() }), isExpandable: false }) }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("cardinality")).clone() }) }), Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: Arc::new(DAE::Type::T_ANYTYPE { anyClassType: Some(ClassInf::State::CONNECTOR { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$dummy$")).clone() }), isExpandable: true }) }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("cardinality")).clone() }) })], FGraph::top(graph.clone())?, (literal!("cardinality")).clone(), graph.clone())?;
    graph = mkTypeNode(list![enumeration2int.clone()], FGraph::top(graph.clone())?, (literal!("Integer")).clone(), graph.clone())?;
    graph = mkTypeNode(list![enumeration2int.clone()], FGraph::top(graph.clone())?, (literal!("EnumToInteger")).clone(), graph.clone())?;
    graph = mkTypeNode(list![Arc::new(DAE::Type::T_FUNCTION { funcArg: argsRealX().clone(), funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("noEvent")).clone() }) })], FGraph::top(graph.clone())?, (literal!("noEvent")).clone(), graph.clone())?;
    graph = mkTypeNode(list![Arc::new(DAE::Type::T_FUNCTION { funcArg: argsRealX().clone(), funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("actualStream")).clone() }) })], FGraph::top(graph.clone())?, (literal!("actualStream")).clone(), graph.clone())?;
    graph = mkTypeNode(list![Arc::new(DAE::Type::T_FUNCTION { funcArg: argsRealX().clone(), funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("inStream")).clone() }) })], FGraph::top(graph.clone())?, (literal!("inStream")).clone(), graph.clone())?;
    Ok(graph)
}

pub fn initialGraphMetaModelica(mut graph: FCore::Graph, mut mkTypeNode: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, metamodelica::Array<FCore::Node>, ArcStr, FCore::Graph) -> Result<FCore::Graph> + 'static>) -> Result<FCore::Graph> {
    let mut graph: FCore::Graph = graph;
    if !(Config::acceptMetaModelicaGrammar()?) {
        return Ok(graph);
    }
    graph = mkTypeNode(list![Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("index")).clone(), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_METABOXED_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("getGlobalRoot")).clone() }) })], FGraph::top(graph.clone())?, (literal!("getGlobalRoot")).clone(), graph.clone())?;
    Ok(graph)
}

pub fn initialGraphOptimica(mut graph: FCore::Graph, mut mkCompNode: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>) -> Result<FCore::Graph> {
    let mut graph: FCore::Graph = graph;
    if !(Config::acceptOptimicaGrammar()?) {
        return Ok(graph);
    }
    graph = mkCompNode(objectiveVarComp.clone(), FGraph::top(graph.clone())?, crate::FCore::Kind::BUILTIN, graph.clone())?;
    graph = FGraph::updateComp(graph.clone(), objectiveVar().clone(), crate::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
    graph = mkCompNode(objectiveIntegrandComp.clone(), FGraph::top(graph.clone())?, crate::FCore::Kind::BUILTIN, graph.clone())?;
    graph = FGraph::updateComp(graph.clone(), objectiveIntegrandVar().clone(), crate::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
    graph = mkCompNode(startTimeComp.clone(), FGraph::top(graph.clone())?, crate::FCore::Kind::BUILTIN, graph.clone())?;
    graph = FGraph::updateComp(graph.clone(), startTimeVar().clone(), crate::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
    graph = mkCompNode(finalTimeComp.clone(), FGraph::top(graph.clone())?, crate::FCore::Kind::BUILTIN, graph.clone())?;
    graph = FGraph::updateComp(graph.clone(), finalTimeVar().clone(), crate::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
    Ok(graph)
}

pub fn getElementWithPathCheckBuiltin(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = 'mc: {
        let __mc_input = (inProgram.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(SCodeUtil::getElementWithPath(inProgram.clone(), inPath.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    (_, sp) = getInitialFunctions()?;
                    Ok(SCodeUtil::getElementWithPath(sp.clone(), inPath.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElement)
}

