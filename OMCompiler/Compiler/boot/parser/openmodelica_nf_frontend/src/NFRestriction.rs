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

use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub enum NFRestriction {
    BLOCK,
    CLASS,
    CLOCK,
    CONNECTOR {
        isExpandable: bool,
    },
    ENUMERATION,
    EXTERNAL_OBJECT,
    FUNCTION,
    MODEL,
    PACKAGE,
    OPERATOR,
    RECORD {
        isOperator: bool,
        usedExternally: bool,
    },
    RECORD_CONSTRUCTOR,
    TYPE,
    UNKNOWN,
}
pub use self::NFRestriction::{BLOCK,CLASS,CLOCK,CONNECTOR,ENUMERATION,EXTERNAL_OBJECT,FUNCTION,MODEL,PACKAGE,OPERATOR,RECORD,RECORD_CONSTRUCTOR,TYPE,UNKNOWN};
pub fn fromSCode(mut sres: SCode::Restriction) -> Arc<NFRestriction> {
    let mut res: Arc<NFRestriction> = Arc::new(NFRestriction::BLOCK);
    res = (match sres.clone() {
        SCode::Restriction::R_BLOCK { .. } => Arc::new(crate::NFRestriction::BLOCK),
        SCode::Restriction::R_CLASS { .. } => Arc::new(crate::NFRestriction::CLASS),
        SCode::Restriction::R_PREDEFINED_CLOCK { .. } => Arc::new(crate::NFRestriction::CLOCK),
        SCode::Restriction::R_CONNECTOR { .. } => Arc::new(NFRestriction::CONNECTOR { isExpandable: var_field!(sres.isExpandable, SCode::Restriction::R_CONNECTOR).clone() }),
        SCode::Restriction::R_ENUMERATION { .. } => Arc::new(crate::NFRestriction::ENUMERATION),
        SCode::Restriction::R_FUNCTION { .. } => Arc::new(crate::NFRestriction::FUNCTION),
        SCode::Restriction::R_MODEL { .. } => Arc::new(crate::NFRestriction::MODEL),
        SCode::Restriction::R_OPERATOR { .. } => Arc::new(crate::NFRestriction::OPERATOR),
        SCode::Restriction::R_PACKAGE { .. } => Arc::new(crate::NFRestriction::PACKAGE),
        SCode::Restriction::R_RECORD { .. } => Arc::new(NFRestriction::RECORD { isOperator: var_field!(sres.isOperator, SCode::Restriction::R_RECORD).clone(), usedExternally: false }),
        SCode::Restriction::R_TYPE { .. } => Arc::new(crate::NFRestriction::TYPE),
        _ => Arc::new(crate::NFRestriction::MODEL),
    });
    res
}

pub fn toDAE(mut res: Arc<NFRestriction>, mut path: Arc<Absyn::Path>) -> ClassInf::State {
    let mut state: ClassInf::State;
    state = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ BLOCK { .. } => ClassInf::State::BLOCK { path: path.clone() },
        Deref @ CLOCK { .. } => ClassInf::State::TYPE_CLOCK { path: path.clone() },
        Deref @ CONNECTOR { .. } => ClassInf::State::CONNECTOR { path: path.clone(), isExpandable: var_field!((*res).isExpandable, NFRestriction::CONNECTOR).clone() },
        Deref @ ENUMERATION { .. } => ClassInf::State::ENUMERATION { path: path.clone() },
        Deref @ EXTERNAL_OBJECT { .. } => ClassInf::State::EXTERNAL_OBJ { path: path.clone() },
        Deref @ FUNCTION { .. } => ClassInf::State::FUNCTION { path: path.clone(), isImpure: false },
        Deref @ MODEL { .. } => ClassInf::State::MODEL { path: path.clone() },
        Deref @ OPERATOR { .. } => ClassInf::State::FUNCTION { path: path.clone(), isImpure: false },
        Deref @ PACKAGE { .. } => ClassInf::State::PACKAGE { path: path.clone() },
        Deref @ RECORD { .. } => ClassInf::State::RECORD { path: path.clone() },
        Deref @ RECORD_CONSTRUCTOR { .. } => ClassInf::State::RECORD { path: path.clone() },
        Deref @ TYPE { .. } => ClassInf::State::TYPE { path: path.clone() },
        _ => ClassInf::State::UNKNOWN { path: path.clone() },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    state
}

pub fn isConnector(mut res: Arc<NFRestriction>) -> bool {
    let mut isConnector: bool = false;
    isConnector = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ CONNECTOR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConnector
}

pub fn isExpandableConnector(mut res: Arc<NFRestriction>) -> bool {
    let mut isConnector: bool = false;
    isConnector = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ CONNECTOR { .. } => var_field!((*res).isExpandable, NFRestriction::CONNECTOR).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConnector
}

pub fn isNonexpandableConnector(mut res: Arc<NFRestriction>) -> bool {
    let mut isNonexpandable: bool = false;
    isNonexpandable = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ CONNECTOR { .. } => !(var_field!((*res).isExpandable, NFRestriction::CONNECTOR).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNonexpandable
}

pub fn isExternalObject(mut res: Arc<NFRestriction>) -> bool {
    let mut isExternalObject: bool = false;
    isExternalObject = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ EXTERNAL_OBJECT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExternalObject
}

pub fn isFunction(mut res: Arc<NFRestriction>) -> bool {
    let mut isFunction: bool = false;
    isFunction = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ FUNCTION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isFunction
}

pub fn isRecordConstructor(mut res: Arc<NFRestriction>) -> bool {
    let mut isConstructor: bool = false;
    isConstructor = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ RECORD_CONSTRUCTOR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConstructor
}

pub fn isRecord(mut res: Arc<NFRestriction>) -> bool {
    let mut isRecord: bool = false;
    isRecord = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRecord
}

pub fn isExternalRecord(mut res: Arc<NFRestriction>) -> bool {
    let mut isExtRecord: bool = false;
    isExtRecord = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ RECORD { .. } => var_field!((*res).usedExternally, NFRestriction::RECORD).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtRecord
}

pub fn setExternalRecord(mut res: Arc<NFRestriction>) -> Arc<NFRestriction> {
    let mut res: Arc<NFRestriction> = res;
    let () = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ RECORD { usedExternally: false, .. } => {
            assign_variant_field!(res => NFRestriction::RECORD; usedExternally = true);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isOperatorRecord(mut res: Arc<NFRestriction>) -> bool {
    let mut isOpRecord: bool = false;
    isOpRecord = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ RECORD { .. } => var_field!((*res).isOperator, NFRestriction::RECORD).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOpRecord
}

pub fn isOperator(mut res: Arc<NFRestriction>) -> bool {
    let mut isOperator: bool = false;
    isOperator = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ OPERATOR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOperator
}

pub fn isType(mut res: Arc<NFRestriction>) -> bool {
    let mut isType: bool = false;
    isType = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ TYPE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isType
}

pub fn isClock(mut res: Arc<NFRestriction>) -> bool {
    let mut isClock: bool = false;
    isClock = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isClock
}

pub fn isModel(mut res: Arc<NFRestriction>) -> bool {
    let mut isModel: bool = false;
    isModel = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ MODEL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isModel
}

pub fn toString(mut res: Arc<NFRestriction>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(res.clone()) {
        Deref @ BLOCK { .. } => literal!("block"),
        Deref @ CLASS { .. } => literal!("class"),
        Deref @ CLOCK { .. } => literal!("clock"),
        Deref @ CONNECTOR { .. } => if (var_field!((*res).isExpandable, NFRestriction::CONNECTOR).clone()) {literal!("expandable connector")} else {literal!("connector")},
        Deref @ ENUMERATION { .. } => literal!("enumeration"),
        Deref @ EXTERNAL_OBJECT { .. } => literal!("ExternalObject"),
        Deref @ FUNCTION { .. } => literal!("function"),
        Deref @ MODEL { .. } => literal!("model"),
        Deref @ OPERATOR { .. } => literal!("operator"),
        Deref @ PACKAGE { .. } => literal!("package"),
        Deref @ RECORD { .. } => literal!("record"),
        Deref @ RECORD_CONSTRUCTOR { .. } => literal!("record"),
        Deref @ TYPE { .. } => literal!("type"),
        _ => literal!("unknown"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    r#str
}

pub fn assertNoEquations(mut equations: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut initialEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut res: Arc<NFRestriction>, mut onlyDeprecated: bool) -> Result<()> {
    let mut eq: Arc<SCode::Equation>;
    if equations.clone().is_empty() && initialEquations.clone().is_empty() {
        return Ok(());
    }
    eq = listHead(if (equations.clone().is_empty()) {initialEquations.clone()} else {equations.clone()})?;
    if onlyDeprecated.clone() {
        Error::addSourceMessage(Error::DEPRECATED_TRANSITION_FAILURE.clone(), list![(literal!("Equation sections")).clone(), (toString(res.clone())).clone()], SCodeUtil::getEquationInfo(eq.clone())?)?;
    } else {
        Error::addSourceMessage(Error::EQUATION_TRANSITION_FAILURE.clone(), list![(toString(res.clone())).clone()], SCodeUtil::getEquationInfo(eq.clone())?)?;
        bail!("fail");
    }
    Ok(())
}

pub fn assertNoAlgorithms(mut algorithms: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut initialAlgorithms: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut res: Arc<NFRestriction>, mut onlyDeprecated: bool) -> Result<()> {
    let mut alg_opt: Option<Arc<SCode::AlgorithmSection>> = None;
    let mut alg: Arc<SCode::AlgorithmSection>;
    let mut info: SourceInfo;
    alg_opt = List::findOption(algorithms.clone(), Arc::new(fnptr!(SCodeUtil::isNonEmptyAlgorithm, Arc<SCode::AlgorithmSection>)));
    if isNone(alg_opt.clone()) {
        alg_opt = List::findOption(initialAlgorithms.clone(), Arc::new(fnptr!(SCodeUtil::isNonEmptyAlgorithm, Arc<SCode::AlgorithmSection>)));
    }
    if isSome(alg_opt.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(alg_opt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        alg = __pa0.clone();
        info = SCodeUtil::getStatementInfo(listHead(alg.statements.clone())?)?;
        if onlyDeprecated.clone() {
            Error::addSourceMessage(Error::DEPRECATED_TRANSITION_FAILURE.clone(), list![(literal!("Algorithm sections")).clone(), (toString(res.clone())).clone()], info.clone())?;
            return Ok(());
        } else {
            Error::addSourceMessage(Error::ALGORITHM_TRANSITION_FAILURE.clone(), list![(toString(res.clone())).clone()], info.clone())?;
            bail!("fail");
        }
    }
    Ok(())
}

pub fn assertNoInitialAlgorithms(mut algs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut res: Arc<NFRestriction>) -> Result<()> {
    for mut alg in &*algs.clone() {
        let mut alg = alg.clone();
        if !(alg.statements.clone().is_empty()) {
            Error::addSourceMessage(Error::INITIAL_ALGORITHM_TRANSITION_FAILURE.clone(), list![(toString(res.clone())).clone()], SCodeUtil::getStatementInfo(listHead(alg.statements.clone())?)?)?;
            bail!("fail");
        }
    }
    Ok(())
}

pub fn assertNoProtected(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut res: Arc<NFRestriction>) -> Result<()> {
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        if SCodeUtil::isElementProtected(e.clone())? {
            Error::addSourceMessage(Error::PROTECTED_TRANSITION_FAILURE.clone(), list![(toString(res.clone())).clone()], SCodeUtil::elementInfo(e.clone()))?;
            bail!("fail");
        }
    }
    Ok(())
}

pub fn assertNoComponents(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut res: Arc<NFRestriction>) -> Result<()> {
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        if SCodeUtil::isComponent(e.clone()) {
            Error::addSourceMessage(Error::DEPRECATED_TRANSITION_FAILURE.clone(), list![(literal!("Components")).clone(), (toString(res.clone())).clone()], SCodeUtil::elementInfo(e.clone()))?;
        }
    }
    Ok(())
}

pub fn assertOnlyConstantComponents(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut clsNode: Arc<InstNode::InstNode>) -> Result<()> {
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } if (!(SCodeUtil::isConstant(SCodeUtil::attrVariability(var_field!((*e).attributes, SCode::Element::COMPONENT).clone())?))) => {
            Error::addSourceMessage(Error::PACKAGE_VARIABLE_NOT_CONSTANT.clone(), list![(var_field!((*e).name, SCode::Element::COMPONENT).clone()).clone(), (InstNode::name(clsNode.clone())?).clone()], var_field!((*e).info, SCode::Element::COMPONENT).clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

pub fn assertOnlyFunctions(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut res: Arc<NFRestriction>) -> () {
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        if !(SCodeUtil::isFunction(e.clone())) {
        }
    }
    ()
}

pub fn checkClass(mut node: Arc<InstNode::InstNode>, mut restriction: Arc<NFRestriction>, mut context: i32) -> Result<()> {
    let mut cdef: Arc<SCode::ClassDef>;
    if InstContext::inRelaxed(context.clone()) {
        return Ok(());
    }
    cdef = SCodeUtil::getClassBody(InstNode::definition(node.clone())?)?;
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            let () = (::match_deref::match_deref! { match &(restriction.clone()) {
        Deref @ CLASS => {
            assertNoComponents(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), true)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), true)?;
            ()
        },
        Deref @ RECORD { .. } => {
            assertNoProtected(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            ()
        },
        Deref @ TYPE => {
            assertNoProtected(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            ()
        },
        Deref @ BLOCK => (),
        Deref @ FUNCTION => {
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoInitialAlgorithms(var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            ()
        },
        Deref @ CONNECTOR { .. } => {
            assertNoProtected(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            ()
        },
        Deref @ PACKAGE => {
            assertOnlyConstantComponents(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), node.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            ()
        },
        Deref @ OPERATOR => {
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}


