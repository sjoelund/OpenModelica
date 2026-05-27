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

use crate::AdjacencyMatrix;
use crate::BackendDAE;
use crate::BackendDAEEXT;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::DumpGraphML;
use crate::ExpressionSolve;
use crate::Matching;
use crate::Sorting;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// =============================================================================
// section for type definitions
//
//
// =============================================================================
pub const BORDER: &'static str = "****************************************";

pub const UNDERLINE: &'static str = "========================================";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TearingMethod {
    /// Only tear discrete variables from loops
    MINIMAL_TEARING,
    OMC_TEARING,
    CELLIER_TEARING,
    TOTAL_TEARING,
    USER_DEFINED_TEARING,
}
pub use self::TearingMethod::{MINIMAL_TEARING,OMC_TEARING,CELLIER_TEARING,TOTAL_TEARING,USER_DEFINED_TEARING};

// =============================================================================
// section for all public functions
//
// main function to divide to the selected tearing method
// =============================================================================
pub fn tearingSystem(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut methodString: ArcStr = Config::getTearingMethod()?;
    let mut method: TearingMethod = TearingMethod::CELLIER_TEARING;
    let mut DAEtype: BackendDAE::BackendDAEType = BackendDAE::BackendDAEType::ALGEQSYSTEM;
    let mut strongComponentIndex: i32 = System::tmpTickIndex(Global::strongComponent_index.clone());
    if Flags::getConfigInt(Flags::MAX_SIZE_LINEAR_TEARING.clone())? < 0 {
        Error::addMessage(Error::INVALID_FLAG_TYPE.clone(), list![(literal!("maxSizeLinearTearing")).clone(), (literal!("non-negative integer")).clone(), (intString(Flags::getConfigInt(Flags::MAX_SIZE_LINEAR_TEARING.clone())?)).clone()])?;
        bail!("fail");
    } else if Flags::getConfigInt(Flags::MAX_SIZE_NONLINEAR_TEARING.clone())? < 0 {
        Error::addMessage(Error::INVALID_FLAG_TYPE.clone(), list![(literal!("maxSizeNonlinearTearing")).clone(), (literal!("non-negative integer")).clone(), (intString(Flags::getConfigInt(Flags::MAX_SIZE_NONLINEAR_TEARING.clone())?)).clone()])?;
        bail!("fail");
    }
    match '__try0: {
        method = unwrap_break_err!(getTearingMethod((methodString.clone()).clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::TEARING_DUMP.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone()), '__try0) {
            let __pa1 = ::match_deref::match_deref! { match &(inDAE.shared.clone()) {
                Deref @ BackendDAE::Shared { backendDAEType: __pa1, .. } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            DAEtype = __pa1.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\n\n\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\nCalling Tearing for ")); __mm_s.push_str(&*unwrap_break_err!(BackendDump::printBackendDAEType2String(DAEtype.clone()), '__try0)); __mm_s.push_str(&*literal!("!\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        (outDAE, strongComponentIndex) = unwrap_break_err!(BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), Arc::new({ let __pe_b0 = method.clone(); move |__pe_a1, __pe_a2, __pe_a3| tearingSystemWork(__pe_b0.clone(), __pe_a1, __pe_a2, __pe_a3) }), strongComponentIndex.clone()), '__try0);
        System::tmpTickSetIndex(strongComponentIndex.clone(), Global::strongComponent_index.clone());
        Ok::<_, anyhow::Error>((method.clone(), outDAE.clone(), strongComponentIndex.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            method = __try0_o0;
            outDAE = __try0_o1;
            strongComponentIndex = __try0_o2;
        }
        Err(_) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing.tearingSystem")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok(outDAE)
}

// =============================================================================
// protected
//
//
// =============================================================================
fn getTearingMethod(mut inTearingMethod: ArcStr) -> Result<TearingMethod> {
    let mut outTearingMethod: TearingMethod = TearingMethod::CELLIER_TEARING;
    outTearingMethod = (::match_deref::match_deref! { match &(inTearingMethod.clone()) {
        Deref @ "minimalTearing" => crate::Tearing::TearingMethod::MINIMAL_TEARING,
        Deref @ "omcTearing" => crate::Tearing::TearingMethod::OMC_TEARING,
        Deref @ "cellier" => crate::Tearing::TearingMethod::CELLIER_TEARING,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing.getTearingMethod")); __mm_s.push_str(&*literal!(" got invalid name \"")); __mm_s.push_str(&*inTearingMethod.clone()); __mm_s.push_str(&*literal!("\".")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTearingMethod)
}

fn callTearingMethod(mut inTearingMethod: TearingMethod, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut eindex: Arc<metamodelica::List<i32>>, mut vindx: Arc<metamodelica::List<i32>>, mut ojac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>, mut jacType: BackendDAE::JacobianType, mut mixedSystem: bool, mut strongComponentIndex: i32) -> Result<(Arc<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: Arc<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool = false;
    let debug: bool = false;
    let mut userTVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut userResiduals: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tearingMethod: TearingMethod = inTearingMethod.clone();
    if listMember(strongComponentIndex.clone(), Flags::getConfigIntList(Flags::TOTAL_TEARING.clone())?) {
        tearingMethod = crate::Tearing::TearingMethod::TOTAL_TEARING;
    } else {
        userTVars = Flags::getConfigIntList(Flags::SET_TEARING_VARS.clone())?;
        userResiduals = Flags::getConfigIntList(Flags::SET_RESIDUAL_EQNS.clone())?;
        (userTVars, userResiduals) = getUserTearingSet(userTVars.clone(), userResiduals.clone(), strongComponentIndex.clone())?;
        if !(userTVars.clone().is_empty()) && !(userResiduals.clone().is_empty()) {
            tearingMethod = crate::Tearing::TearingMethod::USER_DEFINED_TEARING;
        }
    }
    (ocomp, outRunMatching) = (match tearingMethod.clone() {
        TearingMethod::OMC_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\nTearing type: heuristic\n")).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing strictness: ")); __mm_s.push_str(&*Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (ocomp, outRunMatching) = omcTearing(isyst.clone(), ishared.clone(), eindex.clone(), vindx.clone(), ojac.clone(), jacType.clone(), mixedSystem.clone())?;
            if debug.clone() {
                execStat((literal!("Tearing.omcTearing")).clone())?;
            }
            (ocomp.clone(), outRunMatching.clone())
        },
        TearingMethod::CELLIER_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\nTearing type: heuristic\n")).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing strictness: ")); __mm_s.push_str(&*Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (ocomp, outRunMatching) = CellierTearing(isyst.clone(), ishared.clone(), eindex.clone(), vindx.clone(), userTVars.clone(), ojac.clone(), jacType.clone(), mixedSystem.clone(), strongComponentIndex.clone())?;
            if debug.clone() {
                execStat((literal!("Tearing.CellierTearing")).clone())?;
            }
            (ocomp.clone(), outRunMatching.clone())
        },
        TearingMethod::TOTAL_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\nTearing type: total\n")).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing strictness: ")); __mm_s.push_str(&*Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (ocomp, outRunMatching) = totalTearing(isyst.clone(), ishared.clone(), eindex.clone(), vindx.clone(), ojac.clone(), jacType.clone(), mixedSystem.clone())?;
            if debug.clone() {
                execStat((literal!("Tearing.totalTearing")).clone())?;
            }
            (ocomp.clone(), outRunMatching.clone())
        },
        TearingMethod::MINIMAL_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\nTearing type: minimal\n")).clone());
            }
            ocomp = minimalTearing(isyst.clone(), ishared.clone(), eindex.clone(), vindx.clone(), jacType.clone(), mixedSystem.clone())?;
            if debug.clone() {
                execStat((literal!("Tearing.minimalTearing")).clone())?;
            }
            (ocomp.clone(), true)
        },
        TearingMethod::USER_DEFINED_TEARING { .. } => {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\nTearing type: user defined\n")).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing strictness: ")); __mm_s.push_str(&*Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (ocomp, outRunMatching) = userDefinedTearing(isyst.clone(), ishared.clone(), eindex.clone(), vindx.clone(), ojac.clone(), jacType.clone(), mixedSystem.clone(), userTVars.clone(), userResiduals.clone())?;
            if debug.clone() {
                execStat((literal!("Tearing.userDefinedTearing")).clone())?;
            }
            (ocomp.clone(), outRunMatching.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((ocomp, outRunMatching))
}

fn tearingSystemWork(mut tearingMethod: TearingMethod, mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inStrongComponentIndex: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outStrongComponentIndex: i32 = 0;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut runMatching: bool = false;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, ass2: __pa1, ass1: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    ass2 = __pa1.clone();
    ass1 = __pa2.clone();
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of traverseComponents\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (comps, runMatching, outStrongComponentIndex) = traverseComponents(comps.clone(), isyst.clone(), inShared.clone(), tearingMethod.clone(), inStrongComponentIndex.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of traverseComponents\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    osyst = if (runMatching.clone()) {BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() }))?} else {isyst.clone()};
    Ok((osyst, outShared, outStrongComponentIndex))
}

fn traverseComponents(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMethod: TearingMethod, mut strongComponentIndexIn: i32) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, bool, i32)> {
    let mut oComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut outRunMatching: bool = false;
    let mut strongComponentIndexOut: i32 = strongComponentIndexIn.clone();
    oComps = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
        for mut co in (inComps.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(co.clone()) {
        comp => {
            let mut b: bool = false;
            let mut comp = (*comp).clone();
            (comp, b, strongComponentIndexOut) = traverseComponent(comp.clone(), isyst.clone(), ishared.clone(), inMethod.clone(), strongComponentIndexOut.clone())?;
            outRunMatching = outRunMatching.clone() || b.clone();
            comp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((oComps, outRunMatching, strongComponentIndexOut))
}

fn traverseComponent(mut inComp: Arc<BackendDAE::StrongComponent>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMethod: TearingMethod, mut strongComponentIndexIn: i32) -> Result<(Arc<BackendDAE::StrongComponent>, bool, i32)> {
    let mut oComp: Arc<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool = false;
    let mut strongComponentIndexOut: i32 = strongComponentIndexIn.clone();
    let debug: bool = false;
    let mut debugFlag: bool = Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?;
    strongComponentIndexOut = (::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { .. }, .. } => {
            if debugFlag.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Handle strong component with index: ")); __mm_s.push_str(&*intString(strongComponentIndexOut.clone() + 1)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                if !(listMember(strongComponentIndexOut.clone() + 1, Flags::getConfigIntList(Flags::NO_TEARING_FOR_COMPONENT.clone())?)) {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("To disable tearing of this component use '--noTearingForComponent=")); __mm_s.push_str(&*intString(strongComponentIndexOut.clone() + 1)); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            strongComponentIndexOut.clone() + 1
        },
        _ => strongComponentIndexOut.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oComp, outRunMatching) = (::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { mixedSystem, jacType, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: ojac }, vars: vindx, eqns: eindex } => {
            let mut isLinear: bool = false;
            let mut useTearing: bool = false;
            isLinear = BackendDAEUtil::getLinearfromJacType(jacType.clone())?;
            useTearing = checkTearingSettings(isLinear.clone(), strongComponentIndexOut.clone(), (vindx.clone().len() as i32))?;
            if useTearing.clone() {
                if debugFlag.clone() {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nTearing of ")); __mm_s.push_str(&*if (isLinear.clone()) {literal!("LINEAR")} else {literal!("NONLINEAR")}); __mm_s.push_str(&*literal!(" component\n")); ArcStr::from(__mm_s) }).clone());
                    let _ = (match (Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?, Flags::isSet(Flags::ITERATION_VARS.clone())?) {
        (false, false) => {
            println!("{}", (literal!("Use Flag '-d=tearingdumpV' and '-d=iterationVars' for more details\n\n")).clone());
            ()
        },
        (false, true) => {
            println!("{}", (literal!("Use Flag '-d=tearingdumpV' for more details\n\n")).clone());
            ()
        },
        (true, false) => {
            println!("{}", (literal!("Use Flag '-d=iterationVars' for more details\n\n")).clone());
            ()
        },
        (true, true) => {
            println!("{}", (literal!("\n")).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
                }
                if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Jacobian:\n")); __mm_s.push_str(&*BackendDump::dumpJacobianStr(ojac.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                }
                if debug.clone() {
                    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tearing.traverseComponent ")); __mm_s.push_str(&*if (isLinear.clone()) {literal!("LS")} else {literal!("NLS")}); __mm_s.push_str(&*literal!(" start")); ArcStr::from(__mm_s) }).clone())?;
                }
                match '__try0: {
                    (oComp, _) = unwrap_break_err!(callTearingMethod(inMethod.clone(), isyst.clone(), ishared.clone(), eindex.clone(), vindx.clone(), ojac.clone(), jacType.clone(), mixedSystem.clone(), strongComponentIndexOut.clone()), '__try0);
                    outRunMatching = true;
                    Ok::<_, anyhow::Error>((oComp.clone(), outRunMatching.clone()))
                } {
                    Ok((__try0_o0, __try0_o1)) => {
                        oComp = __try0_o0;
                        outRunMatching = __try0_o1;
                    }
                    Err(_) => {
                        oComp = inComp.clone();
                        outRunMatching = false;
                    }
                }
            } else {
                oComp = inComp.clone();
                outRunMatching = false;
            }
            (oComp.clone(), outRunMatching.clone())
        },
        _ => {
            (inComp.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oComp, outRunMatching, strongComponentIndexOut))
}

fn checkTearingSettings(mut isLinear: bool, mut strongComponentIndex: i32, mut numVars: i32) -> Result<bool> {
    let mut activateTearing: bool = false;
    let withLSS: Arc<metamodelica::List<ArcStr>> = list![(literal!("C")).clone()];
    let withNSS: Arc<metamodelica::List<ArcStr>> = list![(literal!("C")).clone()];
    let mut debugFlag: bool = Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?;
    let mut maxSize: i32 = 0;
    let mut isDense: bool = false;
    let mut hasSparseSolver: bool = false;
    let mut forcedTearing: bool = false;
    maxSize = Flags::getConfigInt(if (isLinear.clone()) {Flags::MAX_SIZE_LINEAR_TEARING.clone()} else {Flags::MAX_SIZE_NONLINEAR_TEARING.clone()})?;
    if maxSize.clone() == 0 {
        return Ok(activateTearing);
    }
    isDense = Flags::getConfigString(Flags::MATRIX_FORMAT.clone())? == literal!("dense");
    hasSparseSolver = listMember((Config::simCodeTarget()?).clone(), if (isLinear.clone()) {withLSS.clone()} else {withNSS.clone()});
    forcedTearing = isDense.clone() && !(hasSparseSolver.clone());
    if numVars.clone() > maxSize.clone() && !(forcedTearing.clone()) {
        Error::addMessage(Error::MAX_TEARING_SIZE.clone(), list![(intString(strongComponentIndex.clone())).clone(), (intString(numVars.clone())).clone(), (if (isLinear.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone(), (intString(maxSize.clone())).clone(), (if (isLinear.clone()) {literal!("maxSizeLinearTearing")} else {literal!("maxSizeNonlinearTearing")}).clone()])?;
        return Ok(activateTearing);
    }
    if listMember(strongComponentIndex.clone(), Flags::getConfigIntList(Flags::NO_TEARING_FOR_COMPONENT.clone())?) {
        if debugFlag.clone() {
            println!("{}", (literal!("\nTearing deactivated by user.\n")).clone());
        }
        Error::addMessage(Error::NO_TEARING_FOR_COMPONENT.clone(), list![(intString(strongComponentIndex.clone())).clone()])?;
        return Ok(activateTearing);
    }
    activateTearing = true;
    Ok(activateTearing)
}

fn getUserTearingSet(mut userTVars: Arc<metamodelica::List<i32>>, mut userResiduals: Arc<metamodelica::List<i32>>, mut strongComponentIndex: i32) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut userTvarsThisComponent: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut userResidualsThisComponent: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut start: i32 = 0;
    let mut end_: i32 = 0;
    let mut arr_TVars: metamodelica::Array<i32>;
    let mut arr_residuals: metamodelica::Array<i32>;
    arr_TVars = metamodelica::arrayFromVec(userTVars.clone().into_iter().cloned().collect());
    arr_residuals = metamodelica::arrayFromVec(userResiduals.clone().into_iter().cloned().collect());
    i = 1;
    len = (userTVars.clone().len() as i32);
    while i.clone() < len.clone() {
        if arr_TVars.borrow()[(i.clone()-1) as usize].clone() == strongComponentIndex.clone() {
            start = i.clone() + 2;
            end_ = i.clone() + 1 + arr_TVars.borrow()[(i.clone() + 1-1) as usize].clone();
            userTvarsThisComponent = List::unique({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut j in (start.clone()..=end_.clone()).into_iter() {
            let __x = arr_TVars.borrow()[(j.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if (userTvarsThisComponent.clone().len() as i32) != arr_TVars.borrow()[(i.clone() + 1-1) as usize].clone() {
                Error::addMessage(Error::USER_DEFINED_TEARING_ERROR.clone(), list![(literal!("The selected tearing variables must have unique indexes.")).clone()])?;
                bail!("fail");
            }
            break;
        } else {
            i = i.clone() + 2 + arr_TVars.borrow()[(i.clone() + 1-1) as usize].clone();
        }
    }
    if !(userTvarsThisComponent.clone().is_empty()) {
        i = 1;
        len = (userResiduals.clone().len() as i32);
        while i.clone() < len.clone() {
            if arr_residuals.borrow()[(i.clone()-1) as usize].clone() == strongComponentIndex.clone() {
                start = i.clone() + 2;
                end_ = i.clone() + 1 + arr_residuals.borrow()[(i.clone() + 1-1) as usize].clone();
                userResidualsThisComponent = List::unique({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut j in (start.clone()..=end_.clone()).into_iter() {
            let __x = arr_residuals.borrow()[(j.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                if (userResidualsThisComponent.clone().len() as i32) != arr_residuals.borrow()[(i.clone() + 1-1) as usize].clone() {
                    Error::addMessage(Error::USER_DEFINED_TEARING_ERROR.clone(), list![(literal!("The selected residual equations must have unique indexes.")).clone()])?;
                    bail!("fail");
                }
                break;
            } else {
                i = i.clone() + 2 + arr_residuals.borrow()[(i.clone() + 1-1) as usize].clone();
            }
        }
    }
    Ok((userTvarsThisComponent, userResidualsThisComponent))
}

// =============================================================================
//
// method: omc tearing
//
// =============================================================================
fn getUnsolvableVars(mut size: i32, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut unsolvables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut isUnsolvable: bool = false;
    for mut index in 1..=size.clone() {
        isUnsolvable = unsolvable(meT.borrow()[(index.clone()-1) as usize].clone())?;
        if isUnsolvable.clone() {
            unsolvables = cons(index.clone(), unsolvables.clone());
        }
    }
    Ok(unsolvables)
}

pub fn unsolvable(mut elem: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<bool> {
    let mut isUnsolvable: bool = true;
    let mut e: i32 = 0;
    let mut s: BackendDAE::Solvability = BackendDAE::Solvability::SOLVABILITY_CONSTONE;
    for mut el in &*elem.clone() {
        let mut el = el.clone();
        (e, s, _) = el.clone();
        if solvable(s.clone())? {
            if e.clone() > 0 {
                isUnsolvable = false;
                return Ok(isUnsolvable);
            }
        }
    }
    Ok(isUnsolvable)
}

fn unassignTVars(mut v: i32, mut inAss: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut outAss: metamodelica::Array<i32>;
    outAss = {let _arr = inAss.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = -1; _arr};
    Ok(outAss)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDependenciesOfVars(mut iComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut visited: metamodelica::Array<i32>, mut iMark: i32) -> Result<i32> {
    let mut oMark: i32 = 0;
    oMark = (::match_deref::match_deref! { match &((iComps.clone(), ass1.clone(), ass2.clone(), m.clone(), mT.clone(), visited.clone(), iMark.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
            iMark.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: c, tail: Deref @ metamodelica::List::Nil }, tail: comps }, _, _, _, _, _, _) => {
            let mut v: i32 = 0;
            let mut tvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            v = ass2.borrow()[(c.clone()-1) as usize].clone();
            vars = List::select(m.borrow()[(c.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
            tvars = tVarsofEqn(vars.clone(), ass1.clone(), mT.clone(), visited.clone(), iMark.clone(), metamodelica::nil())?;
            {let _arr = mT.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = tvars.clone(); _arr};
            getDependenciesOfVars(comps.clone(), ass1.clone(), ass2.clone(), m.clone(), mT.clone(), visited.clone(), iMark.clone() + 1)?
        },
        (Deref @ metamodelica::List::Cons { head: comp, tail: comps }, _, _, _, _, _, _) => {
            let mut tvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            vars = List::map1r(comp.clone(), Arc::new(arrayGet.clone()), ass2.clone());
            tvars = tVarsofEqns(comp.clone(), m.clone(), ass1.clone(), mT.clone(), visited.clone(), iMark.clone())?;
            let _ = List::fold1r(vars.clone(), Arc::new(arrayUpdate.clone()), tvars.clone(), mT.clone());
            getDependenciesOfVars(comps.clone(), ass1.clone(), ass2.clone(), m.clone(), mT.clone(), visited.clone(), iMark.clone() + 1)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oMark)
}

fn tVarsofEqns(mut iEqns: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut visited: metamodelica::Array<i32>, mut iMark: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut e in &*iEqns.clone() {
        let mut e = e.clone();
        vars = List::select(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
        oAcc = tVarsofEqn(vars.clone(), ass1.clone(), mT.clone(), visited.clone(), iMark.clone(), oAcc.clone())?;
    }
    Ok(oAcc)
}

fn tVarsofEqn(mut iVars: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut visited: metamodelica::Array<i32>, mut iMark: i32, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = iAcc.clone();
    for mut v in &*iVars.clone() {
        let mut v = v.clone();
        if intLt(ass1.borrow()[(v.clone()-1) as usize].clone(), 0) {
            oAcc = uniqueIntLst(v.clone(), iMark.clone(), visited.clone(), oAcc.clone())?;
        } else {
            oAcc = List::fold2(mT.borrow()[(v.clone()-1) as usize].clone(), (std::sync::Arc::new(uniqueIntLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iMark.clone(), visited.clone(), oAcc.clone());
        }
    }
    Ok(oAcc)
}

fn uniqueIntLst(mut c: i32, mut mark: i32, mut markarray: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = iAcc.clone();
    if !(intEq(mark.clone(), markarray.borrow()[(c.clone()-1) as usize].clone())) {
        {let _arr = markarray.clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = mark.clone(); _arr};
        oAcc = cons(c.clone(), oAcc.clone());
    }
    Ok(oAcc)
}

fn sortResidualDepentOnTVars(mut iResiduals: Arc<metamodelica::List<i32>>, mut iTVars: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut visited: metamodelica::Array<i32>, mut iMark: i32) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut oResiduals: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oMark: i32 = 0;
    let mut size: i32 = 0;
    let mut maplst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqnLocalGlobal: metamodelica::Array<i32>;
    let mut varGlobalLocal: metamodelica::Array<i32>;
    let mut v1: metamodelica::Array<i32>;
    let mut v2: metamodelica::Array<i32>;
    eqnLocalGlobal = metamodelica::arrayFromVec(iResiduals.clone().into_iter().cloned().collect());
    varGlobalLocal = arrayCreate((m.clone().borrow().len() as i32), -1);
    varGlobalLocal = getGlobalLocal(iTVars.clone(), 1, varGlobalLocal.clone())?;
    (oMark, maplst) = tVarsofResidualEqns(iResiduals.clone(), m.clone(), ass1.clone(), mT.clone(), varGlobalLocal.clone(), visited.clone(), iMark.clone())?;
    map = metamodelica::arrayFromVec(maplst.clone().into_iter().cloned().collect());
    size = (map.clone().borrow().len() as i32);
    Matching::matchingExternalsetAdjacencyMatrix(size.clone(), size.clone(), map.clone());
    BackendDAEEXT::matching(size.clone(), size.clone(), 5, -1, metamodelica::OrderedFloat(1.0_f64), 1);
    v1 = arrayCreate(size.clone(), -1);
    v2 = arrayCreate(size.clone(), -1);
    BackendDAEEXT::getAssignment(v2.clone(), v1.clone())?;
    oResiduals = getTVarResiduals(size.clone(), v1.clone(), eqnLocalGlobal.clone(), metamodelica::nil());
    Ok((oResiduals, oMark))
}

fn getGlobalLocal(mut iTVars: Arc<metamodelica::List<i32>>, mut index: i32, mut iVarGlobalLocal: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oVarGlobalLocal: metamodelica::Array<i32> = iVarGlobalLocal.clone();
    let mut idx: i32 = index.clone();
    for mut i in &*iTVars.clone() {
        let mut i = i.clone();
        {let _arr = oVarGlobalLocal.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = idx.clone(); _arr};
        idx = idx.clone() + 1;
    }
    Ok(oVarGlobalLocal)
}

fn tVarsofResidualEqns(mut iEqns: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varGlobalLocal: metamodelica::Array<i32>, mut visited: metamodelica::Array<i32>, mut iMark: i32) -> Result<(i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut oMark: i32 = iMark.clone();
    let mut oAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    oAcc = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut eq in (iEqns.clone()).into_iter().cloned() {
            let __x = (match eq.clone() {
        mut e => {
            let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            vars = List::select(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
            tvars = tVarsofEqn(vars.clone(), ass1.clone(), mT.clone(), visited.clone(), oMark.clone(), metamodelica::nil())?;
            tvars = List::map1r(tvars.clone(), Arc::new(arrayGet.clone()), varGlobalLocal.clone());
            oMark = oMark.clone() + 1;
            tvars.clone()
        },
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((oMark, oAcc))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getTVarResiduals(mut index: i32, mut v1: metamodelica::Array<i32>, mut eqnLocalGlobal: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oAcc = (::match_deref::match_deref! { match &((index.clone(), v1.clone(), eqnLocalGlobal.clone(), iAcc.clone())) {
        (0, _, _, _) => {
            iAcc.clone()
        },
        (_, _, _, _) => {
            let mut e: i32 = 0;
            e = v1.borrow()[(index.clone()-1) as usize].clone();
            e = eqnLocalGlobal.borrow()[(e.clone()-1) as usize].clone();
            getTVarResiduals(index.clone() - 1, v1.clone(), eqnLocalGlobal.clone(), cons(e.clone(), iAcc.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oAcc
}

fn omcTearing2(mut unsolvables: Arc<metamodelica::List<i32>>, mut tSel_always: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut size: i32, mut vars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut columark: metamodelica::Array<i32>, mut mark: i32, mut inTVars: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut outTVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oMark: i32 = 0;
    (outTVars, oMark) = 'mc: {
        let __mc_input = (unsolvables.clone(), tSel_always.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut tvar: i32 = 0;
                    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vareqns: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut oMark: i32 = oMark.clone();
                    let mut outTVars: Arc<metamodelica::List<i32>> = outTVars.clone();
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of omcTearingSelectTearingVar\n\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    tvar = omcTearingSelectTearingVar(vars.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone())?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of omcTearingSelectTearingVar\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    {let _arr = ass1.clone(); _arr.borrow_mut()[(tvar.clone()-1) as usize] = size.clone() * 2; _arr};
                    vareqns = List::removeOnTrue(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), mt.borrow()[(tvar.clone()-1) as usize].clone());
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", (literal!("Assignable equations containing new tvar:\n")).clone());
                        BackendDump::dumpAdjacencyRowEnhanced(vareqns.clone())?;
                        println!("{}", (literal!("\n")).clone());
                    }
                    tearingBFS(vareqns.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    unassigned = Matching::getUnassigned(size.clone(), ass1.clone(), metamodelica::nil());
                    (outTVars, oMark) = omcTearing3(unassigned.clone(), metamodelica::nil(), tSel_always.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), vars.clone(), ishared.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone() + 1, cons(tvar.clone(), inTVars.clone()))?;
                    Ok((outTVars.clone(), oMark.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: tvar, tail: rest }, Deref @ metamodelica::List::Nil) => {
                    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vareqns: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut oMark: i32 = oMark.clone();
                    let mut outTVars: Arc<metamodelica::List<i32>> = outTVars.clone();
                    if listMember(tvar.clone(), tSel_never.clone()) {
                        Error::addCompilerWarning((literal!("There are tearing variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.never'. Use -d=tearingdump and -d=tearingdumpV for more information.")).clone())?;
                    }
                    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nForced selection of Tearing Variable:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tVar: ")); __mm_s.push_str(&*intString(tvar.clone())); __mm_s.push_str(&*literal!(" (unsolvable in omcTearing2)\n\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    {let _arr = ass1.clone(); _arr.borrow_mut()[(tvar.clone()-1) as usize] = size.clone() * 2; _arr};
                    vareqns = List::removeOnTrue(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), mt.borrow()[(tvar.clone()-1) as usize].clone());
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", (literal!("Assignable equations containing new tvar:\n")).clone());
                        BackendDump::dumpAdjacencyRowEnhanced(vareqns.clone())?;
                        println!("{}", (literal!("\n")).clone());
                    }
                    tearingBFS(vareqns.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    unassigned = Matching::getUnassigned(size.clone(), ass1.clone(), metamodelica::nil());
                    (outTVars, oMark) = omcTearing3(unassigned.clone(), rest.clone(), tSel_always.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), vars.clone(), ishared.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone() + 1, cons(tvar.clone(), inTVars.clone()))?;
                    Ok((outTVars.clone(), oMark.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unsolv: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vareqns: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut outTVars: Arc<metamodelica::List<i32>> = outTVars.clone();
                    let mut oMark: i32 = oMark.clone();
                    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nForced selection of Tearing Variables:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variables with annotation attribute 'always' as tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(tSel_always.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    markTVarsOrResiduals(tSel_always.clone(), ass1.clone())?;
                    (_, unsolv, _) = List::intersection1OnTrue(unsolvables.clone(), tSel_always.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    vareqns = findVareqns(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), mt.clone(), tSel_always.clone());
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", (literal!("Assignable equations containing new tvars:\n")).clone());
                        BackendDump::dumpAdjacencyRowEnhanced(vareqns.clone())?;
                        println!("{}", (literal!("\n")).clone());
                    }
                    tearingBFS(vareqns.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    unassigned = Matching::getUnassigned(size.clone(), ass1.clone(), metamodelica::nil());
                    (outTVars, oMark) = omcTearing3(unassigned.clone(), unsolv.clone(), metamodelica::nil(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), vars.clone(), ishared.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone() + 1, listAppend(tSel_always.clone(), inTVars.clone()))?;
                    Ok((outTVars.clone(), oMark.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("Tearing.omcTearing2 failed!")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outTVars, oMark))
}

fn findVareqns(mut ass2In: metamodelica::Array<i32>, mut inCompFunc: Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut tSel_alwaysIn: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> {
    pub type CompFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>;

    let mut vareqnsOut: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    for mut tvar in &*tSel_alwaysIn.clone() {
        let mut tvar = tvar.clone();
        vareqnsOut = List::append_reverse(List::removeOnTrue(ass2In.clone(), inCompFunc.clone(), mt.borrow()[(tvar.clone()-1) as usize].clone()), vareqnsOut.clone());
    }
    vareqnsOut = List::unique(vareqnsOut.clone());
    vareqnsOut
}

fn omcTearingSelectTearingVar(mut vars: BackendDAE::Variables, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>) -> Result<i32> {
    let mut tearingVar: i32 = 0;
    tearingVar = 'mc: {
        let __mc_input = (vars.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let mut unsolvables: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut tvar: i32 = 0;
                    unsolvables = getUnsolvableVarsConsiderMatching(BackendVariable::varsSize(vars.clone())?, mt.clone(), ass1.clone(), ass2.clone())?;
                    let false = (unsolvables.clone().is_empty()) else { bail!("pattern mismatch") };
                    tvar = listHead(unsolvables.clone())?;
                    if listMember(tvar.clone(), tSel_never.clone()) {
                        Error::addCompilerWarning((literal!("There are tearing variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.never'. Use -d=tearingdump and -d=tearingdumpV for more information.")).clone())?;
                    }
                    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nForced selection of Tearing Variable:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tVar: ")); __mm_s.push_str(&*intString(tvar.clone())); __mm_s.push_str(&*literal!(" (unsolvable in omcTearingSelectTearingVar)\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(tvar.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let mut freeVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pointsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut tvar: i32 = 0;
                    let mut varsize: i32 = 0;
                    let mut points: metamodelica::Array<i32>;
                    varsize = BackendVariable::varsSize(vars.clone())?;
                    freeVars = Matching::getUnassigned(varsize.clone(), ass1.clone(), metamodelica::nil());
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", (literal!("omcTearingSelectTearingVar Candidates(unassigned vars):\n")).clone());
                        BackendDump::debuglst(freeVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(", ")).clone(), (literal!("\n")).clone());
                    }
                    (_, freeVars, _) = List::intersection1OnTrue(freeVars.clone(), tSel_never.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", (literal!("Candidates without variables with annotation attribute 'never':\n")).clone());
                        BackendDump::debuglst(freeVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(", ")).clone(), (literal!("\n")).clone());
                    }
                    let false = (freeVars.clone().is_empty()) else { bail!("pattern mismatch") };
                    points = arrayCreate(varsize.clone(), 0);
                    points = List::fold2(freeVars.clone(), (std::sync::Arc::new(calcVarWeights) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), mt.clone(), ass2.clone(), points.clone());
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPoints after 'calcVarWeights':\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(points.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    eqns = Matching::getUnassigned((m.clone().borrow().len() as i32), ass2.clone(), metamodelica::nil());
                    points = List::fold2(eqns.clone(), (std::sync::Arc::new(addEqnWeights) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), m.clone(), ass1.clone(), points.clone());
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Points after 'addEqnWeights':\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(points.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    points = List::fold1(freeVars.clone(), (std::sync::Arc::new(discriminateDiscrete) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), vars.clone(), points.clone());
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Points after 'discriminateDiscrete':\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(points.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    pointsLst = preferAvoidVariables(freeVars.clone(), Arc::new(points.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), tSel_prefer.clone(), metamodelica::OrderedFloat(3.0_f64));
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Points after preferring variables with attribute 'prefer':\n")); __mm_s.push_str(&*stringDelimitList(List::map(pointsLst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    pointsLst = preferAvoidVariables(freeVars.clone(), pointsLst.clone(), tSel_avoid.clone(), metamodelica::OrderedFloat(0.334_f64));
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Points after discrimination against variables with attribute 'avoid':\n")); __mm_s.push_str(&*stringDelimitList(List::map(pointsLst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    tvar = selectVarWithMostPoints(freeVars.clone(), pointsLst.clone());
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tVar: ")); __mm_s.push_str(&*intString(tvar.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((pointsLst.clone()).get(tvar.clone())?)); __mm_s.push_str(&*literal!(" points)\n\n")); ArcStr::from(__mm_s) }).clone());
                    } else if listMember(tvar.clone(), tSel_avoid.clone()) {
                        Error::addCompilerWarning((literal!("The Tearing heuristic has chosen variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.avoid'. Use -d=tearingdump and -d=tearingdumpV for more information.")).clone())?;
                    }
                    Ok(tvar.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("omcTearingSelectTearingVar failed because no unmatched var!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tearingVar)
}

fn getUnsolvableVarsConsiderMatching(mut size: i32, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut unsolvables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut elem: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut isUnsolvable: bool = false;
    for mut index in 1..=size.clone() {
        if intLt(ass1.borrow()[(index.clone()-1) as usize].clone(), 0) {
            elem = meT.borrow()[(index.clone()-1) as usize].clone();
            elem = removeMatched(elem.clone(), ass2.clone());
            isUnsolvable = unsolvable(elem.clone())?;
            if isUnsolvable.clone() {
                unsolvables = cons(index.clone(), unsolvables.clone());
            }
        }
    }
    Ok(unsolvables)
}

fn removeMatched(mut elem: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut ass2: metamodelica::Array<i32>) -> Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> {
    let mut oAcc: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut e: i32 = 0;
    for mut el in &*elem.clone() {
        let mut el = el.clone();
        (e, _, _) = el.clone();
        if intGt(e.clone(), 0) && intLt(ass2.borrow()[(e.clone()-1) as usize].clone(), 0) {
            oAcc = cons(el.clone(), oAcc.clone());
        }
    }
    oAcc
}

fn calcVarWeights(mut v: i32, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass2: metamodelica::Array<i32>, mut iPoints: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oPoints: metamodelica::Array<i32>;
    let mut p: i32 = 0;
    p = calcSolvabilityWeight(mt.borrow()[(v.clone()-1) as usize].clone(), ass2.clone());
    oPoints = {let _arr = iPoints.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = p.clone(); _arr};
    Ok(oPoints)
}

fn calcSolvabilityWeight(mut inRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut ass2: metamodelica::Array<i32>) -> i32 {
    let mut w: i32 = 0;
    w = List::fold1(inRow.clone(), (std::sync::Arc::new(fnptr!(solvabilityWeightsnoStates, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn((i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), metamodelica::Array<i32>, i32) -> Result<i32> + 'static>), ass2.clone(), 0);
    w
}

fn solvabilityWeightsnoStates(mut inTpl: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), mut ass: metamodelica::Array<i32>, mut iW: i32) -> i32 {
    let mut oW: i32 = 0;
    oW = (::match_deref::match_deref! { match &((inTpl.clone(), ass.clone(), iW.clone())) {
        ((eq, s, _), _, _) if (intGt(eq.clone(), 0) && !(intGt(ass.borrow()[(eq.clone()-1) as usize].clone(), 0))) => {
            let mut w: i32 = 0;
            w = solvabilityWeights(s.clone());
            intAdd(w.clone(), iW.clone())
        },
        _ => {
            iW.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oW
}

fn solvabilityWeights(mut solva: BackendDAE::Solvability) -> i32 {
    let mut i: i32 = 0;
    i = (match solva.clone() {
        BackendDAE::Solvability::SOLVABILITY_SOLVED => 0,
        BackendDAE::Solvability::SOLVABILITY_CONSTONE => 2,
        BackendDAE::Solvability::SOLVABILITY_CONST { .. } => 5,
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false } => 0,
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true } => 50,
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: false } => 0,
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: true } => 100,
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR => 200,
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE => 300,
        _ => 0,
    });
    i
}

fn addEqnWeights(mut e: i32, mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1: metamodelica::Array<i32>, mut iPoints: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oPoints: metamodelica::Array<i32>;
    oPoints = 'mc: {
        let __mc_input = (e.clone(), m.clone(), ass1.clone(), iPoints.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut v1: i32 = 0;
            let mut v2: i32 = 0;
            let mut points: metamodelica::Array<i32>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::removeOnTrue(ass1.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), m.borrow()[(e.clone()-1) as usize].clone())) {
                Deref @ metamodelica::List::Cons { head: (__pa0, _, _), tail: Deref @ metamodelica::List::Cons { head: (__pa1, _, _), tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            v2 = __pa1.clone();
            points = {let _arr = iPoints.clone(); let _val = iPoints.borrow()[(v1.clone()-1) as usize].clone() + 5; _arr.borrow_mut()[(v1.clone()-1) as usize] = _val; _arr};
            points = {let _arr = iPoints.clone(); let _val = points.borrow()[(v2.clone()-1) as usize].clone() + 5; _arr.borrow_mut()[(v2.clone()-1) as usize] = _val; _arr};
            Ok(points.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iPoints.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oPoints)
}

fn isAssignedSaveEnhanced(mut ass: metamodelica::Array<i32>, mut inTpl: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &((ass.clone(), inTpl.clone())) {
        (_, (i, _, _)) if (intGt(i.clone(), 0)) => {
            intGt(ass.borrow()[(i.clone()-1) as usize].clone(), 0)
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn discriminateDiscrete(mut v: i32, mut vars: BackendDAE::Variables, mut iPoints: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oPoints: metamodelica::Array<i32>;
    let mut p: i32 = 0;
    let mut b: bool = false;
    let mut var: BackendDAE::Var;
    var = BackendVariable::getVarAt(vars.clone(), v.clone())?;
    b = BackendVariable::isVarDiscrete(var.clone());
    p = iPoints.borrow()[(v.clone()-1) as usize].clone();
    p = if (b.clone()) {intDiv(p.clone(), 10)} else {p.clone()};
    oPoints = {let _arr = iPoints.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = p.clone(); _arr};
    Ok(oPoints)
}

fn selectVarWithMostPoints(mut vars: Arc<metamodelica::List<i32>>, mut points: Arc<metamodelica::List<i32>>) -> i32 {
    let mut oVar: i32 = -1;
    let mut defp: i32 = -1;
    let mut p: i32 = 0;
    for mut v in &*vars.clone() {
        let mut v = v.clone();
        p = (points.clone()).get(v.clone()).unwrap();
        if p.clone() > defp.clone() {
            defp = p.clone();
            oVar = v.clone();
        }
    }
    oVar
}

fn tearingBFS(mut queue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut size: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut nextQueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((queue.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), ass1.clone(), ass2.clone(), nextQueue.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _) => {
            let mut newqueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
            newqueue = List::removeOnTrue(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), nextQueue.clone());
            newqueue = sortEqnsSolvable(newqueue.clone(), m.clone());
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("Use next Queue!\n")).clone());
            }
            tearingBFS(newqueue.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: (c, _, _), tail: rest }, _, _, _, _, _, _, _, _) => {
            let mut eqnsize: i32 = 0;
            let mut cnonscalar: i32 = 0;
            let mut newqueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
            let mut rows: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("Queue:\n")).clone());
                BackendDump::dumpAdjacencyRowEnhanced(queue.clone())?;
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Process Eqn: ")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            rows = List::removeOnTrue(ass1.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), m.borrow()[(c.clone()-1) as usize].clone());
            cnonscalar = mapIncRowEqn.borrow()[(c.clone()-1) as usize].clone();
            eqnsize = (mapEqnIncRow.borrow()[(cnonscalar.clone()-1) as usize].clone().len() as i32);
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Eqn Size: ")); __mm_s.push_str(&*intString(eqnsize.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Rows (not assigned variables in eqn ")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!("):\n")); ArcStr::from(__mm_s) }).clone());
                BackendDump::dumpAdjacencyRowEnhanced(rows.clone())?;
                println!("{}", (literal!("\n")).clone());
            }
            newqueue = tearingBFS1(rows.clone(), eqnsize.clone(), mapEqnIncRow.borrow()[(cnonscalar.clone()-1) as usize].clone(), mt.clone(), ass1.clone(), ass2.clone(), nextQueue.clone())?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("Next Queue:\n")).clone());
                BackendDump::dumpAdjacencyRowEnhanced(newqueue.clone())?;
                println!("{}", (literal!("\n\n")).clone());
            }
            tearingBFS(rest.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), ass1.clone(), ass2.clone(), newqueue.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn sortEqnsSolvable(mut queue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> {
    let mut nextQueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut qnon: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut qsolv: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    (qnon, qsolv) = List::split1OnTrue(queue.clone(), (std::sync::Arc::new(fnptr!(hasnonlinearVars, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>)) as std::sync::Arc<dyn ::std::ops::Fn((i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<bool> + 'static>), m.clone());
    nextQueue = listAppend(qsolv.clone(), qnon.clone());
    nextQueue
}

fn hasnonlinearVars(mut entry: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> bool {
    let mut hasnonlinear: bool = false;
    let mut r: i32 = 0;
    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    (r, _, _) = entry.clone();
    row = m.borrow()[(r.clone()-1) as usize].clone();
    hasnonlinear = hasnonlinearVars1(row.clone());
    hasnonlinear
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasnonlinearVars1(mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> bool {
    let mut hasnonlinear: bool = false;
    hasnonlinear = (::match_deref::match_deref! { match &(row.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: (_, BackendDAE::Solvability::SOLVABILITY_NONLINEAR, _), tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            hasnonlinearVars1(rest.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasnonlinear
}

fn tearingBFS1(mut rows: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut size: i32, mut c: Arc<metamodelica::List<i32>>, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inNextQueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> {
    let mut outNextQueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    outNextQueue = (::match_deref::match_deref! { match &((rows.clone(), size.clone(), c.clone(), mt.clone(), ass1.clone(), ass2.clone(), inNextQueue.clone())) {
        (_, _, _, _, _, _, _) if (intEq((rows.clone().len() as i32), size.clone()) && solvableLst(rows.clone())) => {
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Assign Eqns: ")); __mm_s.push_str(&*stringDelimitList(List::map(c.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            tearingBFS2(rows.clone(), c.clone(), mt.clone(), ass1.clone(), ass2.clone(), inNextQueue.clone())?
        },
        _ => inNextQueue.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNextQueue)
}

fn solvableLst(mut rows: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> bool {
    let mut solvable: bool = true;
    let mut s: BackendDAE::Solvability = BackendDAE::Solvability::SOLVABILITY_CONSTONE;
    for mut r in &*rows.clone() {
        let mut r = r.clone();
        (_, s, _) = r.clone();
        if !(self::solvable(s.clone()).unwrap()) {
            solvable = false;
            return solvable;
        }
    }
    solvable
}

fn solvable(mut s: BackendDAE::Solvability) -> Result<bool> {
    let mut b: bool = false;
    b = (match s.clone() {
        BackendDAE::Solvability::SOLVABILITY_SOLVED => true,
        BackendDAE::Solvability::SOLVABILITY_CONSTONE => true,
        BackendDAE::Solvability::SOLVABILITY_CONST { b: mut b } => b.clone(),
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: mut b } => b.clone() && !(stringEqual((Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?).clone(), (literal!("veryStrict")).clone())),
        BackendDAE::Solvability::SOLVABILITY_LINEAR { .. } => false,
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR => false,
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE => false,
        BackendDAE::Solvability::SOLVABILITY_SOLVABLE => true,
        _ => false,
    });
    Ok(b)
}

fn isEntrySolved(mut entry: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(entry.clone()) {
        (_, BackendDAE::Solvability::SOLVABILITY_SOLVED, _) => true,
        (_, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b }, _) => {
            Error::addInternalError((literal!("SOLVABILITY_PARAMETER is not handled yet. Requires revision.")).clone(), metamodelica::sourceInfo!())?;
            b.clone() && !(stringEqual((Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?).clone(), (literal!("veryStrict")).clone()))
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn isEntrySolvable(mut entry: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> {
    let mut b: bool = false;
    b = solvable(Util::tuple32(entry.clone()))?;
    Ok(b)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn tearingBFS2(mut rows: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut clst: Arc<metamodelica::List<i32>>, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inNextQueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> {
    let mut outNextQueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    outNextQueue = (::match_deref::match_deref! { match &((rows.clone(), clst.clone(), mt.clone(), ass1.clone(), ass2.clone(), inNextQueue.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            inNextQueue.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (r, _, _), tail: rest }, Deref @ metamodelica::List::Cons { head: c, tail: ilst }, _, _, _, _) => {
            let mut vareqns: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
            let mut newqueue: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Assignment: Eq ")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!(" - Var ")); __mm_s.push_str(&*intString(r.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            {let _arr = ass1.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = c.clone(); _arr};
            {let _arr = ass2.clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = r.clone(); _arr};
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ass1: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ass2: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            vareqns = List::removeOnTrue(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), mt.borrow()[(r.clone()-1) as usize].clone());
            newqueue = listAppend(inNextQueue.clone(), vareqns.clone());
            tearingBFS2(rest.clone(), ilst.clone(), mt.clone(), ass1.clone(), ass2.clone(), newqueue.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNextQueue)
}

fn omcTearing3(mut unassigned: Arc<metamodelica::List<i32>>, mut unsolvables: Arc<metamodelica::List<i32>>, mut tSel_always: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut size: i32, mut vars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut columark: metamodelica::Array<i32>, mut mark: i32, mut inTVars: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut outTVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oMark: i32 = 0;
    (outTVars, oMark) = (::match_deref::match_deref! { match &((unassigned.clone(), unsolvables.clone(), tSel_always.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), vars.clone(), ishared.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone(), inTVars.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => (inTVars.clone(), mark.clone()),
        _ => {
            (outTVars, oMark) = omcTearing2(unsolvables.clone(), tSel_always.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), size.clone(), vars.clone(), ishared.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone(), inTVars.clone())?;
            (outTVars.clone(), oMark.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outTVars, oMark))
}

fn omcTearing4(mut jacType: BackendDAE::JacobianType, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut subsyst: Arc<BackendDAE::EqSystem>, mut tvars: Arc<metamodelica::List<i32>>, mut residual: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut othercomps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut eindex: Arc<metamodelica::List<i32>>, mut vindx: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut columark: metamodelica::Array<i32>, mut mark: i32, mut mixedSystem: bool) -> Result<(Arc<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: Arc<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool = false;
    (ocomp, outRunMatching) = 'mc: {
        let __mc_input = (jacType.clone(), isyst.clone(), ishared.clone(), subsyst.clone(), tvars.clone(), residual.clone(), ass1.clone(), ass2.clone(), othercomps.clone(), eindex.clone(), vindx.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), columark.clone(), mark.clone(), mixedSystem.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut ores: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut residual1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut ovars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
                    let mut eindxarr: metamodelica::Array<i32>;
                    let mut varindxarr: metamodelica::Array<i32>;
                    let mut linear: bool = false;
                    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                        println!("{}", (literal!("handle torn System\n")).clone());
                    }
                    residual1 = List::map1r(residual.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
                    residual1 = List::fold2(residual1.clone(), (std::sync::Arc::new(uniqueIntLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mark.clone(), columark.clone(), metamodelica::nil());
                    eindxarr = metamodelica::arrayFromVec(eindex.clone().into_iter().cloned().collect());
                    ores = List::map1r(residual1.clone(), Arc::new(arrayGet.clone()), eindxarr.clone());
                    varindxarr = metamodelica::arrayFromVec(vindx.clone().into_iter().cloned().collect());
                    ovars = List::map1r(tvars.clone(), Arc::new(arrayGet.clone()), varindxarr.clone());
                    innerEquations = omcTearing4_1(othercomps.clone(), ass2.clone(), mapIncRowEqn.clone(), eindxarr.clone(), varindxarr.clone(), columark.clone(), mark.clone())?;
                    linear = BackendDAEUtil::getLinearfromJacType(jacType.clone())?;
                    Ok((Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ovars.clone(), residualequations: ores.clone(), innerEquations: innerEquations.clone(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) }, casualTearingSet: None, linear: linear.clone(), mixedSystem: mixedSystem.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    Ok((Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: metamodelica::nil(), residualequations: metamodelica::nil(), innerEquations: metamodelica::nil(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) }, casualTearingSet: None, linear: false, mixedSystem: mixedSystem.clone() }), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((ocomp, outRunMatching))
}

fn omcTearing4_1(mut othercomps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut ass2: metamodelica::Array<i32>, mut mapIncRowEqn: metamodelica::Array<i32>, mut eindxarr: metamodelica::Array<i32>, mut varindxarr: metamodelica::Array<i32>, mut columark: metamodelica::Array<i32>, mut mark: i32) -> Result<Arc<metamodelica::List<BackendDAE::InnerEquation>>> {
    let mut outInnerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    outInnerEquations = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
        for mut x in (othercomps.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(x.clone()) {
        Deref @ metamodelica::List::Cons { head: c, tail: Deref @ metamodelica::List::Nil } => {
            let mut e: i32 = 0;
            let mut v: i32 = 0;
            e = mapIncRowEqn.borrow()[(c.clone()-1) as usize].clone();
            e = eindxarr.borrow()[(e.clone()-1) as usize].clone();
            v = ass2.borrow()[(c.clone()-1) as usize].clone();
            v = varindxarr.borrow()[(v.clone()-1) as usize].clone();
            BackendDAE::InnerEquation::INNEREQUATION { vars: list![v.clone()], eqn: e.clone() }
        },
        clst => {
            let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut e: i32 = 0;
            elst = List::map1r(clst.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
            elst = List::fold2(elst.clone(), (std::sync::Arc::new(uniqueIntLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mark.clone(), columark.clone(), metamodelica::nil());
            let __pa0 = ::match_deref::match_deref! { match &(elst.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            e = eindxarr.borrow()[(e.clone()-1) as usize].clone();
            vlst = List::map1r(clst.clone(), Arc::new(arrayGet.clone()), ass2.clone());
            vlst = List::map1r(vlst.clone(), Arc::new(arrayGet.clone()), varindxarr.clone());
            BackendDAE::InnerEquation::INNEREQUATION { vars: vlst.clone(), eqn: e.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outInnerEquations)
}

// ============================================================================
// Section for minimal tearing
//   Tear only the minimal amount of variables from strong components which are
//   all discrete variables and CSE variables.
// ============================================================================
fn minimalTearing(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut eindex: Arc<metamodelica::List<i32>>, mut vindx: Arc<metamodelica::List<i32>>, mut jacType: BackendDAE::JacobianType, mut mixedSystem: bool) -> Result<Arc<BackendDAE::StrongComponent>> {
    let mut ocomp: Arc<BackendDAE::StrongComponent>;
    let mut size: i32 = 0;
    let mut qidx: i32 = 0;
    let mut vidx: i32 = 0;
    let mut nE: metamodelica::Array<i32>;
    let mut nV: metamodelica::Array<i32>;
    let mut varArray: metamodelica::Array<bool>;
    let mut eqArray: metamodelica::Array<bool>;
    let mut unsolvedDiscreteVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvedCSEVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvedCombined: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut algSolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut iterationVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut residualequations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut innerEquationsLocalIndex: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut adjEnh: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut adjEnhT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut linear: bool = false;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut subsyst: Arc<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    linear = BackendDAEUtil::getLinearfromJacType(jacType.clone())?;
    match '__try0: {
        eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()));
        eqns = unwrap_break_err!(BackendEquation::listEquation(eqn_lst.clone()), '__try0);
        var_lst = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), BackendVariable::daeVars(isyst.clone()));
        vars = BackendVariable::listVar1(var_lst.clone());
        subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        (adjEnh, adjEnhT) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrixEnhanced(subsyst.clone(), ishared.clone(), BackendDAEUtil::isInitializationDAE(ishared.clone())), '__try0);
        size = (vindx.clone().len() as i32);
        varArray = arrayCreate(size.clone(), true);
        eqArray = arrayCreate(size.clone(), true);
        nE = arrayCreate(size.clone(), -1);
        nV = arrayCreate(size.clone(), -1);
        unsolvedDiscreteVars = unwrap_break_err!(findDiscreteWarnTearingSelect(var_lst.clone()), '__try0);
        unsolvedCSEVars = findCSE(var_lst.clone());
        unsolvedCombined = unwrap_break_err!(List::uniqueIntN(listAppend(unsolvedDiscreteVars.clone(), unsolvedCSEVars.clone()), (var_lst.clone().len() as i32)), '__try0).reverse();
        qidx = 1;
        for mut eqn in &*eqn_lst.clone() {
            let mut eqn = eqn.clone();
            if BackendEquation::isAlgorithm(eqn.clone()) {
                {
                    let __cell1 = false;
                    eqArray.clone().borrow_mut()[(qidx.clone()-1) as usize] = __cell1;
                }
                algSolvedVars = metamodelica::nil();
                let __range2 = &*adjEnh.borrow()[(qidx.clone()-1) as usize].clone();
                for mut entr in __range2 {
                    let mut entr = entr.clone();
                    if unwrap_break_err!(isEntrySolved(entr.clone()), '__try0) {
                        (vidx, _, _) = entr.clone();
                        algSolvedVars = cons(vidx.clone(), algSolvedVars.clone());
                        (unsolvedCombined, _) = unwrap_break_err!(List::deleteMemberOnTrue(vidx.clone(), unsolvedCombined.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)), '__try0);
                        {
                            let __cell3 = false;
                            varArray.clone().borrow_mut()[(vidx.clone()-1) as usize] = __cell3;
                        }
                    }
                }
                innerEquationsLocalIndex = cons(BackendDAE::InnerEquation::INNEREQUATION { eqn: qidx.clone(), vars: algSolvedVars.clone() }, innerEquationsLocalIndex.clone());
            }
            qidx = qidx.clone() + 1;
        }
        if !(unsolvedCombined.clone().is_empty()) {
            unwrap_break_err!(matchDiscreteVars(unsolvedCombined.clone(), adjEnhT.clone(), varArray.clone(), eqArray.clone(), nE.clone(), nV.clone()), '__try0);
            (varArray, eqArray, innerEquations) = unwrap_break_err!(getTearingSetfromAssign(unsolvedCombined.clone(), nE.clone(), varArray.clone(), eqArray.clone()), '__try0);
            for mut iq in &*innerEquations.clone() {
                let mut iq = iq.clone();
                innerEquationsLocalIndex = cons(iq.clone(), innerEquationsLocalIndex.clone());
            }
        }
        for mut i in 1..=(eindex.clone().len() as i32) {
            if eqArray.borrow()[(i.clone()-1) as usize].clone() {
                residualequations = cons(i.clone(), residualequations.clone());
            }
        }
        for mut i in 1..=(vindx.clone().len() as i32) {
            if varArray.borrow()[(i.clone()-1) as usize].clone() {
                iterationVars = cons(i.clone(), iterationVars.clone());
            }
        }
        innerEquations = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
        for mut ieqn in (innerEquationsLocalIndex.clone()).into_iter().cloned() {
            let __x = (match ieqn.clone() {
        BackendDAE::InnerEquation::INNEREQUATION { .. } => {
            let __owned_variant_vars_0 = selectFromList_rev(vindx.clone(), var_field!(ieqn.vars, BackendDAE::InnerEquation::INNEREQUATION).clone());
            let __owned_variant_eqn_1 = unwrap_break_err!((eindex.clone()).get(var_field!(ieqn.eqn, BackendDAE::InnerEquation::INNEREQUATION).clone()), '__try0);
            if let BackendDAE::InnerEquation::INNEREQUATION { vars, eqn, .. } = &mut ieqn {
                *vars = __owned_variant_vars_0;
                *eqn = __owned_variant_eqn_1;
            } else { panic!("owned-variant field-assign: value held a different variant than BackendDAE::InnerEquation::INNEREQUATION"); }
            ieqn.clone()
        },
        _ => break '__try0 Err::<_, _>(anyhow::anyhow!("fail")),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        iterationVars = selectFromList_rev(vindx.clone(), iterationVars.clone());
        residualequations = selectFromList_rev(eindex.clone(), residualequations.clone());
        ocomp = Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: iterationVars.clone().reverse(), residualequations: residualequations.clone().reverse(), innerEquations: innerEquations.clone().reverse(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) }, casualTearingSet: None, linear: linear.clone(), mixedSystem: mixedSystem.clone() });
        Ok::<_, anyhow::Error>((adjEnh.clone(), adjEnhT.clone(), eqArray.clone(), eqn_lst.clone(), eqns.clone(), innerEquations.clone(), iterationVars.clone(), nE.clone(), nV.clone(), ocomp.clone(), qidx.clone(), residualequations.clone(), size.clone(), subsyst.clone(), unsolvedCSEVars.clone(), unsolvedCombined.clone(), unsolvedDiscreteVars.clone(), varArray.clone(), var_lst.clone(), vars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10, __try0_o11, __try0_o12, __try0_o13, __try0_o14, __try0_o15, __try0_o16, __try0_o17, __try0_o18, __try0_o19)) => {
            adjEnh = __try0_o0;
            adjEnhT = __try0_o1;
            eqArray = __try0_o2;
            eqn_lst = __try0_o3;
            eqns = __try0_o4;
            innerEquations = __try0_o5;
            iterationVars = __try0_o6;
            nE = __try0_o7;
            nV = __try0_o8;
            ocomp = __try0_o9;
            qidx = __try0_o10;
            residualequations = __try0_o11;
            size = __try0_o12;
            subsyst = __try0_o13;
            unsolvedCSEVars = __try0_o14;
            unsolvedCombined = __try0_o15;
            unsolvedDiscreteVars = __try0_o16;
            varArray = __try0_o17;
            var_lst = __try0_o18;
            vars = __try0_o19;
        }
        Err(_) => {
            Error::addInternalError((literal!("function minimalTearing failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok(ocomp)
}

fn matchDiscreteVars(mut inDiscreteVars: Arc<metamodelica::List<i32>>, mut adjEnhT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut varArray: metamodelica::Array<bool>, mut eqArray: metamodelica::Array<bool>, mut nE: metamodelica::Array<i32>, mut nV: metamodelica::Array<i32>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut nE: metamodelica::Array<i32> = nE;
    let mut nV: metamodelica::Array<i32> = nV;
    let mut eqMarker: metamodelica::Array<bool>;
    if '__try0: {
        for mut varIdx in &*inDiscreteVars.clone() {
            let mut varIdx = varIdx.clone();
            eqMarker = metamodelica::arrayFromVec(eqArray.clone().borrow().clone());
            let (__pa1, __pa2, __pa3, true) = (unwrap_break_err!(pathFound(varIdx.clone(), adjEnhT.clone(), varArray.clone(), eqArray.clone(), eqMarker.clone(), nE.clone(), nV.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            eqMarker = __pa1.clone();
            nE = __pa2.clone();
            nV = __pa3.clone();
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addInternalError((literal!("function matchDiscreteVars failed")).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok((nE, nV))
}

fn pathFound(mut varIdx: i32, mut adjEnhT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut varArray: metamodelica::Array<bool>, mut eqArray: metamodelica::Array<bool>, mut eqMarker: metamodelica::Array<bool>, mut nE: metamodelica::Array<i32>, mut nV: metamodelica::Array<i32>) -> Result<(metamodelica::Array<bool>, metamodelica::Array<i32>, metamodelica::Array<i32>, bool)> {
    let mut eqMarker: metamodelica::Array<bool> = eqMarker;
    let mut nE: metamodelica::Array<i32> = nE;
    let mut nV: metamodelica::Array<i32> = nV;
    let mut success: bool = false;
    let mut eqIdx: i32 = 0;
    if '__try0: {
        let __range1 = &*adjEnhT.borrow()[(varIdx.clone()-1) as usize].clone();
        for mut entry in __range1 {
            let mut entry = entry.clone();
            (eqIdx, _, _) = entry.clone();
            if unwrap_break_err!(isEntrySolvable(entry.clone()), '__try0) && eqIdx.clone() > 0 {
                if eqArray.borrow()[(eqIdx.clone()-1) as usize].clone() && nV.borrow()[(eqIdx.clone()-1) as usize].clone() == -1 {
                    {
                        let __cell2 = varIdx.clone();
                        nV.clone().borrow_mut()[(eqIdx.clone()-1) as usize] = __cell2;
                    }
                    {
                        let __cell3 = eqIdx.clone();
                        nE.clone().borrow_mut()[(varIdx.clone()-1) as usize] = __cell3;
                    }
                    success = true;
                    return Ok((eqMarker, nE, nV, success));
                }
            }
        }
        let __range4 = &*adjEnhT.borrow()[(varIdx.clone()-1) as usize].clone();
        for mut entry in __range4 {
            let mut entry = entry.clone();
            (eqIdx, _, _) = entry.clone();
            if unwrap_break_err!(isEntrySolvable(entry.clone()), '__try0) && eqIdx.clone() > 0 {
                if eqMarker.borrow()[(eqIdx.clone()-1) as usize].clone() {
                    {
                        let __cell5 = false;
                        eqMarker.clone().borrow_mut()[(eqIdx.clone()-1) as usize] = __cell5;
                    }
                    (eqMarker, nE, nV, success) = unwrap_break_err!(pathFound(nV.borrow()[(eqIdx.clone()-1) as usize].clone(), adjEnhT.clone(), varArray.clone(), eqArray.clone(), eqMarker.clone(), nE.clone(), nV.clone()), '__try0);
                }
            }
            if success.clone() {
                {
                    let __cell6 = varIdx.clone();
                    nV.clone().borrow_mut()[(eqIdx.clone()-1) as usize] = __cell6;
                }
                {
                    let __cell7 = eqIdx.clone();
                    nE.clone().borrow_mut()[(varIdx.clone()-1) as usize] = __cell7;
                }
                return Ok((eqMarker, nE, nV, success));
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addInternalError((literal!("function pathFound failed")).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok((eqMarker, nE, nV, success))
}

fn getTearingSetfromAssign(mut inDiscreteVars: Arc<metamodelica::List<i32>>, mut assign1: metamodelica::Array<i32>, mut varArray: metamodelica::Array<bool>, mut equationArray: metamodelica::Array<bool>) -> Result<(metamodelica::Array<bool>, metamodelica::Array<bool>, Arc<metamodelica::List<BackendDAE::InnerEquation>>)> {
    let mut varArray: metamodelica::Array<bool> = varArray;
    let mut equationArray: metamodelica::Array<bool> = equationArray;
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut eqIdx: i32 = 0;
    if '__try0: {
        for mut varIdx in &*inDiscreteVars.clone() {
            let mut varIdx = varIdx.clone();
            {let _arr = varArray.clone(); _arr.borrow_mut()[(varIdx.clone()-1) as usize] = false; _arr};
            eqIdx = assign1.borrow()[(varIdx.clone()-1) as usize].clone();
            {let _arr = equationArray.clone(); _arr.borrow_mut()[(eqIdx.clone()-1) as usize] = false; _arr};
            innerEquations = cons(BackendDAE::InnerEquation::INNEREQUATION { eqn: eqIdx.clone(), vars: list![varIdx.clone()] }, innerEquations.clone());
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addInternalError((literal!("function getTearingSetfromAssign failed")).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok((varArray, equationArray, innerEquations))
}

// =============================================================================
//
// Tearing from Book of Cellier
//
// =============================================================================
fn CellierTearing(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut eindex: Arc<metamodelica::List<i32>>, mut vindx: Arc<metamodelica::List<i32>>, mut tearingSelect_always: Arc<metamodelica::List<i32>>, mut ojac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>, mut jacType: BackendDAE::JacobianType, mut mixedSystem: bool, mut strongComponentIndex: i32) -> Result<(Arc<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: Arc<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool = false;
    let mut size: i32 = 0;
    let mut tornsize: i32 = 0;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut eqnNonlinPoints: metamodelica::Array<i32>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut OutTVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut residual: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut residual_coll: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut discreteVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tSel_always: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tSel_alwaysByUser: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tSel_prefer: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tSel_avoid: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tSel_never: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut subsyst: Arc<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut DAEtype: BackendDAE::BackendDAEType = BackendDAE::BackendDAEType::ALGEQSYSTEM;
    let mut DAEtypeStr: ArcStr = arcstr::literal!("");
    let mut strictTearingSet: BackendDAE::TearingSet;
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
    let mut casualTearingSet: Option<BackendDAE::TearingSet> = None;
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut linear: bool = false;
    let mut b: bool = false;
    let mut noDynamicStateSelection: bool = false;
    let mut dynamicTearing: bool = false;
    let mut s: ArcStr = arcstr::literal!("");
    let mut modelName: ArcStr = arcstr::literal!("");
    let debug: bool = false;
    linear = BackendDAEUtil::getLinearfromJacType(jacType.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { stateSets: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stateSets = __pa0.clone();
    noDynamicStateSelection = stateSets.clone().is_empty();
    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(ishared.clone()) {
        Deref @ BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: __pa1, .. }, backendDAEType: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    modelName = __pa1.clone();
    DAEtype = __pa2.clone();
    DAEtypeStr = (BackendDump::printBackendDAEType2String(DAEtype.clone())?).clone();
    dynamicTearing = (::match_deref::match_deref! { match &((Config::dynamicTearing()?, linear.clone(), noDynamicStateSelection.clone(), DAEtypeStr.clone(), Flags::getConfigBool(Flags::DYNAMIC_TEARING_FOR_INITIALIZATION.clone())?, Config::simCodeTarget()?)) {
        (Deref @ "true", _, true, Deref @ "simulation", _, Deref @ "C") => true,
        (Deref @ "true", _, true, Deref @ "initialization", true, Deref @ "C") => true,
        (Deref @ "linear", true, true, Deref @ "simulation", _, Deref @ "C") => true,
        (Deref @ "linear", true, true, Deref @ "initialization", true, Deref @ "C") => true,
        (Deref @ "nonlinear", false, true, Deref @ "simulation", _, Deref @ "C") => true,
        (Deref @ "nonlinear", false, true, Deref @ "initialization", true, Deref @ "C") => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of CellierTearing\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    size = (vindx.clone().len() as i32);
    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()));
    eqns = BackendEquation::listEquation(eqn_lst.clone())?;
    var_lst = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), BackendVariable::daeVars(isyst.clone()));
    vars = BackendVariable::listVar1(var_lst.clone());
    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (subsyst, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(subsyst.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 1")).clone())?;
    }
    m = Array::map(m.clone(), (std::sync::Arc::new(fnptr!(deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>));
    mt = Array::map(mt.clone(), (std::sync::Arc::new(fnptr!(deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>));
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("\n\n###BEGIN print Strong Component#####################\n(Function:CellierTearing)\n")).clone());
        BackendDump::printEqSystem(subsyst.clone())?;
        println!("{}", (literal!("\n###END print Strong Component#######################\n(Function:CellierTearing)\n\n\n")).clone());
    }
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nDetermine STRICT TEARING SET\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(subsyst.clone(), ishared.clone(), false)?;
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 1.5")).clone())?;
    }
    unsolvables = getUnsolvableVars(size.clone(), meT.clone())?;
    eqnNonlinPoints = arrayCreate(size.clone(), -1);
    getEquationNonlinearityPoints(eqnNonlinPoints.clone(), me.clone(), size.clone());
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 2")).clone())?;
    }
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("\nAdjacencyMatrixEnhanced:\n")).clone());
        BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
        println!("{}", (literal!("\nAdjacencyMatrixTransposedEnhanced:\n")).clone());
        BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\neqLinPoints:\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(eqnNonlinPoints.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("mapEqnIncRow:")).clone());
        BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(mapIncRowEqn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nUNSOLVABLES:\n")); __mm_s.push_str(&*stringDelimitList(List::map(unsolvables.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    discreteVars = findDiscrete(var_lst.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDiscrete Vars:\n")); __mm_s.push_str(&*stringDelimitList(List::map(discreteVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (tSel_always, tSel_prefer, tSel_avoid, tSel_never, tSel_alwaysByUser) = tearingSelect(var_lst.clone(), tearingSelect_always.clone(), (DAEtypeStr.clone()).clone())?;
    if !(tSel_alwaysByUser.clone().is_empty()) {
        Error::addMessage(Error::USER_TEARING_VARS.clone(), list![(intString(strongComponentIndex.clone())).clone(), (BackendDump::printBackendDAEType2String(DAEtype.clone())?).clone(), (BackendDump::dumpMarkedVarList(var_lst.clone(), tSel_alwaysByUser.clone())?).clone()])?;
    }
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 3")).clone())?;
    }
    ass1 = arrayCreate(size.clone(), -1);
    ass2 = arrayCreate(size.clone(), -1);
    order = metamodelica::nil();
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 3.1")).clone())?;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of CellierTearing2\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (OutTVars, order) = CellierTearing2(false, m.clone(), mt.clone(), me.clone(), meT.clone(), ass1.clone(), ass2.clone(), unsolvables.clone(), metamodelica::nil(), discreteVars.clone(), tSel_always.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), order.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of CellierTearing2\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    tornsize = (OutTVars.clone().len() as i32);
    b = intLt(tornsize.clone(), size.clone());
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 3.2")).clone())?;
    }
    residual = getUnassigned(ass2.clone());
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 3.3")).clone())?;
    }
    residual_coll = List::map1r(residual.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 3.4")).clone())?;
    }
    residual_coll = List::unique(residual_coll.clone());
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 3.5")).clone())?;
    }
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        dumpTearingSetLocalIndexes(OutTVars.clone(), residual_coll.clone(), order.clone(), ass2.clone(), size.clone(), mapEqnIncRow.clone(), vars.clone(), eqns.clone(), (literal!(" - STRICT SET")).clone())?;
    }
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 4")).clone())?;
    }
    OutTVars = selectFromList_rev(vindx.clone(), OutTVars.clone());
    residual = selectFromList_rev(eindex.clone(), residual_coll.clone());
    innerEquations = assignInnerEquations(order.clone(), eindex.clone(), vindx.clone(), ass2.clone(), mapEqnIncRow.clone(), None)?;
    if debug.clone() {
        execStat((literal!("Tearing.CellierTearing -> 5")).clone())?;
    }
    strictTearingSet = BackendDAE::TearingSet { tearingvars: OutTVars.clone(), residualequations: residual.clone(), innerEquations: innerEquations.clone(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) };
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        dumpTearingSetGlobalIndexes(strictTearingSet.clone(), size.clone(), (literal!(" - STRICT SET")).clone())?;
    }
    if dynamicTearing.clone() {
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nDetermine CASUAL TEARING SET\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        (_, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(subsyst.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
        m = Array::map(m.clone(), (std::sync::Arc::new(fnptr!(deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>));
        mt = Array::map(mt.clone(), (std::sync::Arc::new(fnptr!(deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>));
        (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(subsyst.clone(), ishared.clone(), true)?;
        unsolvables = getUnsolvableVars(size.clone(), meT.clone())?;
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("\nAdjacencyMatrixEnhanced:\n")).clone());
            BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
            println!("{}", (literal!("\nAdjacencyMatrixTransposedEnhanced:\n")).clone());
            BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\neqLinPoints:\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(eqnNonlinPoints.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("mapEqnIncRow:")).clone());
            BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(mapIncRowEqn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nUNSOLVABLES:\n")); __mm_s.push_str(&*stringDelimitList(List::map(unsolvables.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDiscrete Vars:\n")); __mm_s.push_str(&*stringDelimitList(List::map(discreteVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        ass1 = arrayCreate(size.clone(), -1);
        ass2 = arrayCreate(size.clone(), -1);
        order = metamodelica::nil();
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of CellierTearing2\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        (OutTVars, order) = CellierTearing2(false, m.clone(), mt.clone(), me.clone(), meT.clone(), ass1.clone(), ass2.clone(), unsolvables.clone(), metamodelica::nil(), discreteVars.clone(), tSel_always.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), order.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone())?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of CellierTearing2\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        if intLt((OutTVars.clone().len() as i32), tornsize.clone()) {
            residual = getUnassigned(ass2.clone());
            residual_coll = List::map1r(residual.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
            residual_coll = List::unique(residual_coll.clone());
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                dumpTearingSetLocalIndexes(OutTVars.clone(), residual_coll.clone(), order.clone(), ass2.clone(), size.clone(), mapEqnIncRow.clone(), vars.clone(), eqns.clone(), (literal!(" - CASUAL SET")).clone())?;
            }
            OutTVars = selectFromList_rev(vindx.clone(), OutTVars.clone());
            residual = selectFromList_rev(eindex.clone(), residual_coll.clone());
            innerEquations = assignInnerEquations(order.clone(), eindex.clone(), vindx.clone(), ass2.clone(), mapEqnIncRow.clone(), Some(me.clone()))?;
            casualTearingSet = Some(BackendDAE::TearingSet { tearingvars: OutTVars.clone(), residualequations: residual.clone(), innerEquations: innerEquations.clone(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) });
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                dumpTearingSetGlobalIndexes(BackendDAE::TearingSet { tearingvars: OutTVars.clone(), residualequations: residual.clone(), innerEquations: innerEquations.clone(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) }, size.clone(), (literal!(" - CASUAL SET")).clone())?;
            }
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                if linear.clone() {
                    s = (literal!("Linear")).clone();
                } else {
                    s = (literal!("Nonlinear")).clone();
                }
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nNote:\n=====\n")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" dynamic tearing for this strong component in model:\n")); __mm_s.push_str(&*modelName.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
        } else {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n* TEARING RESULTS (CASUAL SET):\n*\n* No of equations in strong component: ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* No of tVars: ")); __mm_s.push_str(&*intString((OutTVars.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*\n* tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(OutTVars.clone().reverse(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*\n* The casual tearing set is not smaller\n* than the strict tearing set and there-\n* fore it is discarded.\n*")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            if !(b.clone()) && !(Flags::getConfigBool(Flags::FORCE_TEARING.clone())?) {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    println!("{}", (literal!("\nNote:\n=====\nTearing set is discarded because it is not smaller than the original set. Use +forceTearing to prevent this.\n\n")).clone());
                }
                bail!("fail");
            }
            casualTearingSet = None;
        }
        if debug.clone() {
            execStat((literal!("Tearing.CellierTearing -> 6")).clone())?;
        }
    } else {
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("Note:\n=====\nNo dynamic Tearing for this strong component. Check if\n- flag 'dynamicTearing' is set proper\n- strong component does not contain statesets\n- system belongs to simulation\n- SimCode target is 'C'\n\n")).clone());
        }
        if !(b.clone()) && !(Flags::getConfigBool(Flags::FORCE_TEARING.clone())?) {
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\nNote:\n=====\nTearing set is discarded because it is not smaller than the original set. Use +forceTearing to prevent this.\n\n")).clone());
            }
            bail!("fail");
        }
        casualTearingSet = None;
        if debug.clone() {
            execStat((literal!("Tearing.CellierTearing -> 7")).clone())?;
        }
    }
    ocomp = Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: strictTearingSet.clone(), casualTearingSet: casualTearingSet.clone(), linear: linear.clone(), mixedSystem: mixedSystem.clone() });
    outRunMatching = true;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of CellierTearing\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((ocomp, outRunMatching))
}

fn tearingSelect(mut var_lstIn: Arc<metamodelica::List<BackendDAE::Var>>, mut always: Arc<metamodelica::List<i32>>, mut DAEtypeStr: ArcStr) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut always: Arc<metamodelica::List<i32>> = always;
    let mut prefer: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut avoid: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut never: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut alwaysByUser: Arc<metamodelica::List<i32>> = always.clone();
    let mut var: BackendDAE::Var;
    let mut index: i32 = 1;
    let mut ts: Option<BackendDAE::TearingSelect> = None;
    let mut preferTVarsWithStartValue: bool = false;
    let mut inSimulation: bool = DAEtypeStr.clone() == literal!("simulation");
    let mut decided: bool = false;
    preferTVarsWithStartValue = Flags::getConfigBool(Flags::PREFER_TVARS_WITH_START_VALUE.clone())? && DAEtypeStr.clone() == literal!("initialization");
    for mut var in &*var_lstIn.clone() {
        let mut var = var.clone();
        let BackendDAE::VAR { tearingSelectOption: __pa0, .. } = (var.clone()) else { bail!("pattern mismatch") };
        ts = __pa0.clone();
        decided = (match ts.clone() {
        None => false,
        Some(BackendDAE::TearingSelect::ALWAYS) => {
            if !(listMember(index.clone(), always.clone())) {
                always = cons(index.clone(), always.clone());
                alwaysByUser = cons(index.clone(), alwaysByUser.clone());
            }
            true
        },
        Some(BackendDAE::TearingSelect::PREFER) => {
            prefer = cons(index.clone(), prefer.clone());
            true
        },
        Some(BackendDAE::TearingSelect::DEFAULT) => true,
        Some(BackendDAE::TearingSelect::AVOID) => {
            avoid = cons(index.clone(), avoid.clone());
            true
        },
        Some(BackendDAE::TearingSelect::NEVER) => {
            never = cons(index.clone(), never.clone());
            true
        },
        _ => bail!("match: no arm matched"),
    });
        if !(decided.clone()) {
            if Flags::getConfigBool(Flags::TEARING_ALWAYS_DERIVATIVES.clone())? && inSimulation.clone() && BackendVariable::isStateVar(var.clone()) && !(listMember(index.clone(), always.clone())) {
                always = cons(index.clone(), always.clone());
            } else if preferTVarsWithStartValue.clone() && BackendVariable::varHasStartValue(var.clone())? {
                prefer = cons(index.clone(), prefer.clone());
            }
        }
        index = index.clone() + 1;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExternal influence on selection of iteration variables by variable annotations (__OpenModelica_tearingSelect)")); __mm_s.push_str(&*if (preferTVarsWithStartValue.clone()) {literal!(" and preference of variables with start attribute")} else {literal!("")}); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Always: ")); __mm_s.push_str(&*stringDelimitList(List::map(always.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Prefer: ")); __mm_s.push_str(&*stringDelimitList(List::map(prefer.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Avoid: ")); __mm_s.push_str(&*stringDelimitList(List::map(avoid.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Never: ")); __mm_s.push_str(&*stringDelimitList(List::map(never.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((always, prefer, avoid, never, alwaysByUser))
}

pub fn deleteNegativeEntries(mut rowIn: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut rowOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    rowOut = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut r in (rowIn.clone()).into_iter().cloned() {
            if !(r.clone() > 0) { continue; }
            let __x = r.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    rowOut
}

fn findDiscrete(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Arc<metamodelica::List<i32>> {
    let mut discreteVarsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut var in &*inVars.clone() {
        let mut var = var.clone();
        if BackendVariable::isVarDiscrete(var.clone()) {
            discreteVarsOut = cons(index.clone(), discreteVarsOut.clone());
        }
        index = index.clone() + 1;
    }
    discreteVarsOut
}

fn findDiscreteWarnTearingSelect(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut discreteVarsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut var in &*inVars.clone() {
        let mut var = var.clone();
        if BackendVariable::isVarDiscrete(var.clone()) {
            discreteVarsOut = cons(index.clone(), discreteVarsOut.clone());
            let _ = (match var.tearingSelectOption.clone() {
        Some(BackendDAE::TearingSelect::ALWAYS) => {
            Error::addSourceMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Minimal Tearing is ignoring '__OpenModelica_tearingSelect = TearingSelect.always' annotation for discrete variable: ")); __mm_s.push_str(&*BackendDump::varString(var.clone())?); ArcStr::from(__mm_s) }).clone()], ElementSource::getInfo(var.source.clone()))?;
            ()
        },
        Some(BackendDAE::TearingSelect::PREFER) => {
            Error::addSourceMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Minimal Tearing is ignoring '__OpenModelica_tearingSelect = TearingSelect.prefer' annotation for discrete variable: ")); __mm_s.push_str(&*BackendDump::varString(var.clone())?); ArcStr::from(__mm_s) }).clone()], ElementSource::getInfo(var.source.clone()))?;
            ()
        },
        _ => (),
    });
        }
        index = index.clone() + 1;
    }
    Ok(discreteVarsOut)
}

fn findCSE(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Arc<metamodelica::List<i32>> {
    let mut cseVarsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut var in &*inVars.clone() {
        let mut var = var.clone();
        if BackendVariable::isCSEVar(var.clone()) {
            cseVarsOut = cons(index.clone(), cseVarsOut.clone());
        }
        index = index.clone() + 1;
    }
    cseVarsOut
}

fn getEquationNonlinearityPoints(mut eqnNonlinPoints: metamodelica::Array<i32>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut size: i32) -> metamodelica::Array<i32> {
    let mut eqnNonlinPoints: metamodelica::Array<i32> = eqnNonlinPoints;
    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut sum: i32 = 0;
    for mut i in 1..=size.clone() {
        row = me.borrow()[(i.clone()-1) as usize].clone();
        sum = 0;
        for mut entry in &*row.clone() {
            let mut entry = entry.clone();
            sum = sum.clone() + nonlinearityWeight(entry.clone());
        }
        {
            let __cell0 = sum.clone();
            eqnNonlinPoints.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
        }
    }
    eqnNonlinPoints
}

fn nonlinearityWeight(mut entry: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> i32 {
    let mut weight: i32 = 0;
    weight = (::match_deref::match_deref! { match &(entry.clone()) {
        (_, BackendDAE::Solvability::SOLVABILITY_SOLVED, _) => 0,
        (_, BackendDAE::Solvability::SOLVABILITY_CONSTONE, _) => 2,
        (_, BackendDAE::Solvability::SOLVABILITY_CONST { .. }, _) => 5,
        (_, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true }, _) => 10,
        (_, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false }, _) => 20,
        (_, BackendDAE::Solvability::SOLVABILITY_LINEAR { b: true }, _) => 20,
        (_, BackendDAE::Solvability::SOLVABILITY_LINEAR { b: false }, _) => 50,
        (_, BackendDAE::Solvability::SOLVABILITY_NONLINEAR, _) => 50,
        (_, BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE, _) => 100,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    weight
}

fn CellierTearing2(mut inCausal: bool, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meTIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut Unsolvables: Arc<metamodelica::List<i32>>, mut tvarsIn: Arc<metamodelica::List<i32>>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_always: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut orderIn: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut eqnNonlinPoints: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut OutTVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut orderOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let debug: bool = false;
    if inCausal.clone() {
        OutTVars = tvarsIn.clone();
        orderOut = orderIn.clone();
        if debug.clone() {
            execStat((literal!("Tearing.CellierTearing2 - done")).clone())?;
        }
        return Ok((OutTVars, orderOut));
    }
    (OutTVars, orderOut) = (::match_deref::match_deref! { match &((Unsolvables.clone(), tSel_always.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            let mut tvar: i32 = 0;
            let mut tvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut unsolvables: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut causal: bool = false;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of selectTearingVar\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            tvar = selectTearingVar(meIn.clone(), meTIn.clone(), mIn.clone(), mtIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
            if debug.clone() {
                execStat((literal!("Tearing.CellierTearing2 - 1.0")).clone())?;
            }
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of selectTearingVar\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            {let _arr = ass1In.clone(); let _val = (ass1In.clone().borrow().len() as i32) * 2; _arr.borrow_mut()[(tvar.clone()-1) as usize] = _val; _arr};
            deleteEntriesFromAdjacencyMatrix(mIn.clone(), mtIn.clone(), list![tvar.clone()])?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\n\n###BEGIN print Adjacency Matrix w/o tvar############\n(Function: CellierTearing2)\n")).clone());
                BackendDump::dumpAdjacencyMatrix(mIn.clone());
            }
            let _ = Array::replaceAtWithFill(tvar.clone(), metamodelica::nil(), metamodelica::nil(), mtIn.clone())?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                BackendDump::dumpAdjacencyMatrixT(mtIn.clone());
                println!("{}", (literal!("\n###END print Adjacency Matrix w/o tvar##############\n(Function: CellierTearing2)\n\n\n")).clone());
            }
            if debug.clone() {
                execStat((literal!("Tearing.CellierTearing2 - 1.1")).clone())?;
            }
            tvars = cons(tvar.clone(), tvarsIn.clone());
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of TarjanMatching\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            (order, causal) = TarjanMatching(mIn.clone(), mtIn.clone(), meIn.clone(), ass1In.clone(), ass2In.clone(), orderIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone())?;
            if debug.clone() {
                execStat((literal!("Tearing.CellierTearing2 - 1.2")).clone())?;
            }
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of TarjanMatching\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n* TARJAN RESULTS:\n* ass1: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass1In.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* ass2: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass2In.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* order: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            if causal.clone() && (Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?) {
                println!("{}", (literal!("\n")).clone());
                BackendDump::dumpMatching(ass1In.clone())?;
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\norder: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            unsolvables = getUnsolvableVarsConsiderMatching((meTIn.clone().borrow().len() as i32), meTIn.clone(), ass1In.clone(), ass2In.clone())?;
            if debug.clone() {
                execStat((literal!("Tearing.CellierTearing2 - 1.3")).clone())?;
            }
            (_, unsolvables, _) = List::intersection1OnTrue(unsolvables.clone(), tvars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            if debug.clone() {
                execStat((literal!("Tearing.CellierTearing2 - 1 done")).clone())?;
            }
            (tvars, order) = CellierTearing2(causal.clone(), mIn.clone(), mtIn.clone(), meIn.clone(), meTIn.clone(), ass1In.clone(), ass2In.clone(), unsolvables.clone(), tvars.clone(), discreteVars.clone(), tSel_always.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), order.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone())?;
            (tvars.clone(), order.clone())
        },
        _ => {
            let mut tvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut unsolvables: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tVar_never: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tVar_discrete: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut causal: bool = false;
            tvars = List::unique(listAppend(Unsolvables.clone(), tSel_always.clone()));
            tVar_never = List::intersectionOnTrue(tSel_never.clone(), tvars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
            tVar_discrete = List::intersectionOnTrue(discreteVars.clone(), tvars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
            if !(tVar_never.clone().is_empty()) {
                Error::addCompilerWarning((literal!("There are tearing variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.never'. Use -d=tearingdump and -d=tearingdumpV for more information.")).clone())?;
            }
            if !(tVar_discrete.clone().is_empty()) {
                Error::addCompilerWarning((literal!("There are discrete tearing variables because otherwise the system could not have been torn (unsolvables). This may lead to problems during simulation.")).clone())?;
            }
            if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nForced selection of Tearing Variables:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\nUnsolvables as tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(Unsolvables.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variables with annotation attribute 'always' as tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(tSel_always.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            markTVarsOrResiduals(tvars.clone(), ass1In.clone())?;
            deleteEntriesFromAdjacencyMatrix(mIn.clone(), mtIn.clone(), tvars.clone())?;
            deleteRowsFromAdjacencyMatrix(mtIn.clone(), tvars.clone())?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\n\n###BEGIN print Adjacency Matrix w/o tvars###########\n(Function: CellierTearing2)\n")).clone());
                BackendDump::dumpAdjacencyMatrix(mIn.clone());
                BackendDump::dumpAdjacencyMatrixT(mtIn.clone());
                println!("{}", (literal!("\n###END print Adjacency Matrix w/o tvars#############\n(Function: CellierTearing2)\n\n\n")).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of TarjanMatching\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            tvars = listAppend(tvars.clone(), tvarsIn.clone());
            (order, causal) = TarjanMatching(mIn.clone(), mtIn.clone(), meIn.clone(), ass1In.clone(), ass2In.clone(), orderIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone())?;
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of TarjanMatching\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n* TARJAN RESULTS:\n* ass1: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass1In.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* ass2: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass2In.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* order: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            if causal.clone() && (Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())?) {
                println!("{}", (literal!("\n")).clone());
                BackendDump::dumpMatching(ass1In.clone())?;
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\norder: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            unsolvables = getUnsolvableVarsConsiderMatching((meTIn.clone().borrow().len() as i32), meTIn.clone(), ass1In.clone(), ass2In.clone())?;
            (_, unsolvables, _) = List::intersection1OnTrue(unsolvables.clone(), tvars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            if debug.clone() {
                execStat((literal!("Tearing.CellierTearing2 - 2")).clone())?;
            }
            (tvars, order) = CellierTearing2(causal.clone(), mIn.clone(), mtIn.clone(), meIn.clone(), meTIn.clone(), ass1In.clone(), ass2In.clone(), unsolvables.clone(), tvars.clone(), discreteVars.clone(), metamodelica::nil(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), order.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone())?;
            (tvars.clone(), order.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((OutTVars, orderOut))
}

fn selectTearingVar(mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<i32> {
    let mut OutTVar: i32 = 0;
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut heuristic: ArcStr = arcstr::literal!("");
    let mut tearingHeuristic: TearingHeuristic;
    heuristic = (Config::getTearingHeuristic()?).clone();
    tearingHeuristic = (::match_deref::match_deref! { match &(heuristic.clone()) {
        Deref @ "MC1" => (std::sync::Arc::new(ModifiedCellierHeuristic_1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC2" => (std::sync::Arc::new(ModifiedCellierHeuristic_2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC11" => (std::sync::Arc::new(ModifiedCellierHeuristic_1_1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC21" => (std::sync::Arc::new(ModifiedCellierHeuristic_2_1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC12" => (std::sync::Arc::new(ModifiedCellierHeuristic_1_2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC22" => (std::sync::Arc::new(ModifiedCellierHeuristic_2_2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC13" => (std::sync::Arc::new(ModifiedCellierHeuristic_1_3) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC23" => (std::sync::Arc::new(ModifiedCellierHeuristic_2_3) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC231" => (std::sync::Arc::new(ModifiedCellierHeuristic_2_3_1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC3" => (std::sync::Arc::new(ModifiedCellierHeuristic_3) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        Deref @ "MC4" => (std::sync::Arc::new(ModifiedCellierHeuristic_4) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown tearing heuristic: ")); __mm_s.push_str(&*heuristic.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of TearingHeuristic\n\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Chosen Heuristic: ")); __mm_s.push_str(&*heuristic.clone()); __mm_s.push_str(&*literal!("\n\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    match '__try0: {
        potentials = unwrap_break_err!(tearingHeuristic(m.clone(), mt.clone(), me.clone(), meT.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(potentials.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        OutTVar = __pa1.clone();
        Ok::<_, anyhow::Error>((OutTVar.clone(), potentials.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            OutTVar = __try0_o0;
            potentials = __try0_o1;
        }
        Err(_) => {
            println!("{}", (literal!("\nThe selection of a new tearing variable failed.\n")).clone());
            Error::addCompilerWarning((literal!("Function Tearing.selectTearingVar failed at least once. Use -d=tearingdump or -d=tearingdumpV for more information.")).clone())?;
            bail!("fail");
        }
    }
    if listMember(OutTVar.clone(), tSel_avoid.clone()) {
        Error::addCompilerWarning((literal!("The Tearing heuristic has chosen variables with annotation attribute '__OpenModelica_tearingSelect = TearingSelect.avoid'. Use -d=tearingdump and -d=tearingdumpV for more information.")).clone())?;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of TearingHeuristic\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(OutTVar)
}

type TearingHeuristic = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>;

fn ModifiedCellierHeuristic_1(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    selectedcols1 = getUnassigned(ass1In.clone());
    selectedcols1 = getVarsOfEqnsWithMostVars(selectedcols1.clone(), mIn.clone(), mtIn.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (_, selectedcols1, _) = List::intersection1OnTrue(selectedcols1.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Without Discrete: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables in the equation(s) with most Variables)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (1st) with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, edges) = selectOneMostCausalizingVar(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (2nd) causalizing most equations [")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut varlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols1, _) = List::intersection1OnTrue(varlst.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Non-discrete variables with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, edges) = selectOneMostCausalizingVar(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (1st) causalizing most equations [")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_1_1(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    selectedcols1 = getUnassigned(ass1In.clone());
    selectedcols1 = getVarsOfEqnsWithMostVars(selectedcols1.clone(), mIn.clone(), mtIn.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (_, selectedcols1, _) = List::intersection1OnTrue(selectedcols1.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Without Discrete: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables in the equation(s) with most Variables)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (1st) with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, _) = selectMostCausalizingVars(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (2nd) causalizing most equations)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, edges) = getOneVarWithMostImpAss(potentials.clone(), ass2In.clone(), metIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n4th: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from from (3rd) with most incident impossible assignments [")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2_1(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut varlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols1, _) = List::intersection1OnTrue(varlst.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Non-discrete variables with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, _) = selectMostCausalizingVars(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (1st) causalizing most equations)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, edges) = getOneVarWithMostImpAss(potentials.clone(), ass2In.clone(), metIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (2nd) with most incident impossible assignments [")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_1_2(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    selectedcols1 = getUnassigned(ass1In.clone());
    selectedcols1 = getVarsOfEqnsWithMostVars(selectedcols1.clone(), mIn.clone(), mtIn.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (_, selectedcols1, _) = List::intersection1OnTrue(selectedcols1.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Without Discrete: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables in the equation(s) with most Variables)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (1st) with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (selectedcols1, _, _) = getAllVarsWithMostImpAss(selectedcols1.clone(), ass2In.clone(), metIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (2nd) with most incident impossible assignments)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, edges) = selectOneMostCausalizingVar(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n4th: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable.One from (3rd) causalizing most equations [")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2_2(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut varlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols1, _) = List::intersection1OnTrue(varlst.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Non-discrete variables with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (selectedcols1, _, _) = getAllVarsWithMostImpAss(selectedcols1.clone(), ass2In.clone(), metIn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (1st) with most incident impossible assignments)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, edges) = selectOneMostCausalizingVar(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (2nd) causalizing most equations [")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_1_3(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut maxPoints: i32 = 0;
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut points: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    selectedcols1 = getUnassigned(ass1In.clone());
    selectedcols1 = getVarsOfEqnsWithMostVars(selectedcols1.clone(), mIn.clone(), mtIn.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (_, selectedcols1, _) = List::intersection1OnTrue(selectedcols1.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Without Discrete: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables in the equation(s) with most Variables)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (1st) with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (_, counts1) = selectMostCausalizingVars(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    counts1 = counts1.clone().reverse();
    (_, counts2, _) = getAllVarsWithMostImpAss(selectedcols1.clone(), ass2In.clone(), metIn.clone())?;
    points = List::threadMap(counts1.clone(), counts2.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>));
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPoints: ")); __mm_s.push_str(&*stringDelimitList(List::map(points.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Sum of impossible assignments and causalizable equations)\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, maxPoints) = getOneVarWithMostPoints(selectedcols1.clone(), points.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (2nd) with most points [")); __mm_s.push_str(&*intString(maxPoints.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2_3(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut maxPoints: i32 = 0;
    let mut varlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut points: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols1, _) = List::intersection1OnTrue(varlst.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols1.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Non-discrete variables with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (_, counts1) = selectMostCausalizingVars(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    counts1 = counts1.clone().reverse();
    (_, counts2, _) = getAllVarsWithMostImpAss(selectedcols1.clone(), ass2In.clone(), metIn.clone())?;
    points = List::threadMap(counts1.clone(), counts2.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>));
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPoints: ")); __mm_s.push_str(&*stringDelimitList(List::map(points.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Sum of impossible assignments and causalizable equations)\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials, maxPoints) = getOneVarWithMostPoints(selectedcols1.clone(), points.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (1st) with most points [")); __mm_s.push_str(&*intString(maxPoints.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_2_3_1(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut potpoints1: i32 = 0;
    let mut potpoints2: i32 = 0;
    let mut varlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedcols0: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedcols1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut points1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut points2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("Start round 1:\n==============\n\n")).clone());
    }
    varlst = getUnassigned(ass1In.clone());
    (_, selectedcols0, _) = List::intersection1OnTrue(varlst.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    (edges, selectedcols1) = getVarsOccurringInMostEquations(mtIn.clone(), selectedcols0.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Non-discrete variables with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedrows = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (_, counts1) = selectMostCausalizingVars(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    counts1 = counts1.clone().reverse();
    (_, counts2, _) = getAllVarsWithMostImpAss(selectedcols1.clone(), ass2In.clone(), metIn.clone())?;
    points1 = List::threadMap(counts1.clone(), counts2.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>));
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPoints: ")); __mm_s.push_str(&*stringDelimitList(List::map(points1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Sum of impossible assignments and causalizable equations)\n")); ArcStr::from(__mm_s) }).clone());
    }
    (potentials1, potpoints1) = getOneVarWithMostPoints(selectedcols1.clone(), points1.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (1st) with most points (")); __mm_s.push_str(&*intString(potpoints1.clone())); __mm_s.push_str(&*literal!(" points))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedcols1 = findNEntries(mtIn.clone(), selectedcols0.clone(), edges.clone() - 1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart round 2:\n==============\n\n1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedcols1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables with occurrence in ")); __mm_s.push_str(&*intString(edges.clone() - 1)); __mm_s.push_str(&*literal!(" equations)\n\n")); __mm_s.push_str(&*stringDelimitList(List::map(selectedrows.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more Var)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    if selectedcols1.clone().is_empty() {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("Second set is empty.")).clone());
        }
        potentials = potentials1.clone();
        potpoints2 = 0;
    } else {
        (_, counts1) = selectMostCausalizingVars(mtIn.clone(), selectedcols1.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(selectedrows.clone(), (ass1In.clone().borrow().len() as i32))?)?;
        counts1 = counts1.clone().reverse();
        (_, counts2, _) = getAllVarsWithMostImpAss(selectedcols1.clone(), ass2In.clone(), metIn.clone())?;
        points2 = List::threadMap(counts1.clone(), counts2.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>));
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPoints: ")); __mm_s.push_str(&*stringDelimitList(List::map(points2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Sum of impossible assignments and causalizable equations)\n")); ArcStr::from(__mm_s) }).clone());
        }
        (potentials2, potpoints2) = getOneVarWithMostPoints(selectedcols1.clone(), points2.clone());
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (1st) with most points (")); __mm_s.push_str(&*intString(potpoints2.clone())); __mm_s.push_str(&*literal!(" points))\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        potentials = if (intGe(potpoints1.clone(), potpoints2.clone())) {potentials1.clone()} else {potentials2.clone()};
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n=====================\nChosen tearing variable: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n=====================\n(from round 1: ")); __mm_s.push_str(&*boolString(intGe(potpoints1.clone(), potpoints2.clone()))); __mm_s.push_str(&*literal!(")\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_3(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut maxPoints: i32 = 0;
    let mut potentialTVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentialTVars2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut bestPotentialTVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut causEq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut points: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varsWithPoints: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let debug: bool = false;
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC0")).clone())?;
    }
    causEq = traverseSingleEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(causEq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Equations which could be causalized by knowing one more variable)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC1")).clone())?;
    }
    potentialTVars = getUnassigned(ass1In.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentialTVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(All unassigned variables)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC2")).clone())?;
    }
    (_, potentialTVars, _) = List::intersection1OnTrue(potentialTVars.clone(), tSel_never.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if potentialTVars.clone().is_empty() {
        Error::addCompilerError((literal!("It is not possible to select a new tearing variable, because all remaining variables have the attribute '__OpenModelica_tearingSelect = TearingSelect.never'.")).clone())?;
        return Ok(potentials);
    }
    (_, potentialTVars2, _) = List::intersection1OnTrue(potentialTVars.clone(), discreteVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if potentialTVars2.clone().is_empty() {
        potentialTVars2 = potentialTVars.clone();
        Error::addCompilerWarning((literal!("The tearing heuristic was not able to avoid discrete iteration variables because otherwise the system could not have been torn. This may lead to problems during simulation.")).clone())?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentialTVars2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(All unassigned variables without attribute 'never' (only discrete variables left))\n\n")); ArcStr::from(__mm_s) }).clone());
        }
    } else {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentialTVars2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(All non-discrete variables from (2nd) without attribute 'never')\n\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC3")).clone())?;
    }
    (potentialTVars, counts1) = selectCausalizingVars(mtIn.clone(), potentialTVars2.clone(), meIn.clone(), ass1In.clone(), selectCausalVarsPrepareSelectionSet(causEq.clone(), (ass1In.clone().borrow().len() as i32))?)?;
    if potentialTVars.clone().is_empty() {
        potentialTVars = potentialTVars2.clone();
        counts1 = List::fill(0, (potentialTVars2.clone().len() as i32));
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC4_1")).clone())?;
    }
    (_, counts2, _) = getAllVarsWithMostImpAss(potentialTVars.clone(), ass2In.clone(), metIn.clone())?;
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC4_2")).clone())?;
    }
    points = List::threadMap(counts1.clone(), counts2.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>));
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n4th (Points): ")); __mm_s.push_str(&*stringDelimitList(List::map(points.clone().reverse(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Sum of impossible assignments and causalizable equations)\n")); ArcStr::from(__mm_s) }).clone());
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC4_3")).clone())?;
    }
    if !(tSel_prefer.clone().is_empty()) {
        points = preferAvoidVariables(potentialTVars.clone(), points.clone(), tSel_prefer.clone(), metamodelica::OrderedFloat(3.0_f64));
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    (Points): ")); __mm_s.push_str(&*stringDelimitList(List::map(points.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Points after preferring variables with attribute 'prefer')\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC4_4")).clone())?;
    }
    if !(tSel_avoid.clone().is_empty()) {
        points = preferAvoidVariables(potentialTVars.clone(), points.clone(), tSel_avoid.clone(), metamodelica::OrderedFloat(0.334_f64));
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    (Points): ")); __mm_s.push_str(&*stringDelimitList(List::map(points.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Points after discrimination against variables with attribute 'avoid')\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC4_5")).clone())?;
    }
    (bestPotentialTVars, maxPoints) = getAllVarsWithMostPoints(potentialTVars.clone(), points.clone(), metamodelica::nil(), -1)?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n5th: ")); __mm_s.push_str(&*stringDelimitList(List::map(bestPotentialTVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (3rd) with most points [")); __mm_s.push_str(&*intString(maxPoints.clone())); __mm_s.push_str(&*literal!("])\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC5")).clone())?;
    }
    (edges, potentials) = getVarOccurringInMostEquations(mtIn.clone(), bestPotentialTVars.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("6th: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from (5th) with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    if debug.clone() {
        execStat((literal!("TEARINGHEURISTIC6")).clone())?;
    }
    Ok(potentials)
}

fn ModifiedCellierHeuristic_4(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut metIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut discreteVars: Arc<metamodelica::List<i32>>, mut tSel_prefer: Arc<metamodelica::List<i32>>, mut tSel_avoid: Arc<metamodelica::List<i32>>, mut tSel_never: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut potentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut edges: i32 = 0;
    let mut potentials1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials4: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials5: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials6: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials7: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials8: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials9: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut potentials10: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut selectedvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut count: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Heuristic uses all modified Cellier-Heuristics\n\nHeuristic [MC1]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials1 = ModifiedCellierHeuristic_1(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC2]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials2 = ModifiedCellierHeuristic_2(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC11]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials3 = ModifiedCellierHeuristic_1_1(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC21]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials4 = ModifiedCellierHeuristic_2_1(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC12]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials5 = ModifiedCellierHeuristic_1_2(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC22]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials6 = ModifiedCellierHeuristic_2_2(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC13]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials7 = ModifiedCellierHeuristic_1_3(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC23]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials8 = ModifiedCellierHeuristic_2_3(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC231]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials9 = ModifiedCellierHeuristic_2_3_1(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHeuristic [MC3]\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    potentials10 = ModifiedCellierHeuristic_3(mIn.clone(), mtIn.clone(), meIn.clone(), metIn.clone(), ass1In.clone(), ass2In.clone(), discreteVars.clone(), tSel_prefer.clone(), tSel_avoid.clone(), tSel_never.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\nSynopsis:\n=========\n[MC1]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC2]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC11]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials3.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC21]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials4.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC12]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials5.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC22]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials6.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC13]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials7.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC23]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials8.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC231]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials9.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[MC3]: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials10.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    selectedvars = listAppend(potentials1.clone(), listAppend(potentials2.clone(), listAppend(potentials3.clone(), listAppend(potentials4.clone(), listAppend(potentials5.clone(), listAppend(potentials6.clone(), listAppend(potentials7.clone(), listAppend(potentials8.clone(), listAppend(potentials9.clone(), potentials10.clone())))))))));
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1st: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedvars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(All potentials)\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (count, selectedvars, _) = countMultiples(arrayCreate(1, selectedvars.clone()));
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("2nd: ")); __mm_s.push_str(&*stringDelimitList(List::map(selectedvars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Variables from (1st) occurring in most potential-sets (")); __mm_s.push_str(&*stringDelimitList(List::map(count.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!(" sets))\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    (edges, potentials) = getVarOccurringInMostEquations(mtIn.clone(), selectedvars.clone())?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("3rd: ")); __mm_s.push_str(&*stringDelimitList(List::map(potentials.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n(Chosen tearing variable. One from from (2nd) with most occurrence in equations (")); __mm_s.push_str(&*intString(edges.clone())); __mm_s.push_str(&*literal!(" times))\n\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(potentials)
}

fn preferAvoidVariables(mut varsIn: Arc<metamodelica::List<i32>>, mut points: Arc<metamodelica::List<i32>>, mut preferAvoidIn: Arc<metamodelica::List<i32>>, mut factor: metamodelica::Real) -> Arc<metamodelica::List<i32>> {
    let mut points: Arc<metamodelica::List<i32>> = points;
    let mut preferAvoidVar: i32 = 0;
    let mut pos: i32 = 0;
    for mut preferAvoidVar in &*preferAvoidIn.clone() {
        let mut preferAvoidVar = preferAvoidVar.clone();
        match '__try0: {
            pos = unwrap_break_err!(List::position(preferAvoidVar.clone(), varsIn.clone()), '__try0);
            points = unwrap_break_err!(List::set(points.clone(), pos.clone(), (((factor.clone()) * (intReal((points.clone()).get(pos.clone()).unwrap()))).0 as i32)), '__try0);
            Ok::<_, anyhow::Error>((points.clone(), pos.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                points = __try0_o0;
                pos = __try0_o1;
            }
            Err(_) => {
                panic!("try/else: outputs not set in else branch");
            }
        }
    }
    points
}

fn selectCausalVarsPrepareSelectionSet(mut selEqs: Arc<metamodelica::List<i32>>, mut ass1In_size: i32) -> Result<metamodelica::Array<bool>> {
    let mut selEqsSetArray: metamodelica::Array<bool>;
    selEqsSetArray = arrayCreate(ass1In_size.clone(), false);
    for mut e in &*selEqs.clone() {
        let mut e = e.clone();
        {let _arr = selEqsSetArray.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = true; _arr};
    }
    Ok(selEqsSetArray)
}

fn selectMostCausalizingVars(mut inMt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut selVars: Arc<metamodelica::List<i32>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut selEqsSetArray: metamodelica::Array<bool>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut cVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut num: i32 = 0;
    for mut var in &*selVars.clone() {
        let mut var = var.clone();
        row = inMt.clone().borrow()[(var.clone()-1) as usize].clone();
        {let _arr = ass1In.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = 1; _arr};
        size = 0;
        for mut i in &*row.clone() {
            let mut i = i.clone();
            if selEqsSetArray.clone().borrow()[(i.clone()-1) as usize].clone() {
                size = sizeOfAssignable(i.clone(), me.clone(), ass1In.clone(), size.clone());
            }
        }
        {let _arr = ass1In.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = -1; _arr};
        if size.clone() < num.clone() {
            counts = cons(size.clone(), counts.clone());
        } else if size.clone() == num.clone() {
            cVars = cons(var.clone(), cVars.clone());
            counts = cons(size.clone(), counts.clone());
        } else {
            cVars = list![var.clone()];
            num = size.clone();
            counts = cons(size.clone(), counts.clone());
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Var ")); __mm_s.push_str(&*intString(var.clone())); __mm_s.push_str(&*literal!(" would causalize ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!(" Eqns\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok((cVars, counts))
}

fn selectCausalizingVars(mut inMt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut selVars: Arc<metamodelica::List<i32>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut selEqsSetArray: metamodelica::Array<bool>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut cVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut num: i32 = 0;
    for mut var in &*selVars.clone() {
        let mut var = var.clone();
        row = inMt.clone().borrow()[(var.clone()-1) as usize].clone();
        {let _arr = ass1In.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = 1; _arr};
        size = 0;
        for mut i in &*row.clone() {
            let mut i = i.clone();
            if selEqsSetArray.clone().borrow()[(i.clone()-1) as usize].clone() {
                size = sizeOfAssignable(i.clone(), me.clone(), ass1In.clone(), size.clone());
            }
        }
        {let _arr = ass1In.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = -1; _arr};
        if !(size.clone() == 0) {
            cVars = cons(var.clone(), cVars.clone());
            counts = cons(size.clone(), counts.clone());
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Var ")); __mm_s.push_str(&*intString(var.clone())); __mm_s.push_str(&*literal!(" would causalize ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!(" Eqns\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok((cVars, counts))
}

fn selectOneMostCausalizingVar(mut inMt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut selVars: Arc<metamodelica::List<i32>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut selEqsSetArray: metamodelica::Array<bool>) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut cVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outMax: i32 = 0;
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut size: i32 = 0;
    for mut var in &*selVars.clone() {
        let mut var = var.clone();
        row = inMt.clone().borrow()[(var.clone()-1) as usize].clone();
        {let _arr = ass1In.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = 1; _arr};
        size = 0;
        for mut i in &*row.clone() {
            let mut i = i.clone();
            if selEqsSetArray.clone().borrow()[(i.clone()-1) as usize].clone() {
                size = sizeOfAssignable(i.clone(), me.clone(), ass1In.clone(), size.clone());
            }
        }
        {let _arr = ass1In.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = -1; _arr};
        if intGe(size.clone(), outMax.clone()) {
            cVars = list![var.clone()];
            outMax = size.clone();
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Var ")); __mm_s.push_str(&*intString(var.clone())); __mm_s.push_str(&*literal!(" would causalize ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!(" Eqns\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok((cVars, outMax))
}

fn getOneVarWithMostPoints(mut inVarList: Arc<metamodelica::List<i32>>, mut inPointsLst: Arc<metamodelica::List<i32>>) -> (Arc<metamodelica::List<i32>>, i32) {
    let mut outVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outMax: i32 = 0;
    let mut index: i32 = 1;
    outMax = {
        let mut __acc: Option<i32> = None;
        for mut i in (inPointsLst.clone()).into_iter().cloned() {
            let __x = i.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction")).unwrap()
    };
    for mut i in &*inPointsLst.clone() {
        let mut i = i.clone();
        if i.clone() == outMax.clone() {
            outVarList = list![(inVarList.clone()).get(index.clone()).unwrap()];
            return (outVarList, outMax);
        }
        index = index.clone() + 1;
    }
    (outVarList, outMax)
}

fn getAllVarsWithMostPoints(mut inVarList: Arc<metamodelica::List<i32>>, mut inPointsLst: Arc<metamodelica::List<i32>>, mut outVarList: Arc<metamodelica::List<i32>>, mut outMax: i32) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut outVarList: Arc<metamodelica::List<i32>> = outVarList;
    let mut outMax: i32 = outMax;
    let _ = (::match_deref::match_deref! { match &((inVarList.clone(), inPointsLst.clone())) {
        (Deref @ metamodelica::List::Cons { head: v, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: p, tail: Deref @ metamodelica::List::Nil }) => {
            if intGt(p.clone(), outMax.clone()) {
                outMax = p.clone();
                outVarList = list![v.clone()];
            } else if intEq(p.clone(), outMax.clone()) {
                outVarList = cons(v.clone(), outVarList.clone());
            }
            ()
        },
        (Deref @ metamodelica::List::Cons { head: v, tail: vrest }, Deref @ metamodelica::List::Cons { head: p, tail: prest }) => {
            if intGt(p.clone(), outMax.clone()) {
                outMax = p.clone();
                outVarList = list![v.clone()];
            } else if intEq(p.clone(), outMax.clone()) {
                outVarList = cons(v.clone(), outVarList.clone());
            }
            (outVarList, outMax) = getAllVarsWithMostPoints(vrest.clone(), prest.clone(), outVarList.clone(), outMax.clone())?;
            ()
        },
        _ => {
            Error::addCompilerError((literal!("Tearing.getAllVarsWithMostPoints: Finding variables with most points failed.")).clone())?;
            bail!("fail");
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVarList, outMax))
}

fn sizeOfAssignable(mut Eqn: i32, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1: metamodelica::Array<i32>, mut inSize: i32) -> i32 {
    let mut outSize: i32 = 0;
    let mut vars: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut b: bool = false;
    vars = List::removeOnTrue(ass1.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), me.borrow()[(Eqn.clone()-1) as usize].clone());
    b = solvableLst(vars.clone());
    outSize = if (b.clone()) {inSize.clone() + 1} else {inSize.clone()};
    outSize
}

fn getAllVarsWithMostImpAss(mut inPotentials: Arc<metamodelica::List<i32>>, mut ass2: metamodelica::Array<i32>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, i32)> {
    let mut outPotentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outCounts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outMax: i32 = 0;
    let mut count: i32 = 0;
    let mut elem: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    for mut v in &*inPotentials.clone() {
        let mut v = v.clone();
        elem = List::removeOnTrue(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), meT.borrow()[(v.clone()-1) as usize].clone());
        count = countImpossibleAss(elem.clone())?;
        if count.clone() > outMax.clone() {
            outPotentials = list![v.clone()];
            outMax = count.clone();
        } else if count.clone() == outMax.clone() {
            outPotentials = cons(v.clone(), outPotentials.clone());
        }
        outCounts = cons(count.clone(), outCounts.clone());
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Var ")); __mm_s.push_str(&*intString(v.clone())); __mm_s.push_str(&*literal!(" has ")); __mm_s.push_str(&*intString(count.clone())); __mm_s.push_str(&*literal!(" incident impossible assignments\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    outCounts = outCounts.clone().reverse();
    Ok((outPotentials, outCounts, outMax))
}

fn getOneVarWithMostImpAss(mut inPotentials: Arc<metamodelica::List<i32>>, mut ass2: metamodelica::Array<i32>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut outPotentials: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outMax: i32 = -1;
    let mut count: i32 = 0;
    let mut elem: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    for mut v in &*inPotentials.clone() {
        let mut v = v.clone();
        elem = List::removeOnTrue(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), meT.borrow()[(v.clone()-1) as usize].clone());
        count = countImpossibleAss(elem.clone())?;
        if count.clone() > outMax.clone() {
            outPotentials = list![v.clone()];
            outMax = count.clone();
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Var ")); __mm_s.push_str(&*intString(v.clone())); __mm_s.push_str(&*literal!(" has ")); __mm_s.push_str(&*intString(count.clone())); __mm_s.push_str(&*literal!(" incident impossible assignments\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok((outPotentials, outMax))
}

fn countImpossibleAss(mut elem: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<i32> {
    let mut outCount: i32 = 0;
    let mut s: BackendDAE::Solvability = BackendDAE::Solvability::SOLVABILITY_CONSTONE;
    for mut e in &*elem.clone() {
        let mut e = e.clone();
        (_, s, _) = e.clone();
        if !(solvable(s.clone())?) {
            outCount = outCount.clone() + 1;
        }
    }
    Ok(outCount)
}

fn TarjanMatching(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut orderIn: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut eqnNonlinPoints: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, bool)> {
    let mut orderOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut causal: bool = false;
    let mut subOrder: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = orderIn.clone();
    let mut assignable: bool = true;
    let debug: bool = false;
    while assignable.clone() {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("\nTarjanAssignment:\n")).clone());
        }
        (order, assignable) = TarjanAssignment(mIn.clone(), mtIn.clone(), meIn.clone(), ass1In.clone(), ass2In.clone(), order.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone())?;
    }
    if debug.clone() {
        execStat((literal!("Tearing.TarjanMatching iters done")).clone())?;
    }
    unassigned = getUnassigned(ass1In.clone());
    if unassigned.clone().is_empty() {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("\ncausal\n")).clone());
        }
        orderOut = order.clone().reverse();
        causal = true;
    } else {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("\nnoncausal\n")).clone());
        }
        orderOut = order.clone();
        causal = false;
    }
    if debug.clone() {
        execStat((literal!("Tearing.TarjanMatching done")).clone())?;
    }
    Ok((orderOut, causal))
}

fn TarjanAssignment(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meIn: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut orderIn: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut eqnNonlinPoints: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, bool)> {
    let mut orderOut: Arc<metamodelica::List<i32>> = orderIn.clone();
    let mut assignable: bool = false;
    let mut eq_coll: i32 = 0;
    let mut assEq_coll: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    assEq_coll = traverseCollectiveEqnsforAssignable(ass2In.clone(), mIn.clone(), mapEqnIncRow.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("New assEq_coll: ")); __mm_s.push_str(&*stringDelimitList(List::map(assEq_coll.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    if '__try0: {
        (eq_coll, eqns, vars) = unwrap_break_err!(getNextSolvableEqn(assEq_coll.clone(), mIn.clone(), meIn.clone(), ass1In.clone(), ass2In.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), eqnNonlinPoints.clone()), '__try0);
        orderOut = cons(eq_coll.clone(), orderOut.clone());
        assignable = true;
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    if assignable.clone() {
        makeAssignment(eqns.clone(), vars.clone(), ass1In.clone(), ass2In.clone(), mIn.clone(), mtIn.clone())?;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("order: ")); __mm_s.push_str(&*stringDelimitList(List::map(orderOut.clone().reverse(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((orderOut, assignable))
}

fn traverseSingleEqnsforAssignable(mut inAss: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnColl: i32 = 0;
    let mut eqnSize: i32 = 0;
    let mut delst: DoubleEnded::MutableList<i32>;
    delst = DoubleEnded::empty(0);
    let __range0 = 1..=(inAss.clone().borrow().len() as i32);
    for mut e in __range0 {
        if inAss.clone().borrow()[(e.clone()-1) as usize].clone() != -1 {
            continue;
        }
        eqnColl = mapIncRowEqn.borrow()[(e.clone()-1) as usize].clone();
        eqnSize = (mapEqnIncRow.borrow()[(eqnColl.clone()-1) as usize].clone().len() as i32);
        if (m.borrow()[(e.clone()-1) as usize].clone().len() as i32) == eqnSize.clone() + 1 {
            if eqnSize.clone() == 1 {
                DoubleEnded::push_back(delst.clone(), e.clone());
            } else {
                DoubleEnded::push_front(delst.clone(), e.clone());
            }
        }
    }
    selectedrows = DoubleEnded::toListAndClear(delst.clone(), metamodelica::nil());
    Ok(selectedrows)
}

fn traverseCollectiveEqnsforAssignable(mut inAss: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut selectedrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnSize: i32 = 0;
    let mut e: i32 = 0;
    let mut eqnColl: i32 = 0;
    let mut delst: DoubleEnded::MutableList<i32>;
    delst = DoubleEnded::empty(0);
    let __range0 = mapEqnIncRow.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut eqnLst in __range0 {
        eqnColl = eqnColl.clone() + 1;
        e = listHead(eqnLst.clone())?;
        if inAss.clone().borrow()[(e.clone()-1) as usize].clone() != -1 {
            continue;
        }
        eqnSize = (eqnLst.clone().len() as i32);
        if (m.borrow()[(e.clone()-1) as usize].clone().len() as i32) == eqnSize.clone() {
            if eqnSize.clone() == 1 {
                DoubleEnded::push_back(delst.clone(), eqnColl.clone());
            } else {
                DoubleEnded::push_front(delst.clone(), eqnColl.clone());
            }
        }
    }
    selectedrows = DoubleEnded::toListAndClear(delst.clone(), metamodelica::nil());
    Ok(selectedrows)
}

fn makeAssignment(mut eqns: Arc<metamodelica::List<i32>>, mut vars: Arc<metamodelica::List<i32>>, mut ass1In: metamodelica::Array<i32>, mut ass2In: metamodelica::Array<i32>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut eq: i32 = 0;
    let mut var: i32 = 0;
    for mut index in 1..=(eqns.clone().len() as i32) {
        eq = (eqns.clone()).get(index.clone())?;
        var = (vars.clone()).get(index.clone())?;
        {let _arr = ass1In.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = eq.clone(); _arr};
        {let _arr = ass2In.clone(); _arr.borrow_mut()[(eq.clone()-1) as usize] = var.clone(); _arr};
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("assignment: Eq ")); __mm_s.push_str(&*intString(eq.clone())); __mm_s.push_str(&*literal!(" - Var ")); __mm_s.push_str(&*intString(var.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        let _ = Array::replaceAtWithFill(eq.clone(), metamodelica::nil(), metamodelica::nil(), mIn.clone())?;
        deleteEntriesFromAdjacencyMatrix(mIn.clone(), mtIn.clone(), list![var.clone()])?;
        let _ = Array::replaceAtWithFill(var.clone(), metamodelica::nil(), metamodelica::nil(), mtIn.clone())?;
        deleteEntriesFromAdjacencyMatrix(mtIn.clone(), mIn.clone(), list![eq.clone()])?;
    }
    Ok(())
}

fn getNextSolvableEqn(mut assEq_coll: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut eqnNonlinPoints: metamodelica::Array<i32>) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut eqOut: i32 = 0;
    let mut eqnsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut solvable: bool = false;
    let mut eqns: Arc<metamodelica::List<i32>> = assEq_coll.clone();
    while !(eqns.clone().is_empty()) {
        eqOut = getMostNonlinearEquation(eqnNonlinPoints.clone(), eqns.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
        (solvable, eqnsOut, varsOut) = eqnSolvableCheck(eqOut.clone(), mapEqnIncRow.clone(), ass1.clone(), m.clone(), me.clone())?;
        (eqns, _) = List::deleteMemberOnTrue(eqOut.clone(), eqns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Most nonlinear equation: ")); __mm_s.push_str(&*intString(eqOut.clone())); __mm_s.push_str(&*literal!(" - solvable?: ")); __mm_s.push_str(&*boolString(solvable.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        if solvable.clone() {
            break;
        } else {
            let __range0 = &*mapEqnIncRow.borrow()[(eqOut.clone()-1) as usize].clone();
            for mut eq in __range0 {
                let mut eq = eq.clone();
                {let _arr = ass2.clone(); _arr.borrow_mut()[(eq.clone()-1) as usize] = -2; _arr};
            }
        }
    }
    if !(solvable.clone()) {
        bail!("fail");
    }
    Ok((eqOut, eqnsOut, varsOut))
}

fn eqnSolvableCheck(mut eqn_coll: i32, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<(bool, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut solvable: bool = false;
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqn: i32 = 0;
    let mut vars_enh: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    eqns = mapEqnIncRow.borrow()[(eqn_coll.clone()-1) as usize].clone();
    eqn = listHead(eqns.clone())?;
    vars = m.clone().borrow()[(eqn.clone()-1) as usize].clone();
    vars_enh = List::removeOnTrue(ass1.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), me.borrow()[(eqn.clone()-1) as usize].clone());
    solvable = solvableLst(vars_enh.clone());
    Ok((solvable, eqns, vars))
}

fn assignInnerEquations(mut inEqns: Arc<metamodelica::List<i32>>, mut eindex: Arc<metamodelica::List<i32>>, mut vindex: Arc<metamodelica::List<i32>>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut meOpt: Option<metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>>) -> Result<Arc<metamodelica::List<BackendDAE::InnerEquation>>> {
    let mut outInnerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    outInnerEquations = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
        for mut eqn in (inEqns.clone()).into_iter().cloned() {
            let __x = (match (eqn.clone(), meOpt.clone()) {
        (mut eq, None) => {
            let mut otherEqn: i32 = 0;
            let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            vars = List::map1r(mapEqnIncRow.borrow()[(eq.clone()-1) as usize].clone(), Arc::new(arrayGet.clone()), ass2.clone());
            otherEqn = (eindex.clone()).get(eq.clone())?;
            otherVars = selectFromList_rev(vindex.clone(), vars.clone());
            BackendDAE::InnerEquation::INNEREQUATION { vars: otherVars.clone(), eqn: otherEqn.clone() }
        },
        (mut eq, Some(mut me)) => {
            let mut otherEqn: i32 = 0;
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut innerEquation: BackendDAE::InnerEquation;
            let mut constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
            eqns = mapEqnIncRow.borrow()[(eq.clone()-1) as usize].clone();
            vars = List::map1r(eqns.clone(), Arc::new(arrayGet.clone()), ass2.clone());
            otherEqn = (eindex.clone()).get(eq.clone())?;
            otherVars = selectFromList_rev(vindex.clone(), vars.clone());
            constraints = findConstraintForInnerEquation(me.borrow()[(listHead(eqns.clone())?-1) as usize].clone(), listHead(vars.clone())?);
            if constraints.clone().is_empty() {
                innerEquation = BackendDAE::InnerEquation::INNEREQUATION { vars: otherVars.clone(), eqn: otherEqn.clone() };
            } else {
                innerEquation = BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS { cons: constraints.clone(), vars: otherVars.clone(), eqn: otherEqn.clone() };
            }
            innerEquation.clone()
        },
        _ => bail!("match: no arm matched"),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outInnerEquations)
}

fn findConstraintForInnerEquation(mut meRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut searchIndex: i32) -> Arc<metamodelica::List<Arc<DAE::Constraint>>> {
    let mut constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut meElem: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>);
    let mut cons: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    for mut meElem in &*meRow.clone() {
        let mut meElem = meElem.clone();
        (index, _, cons) = meElem.clone();
        if intEq(index.clone(), searchIndex.clone()) {
            constraints = cons.clone();
            break;
        }
    }
    constraints
}

fn markTVarsOrResiduals(mut markList: Arc<metamodelica::List<i32>>, mut assIn: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut assOut: metamodelica::Array<i32> = assIn.clone();
    let mut len: i32 = 0;
    len = (assIn.clone().borrow().len() as i32);
    for mut i in &*markList.clone() {
        let mut i = i.clone();
        {let _arr = assOut.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = len.clone() * 2; _arr};
    }
    Ok(assOut)
}

fn countMultiples(mut inArr: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut counter: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut numbers: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut values: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (counter, numbers, values, _) = Array::fold(inArr.clone(), (std::sync::Arc::new(countMultiples2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, i32)> + 'static>), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), 1));
    (counter, numbers, values)
}

fn countMultiples2(mut rowIn: Arc<metamodelica::List<i32>>, mut valIn: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, i32)> {
    let mut valOut: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, i32);
    let mut counter: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut values: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut set: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut num: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut val: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut positions: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut numbers: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut indx: i32 = 0;
    let mut value: i32 = 0;
    let mut number: i32 = 0;
    let mut position: i32 = 0;
    (counter, _, values, indx) = valIn.clone();
    row = List::removeOnTrue(0, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rowIn.clone());
    set = List::unique(row.clone());
    if set.clone().is_empty() {
        val = list![0];
        num = list![0];
    } else {
        (val, num) = countMultiples3(row.clone(), set.clone(), metamodelica::nil(), metamodelica::nil());
    }
    positions = maxListInt(num.clone());
    position = listHead(positions.clone())?;
    number = (num.clone()).get(position.clone())?;
    numbers = selectFromList(val.clone(), positions.clone());
    value = (val.clone()).get(position.clone())?;
    counter = List::set(counter.clone(), indx.clone(), number.clone())?;
    values = List::set(values.clone(), indx.clone(), value.clone())?;
    valOut = (counter.clone(), numbers.clone(), values.clone(), indx.clone() + 1);
    Ok(valOut)
}

fn countMultiples3(mut lstIn: Arc<metamodelica::List<i32>>, mut set: Arc<metamodelica::List<i32>>, mut valIn: Arc<metamodelica::List<i32>>, mut numIn: Arc<metamodelica::List<i32>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut valOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut numOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (valOut, numOut) = (::match_deref::match_deref! { match &((lstIn.clone(), set.clone(), valIn.clone(), numIn.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: value, tail: rest }, _, _) => {
            let mut number: i32 = 0;
            let mut val: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut num: Arc<metamodelica::List<i32>> = metamodelica::nil();
            number = (lstIn.clone().len() as i32) - (List::removeOnTrue(value.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lstIn.clone()).len() as i32);
            (val, num) = countMultiples3(lstIn.clone(), rest.clone(), cons(value.clone(), valIn.clone()), cons(number.clone(), numIn.clone()));
            (val.clone(), num.clone())
        },
        _ => {
            (valIn.clone(), numIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (valOut, numOut)
}

fn maxListInt(mut inList: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut maxi: i32 = 0;
    let mut index: i32 = 1;
    maxi = {
        let mut __acc: Option<i32> = None;
        for mut i in (inList.clone()).into_iter().cloned() {
            let __x = i.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction")).unwrap()
    };
    for mut i in &*inList.clone() {
        let mut i = i.clone();
        if i.clone() == maxi.clone() {
            outList = cons(index.clone(), outList.clone());
        }
        index = index.clone() + 1;
    }
    outList
}

fn getMostNonlinearEquation(mut inArray: metamodelica::Array<i32>, mut inList: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<i32> {
    let mut index: i32 = 1;
    let mut maxi: i32 = 0;
    maxi = {
        let mut __acc: Option<i32> = None;
        for mut i in (inList.clone()).into_iter().cloned() {
            let __x = inArray.borrow()[(listHead(mapEqnIncRow.borrow()[(i.clone()-1) as usize].clone())?-1) as usize].clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
    for mut i in &*inList.clone() {
        let mut i = i.clone();
        index = listHead(mapEqnIncRow.borrow()[(i.clone()-1) as usize].clone())?;
        if inArray.borrow()[(index.clone()-1) as usize].clone() == maxi.clone() {
            index = mapIncRowEqn.borrow()[(index.clone()-1) as usize].clone();
            return Ok(index);
        }
    }
    Ok(index)
}

fn selectFromList_rev(mut inList: Arc<metamodelica::List<i32>>, mut selList: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut actual: i32 = 0;
    let mut len: i32 = 0;
    let mut lst: Arc<metamodelica::List<i32>> = selList.clone();
    len = (inList.clone().len() as i32);
    outList = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut num in (selList.clone()).into_iter().cloned() {
            if !(num.clone() > 0 && num.clone() <= len.clone()) { continue; }
            let __x = (inList.clone()).get(num.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

fn selectFromList(mut inList: Arc<metamodelica::List<i32>>, mut selList: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut num: i32 = 0;
    let mut actual: i32 = 0;
    let mut len: i32 = 0;
    let mut lst: Arc<metamodelica::List<i32>> = selList.clone();
    len = (inList.clone().len() as i32);
    for mut num in &*selList.clone() {
        let mut num = num.clone();
        if num.clone() > 0 && num.clone() <= len.clone() {
            actual = (inList.clone()).get(num.clone()).unwrap();
            outList = cons(actual.clone(), outList.clone());
        }
    }
    outList
}

fn deleteEntriesFromAdjacencyMatrix(mut mUpdate: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mHelp: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut entries: Arc<metamodelica::List<i32>>) -> Result<()> {
    let mut rowIndx: i32 = 0;
    let mut rowsIndx: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut entry in &*entries.clone() {
        let mut entry = entry.clone();
        rowsIndx = mHelp.clone().borrow()[(entry.clone()-1) as usize].clone();
        for mut rowIndx in &*rowsIndx.clone() {
            let mut rowIndx = rowIndx.clone();
            row = mUpdate.clone().borrow()[(rowIndx.clone()-1) as usize].clone();
            (row, _) = List::deleteMemberOnTrue(entry.clone(), row.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            Array::replaceAtWithFill(rowIndx.clone(), row.clone(), row.clone(), mUpdate.clone())?;
        }
    }
    Ok(())
}

fn deleteRowsFromAdjacencyMatrix(mut mUpdate: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rows: Arc<metamodelica::List<i32>>) -> Result<()> {
    for mut row in &*rows.clone() {
        let mut row = row.clone();
        let _ = Array::replaceAtWithFill(row.clone(), metamodelica::nil(), metamodelica::nil(), mUpdate.clone())?;
    }
    Ok(())
}

fn getVarsOfEqnsWithMostVars(mut inVars: Arc<metamodelica::List<i32>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Arc<metamodelica::List<i32>> {
    let mut outVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut maxSize: i32 = 0;
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqn_size_arr: metamodelica::Array<i32>;
    eqn_size_arr = arrayCreate((mIn.clone().borrow().len() as i32), -1);
    let __range0 = 1..=(mIn.clone().borrow().len() as i32);
    for mut i in __range0 {
        size = (mIn.borrow()[(i.clone()-1) as usize].clone().len() as i32);
        {
            let __cell1 = size.clone();
            eqn_size_arr.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
        }
        if size.clone() > maxSize.clone() {
            maxSize = size.clone();
        }
    }
    for mut var in &*inVars.clone() {
        let mut var = var.clone();
        eqns = mtIn.borrow()[(var.clone()-1) as usize].clone();
        for mut e in &*eqns.clone() {
            let mut e = e.clone();
            if eqn_size_arr.borrow()[(e.clone()-1) as usize].clone() == maxSize.clone() {
                outVars = cons(var.clone(), outVars.clone());
                break;
            }
        }
    }
    GCExt::free(eqn_size_arr.clone());
    outVars
}

fn getVarsOccurringInMostEquations(mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inSelect: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut length: i32 = 0;
    let mut outLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut length1: i32 = 0;
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut sel in &*inSelect.clone() {
        let mut sel = sel.clone();
        row = mtIn.clone().borrow()[(sel.clone()-1) as usize].clone();
        length1 = (row.clone().len() as i32);
        if intGt(length1.clone(), length.clone()) {
            length = length1.clone();
            outLst = list![sel.clone()];
        } else if intEq(length1.clone(), length.clone()) {
            outLst = cons(sel.clone(), outLst.clone());
        }
    }
    Ok((length, outLst))
}

fn getVarOccurringInMostEquations(mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inSelect: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut length: i32 = 0;
    let mut outLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut length1: i32 = 0;
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut sel in &*inSelect.clone() {
        let mut sel = sel.clone();
        row = mtIn.clone().borrow()[(sel.clone()-1) as usize].clone();
        length1 = (row.clone().len() as i32);
        if intGt(length1.clone(), length.clone()) {
            length = length1.clone();
            outLst = list![sel.clone()];
        }
    }
    Ok((length, outLst))
}

fn findNEntries(mut mtIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inSelect: Arc<metamodelica::List<i32>>, mut num: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut length: i32 = 0;
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut sel in &*inSelect.clone() {
        let mut sel = sel.clone();
        row = mtIn.clone().borrow()[(sel.clone()-1) as usize].clone();
        length = (row.clone().len() as i32);
        if intEq(num.clone(), length.clone()) {
            outList = cons(sel.clone(), outList.clone());
        }
    }
    Ok(outList)
}

// =============================================================================
// section for preOptModule >>recursiveTearing<<
//
// inline and repeat tearing
// author: Vitalij Ruge
// =============================================================================
pub fn recursiveTearing(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut con: bool = false;
    if Flags::getConfigInt(Flags::RTEARING.clone())? > 0 {
        (outDAE, con) = recursiveTearingMain(inDAE.clone())?;
        while con.clone() {
            outDAE = tearingSystem(outDAE.clone())?;
            (outDAE, con) = recursiveTearingMain(outDAE.clone())?;
        }
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

fn recursiveTearingCollect(mut tear_exp: metamodelica::Array<Arc<DAE::Exp>>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut k: i32 = 0;
    let mut lhs: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    (e1, e2) = ExpressionSolve::collectX(inExp.clone(), tear_exp.clone().borrow()[(1-1) as usize].clone(), true)?;
    let __range0 = 2..=(tear_exp.clone().borrow().len() as i32);
    for mut k in __range0 {
        (lhs, e2) = ExpressionSolve::collectX(e2.clone(), tear_exp.clone().borrow()[(k.clone()-1) as usize].clone(), true)?;
        e1 = Expression::expAdd(e1.clone(), lhs.clone())?;
    }
    outExp = Expression::expAdd(e2.clone(), e1.clone())?;
    Ok(outExp)
}

fn isTornsystem(mut comp: Arc<BackendDAE::StrongComponent>, mut getLin: bool, mut getNoLin: bool) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear, .. } if (linear.clone() == getLin.clone() || getNoLin.clone() == !(linear.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn recursiveTearingHelper(mut rhs1: Arc<DAE::Exp>, mut tear_exp: metamodelica::Array<Arc<DAE::Exp>>, mut m: i32) -> Result<Arc<DAE::Exp>> {
    let mut sumRhs: Arc<DAE::Exp> = Expression::makeConstZeroE(rhs1.clone());
    let mut k: i32 = 0;
    let mut e: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp> = rhs1.clone();
    for mut k in 1..=m.clone() {
        (e, rhs) = ExpressionSolve::collectX(rhs.clone(), tear_exp.clone().borrow()[(k.clone()-1) as usize].clone(), true)?;
        sumRhs = Expression::expAdd(e.clone(), sumRhs.clone())?;
    }
    sumRhs = Expression::expAdd(rhs.clone(), sumRhs.clone())?;
    (sumRhs, _) = ExpressionSimplify::simplify(sumRhs.clone())?;
    Ok(sumRhs)
}

fn recursiveTearingReplace(mut inExp: Arc<DAE::Exp>, mut inSourceExp: Arc<DAE::ComponentRef>, mut inTargetExp: Arc<DAE::Exp>, mut isDer: bool) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    if isDer.clone() {
        res = Expression::crefExp(inSourceExp.clone())?;
        res = Expression::expDer(res.clone());
        (res, _) = Expression::replaceExp(inExp.clone(), res.clone(), inTargetExp.clone())?;
    } else {
        res = Expression::replaceCrefBottomUp(inExp.clone(), inSourceExp.clone(), inTargetExp.clone())?;
    }
    Ok(res)
}

fn getUnassigned(mut ass: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __range0 = 1..=(ass.clone().borrow().len() as i32);
    for mut i in __range0 {
        if Dangerous::arrayGet(ass.clone(), i.clone()).unwrap() < 0 {
            unassigned = cons(i.clone(), unassigned.clone());
        }
    }
    unassigned
}

fn dumpTearingSetLocalIndexes(mut tVars: Arc<metamodelica::List<i32>>, mut residuals: Arc<metamodelica::List<i32>>, mut order: Arc<metamodelica::List<i32>>, mut ass2: metamodelica::Array<i32>, mut size: i32, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut setString: ArcStr) -> Result<()> {
    let mut s: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n* TEARING RESULTS")); __mm_s.push_str(&*setString.clone()); __mm_s.push_str(&*literal!(":\n* (Local Indexes)\n*\n* No of equations in strong component: ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* No of tVars: ")); __mm_s.push_str(&*intString((tVars.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*\n* tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(tVars.clone().reverse(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if Flags::isSet(Flags::ITERATION_VARS.clone())? {
        s = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut tVar in (tVars.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* ")); __mm_s.push_str(&*intString(tVar.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*BackendDump::varString(BackendVariable::getVarAt(vars.clone(), tVar.clone())?)?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(s.clone(), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*\n* resEq: ")); __mm_s.push_str(&*stringDelimitList(List::map(residuals.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if Flags::isSet(Flags::ITERATION_VARS.clone())? && Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        s = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut eqn in (residuals.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* ")); __mm_s.push_str(&*intString(eqn.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*BackendDump::equationString(BackendEquation::get(eqns.clone(), eqn.clone()))?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(s.clone(), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    s = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (order.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*stringDelimitList(List::map(List::map1r(mapEqnIncRow.borrow()[(e.clone()-1) as usize].clone(), Arc::new(arrayGet.clone()), ass2.clone()), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*\n* innerEquations ({eqn,vars}):\n* ")); __mm_s.push_str(&*stringDelimitList(s.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n*\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn dumpTearingSetGlobalIndexes(mut tearingSet: BackendDAE::TearingSet, mut size: i32, mut setString: ArcStr) -> Result<()> {
    let mut tVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut residuals: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let BackendDAE::TEARINGSET { innerEquations: __pa0, residualequations: __pa1, tearingvars: __pa2, .. } = (tearingSet.clone()) else { bail!("pattern mismatch") };
    innerEquations = __pa0.clone();
    residuals = __pa1.clone();
    tVars = __pa2.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n* TEARING RESULTS")); __mm_s.push_str(&*setString.clone()); __mm_s.push_str(&*literal!(":\n* (Global Indexes)\n*\n* No of equations in strong component: ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* No of tVars: ")); __mm_s.push_str(&*intString((tVars.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*\n* tVars: ")); __mm_s.push_str(&*stringDelimitList(List::map(tVars.clone().reverse(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*\n* resEq: ")); __mm_s.push_str(&*stringDelimitList(List::map(residuals.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*\n* innerEquations ({eqn,vars}):\n* ")); __mm_s.push_str(&*stringDelimitList(List::map(innerEquations.clone(), (std::sync::Arc::new(BackendDump::innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n*\n*")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn dumpTearingSetsGlobalIndexes(mut tearingSets: Arc<metamodelica::List<BackendDAE::TearingSet>>, mut size: i32) -> Result<()> {
    let mut tVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut residuals: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    for mut tearingSet in &*tearingSets.clone() {
        let mut tearingSet = tearingSet.clone();
        dumpTearingSetGlobalIndexes(tearingSet.clone(), size.clone(), (literal!("")).clone())?;
    }
    Ok(())
}

// =============================================================================
//
// Total Tearing - Determination of All Possible Tearing Sets
// author: ptaeuber FHB 2016
//
// =============================================================================
fn totalTearing(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut eindex: Arc<metamodelica::List<i32>>, mut vindx: Arc<metamodelica::List<i32>>, mut ojac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>, mut jacType: BackendDAE::JacobianType, mut mixedSystem: bool) -> Result<(Arc<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: Arc<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool = false;
    let mut size: i32 = 0;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut causEq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut discreteVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subsyst: Arc<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mLoop: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mtLoop: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut DAEtype: BackendDAE::BackendDAEType = BackendDAE::BackendDAEType::ALGEQSYSTEM;
    let mut tearingSets: Arc<metamodelica::List<BackendDAE::TearingSet>> = metamodelica::nil();
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut linear: bool = false;
    let mut simulation: bool = false;
    let mut modelName: ArcStr = arcstr::literal!("");
    let mut powerSet: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut matchingList: Arc<metamodelica::List<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    linear = BackendDAEUtil::getLinearfromJacType(jacType.clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ishared.clone()) {
        Deref @ BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: __pa0, .. }, backendDAEType: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    modelName = __pa0.clone();
    DAEtype = __pa1.clone();
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of totalTearing\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    size = (vindx.clone().len() as i32);
    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()));
    eqns = BackendEquation::listEquation(eqn_lst.clone())?;
    var_lst = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), BackendVariable::daeVars(isyst.clone()));
    vars = BackendVariable::listVar1(var_lst.clone());
    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (subsyst, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(subsyst.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
    m = Array::map(m.clone(), (std::sync::Arc::new(fnptr!(deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>));
    mt = Array::map(mt.clone(), (std::sync::Arc::new(fnptr!(deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>));
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("\n\n###BEGIN print Strong Component#####################\n(Function:totalTearing)\n")).clone());
        BackendDump::printEqSystem(subsyst.clone())?;
        println!("{}", (literal!("\n###END print Strong Component#######################\n(Function:totalTearing)\n\n\n")).clone());
    }
    (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(subsyst.clone(), ishared.clone(), false)?;
    unsolvables = getUnsolvableVars(size.clone(), meT.clone())?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("\nAdjacencyMatrixEnhanced:\n")).clone());
        BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
        println!("{}", (literal!("\nAdjacencyMatrixTransposedEnhanced:\n")).clone());
        BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("\n\nmapEqnIncRow:")).clone());
        BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(mapIncRowEqn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nUNSOLVABLES:\n")); __mm_s.push_str(&*stringDelimitList(List::map(unsolvables.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    discreteVars = findDiscrete(var_lst.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDiscrete Vars:\n")); __mm_s.push_str(&*stringDelimitList(List::map(discreteVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    for mut i in (1..=Util::intPow(2, size.clone())? - 1).rev() {
        powerSet = cons(getPowerSetElement(i.clone()), powerSet.clone());
    }
    if Flags::isSet(Flags::TOTAL_TEARING_DUMP.clone())? || Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
        BackendDump::dumpListList(powerSet.clone(), (literal!("Power Set")).clone());
    }
    tearingSets = metamodelica::nil();
    if Flags::isSet(Flags::TOTAL_TEARING_DUMP.clone())? || Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\n###BEGIN TO LOOP#####################\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    for mut tVars in &*powerSet.clone() {
        let mut tVars = tVars.clone();
        if Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ntVars:\n")); __mm_s.push_str(&*stringDelimitList(List::map(tVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        ass1 = arrayCreate(size.clone(), -1);
        ass2 = arrayCreate(size.clone(), -1);
        order = metamodelica::nil();
        mLoop = metamodelica::arrayFromVec(m.clone().borrow().clone());
        mtLoop = metamodelica::arrayFromVec(mt.clone().borrow().clone());
        markTVarsOrResiduals(tVars.clone(), ass1.clone())?;
        deleteEntriesFromAdjacencyMatrix(mLoop.clone(), mtLoop.clone(), tVars.clone())?;
        deleteRowsFromAdjacencyMatrix(mtLoop.clone(), tVars.clone())?;
        causEq = traverseCollectiveEqnsforAssignable(ass2.clone(), mLoop.clone(), mapEqnIncRow.clone())?;
        matchingList = totalMatching(ass1.clone(), ass2.clone(), order.clone(), causEq.clone(), mLoop.clone(), mtLoop.clone(), me.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), metamodelica::nil())?;
        if Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
            dumpMatchingList(matchingList.clone());
        }
        tearingSets = createTearingSets(tVars.clone(), matchingList.clone(), vindx.clone(), eindex.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), tearingSets.clone())?;
    }
    if Flags::isSet(Flags::TOTAL_TEARING_DUMP.clone())? || Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
        dumpTearingSetsGlobalIndexes(tearingSets.clone(), size.clone())?;
    }
    ocomp = Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: listHead(tearingSets.clone())?, casualTearingSet: None, linear: linear.clone(), mixedSystem: mixedSystem.clone() });
    outRunMatching = true;
    if Flags::isSet(Flags::TOTAL_TEARING_DUMP.clone())? || Flags::isSet(Flags::TOTAL_TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nTotal number of different tearing sets: ")); __mm_s.push_str(&*intString((tearingSets.clone().len() as i32))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of totalTearing\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((ocomp, outRunMatching))
}

fn getPowerSetElement(mut i: i32) -> Arc<metamodelica::List<i32>> {
    let mut powerSetElement: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut c: i32 = 0;
    let mut e: i32 = i.clone();
    let mut r: i32 = 0;
    while !(intEq(e.clone(), 0)) {
        c = c.clone() + 1;
        r = intMod(e.clone(), 2);
        e = intDiv(e.clone(), 2);
        if intEq(r.clone(), 1) {
            powerSetElement = cons(c.clone(), powerSetElement.clone());
        }
    }
    powerSetElement
}

fn totalMatching(mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut orderIn: Arc<metamodelica::List<i32>>, mut causEqIn: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut matchingListIn: Arc<metamodelica::List<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)>>) -> Result<Arc<metamodelica::List<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)>>> {
    let mut matchingListOut: Arc<metamodelica::List<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)>> = matchingListIn.clone();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut causEq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_exp: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ass1Copy: metamodelica::Array<i32>;
    let mut ass2Copy: metamodelica::Array<i32>;
    let mut mCopy: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mtCopy: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut solvable: bool = false;
    for mut e in &*causEqIn.clone() {
        let mut e = e.clone();
        ass1Copy = metamodelica::arrayFromVec(ass1.clone().borrow().clone());
        ass2Copy = metamodelica::arrayFromVec(ass2.clone().borrow().clone());
        mCopy = metamodelica::arrayFromVec(m.clone().borrow().clone());
        mtCopy = metamodelica::arrayFromVec(mt.clone().borrow().clone());
        (solvable, e_exp, vars) = eqnSolvableCheck(e.clone(), mapEqnIncRow.clone(), ass1Copy.clone(), mCopy.clone(), me.clone())?;
        if !(solvable.clone()) {
            continue;
        } else {
            makeAssignment(e_exp.clone(), vars.clone(), ass1Copy.clone(), ass2Copy.clone(), mCopy.clone(), mtCopy.clone())?;
            order = cons(e.clone(), orderIn.clone());
            causEq = traverseCollectiveEqnsforAssignable(ass2Copy.clone(), mCopy.clone(), mapEqnIncRow.clone())?;
            if causEq.clone().is_empty() {
                unassigned = getUnassigned(ass1Copy.clone());
                if unassigned.clone().is_empty() {
                    if isNewMatching(matchingListOut.clone(), ass1Copy.clone())? {
                        matchingListOut = cons((ass1Copy.clone(), ass2Copy.clone(), order.clone().reverse()), matchingListOut.clone());
                    }
                }
            } else {
                matchingListOut = totalMatching(ass1Copy.clone(), ass2Copy.clone(), order.clone(), causEq.clone(), mCopy.clone(), mtCopy.clone(), me.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), matchingListOut.clone())?;
            }
        }
    }
    Ok(matchingListOut)
}

fn isNewMatching(mut matchingList: Arc<metamodelica::List<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)>>, mut ass1In: metamodelica::Array<i32>) -> Result<bool> {
    let mut b: bool = true;
    let mut ass1: metamodelica::Array<i32>;
    for mut matching in &*matchingList.clone() {
        let mut matching = matching.clone();
        (ass1, _, _) = matching.clone();
        if Array::isEqual(ass1In.clone(), ass1.clone())? {
            b = false;
            break;
        }
    }
    Ok(b)
}

fn createTearingSets(mut tVarsIn: Arc<metamodelica::List<i32>>, mut matchingList: Arc<metamodelica::List<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)>>, mut vindx: Arc<metamodelica::List<i32>>, mut eindex: Arc<metamodelica::List<i32>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut tearingSetsIn: Arc<metamodelica::List<BackendDAE::TearingSet>>) -> Result<Arc<metamodelica::List<BackendDAE::TearingSet>>> {
    let mut tearingSetsOut: Arc<metamodelica::List<BackendDAE::TearingSet>> = tearingSetsIn.clone();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut tVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut residual: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut residual_coll: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    for mut matching in &*matchingList.clone() {
        let mut matching = matching.clone();
        (ass1, ass2, order) = matching.clone();
        residual = getUnassigned(ass2.clone());
        residual_coll = List::map1r(residual.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
        residual_coll = List::unique(residual_coll.clone());
        tVars = selectFromList_rev(vindx.clone(), tVarsIn.clone());
        residual = selectFromList_rev(eindex.clone(), residual_coll.clone());
        innerEquations = assignInnerEquations(order.clone(), eindex.clone(), vindx.clone(), ass2.clone(), mapEqnIncRow.clone(), None)?;
        tearingSetsOut = cons(BackendDAE::TearingSet { tearingvars: tVars.clone(), residualequations: residual.clone(), innerEquations: innerEquations.clone(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) }, tearingSetsOut.clone());
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nTearing Variables:\n")); __mm_s.push_str(&*stringDelimitList(List::map(tVarsIn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Residual Equations:\n")); __mm_s.push_str(&*stringDelimitList(List::map(residual_coll.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok(tearingSetsOut)
}

fn dumpMatchingList(mut matchingList: Arc<metamodelica::List<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)>>) -> () {
    let mut c: i32 = 0;
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    println!("{}", (literal!("\n")).clone());
    for mut matching in &*matchingList.clone() {
        let mut matching = matching.clone();
        c = c.clone() + 1;
        (ass1, ass2, order) = matching.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Matching ")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ass1: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ass2: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("order: ")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    ()
}

// =============================================================================
//
// User-Defined Tearing - Determine the tearing set defined by the user
// author: ptaeuber FHB 2016
//
// =============================================================================
fn userDefinedTearing(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut eindex: Arc<metamodelica::List<i32>>, mut vindx: Arc<metamodelica::List<i32>>, mut ojac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>, mut jacType: BackendDAE::JacobianType, mut mixedSystem: bool, mut userTVars: Arc<metamodelica::List<i32>>, mut userResiduals: Arc<metamodelica::List<i32>>) -> Result<(Arc<BackendDAE::StrongComponent>, bool)> {
    let mut ocomp: Arc<BackendDAE::StrongComponent>;
    let mut outRunMatching: bool = false;
    let mut size: i32 = 0;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut residuals: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut causEq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut discreteVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut userResiduals_exp: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subsyst: Arc<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut DAEtype: BackendDAE::BackendDAEType = BackendDAE::BackendDAEType::ALGEQSYSTEM;
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut tearingSet: BackendDAE::TearingSet;
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut linear: bool = false;
    let mut modelName: ArcStr = arcstr::literal!("");
    linear = BackendDAEUtil::getLinearfromJacType(jacType.clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ishared.clone()) {
        Deref @ BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: __pa0, .. }, backendDAEType: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    modelName = __pa0.clone();
    DAEtype = __pa1.clone();
    size = (vindx.clone().len() as i32);
    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()));
    eqns = BackendEquation::listEquation(eqn_lst.clone())?;
    var_lst = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), BackendVariable::daeVars(isyst.clone()));
    vars = BackendVariable::listVar1(var_lst.clone());
    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (subsyst, m, mt, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(subsyst.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
    m = Array::map(m.clone(), (std::sync::Arc::new(fnptr!(deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>));
    mt = Array::map(mt.clone(), (std::sync::Arc::new(fnptr!(deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>));
    (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(subsyst.clone(), ishared.clone(), false)?;
    match '__try2: {
        userResiduals_exp = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut i in (userResiduals.clone()).into_iter().cloned() {
            let __x = mapEqnIncRow.clone().borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        Ok::<_, anyhow::Error>((userResiduals_exp.clone(),))
    } {
        Ok((__try2_o0,)) => {
            userResiduals_exp = __try2_o0;
        }
        Err(_) => {
            Error::addMessage(Error::USER_DEFINED_TEARING_ERROR.clone(), list![(literal!("Index out of bounds.")).clone()])?;
            bail!("fail");
        }
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nBEGINNING of userDefinedTearing\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUsers tearing vars: ")); __mm_s.push_str(&*stringDelimitList(List::map(userTVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUsers residual equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(userResiduals.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUsers residual equations expanded: ")); __mm_s.push_str(&*stringDelimitList(List::map(userResiduals_exp.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n\n###BEGIN print Strong Component#####################\n(Function:userDefinedTearing)\n")).clone());
        BackendDump::printEqSystem(subsyst.clone())?;
        println!("{}", (literal!("\n###END print Strong Component#######################\n(Function:userDefinedTearing)\n\n\n")).clone());
    }
    if !(intEq((userTVars.clone().len() as i32), (userResiduals_exp.clone().len() as i32))) {
        Error::addMessage(Error::USER_DEFINED_TEARING_ERROR.clone(), list![(literal!("The number of tearing variables and residual equations is not identical.")).clone()])?;
        bail!("fail");
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("\nAdjacencyMatrixEnhanced:\n")).clone());
        BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
        println!("{}", (literal!("\nAdjacencyMatrixTransposedEnhanced:\n")).clone());
        BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
    }
    unsolvables = getUnsolvableVars(size.clone(), meT.clone())?;
    discreteVars = findDiscrete(var_lst.clone());
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("\n\nmapEqnIncRow:")).clone());
        BackendDump::dumpAdjacencyMatrix(mapEqnIncRow.clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmapIncRowEqn:\n")); __mm_s.push_str(&*stringDelimitList(List::mapArray(mapIncRowEqn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nUNSOLVABLES:\n")); __mm_s.push_str(&*stringDelimitList(List::map(unsolvables.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDiscrete Vars:\n")); __mm_s.push_str(&*stringDelimitList(List::map(discreteVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    ass1 = arrayCreate(size.clone(), -1);
    ass2 = arrayCreate(size.clone(), -1);
    order = metamodelica::nil();
    markTVarsOrResiduals(userTVars.clone(), ass1.clone())?;
    markTVarsOrResiduals(userResiduals_exp.clone(), ass2.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nass1: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ass2: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(ass2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    deleteEntriesFromAdjacencyMatrix(m.clone(), mt.clone(), userTVars.clone())?;
    deleteRowsFromAdjacencyMatrix(mt.clone(), userTVars.clone())?;
    deleteEntriesFromAdjacencyMatrix(mt.clone(), m.clone(), userResiduals_exp.clone())?;
    deleteRowsFromAdjacencyMatrix(m.clone(), userResiduals_exp.clone())?;
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", (literal!("\nAdjacency Matrix without tvars and residuals:\n")).clone());
        BackendDump::dumpAdjacencyMatrix(m.clone());
        BackendDump::dumpAdjacencyMatrix(mt.clone());
    }
    if intEq((userTVars.clone().len() as i32), countEmptyRows(m.clone())) && intEq((userResiduals_exp.clone().len() as i32), countEmptyRows(mt.clone())) {
        causEq = traverseCollectiveEqnsforAssignable(ass2.clone(), m.clone(), mapEqnIncRow.clone())?;
        order = simpleMatching(ass1.clone(), ass2.clone(), order.clone(), causEq.clone(), m.clone(), mt.clone(), me.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
        tVars = selectFromList_rev(vindx.clone(), userTVars.clone());
        residuals = selectFromList_rev(eindex.clone(), userResiduals.clone());
        innerEquations = assignInnerEquations(order.clone(), eindex.clone(), vindx.clone(), ass2.clone(), mapEqnIncRow.clone(), None)?;
        tearingSet = BackendDAE::TearingSet { tearingvars: tVars.clone(), residualequations: residuals.clone(), innerEquations: innerEquations.clone(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) };
        ocomp = Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: tearingSet.clone(), casualTearingSet: None, linear: linear.clone(), mixedSystem: mixedSystem.clone() });
        outRunMatching = true;
        if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            dumpTearingSetLocalIndexes(userTVars.clone(), userResiduals.clone(), order.clone(), ass2.clone(), size.clone(), mapEqnIncRow.clone(), vars.clone(), eqns.clone(), (literal!("")).clone())?;
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            dumpTearingSetGlobalIndexes(tearingSet.clone(), size.clone(), (literal!("")).clone())?;
        }
    } else {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("\nMatching failed, choose different tearing set!\n\n\n")).clone());
        }
        Error::addCompilerError((literal!("There is no possible matching for a user-defined tearing set.")).clone())?;
        bail!("fail");
    }
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of userDefinedTearing\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((ocomp, outRunMatching))
}

fn countEmptyRows(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> i32 {
    let mut count: i32 = 0;
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        if row.clone().is_empty() {
            count = count.clone() + 1;
        }
    }
    count
}

fn simpleMatching(mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut orderIn: Arc<metamodelica::List<i32>>, mut causEqIn: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut orderOut: Arc<metamodelica::List<i32>> = orderIn.clone();
    let mut e: i32 = 0;
    let mut causEq: Arc<metamodelica::List<i32>> = causEqIn.clone();
    let mut e_exp: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart Matching:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    while !(causEq.clone().is_empty()) {
        if let Ok((__pa0, __pa1, __pa2)) = getNextSolvableEqn(causEq.clone(), m.clone(), me.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), ass1.clone()) {
            e = __pa0.clone();
            e_exp = __pa1.clone();
            vars = __pa2.clone();
        } else {
            if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                println!("{}", (literal!("\nMatching failed, choose different tearing set!\n\n\n")).clone());
            }
            Error::addCompilerError((literal!("There is no possible matching for a user-defined tearing set.")).clone())?;
            bail!("fail");
        }
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("causEq: ")); __mm_s.push_str(&*stringDelimitList(List::map(causEq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\nProcess ")); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!(":\ne_exp: ")); __mm_s.push_str(&*stringDelimitList(List::map(e_exp.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        makeAssignment(e_exp.clone(), vars.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone())?;
        orderOut = cons(e.clone(), orderOut.clone());
        causEq = traverseCollectiveEqnsforAssignable(ass2.clone(), m.clone(), mapEqnIncRow.clone())?;
    }
    if getUnassigned(ass1.clone()).is_empty() {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("\nMatching succeeded!\n")).clone());
        }
        orderOut = orderOut.clone().reverse();
    } else {
        if Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
            println!("{}", (literal!("\nMatching failed, choose different tearing set!\n\n\n")).clone());
        }
        Error::addCompilerError((literal!("There is no possible matching for a user-defined tearing set.")).clone())?;
        bail!("fail");
    }
    Ok(orderOut)
}

