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

use crate::AbsynUtil;
use crate::SCodeDump;
use crate::SCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;

pub fn printStateStr(mut inState: ClassInf::State) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inState.clone() {
        ClassInf::State::UNKNOWN { .. } => {
            literal!("unknown")
        },
        ClassInf::State::OPTIMIZATION { .. } => {
            literal!("optimization")
        },
        ClassInf::State::MODEL { .. } => {
            literal!("model")
        },
        ClassInf::State::RECORD { .. } => {
            literal!("record")
        },
        ClassInf::State::BLOCK { .. } => {
            literal!("block")
        },
        ClassInf::State::CONNECTOR { .. } => {
            literal!("connector")
        },
        ClassInf::State::TYPE { .. } => {
            literal!("type")
        },
        ClassInf::State::PACKAGE { .. } => {
            literal!("package")
        },
        ClassInf::State::FUNCTION { isImpure: true, .. } => {
            literal!("impure function")
        },
        ClassInf::State::FUNCTION { .. } => {
            literal!("function")
        },
        ClassInf::State::TYPE_INTEGER { .. } => {
            literal!("Integer")
        },
        ClassInf::State::TYPE_REAL { .. } => {
            literal!("Real")
        },
        ClassInf::State::TYPE_STRING { .. } => {
            literal!("String")
        },
        ClassInf::State::TYPE_BOOL { .. } => {
            literal!("Boolean")
        },
        ClassInf::State::TYPE_CLOCK { .. } => {
            literal!("Clock")
        },
        ClassInf::State::HAS_RESTRICTIONS { hasConstraints: false, hasAlgorithms: false, hasEquations: false, .. } => {
            literal!("new def")
        },
        ClassInf::State::HAS_RESTRICTIONS { hasAlgorithms: mut b2, hasEquations: mut b1, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("has")); __mm_s.push_str(&*if (b1.clone()) {literal!(" equations")} else {literal!("")}); __mm_s.push_str(&*if (b2.clone()) {literal!(" algorithms")} else {literal!("")}); __mm_s.push_str(&*if (b1.clone()) {literal!(" constraints")} else {literal!("")}); ArcStr::from(__mm_s) }
        },
        ClassInf::State::EXTERNAL_OBJ { .. } => {
            literal!("ExternalObject")
        },
        ClassInf::State::META_TUPLE { .. } => {
            literal!("tuple")
        },
        ClassInf::State::META_LIST { .. } => {
            literal!("list")
        },
        ClassInf::State::META_OPTION { .. } => {
            literal!("Option")
        },
        ClassInf::State::META_RECORD { .. } => {
            literal!("meta_record")
        },
        ClassInf::State::META_POLYMORPHIC { .. } => {
            literal!("polymorphic")
        },
        ClassInf::State::META_ARRAY { .. } => {
            literal!("meta_array")
        },
        ClassInf::State::META_UNIONTYPE { .. } => {
            literal!("uniontype")
        },
        _ => {
            literal!("#printStateStr failed#")
        },
    })).clone();
    outString
}

pub fn printState(mut inState: ClassInf::State) -> Result<()> {
    let () = (match inState.clone() {
        ClassInf::State::UNKNOWN { path: ref p } => {
            Print::printBuf((literal!("UNKNOWN ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::OPTIMIZATION { path: ref p } => {
            Print::printBuf((literal!("OPTIMIZATION ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::MODEL { path: ref p } => {
            Print::printBuf((literal!("MODEL ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::RECORD { path: ref p } => {
            Print::printBuf((literal!("RECORD ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::BLOCK { path: ref p } => {
            Print::printBuf((literal!("BLOCK ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::CONNECTOR { path: ref p, .. } => {
            Print::printBuf((literal!("CONNECTOR ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::TYPE { path: ref p } => {
            Print::printBuf((literal!("TYPE ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::PACKAGE { path: ref p } => {
            Print::printBuf((literal!("PACKAGE ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::FUNCTION { isImpure: true, path: ref p } => {
            Print::printBuf((literal!("IMPURE FUNCTION ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::FUNCTION { path: ref p, .. } => {
            Print::printBuf((literal!("FUNCTION ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::TYPE_INTEGER { path: ref p } => {
            Print::printBuf((literal!("TYPE_INTEGER ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::TYPE_REAL { path: ref p } => {
            Print::printBuf((literal!("TYPE_REAL ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::TYPE_STRING { path: ref p } => {
            Print::printBuf((literal!("TYPE_STRING ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::TYPE_BOOL { path: ref p } => {
            Print::printBuf((literal!("TYPE_BOOL ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::TYPE_CLOCK { path: ref p } => {
            Print::printBuf((literal!("TYPE_CLOCK ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        ClassInf::State::HAS_RESTRICTIONS { path: ref p, .. } => {
            Print::printBuf((literal!("HAS_RESTRICTIONS ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            Print::printBuf((printStateStr(inState.clone())).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn getStateName(mut inState: ClassInf::State) -> Arc<Absyn::Path> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (match inState.clone() {
        ClassInf::State::UNKNOWN { path: ref p } => {
            p.clone()
        },
        ClassInf::State::OPTIMIZATION { path: ref p } => {
            p.clone()
        },
        ClassInf::State::MODEL { path: ref p } => {
            p.clone()
        },
        ClassInf::State::RECORD { path: ref p } => {
            p.clone()
        },
        ClassInf::State::BLOCK { path: ref p } => {
            p.clone()
        },
        ClassInf::State::CONNECTOR { path: ref p, .. } => {
            p.clone()
        },
        ClassInf::State::TYPE { path: ref p } => {
            p.clone()
        },
        ClassInf::State::PACKAGE { path: ref p } => {
            p.clone()
        },
        ClassInf::State::FUNCTION { path: ref p, .. } => {
            p.clone()
        },
        ClassInf::State::ENUMERATION { path: ref p } => {
            p.clone()
        },
        ClassInf::State::HAS_RESTRICTIONS { path: ref p, .. } => {
            p.clone()
        },
        ClassInf::State::TYPE_INTEGER { path: ref p } => {
            p.clone()
        },
        ClassInf::State::TYPE_REAL { path: ref p } => {
            p.clone()
        },
        ClassInf::State::TYPE_STRING { path: ref p } => {
            p.clone()
        },
        ClassInf::State::TYPE_BOOL { path: ref p } => {
            p.clone()
        },
        ClassInf::State::TYPE_CLOCK { path: ref p } => {
            p.clone()
        },
        ClassInf::State::TYPE_ENUM { path: ref p } => {
            p.clone()
        },
        ClassInf::State::EXTERNAL_OBJ { path: ref p } => {
            p.clone()
        },
        ClassInf::State::META_TUPLE { path: ref p } => {
            p.clone()
        },
        ClassInf::State::META_LIST { path: ref p } => {
            p.clone()
        },
        ClassInf::State::META_OPTION { path: ref p } => {
            p.clone()
        },
        ClassInf::State::META_RECORD { path: ref p } => {
            p.clone()
        },
        ClassInf::State::META_UNIONTYPE { path: ref p, .. } => {
            p.clone()
        },
        ClassInf::State::META_ARRAY { path: ref p } => {
            p.clone()
        },
        ClassInf::State::META_POLYMORPHIC { path: ref p } => {
            p.clone()
        },
        _ => {
            Arc::new(Absyn::Path::IDENT { name: (literal!("#getStateName failed#")).clone() })
        },
    });
    outPath
}

fn printEventStr(mut inEvent: ClassInf::Event) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inEvent.clone() {
        ClassInf::Event::FOUND_EQUATION => {
            literal!("equation")
        },
        ClassInf::Event::FOUND_CONSTRAINT => {
            literal!("constraint")
        },
        ClassInf::Event::NEWDEF => {
            literal!("new definition")
        },
        ClassInf::Event::FOUND_COMPONENT { name: mut name } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }
        },
        ClassInf::Event::FOUND_EXT_DECL => {
            literal!("external function declaration")
        },
        _ => {
            literal!("Unknown event")
        },
    })).clone();
    r#str
}

pub fn start(mut inRestriction: SCode::Restriction, mut inPath: Arc<Absyn::Path>) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = start_dispatch(inRestriction.clone(), AbsynUtil::makeFullyQualified(inPath.clone()))?;
    Ok(outState)
}

// Transitions
fn start_dispatch(mut inRestriction: SCode::Restriction, mut inPath: Arc<Absyn::Path>) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = (::match_deref::match_deref! { match &((inRestriction.clone(), inPath.clone())) {
        (SCode::Restriction::R_CLASS, p) => {
            ClassInf::State::UNKNOWN { path: p.clone() }
        },
        (SCode::Restriction::R_OPTIMIZATION, p) => {
            ClassInf::State::OPTIMIZATION { path: p.clone() }
        },
        (SCode::Restriction::R_MODEL, p) => {
            ClassInf::State::MODEL { path: p.clone() }
        },
        (SCode::Restriction::R_RECORD { isOperator: _ }, p) => {
            ClassInf::State::RECORD { path: p.clone() }
        },
        (SCode::Restriction::R_BLOCK, p) => {
            ClassInf::State::BLOCK { path: p.clone() }
        },
        (SCode::Restriction::R_CONNECTOR { isExpandable }, p) => {
            ClassInf::State::CONNECTOR { path: p.clone(), isExpandable: isExpandable.clone() }
        },
        (SCode::Restriction::R_TYPE, p) => {
            ClassInf::State::TYPE { path: p.clone() }
        },
        (SCode::Restriction::R_PACKAGE, p) => {
            ClassInf::State::PACKAGE { path: p.clone() }
        },
        (SCode::Restriction::R_FUNCTION { .. }, p) => {
            ClassInf::State::FUNCTION { path: p.clone(), isImpure: SCodeUtil::isRestrictionImpure(inRestriction.clone(), true) }
        },
        (SCode::Restriction::R_OPERATOR, p) => {
            ClassInf::State::FUNCTION { path: p.clone(), isImpure: false }
        },
        (SCode::Restriction::R_ENUMERATION, p) => {
            ClassInf::State::ENUMERATION { path: p.clone() }
        },
        (SCode::Restriction::R_PREDEFINED_INTEGER, p) => {
            ClassInf::State::TYPE_INTEGER { path: p.clone() }
        },
        (SCode::Restriction::R_PREDEFINED_REAL, p) => {
            ClassInf::State::TYPE_REAL { path: p.clone() }
        },
        (SCode::Restriction::R_PREDEFINED_STRING, p) => {
            ClassInf::State::TYPE_STRING { path: p.clone() }
        },
        (SCode::Restriction::R_PREDEFINED_BOOLEAN, p) => {
            ClassInf::State::TYPE_BOOL { path: p.clone() }
        },
        (SCode::Restriction::R_PREDEFINED_CLOCK, p) => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            ClassInf::State::TYPE_CLOCK { path: p.clone() }
        },
        (SCode::Restriction::R_PREDEFINED_ENUMERATION, p) => {
            ClassInf::State::TYPE_ENUM { path: p.clone() }
        },
        (SCode::Restriction::R_UNIONTYPE { .. }, p) => {
            ClassInf::State::META_UNIONTYPE { path: p.clone(), typeVars: var_field!(inRestriction.typeVars, SCode::Restriction::R_UNIONTYPE).clone() }
        },
        (SCode::Restriction::R_METARECORD { .. }, p) => {
            ClassInf::State::META_RECORD { path: p.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outState)
}

pub fn trans(mut inState: ClassInf::State, mut inEvent: ClassInf::Event) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = (match (inState.clone(), inEvent.clone()) {
        (ClassInf::State::UNKNOWN { path: ref p }, ClassInf::Event::NEWDEF) => {
            ClassInf::State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: false, hasAlgorithms: false, hasConstraints: false }
        },
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::MODEL { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::RECORD { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::BLOCK { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::CONNECTOR { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::TYPE { path: ref p }, ClassInf::Event::NEWDEF) => {
            ClassInf::State::TYPE { path: p.clone() }
        },
        (ClassInf::State::PACKAGE { path: ref p }, ClassInf::Event::NEWDEF) => {
            ClassInf::State::PACKAGE { path: p.clone() }
        },
        (ClassInf::State::FUNCTION { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::ENUMERATION { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_INTEGER { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_REAL { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_STRING { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_BOOL { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_CLOCK { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_ENUM { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::META_UNIONTYPE { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::META_RECORD { .. }, ClassInf::Event::NEWDEF) => {
            inState.clone()
        },
        (ClassInf::State::UNKNOWN { path: ref p }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            ClassInf::State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: false, hasAlgorithms: false, hasConstraints: false }
        },
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::MODEL { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::RECORD { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::BLOCK { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::CONNECTOR { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::TYPE { path: ref p }, ClassInf::Event::FOUND_COMPONENT { name: mut s }) => {
            if !(isBasicTypeComponentName((s.clone()).clone())) {
                Error::addMessage(Error::TYPE_NOT_FROM_PREDEFINED.clone(), list![(AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone()])?;
                bail!("fail");
            }
            ClassInf::State::TYPE { path: p.clone() }
        },
        (ClassInf::State::PACKAGE { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::FUNCTION { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::ENUMERATION { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::HAS_RESTRICTIONS { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_INTEGER { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_REAL { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_STRING { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_BOOL { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_CLOCK { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::TYPE_ENUM { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::META_RECORD { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::META_UNIONTYPE { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            inState.clone()
        },
        (ClassInf::State::UNKNOWN { path: ref p }, ClassInf::Event::FOUND_EQUATION) => {
            ClassInf::State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: true, hasAlgorithms: false, hasConstraints: false }
        },
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::FOUND_EQUATION) => {
            inState.clone()
        },
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::FOUND_CONSTRAINT) => {
            inState.clone()
        },
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::FOUND_ALGORITHM) => {
            inState.clone()
        },
        (ClassInf::State::MODEL { .. }, ClassInf::Event::FOUND_EQUATION) => {
            inState.clone()
        },
        (ClassInf::State::BLOCK { .. }, ClassInf::Event::FOUND_EQUATION) => {
            inState.clone()
        },
        (ClassInf::State::MODEL { .. }, ClassInf::Event::FOUND_ALGORITHM) => {
            inState.clone()
        },
        (ClassInf::State::BLOCK { .. }, ClassInf::Event::FOUND_ALGORITHM) => {
            inState.clone()
        },
        (ClassInf::State::FUNCTION { .. }, ClassInf::Event::FOUND_ALGORITHM) => {
            inState.clone()
        },
        (ClassInf::State::HAS_RESTRICTIONS { hasConstraints: mut b3, hasAlgorithms: mut b2, path: ref p, .. }, ClassInf::Event::FOUND_EQUATION) => {
            ClassInf::State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: true, hasAlgorithms: b2.clone(), hasConstraints: b3.clone() }
        },
        (ClassInf::State::HAS_RESTRICTIONS { hasAlgorithms: mut b2, hasEquations: mut b1, path: ref p, .. }, ClassInf::Event::FOUND_CONSTRAINT) => {
            ClassInf::State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: b1.clone(), hasAlgorithms: b2.clone(), hasConstraints: true }
        },
        (ClassInf::State::HAS_RESTRICTIONS { hasConstraints: mut b3, hasEquations: mut b1, path: ref p, .. }, ClassInf::Event::FOUND_ALGORITHM) => {
            ClassInf::State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: b1.clone(), hasAlgorithms: true, hasConstraints: b3.clone() }
        },
        (ClassInf::State::FUNCTION { .. }, ClassInf::Event::FOUND_EXT_DECL) => {
            inState.clone()
        },
        (_, ClassInf::Event::FOUND_EXT_DECL) => {
            bail!("fail")
        },
        (_, ClassInf::Event::FOUND_EQUATION) => {
            bail!("fail")
        },
        (_, ClassInf::Event::FOUND_CONSTRAINT) => {
            bail!("fail")
        },
        (mut st, mut ev) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ClassInfUtil.trans failed: ")); __mm_s.push_str(&*printStateStr(st.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*printEventStr(ev.clone())); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
    });
    Ok(outState)
}

pub fn valid(mut inState: ClassInf::State, mut inRestriction: SCode::Restriction) -> Result<()> {
    let () = (match (inState.clone(), inRestriction.clone()) {
        (ClassInf::State::UNKNOWN { .. }, _) => (),
        (ClassInf::State::HAS_RESTRICTIONS { .. }, SCode::Restriction::R_CLASS) => (),
        (ClassInf::State::HAS_RESTRICTIONS { .. }, SCode::Restriction::R_MODEL) => (),
        (ClassInf::State::HAS_RESTRICTIONS { .. }, SCode::Restriction::R_OPTIMIZATION) => (),
        (ClassInf::State::MODEL { .. }, SCode::Restriction::R_MODEL) => (),
        (ClassInf::State::RECORD { .. }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (ClassInf::State::RECORD { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::HAS_RESTRICTIONS { hasAlgorithms: false, hasConstraints: false, hasEquations: false, .. }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (ClassInf::State::BLOCK { .. }, SCode::Restriction::R_BLOCK) => (),
        (ClassInf::State::MODEL { .. }, SCode::Restriction::R_MODEL) => (),
        (ClassInf::State::CONNECTOR { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::CONNECTOR { isExpandable: false, .. }, SCode::Restriction::R_CONNECTOR { isExpandable: false }) => (),
        (ClassInf::State::CONNECTOR { isExpandable: true, .. }, SCode::Restriction::R_CONNECTOR { isExpandable: true }) => (),
        (ClassInf::State::HAS_RESTRICTIONS { hasAlgorithms: false, hasConstraints: false, hasEquations: false, .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_INTEGER { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_REAL { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_STRING { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_BOOL { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_CLOCK { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_ENUM { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::ENUMERATION { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE { .. }, SCode::Restriction::R_CONNECTOR { .. }) => (),
        (ClassInf::State::TYPE { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::TYPE_INTEGER { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::TYPE_REAL { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::TYPE_STRING { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::TYPE_BOOL { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::TYPE_CLOCK { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::TYPE_ENUM { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::ENUMERATION { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::PACKAGE { .. }, SCode::Restriction::R_PACKAGE) => (),
        (ClassInf::State::HAS_RESTRICTIONS { hasAlgorithms: false, hasConstraints: false, hasEquations: false, .. }, SCode::Restriction::R_PACKAGE) => (),
        (ClassInf::State::FUNCTION { .. }, SCode::Restriction::R_FUNCTION { functionRestriction: _ }) => (),
        (ClassInf::State::HAS_RESTRICTIONS { hasConstraints: false, hasEquations: false, .. }, SCode::Restriction::R_FUNCTION { functionRestriction: _ }) => (),
        (ClassInf::State::META_TUPLE { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::META_LIST { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::META_OPTION { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::META_RECORD { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::META_ARRAY { .. }, SCode::Restriction::R_TYPE) => (),
        (ClassInf::State::META_UNIONTYPE { .. }, SCode::Restriction::R_TYPE) => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn assertValid(mut inState: ClassInf::State, mut inRestriction: SCode::Restriction, mut info: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inState.clone(), inRestriction.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut st, mut re) = __mc_input.clone() else { bail!("nomatch") };
            valid(st.clone(), re.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut st, mut re) = __mc_input.clone() else { bail!("nomatch") };
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            let mut str3: ArcStr = arcstr::literal!("");
            str1 = (AbsynUtil::pathString(getStateName(st.clone()), (literal!(".")).clone(), true, false)?).clone();
            str2 = (printStateStr(st.clone())).clone();
            str3 = (SCodeDump::restrictionStringPP(re.clone())?).clone();
            Error::addSourceMessage(Error::RESTRICTION_VIOLATION.clone(), list![(str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone()], info.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn assertTrans(mut inState: ClassInf::State, mut event: ClassInf::Event, mut info: SourceInfo) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = 'mc: {
        let __mc_input = inState.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut st = __mc_input.clone() else { bail!("nomatch") };
            Ok(trans(st.clone(), event.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut st = __mc_input.clone() else { bail!("nomatch") };
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            let mut str3: ArcStr = arcstr::literal!("");
            str1 = (AbsynUtil::pathString(getStateName(st.clone()), (literal!(".")).clone(), true, false)?).clone();
            str2 = (printStateStr(st.clone())).clone();
            str3 = (printEventStr(event.clone())).clone();
            Error::addSourceMessage(Error::TRANS_VIOLATION.clone(), list![(str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone()], info.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outState)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn matchingState(mut inState: ClassInf::State, mut inStateLst: Arc<metamodelica::List<ClassInf::State>>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inState.clone(), inStateLst.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            false
        },
        (ClassInf::State::UNKNOWN { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::UNKNOWN { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::MODEL { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::MODEL { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::RECORD { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::RECORD { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::BLOCK { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::BLOCK { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::CONNECTOR { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::CONNECTOR { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::TYPE { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::TYPE { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::PACKAGE { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::PACKAGE { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::FUNCTION { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::FUNCTION { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::ENUMERATION { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::ENUMERATION { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::TYPE_INTEGER { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::TYPE_INTEGER { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::TYPE_REAL { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::TYPE_REAL { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::TYPE_STRING { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::TYPE_STRING { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::TYPE_BOOL { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::TYPE_BOOL { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::TYPE_CLOCK { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::TYPE_CLOCK { .. }, tail: _ }) => {
            true
        },
        (ClassInf::State::TYPE_ENUM { .. }, Deref @ metamodelica::List::Cons { head: ClassInf::State::TYPE_ENUM { .. }, tail: _ }) => {
            true
        },
        (_, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut res: bool = false;
            res = matchingState(inState.clone(), rest.clone())?;
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBoolean)
}

pub fn isFunction(mut inState: ClassInf::State) -> bool {
    let mut b: bool = false;
    b = (match inState.clone() {
        ClassInf::State::FUNCTION { .. } => true,
        _ => false,
    });
    b
}

pub fn isFunctionOrRecord(mut inState: ClassInf::State) -> bool {
    let mut b: bool = false;
    b = (match inState.clone() {
        ClassInf::State::FUNCTION { .. } => true,
        ClassInf::State::RECORD { .. } => true,
        _ => false,
    });
    b
}

pub fn isConnector(mut inState: ClassInf::State) -> Result<()> {
    let () = (match inState.clone() {
        ClassInf::State::CONNECTOR { .. } => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub static basicTypeMods: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("quantity")).clone(), (literal!("unit")).clone(), (literal!("displayUnit")).clone(), (literal!("min")).clone(), (literal!("max")).clone(), (literal!("start")).clone(), (literal!("fixed")).clone(), (literal!("nominal")).clone(), (literal!("stateSelect")).clone(), (literal!("uncertain")).clone(), (literal!("distribution")).clone()] });

pub fn isBasicTypeComponentName(mut name: ArcStr) -> bool {
    let mut res: bool = false;
    res = listMember((name.clone()).clone(), basicTypeMods.clone());
    res
}

pub fn isTypeOrRecord(mut inState: ClassInf::State) -> bool {
    let mut outIsTypeOrRecord: bool = false;
    outIsTypeOrRecord = (match inState.clone() {
        ClassInf::State::TYPE { .. } => true,
        ClassInf::State::RECORD { .. } => true,
        _ => false,
    });
    outIsTypeOrRecord
}

pub fn isRecord(mut inState: ClassInf::State) -> bool {
    let mut outIsRecord: bool = false;
    outIsRecord = (match inState.clone() {
        ClassInf::State::RECORD { .. } => true,
        _ => false,
    });
    outIsRecord
}

pub fn isMetaRecord(mut inState: ClassInf::State) -> bool {
    let mut outIsRecord: bool = false;
    outIsRecord = (match inState.clone() {
        ClassInf::State::META_RECORD { .. } => true,
        _ => false,
    });
    outIsRecord
}

