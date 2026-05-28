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
use crate::BackendDAECreate;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendUtil;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::SymbolicJacobian::DAE_CJ;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::AvlSetPath;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::Inline;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTpl;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_susan::Tpl;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub const defaultMaxIter: i32 = 20;

// =============================================================================
// differentiation interfaces:
//  - createDifferentiatedCrefName
//  - createSeedCrefName
//  - differentiateEquation
//  - differentiateEquationTime
//  - differentiateExpCrefFullJacobian
//  - differentiateExpSolve
//  - differentiateExpTime
// =============================================================================
pub fn differentiateEquationTime(mut inEquation: Arc<BackendDAE::Equation>, mut inVariables: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Shared>)> {
    let mut outEquation: Option<Arc<BackendDAE::Equation>> = None;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut diffData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut knvars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrEqnStr((literal!("### differentiateEquationTime\n")).clone(), inEquation.clone(), (literal!(" w.r.t. time\n")).clone()), '__try0);
        }
        funcs = unwrap_break_err!(BackendDAEUtil::getFunctions(inShared.clone()), '__try0);
        knvars = unwrap_break_err!(BackendDAEUtil::getGlobalKnownVarsFromShared(inShared.clone()), '__try0);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        diffData.allVars = Some(inVariables.clone());
        (eqn, funcs) = unwrap_break_err!(differentiateEquation(inEquation.clone(), DAE::crefTime().clone(), diffData.clone(), crate::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, funcs.clone()), '__try0);
        outEquation = Some(eqn.clone());
        outShared = unwrap_break_err!(BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrEqnStr((literal!("### Result of differentiateEquationTime\n --> ")).clone(), eqn.clone(), (literal!("\n")).clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((outEquation.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outEquation = __try0_o0;
        }
        Err(_) => {
            source = BackendEquation::equationSource(inEquation.clone())?;
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate.differentiateEquationTime failed for ")); __mm_s.push_str(&*BackendDump::equationString(inEquation.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            outEquation = None;
        }
    }
    Ok((outEquation, outShared))
}

pub fn differentiateExpTime(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::Exp>, Arc<BackendDAE::Shared>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut dexp: Arc<DAE::Exp>;
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut diffData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
    let mut knvars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr((literal!("### differentiateExpTime\n ")).clone(), inExp.clone(), (literal!(" w.r.t. time\n")).clone()), '__try0);
        }
        funcs = unwrap_break_err!(BackendDAEUtil::getFunctions(inShared.clone()), '__try0);
        knvars = unwrap_break_err!(BackendDAEUtil::getGlobalKnownVarsFromShared(inShared.clone()), '__try0);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        (dexp, funcs) = unwrap_break_err!(differentiateExp(inExp.clone(), DAE::crefTime().clone(), diffData.clone(), crate::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, funcs.clone(), defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        outShared = unwrap_break_err!(BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr((literal!("### Result of differentiateExpTime\n --> ")).clone(), outExp.clone(), (literal!("n")).clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((dexp.clone(), diffData.clone(), funcs.clone(), knvars.clone(), outExp.clone(), outShared.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            dexp = __try0_o0;
            diffData = __try0_o1;
            funcs = __try0_o2;
            knvars = __try0_o3;
            outExp = __try0_o4;
            outShared = __try0_o5;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(ExpressionBasics::printExpStr(inExp.clone())?).clone(), (literal!("time")).clone()], metamodelica::sourceInfo!())?;
            }
            bail!("fail");
        }
    }
    Ok((outExp, outShared))
}

pub fn differentiateExpSolve(mut inExp: Arc<DAE::Exp>, mut inCref: Arc<DAE::ComponentRef>, mut functions: Option<Arc<AvlTreePathFunction::Tree>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut fac: Arc<metamodelica::List<Arc<DAE::Exp>>> = Expression::factors(inExp.clone())?;
    let mut dexp: Arc<DAE::Exp>;
    let mut fun: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    ::match_deref::match_deref! { match &(List::split1OnTrue(fac.clone(), (std::sync::Arc::new(Expression::expHasCrefInIf) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), inCref.clone())) {
        (Deref @ metamodelica::List::Nil, _) => (),
        _ => bail!("pattern mismatch"),
    } };
    match '__try0: {
        fun = (::match_deref::match_deref! { match &(functions.clone()) {
        Some(fun_) => {
            fun_.clone()
        },
        _ => {
            Arc::new(openmodelica_frontend_dump::AvlTreePathFunction::Tree::EMPTY)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStrCrefStr((literal!("### differentiateExpSolve\n ")).clone(), inExp.clone(), (literal!(" w.r.t. ")).clone(), inCref.clone(), (literal!("\n")).clone()), '__try0);
        }
        (dexp, _) = unwrap_break_err!(differentiateExp(inExp.clone(), inCref.clone(), BackendDAE::emptyInputData().clone(), crate::BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION, fun.clone(), defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr((literal!("### Result of differentiateExpSolve\n --> ")).clone(), outExp.clone(), (literal!("\n")).clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((dexp.clone(), fun.clone(), outExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            dexp = __try0_o0;
            fun = __try0_o1;
            outExp = __try0_o2;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(ExpressionBasics::printExpStr(inExp.clone())?).clone(), (ComponentReference::crefStr(inCref.clone())?).clone()], metamodelica::sourceInfo!())?;
            }
            bail!("fail");
        }
    }
    Ok(outExp)
}

pub fn differentiateExpCrefFullJacobian(mut inExp: Arc<DAE::Exp>, mut inCref: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::Exp>, Arc<BackendDAE::Shared>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut dexp: Arc<DAE::Exp>;
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut diffData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
    let mut knvars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    match '__try0: {
        funcs = unwrap_break_err!(BackendDAEUtil::getFunctions(inShared.clone()), '__try0);
        knvars = unwrap_break_err!(BackendDAEUtil::getGlobalKnownVarsFromShared(inShared.clone()), '__try0);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        (dexp, funcs) = unwrap_break_err!(differentiateExp(inExp.clone(), inCref.clone(), diffData.clone(), crate::BackendDAE::DifferentiationType::DIFF_FULL_JACOBIAN, funcs.clone(), defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        outShared = unwrap_break_err!(BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone()), '__try0);
        Ok::<_, anyhow::Error>((dexp.clone(), diffData.clone(), funcs.clone(), knvars.clone(), outExp.clone(), outShared.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            dexp = __try0_o0;
            diffData = __try0_o1;
            funcs = __try0_o2;
            knvars = __try0_o3;
            outExp = __try0_o4;
            outShared = __try0_o5;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(ExpressionBasics::printExpStr(inExp.clone())?).clone(), (ComponentReference::crefStr(inCref.clone())?).clone()], metamodelica::sourceInfo!())?;
            }
            bail!("fail");
        }
    }
    Ok((outExp, outShared))
}

// =============================================================================
// further interface functions to differentiation
//  - differentiateEquation
//  - differentiateBackendDAE
//
// =============================================================================
pub fn differentiateEquation(mut inEquation: Arc<BackendDAE::Equation>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<BackendDAE::Equation>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    if let Ok((__pa0, __pa1)) = differentiateEquationFragile(inEquation.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()) {
        outEquation = __pa0.clone();
        outFunctionTree = __pa1.clone();
    } else {
        Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(BackendDump::equationString(inEquation.clone())?).clone(), (ComponentReference::crefStr(inDiffwrtCref.clone())?).clone()], metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok((outEquation, outFunctionTree))
}

pub fn differentiateEquationFragile(mut inEquation: Arc<BackendDAE::Equation>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<BackendDAE::Equation>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        BackendDump::debugStrEqnStr((literal!("### differentiateEquation\n ")).clone(), inEquation.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReference::crefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    }
    (outEquation, outFunctionTree) = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: e2, exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut op2: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
            (e2_1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(list![op1.clone(), op2.clone()], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
            (Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1.clone(), scalar: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs.clone())
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: eqAttr, source, exp: e2, componentRef: cref } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut op2: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut source = (*source).clone();
            e1 = Expression::crefExp(cref.clone())?;
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
            (e2_1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(list![op1.clone(), op2.clone()], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
            (Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1.clone(), scalar: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs.clone())
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: eqAttr, source, exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            source = List::foldr(list![op1.clone()], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
            (Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1_1.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs.clone())
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: eqAttr, source, right: e2, left: e1, size } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut op2: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
            (e2_1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(list![op1.clone(), op2.clone()], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
            (Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs.clone())
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { recordSize, attr: eqAttr, source, right: e2, left: e1, dimSize } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut op2: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
            (e2_1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(list![op1.clone(), op2.clone()], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
            (Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), funcs.clone())
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr: eqAttr, expand, source, alg: Deref @ DAE::Algorithm { statementLst }, size } => {
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut alg: Arc<DAE::Algorithm> = Arc::new(<DAE::Algorithm as ::std::default::Default>::default());
            let mut statementLst = (*statementLst).clone();
            (statementLst, funcs) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), defaultMaxIter.clone())?;
            alg = Arc::new(DAE::Algorithm { statementLst: statementLst.clone() });
            (Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: expand.clone(), attr: eqAttr.clone() }), funcs.clone())
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { attr: eqAttr, source, eqnsfalse: eqns, eqnstrue: eqnslst, conditions: expExpLst } => {
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut eqns = (*eqns).clone();
            let mut eqnslst = (*eqnslst).clone();
            (eqnslst, funcs) = differentiateEquationsLst(eqnslst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone())?;
            (eqns, funcs) = differentiateEquations(eqns.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), funcs.clone())?;
            (Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: expExpLst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs.clone())
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: eqAttr, source, whenEquation: whenEqn, size } => {
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut whenEqn = (*whenEqn).clone();
            (whenEqn, funcs) = differentiateWhenEquations(whenEqn.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
            (Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqn.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs.clone())
        },
        _ => {
            Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(BackendDump::equationString(inEquation.clone())?).clone(), (ComponentReference::crefStr(inDiffwrtCref.clone())?).clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        BackendDump::debugStrEqnStr((literal!("### Result of differentiateEquation\n --> ")).clone(), outEquation.clone(), (literal!("\n")).clone())?;
    }
    Ok((outEquation, outFunctionTree))
}

fn differentiateEquations(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inEquationsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outEquations, outFunctionTree) = 'mc: {
        let __mc_input = inEquations.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inEquationsAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqn, tail: rest } => {
                    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut eqn = (*eqn).clone();
                    (eqn, funcs) = differentiateEquation(eqn.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    eqns = cons(eqn.clone(), inEquationsAccum.clone());
                    (eqns, funcs) = differentiateEquations(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), eqns.clone(), funcs.clone())?;
                    Ok((eqns.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqn, tail: _ } => {
                    Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(BackendDump::equationString(eqn.clone())?).clone(), (ComponentReference::crefStr(inDiffwrtCref.clone())?).clone()], metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquations, outFunctionTree))
}

fn differentiateEquationsLst(mut inEquationsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inEquationsLstAccum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outEquationsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outEquationsLst, outFunctionTree) = 'mc: {
        let __mc_input = inEquationsLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inEquationsLstAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqns, tail: rest } => {
                    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut eqnsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
                    let mut eqns = (*eqns).clone();
                    (eqns, funcs) = differentiateEquations(eqns.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone())?;
                    eqnsLst = cons(eqns.clone(), inEquationsLstAccum.clone());
                    (eqnsLst, funcs) = differentiateEquationsLst(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), eqnsLst.clone(), funcs.clone())?;
                    Ok((eqnsLst.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqns, tail: _ } => {
                    Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(BackendDump::equationListString(eqns.clone(), (literal!("equation list")).clone())).clone(), (ComponentReference::crefStr(inDiffwrtCref.clone())?).clone()], metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquationsLst, outFunctionTree))
}

fn differentiateWhenEquations(mut inWhenEquations: Arc<BackendDAE::WhenEquation>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<BackendDAE::WhenEquation>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outWhenEquations: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut elsewhenPart: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
    let mut delsewhenPart: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
    let mut oelsepart: Option<Arc<BackendDAE::WhenEquation>> = None;
    let mut whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut stmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut condition: Arc<DAE::Exp>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inWhenEquations.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: __pa0, whenStmtLst: __pa1, condition: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    oelsepart = __pa0.clone();
    whenStmtLst = __pa1.clone();
    condition = __pa2.clone();
    funcs = inFunctionTree.clone();
    stmtLst = metamodelica::nil();
    for mut rs in &*whenStmtLst.clone() {
        let mut rs = rs.clone();
        rs = (match rs.clone() {
        BackendDAE::WhenOperator::ASSIGN { left: ref eleft, right: mut right, source: ref src } => {
            let mut dright: Arc<DAE::Exp>;
            let mut dleft: Arc<DAE::Exp>;
            (dleft, funcs) = differentiateExp(eleft.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), defaultMaxIter.clone())?;
            (dright, funcs) = differentiateExp(right.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), defaultMaxIter.clone())?;
            BackendDAE::WhenOperator::ASSIGN { left: dleft.clone(), right: dright.clone(), source: src.clone() }
        },
        _ => {
            rs.clone()
        },
    });
        stmtLst = cons(rs.clone(), stmtLst.clone());
    }
    if isSome(oelsepart.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(oelsepart.clone()) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        elsewhenPart = __pa3.clone();
        (delsewhenPart, funcs) = differentiateWhenEquations(elsewhenPart.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone())?;
        oelsepart = Some(delsewhenPart.clone());
    } else {
        oelsepart = None;
    }
    outWhenEquations = Arc::new(BackendDAE::WhenEquation { condition: condition.clone(), whenStmtLst: stmtLst.clone(), elsewhenPart: oelsepart.clone() });
    outFunctionTree = funcs.clone();
    Ok((outWhenEquations, outFunctionTree))
}

// =============================================================================
// main differentiation functions
//  - differentiateExp
//  - differentiateStatements
//
// =============================================================================
fn differentiateExp(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let debug: bool = false;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate Exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::SCONST { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::CLKCONST { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            (Arc::new(DAE::Exp::ICONST { integer: 0 }), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            (Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::CREF { ty: tp, componentRef: cref } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            if ComponentReference::isStartCref(cref.clone()) {
                res = Expression::makeConstZero(tp.clone());
                functionTree = inFunctionTree.clone();
            } else {
                (res, functionTree) = differentiateCrefs(inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            }
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::BINARY { .. } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res, functionTree) = differentiateBinary(inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            res = ExpressionSimplify::simplifyBinaryExp(res.clone())?;
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::UNARY { exp: e1, operator: op } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            res = Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: res.clone() });
            res = ExpressionSimplify::simplifyUnaryExp(res.clone())?;
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::LBINARY { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::LUNARY { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::RELATION { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 } => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, functionTree) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            (res2, functionTree) = differentiateExp(e3.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functionTree.clone(), maxIter.clone() - 1)?;
            res = Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: res1.clone(), expElse: res2.clone() });
            (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: actual, tail: Deref @ metamodelica::List::Cons { head: simplified, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            let mut lambda: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            lambda = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::homotopyLambda)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            (e1, functionTree) = differentiateExp(actual.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (e2, functionTree) = differentiateExp(simplified.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functionTree.clone(), maxIter.clone())?;
            e3 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: lambda.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e1.clone() }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lambda.clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }) });
            (e3.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. } if (Expression::expHasCref(e2.clone(), inDiffwrtCref.clone())? || Expression::expHasCref(e3.clone(), inDiffwrtCref.clone())?) => {
            bail!("fail")
        },
        Deref @ DAE::Exp::CALL { .. } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res, functionTree) = differentiateCalls(inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::RECORD { ty: tp, comp: strLst, exps: expl, path: p } => {
            let mut e1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut sub: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            sub = metamodelica::nil();
            functionTree = inFunctionTree.clone();
            for mut e in &*expl.clone() {
                let mut e = e.clone();
                (e1, functionTree) = differentiateExp(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functionTree.clone(), maxIter.clone())?;
                sub = cons(e1.clone(), sub.clone());
            }
            (Arc::new(DAE::Exp::RECORD { path: p.clone(), exps: sub.clone().reverse(), comp: strLst.clone(), ty: tp.clone() }), functionTree.clone())
        },
        Deref @ DAE::Exp::ARRAY { array: expl, scalar: b, ty: tp } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut expl = (*expl).clone();
            (expl, functionTree) = List::map3Fold(expl.clone(), Arc::new({ let __pe_b5 = maxIter.clone() - 1; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
            res = Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: b.clone(), array: expl.clone() });
            (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::MATRIX { matrix, integer: i, ty: tp } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut dmatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            (dmatrix, functionTree) = List::mapFoldList(matrix.clone(), Arc::new({ let __pe_b1 = inDiffwrtCref.clone(); let __pe_b2 = inInputData.clone(); let __pe_b3 = inDiffType.clone(); let __pe_b5 = maxIter.clone() - 1; move |__pe_a0, __pe_a4| differentiateExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4, __pe_b5.clone()) }), inFunctionTree.clone());
            res = Arc::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: i.clone(), matrix: dmatrix.clone() });
            (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::RANGE { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::TUPLE { PR: expl } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut expl = (*expl).clone();
            (expl, functionTree) = List::map3Fold(expl.clone(), Arc::new({ let __pe_b5 = maxIter.clone() - 1; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
            res = Arc::new(DAE::Exp::TUPLE { PR: expl.clone() });
            (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::CAST { exp: e1, ty: tp } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            (Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: res.clone() }), functionTree.clone())
        },
        Deref @ DAE::Exp::ASUB { sub: subs, exp: e1 } => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            res = Expression::makeASUB(res1.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
            (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::TSUB { ty: tp, ix: i, exp: e1 } => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            if !(referenceEq(&e1.clone(),&res1.clone())) {
                res = Arc::new(DAE::Exp::TSUB { exp: res1.clone(), ix: i.clone(), ty: tp.clone() });
                (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            } else {
                res = inExp.clone();
            }
            (res.clone(), functionTree.clone())
        },
        e1 @ Deref @ DAE::Exp::RSUB { .. } => {
            let mut p1: Arc<Absyn::Path>;
            let mut p2: Arc<Absyn::Path>;
            let mut b: bool = false;
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut strLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut e1 = (*e1).clone();
            (res, b) = ExpressionSimplify::simplify(e1.clone())?;
            if b.clone() {
                (res, functionTree) = differentiateExp(res.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            } else {
                (res1, functionTree) = differentiateExp(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
                if !(referenceEq(&var_field!((*e1).exp, DAE::Exp::RSUB).clone(),&res1.clone())) {
                    match '__try0: {
                        (expl, strLst) = (::match_deref::match_deref! { match &(res1.clone()) {
        Deref @ DAE::Exp::RECORD { comp: strLst, exps: expl, .. } => (expl.clone(), strLst.clone()),
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. }, expLst: expl, path: p1 } if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) => (expl.clone(), {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        _ => bail!("match: no arm matched"),
    } });
                        res = unwrap_break_err!((expl.clone()).get(List::position1OnTrue(strLst.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (var_field!((*e1).fieldName, DAE::Exp::RSUB).clone()).clone())), '__try0);
                        Ok::<_, anyhow::Error>((res.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            res = __try0_o0;
                        }
                        Err(_) => {
                            assign_variant_field!(e1 => DAE::Exp::RSUB; exp = res1.clone());
                            (res, _) = ExpressionSimplify::simplify1(e1.clone())?;
                        }
                    }
                }
            }
            (res.clone(), functionTree.clone())
        },
        Deref @ DAE::Exp::SIZE { .. } => {
            (inExp.clone(), inFunctionTree.clone())
        },
        Deref @ DAE::Exp::REDUCTION { .. } => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, functionTree) = differentiateExp(var_field!((*inExp).expr, DAE::Exp::REDUCTION).clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone() - 1)?;
            if !(referenceEq(&var_field!((*inExp).expr, DAE::Exp::REDUCTION).clone(),&res1.clone())) {
                res = Arc::new(DAE::Exp::REDUCTION { reductionInfo: var_field!((*inExp).reductionInfo, DAE::Exp::REDUCTION).clone(), expr: res1.clone(), iterators: var_field!((*inExp).iterators, DAE::Exp::REDUCTION).clone() });
                (res, _) = ExpressionSimplify::simplify1(res.clone())?;
            } else {
                res = inExp.clone();
            }
            (res.clone(), functionTree.clone())
        },
        _ => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut stp: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            s1 = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?).clone();
            stp = (TypesDump::printTypeStr(Expression::r#typeof(inExp.clone())?)?).clone();
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- differentiateExp ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*stp.clone()); __mm_s.push_str(&*literal!(" w.r.t ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate-Exp-result: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateStatements(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inStmtsAccum: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outDiffedStmts, outFunctionTree) = 'mc: {
        let __mc_input = inStmts.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inStmtsAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_ASSIGN { source, exp: rhs, exp1: lhs, type_ }, tail: restStatements } => {
                    let mut derivedLHS: Arc<DAE::Exp>;
                    let mut derivedRHS: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedLHS, functions) = differentiateExp(lhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    (derivedRHS, functions) = differentiateExp(rhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), maxIter.clone())?;
                    (derivedRHS, _) = ExpressionSimplify::simplify(derivedRHS.clone())?;
                    if Expression::isZero(derivedLHS.clone()) {
                        derivedStatements1 = list![currStatement.clone()];
                    } else {
                        derivedStatements1 = list![Arc::new(DAE::Statement::STMT_ASSIGN { type_: type_.clone(), exp1: derivedLHS.clone(), exp: derivedRHS.clone(), source: source.clone() }), currStatement.clone()];
                    }
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, exp: rhs, expExpLst: expLst, .. }, tail: restStatements } => {
                    let mut derivedRHS: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expLstRHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exptl: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut optDerivedStatements1: Arc<metamodelica::List<Option<Arc<DAE::Statement>>>> = metamodelica::nil();
                    (dexpLst, functions) = List::map3Fold(expLst.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
                    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(differentiateExp(rhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), maxIter.clone())?) {
                        (__pa1 @ Deref @ DAE::Exp::TUPLE { PR: __pa0 }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLstRHS = __pa0.clone();
                    derivedRHS = __pa1.clone();
                    functions = __pa2.clone();
                    let __pa3 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(derivedRHS.clone())?) {
                        (Deref @ DAE::Exp::TUPLE { PR: __pa3 }, _) => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLstRHS = __pa3.clone();
                    exptl = List::zip(dexpLst.clone(), expLstRHS.clone());
                    optDerivedStatements1 = List::map2(exptl.clone(), (std::sync::Arc::new(makeAssignmentfromTuple) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::Exp>), Arc<DAE::ElementSource>, Arc<AvlTreePathFunction::Tree>) -> Result<Option<Arc<DAE::Statement>>> + 'static>), source.clone(), inFunctionTree.clone());
                    derivedStatements1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for mut s in (optDerivedStatements1.clone()).into_iter().cloned() {
                    if !(isSome(s.clone())) { continue; }
                    let __x = Util::getOption(s.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    derivedStatements2 = listAppend(derivedStatements1.clone(), list![currStatement.clone()]);
                    derivedStatements1 = listAppend(derivedStatements2.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, type_, exp: rhs @ Deref @ DAE::Exp::CALL { .. }, expExpLst: expLst }, tail: restStatements } => {
                    let mut derivedRHS: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut optDerivedStatements1: Arc<metamodelica::List<Option<Arc<DAE::Statement>>>> = metamodelica::nil();
                    let mut type_ = (*type_).clone();
                    (dexpLst, functions) = List::map3Fold(expLst.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
                    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(differentiateExp(rhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), maxIter.clone())?) {
                        (__pa1 @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: __pa0, .. }, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    type_ = __pa0.clone();
                    derivedRHS = __pa1.clone();
                    functions = __pa2.clone();
                    optDerivedStatements1 = list![Some(Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: type_.clone(), expExpLst: dexpLst.clone(), exp: derivedRHS.clone(), source: source.clone() }))];
                    derivedStatements1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for mut s in (optDerivedStatements1.clone()).into_iter().cloned() {
                    if !(isSome(s.clone())) { continue; }
                    let __x = Util::getOption(s.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    derivedStatements2 = listAppend(derivedStatements1.clone(), list![currStatement.clone()]);
                    derivedStatements1 = listAppend(derivedStatements2.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, type_, exp: rhs, lhs }, tail: restStatements } => {
                    let mut derivedLHS: Arc<DAE::Exp>;
                    let mut derivedRHS: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedLHS, functions) = differentiateExp(lhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    (derivedRHS, functions) = differentiateExp(rhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), maxIter.clone())?;
                    (derivedRHS, _) = ExpressionSimplify::simplify(derivedRHS.clone())?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: derivedLHS.clone(), exp: derivedRHS.clone(), source: source.clone() }), currStatement.clone()];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_FOR { source, statementLst, range: exp, iter: ident, iterIsArray, type_ }, tail: restStatements } => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut inputData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
                    let mut controlVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    cref = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
                    controlVar = BackendDAE::Var { varName: cref.clone(), varKind: crate::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    inputData = addGlobalVars(list![controlVar.clone()], inInputData.clone())?;
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter.clone())?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_FOR { type_: type_.clone(), iterIsArray: iterIsArray.clone(), iter: (ident.clone()).clone(), range: exp.clone(), statementLst: derivedStatements1.clone(), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { source, else_: Deref @ DAE::Else::NOELSE, statementLst, exp }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter.clone())?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { source, else_: Deref @ DAE::Else::ELSEIF { else_: elseif_else_, statementLst: elseif_statementLst, exp: elseif_exp }, statementLst, exp }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter.clone())?;
                    (derivedStatements2, functions) = differentiateStatements(list![Arc::new(DAE::Statement::STMT_IF { exp: elseif_exp.clone(), statementLst: elseif_statementLst.clone(), else_: elseif_else_.clone(), source: source.clone() })], inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), functions.clone(), maxIter.clone())?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: Arc::new(DAE::Else::ELSE { statementLst: derivedStatements2.clone() }), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { source, else_: Deref @ DAE::Else::ELSE { statementLst: else_statementLst }, statementLst, exp }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter.clone())?;
                    (derivedStatements2, functions) = differentiateStatements(else_statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), functions.clone(), maxIter.clone())?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: Arc::new(DAE::Else::ELSE { statementLst: derivedStatements2.clone() }), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHILE { source, statementLst, exp }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter.clone())?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_WHILE { exp: exp.clone(), statementLst: derivedStatements1.clone(), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: None, statementLst, initialCall, exp, .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter.clone())?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_WHEN { exp: exp.clone(), conditions: metamodelica::nil(), initialCall: initialCall.clone(), statementLst: derivedStatements1.clone(), elseWhen: None, source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: Some(stmt), statementLst, initialCall, exp, .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut dstmt: Arc<DAE::Statement>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateStatements(list![stmt.clone()], inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), functions.clone(), maxIter.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    dstmt = __pa0.clone();
                    functions = __pa1.clone();
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_WHEN { exp: exp.clone(), conditions: metamodelica::nil(), initialCall: initialCall.clone(), statementLst: derivedStatements1.clone(), elseWhen: Some(dstmt.clone()), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inStmtsAccum.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TERMINATE { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    derivedStatements1 = cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_REINIT { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    derivedStatements1 = cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_NORETCALL { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    derivedStatements1 = cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_RETURN { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    derivedStatements1 = cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_BREAK { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    derivedStatements1 = cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut currStatement: Arc<DAE::Statement>;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        let __pa0 = ::match_deref::match_deref! { match &(inStmts.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        currStatement = __pa0.clone();
                        s1 = (DAEDump::ppStatementStr(currStatement.clone())?).clone();
                        s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?).clone();
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- differentiateStatements ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" w.r.t: ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDiffedStmts, outFunctionTree))
}

fn isDiscreteAssignStatment(mut inStmt: Arc<DAE::Statement>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(inStmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn makeAssignmentfromTuple(mut inTpl: (Arc<DAE::Exp>, Arc<DAE::Exp>), mut source: Arc<DAE::ElementSource>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<Option<Arc<DAE::Statement>>> {
    let mut outStmt: Option<Arc<DAE::Statement>> = None;
    outStmt = (::match_deref::match_deref! { match &(inTpl.clone()) {
        (e1 @ Deref @ DAE::Exp::CREF { ty: tp, .. }, e2) => {
            Some(Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e1.clone(), exp: e2.clone(), source: source.clone() }))
        },
        (e1 @ Deref @ DAE::Exp::CALL { .. }, e2) if (Expression::isRecordCall(e1.clone(), inFunctionTree.clone())?) => {
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(e1.clone())?;
            Some(Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e1.clone(), exp: e2.clone(), source: source.clone() }))
        },
        (e1, e2) if (Expression::isZero(e1.clone())) => {
            let _ = Expression::r#typeof(e2.clone())?;
            None
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStmt)
}

// =============================================================================
// help functions for differentiation
//  - differentiateCrefs
//  - differentiateCalls
//  - differentiateBinary (e.g.: ADD, SUB, MUL, DIV, POW, ...
//
// =============================================================================
fn differentiateCrefs(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let debug: bool = false;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate Exp-Cref: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (outDiffedExp, outFunctionTree) = ({
        let mut diffed_exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        'mc: {
        let __mc_input = (inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, componentRef: cr }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), cr.clone())?;
                    cr = ComponentReference::prependStringCref((matrixName.clone()).clone(), cr.clone())?;
                    res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst, .. }, componentRef: cr }, _, _, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    expl = List::map1(varLst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone());
                    (expl_1, outFunctionTree) = List::map3Fold(expl.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
                    res = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expl_1.clone(), attr: Arc::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
                    Ok((res.clone(), outFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp @ Deref @ DAE::Type::T_ARRAY { .. }, componentRef: cr }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), cr.clone())?;
                    cr = ComponentReference::prependStringCref((matrixName.clone()).clone(), cr.clone())?;
                    res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, _, _, diffType, _) => {
                    if !(((match diffType.clone() {
        BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. } => false,
        _ => true,
    }))) { bail!("guard") }
                    let mut e1: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    let true = (Flags::isSet(Flags::NF_SCALARIZE.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (res, outFunctionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    Ok((res.clone(), outFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _, _, _, _) => {
                    Ok((e.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, _, _, _) => {
                    let mut one: Arc<DAE::Exp>;
                    let true = (ComponentReferenceBasics::crefEqual(cr.clone(), inDiffwrtCref.clone())?) else { bail!("pattern mismatch") };
                    (one, _) = Expression::makeOneExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((one.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION, _) => {
                    let mut zero: Arc<DAE::Exp>;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::DIFF_FULL_JACOBIAN, _) => {
                    let mut zero: Arc<DAE::Exp>;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { knownVars: Some(knvars), .. }, _, _) => {
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut zero: Arc<DAE::Exp>;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), knvars.clone())?;
                    let false = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { allVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, _) => {
                    let mut kind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
                    let mut zero: Arc<DAE::Exp>;
                    let (BackendDAE::VAR { varKind: __pa0, .. }, _) = (BackendVariable::getVarSingle(cr.clone(), timevars.clone())?) else { bail!("pattern mismatch") };
                    kind = __pa0.clone();
                    let true = (listMember(kind.clone(), list![crate::BackendDAE::VarKind::DISCRETE]) || !(Types::isReal(tp.clone()))) else { bail!("pattern mismatch") };
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, _) => {
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), timevars.clone())?;
                    let true = (BackendVariable::isDummyStateVar(var.clone())) else { bail!("pattern mismatch") };
                    cr = ComponentReference::crefPrefixDer(cr.clone());
                    res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, _) => {
                    let mut res: Arc<DAE::Exp>;
                    (_, _) = BackendVariable::getVarSingle(cr.clone(), timevars.clone())?;
                    res = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![e.clone()], attr: Arc::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, _) => {
                    let mut zero: Arc<DAE::Exp>;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cr1 = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    (_, _) = BackendVariable::getVar(cr1.clone(), timevars.clone())?;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, _) => {
                    let mut zero: Arc<DAE::Exp>;
                    (_, _) = BackendVariable::getVar(cr.clone(), timevars.clone())?;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), cr.clone())?;
                    cr = ComponentReference::prependStringCref((matrixName.clone()).clone(), cr.clone())?;
                    res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }, _) => {
                    let mut res: Arc<DAE::Exp>;
                    (res, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), independenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }, _) => {
                    let mut scalarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut arrayType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut res: Arc<DAE::Exp>;
                    let mut res1: Arc<DAE::Exp>;
                    let mut scalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    (scalarLst, _) = BackendVariable::getVar(cr.clone(), timevars.clone())?;
                    arrayType = ComponentReference::crefTypeFull(cr.clone())?;
                    if !(scalarLst.clone().is_empty()) && (scalarLst.clone().len() as i32) != Types::getDimensionProduct(arrayType.clone())? {
                        scalarCrefs = ComponentReference::expandCref(cr.clone(), true)?;
                        outFunctionTree = inFunctionTree.clone();
                        for mut cref in &*scalarCrefs.clone() {
                            let mut cref = cref.clone();
                            (res1, outFunctionTree) = differentiateCrefs(Expression::crefExp(cref.clone())?, inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), outFunctionTree.clone(), maxIter.clone())?;
                            diffed_exps = cons(res1.clone(), diffed_exps.clone());
                        }
                        res = Expression::listToArray(diffed_exps.clone().reverse(), TypesDump::getDimensions(arrayType.clone()))?;
                    } else {
                        cr = createSeedCrefName(cr.clone(), (matrixName.clone()).clone())?;
                        res = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    }
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), allVars: Some(timevars), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }, _) => {
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), timevars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    let false = (BackendVariable::isStateVar(var.clone())) else { bail!("pattern mismatch") };
                    cr = ComponentReference::createDifferentiatedCrefName(cr.clone(), inDiffwrtCref.clone(), (matrixName.clone()).clone())?;
                    res = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }, _) => {
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), timevars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    let false = (BackendVariable::isStateVar(var.clone())) else { bail!("pattern mismatch") };
                    cr = ComponentReference::createDifferentiatedCrefName(cr.clone(), inDiffwrtCref.clone(), (matrixName.clone()).clone())?;
                    res = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }, _) => {
                    let mut zero: Arc<DAE::Exp>;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, _) => {
                    let mut zero: Arc<DAE::Exp>;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut serr: ArcStr = arcstr::literal!("");
                    let mut se1: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    s1 = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                    se1 = (TypesDump::printTypeStr(Expression::r#typeof(inExp.clone())?)?).clone();
                    s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?).clone();
                    serr = stringAppendList(list![(literal!("\n- differentiateCrefs ")).clone(), (s1.clone()).clone(), (literal!(" type:")).clone(), (se1.clone()).clone(), (literal!(" w.r.t: ")).clone(), (s2.clone()).clone(), (literal!(" failed\n")).clone()]);
                    Debug::trace((serr.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate-ExpCref-result: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((outDiffedExp, outFunctionTree))
}

pub fn createDiffedCrefName(mut inCref: Arc<DAE::ComponentRef>, mut inMatrixName: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    subs = ComponentReference::crefLastSubs(inCref.clone())?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?;
    outCref = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), outCref.clone())?;
    outCref = ComponentReference::prependStringCref((inMatrixName.clone()).clone(), outCref.clone())?;
    outCref = ComponentReference::crefSetLastSubs(outCref.clone(), subs.clone())?;
    outCref = ComponentReference::crefSetLastType(outCref.clone(), ComponentReference::crefLastType(inCref.clone())?)?;
    Ok(outCref)
}

pub fn createSeedCrefName(mut inCref: Arc<DAE::ComponentRef>, mut inMatrixName: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let debug: bool = false;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("inCref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after full type  ")); __mm_s.push_str(&*TypesDump::printTypeStr(ComponentReference::crefTypeConsiderSubs(inCref.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    subs = ComponentReference::crefLastSubs(inCref.clone())?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?;
    outCref = ComponentReference::crefSetLastType(outCref.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
    outCref = ComponentReference::joinCrefs(outCref.clone(), ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Seed")); __mm_s.push_str(&*inMatrixName.clone()); ArcStr::from(__mm_s) }).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after join: ")); __mm_s.push_str(&*ComponentReference::printComponentRefListStr(ComponentReference::expandCref(outCref.clone(), true)?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    outCref = ComponentReference::crefSetLastSubs(outCref.clone(), subs.clone())?;
    outCref = ComponentReference::crefSetLastType(outCref.clone(), ComponentReference::crefLastType(inCref.clone())?)?;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("outCref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(outCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(outCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isSeedCref(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => StringUtil::startsWith((var_field!((*cr).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (literal!("Seed")).clone()),
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => isSeedCref(var_field!((*cr).componentRef, DAE::ComponentRef::CREF_QUAL).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn differentiateCalls(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let debug: bool = false;
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate Exp-Call: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())) {
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: actual, tail: Deref @ metamodelica::List::Cons { head: simplified, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, _, _, _, _) => {
            let mut e1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (e1, funcs) = differentiateExp(actual.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (_, funcs) = differentiateExp(simplified.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (e1.clone(), funcs.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), independenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }, _) => {
            let mut res: Arc<DAE::Exp>;
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::previousNamePrefix)).clone(), tp.clone(), metamodelica::nil(), cr.clone());
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), timevars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (),
                _ => bail!("pattern mismatch"),
            } };
            cr = createSeedCrefName(cr.clone(), (matrixName.clone()).clone())?;
            res = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
            (res.clone(), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "der" } }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, _) => {
            (Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![e.clone(), Arc::new(DAE::Exp::ICONST { integer: 2 })], attr: attr.clone() }), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Nil } }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "der" } }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, _) => {
            let mut i = (*i).clone();
            i = i.clone() + 1;
            (Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![e.clone(), Arc::new(DAE::Exp::ICONST { integer: i.clone() })], attr: attr.clone() }), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: true }, _) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cj: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut res: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            cj = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (arcstr::literal!(DAE_CJ)).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            cr = Expression::expCref(e.clone())?;
            tp = Expression::r#typeof(e.clone())?;
            cr = createSeedCrefName(cr.clone(), (matrixName.clone()).clone())?;
            res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
            res = Arc::new(DAE::Exp::BINARY { exp1: Expression::makeCrefExp(cj.clone(), DAE::T_REAL_DEFAULT().clone())?, operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: res.clone() });
            (res.clone(), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, _, _) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut res: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            cr = Expression::expCref(e.clone())?;
            tp = Expression::r#typeof(e.clone())?;
            cr = ComponentReference::crefPrefixDer(cr.clone());
            cr = ComponentReference::createDifferentiatedCrefName(cr.clone(), inDiffwrtCref.clone(), (matrixName.clone()).clone())?;
            res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
            if ComponentReferenceBasics::crefEqual(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), inDiffwrtCref.clone())? {
                (res, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            }
            (res.clone(), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, _, _) => {
            let mut zero: Arc<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(Expression::r#typeof(e.clone())?))?;
            (zero.clone(), inFunctionTree.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, ty: tp, .. }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, _, _) if (!(Expression::isRecordCall(e.clone(), inFunctionTree.clone())?)) => {
            let mut zero: Arc<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (zero.clone(), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, path: Deref @ Absyn::Path::IDENT { name } }, _, _, _, _) => {
            let mut res: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res, funcs) = differentiateCallExp1Arg((name.clone()).clone(), e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (res.clone(), funcs.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::RCONST { real: __rlit_0 }, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" } }, _, _, _, _) if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: expl @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, attr: attr @ Deref @ DAE::CallAttributes { builtin: true, .. }, path: Deref @ Absyn::Path::IDENT { name } }, _, _, _, _) => {
            let mut res: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res, funcs) = differentiateCallExpNArg((name.clone()).clone(), expl.clone(), attr.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (res.clone(), funcs.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { .. }, _, _, _, _) => {
            let mut e1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (e1, funcs) = differentiateFunctionCall(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (e1, _, _, _) = Inline::inlineExp(e1.clone(), (Some(funcs.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE]), DAE::emptyElementSource().clone())?;
            (e1.clone(), funcs.clone())
        },
        _ => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut serr: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            s1 = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?).clone();
            serr = stringAppendList(list![(literal!("\n- Function differentiateCalls failed. differentiateExp ")).clone(), (s1.clone()).clone(), (literal!(" w.r.t: ")).clone(), (s2.clone()).clone(), (literal!(" failed\n")).clone()]);
            Debug::trace((serr.clone()).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate-ExpCall-result: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateCallExp1Arg(mut name: ArcStr, mut exp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFuncs: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((name.clone(), exp.clone())) {
        (Deref @ "pre", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1.clone(), inFuncs.clone())
        },
        (Deref @ "previous", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1.clone(), inFuncs.clone())
        },
        (Deref @ "$getPart", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            (exp_1.clone(), funcs.clone())
        },
        (Deref @ "firstTick", _) => {
            (exp.clone(), inFuncs.clone())
        },
        (Deref @ "interval", _) => {
            (exp.clone(), inFuncs.clone())
        },
        (Deref @ "sin", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("cos")).clone(), list![exp.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1.clone() }), funcs.clone())
        },
        (Deref @ "cos", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sin")).clone(), list![exp.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: exp_2.clone() }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1.clone() }), funcs.clone())
        },
        (Deref @ "tan", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("cos")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp.clone() })], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1.clone() }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp_2.clone(), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }) }), funcs.clone())
        },
        (Deref @ "asin", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp.clone() }) })], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: exp_2.clone() }), funcs.clone())
        },
        (Deref @ "acos", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp.clone() }) })], tp.clone());
            (Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: exp_2.clone() }) }), funcs.clone())
        },
        (Deref @ "atan", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp.clone() }) }) }), funcs.clone())
        },
        (Deref @ "sinh", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("cosh")).clone(), list![exp.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_2.clone() }), funcs.clone())
        },
        (Deref @ "cosh", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sinh")).clone(), list![exp.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_2.clone() }), funcs.clone())
        },
        (Deref @ "tanh", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("cosh")).clone(), list![exp.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp_2.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) }) }), funcs.clone())
        },
        (Deref @ "exp", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("exp")).clone(), list![exp.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1.clone() }), funcs.clone())
        },
        (Deref @ "log", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: exp.clone() }), funcs.clone())
        },
        (Deref @ "log10", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) })], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_2.clone() }) }), funcs.clone())
        },
        (Deref @ "sqrt", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![exp.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_2.clone() }) }), funcs.clone())
        },
        (Deref @ "abs", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sign")).clone(), list![exp.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1.clone() }), funcs.clone())
        },
        (Deref @ "sign", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1.clone(), inFuncs.clone())
        },
        (Deref @ "transpose", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("transpose")).clone(), list![exp_1.clone()], tp.clone());
            (exp_2.clone(), funcs.clone())
        },
        (Deref @ "sum", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sum")).clone(), list![exp_1.clone()], tp.clone());
            (exp_2.clone(), funcs.clone())
        },
        (Deref @ "max", Deref @ DAE::Exp::ARRAY { ty: tp, array: expl, .. }) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp = (*tp).clone();
            tp = Types::arrayElementType(tp.clone());
            exp_1 = createFromNCall2ArgsCall((literal!("max")).clone(), expl.clone(), tp.clone())?;
            (exp_2, funcs) = differentiateExp(exp_1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            (exp_2.clone(), funcs.clone())
        },
        (Deref @ "min", Deref @ DAE::Exp::ARRAY { ty: tp, array: expl, .. }) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp = (*tp).clone();
            tp = Types::arrayElementType(tp.clone());
            exp_1 = createFromNCall2ArgsCall((literal!("min")).clone(), expl.clone(), tp.clone())?;
            (exp_2, funcs) = differentiateExp(exp_1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFuncs.clone(), maxIter.clone())?;
            (exp_2.clone(), funcs.clone())
        },
        (Deref @ "floor", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1.clone(), inFuncs.clone())
        },
        (Deref @ "ceil", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1.clone(), inFuncs.clone())
        },
        (Deref @ "integer", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1.clone(), inFuncs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

fn createFromNCall2ArgsCall(mut funcName: ArcStr, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut result: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut rest: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(expl.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    rest = __pa2.clone();
    result = Expression::makePureBuiltinCall((funcName.clone()).clone(), list![e1.clone(), e2.clone()], tp.clone());
    for mut elem in &*rest.clone() {
        let mut elem = elem.clone();
        result = Expression::makePureBuiltinCall((funcName.clone()).clone(), list![result.clone(), elem.clone()], tp.clone());
    }
    Ok(result)
}

fn differentiateCallExpNArg(mut name: ArcStr, mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((name.clone(), inExpl.clone(), inAttr.clone())) {
        (Deref @ "smooth", Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e1: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            e1 = Arc::new(DAE::Exp::ICONST { integer: i.clone() - 1 });
            res2 = if (intGe(i.clone(), 1)) {Expression::makePureBuiltinCall((literal!("smooth")).clone(), list![e1.clone(), res1.clone()], tp.clone())} else {res1.clone()};
            (res2.clone(), funcs.clone())
        },
        (Deref @ "noEvent", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            res1 = Expression::makePureBuiltinCall((literal!("noEvent")).clone(), list![res1.clone()], tp.clone());
            (res1.clone(), funcs.clone())
        },
        (Deref @ "atan2", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e2: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            e2 = Expression::makeDiv(e.clone(), e1.clone())?;
            (res1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            res2 = Expression::addNoEventToRelations(Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: e1.clone(), expElse: Arc::new(DAE::Exp::BINARY { exp1: res1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }) }) }) }))?;
            (res2.clone(), funcs.clone())
        },
        (Deref @ "semiLinear", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res, funcs) = differentiateExp(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            res1 = Expression::expAdd(Expression::expMul(res1.clone(), e.clone())?, Expression::expMul(e1.clone(), res.clone())?)?;
            res2 = Expression::expAdd(Expression::expMul(res2.clone(), e.clone())?, Expression::expMul(e2.clone(), res.clone())?)?;
            (res, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            res = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: res.clone(), index: -1, optionExpisASUB: None });
            (Arc::new(DAE::Exp::IFEXP { expCond: res.clone(), expThen: res1.clone(), expElse: res2.clone() }), funcs.clone())
        },
        (Deref @ "cross", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            res2 = Expression::makePureBuiltinCall((literal!("cross")).clone(), list![e1.clone(), res2.clone()], tp.clone());
            res1 = Expression::makePureBuiltinCall((literal!("cross")).clone(), list![res1.clone(), e2.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: res2.clone(), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: res1.clone() }), funcs.clone())
        },
        (Deref @ "max", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("noEvent")).clone() }), expLst: list![Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::GREATER { ty: tp.clone() }, exp2: e2.clone(), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool().clone() }), expThen: res1.clone(), expElse: res2.clone() }), funcs.clone())
        },
        (Deref @ "min", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("noEvent")).clone() }), expLst: list![Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: e2.clone(), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool().clone() }), expThen: res1.clone(), expElse: res2.clone() }), funcs.clone())
        },
        (Deref @ "div", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1.clone(), inFunctionTree.clone())
        },
        (Deref @ "mod", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e: Arc<DAE::Exp>;
            let mut etmp: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            etmp = Expression::makePureBuiltinCall((literal!("floor")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e2.clone() })], tp.clone());
            e = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp.clone() }) });
            (res1, funcs) = differentiateExp(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (res1.clone(), funcs.clone())
        },
        (Deref @ "rem", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let _ = Expression::makePureBuiltinCall((literal!("div")).clone(), list![e1.clone(), e2.clone()], tp.clone());
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (res1.clone(), funcs.clone())
        },
        (Deref @ "delay", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Cons { head: e4, tail: Deref @ metamodelica::List::Nil } } } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e: Arc<DAE::Exp>;
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            res1 = (match inDiffType.clone() {
        BackendDAE::DifferentiationType::DIFFERENTIATION_TIME => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }),
        _ => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }),
    });
            (res2, funcs) = differentiateExp(e3.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            res2 = Arc::new(DAE::Exp::BINARY { exp1: res1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: res2.clone() });
            (res2, _) = ExpressionSimplify::simplify(res2.clone())?;
            if Expression::isZero(res2.clone()) {
                (res, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            } else {
                (e, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), crate::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, funcs.clone(), maxIter.clone())?;
                e = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), expLst: list![Arc::new(DAE::Exp::ICONST { integer: -1 }), e.clone(), e3.clone(), e4.clone()], attr: inAttr.clone() });
                res = Arc::new(DAE::Exp::BINARY { exp1: res2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e.clone() });
                (res, _) = ExpressionSimplify::simplify(res.clone())?;
            }
            (res.clone(), funcs.clone())
        },
        (Deref @ "sample", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1.clone(), inFunctionTree.clone())
        },
        (Deref @ "floor", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1.clone(), inFunctionTree.clone())
        },
        (Deref @ "ceil", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1.clone(), inFunctionTree.clone())
        },
        (Deref @ "integer", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1.clone(), inFunctionTree.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateBinary(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::ADD { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: de2.clone() }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::ADD_ARR { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: de2.clone() }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::ADD_ARRAY_SCALAR { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::ADD_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2.clone() }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::SUB { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: de2.clone() }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::SUB_ARR { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: de2.clone() }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::SUB_SCALAR_ARRAY { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::SUB_SCALAR_ARRAY { ty: tp.clone() }, exp2: de2.clone() }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2.clone() }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL_ARR { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: de2.clone() }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2.clone() }), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp.clone() }, exp2: de2.clone() }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp.clone() }, exp2: de2.clone() }), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2.clone() }) }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV_ARR { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: de2.clone() }) }), operator: DAE::Operator::DIV_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut tp1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            tp1 = Expression::r#typeof(e2.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2.clone() }) }), operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp1.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: tp }, exp1: e1 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), maxIter.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: de1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2.clone() }) }), operator: DAE::Operator::DIV_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2 @ Deref @ DAE::Exp::RCONST { real: r }, operator: DAE::Operator::POW { ty: tp }, exp1: e1 } => {
            let mut e: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut r = (*r).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            r = r.clone() - metamodelica::OrderedFloat(1.0_f64);
            e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: r.clone() }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1.clone() });
            (e.clone(), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2 @ Deref @ DAE::Exp::ICONST { integer: i }, operator: DAE::Operator::POW { ty: tp }, exp1: e1 } => {
            let mut e: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut i = (*i).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            i = i.clone() - 1;
            e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1.clone() });
            (e.clone(), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW { ty: tp }, exp1: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, .. } if __rlit_1.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            let mut zero: Arc<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (zero.clone(), inFunctionTree.clone())
        },
        e0 @ Deref @ DAE::Exp::BINARY { exp2: e1, operator: DAE::Operator::POW { ty: tp }, exp1: Deref @ DAE::Exp::RCONST { real: r } } => {
            let mut e: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut r = (*r).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            r = (r.clone()).ln();
            e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e0.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: r.clone() }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1.clone() });
            (e.clone(), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::POW { ty: tp }, exp1: e1 } if (isParamOrConstant(cr.clone(), inInputData.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            let mut etmp: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            etmp = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 1 }) }),
        _ => Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: etmp.clone() }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1.clone() });
            (e.clone(), funcs.clone())
        },
        e0 @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::POW { ty: tp }, exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. } } if (isParamOrConstant(cr.clone(), inInputData.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            let mut etmp: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            etmp = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], tp.clone());
            e = Expression::addNoEventToRelations(Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e0.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp.clone() }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2.clone() }) }))?;
            (e.clone(), funcs.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::POW { ty: tp }, exp1: e1 } => {
            let mut e: Arc<DAE::Exp>;
            let mut etmp: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
            etmp = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], tp.clone());
            e = Expression::addNoEventToRelations(Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp.clone() }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2.clone() }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1.clone() }) }) }) }))?;
            (e.clone(), funcs.clone())
        },
        _ => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut serr: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            s1 = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?).clone();
            serr = stringAppendList(list![(literal!("\n- Function differentiateBinary failed. differentiateExp ")).clone(), (s1.clone()).clone(), (literal!(" w.r.t: ")).clone(), (s2.clone()).clone(), (literal!(" failed\n")).clone()]);
            Debug::trace((serr.clone()).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

// =============================================================================
// functions to generate derivative of a function
// =============================================================================
fn differentiateFunctionCall(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outDiffedExp, outFunctionTree) = 'mc: {
        let __mc_input = (inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION, _) => {
                    if !((!(Expression::expHasCref(inExp.clone(), inDiffwrtCref.clone())?))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    (e, _) = Expression::makeZeroExpression(Expression::arrayDimension(ComponentReference::crefTypeFull(inDiffwrtCref.clone())?))?;
                    Ok((e.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: tc, ty, isImpure, builtin: c, tuple_: b, .. }, expLst: expl, path }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, _) => {
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dexpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dpath: Arc<Absyn::Path>;
                    let mut dinl: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    (mapper, tp) = getFunctionMapper(path.clone(), inFunctionTree.clone())?;
                    (dpath, blst) = differentiateFunction1(path.clone(), mapper.clone(), tp.clone(), expl.clone(), (inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()))?;
                    let Some(DAE::FUNCTION { inlineType: __pa0, type_: __pa1, .. }) = (AvlTreePathFunction::get(inFunctionTree.clone(), dpath.clone())?) else { bail!("pattern mismatch") };
                    dinl = __pa0.clone();
                    dtp = __pa1.clone();
                    ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), tp.clone(), dtp.clone())?) {
                        (true, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (expl1, _) = List::splitOnBoolList(expl.clone(), blst.clone())?;
                    (dexpl, outFunctionTree) = List::map3Fold(expl1.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
                    expl1 = listAppend(expl.clone(), dexpl.clone());
                    Ok((Arc::new(DAE::Exp::CALL { path: dpath.clone(), expLst: expl1.clone(), attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: b.clone(), builtin: c.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: dinl.clone(), tailCall: tc.clone() }) }), outFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: expl, path, .. }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, _) => {
                    let mut dpath: Arc<Absyn::Path>;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut tlst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut typstring: ArcStr = arcstr::literal!("");
                    let mut dastring: ArcStr = arcstr::literal!("");
                    let mut typlststring: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (mapper, tp) = getFunctionMapper(path.clone(), inFunctionTree.clone())?;
                    (dpath, blst) = differentiateFunction1(path.clone(), mapper.clone(), tp.clone(), expl.clone(), (inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()))?;
                    let Some(DAE::FUNCTION { type_: __pa0, .. }) = (AvlTreePathFunction::get(inFunctionTree.clone(), dpath.clone())?) else { bail!("pattern mismatch") };
                    dtp = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), tp.clone(), dtp.clone())?) {
                        (false, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tlst = __pa1.clone();
                    typlststring = List::map(tlst.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>));
                    typstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(typlststring.clone(), (literal!(";\n")).clone())); ArcStr::from(__mm_s) }).clone();
                    dastring = (AbsynUtil::pathString(dpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    println!("{}", (literal!("Input warnings for function mapper2\n")).clone());
                    Error::addMessage(Error::UNEXPECTED_FUNCTION_INPUTS_WARNING.clone(), list![(dastring.clone()).clone(), (typstring.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, _, _, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    if '__try0: {
                        let BackendDAE::DIFF_FULL_JACOBIAN { .. } = (inDiffType.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let __pa1 = ::match_deref::match_deref! { match &(Inline::forceInlineExp(inExp.clone(), (Some(inFunctionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone())?) {
                        (__pa1, _, true) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa1.clone();
                    (e, functions) = differentiateExp(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    Ok((e.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: expl, path }, _, _, _, _) => {
                    if !((Expression::isRecordCall(e.clone(), inFunctionTree.clone())?)) { bail!("guard") }
                    let mut dexpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    (dexpl, functions) = List::map3Fold(expl.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
                    Ok((Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: dexpl.clone(), attr: attr.clone() }), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _, _, _, _) => {
                    let mut de: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut e = (*e).clone();
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                        BackendDump::debugStrExpStr((literal!("### Differentiate call\n ")).clone(), e.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReference::crefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (de, functions) = differentiateFunctionCallPartial(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    (e, _, b) = Inline::forceInlineExp(de.clone(), (Some(functions.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone())?;
                    if b.clone() {
                        de = e.clone();
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                        BackendDump::debugStrExpStr((literal!("### result output :\n")).clone(), de.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReference::crefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((de.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut zero: Arc<DAE::Exp>;
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let false = (Expression::expContains(inExp.clone(), Expression::crefExp(inDiffwrtCref.clone())?)?) else { bail!("pattern mismatch") };
                    tp = Expression::r#typeof(inExp.clone())?;
                    zero = Expression::createZeroExpression(tp.clone())?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate.differentiateFunctionCall")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateFunctionCallPartial(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outDiffedExp, outFunctionTree) = 'mc: {
        let __mc_input = (inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: tc, ty, isImpure, builtin: c, tuple_: b, .. }, expLst: expl, path }, _, _, _, _) => {
                    let mut diffFuncData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
                    let mut e: Arc<DAE::Exp>;
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dexpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dexplZero: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dpath: Arc<Absyn::Path>;
                    let mut dinl: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut funcname: ArcStr = arcstr::literal!("");
                    (mapper, tp) = getFunctionMapper(path.clone(), inFunctionTree.clone())?;
                    (dpath, blst) = differentiateFunction1(path.clone(), mapper.clone(), tp.clone(), expl.clone(), (inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()))?;
                    let Some(DAE::FUNCTION { inlineType: __pa0, type_: __pa1, .. }) = (AvlTreePathFunction::get(inFunctionTree.clone(), dpath.clone())?) else { bail!("pattern mismatch") };
                    dinl = __pa0.clone();
                    dtp = __pa1.clone();
                    ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), tp.clone(), dtp.clone())?) {
                        (true, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (expl1, _) = List::splitOnBoolList(expl.clone(), blst.clone())?;
                    (dexpl, functions) = List::map3Fold(expl1.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
                    funcname = (BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), false)?).clone();
                    diffFuncData = BackendDAE::emptyInputData().clone();
                    diffFuncData.matrixName = Some((funcname.clone()).clone());
                    (dexplZero, functions) = List::map3Fold(expl1.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), diffFuncData.clone(), BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: false }, functions.clone());
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                        println!("{}", (literal!("### differentiated argument list:\n")).clone());
                        println!("{}", (literal!("Diffed ExpList: \n")).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(dexpl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    e = Arc::new(DAE::Exp::CALL { path: dpath.clone(), expLst: expl1.clone(), attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: b.clone(), builtin: c.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: dinl.clone(), tailCall: tc.clone() }) });
                    e = createPartialArguments(ty.clone(), dexpl.clone(), dexplZero.clone(), expl.clone(), e.clone())?;
                    Ok((e.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: expl, path, .. }, _, _, _, _) => {
                    let mut dpath: Arc<Absyn::Path>;
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut tlst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut typstring: ArcStr = arcstr::literal!("");
                    let mut dastring: ArcStr = arcstr::literal!("");
                    let mut typlststring: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (mapper, tp) = getFunctionMapper(path.clone(), inFunctionTree.clone())?;
                    (dpath, blst) = differentiateFunction1(path.clone(), mapper.clone(), tp.clone(), expl.clone(), (inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()))?;
                    let Some(DAE::FUNCTION { type_: __pa0, .. }) = (AvlTreePathFunction::get(inFunctionTree.clone(), dpath.clone())?) else { bail!("pattern mismatch") };
                    dtp = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), tp.clone(), dtp.clone())?) {
                        (false, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tlst = __pa1.clone();
                    typlststring = List::map(tlst.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>));
                    typstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(typlststring.clone(), (literal!(";\n")).clone())); ArcStr::from(__mm_s) }).clone();
                    dastring = (AbsynUtil::pathString(dpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    println!("{}", (literal!("Input warnings for function mapper2\n")).clone());
                    Error::addMessage(Error::UNEXPECTED_FUNCTION_INPUTS_WARNING.clone(), list![(dastring.clone()).clone(), (typstring.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: tc, ty, isImpure, builtin: false, tuple_: b, .. }, expLst: expl, path }, _, _, _, _) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dexpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dexplZero: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dpath: Arc<Absyn::Path>;
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut expBoolLst: Arc<metamodelica::List<(Arc<DAE::Exp>, bool)>> = metamodelica::nil();
                    let mut funstring: ArcStr = arcstr::literal!("");
                    let mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut func: DAE::Function;
                    let mut dfunc: DAE::Function;
                    let mut success: bool = false;
                    let mut e = (*e).clone();
                    let mut inInputData: BackendDAE::DifferentiateInputData = inInputData.clone();
                    if '__try0: {
                        let BackendDAE::SIMPLE_DIFFERENTIATION { .. } = (inDiffType.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    if '__try1: {
                        let BackendDAE::DIFF_FULL_JACOBIAN { .. } = (inDiffType.clone()) else { break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let Some(__pa2) = (AvlTreePathFunction::get(inFunctionTree.clone(), path.clone())?) else { bail!("pattern mismatch") };
                    func = __pa2.clone();
                    if !(AvlSetPath::hasKey(inInputData.diffedFunctions.clone(), path.clone())?) {
                        inInputData.diffedFunctions = AvlSetPath::add(inInputData.diffedFunctions.clone(), path.clone())?;
                        (dfunc, functions, blst) = differentiatePartialFunction(func.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                        dpath = DAEUtil::functionName(dfunc.clone())?;
                        let __pa3 = ::match_deref::match_deref! { match &(DAEUtil::getFunctionType(dfunc.clone())?) {
                            Deref @ DAE::Type::T_FUNCTION { funcResultType: __pa3, .. } => __pa3.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        dtp = __pa3.clone();
                        if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                            funstring = (Tpl::tplString((std::sync::Arc::new(DAEDumpTpl::dumpFunction) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, DAE::Function) -> Result<Tpl::Text> + 'static>), dfunc.clone())?).clone();
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Differentiate function: \n")); __mm_s.push_str(&*funstring.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                        }
                        functions = DAEUtil::addDaeFunction(list![dfunc.clone()], functions.clone())?;
                        func = DAEUtil::addFunctionDefinition(func.clone(), DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: path.clone(), derivativeFunction: dpath.clone(), derivativeOrder: 1, conditionRefs: metamodelica::nil(), defaultDerivative: None, lowerOrderDerivatives: metamodelica::nil() });
                        functions = AvlTreePathFunction::add(functions.clone(), path.clone(), Some(func.clone()), (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
                    } else {
                        (functions, inputVarsDer, _, outputVarsDer, _, blst) = getFunctionInOutVars(func.clone(), inFunctionTree.clone(), inDiffwrtCref.clone(), maxIter.clone())?;
                        (dpath, dtp) = getDiffedTypeandName(func.clone(), inputVarsDer.clone(), outputVarsDer.clone(), blst.clone())?;
                        let __pa4 = ::match_deref::match_deref! { match &(dtp.clone()) {
                            Deref @ DAE::Type::T_FUNCTION { funcResultType: __pa4, .. } => __pa4.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        dtp = __pa4.clone();
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        println!("{}", (literal!("### Detailed arguments list: \n")).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(expl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", (literal!("### and argument types: \n")).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::mapMap(expl.clone(), (std::sync::Arc::new(Expression::r#typeof) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Type>> + 'static>), (std::sync::Arc::new(TypesDump::printTypeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>)), (literal!(" | ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### and output type: \n")); __mm_s.push_str(&*TypesDump::printTypeStr(dtp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    expBoolLst = List::zip(expl.clone(), blst.clone());
                    expBoolLst = List::filterOnTrue(expBoolLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)));
                    expl1 = List::map(expBoolLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        println!("{}", (literal!("### Selected Arguments: \n")).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(expl1.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    (dexpl, functions) = List::map3Fold(expl1.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone());
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        println!("{}", (literal!("### Diffed ExpList: \n")).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(dexpl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    (dexplZero, functions, success) = tryZeroDiff(expl1.clone(), functions.clone(), maxIter.clone());
                    if success.clone() {
                        e = Arc::new(DAE::Exp::CALL { path: dpath.clone(), expLst: dexpl.clone(), attr: Arc::new(DAE::CallAttributes { ty: dtp.clone(), tuple_: b.clone(), builtin: false, isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: tc.clone() }) });
                        exp = createPartialArguments(ty.clone(), dexpl.clone(), dexplZero.clone(), expl.clone(), e.clone())?;
                    } else {
                        exp = Arc::new(DAE::Exp::CALL { path: dpath.clone(), expLst: listAppend(expl.clone(), dexpl.clone()), attr: Arc::new(DAE::CallAttributes { ty: dtp.clone(), tuple_: b.clone(), builtin: false, isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: tc.clone() }) });
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        println!("{}", (literal!("### differentiated result CALL :\n")).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok((exp.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate.differentiateFunctionCallPartial failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Debug::trace((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDiffedExp, outFunctionTree))
}

fn addFunctionConstantsAndParameters(mut knownVars_opt: Option<BackendDAE::Variables>, mut func: DAE::Function) -> Result<Option<BackendDAE::Variables>> {
    let mut knownVars_opt: Option<BackendDAE::Variables> = knownVars_opt;
    knownVars_opt = ({
        let mut body_knowns: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(func.clone()) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body }, tail: _ }, .. } => {
            let mut var_opt: Option<BackendDAE::Var> = None;
            for mut element in &*body.clone() {
                let mut element = element.clone();
                var_opt = BackendDAECreate::lowerKnownVarSingle(element.clone())?;
                if isSome(var_opt.clone()) {
                    body_knowns = cons(Util::getOption(var_opt.clone())?, body_knowns.clone());
                }
            }
            if body_knowns.clone().is_empty() {
                knownVars_opt = knownVars_opt.clone();
            } else if isSome(knownVars_opt.clone()) {
                knownVars_opt = Some(BackendVariable::addVars(body_knowns.clone(), Util::getOption(knownVars_opt.clone())?));
            } else {
                knownVars_opt = Some(BackendVariable::listVar(body_knowns.clone()));
            }
            knownVars_opt.clone()
        },
        _ => {
            knownVars_opt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(knownVars_opt)
}

fn tryZeroDiff(mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut functions: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> (Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<AvlTreePathFunction::Tree>, bool) {
    let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = explist;
    let mut functions: Arc<AvlTreePathFunction::Tree> = functions;
    let mut success: bool = false;
    match '__try0: {
        (explist, functions) = List::map3Fold(explist.clone(), Arc::new({ let __pe_b5 = maxIter.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), BackendDAE::emptyInputData().clone(), BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: false }, functions.clone());
        success = true;
        Ok::<_, anyhow::Error>((explist.clone(), functions.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            explist = __try0_o0;
            functions = __try0_o1;
            success = __try0_o2;
        }
        Err(_) => {
            explist = metamodelica::nil();
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (explist, functions, success)
}

fn createPartialArguments(mut outputType: Arc<DAE::Type>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (outputType.clone(), inCall.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: rPath }, .. }, Deref @ DAE::Exp::CALL { path, .. }) => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut varNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    tys = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = DAEUtil::varType(v.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    varNames = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = DAEUtil::typeVarIdent(v.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    expLst = createPartialArgumentsRecord(tys.clone(), varNames.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone())?;
                    Ok(Arc::new(DAE::Exp::RECORD { path: rPath.clone(), exps: expLst.clone(), comp: varNames.clone(), ty: outputType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::CALL { attr, path, .. }, .. }) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: tys, .. }, _) => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expLst = createPartialArgumentsTuple(tys.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone())?;
                    Ok(Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut ezero: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    dims = Expression::arrayDimension(outputType.clone());
                    (ezero, _) = Expression::makeZeroExpression(dims.clone())?;
                    e = createPartialDifferentiatedExp(inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone(), 1, ezero.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CALL { attr, path, .. }) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn createPartialArgumentsTuple(mut inTypesLst: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for (tp, number) in (&(inTypesLst.clone())).into_iter().zip((1..=(inTypesLst.clone().len() as i32)).into_iter()) {
            let __x = createPartialArguments(tp.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), Arc::new(DAE::Exp::TSUB { exp: inCall.clone(), ix: number.clone(), ty: tp.clone() }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outExpLst)
}

fn createPartialArgumentsRecord(mut inTypesLst: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inVarNames: Arc<metamodelica::List<ArcStr>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for (tp, name) in (&(inTypesLst.clone())).into_iter().zip((&(inVarNames.clone())).into_iter()) {
            let __x = createPartialArguments(tp.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), Arc::new(DAE::Exp::RSUB { exp: inCall.clone(), ix: -1, fieldName: (name.clone()).clone(), ty: tp.clone() }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outExpLst)
}

fn createPartialDifferentiatedExp(mut inDiffExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffExplZero: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>, mut currentLstElement: i32, mut inAccum: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = inAccum.clone();
    let mut i: i32 = currentLstElement.clone();
    for mut de in &*inDiffExpl.clone() {
        let mut de = de.clone();
        outExp = (::match_deref::match_deref! { match &((de.clone(), inCall.clone())) {
        (_, Deref @ DAE::Exp::CALL { attr, path, .. }) if (Types::isRecord(Expression::r#typeof(de.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            dexpLst = List::set(inDiffExplZero.clone(), i.clone(), de.clone())?;
            expLst = listAppend(inOrginalExpl.clone(), dexpLst.clone());
            e = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() });
            e.clone()
        },
        (Deref @ DAE::Exp::ARRAY { array: expl, scalar: b, ty: tp }, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut eArray: Arc<DAE::Exp>;
            let mut arrayArgs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            eArray = (inDiffExplZero.clone()).get(i.clone())?;
            dexpLst = Expression::arrayElements(eArray.clone())?;
            arrayArgs = prepareArgumentsExplArray(expl.clone(), dexpLst.clone(), 1, metamodelica::nil())?;
            expLst = List::map2(arrayArgs.clone(), (std::sync::Arc::new(fnptr!(Expression::makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), tp.clone(), b.clone());
            arrayArgs = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut exp in (expLst.clone()).into_iter().cloned() {
            let __x = List::set(inDiffExplZero.clone(), i.clone(), exp.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            arrayArgs = List::map1r(arrayArgs.clone(), Arc::new(listAppend.clone()), inOrginalExpl.clone());
            e = createPartialSum(arrayArgs.clone(), expl.clone(), inCall.clone(), outExp.clone())?;
            e.clone()
        },
        _ => {
            let mut e: Arc<DAE::Exp>;
            let mut eone: Arc<DAE::Exp>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            tp = Expression::r#typeof(de.clone())?;
            dims = Expression::arrayDimension(tp.clone());
            (eone, _) = Expression::makeOneExpression(dims.clone())?;
            dexpLst = List::set(inDiffExplZero.clone(), i.clone(), eone.clone())?;
            expLst = listAppend(inOrginalExpl.clone(), dexpLst.clone());
            e = createPartialSum(list![expLst.clone()], list![de.clone()], inCall.clone(), outExp.clone())?;
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        i = i.clone() + 1;
    }
    Ok(outExp)
}

fn createPartialSum(mut inArgsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inDiff: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>, mut inAccum: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = inAccum.clone();
    let mut restDiff: Arc<metamodelica::List<Arc<DAE::Exp>>> = inDiff.clone();
    let mut de: Arc<DAE::Exp>;
    let mut res: Arc<DAE::Exp>;
    for mut expLst in &*inArgsLst.clone() {
        let mut expLst = expLst.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(restDiff.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        de = __pa0.clone();
        restDiff = __pa1.clone();
        if !(Expression::isZero(de.clone())) {
            res = (::match_deref::match_deref! { match &(inCall.clone()) {
        Deref @ DAE::Exp::RSUB { ty, fieldName: name, ix, exp: Deref @ DAE::Exp::CALL { attr, path, .. } } => {
            Arc::new(DAE::Exp::RSUB { exp: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() }), ix: ix.clone(), fieldName: (name.clone()).clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::TSUB { ty, ix, exp: Deref @ DAE::Exp::CALL { attr, path, .. } } => {
            Arc::new(DAE::Exp::TSUB { exp: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() }), ix: ix.clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { attr, path, .. } => {
            Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            res = Expression::expMul(de.clone(), res.clone())?;
            outExp = Expression::expAdd(outExp.clone(), res.clone())?;
        }
    }
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn prepareArgumentsExplArray(mut inWorkLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCurrentArg: i32, mut inAccum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>> {
    let mut outExpLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    outExpLstLst = (::match_deref::match_deref! { match &((inWorkLst.clone(), inArgs.clone(), inCurrentArg.clone(), inAccum.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            inAccum.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _) => {
            let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut eone: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            tp = Expression::r#typeof(e.clone())?;
            dims = Expression::arrayDimension(tp.clone());
            (eone, _) = Expression::makeOneExpression(dims.clone())?;
            args = List::set(inArgs.clone(), inCurrentArg.clone(), eone.clone())?;
            prepareArgumentsExplArray(rest.clone(), inArgs.clone(), inCurrentArg.clone() + 1, cons(args.clone(), inAccum.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpLstLst)
}

fn differentiatePartialFunction(mut inFunction: DAE::Function, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(DAE::Function, Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<bool>>)> {
    let mut outDerFunction: DAE::Function;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut outBooleanlst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    (outDerFunction, outFunctionTree, outBooleanlst) = 'mc: {
        let __mc_input = (inFunction.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (func, _, _, _, _) => {
                    let mut inputData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
                    let mut diffFuncData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
                    let mut path: Arc<Absyn::Path>;
                    let mut dpath: Arc<Absyn::Path>;
                    let mut isImpure: bool = false;
                    let mut dinl: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut dtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut funcbodyDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut inputVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut inputVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut outputVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut outputVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut protectedVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut protectedVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut protectedVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut newProtectedVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut bodyStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut derbodyStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut dfunc: DAE::Function;
                    let mut funcname: ArcStr = arcstr::literal!("");
                    let mut funstring: ArcStr = arcstr::literal!("");
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut visibility: SCode::Visibility = SCode::Visibility::PROTECTED;
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        funstring = (Tpl::tplString((std::sync::Arc::new(DAEDumpTpl::dumpFunction) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, DAE::Function) -> Result<Tpl::Text> + 'static>), func.clone())?).clone();
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Differentiate differentiateFunctionCallPartial: \n")); __mm_s.push_str(&*funstring.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    inputVars = DAEUtil::getFunctionInputVars(func.clone())?;
                    outputVars = DAEUtil::getFunctionOutputVars(func.clone())?;
                    protectedVars = DAEUtil::getFunctionProtectedVars(func.clone())?;
                    bodyStmts = DAEUtil::getFunctionAlgorithmStmts(func.clone())?;
                    visibility = DAEUtil::getFunctionVisibility(func.clone());
                    (functions, inputVarsDer, inputVarsNoDer, outputVarsDer, outputVarsNoDer, blst) = getFunctionInOutVars(func.clone(), inFunctionTree.clone(), inDiffwrtCref.clone(), maxIter.clone())?;
                    path = DAEUtil::functionName(func.clone())?;
                    funcname = (BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), false)?).clone();
                    diffFuncData = BackendDAE::emptyInputData().clone();
                    diffFuncData.matrixName = Some((funcname.clone()).clone());
                    diffFuncData.diffedFunctions = inInputData.diffedFunctions.clone();
                    (inputData, _) = addElementVars2Dep(inputVarsNoDer.clone(), functions.clone(), diffFuncData.clone())?;
                    (inputData, _) = addElementVars2Dep(outputVarsNoDer.clone(), functions.clone(), inputData.clone())?;
                    (protectedVarsDer, functions, protectedVarsNoDer, _) = differentiateElementVars(protectedVars.clone(), inDiffwrtCref.clone(), inputData.clone(), crate::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, functions.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), maxIter.clone(), false)?;
                    (inputData, _) = addElementVars2Dep(protectedVarsNoDer.clone(), functions.clone(), inputData.clone())?;
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        dumpInputData(inputData.clone())?;
                    }
                    inputData.knownVars = addFunctionConstantsAndParameters(inputData.knownVars.clone(), func.clone())?;
                    (derbodyStmts, functions) = differentiateStatements(bodyStmts.clone().reverse(), inDiffwrtCref.clone(), inputData.clone(), crate::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, metamodelica::nil(), functions.clone(), maxIter.clone())?;
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        funstring = (DAEDump::ppStmtListStr(derbodyStmts.clone(), 0)?).clone();
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Differentiate differentiateFunctionCallPartial stmts: \n")); __mm_s.push_str(&*funstring.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    (dpath, dtp) = getDiffedTypeandName(func.clone(), inputVarsDer.clone(), outputVarsDer.clone(), blst.clone())?;
                    newProtectedVars = List::map1(outputVars.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::setElementVarVisibility, Arc<DAE::Element>, DAE::VarVisibility)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::VarVisibility) -> Result<Arc<DAE::Element>> + 'static>), openmodelica_frontend_types::DAE::VarVisibility::PROTECTED);
                    newProtectedVars = List::map1(newProtectedVars.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::setElementVarDirection, Arc<DAE::Element>, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::VarDirection) -> Result<Arc<DAE::Element>> + 'static>), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    funcbodyDer = listAppend(newProtectedVars.clone(), list![Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: derbodyStmts.clone() }), source: DAE::emptyElementSource().clone() })]);
                    funcbodyDer = listAppend(protectedVarsDer.clone(), funcbodyDer.clone());
                    funcbodyDer = listAppend(protectedVars.clone(), funcbodyDer.clone());
                    funcbodyDer = listAppend(outputVarsDer.clone(), funcbodyDer.clone());
                    funcbodyDer = listAppend(inputVarsDer.clone(), funcbodyDer.clone());
                    funcbodyDer = listAppend(inputVars.clone(), funcbodyDer.clone());
                    isImpure = DAEUtil::getFunctionImpureAttribute(func.clone())?;
                    dinl = DAEUtil::getFunctionInlineType(func.clone())?;
                    dfunc = DAE::Function::FUNCTION { path: dpath.clone(), functions: list![DAE::FunctionDefinition::FUNCTION_DEF { body: funcbodyDer.clone() }], type_: dtp.clone(), visibility: visibility.clone(), partialPrefix: false, isImpure: isImpure.clone(), inlineType: dinl.clone(), unusedInputs: metamodelica::nil(), source: DAE::emptyElementSource().clone(), comment: None };
                    Ok((dfunc.clone(), functions.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: Arc<Absyn::Path>;
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    path = DAEUtil::functionName(inFunction.clone())?;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate.differentiatePartialFunction failed for function: ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Debug::trace((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDerFunction, outFunctionTree, outBooleanlst))
}

fn getDiffedTypeandName(mut inFunction: DAE::Function, mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut blst: Arc<metamodelica::List<bool>>) -> Result<(Arc<Absyn::Path>, Arc<DAE::Type>)> {
    let mut diffedName: Arc<Absyn::Path>;
    let mut diffedType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    diffedType = Types::extendsFunctionTypeArgs(DAEUtil::getFunctionType(inFunction.clone())?, inputVarsDer.clone(), outputVarsDer.clone(), blst.clone())?;
    diffedName = AbsynUtil::stringPath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$DER")); __mm_s.push_str(&*BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(DAEUtil::functionName(inFunction.clone())?, (literal!(".")).clone(), true, false)?).clone(), false)?); ArcStr::from(__mm_s) }).clone())?;
    Ok((diffedName, diffedType))
}

fn getFunctionInOutVars(mut inFunction: DAE::Function, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut maxIter: i32) -> Result<(Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<bool>>)> {
    let mut functions: Arc<AvlTreePathFunction::Tree> = inFunctionTree.clone();
    let mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut inputVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outputVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut inputVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outputVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut diffData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
    inputVars = DAEUtil::getFunctionInputVars(inFunction.clone())?;
    outputVars = DAEUtil::getFunctionOutputVars(inFunction.clone())?;
    diffData = BackendDAE::emptyInputData().clone();
    diffData.matrixName = Some((BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(DAEUtil::functionName(inFunction.clone())?, (literal!(".")).clone(), true, false)?).clone(), false)?).clone());
    (inputVarsDer, functions, inputVarsNoDer, blst) = differentiateElementVars(inputVars.clone(), inDiffwrtCref.clone(), diffData.clone(), crate::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, functions.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), maxIter.clone(), true)?;
    (outputVarsDer, functions, outputVarsNoDer, _) = differentiateElementVars(outputVars.clone(), inDiffwrtCref.clone(), diffData.clone(), crate::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, functions.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), maxIter.clone(), false)?;
    Ok((functions, inputVarsDer, inputVarsNoDer, outputVarsDer, outputVarsNoDer, blst))
}

fn differentiateElementVars(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut inElementsDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inElementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inBooleanLst: Arc<metamodelica::List<bool>>, mut maxIter: i32, mut elementListInputs: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<bool>>)> {
    let mut outElements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut outElementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outBooleanLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    (outElements, outFunctionTree, outElementsNoDer, outBooleanLst) = 'mc: {
        let __mc_input = (inElements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), inElementsDer.clone(), inElementsNoDer.clone(), inBooleanLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    Ok((metamodelica::Dangerous::listReverseInPlace(inElementsDer.clone()), inFunctionTree.clone(), metamodelica::Dangerous::listReverseInPlace(inElementsNoDer.clone()), metamodelica::Dangerous::listReverseInPlace(inBooleanLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var1 @ Deref @ DAE::Element::VAR { binding: Some(binding), ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, componentRef: cref, .. }, tail: rest }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, _, _, _, _, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut var: Arc<DAE::Element>;
                    let mut dcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut dbinding: Arc<DAE::Exp>;
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    dcref = createDiffedCrefName(cref.clone(), (matrixName.clone()).clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), var1.clone())?;
                    (dbinding, _) = differentiateExp(binding.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    var = DAEUtil::replaceBindungInVar(dbinding.clone(), var.clone())?;
                    vars = cons(var.clone(), inElementsDer.clone());
                    blst = cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), vars.clone(), inElementsNoDer.clone(), blst.clone(), maxIter.clone(), elementListInputs.clone())?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var1 @ Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, componentRef: cref, .. }, tail: rest }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, _, _, _, _, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut var: Arc<DAE::Element>;
                    let mut dcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    dcref = createDiffedCrefName(cref.clone(), (matrixName.clone()).clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), var1.clone())?;
                    vars = cons(var.clone(), inElementsDer.clone());
                    blst = cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), vars.clone(), inElementsNoDer.clone(), blst.clone(), maxIter.clone(), elementListInputs.clone())?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var @ Deref @ DAE::Element::VAR { binding: Some(binding), .. }, tail: rest }, _, BackendDAE::DifferentiateInputData { independenentVars: Some(timevars), .. }, _, _, _, _, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    crefLst = Expression::extractCrefsFromExp(binding.clone())?;
                    ::match_deref::match_deref! { match &(BackendVariable::getVarLst(crefLst.clone(), timevars.clone())) {
                        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    vars = cons(var.clone(), inElementsNoDer.clone());
                    blst = cons(false, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), inElementsDer.clone(), vars.clone(), blst.clone(), maxIter.clone(), elementListInputs.clone())?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var1 @ Deref @ DAE::Element::VAR { binding: Some(binding), ty: tp, componentRef: cref, .. }, tail: rest }, _, _, _, _, _, _, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut var: Arc<DAE::Element>;
                    let mut dcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut dbinding: Arc<DAE::Exp>;
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    if elementListInputs.clone() {
                        let true = (Types::isRealOrSubTypeReal(tp.clone())?) else { bail!("pattern mismatch") };
                    }
                    e = Expression::crefExp(cref.clone())?;
                    (e, functions) = differentiateCrefs(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    dcref = Expression::expCref(e.clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), var1.clone())?;
                    (dbinding, functions) = differentiateExp(binding.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    var = DAEUtil::replaceBindungInVar(dbinding.clone(), var.clone())?;
                    vars = cons(var.clone(), inElementsDer.clone());
                    blst = cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), vars.clone(), inElementsNoDer.clone(), blst.clone(), maxIter.clone(), elementListInputs.clone())?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var1 @ Deref @ DAE::Element::VAR { ty: tp, componentRef: cref, .. }, tail: rest }, _, _, _, _, _, _, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut var: Arc<DAE::Element>;
                    let mut dcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    if elementListInputs.clone() {
                        let true = (Types::isRealOrSubTypeReal(tp.clone())?) else { bail!("pattern mismatch") };
                    }
                    e = Expression::crefExp(cref.clone())?;
                    (e, functions) = differentiateCrefs(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter.clone())?;
                    dcref = Expression::expCref(e.clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), var1.clone())?;
                    vars = cons(var.clone(), inElementsDer.clone());
                    blst = cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), vars.clone(), inElementsNoDer.clone(), blst.clone(), maxIter.clone(), elementListInputs.clone())?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var @ Deref @ DAE::Element::VAR { .. }, tail: rest }, _, _, _, _, _, _, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut functions: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    elementsNoDer = cons(var.clone(), inElementsNoDer.clone());
                    blst = cons(false, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), inElementsDer.clone(), elementsNoDer.clone(), blst.clone(), maxIter.clone(), elementListInputs.clone())?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElements, outFunctionTree, outElementsNoDer, outBooleanLst))
}

fn differentiateFunction1(mut inFuncName: Arc<Absyn::Path>, mut inMapper: DAE::FunctionDefinition, mut inTp: Arc<DAE::Type>, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffArgs: (Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<Absyn::Path>, Arc<metamodelica::List<bool>>)> {
    let mut outFuncName: Arc<Absyn::Path>;
    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    (outFuncName, blst) = 'mc: {
        let __mc_input = (inFuncName.clone(), inMapper.clone(), inTp.clone(), expl.clone(), inDiffArgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionDefinition::FUNCTION_DER_MAPPER { conditionRefs: cr, derivativeOrder, derivativeFunction: inDFuncName, .. }, Deref @ DAE::Type::T_FUNCTION { funcArg, .. }, _, _) => {
                    if !((intEq(1, derivativeOrder.clone()))) { bail!("guard") }
                    let mut tplst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut bl: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut ba: metamodelica::Array<bool>;
                    tplst = List::map(funcArg.clone(), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                    ba = Array::mapList(tplst.clone(), (std::sync::Arc::new(fnptr!(diffableTypes, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>))?;
                    bl = checkDerFunctionConds(ba.clone(), cr.clone(), expl.clone(), inDiffArgs.clone())?;
                    Ok((inDFuncName.clone(), bl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionDefinition::FUNCTION_DER_MAPPER { conditionRefs: cr, derivativeOrder, derivativeFunction: inDFuncName, .. }, tp, _, (_, _, _, functions)) => {
                    if !((!(intEq(1, derivativeOrder.clone())))) { bail!("guard") }
                    let mut fname: Arc<Absyn::Path>;
                    let mut bl: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut mapper: DAE::FunctionDefinition;
                    let mut ba: metamodelica::Array<bool>;
                    let mut tp = (*tp).clone();
                    let mut blst: Arc<metamodelica::List<bool>> = blst.clone();
                    fname = getlowerOrderDerivative(inFuncName.clone(), functions.clone())?;
                    (mapper, tp) = getFunctionMapper(fname.clone(), functions.clone())?;
                    (_, blst) = differentiateFunction1(fname.clone(), mapper.clone(), tp.clone(), expl.clone(), inDiffArgs.clone())?;
                    (bl, _) = List::split1OnTrue(blst.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)), true);
                    ba = arrayAppend(arrayCreate((blst.clone().len() as i32), false), metamodelica::arrayFromVec(bl.clone().into_iter().cloned().collect()))?;
                    bl = checkDerFunctionConds(ba.clone(), cr.clone(), expl.clone(), inDiffArgs.clone())?;
                    Ok((inDFuncName.clone(), bl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionDefinition::FUNCTION_DER_MAPPER { lowerOrderDerivatives, defaultDerivative: Some(default), derivativeOrder, derivedFunction: fname, .. }, tp, _, _) => {
                    let mut da: Arc<Absyn::Path>;
                    let mut bl: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    (da, bl) = differentiateFunction1(inFuncName.clone(), DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: fname.clone(), derivativeFunction: default.clone(), derivativeOrder: derivativeOrder.clone(), conditionRefs: metamodelica::nil(), defaultDerivative: Some(default.clone()), lowerOrderDerivatives: lowerOrderDerivatives.clone() }, tp.clone(), expl.clone(), inDiffArgs.clone())?;
                    Ok((da.clone(), bl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outFuncName, blst))
}

fn checkDerivativeFunctionInputs(mut blst: Arc<metamodelica::List<bool>>, mut tp: Arc<DAE::Type>, mut dtp: Arc<DAE::Type>) -> Result<(bool, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outBoolean: bool = false;
    let mut outExpectedTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outBoolean, outExpectedTypeLst) = 'mc: {
        let __mc_input = (blst.clone(), tp.clone(), dtp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_FUNCTION { funcArg: falst, .. }, Deref @ DAE::Type::T_FUNCTION { funcArg: dfalst, .. }) => {
                    let mut falst1: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut falst2: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut tlst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut dtlst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut ret: bool = false;
                    (falst1, _) = List::splitOnBoolList(falst.clone(), blst.clone())?;
                    falst2 = listAppend(falst.clone(), falst1.clone());
                    tlst = List::map(falst2.clone(), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                    dtlst = List::map(dfalst.clone(), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                    ret = List::isEqualOnTrue(tlst.clone(), dtlst.clone(), (std::sync::Arc::new(Types::equivtypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<bool> + 'static>));
                    Ok((ret.clone(), tlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Differentiate.checkDerivativeFunctionInputs failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outBoolean, outExpectedTypeLst))
}

fn checkDerFunctionConds(mut inbarr: metamodelica::Array<bool>, mut icrlst: Arc<metamodelica::List<(i32, DAE::derivativeCond)>>, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffArgs: (Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>)) -> Result<Arc<metamodelica::List<bool>>> {
    let mut outblst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut dc: DAE::derivativeCond = DAE::derivativeCond::ZERO_DERIVATIVE;
    let mut e: Arc<DAE::Exp>;
    let mut p1: Arc<Absyn::Path>;
    let mut p2: Arc<Absyn::Path>;
    let mut ba: metamodelica::Array<bool> = inbarr.clone();
    let mut diffwrtCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut inputData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
    let mut diffType: BackendDAE::DifferentiationType = BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION;
    let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (diffwrtCref, inputData, diffType, functionTree) = inDiffArgs.clone();
    for mut tpl in &*icrlst.clone() {
        let mut tpl = tpl.clone();
        (i, dc) = tpl.clone();
        let _ = 'mc: {
        let __mc_input = dc.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::derivativeCond::ZERO_DERIVATIVE => {
                    let mut e: Arc<DAE::Exp>;
                    let mut functionTree: Arc<AvlTreePathFunction::Tree> = functionTree.clone();
                    e = (expl.clone()).get(i.clone())?;
                    (e, functionTree) = differentiateExp(e.clone(), diffwrtCref.clone(), inputData.clone(), diffType.clone(), functionTree.clone(), defaultMaxIter.clone())?;
                    let true = (Expression::isZero(e.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::derivativeCond::NO_DERIVATIVE { binding: Deref @ DAE::Exp::CALL { path: p1, .. } } => {
                    let mut p2: Arc<Absyn::Path>;
                    let __pa0 = ::match_deref::match_deref! { match &((expl.clone()).get(i.clone())?) {
                        Deref @ DAE::Exp::CALL { path: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p2 = __pa0.clone();
                    let true = (AbsynUtil::pathEqual(p1.clone(), p2.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::derivativeCond::NO_DERIVATIVE { binding: Deref @ DAE::Exp::ICONST { .. } } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("-Differentiate.checkDerFunctionConds failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        {let _arr = ba.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = false; _arr};
    }
    outblst = Arc::new(ba.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    Ok(outblst)
}

fn getlowerOrderDerivative(mut fname: Arc<Absyn::Path>, mut functions: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<Absyn::Path>> {
    let mut outFName: Arc<Absyn::Path>;
    outFName = (::match_deref::match_deref! { match &((fname.clone(), functions.clone())) {
        (_, _) => {
            let mut flst: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
            let mut lowerOrderDerivatives: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut name: Arc<Absyn::Path>;
            let Some(DAE::FUNCTION { functions: __pa0, .. }) = (AvlTreePathFunction::get(functions.clone(), fname.clone())?) else { bail!("pattern mismatch") };
            flst = __pa0.clone();
            let DAE::FUNCTION_DER_MAPPER { lowerOrderDerivatives: __pa1, .. } = (getFunctionMapper1(flst.clone())?) else { bail!("pattern mismatch") };
            lowerOrderDerivatives = __pa1.clone();
            name = List::last(lowerOrderDerivatives.clone())?;
            name.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outFName)
}

pub fn getFunctionMapper(mut fname: Arc<Absyn::Path>, mut functions: Arc<AvlTreePathFunction::Tree>) -> Result<(DAE::FunctionDefinition, Arc<DAE::Type>)> {
    let mut mapper: DAE::FunctionDefinition;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (mapper, tp) = 'mc: {
        let __mc_input = (fname.clone(), functions.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut flst: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut m: DAE::FunctionDefinition;
                    let Some(DAE::FUNCTION { type_: __pa0, functions: __pa1, .. }) = (AvlTreePathFunction::get(functions.clone(), fname.clone())?) else { bail!("pattern mismatch") };
                    t = __pa0.clone();
                    flst = __pa1.clone();
                    m = getFunctionMapper1(flst.clone())?;
                    Ok((m.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    s = (AbsynUtil::pathString(fname.clone(), (literal!(".")).clone(), true, false)?).clone();
                    s = (stringAppend((literal!("-Differentiate.getFunctionMapper failed for function ")).clone(), (s.clone()).clone())).clone();
                    Debug::traceln((s.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((mapper, tp))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getFunctionMapper1(mut inFuncDefs: Arc<metamodelica::List<DAE::FunctionDefinition>>) -> Result<DAE::FunctionDefinition> {
    let mut mapper: DAE::FunctionDefinition;
    mapper = 'mc: {
        let __mc_input = inFuncDefs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: m @ DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. }, tail: _ } => {
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: funcDefs } => {
                    let mut m: DAE::FunctionDefinition;
                    m = getFunctionMapper1(funcDefs.clone())?;
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Differentiate.getFunctionMapper1 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(mapper)
}

fn diffableTypes(mut inType: Arc<DAE::Type>) -> bool {
    let mut out: bool = Types::isRealOrSubTypeReal(inType.clone()).unwrap() || Types::isRecord(inType.clone());
    out
}

//
// util functions for Types: DifferentiateInputData, DifferentiateInputArguments, DifferentiationType
//
fn addDependentVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut depVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    if isSome(outDiffData.dependenentVars.clone()) {
        depVars = BackendVariable::addVars(inVarsLst.clone(), Util::getOption(outDiffData.dependenentVars.clone())?);
    } else {
        depVars = BackendVariable::listVar(inVarsLst.clone());
    }
    outDiffData.dependenentVars = Some(depVars.clone());
    Ok(outDiffData)
}

fn addAllVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut allVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    if isSome(outDiffData.allVars.clone()) {
        allVars = BackendVariable::addVars(inVarsLst.clone(), Util::getOption(outDiffData.allVars.clone())?);
    } else {
        allVars = BackendVariable::listVar(inVarsLst.clone());
    }
    outDiffData.allVars = Some(allVars.clone());
    Ok(outDiffData)
}

fn addGlobalVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut glVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    if isSome(outDiffData.knownVars.clone()) {
        glVars = BackendVariable::addVars(inVarsLst.clone(), Util::getOption(outDiffData.knownVars.clone())?);
    } else {
        glVars = BackendVariable::listVar(inVarsLst.clone());
    }
    outDiffData.knownVars = Some(glVars.clone());
    Ok(outDiffData)
}

fn lowerVarsElementVars(mut inElementLstVars: Arc<metamodelica::List<Arc<DAE::Element>>>, mut functions: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut varsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut reqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut knvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut exvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    match '__try0: {
        (vars, knvars, exvars, eqnsLst, reqnsLst) = unwrap_break_err!(BackendDAECreate::lowerVars(inElementLstVars.clone(), functions.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), '__try0);
        varsLst = listAppend(exvars.clone(), listAppend(vars.clone(), knvars.clone()));
        Ok::<_, anyhow::Error>((eqnsLst.clone(), exvars.clone(), knvars.clone(), reqnsLst.clone(), vars.clone(), varsLst.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            eqnsLst = __try0_o0;
            exvars = __try0_o1;
            knvars = __try0_o2;
            reqnsLst = __try0_o3;
            vars = __try0_o4;
            varsLst = __try0_o5;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- Differentiate.lowerVarsElementVars failed.")).clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((varsLst, eqnsLst, reqnsLst))
}

fn addElementVars2Dep(mut inElementLstVars: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inFunctions: Arc<AvlTreePathFunction::Tree>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<(BackendDAE::DifferentiateInputData, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = <BackendDAE::DifferentiateInputData as ::std::default::Default>::default();
    let mut outEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    match '__try0: {
        (varsLst, outEqnsLst, _) = unwrap_break_err!(lowerVarsElementVars(inElementLstVars.clone(), inFunctions.clone()), '__try0);
        outDiffData = unwrap_break_err!(addDependentVars(varsLst.clone(), inDiffData.clone()), '__try0);
        Ok::<_, anyhow::Error>((outDiffData.clone(), outEqnsLst.clone(), varsLst.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            outDiffData = __try0_o0;
            outEqnsLst = __try0_o1;
            varsLst = __try0_o2;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Differentiate.addElementVars2Dep failed")).clone()])?;
            bail!("fail");
        }
    }
    Ok((outDiffData, outEqnsLst))
}

fn dumpInputData(mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<()> {
    let mut independenentVars: Option<BackendDAE::Variables> = None;
    let mut dependenentVars: Option<BackendDAE::Variables> = None;
    let mut knownVars: Option<BackendDAE::Variables> = None;
    let mut allVars: Option<BackendDAE::Variables> = None;
    let mut controlVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut diffCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut matrixName: Option<ArcStr> = None;
    println!("{}", (literal!("### dumpInputData ###\n")).clone());
    if isSome(inDiffData.matrixName.clone()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### for ")); __mm_s.push_str(&*Util::getOption(inDiffData.matrixName.clone())?); __mm_s.push_str(&*literal!(" ###\n")); ArcStr::from(__mm_s) }).clone());
    }
    if isSome(inDiffData.independenentVars.clone()) {
        println!("{}", (literal!("independentVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.independenentVars.clone())?)?;
    }
    if isSome(inDiffData.dependenentVars.clone()) {
        println!("{}", (literal!("dependenentVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.dependenentVars.clone())?)?;
    }
    if isSome(inDiffData.knownVars.clone()) {
        println!("{}", (literal!("knownVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.knownVars.clone())?)?;
    }
    if isSome(inDiffData.allVars.clone()) {
        println!("{}", (literal!("allVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.allVars.clone())?)?;
    }
    if !(inDiffData.controlVars.clone().is_empty()) {
        println!("{}", (literal!("controlVars:\n")).clone());
        BackendDump::printVarList(inDiffData.controlVars.clone());
    }
    if !(inDiffData.diffCrefs.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("diffCrefs:\n")); __mm_s.push_str(&*ComponentReference::printComponentRefListStr(inDiffData.diffCrefs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn isParamOrConstant(mut cref: Arc<DAE::ComponentRef>, mut diffData: BackendDAE::DifferentiateInputData) -> Result<bool> {
    let mut b: bool = false;
    b = (match diffData.clone() {
        BackendDAE::DifferentiateInputData { knownVars: Some(mut knownVars), .. } => {
            let mut var_lst: Option<Arc<metamodelica::List<BackendDAE::Var>>> = None;
            let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            var_lst = BackendVariable::getVarTryHard(cref.clone(), knownVars.clone());
            if isSome(var_lst.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(Util::getOption(var_lst.clone())?) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                var = __pa0.clone();
                b = BackendVariable::isParamOrConstant(var.clone());
            } else {
                b = false;
            }
            b.clone()
        },
        _ => {
            false
        },
    });
    Ok(b)
}

