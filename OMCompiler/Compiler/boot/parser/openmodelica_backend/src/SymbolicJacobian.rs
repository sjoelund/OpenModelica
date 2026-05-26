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

use crate::BackendDAE;
use crate::BackendDAEOptimize;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Differentiate;
use crate::DynamicOptimization;
use crate::IndexReduction;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::ValuesUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::Graph;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// section for postOptModule >>symbolicJacobian<<
//
// Detects the sparse pattern of the ODE system and calculates also the symbolic
// Jacobian if flag "--generateDynamicJacobian=symbolic".
// =============================================================================
// From User Documentation for ida v5.4.0 equation (2.5) aka Alpha
// is the scalar in the system Jacobian, proportional to the inverse of the step
// size used for DAE_Mode symbolic jacobians
pub const DAE_CJ: &'static str = "$DAE_CJ";

pub fn symbolicJacobian(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?) {
        Deref @ "none" => inDAE.clone(),
        Deref @ "numeric" => detectSparsePatternODE(inDAE.clone())?,
        Deref @ "symbolic" => generateSymbolicJacobianPast(inDAE.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

// =============================================================================
// section for postOptModule >>calculateStateSetsJacobians<<
//
// =============================================================================
pub fn calculateStateSetsJacobians(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE.clone(), Arc::new(calculateEqSystemStateSetsJacobians))?;
    Ok(outDAE)
}

// =============================================================================
// section for postOptModule >>calculateStrongComponentJacobians<<
//
// Module for to calculate strong component Jacobian matrices
// =============================================================================
pub fn calculateStrongComponentJacobians(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Arc<BackendDAE::BackendDAE> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    match '__try0: {
        outDAE = unwrap_break_err!(BackendDAEUtil::mapEqSystem(inDAE.clone(), Arc::new(calculateEqSystemJacobians)), '__try0);
        Ok::<_, anyhow::Error>((outDAE.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outDAE = __try0_o0;
        }
        Err(_) => {
            outDAE = inDAE.clone();
        }
    }
    outDAE
}

// =============================================================================
// section for postOptModule >>constantLinearSystem<<
//
// constant Jacobian matrices. Linear system of equations (A x = b) where
// A and b are constant.
// =============================================================================
pub fn constantLinearSystem(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), Arc::new(constantLinearSystem0), (false, 1))?;
    Ok(outDAE)
}

// =============================================================================
// section for postOptModule >>detectSparsePatternODE<<
//
// Generate sparse pattern
// =============================================================================
fn detectSparsePatternODE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    let mut DAE: Arc<BackendDAE::BackendDAE>;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut coloredCols: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let mut sparsePattern: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32);
    let mut states: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut dummyVar: BackendDAE::Var;
    let mut v: BackendDAE::Variables;
    let debug: bool = false;
    match '__try0: {
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("detectSparsePatternODE -> start ")).clone()), '__try0);
        }
        let __pa1 = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
            Deref @ DAE { eqs: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        eqs = __pa1.clone();
        DAE = unwrap_break_err!(BackendDAEUtil::copyBackendDAE(inBackendDAE.clone()), '__try0);
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("detectSparsePatternODE -> copy dae ")).clone()), '__try0);
        }
        DAE = unwrap_break_err!(BackendDAEOptimize::collapseIndependentBlocks(DAE.clone()), '__try0);
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("detectSparsePatternODE -> collapse blocks ")).clone()), '__try0);
        }
        DAE = unwrap_break_err!(BackendDAEUtil::transformBackendDAE(DAE.clone(), Some((crate::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, crate::BackendDAE::EquationConstraints::EXACT)), None, None), '__try0);
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("detectSparsePatternODE -> transform backend dae ")).clone()), '__try0);
        }
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(DAE.clone()) {
            Deref @ DAE { shared: __pa2, eqs: metamodelica::List::Cons { head: BackendDAE::EqSystem { orderedVars: __pa3, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => (__pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        shared = __pa2.clone();
        v = __pa3.clone();
        states = unwrap_break_err!(BackendVariable::getAllStateVarFromVariables(v.clone()), '__try0);
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("detectSparsePatternODE -> get all vars ")).clone()), '__try0);
        }
        (sparsePattern, coloredCols) = unwrap_break_err!(generateSparsePattern(DAE.clone(), states.clone(), states.clone(), false), '__try0);
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("detectSparsePatternODE -> generateSparsePattern ")).clone()), '__try0);
        }
        shared = unwrap_break_err!(addBackendDAESharedJacobianSparsePattern(sparsePattern.clone(), coloredCols.clone(), BackendDAE::SymbolicJacobianAIndex.clone(), shared.clone()), '__try0);
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("detectSparsePatternODE -> addBackendDAESharedJacobianSparsePattern ")).clone()), '__try0);
        }
        outBackendDAE = unwrap_break_err!(BackendDAE::DAE(eqs.clone(), shared.clone()), '__try0);
        Ok::<_, anyhow::Error>((outBackendDAE.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outBackendDAE = __try0_o0;
        }
        Err(_) => {
            Error::addCompilerWarning((literal!("The optimization module detectJacobianSparsePattern failed. This module will be skipped and the transformation process continued.")).clone())?;
            outBackendDAE = inBackendDAE.clone();
        }
    }
    Ok(outBackendDAE)
}

// =============================================================================
// section for postOptModule >>symbolicJacobianDAE<<
//
// Generate symbolic jacobian for DAEMode
// =============================================================================
// =============================================================================
// section for postOptModule >>generateSymbolicJacobianPast<<
//
// Symbolic Jacobian subsection
// =============================================================================
// =============================================================================
// section for postOptModule >>generateSymbolicSensitivities<<
//
// That function generates symbolic sentivities for parameters
// by differentiatiating the states with respect to the parameters
// =============================================================================
// =============================================================================
// section for postOptModule >>generateSymbolicLinearizationPast<<
//
// =============================================================================
pub fn generateSymbolicLinearizationPast(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    outBackendDAE = 'mc: {
        let __mc_input = inBackendDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut linearModelMatrices: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>> = metamodelica::nil();
                    let mut funcs; // TODO: local with unresolved type
                    let mut functionTree; // TODO: local with unresolved type
                    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
                    let true = (Flags::getConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone())?) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
                        Deref @ DAE { shared: __pa0, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    shared = __pa0.clone();
                    eqs = __pa1.clone();
                    (linearModelMatrices, funcs) = createLinearModelMatrices(inBackendDAE.clone(), Config::acceptOptimicaGrammar()?)?;
                    shared = BackendDAEUtil::setSharedSymJacs(shared.clone(), linearModelMatrices.clone());
                    functionTree = BackendDAEUtil::getFunctions(shared.clone())?;
                    functionTree = AvlTreePathFunction::join(functionTree.clone(), funcs.clone())?;
                    shared = BackendDAEUtil::setSharedFunctionTree(shared.clone(), functionTree.clone());
                    outBackendDAE = BackendDAE::DAE(eqs.clone(), shared.clone())?;
                    Ok(outBackendDAE.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inBackendDAE.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBackendDAE)
}

// =============================================================================
// section for postOptModule >>inputDerivativesUsed<<
//
// check for derivatives of inputs
// =============================================================================
pub fn inputDerivativesUsed(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), Arc::new(inputDerivativesUsedWork), false)?;
    Ok(outDAE)
}

fn inputDerivativesUsedWork(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool = false;
    let mut hasFailed: bool = false;
    (osyst, outChanged) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut hasFailed: bool = hasFailed.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendDAEUtil::traverseBackendDAEExpsEqns(orderedEqs.clone(), Arc::new(traverserinputDerivativesUsed), (BackendVariable::daeGlobalKnownVars(inShared.clone()), metamodelica::nil()))?) {
                        (_, __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    explst = __pa0.clone();
                    s = stringDelimitList(List::map(explst.clone(), Arc::new(ExpressionBasics::printExpStr)), (literal!("\n")).clone());
                    Error::addMessage(Error::DERIVATIVE_INPUT.clone(), list![(s.clone()).clone()])?;
                    hasFailed = true;
                    Ok((BackendDAEUtil::setEqSystEqs(isyst.clone(), orderedEqs.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inChanged.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if hasFailed.clone() {
        bail!("fail");
    }
    Ok((osyst, outShared, outChanged))
}

fn traverserinputDerivativesUsed(mut inExp: Arc<DAE::Exp>, mut itpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>))> {
    let mut e: Arc<DAE::Exp>;
    let mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>);
    (e, tpl) = Expression::traverseExpTopDown(inExp.clone(), Arc::new(traverserExpinputDerivativesUsed), itpl.clone())?;
    Ok((e, tpl))
}

fn traverserExpinputDerivativesUsed(mut inExp: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), tpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, explst)) => {
                    let mut var: BackendDAE::Var;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    let true = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), false, (vars.clone(), cons(e.clone(), explst.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, explst)) => {
                    let mut var: BackendDAE::Var;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    let true = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), false, (vars.clone(), cons(e.clone(), explst.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

// =============================================================================
// solve linear systems with constant jacobian and variable b-Vector
//
// =============================================================================
fn jacobianIsConstant(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> bool {
    let mut isConst: bool = false;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    eqs = List::map(jac.clone(), Arc::new(fnptr!(Util::tuple33, _)));
    isConst = !(List::any(eqs.clone(), Arc::new(fnptr!(variableResidual, Arc<BackendDAE::Equation>))));
    isConst
}

fn variableResidual(mut eq: Arc<BackendDAE::Equation>) -> bool {
    let mut isNotConst: bool = false;
    isNotConst = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: Deref @ DAE::Exp::RCONST { real: _ }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNotConst
}

fn replaceStrongComponent(mut systIn: Arc<BackendDAE::EqSystem>, mut idx: i32, mut compsNew: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut compsAdd: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut systOut: Arc<BackendDAE::EqSystem> = systIn.clone();
    let mut orderedVars: BackendDAE::Variables;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut assAdd: metamodelica::Array<i32>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(systIn.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, ass2: __pa1, ass1: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    ass2 = __pa1.clone();
    ass1 = __pa2.clone();
    if !(compsAdd.clone().is_empty()) {
        assAdd = arrayCreate((compsAdd.clone().len() as i32), 0);
        ass1 = arrayAppend(ass1.clone(), assAdd.clone())?;
        ass2 = arrayAppend(ass2.clone(), assAdd.clone())?;
        List::map2_0(compsAdd.clone(), Arc::new(updateAssignment), ass1.clone(), ass2.clone());
    }
    List::map2_0(compsNew.clone(), Arc::new(updateAssignment), ass1.clone(), ass2.clone());
    comps = List::replaceAtWithList(compsNew.clone(), idx.clone() - 1, comps.clone())?;
    assign_field!(systOut.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: listAppend(comps.clone(), compsAdd.clone()) }));
    systOut = BackendDAEUtil::setEqSystMatrices(systOut.clone(), None, None, None)?;
    Ok(systOut)
}

fn updateAssignment(mut comp: Arc<BackendDAE::StrongComponent>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (comp.clone(), ass1.clone(), ass2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var, eqn: eq }, _, _) => {
                    {let _arr = ass2.clone(); _arr.borrow_mut()[(eq.clone()-1) as usize] = var.clone(); _arr};
                    {let _arr = ass1.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = eq.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn createSysEquations(mut A: metamodelica::Array<metamodelica::Array<metamodelica::Real>>, mut b: metamodelica::Array<metamodelica::Real>, mut n: i32, mut order: metamodelica::Array<i32>, mut xVars: Arc<metamodelica::List<BackendDAE::Var>>, mut bVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut sysEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut row: i32 = 0;
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut coeffExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut xExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut bExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut xProds: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut bProds: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut coeffs: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    xExps = List::map(xVars.clone(), Arc::new(BackendVariable::varExp2));
    bExps = List::map(bVars.clone(), Arc::new(BackendVariable::varExp2));
    for mut i in 1..=n.clone() {
        row = order.clone().borrow()[(i.clone()-1) as usize].clone();
        coeffs = Arc::new(A.borrow()[(row.clone()-1) as usize].clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
        coeffExps = List::map(coeffs.clone(), Arc::new(fnptr!(Expression::makeRealExp, metamodelica::Real)));
        xProds = List::threadMap1(coeffExps.clone(), xExps.clone(), Arc::new(fnptr!(makeBinaryExp, Arc<DAE::Exp>, Arc<DAE::Exp>, DAE::Operator)), DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() });
        lhs = List::fold1(xProds.clone(), Arc::new(fnptr!(Expression::makeBinaryExp, Arc<DAE::Exp>, DAE::Operator, Arc<DAE::Exp>)), DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT.clone() }, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
        (lhs, _) = ExpressionSimplify::simplify(lhs.clone())?;
        coeffs = Array::getRange((row.clone() - 1) * n.clone() + 1, row.clone() * n.clone(), b.clone())?;
        coeffExps = List::map(coeffs.clone(), Arc::new(fnptr!(Expression::makeRealExp, metamodelica::Real)));
        bProds = List::threadMap1(coeffExps.clone(), bExps.clone(), Arc::new(fnptr!(makeBinaryExp, Arc<DAE::Exp>, Arc<DAE::Exp>, DAE::Operator)), DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() });
        rhs = List::fold1(bProds.clone(), Arc::new(fnptr!(Expression::makeBinaryExp, Arc<DAE::Exp>, DAE::Operator, Arc<DAE::Exp>)), DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT.clone() }, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
        (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
        eq = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
        sysEqs = cons(eq.clone(), sysEqs.clone());
    }
    Ok(sysEqs)
}

pub fn makeBinaryExp(mut inLhs: Arc<DAE::Exp>, mut inRhs: Arc<DAE::Exp>, mut inOp: DAE::Operator) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::BINARY { exp1: inLhs.clone(), operator: inOp.clone(), exp2: inRhs.clone() });
    outExp
}

fn createBVecVars(mut sysIdx: i32, mut compIdx: i32, mut size: i32, mut typ: Arc<DAE::Type>, mut bExps: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut ident: ArcStr = arcstr::literal!("");
    let mut i: i32 = 0;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: BackendDAE::Var;
    let mut beq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut i in 1..=size.clone() {
        ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$sys")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!("_b")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone();
        cref = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), typ.clone(), metamodelica::nil());
        var = BackendVariable::makeVar(cref.clone());
        varLst = cons(var.clone(), varLst.clone());
        beq = Arc::new(BackendDAE::Equation::EQUATION { exp: (bExps.clone()).get(i.clone())?, scalar: Expression::crefExp(cref.clone())?, source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
        eqLst = cons(beq.clone(), eqLst.clone());
    }
    Ok((varLst, eqLst))
}

fn gauss(mut A: metamodelica::Array<metamodelica::Array<metamodelica::Real>>, mut b: metamodelica::Array<metamodelica::Real>, mut indxIn: i32, mut n: i32, mut rangeIn: Arc<metamodelica::List<i32>>, mut permutation: metamodelica::Array<i32>) -> Result<()> {
    let mut pivotIdx: i32 = 0;
    let mut pos: i32 = 0;
    let mut ir: i32 = 0;
    let mut ic: i32 = 0;
    let mut p_ir: i32 = 0;
    let mut pivot: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut entry: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut pr_entry: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut b_entry: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut first: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut range: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let _ = 'mc: {
        let __mc_input = (A.clone(), b.clone(), indxIn.clone(), n.clone(), rangeIn.clone(), permutation.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut range: Arc<metamodelica::List<i32>> = range.clone();
                    let mut pivotIdx: i32 = pivotIdx.clone();
                    let mut pivot: metamodelica::Real = pivot.clone();
                    let mut b_entry: metamodelica::Real = b_entry.clone();
                    let mut entry: metamodelica::Real = entry.clone();
                    let mut pos: i32 = pos.clone();
                    let mut first: metamodelica::Real = first.clone();
                    let true = (intLe(indxIn.clone(), n.clone())) else { bail!("pattern mismatch") };
                    (pivotIdx, pivot) = getPivotElement(A.clone(), rangeIn.clone(), indxIn.clone(), n.clone())?;
                    {let _arr = permutation.clone(); _arr.borrow_mut()[(indxIn.clone()-1) as usize] = pivotIdx.clone(); _arr};
                    (range, _) = List::deleteMemberOnTrue(pivotIdx.clone(), rangeIn.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
                    for mut ic in indxIn.clone()..=n.clone() {
                        entry = A.borrow()[(pivotIdx.clone()-1) as usize].clone().borrow()[(ic.clone()-1) as usize].clone();
                        entry = realDiv(entry.clone(), pivot.clone());
                        {let _arr = A.borrow()[(pivotIdx.clone()-1) as usize].clone(); _arr.borrow_mut()[(ic.clone()-1) as usize] = entry.clone(); _arr};
                    }
                    for mut ic in 1..=n.clone() {
                        pos = (pivotIdx.clone() - 1) * n.clone() + ic.clone();
                        b_entry = b.clone().borrow()[(pos.clone()-1) as usize].clone();
                        b_entry = realDiv(b_entry.clone(), pivot.clone());
                        {let _arr = b.clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = b_entry.clone(); _arr};
                    }
                    for mut ir in &*range.clone() {
                        let mut ir = ir.clone();
                        first = A.borrow()[(ir.clone()-1) as usize].clone().borrow()[(indxIn.clone()-1) as usize].clone();
                        for mut ic in indxIn.clone()..=n.clone() {
                            pos = (ir.clone() - 1) * n.clone() + ic.clone();
                            entry = A.borrow()[(ir.clone()-1) as usize].clone().borrow()[(ic.clone()-1) as usize].clone();
                            pivot = A.borrow()[(pivotIdx.clone()-1) as usize].clone().borrow()[(ic.clone()-1) as usize].clone();
                            entry = (entry.clone()) - ((first.clone()) * (pivot.clone()));
                            {let _arr = A.borrow()[(ir.clone()-1) as usize].clone(); _arr.borrow_mut()[(ic.clone()-1) as usize] = entry.clone(); _arr};
                            b_entry = b.clone().borrow()[(pos.clone()-1) as usize].clone();
                            pivot = b.clone().borrow()[((pivotIdx.clone() - 1) * n.clone() + ic.clone()-1) as usize].clone();
                            b_entry = b_entry.clone() - (first.clone()) * (pivot.clone());
                            {let _arr = b.clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = b_entry.clone(); _arr};
                        }
                    }
                    gauss(A.clone(), b.clone(), indxIn.clone() + 1, n.clone(), range.clone(), permutation.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getPivotElement(mut A: metamodelica::Array<metamodelica::Array<metamodelica::Real>>, mut rangeIn: Arc<metamodelica::List<i32>>, mut startIdx: i32, mut n: i32) -> Result<(i32, metamodelica::Real)> {
    let mut pos: i32 = 0;
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut i: i32 = 0;
    let mut entry: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    for mut i in &*rangeIn.clone() {
        let mut i = i.clone();
        entry = A.borrow()[(i.clone()-1) as usize].clone().borrow()[(startIdx.clone()-1) as usize].clone();
        if realAbs(entry.clone()) > value.clone() {
            value = entry.clone();
            pos = i.clone();
        }
    }
    Ok((pos, value))
}

fn rListStr(mut l: Arc<metamodelica::List<metamodelica::Real>>) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = stringDelimitList(List::map(l.clone(), Arc::new(fnptr!(realString, metamodelica::Real))), (literal!(" , ")).clone());
    s
}

// =============================================================================
// unsorted section
//
// =============================================================================
fn constantLinearSystem0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut iTpl: (bool, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut oTpl: (bool, i32);
    let mut changed: bool = false;
    let mut sysIdx: i32 = 0;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    (changed, sysIdx) = iTpl.clone();
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    (osyst, outShared, changed, sysIdx) = constantLinearSystem1(isyst.clone(), inShared.clone(), comps.clone(), changed.clone(), sysIdx.clone(), 1)?;
    osyst = constantLinearSystem2(changed.clone(), osyst.clone())?;
    oTpl = (changed.clone(), sysIdx.clone() + 1);
    Ok((osyst, outShared, oTpl))
}

fn constantLinearSystem2(mut b: bool, mut isyst: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &((b.clone(), isyst.clone())) {
        (false, _) => {
            isyst.clone()
        },
        (true, Deref @ BackendDAE::EqSystem { partitionKind, stateSets, orderedEqs: eqns, orderedVars: vars, .. }) => {
            let mut eqns = (*eqns).clone();
            let mut vars = (*vars).clone();
            vars = BackendVariable::listVar1(BackendVariable::varList(vars.clone())?);
            eqns = BackendEquation::listEquation(BackendEquation::equationList(eqns.clone()))?;
            BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), stateSets.clone(), partitionKind.clone(), BackendEquation::emptyEqns())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(osyst)
}

fn constantLinearSystem1(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inRunMatching: bool, mut sysIdxIn: i32, mut compIdxIn: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, i32)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut runMatching: bool = false;
    let mut sysIdxOut: i32 = 0;
    (osyst, oshared, runMatching, sysIdxOut) = (::match_deref::match_deref! { match &(inComps.clone()) {
        Deref @ metamodelica::List::Nil => {
            (isyst.clone(), ishared.clone(), inRunMatching.clone(), sysIdxIn.clone())
        },
        Deref @ metamodelica::List::Cons { head: comp, tail: comps } => {
            let mut b: bool = false;
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut sysIdx: i32 = 0;
            let mut compIdx: i32 = 0;
            (syst, shared, b, sysIdx, compIdx) = constantLinearSystemWork(isyst.clone(), ishared.clone(), comp.clone(), sysIdxIn.clone(), compIdxIn.clone())?;
            (syst, shared, runMatching, sysIdx) = constantLinearSystem1(syst.clone(), shared.clone(), comps.clone(), b.clone() || inRunMatching.clone(), sysIdx.clone(), compIdx.clone())?;
            (syst.clone(), shared.clone(), runMatching.clone(), sysIdx.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared, runMatching, sysIdxOut))
}

fn constantLinearSystemWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut comp: Arc<BackendDAE::StrongComponent>, mut sysIdxIn: i32, mut compIdxIn: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, i32, i32)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outRunMatching: bool = false;
    let mut sysIdxOut: i32 = 0;
    let mut compIdxOut: i32 = 0;
    (osyst, oshared, outRunMatching, sysIdxOut, compIdxOut) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone(), comp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_CONSTANT, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, vars: vindx, eqns: eindex, .. }) => {
                    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut syst = (*syst).clone();
                    let mut shared = (*shared).clone();
                    eqn_lst = BackendEquation::getList(eindex.clone(), syst.orderedEqs.clone());
                    var_lst = List::map1r(vindx.clone(), Arc::new(BackendVariable::getVarAt), syst.orderedVars.clone());
                    (syst, shared) = solveLinearSystem(syst.clone(), shared.clone(), eqn_lst.clone(), eindex.clone(), var_lst.clone(), vindx.clone(), jac.clone())?;
                    Ok((syst.clone(), shared.clone(), true, sysIdxIn.clone(), compIdxIn.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_LINEAR, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, vars: vindx, eqns: eindex, .. }) => {
                    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut sysIdx: i32 = 0;
                    let mut order: metamodelica::Array<i32>;
                    let mut bVarIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut bEqIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut bVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut bEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut sysEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut bComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
                    let mut sysComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
                    let mut syst = (*syst).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (BackendDAEUtil::isSimulationDAE(ishared.clone())) else { bail!("pattern mismatch") };
                    eqn_lst = BackendEquation::getList(eindex.clone(), eqns.clone());
                    var_lst = List::map1r(vindx.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
                    let true = (jacobianIsConstant(jac.clone())) else { bail!("pattern mismatch") };
                    let true = (Flags::isSet(Flags::CONSTJAC.clone())?) else { bail!("pattern mismatch") };
                    eqn_lst = BackendEquation::getList(eindex.clone(), eqns.clone());
                    var_lst = List::map1r(vindx.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
                    (sysEqs, bEqs, bVars, order, sysIdx) = solveConstJacLinearSystem(syst.clone(), shared.clone(), eqn_lst.clone(), eindex.clone(), var_lst.clone().reverse(), vindx.clone(), jac.clone(), sysIdxIn.clone(), compIdxIn.clone())?;
                    bVarIdcs = List::intRange2(BackendVariable::varsSize(vars.clone())? + 1, BackendVariable::varsSize(vars.clone())? + (bVars.clone().len() as i32));
                    bEqIdcs = List::intRange2(BackendEquation::getNumberOfEquations(eqns.clone()) + 1, BackendEquation::getNumberOfEquations(eqns.clone()) + (bEqs.clone().len() as i32));
                    bComps = List::threadMap(bEqIdcs.clone(), bVarIdcs.clone(), Arc::new(fnptr!(BackendDAEUtil::makeSingleEquationComp, i32, i32)));
                    sysComps = List::threadMap(List::map1(Arc::new(order.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), Arc::new(fnptr!(List::getIndexFirst, i32, _)), eindex.clone()), vindx.clone().reverse(), Arc::new(fnptr!(BackendDAEUtil::makeSingleEquationComp, i32, i32)));
                    todo!("unhandled field-assign shape: syst.orderedVars");
                    eqns = BackendEquation::addList(bEqs.clone(), eqns.clone())?;
                    todo!("unhandled field-assign shape: syst.orderedEqs");
                    syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None)?;
                    syst = replaceStrongComponent(syst.clone(), compIdxIn.clone(), sysComps.clone(), bComps.clone())?;
                    Ok((syst.clone(), ishared.clone(), false, sysIdx.clone(), compIdxIn.clone() + (sysComps.clone().len() as i32)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone(), false, sysIdxIn.clone(), compIdxIn.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outRunMatching, sysIdxOut, compIdxOut))
}

fn solveLinearSystem(mut inSyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut eqn_indxs: Arc<metamodelica::List<i32>>, mut var_lst: Arc<metamodelica::List<BackendDAE::Var>>, mut var_indxs: Arc<metamodelica::List<i32>>, mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    (osyst, oshared) = (::match_deref::match_deref! { match &((inSyst.clone(), ishared.clone())) {
        (syst @ Deref @ BackendDAE::EqSystem { .. }, Deref @ BackendDAE::Shared { functionTree: funcs, .. }) => {
            let mut v: BackendDAE::Variables;
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut eqns1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut beqs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut sources: Arc<metamodelica::List<Arc<DAE::ElementSource>>> = metamodelica::nil();
            let mut rhsVals: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut solvedVals: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut jacVals: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut linInfo: i32 = 0;
            let mut names: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared>;
            let mut syst = (*syst).clone();
            eqns1 = BackendEquation::listEquation(eqn_lst.clone())?;
            v = BackendVariable::listVar1(var_lst.clone());
            (beqs, sources) = BackendDAEUtil::getEqnSysRhs(eqns1.clone(), v.clone(), Some(funcs.clone()))?;
            beqs = beqs.clone().reverse();
            rhsVals = ValuesUtil::valueReals(List::map(beqs.clone(), Arc::new(Ceval::cevalSimple)))?;
            jacVals = evaluateConstantJacobian((var_lst.clone().len() as i32), jac.clone());
            (solvedVals, linInfo) = System::dgesv(jacVals.clone(), rhsVals.clone())?;
            names = List::map(var_lst.clone(), Arc::new(BackendVariable::varCref));
            checkLinearSystem(linInfo.clone(), names.clone(), jacVals.clone(), rhsVals.clone(), eqn_lst.clone())?;
            sources = List::map1(sources.clone(), Arc::new(ElementSource::addSymbolicTransformation), Arc::new(DAE::SymbolicOperation::LINEAR_SOLVED { vars: names.clone(), jac: jacVals.clone(), rhs: rhsVals.clone(), result: solvedVals.clone() }));
            (v, eqns, shared) = changeConstantLinearSystemVars(var_lst.clone(), solvedVals.clone(), sources.clone(), var_indxs.clone(), syst.orderedVars.clone(), syst.orderedEqs.clone(), ishared.clone())?;
            assign_field!(
                syst.orderedVars = v.clone(),
                syst.orderedEqs = List::fold(eqn_indxs.clone(), Arc::new(BackendEquation::delete), eqns.clone())
            );
            (BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None)?, shared.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((osyst, oshared))
}

fn changeConstantLinearSystemVars(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inSolvedVals: Arc<metamodelica::List<metamodelica::Real>>, mut inSources: Arc<metamodelica::List<Arc<DAE::ElementSource>>>, mut var_indxs: Arc<metamodelica::List<i32>>, mut inVars: BackendDAE::Variables, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ishared: Arc<BackendDAE::Shared>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>)> {
    let mut outVars: BackendDAE::Variables;
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut oshared: Arc<BackendDAE::Shared>;
    (outVars, oeqns, oshared) = (::match_deref::match_deref! { match &((inVarLst.clone(), inSolvedVals.clone(), inSources.clone(), var_indxs.clone(), inVars.clone(), ieqns.clone(), ishared.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, vars, eqns, _) => {
            (vars.clone(), eqns.clone(), ishared.clone())
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varType: tp, varKind: BackendDAE::VarKind::STATE { .. }, varName: cref, .. }, tail: varlst }, Deref @ metamodelica::List::Cons { head: r, tail: rlst }, Deref @ metamodelica::List::Cons { head: _, tail: slst }, Deref @ metamodelica::List::Cons { head: _, tail: vindxs }, vars, eqns, _) => {
            let mut vars2: BackendDAE::Variables;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut e: Arc<DAE::Exp>;
            let mut eqns = (*eqns).clone();
            e = Expression::makeCrefExp(cref.clone(), tp.clone())?;
            e = Expression::expDer(e.clone());
            eqns = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: Arc::new(DAE::Exp::RCONST { real: r.clone() }), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), eqns.clone())?;
            (vars2, eqns, shared) = changeConstantLinearSystemVars(varlst.clone(), rlst.clone(), slst.clone(), vindxs.clone(), vars.clone(), eqns.clone(), ishared.clone())?;
            (vars2.clone(), eqns.clone(), shared.clone())
        },
        (Deref @ metamodelica::List::Cons { head: v, tail: varlst }, Deref @ metamodelica::List::Cons { head: r, tail: rlst }, Deref @ metamodelica::List::Cons { head: _, tail: slst }, Deref @ metamodelica::List::Cons { head: indx, tail: vindxs }, vars, eqns, _) => {
            let mut v1: BackendDAE::Var;
            let mut vars1: BackendDAE::Variables;
            let mut vars2: BackendDAE::Variables;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut eqns = (*eqns).clone();
            v1 = BackendVariable::setBindExp(v.clone(), Some(Arc::new(DAE::Exp::RCONST { real: r.clone() })));
            v1 = BackendVariable::setVarStartValue(v1.clone(), Arc::new(DAE::Exp::RCONST { real: r.clone() }))?;
            (vars1, _) = BackendVariable::removeVar(indx.clone(), vars.clone())?;
            shared = BackendVariable::addGlobalKnownVarDAE(v1.clone(), ishared.clone())?;
            (vars2, eqns, shared) = changeConstantLinearSystemVars(varlst.clone(), rlst.clone(), slst.clone(), vindxs.clone(), vars1.clone(), eqns.clone(), shared.clone())?;
            (vars2.clone(), eqns.clone(), shared.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outVars, oeqns, oshared))
}

pub fn evaluateConstantJacobian(mut size: i32, mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> {
    let mut vals: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut valarr: metamodelica::Array<metamodelica::Array<metamodelica::Real>>;
    let mut tmp2: Arc<metamodelica::List<metamodelica::Array<metamodelica::Real>>> = metamodelica::nil();
    valarr = evaluateConstantJacobianArray(size.clone(), jac.clone());
    tmp2 = Arc::new(valarr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    vals = List::map(tmp2.clone(), Arc::new(arrayList.clone()));
    vals
}

fn evaluateConstantJacobianArray(mut size: i32, mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> metamodelica::Array<metamodelica::Array<metamodelica::Real>> {
    let mut valarr: metamodelica::Array<metamodelica::Array<metamodelica::Real>>;
    let mut tmp: metamodelica::Array<metamodelica::Real>;
    let mut tmp2: Arc<metamodelica::List<metamodelica::Array<metamodelica::Real>>> = metamodelica::nil();
    tmp = arrayCreate(size.clone(), metamodelica::OrderedFloat(0.0_f64));
    tmp2 = List::map(List::fill(tmp.clone(), size.clone()), Arc::new(arrayCopy.clone()));
    valarr = metamodelica::arrayFromVec(tmp2.clone().into_iter().cloned().collect());
    List::map1_0(jac.clone(), Arc::new(evaluateConstantJacobian2), valarr.clone());
    valarr
}

fn evaluateConstantJacobian2(mut jac: (i32, i32, Arc<BackendDAE::Equation>), mut vals: metamodelica::Array<metamodelica::Array<metamodelica::Real>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((jac.clone(), vals.clone())) {
        ((i1, i2, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp, .. }), _) => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let __pa0 = ::match_deref::match_deref! { match &(Ceval::cevalSimple(exp.clone())?) {
                Deref @ Values::Value::REAL { real: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            {let _arr = vals.clone().borrow()[(i1.clone()-1) as usize].clone(); _arr.borrow_mut()[(i2.clone()-1) as usize] = r.clone(); _arr};
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn checkLinearSystem(mut info: i32, mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut jac: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut rhs: Arc<metamodelica::List<metamodelica::Real>>, mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (info.clone(), vars.clone(), jac.clone(), rhs.clone(), eqnlst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (0, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut infoStr: ArcStr = arcstr::literal!("");
                    let mut syst: ArcStr = arcstr::literal!("");
                    let mut varnames: ArcStr = arcstr::literal!("");
                    let mut varname: ArcStr = arcstr::literal!("");
                    let mut rhsStr: ArcStr = arcstr::literal!("");
                    let mut jacStr: ArcStr = arcstr::literal!("");
                    let mut eqnstr: ArcStr = arcstr::literal!("");
                    let true = (info.clone() > 0) else { bail!("pattern mismatch") };
                    varname = (ComponentReferenceBasics::printComponentRefStr((vars.clone()).get(info.clone())?)?).clone();
                    infoStr = (intString(info.clone())).clone();
                    varnames = stringDelimitList(List::map(vars.clone(), Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!(" ;\n  ")).clone());
                    rhsStr = stringDelimitList(List::map(rhs.clone(), Arc::new(fnptr!(realString, metamodelica::Real))), (literal!(" ;\n  ")).clone());
                    jacStr = stringDelimitList(List::map1(List::mapList(jac.clone(), Arc::new(fnptr!(realString, metamodelica::Real))), Arc::new(fnptr!(stringDelimitList, Arc<metamodelica::List<ArcStr>>, ArcStr)), (literal!(" , ")).clone()), (literal!(" ;\n  ")).clone());
                    eqnstr = (BackendDump::dumpEqnsStr(eqnlst.clone())?).clone();
                    syst = stringAppendList(list![(literal!("\n")).clone(), (eqnstr.clone()).clone(), (literal!("\n[\n  ")).clone(), (jacStr.clone()).clone(), (literal!("\n]\n  *\n[\n  ")).clone(), (varnames.clone()).clone(), (literal!("\n]\n  =\n[\n  ")).clone(), (rhsStr.clone()).clone(), (literal!("\n]")).clone()]);
                    Error::addMessage(Error::LINEAR_SYSTEM_SINGULAR.clone(), list![(syst.clone()).clone(), (infoStr.clone()).clone(), (varname.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut syst: ArcStr = arcstr::literal!("");
                    let mut varnames: ArcStr = arcstr::literal!("");
                    let mut rhsStr: ArcStr = arcstr::literal!("");
                    let mut jacStr: ArcStr = arcstr::literal!("");
                    let mut eqnstr: ArcStr = arcstr::literal!("");
                    let true = (info.clone() < 0) else { bail!("pattern mismatch") };
                    varnames = stringDelimitList(List::map(vars.clone(), Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!(" ;\n  ")).clone());
                    rhsStr = stringDelimitList(List::map(rhs.clone(), Arc::new(fnptr!(realString, metamodelica::Real))), (literal!(" ; ")).clone());
                    jacStr = stringDelimitList(List::map1(List::mapList(jac.clone(), Arc::new(fnptr!(realString, metamodelica::Real))), Arc::new(fnptr!(stringDelimitList, Arc<metamodelica::List<ArcStr>>, ArcStr)), (literal!(" , ")).clone()), (literal!(" ; ")).clone());
                    eqnstr = (BackendDump::dumpEqnsStr(eqnlst.clone())?).clone();
                    syst = stringAppendList(list![(eqnstr.clone()).clone(), (literal!("\n[")).clone(), (jacStr.clone()).clone(), (literal!("] * [")).clone(), (varnames.clone()).clone(), (literal!("] = [")).clone(), (rhsStr.clone()).clone(), (literal!("]")).clone()]);
                    Error::addMessage(Error::LINEAR_SYSTEM_INVALID.clone(), list![(literal!("LAPACK/dgesv")).clone(), (syst.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn generateSparsePattern(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inIndependentVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inDependentVars: Arc<metamodelica::List<BackendDAE::Var>>, mut nonlinearPattern: bool) -> Result<((Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>)> {
    let mut outSparsePattern: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32);
    let mut outColoredCols: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let debug: bool = false;
    let mut patternName: ArcStr = if (nonlinearPattern.clone()) {literal!("Nonlinear")} else {literal!("Sparsity")};
    (outSparsePattern, outColoredCols) = 'mc: {
        let __mc_input = (inBackendDAE.clone(), inIndependentVars.clone(), inDependentVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil) => {
                    Ok(((metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE { eqs: metamodelica::List::Cons { head: syst @ BackendDAE::EqSystem { matching: bdaeMatching @ Deref @ BackendDAE::Matching::MATCHING { ass1, comps, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, independentVars, dependentVars) => {
                    let mut syst1: Arc<BackendDAE::EqSystem>;
                    let mut adjMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut adjMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut sizeN: i32 = 0;
                    let mut sizeM: i32 = 0;
                    let mut adjSize: i32 = 0;
                    let mut adjSizeT: i32 = 0;
                    let mut nonZeroElements: i32 = 0;
                    let mut nodesEqnsIndex: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut sparsepattern: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut sparsepatternT: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut jacDiffVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut varswithDiffs: BackendDAE::Variables;
                    let mut orderedEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut coloredArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut depCompRefsLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut inDepCompRefsLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut depCompRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
                    let mut inDepCompRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
                    let mut eqnSparse: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut varSparse: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut sparseArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut sparseArrayT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mark: metamodelica::Array<i32>;
                    let mut usedvar: metamodelica::Array<i32>;
                    let mut coloring: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
                    let mut translated: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
                    let mut sparsetuple: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
                    let mut sparsetupleT: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
                    let mut outSparsePattern: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32);
                    if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" start getting ")); __mm_s.push_str(&*patternName.clone()); __mm_s.push_str(&*literal!(" pattern for variables : ")); __mm_s.push_str(&*intString((dependentVars.clone().len() as i32))); __mm_s.push_str(&*literal!(" and the independent vars: ")); __mm_s.push_str(&*intString((independentVars.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if debug.clone() {
                        execStat((literal!("generateSparsePattern -> do start ")).clone())?;
                    }
                    depCompRefsLst = List::map(dependentVars.clone(), Arc::new(BackendVariable::varCref));
                    depCompRefs = metamodelica::arrayFromVec(depCompRefsLst.clone().into_iter().cloned().collect());
                    sizeM = (depCompRefs.clone().borrow().len() as i32);
                    (jacDiffVars, inDepCompRefsLst) = createInDepVars(independentVars.clone(), true)?;
                    inDepCompRefs = metamodelica::arrayFromVec(inDepCompRefsLst.clone().into_iter().cloned().collect());
                    sizeN = (inDepCompRefs.clone().borrow().len() as i32);
                    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEUtil::addVarsToEqSystem(syst.clone(), jacDiffVars.clone())?) {
                        __pa2 @ Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedEqns = __pa0.clone();
                    varswithDiffs = __pa1.clone();
                    syst1 = __pa2.clone();
                    (adjMatrix, adjMatrixT) = BackendDAEUtil::adjacencyMatrix(syst1.clone(), crate::BackendDAE::IndexType::SPARSE, None, BackendDAEUtil::isInitializationDAE(inBackendDAE.shared.clone()))?;
                    adjSize = (adjMatrix.clone().borrow().len() as i32);
                    adjSizeT = (adjMatrixT.clone().borrow().len() as i32);
                    if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                        BackendDump::printVarList(BackendVariable::varList(varswithDiffs.clone())?);
                        BackendDump::printEquationList(BackendEquation::equationList(orderedEqns.clone()));
                        BackendDump::dumpAdjacencyMatrix(adjMatrix.clone());
                        BackendDump::dumpAdjacencyMatrixT(adjMatrixT.clone());
                        BackendDump::dumpFullMatching(bdaeMatching.clone(), None)?;
                    }
                    nodesEqnsIndex = BackendVariable::getVarIndexFromVars(dependentVars.clone(), varswithDiffs.clone());
                    nodesEqnsIndex = List::map1(nodesEqnsIndex.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), ass1.clone());
                    if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                        println!("{}", (literal!("nodesEqnsIndexs: ")).clone());
                        BackendDump::dumpAdjacencyRow(nodesEqnsIndex.clone());
                        println!("{}", (literal!("\n")).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians[")); __mm_s.push_str(&*patternName.clone()); __mm_s.push_str(&*literal!("] -> build sparse graph: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    eqnSparse = arrayCreate(adjSize.clone(), metamodelica::nil());
                    varSparse = arrayCreate(adjSizeT.clone(), metamodelica::nil());
                    mark = arrayCreate(adjSizeT.clone(), 0);
                    usedvar = arrayCreate(adjSizeT.clone(), 0);
                    if sizeN.clone() > 0 {
                        usedvar = Array::setRange(adjSizeT.clone() - (sizeN.clone() - 1), adjSizeT.clone(), usedvar.clone(), 1)?;
                    }
                    if debug.clone() {
                        execStat((literal!("generateSparsePattern -> start ")).clone())?;
                    }
                    eqnSparse = getSparsePattern(comps.clone(), eqnSparse.clone(), varSparse.clone(), mark.clone(), usedvar.clone(), 1, adjMatrix.clone(), adjMatrixT.clone())?;
                    if debug.clone() {
                        execStat((literal!("generateSparsePattern -> end ")).clone())?;
                    }
                    if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                        BackendDump::dumpSparsePatternArray(eqnSparse.clone())?;
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians[")); __mm_s.push_str(&*patternName.clone()); __mm_s.push_str(&*literal!("] -> prepared arrayList for transpose list: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    sparseArray = Array::select(eqnSparse.clone(), nodesEqnsIndex.clone())?;
                    sparsepattern = Arc::new(sparseArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
                    sparsepattern = List::map1List(sparsepattern.clone(), Arc::new(fnptr!(intSub, i32, i32)), adjSizeT.clone() - sizeN.clone());
                    sparseArray = metamodelica::arrayFromVec(sparsepattern.clone().into_iter().cloned().collect());
                    if debug.clone() {
                        execStat((literal!("generateSparsePattern -> postProcess ")).clone())?;
                    }
                    sparseArrayT = arrayCreate(sizeN.clone(), metamodelica::nil());
                    sparseArrayT = transposeSparsePattern(sparsepattern.clone(), sparseArrayT.clone(), 1)?;
                    sparsepatternT = Arc::new(sparseArrayT.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
                    nonZeroElements = List::lengthListElements(sparsepattern.clone());
                    if debug.clone() {
                        execStat((literal!("generateSparsePattern -> transpose done ")).clone())?;
                    }
                    if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                        dumpSparsePatternStatistics(nonZeroElements.clone(), sparsepatternT.clone());
                        BackendDump::dumpSparsePattern(sparsepattern.clone())?;
                        BackendDump::dumpSparsePattern(sparsepatternT.clone())?;
                    }
                    if sparsepattern.clone().is_empty() {
                        sparsetuple = metamodelica::nil();
                        sparsetupleT = metamodelica::nil();
                    } else {
                        translated = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
        for mut lst in (sparsepattern.clone()).into_iter().cloned() {
                    let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut i in (lst.clone()).into_iter().cloned() {
                    let __x = inDepCompRefs.clone().borrow()[(i.clone()-1) as usize].clone();
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                        sparsetuple = {
        let mut __acc: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
        for (cr, t) in (depCompRefs.clone()).borrow().iter().cloned().zip((&(translated.clone())).into_iter()) {
                    let __x = (cr.clone(), t.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                        translated = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
        for mut lst in (sparsepatternT.clone()).into_iter().cloned() {
                    let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut i in (lst.clone()).into_iter().cloned() {
                    let __x = depCompRefs.clone().borrow()[(i.clone()-1) as usize].clone();
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                        sparsetupleT = {
        let mut __acc: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
        for (cr, t) in (inDepCompRefs.clone()).borrow().iter().cloned().zip((&(translated.clone())).into_iter()) {
                    let __x = (cr.clone(), t.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    }
                    if debug.clone() {
                        execStat((literal!("generateSparsePattern -> coloring start ")).clone())?;
                    }
                    if nonlinearPattern.clone() || Flags::isSet(Flags::DISABLE_COLORING.clone())? {
                        coloring = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
        for mut i in (1..=sizeN.clone()).into_iter() {
                    let __x = list![inDepCompRefs.clone().borrow()[(i.clone()-1) as usize].clone()];
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    } else {
                        coloredArray = createColoring(sparseArray.clone(), sparseArrayT.clone(), sizeN.clone(), sizeM.clone())?;
                        coloring = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
        for mut lst in (coloredArray.clone()).borrow().iter() {
                    let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut i in (lst.clone()).into_iter().cloned() {
                    let __x = inDepCompRefs.clone().borrow()[(i.clone()-1) as usize].clone();
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    }
                    if debug.clone() {
                        execStat((literal!("generateSparsePattern -> coloring done ")).clone())?;
                    }
                    if Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians[")); __mm_s.push_str(&*patternName.clone()); __mm_s.push_str(&*literal!("] -> ready! ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    outSparsePattern = (sparsetupleT.clone(), sparsetuple.clone(), (inDepCompRefsLst.clone(), depCompRefsLst.clone()), nonZeroElements.clone());
                    if Flags::isSet(Flags::DUMP_SPARSE.clone())? {
                        BackendDump::dumpSparsityPattern(outSparsePattern.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" --- ")); __mm_s.push_str(&*patternName.clone()); __mm_s.push_str(&*literal!(" Pattern ---")); ArcStr::from(__mm_s) }).clone())?;
                        BackendDump::dumpSparseColoring(coloring.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" --- ")); __mm_s.push_str(&*patternName.clone()); __mm_s.push_str(&*literal!(" Coloring ---")); ArcStr::from(__mm_s) }).clone());
                    }
                    if debug.clone() {
                        execStat((literal!("generateSparsePattern -> final end ")).clone())?;
                    }
                    Ok((outSparsePattern.clone(), coloring.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function generateSparsePattern failed")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSparsePattern, outColoredCols))
}

pub fn createColoring(mut sparseArray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sparseArrayT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sizeVars: i32, mut sizeVarswithDep: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut coloredArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let debug: bool = false;
    let mut nodesList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut colored: metamodelica::Array<i32>;
    let mut forbiddenColor: metamodelica::Array<i32>;
    let mut sparseGraph: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut sparseGraphT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut arraysparseGraph: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>;
    let mut maxColor: i32 = 0;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            println!("{}", (literal!("analytical Jacobians[SPARSE] -> build sparse graph.\n")).clone());
        }
        nodesList = List::intRange2(1, sizeVarswithDep.clone());
        sparseGraph = Graph::buildGraph(nodesList.clone(), Arc::new(createBipartiteGraph), sparseArray.clone());
        sparseGraphT = Graph::buildGraph(List::intRange2(1, sizeVars.clone()), Arc::new(createBipartiteGraph), sparseArrayT.clone());
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            println!("{}", (literal!("sparse graph: \n")).clone());
            Graph::printGraphInt(sparseGraph.clone());
            println!("{}", (literal!("transposed sparse graph: \n")).clone());
            Graph::printGraphInt(sparseGraphT.clone());
            println!("{}", (literal!("analytical Jacobians[SPARSE] -> builded graph for coloring.\n")).clone());
        }
        forbiddenColor = arrayCreate(sizeVars.clone(), 0);
        colored = arrayCreate(sizeVars.clone(), 0);
        arraysparseGraph = metamodelica::arrayFromVec(sparseGraph.clone().into_iter().cloned().collect());
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("generateSparsePattern -> coloring start ")).clone()), '__try0);
        }
        if sizeVars.clone() > 0 {
            unwrap_break_err!(Graph::partialDistance2colorInt(sparseGraphT.clone(), forbiddenColor.clone(), nodesList.clone(), arraysparseGraph.clone(), colored.clone()), '__try0);
        }
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("generateSparsePattern -> coloring end ")).clone()), '__try0);
        }
        GCExt::free(forbiddenColor.clone());
        GCExt::free(arraysparseGraph.clone());
        maxColor = Array::fold(colored.clone(), Arc::new(fnptr!(intMax, i32, i32)), 0);
        coloredArray = arrayCreate(maxColor.clone(), metamodelica::nil());
        unwrap_break_err!(mapIndexColors(colored.clone(), sizeVars.clone(), coloredArray.clone()), '__try0);
        GCExt::free(colored.clone());
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            println!("{}", (literal!("Print Coloring Cols: \n")).clone());
            unwrap_break_err!(BackendDump::dumpSparsePattern(Arc::new(coloredArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())), '__try0);
        }
        Ok::<_, anyhow::Error>((arraysparseGraph.clone(), colored.clone(), coloredArray.clone(), forbiddenColor.clone(), maxColor.clone(), nodesList.clone(), sparseGraph.clone(), sparseGraphT.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7)) => {
            arraysparseGraph = __try0_o0;
            colored = __try0_o1;
            coloredArray = __try0_o2;
            forbiddenColor = __try0_o3;
            maxColor = __try0_o4;
            nodesList = __try0_o5;
            sparseGraph = __try0_o6;
            sparseGraphT = __try0_o7;
        }
        Err(_) => {
            Error::addInternalError((literal!("function createColoring failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok(coloredArray)
}

fn dumpSparsePatternStatistics(mut nonZeroElements: i32, mut sparsepatternT: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> () {
    let mut maxDegree: i32 = 0;
    (_, maxDegree) = List::mapFold(sparsepatternT.clone(), Arc::new(fnptr!(findDegrees, _, i32)), 0);
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians[SPARSE] -> got sparse pattern nonZeroElements: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", nonZeroElements.clone()))); __mm_s.push_str(&*literal!(" maxNodeDegree: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", maxDegree.clone()))); __mm_s.push_str(&*literal!(" time : ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", clock()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    ()
}

fn findDegrees<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inValue: i32) -> (i32, i32) {
    let mut outDegree: i32 = 0;
    let mut outMaxDegree: i32 = 0;
    outDegree = (inList.clone().len() as i32);
    outMaxDegree = intMax(inValue.clone(), outDegree.clone());
    (outDegree, outMaxDegree)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getSparsePattern(mut inComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut ineqnSparse: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut invarSparse: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inMark: metamodelica::Array<i32>, mut inUsed: metamodelica::Array<i32>, mut inmarkValue: i32, mut inMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outSparsePattern: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    outSparsePattern = (::match_deref::match_deref! { match &((inComponents.clone(), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone(), inMatrix.clone(), inMatrixT.clone())) {
        (Deref @ metamodelica::List::Nil, result, _, _, _, _, _, _) => {
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var, eqn }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut result = (*result).clone();
            inputVars = inMatrix.clone().borrow()[(eqn.clone()-1) as usize].clone();
            inputVars = List::removeOnTrue(var.clone(), Arc::new(fnptr!(intEq, i32, i32)), inputVars.clone());
            getSparsePattern2(inputVars.clone(), list![var.clone()], list![eqn.clone()], ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: solvedVars, eqn }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut result = (*result).clone();
            inputVars = inMatrix.clone().borrow()[(eqn.clone()-1) as usize].clone();
            inputVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (inputVars.clone()).into_iter().cloned() {
            if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            getSparsePattern2(inputVars.clone(), solvedVars.clone(), list![eqn.clone()], ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: solvedVars, eqn }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut result = (*result).clone();
            inputVars = inMatrixT.clone().borrow()[(eqn.clone()-1) as usize].clone();
            inputVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (inputVars.clone()).into_iter().cloned() {
            if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            getSparsePattern2(inputVars.clone(), solvedVars.clone(), list![eqn.clone()], ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: solvedVars, eqn }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut result = (*result).clone();
            inputVars = inMatrix.clone().borrow()[(eqn.clone()-1) as usize].clone();
            inputVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (inputVars.clone()).into_iter().cloned() {
            if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            getSparsePattern2(inputVars.clone(), solvedVars.clone(), list![eqn.clone()], ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: solvedVars, eqn }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut result = (*result).clone();
            inputVars = inMatrix.clone().borrow()[(eqn.clone()-1) as usize].clone();
            inputVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (inputVars.clone()).into_iter().cloned() {
            if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            getSparsePattern2(inputVars.clone(), solvedVars.clone(), list![eqn.clone()], ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: solvedVars, eqn }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut result = (*result).clone();
            inputVars = inMatrix.clone().borrow()[(eqn.clone()-1) as usize].clone();
            inputVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (inputVars.clone()).into_iter().cloned() {
            if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            getSparsePattern2(inputVars.clone(), solvedVars.clone(), list![eqn.clone()], ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: solvedVars, eqn }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut result = (*result).clone();
            inputVars = inMatrix.clone().borrow()[(eqn.clone()-1) as usize].clone();
            inputVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (inputVars.clone()).into_iter().cloned() {
            if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            getSparsePattern2(inputVars.clone(), solvedVars.clone(), list![eqn.clone()], ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: solvedVars, eqns, .. }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut inputVarsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut result = (*result).clone();
            inputVarsLst = List::map1(eqns.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), inMatrix.clone());
            inputVars = List::flatten(inputVarsLst.clone());
            inputVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (inputVars.clone()).into_iter().cloned() {
            if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            getSparsePattern2(inputVars.clone(), solvedVars.clone(), eqns.clone(), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, tearingvars: vars, residualequations: eqns, .. }, .. }, tail: rest }, result, _, _, _, _, _, _) => {
            let mut vars1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut inputVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut inputVarsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut solvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut result = (*result).clone();
            (eqns1, inputVarsLst, _) = List::map_3(innerEquations.clone(), Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation));
            vars1 = List::flatten(inputVarsLst.clone());
            eqns1 = listAppend(eqns.clone(), eqns1.clone());
            solvedVars = listAppend(vars.clone(), vars1.clone());
            inputVarsLst = List::map1(eqns1.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), inMatrix.clone());
            inputVars = List::flatten(inputVarsLst.clone());
            inputVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (inputVars.clone()).into_iter().cloned() {
            if !(!(listMember(v.clone(), solvedVars.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            getSparsePattern2(inputVars.clone(), solvedVars.clone(), eqns1.clone(), ineqnSparse.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
            result = getSparsePattern(rest.clone(), result.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone() + 1, inMatrix.clone(), inMatrixT.clone())?;
            result.clone()
        },
        _ => {
            let mut comp: Arc<BackendDAE::StrongComponent>;
            let __pa0 = ::match_deref::match_deref! { match &(inComponents.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            comp = __pa0.clone();
            BackendDump::dumpComponent(comp.clone(), None)?;
            Error::addInternalError((literal!("function getSparsePattern failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSparsePattern)
}

fn getSparsePattern2(mut inInputVars: Arc<metamodelica::List<i32>>, mut inSolvedVars: Arc<metamodelica::List<i32>>, mut inEqns: Arc<metamodelica::List<i32>>, mut ineqnSparse: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut invarSparse: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inMark: metamodelica::Array<i32>, mut inUsed: metamodelica::Array<i32>, mut inmarkValue: i32) -> Result<()> {
    let mut localList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    localList = getSparsePatternHelp(inInputVars.clone(), invarSparse.clone(), inMark.clone(), inUsed.clone(), inmarkValue.clone())?;
    List::map2_0(inSolvedVars.clone(), Arc::new(Array::updateIndexFirst), localList.clone(), invarSparse.clone());
    List::map2_0(inEqns.clone(), Arc::new(Array::updateIndexFirst), localList.clone(), ineqnSparse.clone());
    Ok(())
}

fn getSparsePatternHelp(mut inInputVars: Arc<metamodelica::List<i32>>, mut invarSparse: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inMark: metamodelica::Array<i32>, mut inUsed: metamodelica::Array<i32>, mut inmarkValue: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outLocalList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut arrayElement: i32 = 0;
    let mut varSparse: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut var in &*inInputVars.clone() {
        let mut var = var.clone();
        arrayElement = inUsed.clone().borrow()[(var.clone()-1) as usize].clone();
        if intEq(1, arrayElement.clone()) {
            arrayElement = inMark.clone().borrow()[(var.clone()-1) as usize].clone();
            if !(intEq(inmarkValue.clone(), arrayElement.clone())) {
                {let _arr = inMark.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = inmarkValue.clone(); _arr};
                outLocalList = cons(var.clone(), outLocalList.clone());
            }
        }
        varSparse = invarSparse.clone().borrow()[(var.clone()-1) as usize].clone();
        for mut v in &*varSparse.clone() {
            let mut v = v.clone();
            arrayElement = inMark.clone().borrow()[(v.clone()-1) as usize].clone();
            if !(intEq(inmarkValue.clone(), arrayElement.clone())) {
                {let _arr = inMark.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = inmarkValue.clone(); _arr};
                outLocalList = cons(v.clone(), outLocalList.clone());
            }
        }
    }
    Ok(outLocalList)
}

pub fn transposeSparsePattern(mut inSparsePattern: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inAccumList: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inValue: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outSparsePattern: metamodelica::Array<Arc<metamodelica::List<i32>>> = inAccumList.clone();
    let mut value: i32 = inValue.clone();
    let mut tmplist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut oneList in &*inSparsePattern.clone() {
        let mut oneList = oneList.clone();
        for mut oneElem in &*oneList.clone() {
            let mut oneElem = oneElem.clone();
            tmplist = outSparsePattern.clone().borrow()[(oneElem.clone()-1) as usize].clone();
            metamodelica::Dangerous::arrayUpdate(outSparsePattern.clone(), oneElem.clone(), cons(value.clone(), tmplist.clone()))?;
        }
        value = value.clone() + 1;
    }
    Ok(outSparsePattern)
}

pub fn transposeSparsePatternTuple(mut inSparsePattern: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, mut inAccumList: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>) -> Result<metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>> {
    let mut outSparsePattern: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)> = inAccumList.clone();
    let mut value: i32 = 0;
    let mut tmplist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oneList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpTuple: (i32, Arc<metamodelica::List<i32>>);
    let mut i: i32 = 0;
    for mut oneListTuple in &*inSparsePattern.clone() {
        let mut oneListTuple = oneListTuple.clone();
        (value, oneList) = oneListTuple.clone();
        for mut oneElem in &*oneList.clone() {
            let mut oneElem = oneElem.clone();
            tmpTuple = outSparsePattern.clone().borrow()[(oneElem.clone() + 1-1) as usize].clone();
            (_, tmplist) = tmpTuple.clone();
            tmplist = cons(value.clone(), tmplist.clone());
            tmpTuple = (oneElem.clone(), tmplist.clone());
            metamodelica::Dangerous::arrayUpdate(outSparsePattern.clone(), oneElem.clone() + 1, tmpTuple.clone())?;
        }
    }
    for mut i in 1..=(inSparsePattern.clone().len() as i32) {
        tmpTuple = outSparsePattern.clone().borrow()[(i.clone()-1) as usize].clone();
        (value, tmplist) = tmpTuple.clone();
        tmplist = List::heapSortIntList(tmplist.clone());
        tmpTuple = (value.clone(), tmplist.clone());
        metamodelica::Dangerous::arrayUpdate(outSparsePattern.clone(), i.clone(), tmpTuple.clone())?;
    }
    Ok(outSparsePattern)
}

fn mapIndexColors(mut inColors: metamodelica::Array<i32>, mut inMaxIndex: i32, mut inArray: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut index: i32 = 0;
    if '__try0: {
        for mut i in 1..=inMaxIndex.clone() {
            index = inColors.clone().borrow()[(i.clone()-1) as usize].clone();
            {let _arr = inArray.clone(); let _val = cons(i.clone(), inArray.clone().borrow()[(index.clone()-1) as usize].clone()); _arr.borrow_mut()[(index.clone()-1) as usize] = _val; _arr};
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addInternalError((literal!("function mapIndexColors failed")).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok(())
}

fn createBipartiteGraph(mut inNode: i32, mut inSparsePattern: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outEdges: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if inNode.clone() >= 1 && inNode.clone() <= (inSparsePattern.clone().borrow().len() as i32) {
        outEdges = inSparsePattern.clone().borrow()[(inNode.clone()-1) as usize].clone();
    } else {
        outEdges = metamodelica::nil();
    }
    Ok(outEdges)
}

fn createInDepVars(mut independentVars: Arc<metamodelica::List<BackendDAE::Var>>, mut createpDerStates: bool) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut var: BackendDAE::Var;
    for mut v in &*independentVars.clone() {
        let mut v = v.clone();
        if BackendVariable::isClockedStateVar(v.clone()) {
            var = BackendVariable::createClockedState(v.clone())?;
            outVars = cons(var.clone(), outVars.clone());
            outCrefs = cons(var.varName.clone(), outCrefs.clone());
        } else if createpDerStates.clone() {
            outVars = cons(BackendVariable::createpDerVar(v.clone())?, outVars.clone());
            outCrefs = cons(v.varName.clone(), outCrefs.clone());
        } else {
            outVars = cons(v.clone(), outVars.clone());
            outCrefs = cons(v.varName.clone(), outCrefs.clone());
        }
    }
    outVars = outVars.clone().reverse();
    outCrefs = outCrefs.clone().reverse();
    Ok((outVars, outCrefs))
}

fn optimizeJacobianMatrix(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inComRef1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inComRef2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outJacobian: Arc<BackendDAE::BackendDAE>;
    let mut ea: metamodelica::Array<i32> = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut eMatching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::MATCHING { ass1: ea.clone(), ass2: ea.clone(), comps: metamodelica::nil() });
    outJacobian = 'mc: {
        let __mc_input = (inBackendDAE.clone(), inComRef1.clone(), inComRef2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: shared, .. }, Deref @ metamodelica::List::Nil, _) => {
                    let mut syst = (*syst).clone();
                    todo!("unhandled field-assign shape: syst.orderedVars");
                    todo!("unhandled field-assign shape: syst.matching");
                    Ok(BackendDAE::DAE(cons(syst.clone(), metamodelica::nil()), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: shared, .. }, _, Deref @ metamodelica::List::Nil) => {
                    let mut syst = (*syst).clone();
                    todo!("unhandled field-assign shape: syst.orderedVars");
                    todo!("unhandled field-assign shape: syst.matching");
                    Ok(BackendDAE::DAE(cons(syst.clone(), metamodelica::nil()), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (backendDAE, _, _) => {
                    let mut backendDAE2: Arc<BackendDAE::BackendDAE>;
                    let mut b: bool = false;
                    let mut strPostOptModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::JAC_DUMP2.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analytical Jacobians -> optimize jacobians time: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if Flags::isSet(Flags::JAC_DUMP.clone())? {
                        BackendDump::bltdump((literal!("Symbolic Jacobian")).clone(), backendDAE.clone())?;
                    } else {
                        b = FlagsUtil::disableDebug(Flags::EXEC_STAT.clone())?;
                    }
                    strPostOptModules = list![(literal!("wrapFunctionCalls")).clone(), (literal!("inlineArrayEqn")).clone(), (literal!("constantLinearSystem")).clone(), (literal!("solveSimpleEquations")).clone(), (literal!("tearingSystem")).clone(), (literal!("calculateStrongComponentJacobians")).clone(), (literal!("removeConstants")).clone(), (literal!("simplifyTimeIndepFuncCalls")).clone()];
                    if Flags::isSet(Flags::SPLIT_CONSTANT_PARTS_SYMJAC.clone())? {
                        strPostOptModules = List::insert(strPostOptModules.clone(), 4, (literal!("removeSimpleEquations")).clone())?;
                    }
                    backendDAE2 = BackendDAEUtil::getSolvedSystemforJacobians(backendDAE.clone(), list![(literal!("removeEqualRHS")).clone(), (literal!("removeSimpleEquations")).clone(), (literal!("evalFunc")).clone()], None, None, strPostOptModules.clone())?;
                    if Flags::isSet(Flags::JAC_DUMP.clone())? {
                        BackendDump::bltdump((literal!("Symbolic Jacobian")).clone(), backendDAE2.clone())?;
                    } else {
                        let _ = FlagsUtil::set(Flags::EXEC_STAT.clone(), b.clone())?;
                    }
                    Ok(backendDAE2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function optimizeJacobianMatrix failed")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outJacobian)
}

pub fn createSeedVars(mut indiffVar: Arc<DAE::ComponentRef>, mut inMatrixName: ArcStr) -> Result<BackendDAE::Var> {
    let mut outSeedVar: BackendDAE::Var;
    let mut derivedCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    derivedCref = Differentiate::createSeedCrefName(indiffVar.clone(), (inMatrixName.clone()).clone())?;
    outSeedVar = BackendDAE::Var { varName: derivedCref.clone(), varKind: crate::BackendDAE::VarKind::STATE_DER, varDirection: openmodelica_frontend_types::DAE::VarDirection::INPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ComponentReference::crefLastType(derivedCref.clone())?, bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource.clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
    Ok(outSeedVar)
}

fn createAllDiffedVars(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCref: Arc<DAE::ComponentRef>, mut inAllVars: BackendDAE::Variables, mut inMatrixName: ArcStr) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    if let Ok(__iflet0) = createAllDiffedVarsWork(inVars.clone(), inCref.clone(), inAllVars.clone(), 0, (inMatrixName.clone()).clone(), metamodelica::nil()) {
        outVars = __iflet0;
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("SymbolicJacobian.createAllDiffedVars failed")).clone()])?;
        bail!("fail");
    }
    Ok(outVars)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn createAllDiffedVarsWork(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCref: Arc<DAE::ComponentRef>, mut inAllVars: BackendDAE::Variables, mut inIndex: i32, mut inMatrixName: ArcStr, mut iVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVars = (::match_deref::match_deref! { match &((inVars.clone(), inCref.clone(), inAllVars.clone(), inIndex.clone(), inMatrixName.clone(), iVars.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            iVars.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: v @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, varName: currVar, .. }, tail: restVar }, cref, _, index, _, _) => {
            let mut r1: BackendDAE::Var;
            let mut derivedCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut currVar = (*currVar).clone();
            let mut index = (*index).clone();
            match '__try0: {
                (_, _) = unwrap_break_err!(BackendVariable::getVarSingle(currVar.clone(), inAllVars.clone()), '__try0);
                currVar = ComponentReference::crefPrefixDer(currVar.clone());
                derivedCref = unwrap_break_err!(ComponentReference::createDifferentiatedCrefName(currVar.clone(), cref.clone(), (inMatrixName.clone()).clone()), '__try0);
                r1 = BackendVariable::copyVarNewName(derivedCref.clone(), v.clone());
                r1 = unwrap_break_err!(BackendVariable::setVarKind(r1.clone(), crate::BackendDAE::VarKind::STATE_DER), '__try0);
                r1.unreplaceable = true;
                index = index.clone() + 1;
                Ok::<_, anyhow::Error>((currVar.clone(), derivedCref.clone(), r1.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                    currVar = __try0_o0;
                    derivedCref = __try0_o1;
                    r1 = __try0_o2;
                }
                Err(_) => {
                    currVar = ComponentReference::crefPrefixDer(currVar.clone());
                    derivedCref = ComponentReference::createDifferentiatedCrefName(currVar.clone(), cref.clone(), (inMatrixName.clone()).clone())?;
                    r1 = BackendVariable::copyVarNewName(derivedCref.clone(), v.clone());
                    r1 = BackendVariable::setVarKind(r1.clone(), crate::BackendDAE::VarKind::STATE_DER)?;
                }
            }
            createAllDiffedVarsWork(restVar.clone(), cref.clone(), inAllVars.clone(), index.clone(), (inMatrixName.clone()).clone(), cons(r1.clone(), iVars.clone()))?
        },
        (Deref @ metamodelica::List::Cons { head: v @ BackendDAE::Var { varName: currVar, .. }, tail: restVar }, cref, _, index, _, _) => {
            let mut r1: BackendDAE::Var;
            let mut derivedCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut index = (*index).clone();
            match '__try0: {
                (_, _) = unwrap_break_err!(BackendVariable::getVarSingle(currVar.clone(), inAllVars.clone()), '__try0);
                derivedCref = unwrap_break_err!(ComponentReference::createDifferentiatedCrefName(currVar.clone(), cref.clone(), (inMatrixName.clone()).clone()), '__try0);
                r1 = BackendVariable::copyVarNewName(derivedCref.clone(), v.clone());
                r1 = unwrap_break_err!(BackendVariable::setVarKind(r1.clone(), crate::BackendDAE::VarKind::VARIABLE), '__try0);
                r1.unreplaceable = true;
                index = index.clone() + 1;
                Ok::<_, anyhow::Error>((derivedCref.clone(), r1.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    derivedCref = __try0_o0;
                    r1 = __try0_o1;
                }
                Err(_) => {
                    derivedCref = ComponentReference::createDifferentiatedCrefName(currVar.clone(), cref.clone(), (inMatrixName.clone()).clone())?;
                    r1 = BackendVariable::copyVarNewName(derivedCref.clone(), v.clone());
                    r1 = BackendVariable::setVarKind(r1.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                }
            }
            createAllDiffedVarsWork(restVar.clone(), cref.clone(), inAllVars.clone(), index.clone(), (inMatrixName.clone()).clone(), cons(r1.clone(), iVars.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVars)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getJacobianMatrixbyName(mut injacobianMatrices: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>, mut inJacobianName: ArcStr) -> Option<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))> {
    let mut outMatrix: Option<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))> = None;
    outMatrix = (::match_deref::match_deref! { match &(injacobianMatrices.clone()) {
        Deref @ metamodelica::List::Cons { head: matrix @ (Some((_, name, _, _, _, _)), _, _, _), tail: _ } if (stringEq((name.clone()).clone(), (inJacobianName.clone()).clone())) => {
            Some(matrix.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            getJacobianMatrixbyName(rest.clone(), (inJacobianName.clone()).clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatrix
}

pub fn updateJacobianDependencies(mut jacobian: Arc<BackendDAE::Jacobian>) -> Result<Arc<BackendDAE::Jacobian>> {
    let mut jacobian: Arc<BackendDAE::Jacobian> = jacobian;
    jacobian = (::match_deref::match_deref! { match &(jacobian.clone()) {
        jac @ Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { .. } => {
            let mut symJac: (Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut name: ArcStr = arcstr::literal!("");
            let mut diffVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut diffedVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut allDiffedVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut dependencies: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut jac = (*jac).clone();
            let (__pa7, __pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(var_field!((*jac).jacobian, BackendDAE::Jacobian::GENERIC_JACOBIAN).clone()) {
                Some(__pa7 @ (DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: __pa1, .. }, __pa2, __pa3, __pa4, __pa5, __pa6)) => (__pa7.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            syst = __pa0.clone();
            shared = __pa1.clone();
            name = __pa2.clone();
            diffVars = __pa3.clone();
            diffedVars = __pa4.clone();
            allDiffedVars = __pa5.clone();
            dependencies = __pa6.clone();
            symJac = __pa7.clone();
            dependencies = calcJacobianDependencies(symJac.clone())?;
            assign_variant_field!(jac => BackendDAE::Jacobian::GENERIC_JACOBIAN; jacobian = Some((BackendDAE::DAE(list![syst.clone()], shared.clone())?, name.clone(), diffVars.clone(), diffedVars.clone(), allDiffedVars.clone(), dependencies.clone())));
            jac.clone()
        },
        _ => {
            jacobian.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(jacobian)
}

pub fn calcJacobianDependencies(mut jacobian: (Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut dependencies: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut systems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(jacobian.clone()) {
        (Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. }, _, _, _, _, _) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systems = __pa0.clone();
    shared = __pa1.clone();
    syst = listHead(systems.clone())?;
    dependencies = BackendEquation::getCrefsFromEquations(syst.orderedEqs.clone(), syst.orderedVars.clone(), shared.globalKnownVars.clone())?;
    Ok(dependencies)
}

pub fn getJacobianDependencies(mut jacobian: Arc<BackendDAE::Jacobian>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut dependencies: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    dependencies = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, _, _, _, _, dependencies)), .. } => dependencies.clone(),
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: None, .. } => metamodelica::nil(),
        _ => {
            Error::addInternalError((literal!("function getJacobianDependencies failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dependencies)
}

// =============================================================================
// Module for to calculate strong component Jacobains
//
// =============================================================================
fn calculateEqSystemJacobians(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut outSyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared>;
    (outSyst, outShared) = (::match_deref::match_deref! { match &((inSyst.clone(), inShared.clone())) {
        (syst @ Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1, ass2, comps }, orderedEqs: eqns, orderedVars: vars, .. }, shared) => {
            let mut syst = (*syst).clone();
            let mut comps = (*comps).clone();
            let mut shared = (*shared).clone();
            (comps, shared) = calculateJacobiansComponents(comps.clone(), vars.clone(), eqns.clone(), shared.clone())?;
            assign_field!(syst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() }));
            (syst.clone(), shared.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSyst, outShared))
}

fn calculateJacobiansComponents(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>)> {
    let mut outComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    outComps = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
        for mut component in (inComps.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(component.clone()) {
        comp => {
            let mut comp = (*comp).clone();
            (comp, outShared) = calculateJacobianComponent(comp.clone(), inVars.clone(), inEqns.clone(), outShared.clone())?;
            comp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outComps, outShared))
}

fn checkForSymbolicJacobian(mut inResidualEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inOtherEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut name: ArcStr) -> Result<bool> {
    let mut out: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;
    if !(Flags::isSet(Flags::FORCE_NLS_ANALYTIC_JACOBIAN.clone())?) {
        match '__try0: {
            (b1, _) = unwrap_break_err!(BackendEquation::traverseExpsOfEquationList_WithStop(inResidualEqns.clone(), Arc::new(traverserhasEqnNonDiffParts), (metamodelica::nil(), true, false)), '__try0);
            (b2, _) = unwrap_break_err!(BackendEquation::traverseExpsOfEquationList_WithStop(inOtherEqns.clone(), Arc::new(traverserhasEqnNonDiffParts), (metamodelica::nil(), true, false)), '__try0);
            if !(b1.clone() && b2.clone()) {
                if unwrap_break_err!(Flags::isSet(Flags::FAILTRACE.clone()), '__try0) {
                    unwrap_break_err!(Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Skip symbolic jacobian for non-linear system ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()), '__try0);
                }
                out = false;
            } else {
                out = true;
            }
            Ok::<_, anyhow::Error>((out.clone(),))
        } {
            Ok((__try0_o0,)) => {
                out = __try0_o0;
            }
            Err(_) => {
                out = false;
            }
        }
    } else {
        out = true;
    }
    Ok(out)
}

fn calculateTearingSetJacobian(mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inTearingSet: BackendDAE::TearingSet, mut inShared: Arc<BackendDAE::Shared>, mut isLinear: bool) -> Result<(Arc<BackendDAE::Jacobian>, Arc<BackendDAE::Shared>)> {
    let mut outJacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut prename: ArcStr = arcstr::literal!("");
    let mut debug: bool = false;
    let mut onlySparsePattern: bool = false;
    let mut diffVars: BackendDAE::Variables;
    let mut oVars: BackendDAE::Variables;
    let mut resVars: BackendDAE::Variables;
    let mut resEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut oEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    match '__try0: {
        if !(isLinear.clone()) && !(unwrap_break_err!(Flags::isSet(Flags::NLS_ANALYTIC_JACOBIAN.clone()), '__try0)) {
            onlySparsePattern = true;
        }
        if isLinear.clone() {
            prename = (literal!("LS")).clone();
        } else {
            prename = (literal!("NLS")).clone();
        }
        name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prename.clone()); __mm_s.push_str(&*literal!("Jac")); __mm_s.push_str(&*intString(System::tmpTickIndex(Global::backendDAE_jacobianSeq.clone()))); ArcStr::from(__mm_s) }).clone();
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** ")); __mm_s.push_str(&*prename.clone()); __mm_s.push_str(&*literal!("-JAC *** start creating Jacobian for a torn system ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" of size ")); __mm_s.push_str(&*intString((inTearingSet.tearingvars.clone().len() as i32))); __mm_s.push_str(&*literal!(" time: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        (diffVars, resVars, oVars, resEqns, oEqns) = unwrap_break_err!(prepareTornStrongComponentData(inVars.clone(), inEqns.clone(), inTearingSet.tearingvars.clone(), inTearingSet.residualequations.clone(), inTearingSet.innerEquations.clone(), inShared.functionTree.clone(), (name.clone()).clone()), '__try0);
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** ")); __mm_s.push_str(&*prename.clone()); __mm_s.push_str(&*literal!("-JAC *** prepared all data for differentiation at time: ")); __mm_s.push_str(&*realString(clock())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        if !(isLinear.clone() || unwrap_break_err!(checkForSymbolicJacobian(BackendEquation::equationList(resEqns.clone()), BackendEquation::equationList(oEqns.clone()), (name.clone()).clone()), '__try0)) {
            onlySparsePattern = true;
        }
        (outJacobian, outShared) = unwrap_break_err!(getSymbolicJacobian(diffVars.clone(), resEqns.clone(), resVars.clone(), oEqns.clone(), oVars.clone(), inShared.clone(), inVars.clone(), (name.clone()).clone(), onlySparsePattern.clone()), '__try0);
        Ok::<_, anyhow::Error>((diffVars.clone(), name.clone(), oEqns.clone(), oVars.clone(), outJacobian.clone(), outShared.clone(), prename.clone(), resEqns.clone(), resVars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
            diffVars = __try0_o0;
            name = __try0_o1;
            oEqns = __try0_o2;
            oVars = __try0_o3;
            outJacobian = __try0_o4;
            outShared = __try0_o5;
            prename = __try0_o6;
            resEqns = __try0_o7;
            resVars = __try0_o8;
        }
        Err(_) => {
            bail!("fail");
        }
    }
    Ok((outJacobian, outShared))
}

fn calculateJacobianComponent(mut inComp: Arc<BackendDAE::StrongComponent>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::Shared>)> {
    let mut outComp: Arc<BackendDAE::StrongComponent>;
    let mut outShared: Arc<BackendDAE::Shared>;
    (outComp, outShared) = 'mc: {
        let __mc_input = (inComp.clone(), inVars.clone(), inEqns.clone(), inShared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: strictTearingset, casualTearingSet: optCasualTearingSet, linear, mixedSystem }, _, _, _) => {
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut jacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
                    let mut jacobianCausal: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
                    let mut debug: bool = false;
                    let mut onlySparsePattern: bool = true;
                    let mut casualTearingSet: BackendDAE::TearingSet;
                    let mut strictTearingset = (*strictTearingset).clone();
                    let mut optCasualTearingSet = (*optCasualTearingSet).clone();
                    (jacobian, shared) = calculateTearingSetJacobian(inVars.clone(), inEqns.clone(), strictTearingset.clone(), inShared.clone(), linear.clone())?;
                    todo!("unhandled field-assign shape: strictTearingset.jac");
                    if isSome(optCasualTearingSet.clone()) {
                        casualTearingSet = Util::getOption(optCasualTearingSet.clone())?;
                        (jacobianCausal, shared) = calculateTearingSetJacobian(inVars.clone(), inEqns.clone(), casualTearingSet.clone(), shared.clone(), linear.clone())?;
                        casualTearingSet.jac = jacobianCausal.clone();
                        optCasualTearingSet = Some(casualTearingSet.clone());
                    }
                    Ok((Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: strictTearingset.clone(), casualTearingSet: optCasualTearingSet.clone(), linear: linear.clone(), mixedSystem: mixedSystem.clone() }), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_CONSTANT, .. }, _, _, _) => {
                    let mut debug: bool = false;
                    let mut onlySparsePattern: bool = true;
                    Ok((comp.clone(), inShared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { mixedSystem, vars: iterationvarsInts, eqns: residualequations, jacType: BackendDAE::JacobianType::JAC_LINEAR, .. }, _, _, _) => {
                    if !((Flags::isSet(Flags::LS_ANALYTIC_JACOBIAN.clone())?)) { bail!("guard") }
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut jacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
                    let mut debug: bool = false;
                    let mut onlySparsePattern: bool = true;
                    let mut strictTearingset: BackendDAE::TearingSet;
                    strictTearingset = BackendDAE::TearingSet { tearingvars: iterationvarsInts.clone(), residualequations: residualequations.clone(), innerEquations: metamodelica::nil(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) };
                    (jacobian, shared) = calculateTearingSetJacobian(inVars.clone(), inEqns.clone(), strictTearingset.clone(), inShared.clone(), true)?;
                    strictTearingset.jac = jacobian.clone();
                    Ok((Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: strictTearingset.clone(), casualTearingSet: None, linear: true, mixedSystem: mixedSystem.clone() }), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_LINEAR, .. }, _, _, _) => {
                    let mut debug: bool = false;
                    let mut onlySparsePattern: bool = true;
                    Ok((comp.clone(), inShared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { mixedSystem, vars: iterationvarsInts, eqns: residualequations, .. }, _, _, _) => {
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut iterationvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut resVarsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut diffVars: BackendDAE::Variables;
                    let mut ovars: BackendDAE::Variables;
                    let mut resVars: BackendDAE::Variables;
                    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut jacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut debug: bool = false;
                    let mut onlySparsePattern: bool = true;
                    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NLSJac")); __mm_s.push_str(&*intString(System::tmpTickIndex(Global::backendDAE_jacobianSeq.clone()))); ArcStr::from(__mm_s) }).clone();
                    iterationvars = List::map1r(iterationvarsInts.clone(), Arc::new(BackendVariable::getVarAt), inVars.clone());
                    iterationvars = List::map(iterationvars.clone(), Arc::new(fnptr!(BackendVariable::transformXToXd, BackendDAE::Var)));
                    iterationvars = iterationvars.clone().reverse();
                    diffVars = BackendVariable::listVar1(iterationvars.clone());
                    reqns = BackendEquation::getList(residualequations.clone(), inEqns.clone());
                    reqns = BackendEquation::replaceDerOpInEquationList(reqns.clone())?;
                    if checkForSymbolicJacobian(reqns.clone(), metamodelica::nil(), (name.clone()).clone())? && Flags::isSet(Flags::NLS_ANALYTIC_JACOBIAN.clone())? {
                        onlySparsePattern = false;
                    }
                    eqns = BackendEquation::listEquation(reqns.clone())?;
                    (_, reqns) = BackendEquation::traverseEquationArray(eqns.clone(), Arc::new(BackendEquation::traverseEquationToScalarResidualForm.clone()), (inShared.functionTree.clone(), metamodelica::nil()))?;
                    reqns = reqns.clone().reverse();
                    (reqns, resVarsLst, _) = BackendEquation::convertResidualsIntoSolvedEquations(reqns.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$res_")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); ArcStr::from(__mm_s) }).clone(), 1, false)?;
                    resVars = BackendVariable::listVar1(resVarsLst.clone());
                    eqns = BackendEquation::listEquation(reqns.clone())?;
                    oeqns = BackendEquation::listEquation(metamodelica::nil())?;
                    ovars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
                    (jacobian, shared) = getSymbolicJacobian(diffVars.clone(), eqns.clone(), resVars.clone(), oeqns.clone(), ovars.clone(), inShared.clone(), inVars.clone(), (name.clone()).clone(), onlySparsePattern.clone())?;
                    Ok((Arc::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: residualequations.clone(), vars: iterationvarsInts.clone(), jac: jacobian.clone(), jacType: crate::BackendDAE::JacobianType::JAC_GENERIC, mixedSystem: mixedSystem.clone() }), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comp, _, _, _) => {
                    let mut debug: bool = false;
                    let mut onlySparsePattern: bool = true;
                    Ok((comp.clone(), inShared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if BackendDAEUtil::isInitializationDAE(inShared.clone()) {
        if '__try0: {
            unwrap_break_err!(checkNonLinDependecies(outComp.clone(), inEqns.clone()), '__try0);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            Error::addInternalError((literal!("function calculateJacobianComponent failed to check all non-linear iteration variables for start values.")).clone(), metamodelica::sourceInfo!())?;
        }
    }
    Ok((outComp, outShared))
}

fn checkNonLinDependecies(mut inComp: Arc<BackendDAE::StrongComponent>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<()> {
    let mut name: ArcStr = arcstr::literal!("");
    let mut msg: ArcStr = arcstr::literal!("");
    let mut existNonLin: bool = false;
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        let _ = (::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: resIndices, jac, .. }, .. } => {
            let mut eqnIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
            for mut eq in &*innerEquations.clone() {
                let mut eq = eq.clone();
                eqnIndices = (match eq.clone() {
        BackendDAE::InnerEquation::INNEREQUATION { eqn: mut idx, .. } => {
            cons(idx.clone(), eqnIndices.clone())
        },
        BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS { eqn: mut idx, .. } => {
            cons(idx.clone(), eqnIndices.clone())
        },
        _ => {
            eqnIndices.clone()
        },
    });
            }
            eqnIndices = listAppend(resIndices.clone(), eqnIndices.clone());
            printNonLinIterVarsAndEqs(jac.clone(), eqnIndices.clone(), inEqns.clone())?;
            ()
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NONLINEAR, jac, eqns: eqnIndices, .. } => {
            printNonLinIterVarsAndEqs(jac.clone(), eqnIndices.clone(), inEqns.clone())?;
            ()
        },
        _ => {
            let mut eqnIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        (existNonLin, name) = (::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, strictTearingSet: BackendDAE::TearingSet { jac, .. }, .. } => {
            existNonLinIterVars(jac.clone())?
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NONLINEAR, jac, .. } => {
            existNonLinIterVars(jac.clone())?
        },
        _ => {
            (false, literal!(""))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if existNonLin.clone() {
            msg = (System::gettext((literal!("For more information set -d=initialization. In OMEdit Tools->Options->Simulation->Show additional information from the initialization process, in OMNotebook call setCommandLineOptions(\"-d=initialization\")")).clone())).clone();
            Error::addMessage(Error::INITIALIZATION_ITERATION_VARIABLES.clone(), list![(name.clone()).clone(), (msg.clone()).clone()])?;
        }
    }
    Ok(())
}

fn existNonLinIterVars(mut jacobian_in: Arc<BackendDAE::Jacobian>) -> Result<(bool, ArcStr)> {
    let mut existNonLin: bool = false;
    let mut jacName: ArcStr = arcstr::literal!("");
    (existNonLin, jacName) = (::match_deref::match_deref! { match &(jacobian_in.clone()) {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, name, diffVars, _, _, dependentVarsCref)), .. } => {
            let mut varCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut var: BackendDAE::Var;
            let mut exist: bool = false;
            for mut varCref in &*dependentVarsCref.clone() {
                let mut varCref = varCref.clone();
                for mut var in &*diffVars.clone() {
                    let mut var = var.clone();
                    if ComponentReferenceBasics::crefEqual(varCref.clone(), var.varName.clone())? {
                        if !(BackendVariable::varHasStartValue(var.clone())?) {
                            exist = true;
                            break;
                        }
                    }
                }
                if exist.clone() {
                    break;
                }
            }
            (exist.clone(), name.clone())
        },
        _ => {
            let mut exist: bool = false;
            (false, literal!(""))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((existNonLin, jacName))
}

fn printNonLinIterVarsAndEqs(mut jacobian: Arc<BackendDAE::Jacobian>, mut eqnIndices: Arc<metamodelica::List<i32>>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: _, .. }, name, diffVars, _, allDiffedVars, dependentVarsCref)), .. } => {
            let mut idx: i32 = 1;
            let mut nonLin: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut nonLinStart: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut lin: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut varCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut var: BackendDAE::Var;
            for mut varCref in &*dependentVarsCref.clone() {
                let mut varCref = varCref.clone();
                for mut var in &*diffVars.clone() {
                    let mut var = var.clone();
                    if ComponentReferenceBasics::crefEqual(varCref.clone(), var.varName.clone())? {
                        if !(BackendVariable::varHasStartValue(var.clone())?) {
                            nonLin = cons(var.clone(), nonLin.clone());
                        } else {
                            nonLinStart = cons(var.clone(), nonLinStart.clone());
                        }
                    }
                }
            }
            if !(nonLin.clone().is_empty()) {
                BackendDump::dumpVarList(nonLin.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Nonlinear iteration variables with default zero start attribute in ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone());
            }
            if !(nonLinStart.clone().is_empty()) {
                BackendDump::dumpVarList(nonLinStart.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Nonlinear iteration variables with predefined start attribute in ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone());
            }
            for mut var in &*allDiffedVars.clone() {
                let mut var = var.clone();
                if BackendVariable::varHasStartValue(var.clone())? && !(BackendVariable::isVarDiscrete(var.clone())) {
                    lin = cons(var.clone(), lin.clone());
                }
            }
            if !(lin.clone().is_empty()) {
                BackendDump::dumpVarList(lin.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Linear iteration variables with predefined start attributes that are unrelevant in ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone());
            }
            if !(nonLin.clone().is_empty() && nonLinStart.clone().is_empty() && lin.clone().is_empty()) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Info: Only non-linear iteration variables in non-linear eqation systems require start values.")); __mm_s.push_str(&*literal!(" All other start values have no influence on convergence and are ignored.")); __mm_s.push_str(&*if (Flags::isSet(Flags::DUMP_LOOPS.clone())?) {literal!("\n\n")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" Use \"-d=dumpLoops\" to show all loops. In OMEdit Tools->Options->Simulation->Additional Translation Flags,")); __mm_s.push_str(&*literal!(" in OMNotebook call setCommandLineOptions(\"-d=dumpLoops\")\n\n")); ArcStr::from(__mm_s) }}); ArcStr::from(__mm_s) }).clone());
            }
            ()
        },
        _ => {
            let mut idx: i32 = 1;
            let mut nonLin: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut nonLinStart: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut lin: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn getNonLinearVariables(mut jacobian: Arc<BackendDAE::Jacobian>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut nonLin: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    nonLin = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, _, diffVars, _, _, dependentVarsCref)), .. } => {
            for mut varCref in &*dependentVarsCref.clone() {
                let mut varCref = varCref.clone();
                for mut var in &*diffVars.clone() {
                    let mut var = var.clone();
                    if ComponentReferenceBasics::crefEqual(varCref.clone(), var.varName.clone())? {
                        var.initNonlinear = true;
                        nonLin = cons(var.clone(), nonLin.clone());
                        break;
                    }
                }
            }
            nonLin.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(nonLin)
}

fn traverserhasEqnNonDiffParts(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, bool) = inTpl.clone();
    let mut expList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (__pa0, (__pa1, __pa2, _)) = Expression::traverseExpTopDown(inExp.clone(), Arc::new(hasEqnNonDiffParts), inTpl.clone())?;
    outExp = __pa0.clone();
    expList = __pa1.clone();
    cont = __pa2.clone();
    if Flags::isSet(Flags::DUMP_EXCLUDED_EXP.clone())? && !(cont.clone()) {
        println!("{}", (literal!("Traverser for catching functions, that should not be differentiated\n")).clone());
        println!("{}", stringDelimitList(List::map(expList.clone(), Arc::new(ExpressionBasics::printExpStr)), (literal!("\n")).clone()));
        println!("{}", (literal!("\n\n")).clone());
    }
    Ok((outExp, cont, outTpl))
}

fn hasEqnNonDiffParts(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, (expLst, _, insideCall)) => {
                    Ok((inExp.clone(), false, (cons(inExp.clone(), expLst.clone()), false, insideCall.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, (expLst, _, insideCall)) => {
                    Ok((inExp.clone(), false, (cons(inExp.clone(), expLst.clone()), false, insideCall.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (outExp, (_, b, _)) => {
                    Ok((outExp.clone(), b.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isRecordInvoled(mut inType: Arc<DAE::Type>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            true
        },
        Deref @ DAE::Type::T_ARRAY { ty, .. } => {
            isRecordInvoled(ty.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
            isRecordInvoled(ty.clone())
        },
        Deref @ DAE::Type::T_TUPLE { types, .. } => {
            List::any(types.clone(), Arc::new(fnptr!(isRecordInvoled, Arc<DAE::Type>)))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub fn hasGenericSymbolicJacobian(mut inJacobian: Arc<BackendDAE::Jacobian>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(inJacobian.clone()) {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(_), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn calculateEqSystemStateSetsJacobians(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut outSyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared>;
    (outSyst, outShared) = (::match_deref::match_deref! { match &((inSyst.clone(), inShared.clone())) {
        (syst @ Deref @ BackendDAE::EqSystem { stateSets, orderedEqs: eqns, orderedVars: vars, .. }, shared) => {
            let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
            let mut syst = (*syst).clone();
            let mut stateSets = (*stateSets).clone();
            let mut shared = (*shared).clone();
            comps = BackendDAEUtil::getStrongComponents(syst.clone());
            (stateSets, shared) = calculateStateSetsJacobian(stateSets.clone(), vars.clone(), eqns.clone(), comps.clone(), shared.clone())?;
            assign_field!(syst.stateSets = stateSets.clone());
            (syst.clone(), shared.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSyst, outShared))
}

fn calculateStateSetsJacobian(mut inStateSets: Arc<metamodelica::List<BackendDAE::StateSet>>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<BackendDAE::StateSet>>, Arc<BackendDAE::Shared>)> {
    let mut outStateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    outStateSets = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
        for mut s in (inStateSets.clone()).into_iter().cloned() {
            let __x = (match s.clone() {
        mut stateSet => {
            (stateSet, outShared) = calculateStateSetJacobian(stateSet.clone(), inVars.clone(), inEqns.clone(), inComps.clone(), outShared.clone())?;
            stateSet.clone()
        },
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outStateSets, outShared))
}

fn calculateStateSetJacobian(mut inStateSet: BackendDAE::StateSet, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(BackendDAE::StateSet, Arc<BackendDAE::Shared>)> {
    let mut outStateSet: BackendDAE::StateSet;
    let mut outShared: Arc<BackendDAE::Shared>;
    (outStateSet, outShared) = (::match_deref::match_deref! { match &((inStateSet.clone(), inVars.clone(), inEqns.clone(), inComps.clone(), inShared.clone())) {
        (BackendDAE::StateSet { varJ, crJ, oeqns, eqns, ovars, statescandidates, varA, crA, state, rang, index, .. }, _, _, _, _) => {
            let mut shared: Arc<BackendDAE::Shared>;
            let mut crstates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut marked: metamodelica::Array<bool>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut statevars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut compvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut diffVars: BackendDAE::Variables;
            let mut allvars: BackendDAE::Variables;
            let mut oVars: BackendDAE::Variables;
            let mut resVars: BackendDAE::Variables;
            let mut compeqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut ceqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut cEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut oEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut jacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
            let mut name: ArcStr = arcstr::literal!("");
            let mut oeqns = (*oeqns).clone();
            crstates = List::map(statescandidates.clone(), Arc::new(BackendVariable::varCref));
            marked = arrayCreate(BackendVariable::varsSize(inVars.clone())?, false);
            marked = List::fold1(crstates.clone(), Arc::new(markSetStates), inVars.clone(), marked.clone());
            (compeqns, compvars) = getStateSetCompVarEqns(inComps.clone(), marked.clone(), inEqns.clone(), inVars.clone())?;
            compeqns = List::select(compeqns.clone(), Arc::new(fnptr!(removeStateSetEqn, Arc<BackendDAE::Equation>)));
            hs = List::fold(crstates.clone(), Arc::new(BaseHashSet::add), HashSet::emptyHashSet());
            compvars = List::select1(compvars.clone(), Arc::new(removeStateSetStates), hs.clone());
            (ceqns, oeqns) = IndexReduction::splitEqnsinConstraintAndOther(compvars.clone(), compeqns.clone(), inShared.clone())?;
            compvars = List::map(compvars.clone(), Arc::new(fnptr!(BackendVariable::transformXToXd, BackendDAE::Var)));
            ceqns = BackendEquation::replaceDerOpInEquationList(ceqns.clone())?;
            oeqns = BackendEquation::replaceDerOpInEquationList(oeqns.clone())?;
            ceqns = createResidualSetEquations(ceqns.clone(), crJ.clone(), 1, intGt((ceqns.clone().len() as i32), 1))?;
            allvars = BackendVariable::copyVariables(inVars.clone());
            statevars = BackendVariable::getAllStateVarFromVariables(allvars.clone())?;
            statevars = List::map(statevars.clone(), Arc::new(fnptr!(BackendVariable::transformXToXd, BackendDAE::Var)));
            allvars = BackendVariable::addVars(statevars.clone(), allvars.clone());
            resVars = BackendVariable::listVar1(varJ.clone());
            diffVars = BackendVariable::listVar1(statescandidates.clone());
            oVars = BackendVariable::listVar1(compvars.clone());
            cEqns = BackendEquation::listEquation(ceqns.clone())?;
            oEqns = BackendEquation::listEquation(oeqns.clone())?;
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("StateSetJac")); __mm_s.push_str(&*intString(System::tmpTickIndex(Global::backendDAE_jacobianSeq.clone()))); ArcStr::from(__mm_s) }).clone();
            (jacobian, shared) = getSymbolicJacobian(diffVars.clone(), cEqns.clone(), resVars.clone(), oEqns.clone(), oVars.clone(), inShared.clone(), allvars.clone(), (name.clone()).clone(), false)?;
            (BackendDAE::StateSet { index: index.clone(), rang: rang.clone(), state: state.clone(), crA: crA.clone(), varA: varA.clone(), statescandidates: statescandidates.clone(), ovars: ovars.clone(), eqns: eqns.clone(), oeqns: oeqns.clone(), crJ: crJ.clone(), varJ: varJ.clone(), jacobian: jacobian.clone() }, shared.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outStateSet, outShared))
}

fn markSetStates(mut inCr: Arc<DAE::ComponentRef>, mut iVars: BackendDAE::Variables, mut iMark: metamodelica::Array<bool>) -> Result<metamodelica::Array<bool>> {
    let mut oMark: metamodelica::Array<bool>;
    let mut index: i32 = 0;
    (_, index) = BackendVariable::getVarSingle(inCr.clone(), iVars.clone())?;
    oMark = {let _arr = iMark.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = true; _arr};
    Ok(oMark)
}

fn removeStateSetStates(mut inVar: BackendDAE::Var, mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<bool> {
    let mut b: bool = false;
    b = !(BaseHashSet::has(BackendVariable::varCref(inVar.clone())?, hs.clone())?);
    Ok(b)
}

fn removeStateSetEqn(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { source: Deref @ DAE::ElementSource { info: SourceInfo { fileName: Deref @ "stateselection", .. }, .. }, .. } => false,
        Deref @ BackendDAE::Equation::EQUATION { source: Deref @ DAE::ElementSource { info: SourceInfo { fileName: Deref @ "stateselection", .. }, .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn foundMarked(mut ilst: Arc<metamodelica::List<i32>>, mut marked: metamodelica::Array<bool>) -> Result<bool> {
    let mut found: bool = false;
    found = (::match_deref::match_deref! { match &((ilst.clone(), marked.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            false
        },
        (Deref @ metamodelica::List::Cons { head: i, tail: rest }, _) => {
            let mut b: bool = false;
            b = marked.borrow()[(i.clone()-1) as usize].clone();
            b = if (!(b.clone())) {foundMarked(rest.clone(), marked.clone())?} else {b.clone()};
            b.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(found)
}

fn getStateSetCompVarEqns(mut inComp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut marked: metamodelica::Array<bool>, mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVariables: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    for mut comp in &*inComp.clone() {
        let mut comp = comp.clone();
        (elst, vlst) = BackendDAETransform::getEquationAndSolvedVarIndxes(comp.clone())?;
        if foundMarked(vlst.clone(), marked.clone())? {
            eqnlst = BackendEquation::getList(elst.clone(), inEquationArray.clone());
            varlst = List::map1r(vlst.clone(), Arc::new(BackendVariable::getVarAt), inVariables.clone());
            outEquations = listAppend(eqnlst.clone(), outEquations.clone());
            outVars = listAppend(varlst.clone(), outVars.clone());
        }
    }
    Ok((outEquations, outVars))
}

fn createResidualSetEquations(mut iEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut crJ: Arc<DAE::ComponentRef>, mut index: i32, mut applySubs: bool) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut oEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut idx: i32 = index.clone();
    oEqs = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut eq in (iEqs.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: e2, exp: e1 } => {
            let mut crj: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut res: Arc<DAE::Exp>;
            let mut expJ: Arc<DAE::Exp>;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            crj = if (applySubs.clone()) {ComponentReference::subscriptCrefWithInt(crJ.clone(), idx.clone())?} else {crJ.clone()};
            expJ = Expression::crefExp(crj.clone())?;
            res = Expression::expSub(e1.clone(), e2.clone())?;
            eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: expJ.clone(), scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() });
            idx = idx.clone() + 1;
            eqn.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: eqAttr, source, exp: e1 } => {
            let mut expJ: Arc<DAE::Exp>;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            expJ = Expression::crefExp(ComponentReference::subscriptCrefWithInt(crJ.clone(), idx.clone())?)?;
            eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: expJ.clone(), scalar: e1.clone(), source: source.clone(), attr: eqAttr.clone() });
            idx = idx.clone() + 1;
            eqn.clone()
        },
        eqn => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function createResidualSetEquations failed for equation: ")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(oEqs)
}

pub fn calculateJacobian(mut inVariables: BackendDAE::Variables, mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut differentiateIfExp: bool, mut iShared: Arc<BackendDAE::Shared>) -> Result<(Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>, Arc<BackendDAE::Shared>)> {
    let mut outTplIntegerIntegerEquationLstOption: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>> = None;
    let mut oShared: Arc<BackendDAE::Shared>;
    (outTplIntegerIntegerEquationLstOption, oShared) = 'mc: {
        let __mc_input = (inVariables.clone(), inEquationArray.clone(), inAdjacencyMatrix.clone(), differentiateIfExp.clone(), iShared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, eqns, m, _, _) => {
                    let mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared>;
                    (jac, shared) = calculateJacobianRows(eqns.clone(), vars.clone(), m.clone(), 1, 1, differentiateIfExp.clone(), iShared.clone(), Arc::new(BackendDAEUtil::varsInEqn))?;
                    Ok((Some(jac.clone()), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, iShared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outTplIntegerIntegerEquationLstOption, oShared))
}

fn calculateJacobianRows<Type_a: Clone + 'static>(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut m: Type_a, mut eqn_indx: i32, mut scalar_eqn_indx: i32, mut differentiateIfExp: bool, mut iShared: Arc<BackendDAE::Shared>, mut varsInEqn: Arc<dyn ::std::ops::Fn(Type_a, i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>) -> Result<(Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, Arc<BackendDAE::Shared>)> {
    pub type varsInEqnFunc<Type_a: Clone> = fn(Type_a, i32) -> Result<Arc<metamodelica::List<i32>>>;

    let mut outLst: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut oShared: Arc<BackendDAE::Shared> = iShared.clone();
    let mut size: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    i = eqn_indx.clone();
    j = scalar_eqn_indx.clone();
    size = 0;
    n = ExpandableArray::getLastUsedIndex(inEquationArray.clone());
    for mut k in 1..=n.clone() {
        if ExpandableArray::occupied(k.clone(), inEquationArray.clone()) {
            eqn = ExpandableArray::get(k.clone(), inEquationArray.clone())?;
            (outLst, size, oShared) = calculateJacobianRow(eqn.clone(), vars.clone(), m.clone(), i.clone(), j.clone(), differentiateIfExp.clone(), oShared.clone(), varsInEqn.clone(), outLst.clone())?;
            i = i.clone() + 1;
            j = j.clone() + size.clone();
        }
    }
    outLst = metamodelica::Dangerous::listReverseInPlace(outLst.clone());
    Ok((outLst, oShared))
}

fn calculateJacobianRow<Type_a: Clone + 'static>(mut inEquation: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut m: Type_a, mut eqn_indx: i32, mut scalar_eqn_indx: i32, mut differentiateIfExp: bool, mut iShared: Arc<BackendDAE::Shared>, mut fvarsInEqn: Arc<dyn ::std::ops::Fn(Type_a, i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>, mut iAcc: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<(Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, i32, Arc<BackendDAE::Shared>)> {
    pub type varsInEqnFunc<Type_a: Clone> = fn(Type_a, i32) -> Result<Arc<metamodelica::List<i32>>>;

    let mut outLst: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut oShared: Arc<BackendDAE::Shared>;
    (outLst, size, oShared) = (::match_deref::match_deref! { match &((inEquation.clone(), vars.clone(), m.clone(), eqn_indx.clone(), scalar_eqn_indx.clone(), differentiateIfExp.clone(), iShared.clone(), fvarsInEqn.clone(), iAcc.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { source, scalar: e2, exp: e1, .. }, _, _, _, _, _, _, _, _) => {
            let mut var_indxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut var_indxs_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared>;
            var_indxs = fvarsInEqn(m.clone(), eqn_indx.clone())?;
            var_indxs_1 = List::sort(var_indxs.clone(), Arc::new(fnptr!(intGt, i32, i32)))?;
            var_indxs_1 = List::sortedUnique(var_indxs_1.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
            (eqns, shared) = calculateJacobianRow2(Expression::expSub(e1.clone(), e2.clone())?, vars.clone(), scalar_eqn_indx.clone(), var_indxs_1.clone(), differentiateIfExp.clone(), iShared.clone(), source.clone(), iAcc.clone())?;
            (eqns.clone(), 1, shared.clone())
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { source, exp: e, .. }, _, _, _, _, _, _, _, _) => {
            let mut var_indxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut var_indxs_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared>;
            var_indxs = fvarsInEqn(m.clone(), eqn_indx.clone())?;
            var_indxs_1 = List::sort(var_indxs.clone(), Arc::new(fnptr!(intGt, i32, i32)))?;
            var_indxs_1 = List::sortedUnique(var_indxs_1.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
            (eqns, shared) = calculateJacobianRow2(e.clone(), vars.clone(), scalar_eqn_indx.clone(), var_indxs_1.clone(), differentiateIfExp.clone(), iShared.clone(), source.clone(), iAcc.clone())?;
            (eqns.clone(), 1, shared.clone())
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { source, exp: e2, componentRef: cr, .. }, _, _, _, _, _, _, _, _) => {
            let mut var_indxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut var_indxs_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
            let mut e1: Arc<DAE::Exp>;
            let mut shared: Arc<BackendDAE::Shared>;
            e1 = Expression::crefExp(cr.clone())?;
            var_indxs = fvarsInEqn(m.clone(), eqn_indx.clone())?;
            var_indxs_1 = List::sort(var_indxs.clone(), Arc::new(fnptr!(intGt, i32, i32)))?;
            var_indxs_1 = List::sortedUnique(var_indxs_1.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
            (eqns, shared) = calculateJacobianRow2(Expression::expSub(e1.clone(), e2.clone())?, vars.clone(), scalar_eqn_indx.clone(), var_indxs_1.clone(), differentiateIfExp.clone(), iShared.clone(), source.clone(), iAcc.clone())?;
            (eqns.clone(), 1, shared.clone())
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { source, right: e2, left: e1, dimSize: ds, .. }, _, _, _, _, _, _, _, _) => {
            let mut var_indxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut var_indxs_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
            let mut e: Arc<DAE::Exp>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared>;
            let _ = Expression::r#typeof(e1.clone())?;
            e = Expression::expSub(e1.clone(), e2.clone())?;
            (e, _) = Expression::extendArrExp(e.clone(), false)?;
            subslst = Expression::dimensionSizesSubscripts(ds.clone());
            subslst = Expression::rangesToSubscripts(subslst.clone())?;
            expl = List::map1r(subslst.clone(), Arc::new(Expression::applyExpSubscripts), e.clone());
            var_indxs = fvarsInEqn(m.clone(), eqn_indx.clone())?;
            var_indxs_1 = List::sort(var_indxs.clone(), Arc::new(fnptr!(intGt, i32, i32)))?;
            var_indxs_1 = List::sortedUnique(var_indxs_1.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
            (eqns, shared) = calculateJacobianRowLst(expl.clone(), vars.clone(), scalar_eqn_indx.clone(), var_indxs_1.clone(), differentiateIfExp.clone(), iShared.clone(), source.clone(), iAcc.clone())?;
            size = List::fold(ds.clone(), Arc::new(fnptr!(intMul, i32, i32)), 1);
            (eqns.clone(), size.clone(), shared.clone())
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            r#str = (BackendDump::dumpEqnsStr(list![inEquation.clone()])?).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAE.calculateJacobianRow failed on ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outLst, size, oShared))
}

fn calculateJacobianRowLst(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut vars: BackendDAE::Variables, mut eqn_indx: i32, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut differentiateIfExp: bool, mut iShared: Arc<BackendDAE::Shared>, mut source: Arc<DAE::ElementSource>, mut iAcc: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<(Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, Arc<BackendDAE::Shared>)> {
    let mut outLst: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = iAcc.clone();
    let mut oShared: Arc<BackendDAE::Shared> = iShared.clone();
    let mut eqn_indx_arr: i32 = eqn_indx.clone();
    for mut e in &*inExps.clone() {
        let mut e = e.clone();
        (outLst, oShared) = calculateJacobianRow2(e.clone(), vars.clone(), eqn_indx_arr.clone(), inIntegerLst.clone(), differentiateIfExp.clone(), oShared.clone(), source.clone(), outLst.clone())?;
        eqn_indx_arr = eqn_indx_arr.clone() + 1;
    }
    Ok((outLst, oShared))
}

fn calculateJacobianRow2(mut inExp: Arc<DAE::Exp>, mut vars: BackendDAE::Variables, mut eqn_indx: i32, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut differentiateIfExp: bool, mut iShared: Arc<BackendDAE::Shared>, mut source: Arc<DAE::ElementSource>, mut iAcc: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<(Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, Arc<BackendDAE::Shared>)> {
    let mut outLst: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = iAcc.clone();
    let mut oShared: Arc<BackendDAE::Shared> = iShared.clone();
    let mut e: Arc<DAE::Exp>;
    let mut e_1: Arc<DAE::Exp>;
    let mut dcrexp: Arc<DAE::Exp>;
    let mut v: BackendDAE::Var;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut vindx: i32 = 0;
    let mut r#str: ArcStr = arcstr::literal!("");
    if '__try0: {
        for mut vindx in &*inIntegerLst.clone() {
            let mut vindx = vindx.clone();
            v = unwrap_break_err!(BackendVariable::getVarAt(vars.clone(), vindx.clone()), '__try0);
            cr = unwrap_break_err!(BackendVariable::varCref(v.clone()), '__try0);
            if BackendVariable::isStateVar(v.clone()) {
                dcr = ComponentReference::crefPrefixDer(cr.clone());
                dcrexp = unwrap_break_err!(Expression::crefExp(cr.clone()), '__try0);
                dcrexp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![dcrexp.clone()], attr: DAE::callAttrBuiltinReal.clone() });
                (e, _) = unwrap_break_err!(Expression::replaceExp(inExp.clone(), dcrexp.clone(), Expression::crefExp(dcr.clone())?), '__try0);
            }
            (e_1, oShared) = unwrap_break_err!(Differentiate::differentiateExpCrefFullJacobian(inExp.clone(), cr.clone(), vars.clone(), oShared.clone()), '__try0);
            if !(Expression::isZero(e_1.clone())) {
                outLst = cons((eqn_indx.clone(), vindx.clone(), Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e_1.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })), outLst.clone());
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            r#str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAE.calculateJacobianRow2 failed on ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
        }
        bail!("fail");
    }
    Ok((outLst, oShared))
}

pub fn analyzeJacobian(mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inTplIntegerIntegerEquationLstOption: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>) -> Result<(BackendDAE::JacobianType, bool)> {
    let mut outJacobianType: BackendDAE::JacobianType = BackendDAE::JacobianType::JAC_CONSTANT;
    let mut jacConstant: bool = false;
    (outJacobianType, jacConstant) = 'mc: {
        let __mc_input = (vars.clone(), eqns.clone(), inTplIntegerIntegerEquationLstOption.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(jac)) => {
                    let mut b: bool = false;
                    b = jacobianNonlinear(vars.clone(), jac.clone())?;
                    let (_, false) = (if (!(b.clone())) {BackendDAEUtil::traverseBackendDAEExpsEqnsWithStop(eqns.clone(), Arc::new(varsNotInRelations), (vars.clone(), true))?} else {(vars.clone(), false)}) else { bail!("pattern mismatch") };
                    Ok((crate::BackendDAE::JacobianType::JAC_NONLINEAR, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(jac)) => {
                    let mut b: bool = false;
                    let mut jactype: BackendDAE::JacobianType = BackendDAE::JacobianType::JAC_CONSTANT;
                    let true = (jacobianConstant(jac.clone())?) else { bail!("pattern mismatch") };
                    b = rhsConstant(vars.clone(), eqns.clone())?;
                    jactype = if (b.clone()) {crate::BackendDAE::JacobianType::JAC_CONSTANT} else {crate::BackendDAE::JacobianType::JAC_LINEAR};
                    Ok((jactype.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(_)) => {
                    Ok((crate::BackendDAE::JacobianType::JAC_LINEAR, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, None) => {
                    Ok((crate::BackendDAE::JacobianType::JAC_NO_ANALYTIC, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outJacobianType, jacConstant))
}

fn jacobianNonlinear(mut vars: BackendDAE::Variables, mut inTplIntegerIntegerEquationLst: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<bool> {
    let mut isNonLinear: bool = false;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut tpl: (i32, i32, Arc<BackendDAE::Equation>);
    for mut tpl in &*inTplIntegerIntegerEquationLst.clone() {
        let mut tpl = tpl.clone();
        (_, _, eq) = tpl.clone();
        isNonLinear = (::match_deref::match_deref! { match &((vars.clone(), eq.clone())) {
        (_, Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }) => jacobianNonlinearExp(vars.clone(), e1.clone())? || jacobianNonlinearExp(vars.clone(), e2.clone())?,
        (_, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }) => jacobianNonlinearExp(vars.clone(), e.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        if isNonLinear.clone() {
            return Ok(isNonLinear);
        }
    }
    Ok(isNonLinear)
}

fn jacobianNonlinearExp(mut vars: BackendDAE::Variables, mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool = false;
    let (_, (_, __pa0)) = Expression::traverseExpTopDown(inExp.clone(), Arc::new(traverserjacobianNonlinearExp), (vars.clone(), false))?;
    outBoolean = __pa0.clone();
    Ok(outBoolean)
}

fn traverserjacobianNonlinearExp(mut inExp: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), tpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _)) => {
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((e.clone(), false, (vars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, _)) => {
                    (_, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok((e.clone(), false, (vars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((e.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
                    Ok((e.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, (_, b)) => {
                    Ok((e.clone(), !(b.clone()), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

fn jacobianConstant(mut inTplIntegerIntegerEquationLst: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<bool> {
    let mut outBoolean: bool = true;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut tpl: (i32, i32, Arc<BackendDAE::Equation>);
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut tpl in &*inTplIntegerIntegerEquationLst.clone() {
        let mut tpl = tpl.clone();
        eqn = Util::tuple33(tpl.clone());
        outBoolean = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. } => Expression::isConst(e1.clone())? && Expression::isConst(e2.clone())?,
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => Expression::isConst(e.clone())?,
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, .. } => Expression::isConst(e.clone())?,
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. } => Expression::isConst(e1.clone())? && Expression::isConst(e2.clone())?,
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. } => Expression::isConst(e1.clone())? && Expression::isConst(e2.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(outBoolean.clone()) {
            break;
        }
    }
    Ok(outBoolean)
}

pub fn isJacobianGeneric(mut inJac: Arc<BackendDAE::Jacobian>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inJac.clone()) {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn varsNotInRelations(mut exp: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool))> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cont: bool = false;
    let mut tpl: (BackendDAE::Variables, bool) = tpl;
    (exp, cont, tpl) = (::match_deref::match_deref! { match &((exp.clone(), tpl.clone())) {
        (Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: t, expElse: f }, (vars, b)) => {
            let mut t = (*t).clone();
            let mut f = (*f).clone();
            let mut b = (*b).clone();
            let (_, (_, __pa0)) = Expression::traverseExpTopDown(cond.clone(), Arc::new(BackendDAEUtil::getEqnsysRhsExp2), (vars.clone(), b.clone()))?;
            b = __pa0.clone();
            let (__pa1, (_, __pa2)) = Expression::traverseExpTopDown(t.clone(), Arc::new(varsNotInRelations), (vars.clone(), b.clone()))?;
            t = __pa1.clone();
            b = __pa2.clone();
            let (__pa3, (_, __pa4)) = Expression::traverseExpTopDown(f.clone(), Arc::new(varsNotInRelations), (vars.clone(), b.clone()))?;
            f = __pa3.clone();
            b = __pa4.clone();
            (Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: t.clone(), expElse: f.clone() }), false, (vars.clone(), b.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
            (exp.clone(), true, tpl.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (exp.clone(), false, tpl.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (exp.clone(), false, tpl.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst, .. }, _) => {
            (_, tpl) = Expression::traverseExpListTopDown(expLst.clone(), Arc::new(BackendDAEUtil::getEqnsysRhsExp2), tpl.clone())?;
            (exp.clone(), false, tpl.clone())
        },
        (Deref @ DAE::Exp::LBINARY { .. }, _) => {
            (_, tpl) = Expression::traverseExpTopDown(exp.clone(), Arc::new(BackendDAEUtil::getEqnsysRhsExp2), tpl.clone())?;
            (exp.clone(), false, tpl.clone())
        },
        (Deref @ DAE::Exp::LUNARY { .. }, tpl) => {
            let mut tpl = (*tpl).clone();
            (_, tpl) = Expression::traverseExpTopDown(exp.clone(), Arc::new(BackendDAEUtil::getEqnsysRhsExp2), tpl.clone())?;
            (exp.clone(), false, tpl.clone())
        },
        (Deref @ DAE::Exp::RELATION { .. }, tpl) => {
            let mut tpl = (*tpl).clone();
            (_, tpl) = Expression::traverseExpTopDown(exp.clone(), Arc::new(BackendDAEUtil::getEqnsysRhsExp2), tpl.clone())?;
            (exp.clone(), false, tpl.clone())
        },
        (Deref @ DAE::Exp::ASUB { sub: subs, exp: e1 }, _) => {
            let mut b: bool = false;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            let (_, ref __pa1 @ (_, ref __pa0)) = Expression::traverseExpTopDown(e1.clone(), Arc::new(varsNotInRelations), tpl.clone())?;
            b = __pa0.clone();
            tpl = __pa1.clone();
            if b.clone() {
                (_, tpl) = Expression::traverseExpListTopDown(expLst.clone(), Arc::new(BackendDAEUtil::getEqnsysRhsExp2), tpl.clone())?;
            }
            (exp.clone(), false, tpl.clone())
        },
        (_, (_, b)) => {
            (exp.clone(), b.clone(), tpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, cont, tpl))
}

fn rhsConstant(mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<bool> {
    let mut outBoolean: bool = false;
    let mut repl: BackendVarTransform::VariableReplacements;
    if BackendEquation::equationArraySize(eqns.clone())? == 0 {
        outBoolean = true;
    } else {
        repl = BackendDAEUtil::makeZeroReplacements(vars.clone())?;
        (_, outBoolean, _) = BackendEquation::traverseEquationArray_WithStop(eqns.clone(), Arc::new(rhsConstant2), (vars.clone(), true, repl.clone()))?;
    }
    Ok(outBoolean)
}

fn rhsConstant2(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)) -> Result<(Arc<BackendDAE::Equation>, bool, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements);
    (outEq, cont, outTpl) = 'mc: {
        let __mc_input = (inEq.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, (vars, b, repl)) => {
                    let mut new_exp: Arc<DAE::Exp>;
                    let mut rhs_exp: Arc<DAE::Exp>;
                    let mut res: bool = false;
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    rhs_exp = BackendDAEUtil::getEqnsysRhsExp(new_exp.clone(), vars.clone(), None, Some(repl.clone()))?;
                    res = Expression::isConst(rhs_exp.clone())?;
                    Ok((eqn.clone(), res.clone(), (vars.clone(), b.clone() && res.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }, (vars, b, repl)) => {
                    let mut new_exp: Arc<DAE::Exp>;
                    let mut rhs_exp: Arc<DAE::Exp>;
                    let mut res: bool = false;
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    rhs_exp = BackendDAEUtil::getEqnsysRhsExp(new_exp.clone(), vars.clone(), None, Some(repl.clone()))?;
                    res = Expression::isConst(rhs_exp.clone())?;
                    Ok((eqn.clone(), res.clone(), (vars.clone(), b.clone() && res.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. }, (vars, b, repl)) => {
                    let mut new_exp: Arc<DAE::Exp>;
                    let mut rhs_exp: Arc<DAE::Exp>;
                    let mut res: bool = false;
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    rhs_exp = BackendDAEUtil::getEqnsysRhsExp(new_exp.clone(), vars.clone(), None, Some(repl.clone()))?;
                    res = Expression::isConst(rhs_exp.clone())?;
                    Ok((eqn.clone(), res.clone(), (vars.clone(), b.clone() && res.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, (vars, b, repl)) => {
                    let mut rhs_exp: Arc<DAE::Exp>;
                    let mut res: bool = false;
                    rhs_exp = BackendDAEUtil::getEqnsysRhsExp(e.clone(), vars.clone(), None, Some(repl.clone()))?;
                    res = Expression::isConst(rhs_exp.clone())?;
                    Ok((eqn.clone(), res.clone(), (vars.clone(), b.clone() && res.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, (vars, _, repl)) => {
                    Ok((eqn.clone(), false, (vars.clone(), false, repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEq, cont, outTpl))
}

fn getJacobianResiduals(mut jacDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut diffedRes: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut syst: Arc<BackendDAE::EqSystem>;
    let __pa0 = ::match_deref::match_deref! { match &(jacDAE.eqs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    syst = __pa0.clone();
    diffedRes = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (BackendVariable::varList(syst.orderedVars.clone())?).into_iter().cloned() {
            if !(BackendVariable::isRESVar(var.clone())) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(diffedRes)
}

// =============================================================================
// Function detects non-linear strong component in symbolic jacobians
//  - non-linear components should never appear in symbolic jacobian and
//    indicate an singular or wrong system
//  - this modules stops compiling and outputs an error, otherwise we
//    would get error at runtime compiling
// =============================================================================
fn checkForNonLinearStrongComponents(mut symbolicJacobian: (Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<bool> {
    let mut result: bool = false;
    let mut jacBDAE: Arc<BackendDAE::BackendDAE>;
    let mut name: ArcStr = arcstr::literal!("");
    (jacBDAE, name, _, _, _, _) = symbolicJacobian.clone();
    match '__try0: {
        let _ = unwrap_break_err!(BackendDAEUtil::mapEqSystem(jacBDAE.clone(), Arc::new(checkForNonLinearStrongComponents_work)), '__try0);
        result = true;
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            Error::addMessage(Error::INVALID_NONLINEAR_JACOBIAN_COMPONENT.clone(), list![(name.clone()).clone()])?;
            result = false;
        }
    }
    Ok(result)
}

fn checkForNonLinearStrongComponents_work(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        comps = __pa1.clone();
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NONLINEAR, .. } => {
            if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP.clone()), '__try0) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[symjacdump] Following strong component represents a nonlinear symbolic jacobian:\n")); __mm_s.push_str(&*unwrap_break_err!(BackendDump::printComponent(comp.clone(), Some(syst.clone())), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"))
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC, .. } => {
            if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP.clone()), '__try0) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[symjacdump] Following strong component represents a no symbolic jacobian:\n")); __mm_s.push_str(&*unwrap_break_err!(BackendDump::printComponent(comp.clone(), Some(syst.clone())), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"))
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_GENERIC, .. } => {
            if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP.clone()), '__try0) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[symjacdump] Following strong component represents a generic jacobian:\n")); __mm_s.push_str(&*unwrap_break_err!(BackendDump::printComponent(comp.clone(), Some(syst.clone())), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"))
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, .. } => {
            if unwrap_break_err!(Flags::isSet(Flags::JAC_DUMP.clone()), '__try0) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[symjacdump] Following (torn) strong component represents a nonlinear symbolic jacobian:\n")); __mm_s.push_str(&*unwrap_break_err!(BackendDump::printComponent(comp.clone(), Some(syst.clone())), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"))
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok::<_, anyhow::Error>((comps.clone(),))
    } {
        Ok((__try0_o0,)) => {
            comps = __try0_o0;
        }
        Err(_) => {
            bail!("fail");
        }
    }
    Ok((syst, shared))
}

pub fn getFixedStatesForSelfdependentSets(mut stateSet: BackendDAE::StateSet, mut unfixedStates: Arc<metamodelica::List<BackendDAE::Var>>, mut toFix: i32) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut statesToFix: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut nonlinearCountLst: Arc<metamodelica::List<(i32, BackendDAE::Var)>> = metamodelica::nil();
    let mut nonlinearCount: i32 = 0;
    let _ = (::match_deref::match_deref! { match &(stateSet.jacobian.clone()) {
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(sJac), .. } => {
            let mut dae: Arc<BackendDAE::BackendDAE>;
            let mut matrixName: ArcStr = arcstr::literal!("");
            (dae, matrixName, _, _, _, _) = sJac.clone();
            for mut var in &*unfixedStates.clone() {
                let mut var = var.clone();
                nonlinearCountLst = cons(getNonlinearStateCount(var.clone(), unfixedStates.clone(), dae.clone(), (matrixName.clone()).clone())?, nonlinearCountLst.clone());
            }
            0
        },
        _ => bail!("match: no arm matched"),
    } });
    statesToFix = fixedVarsFromNonlinearCount(nonlinearCountLst.clone(), toFix.clone())?;
    Ok(statesToFix)
}

fn getNonlinearStateCount(mut state: BackendDAE::Var, mut diffVars: Arc<metamodelica::List<BackendDAE::Var>>, mut dae: Arc<BackendDAE::BackendDAE>, mut matrixName: ArcStr) -> Result<(i32, BackendDAE::Var)> {
    let mut outTpl: (i32, BackendDAE::Var);
    outTpl = (::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { eqs: systs, .. } => {
            let mut tpl: (BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>, i32, ArcStr);
            let mut outState: BackendDAE::Var;
            let mut nonlinearCount: i32 = 0;
            tpl = (state.clone(), diffVars.clone(), nonlinearCount.clone(), matrixName.clone());
            for mut syst in &*systs.clone() {
                let mut syst = syst.clone();
                let _ = (::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: _, orderedEqs: eqnarray, m: _, mT: _, mapping: _, matching: _, stateSets: _, partitionKind: _, .. } => {
            tpl = BackendEquation::traverseEquationArray(eqnarray.clone(), Arc::new(getNonlinearStateCount0), tpl.clone())?;
            0
        },
        _ => bail!("match: no arm matched"),
    } });
            }
            (outState, _, nonlinearCount, _) = tpl.clone();
            (nonlinearCount.clone(), outState.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpl)
}

fn getNonlinearStateCount0(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>, i32, ArcStr)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>, i32, ArcStr))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTpl: (BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>, i32, ArcStr);
    outEq = inEq.clone();
    outTpl = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: exp, .. } => {
            let mut diffExp: Arc<DAE::Exp>;
            let mut state: BackendDAE::Var;
            let mut diffVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut nonlinearCount: i32 = 0;
            let mut matrixName: ArcStr = arcstr::literal!("");
            let mut seedVar: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (state, diffVars, nonlinearCount, matrixName) = inTpl.clone();
            seedVar = Differentiate::createSeedCrefName(BackendVariable::varCref(state.clone())?, (matrixName.clone()).clone())?;
            diffExp = Differentiate::differentiateExpSolve(exp.clone(), seedVar.clone(), None)?;
            for mut var in &*diffVars.clone() {
                let mut var = var.clone();
                if !(ComponentReferenceBasics::crefEqual(var.varName.clone(), state.varName.clone())?) && Expression::expContains(diffExp.clone(), Expression::crefExp(var.varName.clone())?)? {
                    if Expression::isZero(BackendVariable::varStartValue(var.clone())?) {
                        nonlinearCount = nonlinearCount.clone() + 2;
                    } else {
                        nonlinearCount = nonlinearCount.clone() + 1;
                    }
                }
            }
            (state.clone(), diffVars.clone(), nonlinearCount.clone(), matrixName.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outEq, outTpl))
}

fn fixedVarsFromNonlinearCount(mut tplLst: Arc<metamodelica::List<(i32, BackendDAE::Var)>>, mut toFix: i32) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut fixedVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut sortedTplLst: Arc<metamodelica::List<(i32, BackendDAE::Var)>> = metamodelica::nil();
    let mut strippedTplLst: Arc<metamodelica::List<(i32, BackendDAE::Var)>> = metamodelica::nil();
    let mut fixVar: BackendDAE::Var;
    let mut fixInt: i32 = 0;
    for mut tpl in &*tplLst.clone() {
        let mut tpl = tpl.clone();
        (fixInt, fixVar) = tpl.clone();
    }
    sortedTplLst = List::sort(tplLst.clone(), Arc::new(fnptr!(Util::compareTupleIntGt, _, _)))?;
    strippedTplLst = List::firstN(sortedTplLst.clone(), toFix.clone())?;
    for mut tpl in &*strippedTplLst.clone() {
        let mut tpl = tpl.clone();
        (_, fixVar) = tpl.clone();
        fixVar.values = DAEUtil::setFixedAttr(fixVar.values.clone(), Some(Arc::new(DAE::Exp::BCONST { bool: true })))?;
        fixedVars = cons(fixVar.clone(), fixedVars.clone());
    }
    Ok(fixedVars)
}

fn stripPartialDerNonlinearPattern(mut pat: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32)) -> (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32) {
    let mut pat: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32) = pat;
    let mut pat_cref: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
    let mut pat_crefT: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
    let mut v1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut v2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let (__pa0, __pa1, (__pa2, __pa3), __pa4) = pat.clone();
    pat_cref = __pa0.clone();
    pat_crefT = __pa1.clone();
    v1 = __pa2.clone();
    v2 = __pa3.clone();
    index = __pa4.clone();
    pat_cref = {
        let mut __acc: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
        for mut cref_tpl in (pat_cref.clone()).into_iter().cloned() {
            let __x = stripPartialDer(cref_tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    pat_crefT = {
        let mut __acc: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
        for mut cref_tpl in (pat_crefT.clone()).into_iter().cloned() {
            let __x = stripPartialDer(cref_tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    v1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (v1.clone()).into_iter().cloned() {
            let __x = (stripPartialDerWork(v.clone())).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    v2 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (v2.clone()).into_iter().cloned() {
            let __x = (stripPartialDerWork(v.clone())).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    pat = (pat_cref.clone(), pat_crefT.clone(), (v1.clone(), v2.clone()), index.clone());
    pat
}

fn stripPartialDer(mut cref_tpl: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) {
    let mut cref_tpl: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) = cref_tpl;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut dependencies: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (cref, dependencies) = cref_tpl.clone();
    (cref, _) = stripPartialDerWork(cref.clone());
    dependencies = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = (stripPartialDerWork(dep.clone())).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    cref_tpl = (cref.clone(), dependencies.clone());
    cref_tpl
}

fn stripPartialDerWork(mut cref: Arc<DAE::ComponentRef>) -> (Arc<DAE::ComponentRef>, bool) {
    let mut cref: Arc<DAE::ComponentRef> = cref;
    let mut strip: bool = false;
    (cref, strip) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } if (StringUtil::startsWith((var_field!((*cref).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (literal!("$pDER")).clone())) => {
            (cref.clone(), true)
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } if (StringUtil::startsWith((var_field!((*cref).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (literal!("$pDER")).clone())) => {
            (cref.clone(), true)
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (cr, strip) = stripPartialDerWork(var_field!((*cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone());
            if strip.clone() {
                cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (var_field!((*cref).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), identType: var_field!((*cref).identType, DAE::ComponentRef::CREF_QUAL).clone(), subscriptLst: var_field!((*cref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone() });
            } else {
                cr = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (var_field!((*cref).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), identType: var_field!((*cref).identType, DAE::ComponentRef::CREF_QUAL).clone(), subscriptLst: var_field!((*cref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), componentRef: cr.clone() });
            }
            (cr.clone(), false)
        },
        _ => {
            (cref.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (cref, strip)
}

// =============================================================================
// [ASSC] section for analytical to symbolical singularity transformation
//
// Generates linear jacobian
// =============================================================================
pub type LinearJacobianRow = Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>;

pub type LinearJacobianRhs = metamodelica::Array<Arc<DAE::Exp>>;

pub type LinearJacobianInd = metamodelica::Array<(i32, i32)>;

pub mod LinearJacobian {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct LinearJacobian {
        /// all loop variables entries
        pub rows: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>>,
        /// the expression containing all non loop variable entries
        pub rhs: metamodelica::Array<Arc<DAE::Exp>>,
        /// equation indices  <array, scalar>
        pub ind: metamodelica::Array<(i32, i32)>,
        /// changed equations
        pub eq_marks: metamodelica::Array<bool>,
    }

    impl Default for LinearJacobian {
        fn default() -> Self {
            Self {
                rows: Default::default(),
                rhs: Default::default(),
                ind: Default::default(),
                eq_marks: Default::default(),
            }
        }
    }

    pub type LINEAR_REAL_JACOBIAN = LinearJacobian;

    pub fn toString(mut linJac: Arc<LinearJacobian>, mut heading: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("######################################################\n")); __mm_s.push_str(&*literal!(" LinearJacobian sparsity pattern: ")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("######################################################\n")); __mm_s.push_str(&*literal!("(scal_idx|arr_idx|changed) [var_index, value] || RHS_EXPRESSION\n")); ArcStr::from(__mm_s) }).clone();
        let __range0 = 1..=(linJac.rows.clone().borrow().len() as i32);
        for mut idx in __range0 {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*rowToString(linJac.rows.borrow()[(idx.clone()-1) as usize].clone(), linJac.rhs.borrow()[(idx.clone()-1) as usize].clone(), linJac.ind.borrow()[(idx.clone()-1) as usize].clone(), linJac.eq_marks.borrow()[(idx.clone()-1) as usize].clone())?); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    fn rowToString(mut row: Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>, mut rhs: Arc<DAE::Exp>, mut indices: (i32, i32), mut changed: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut i_arr: i32 = 0;
        let mut i_scal: i32 = 0;
        let mut index: i32 = 0;
        let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut row_lst: Arc<metamodelica::List<(i32, metamodelica::Real)>> = UnorderedMap::toList(row.clone());
        (i_arr, i_scal) = indices.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i_arr.clone())); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*intString(i_scal.clone())); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*boolString(changed.clone())); __mm_s.push_str(&*literal!("):    ")); ArcStr::from(__mm_s) }).clone();
        if row_lst.clone().is_empty() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("EMPTY ROW     ")); ArcStr::from(__mm_s) }).clone();
        } else {
            for mut element in &*row_lst.clone() {
                let mut element = element.clone();
                (index, value) = element.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*realString(value.clone())); __mm_s.push_str(&*literal!("] ")); ArcStr::from(__mm_s) }).clone();
            }
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("    || RHS: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr((ExpressionSimplify::simplify(rhs.clone())?).0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn generate(mut loopEqs: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, (i32, i32))>>, mut loopVars: Arc<metamodelica::List<(BackendDAE::Var, i32)>>, mut ass1: metamodelica::Array<i32>) -> Result<Arc<LinearJacobian>> {
        type evaluateFunc = fn(Arc<DAE::Exp>) -> Result<metamodelica::Real>;

        fn intWrapperFunc(mut e: Arc<DAE::Exp>) -> Result<metamodelica::Real> {
            let v = intReal(Expression::getEvaluatedConstInteger(e.clone())?);
            Ok(())
        }

        let mut linJac: Arc<LinearJacobian>;
        let mut eqn_index: i32 = 1;
        let mut var_index: i32 = 0;
        let mut constReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut row: Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>;
        let mut tmp_mat: Arc<metamodelica::List<Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>>> = metamodelica::nil();
        let mut tmp_rhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        let mut tmp_idx: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
        let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
        let mut index: (i32, i32);
        let mut scal_idx: i32 = 0;
        let mut var: BackendDAE::Var;
        let mut res: Arc<DAE::Exp>;
        let mut pDer: Arc<DAE::Exp>;
        let mut varRep: BackendVarTransform::VariableReplacements;
        let mut eFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<metamodelica::Real> + 'static> = if (Flags::getConfigBool(Flags::REAL_ASSC.clone())?) {Expression::getEvaluatedConstReal} else {intWrapperFunc};
        varRep = BackendVarTransform::emptyReplacements();
        for mut loopVar in &*loopVars.clone() {
            let mut loopVar = loopVar.clone();
            (var, _) = loopVar.clone();
            varRep = BackendVarTransform::addReplacement(varRep.clone(), BackendVariable::varCref(var.clone())?, Arc::new(DAE::Exp::ICONST { integer: 0 }), None)?;
        }
        for mut loopEq in &*loopEqs.clone() {
            let mut loopEq = loopEq.clone();
            row = UnorderedMap::new(fnptr!(Util::id, _), fnptr!(intEq, i32, i32), 1);
            (eqn, index) = loopEq.clone();
            res = BackendEquation::createResidualExp(eqn.clone())?;
            if '__try0: {
                for mut loopVar in &*loopVars.clone() {
                    let mut loopVar = loopVar.clone();
                    (var, var_index) = loopVar.clone();
                    pDer = unwrap_break_err!(Differentiate::differentiateExpSolve(res.clone(), BackendVariable::varCref(var.clone())?, None), '__try0);
                    (pDer, _) = unwrap_break_err!(ExpressionSimplify::simplify(pDer.clone()), '__try0);
                    constReal = unwrap_break_err!(eFunc(pDer.clone()), '__try0);
                    if !(realEq(constReal.clone(), metamodelica::OrderedFloat(0.0_f64))) {
                        unwrap_break_err!(UnorderedMap::add(var_index.clone(), constReal.clone(), row.clone()), '__try0);
                    }
                }
                (res, _) = unwrap_break_err!(BackendVarTransform::replaceExp(res.clone(), varRep.clone(), None), '__try0);
                tmp_mat = cons(row.clone(), tmp_mat.clone());
                tmp_rhs = cons(unwrap_break_err!(ExpressionSimplify::simplify(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::ICONST { integer: -1 }), operator: DAE::Operator::MUL { ty: DAE::T_UNKNOWN_DEFAULT.clone() }, exp2: res.clone() })), '__try0), tmp_rhs.clone());
                tmp_idx = cons(index.clone(), tmp_idx.clone());
                (_, scal_idx) = index.clone();
                eqn_index = eqn_index.clone() + 1;
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
        linJac = Arc::new(LinearJacobian { eq_marks: arrayCreate((tmp_mat.clone().len() as i32), false), ind: metamodelica::arrayFromVec(tmp_idx.clone().into_iter().cloned().collect()), rhs: metamodelica::arrayFromVec(tmp_rhs.clone().into_iter().cloned().collect()), rows: metamodelica::arrayFromVec(tmp_mat.clone().into_iter().cloned().collect()) });
        Ok(linJac)
    }

    pub fn emptyOrSingle(mut linJac: Arc<LinearJacobian>) -> bool {
        let mut empty: bool = (linJac.rows.clone().borrow().len() as i32) < 2 && (linJac.rhs.clone().borrow().len() as i32) < 2 && (linJac.ind.clone().borrow().len() as i32) < 2 && (linJac.eq_marks.clone().borrow().len() as i32) < 2;
        empty
    }

    pub fn solve(mut linJac: Arc<LinearJacobian>) -> Arc<LinearJacobian> {
        let mut linJac: Arc<LinearJacobian> = linJac;
        let mut col_index: i32 = 0;
        let mut piv_value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut row_value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let __range0 = 1..=(linJac.rows.clone().borrow().len() as i32);
        for mut i in __range0 {
            if '__try1: {
                (col_index, piv_value) = unwrap_break_err!(getPivot(linJac.rows.borrow()[(i.clone()-1) as usize].clone()), '__try1);
                let __range2 = i.clone() + 1..=(linJac.rows.clone().borrow().len() as i32);
                for mut j in __range2 {
                    row_value = UnorderedMap::getOrDefault(col_index.clone(), linJac.rows.borrow()[(j.clone()-1) as usize].clone(), metamodelica::OrderedFloat(0.0_f64));
                    if !(realEq(row_value.clone(), metamodelica::OrderedFloat(0.0_f64))) {
                        {
                            let __cell3 = true;
                            linJac.eq_marks.clone().borrow_mut()[(j.clone()-1) as usize] = __cell3;
                        }
                        unwrap_break_err!(solveRow(linJac.rows.borrow()[(i.clone()-1) as usize].clone(), linJac.rows.borrow()[(j.clone()-1) as usize].clone(), piv_value.clone(), row_value.clone()), '__try1);
                        {
                            let __cell4 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: linJac.rhs.borrow()[(j.clone()-1) as usize].clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: piv_value.clone() }) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: linJac.rhs.borrow()[(i.clone()-1) as usize].clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: row_value.clone() }) }) });
                            linJac.rhs.clone().borrow_mut()[(j.clone()-1) as usize] = __cell4;
                        }
                    }
                }
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
        linJac
    }

    pub fn solveRow(mut pivot_row: Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>, mut row: Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>, mut piv_value: metamodelica::Real, mut row_value: metamodelica::Real) -> Result<()> {
        let mut idx: i32 = 0;
        let mut val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut diag_val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        for mut idx in &*UnorderedMap::keyList(pivot_row.clone()) {
            let mut idx = idx.clone();
            let _ = (match (UnorderedMap::get(idx.clone(), row.clone()), UnorderedMap::get(idx.clone(), pivot_row.clone())) {
        (Some(mut val), Some(mut diag_val)) => {
            val = val.clone() * piv_value.clone() - diag_val.clone() * row_value.clone();
            if realAbs(val.clone()) < metamodelica::OrderedFloat(1e-12_f64) {
                UnorderedMap::remove(idx.clone(), row.clone())?;
            } else {
                UnorderedMap::add(idx.clone(), val.clone(), row.clone())?;
            }
            ()
        },
        (None, Some(mut diag_val)) => {
            UnorderedMap::add(idx.clone(), -(diag_val.clone() * row_value.clone()), row.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SymbolicJacobian.LinearJacobian.solveRow")); __mm_s.push_str(&*literal!(" key does not have an element in pivot row.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            ()
        },
    });
        }
        for mut idx in &*UnorderedMap::keyList(row.clone()) {
            let mut idx = idx.clone();
            let _ = (match (UnorderedMap::get(idx.clone(), row.clone()), UnorderedMap::get(idx.clone(), pivot_row.clone())) {
        (Some(mut val), None) => {
            val = val.clone() * piv_value.clone();
            UnorderedMap::add(idx.clone(), val.clone(), row.clone())?;
            ()
        },
        _ => (),
    });
        }
        Ok(())
    }

    pub fn updatePivotRow(mut pivot_row: Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>, mut piv_value: metamodelica::Real) -> Result<()> {
        let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        if !(realEq(piv_value.clone(), metamodelica::OrderedFloat(1.0_f64))) {
            for mut idx in &*UnorderedMap::keyList(pivot_row.clone()) {
                let mut idx = idx.clone();
                value = UnorderedMap::getOrFail(idx.clone(), pivot_row.clone());
                UnorderedMap::add(idx.clone(), value.clone() / piv_value.clone(), pivot_row.clone())?;
            }
        }
        Ok(())
    }

    fn getPivot(mut pivot_row: Arc<UnorderedMap::UnorderedMap<i32, metamodelica::Real>>) -> Result<(i32, metamodelica::Real)> {
        let mut idx: i32 = 0;
        let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        if Vector::isEmpty(pivot_row.keys.clone()) {
            bail!("fail");
        } else {
            idx = UnorderedMap::firstKey(pivot_row.clone());
            value = UnorderedMap::getOrFail(idx.clone(), pivot_row.clone());
        }
        Ok((idx, value))
    }

    pub fn resolveASSC(mut linJac: Arc<LinearJacobian>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut syst: Arc<BackendDAE::EqSystem>, mut init: bool) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>)> {
        let mut ass1: metamodelica::Array<i32> = ass1;
        let mut ass2: metamodelica::Array<i32> = ass2;
        let mut syst: Arc<BackendDAE::EqSystem> = syst;
        let mut i_arr: i32 = 0;
        let mut i_scal: i32 = 0;
        let mut lhs: Arc<DAE::Exp>;
        let mut rhs: Arc<DAE::Exp>;
        let mut newEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
        let mut updateList_arr: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut mapIncRowEqn: metamodelica::Array<i32>;
        let mut indexType: BackendDAE::IndexType = BackendDAE::IndexType::ABSOLUTE;
        let mut fullASSC: bool = Flags::getConfigBool(Flags::FULL_ASSC.clone())?;
        let __range0 = 1..=(linJac.rows.clone().borrow().len() as i32);
        for mut r in __range0 {
            if linJac.eq_marks.borrow()[(r.clone()-1) as usize].clone() && (UnorderedMap::isEmpty(linJac.rows.borrow()[(r.clone()-1) as usize].clone()) || fullASSC.clone()) {
                (i_arr, i_scal) = linJac.ind.borrow()[(r.clone()-1) as usize].clone();
                {
                    let __cell1 = -1;
                    ass2.clone().borrow_mut()[(ass1.borrow()[(i_scal.clone()-1) as usize].clone()-1) as usize] = __cell1;
                }
                {
                    let __cell2 = -1;
                    ass1.clone().borrow_mut()[(i_scal.clone()-1) as usize] = __cell2;
                }
                (rhs, _) = ExpressionSimplify::simplify(linJac.rhs.borrow()[(r.clone()-1) as usize].clone())?;
                lhs = generateLHSfromList(UnorderedMap::keyArray(linJac.rows.borrow()[(r.clone()-1) as usize].clone()), UnorderedMap::valueArray(linJac.rows.borrow()[(r.clone()-1) as usize].clone()), syst.orderedVars.clone())?;
                newEqn = BackendEquation::generateEquation(lhs.clone(), rhs.clone(), DAE::emptyElementSource.clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?;
                if Flags::isSet(Flags::DUMP_ASSC.clone())? || Flags::isSet(Flags::BLT_DUMP.clone())? && UnorderedMap::isEmpty(linJac.rows.borrow()[(r.clone()-1) as usize].clone()) {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[ASSC] The equation: ")); __mm_s.push_str(&*BackendDump::equationString(BackendEquation::get(syst.orderedEqs.clone(), i_arr.clone()))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[ASSC] Gets replaced by equation: ")); __mm_s.push_str(&*BackendDump::equationString(newEqn.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                assign_field!(syst.orderedEqs = BackendEquation::setAtIndex(syst.orderedEqs.clone(), i_arr.clone(), newEqn.clone())?);
                updateList_arr = cons(i_arr.clone(), updateList_arr.clone());
            }
        }
        if !(updateList_arr.clone().is_empty()) {
            match '__try3: {
                let Some((__pa4, __pa5, __pa6, true, _)) = (syst.mapping.clone()) else { break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                mapEqnIncRow = __pa4.clone();
                mapIncRowEqn = __pa5.clone();
                indexType = __pa6.clone();
                syst = unwrap_break_err!(BackendDAEUtil::updateAdjacencyMatrixScalar(syst.clone(), indexType.clone(), None, updateList_arr.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), false), '__try3);
                Ok::<_, anyhow::Error>((syst.clone(),))
            } {
                Ok((__try3_o0,)) => {
                    syst = __try3_o0;
                }
                Err(_) => {
                    syst = BackendDAEUtil::updateAdjacencyMatrix(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, None, updateList_arr.clone(), false)?;
                }
            }
        }
        if !(updateList_arr.clone().is_empty()) && !(Flags::isSet(Flags::DUMP_ASSC.clone())?) && Flags::isSet(Flags::BLT_DUMP.clone())? {
            println!("{}", (literal!("--- Some equations have been changed, for more information please use -d=dumpASSC.---\n\n")).clone());
        }
        Ok((ass1, ass2, syst))
    }

    fn generateLHSfromList(mut row_indices: metamodelica::Array<i32>, mut row_values: metamodelica::Array<metamodelica::Real>, mut vars: BackendDAE::Variables) -> Result<Arc<DAE::Exp>> {
        let mut lhs: Arc<DAE::Exp>;
        let mut length: i32 = (row_indices.clone().borrow().len() as i32);
        if length.clone() == 0 {
            lhs = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
        } else {
            lhs = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: row_values.borrow()[(1-1) as usize].clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: BackendVariable::varExp(BackendVariable::getVarAt(vars.clone(), row_indices.borrow()[(1-1) as usize].clone())?)? });
        }
        let __range0 = 2..=(row_indices.clone().borrow().len() as i32);
        for mut i in __range0 {
            lhs = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: row_values.borrow()[(i.clone()-1) as usize].clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: BackendVariable::varExp(BackendVariable::getVarAt(vars.clone(), row_indices.borrow()[(i.clone()-1) as usize].clone())?)? }) });
        }
        Ok(lhs)
    }

    pub fn anyChanges(mut linJac: Arc<LinearJacobian>) -> bool {
        let mut changed: bool = false;
        let __range0 = 1..=(linJac.eq_marks.clone().borrow().len() as i32);
        for mut i in __range0 {
            if linJac.eq_marks.borrow()[(i.clone()-1) as usize].clone() {
                changed = true;
                return changed;
            }
        }
        changed
    }

}

