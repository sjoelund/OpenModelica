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
use crate::AvlSetInt;
use crate::BackendDAE;
use crate::BackendDAEEXT;
use crate::BackendDAEFunc;
use crate::BackendDAEOptimize;
use crate::BackendDAETransform;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendInline;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::BinaryTree;
use crate::Causalize;
use crate::CommonSubExpression;
use crate::DAEMode;
use crate::DataReconciliation;
use crate::Differentiate;
use crate::DumpGraphML;
use crate::DynamicOptimization;
use crate::EvaluateFunctions;
use crate::EvaluateParameter;
use crate::ExpressionSolve;
use crate::FindZeroCrossings;
use crate::HpcOmEqSystems;
use crate::IndexReduction;
use crate::Initialization;
use crate::InlineArrayEquations;
use crate::Matching;
use crate::OnRelaxation;
use crate::RemoveSimpleEquations;
use crate::ResolveLoops;
use crate::Sorting;
use crate::SymbolicImplicitSolver;
use crate::SymbolicJacobian;
use crate::SynchronousFeatures;
use crate::Tearing;
use crate::XMLDump;
use crate::ZeroCrossings;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::AvlSetPath;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::Inline;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::StackOverflow;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub fn isInitializationDAE(mut inShared: Arc<BackendDAE::Shared>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(inShared.clone()) {
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::INITIALSYSTEM, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isSimulationDAE(mut inShared: Arc<BackendDAE::Shared>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(inShared.clone()) {
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::SIMULATION, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isJacobianDAE(mut inShared: Arc<BackendDAE::Shared>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(inShared.clone()) {
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::JACOBIAN, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

/* ************************************************
 * checkBackendDAE and stuff
 ************************************************/
pub fn checkBackendDAEWithErrorMsg(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut expCrefs: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
    let mut wrongEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    if !(Flags::isSet(Flags::CHECK_BACKEND_DAE.clone())?) {
        return Ok(());
    }
    let () = 'mc: {
        let __mc_input = inBackendDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE { shared: BackendDAE::Shared { .. }, eqs: Deref @ metamodelica::List::Cons { head: BackendDAE::EqSystem { orderedEqs, orderedVars: vars, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut nVars: i32 = 0;
                    let mut nEqns: i32 = 0;
                    let mut samesize: bool = false;
                    let mut wrongEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = wrongEqns.clone();
                    let mut expCrefs: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = expCrefs.clone();
                    nVars = BackendVariable::varsSize(vars.clone())?;
                    nEqns = BackendEquation::equationArraySize(orderedEqs.clone())?;
                    samesize = nVars.clone() == nEqns.clone();
                    if Flags::isSet(Flags::CHECK_BACKEND_DAE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("No. of Equations: ")); __mm_s.push_str(&*intString(nVars.clone())); __mm_s.push_str(&*literal!(" No. of BackendDAE.Variables: ")); __mm_s.push_str(&*intString(nEqns.clone())); __mm_s.push_str(&*literal!(" Samesize: ")); __mm_s.push_str(&*boolString(samesize.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    (expCrefs, wrongEqns) = checkBackendDAE(inBackendDAE.clone())?;
                    printcheckBackendDAEWithErrorMsg(expCrefs.clone(), wrongEqns.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("./Compiler/BackEnd/BackendDAEUtil.mo: function checkBackendDAEWithErrorMsg failed")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn printcheckBackendDAEWithErrorMsg(mut inExpCrefs: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, mut inWrongEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inExpCrefs.clone(), inWrongEqns.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: eqn, tail: wrongEqns }) => {
            printEqnSizeError(eqn.clone())?;
            printcheckBackendDAEWithErrorMsg(metamodelica::nil(), wrongEqns.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: (e, crefs), tail: res }, wrongEqns) => {
            let mut strcrefs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut crefstring: ArcStr = arcstr::literal!("");
            let mut expstr: ArcStr = arcstr::literal!("");
            let mut scopestr: ArcStr = arcstr::literal!("");
            strcrefs = List::map(crefs.clone(), (std::sync::Arc::new(ComponentReference::crefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>));
            crefstring = stringDelimitList(strcrefs.clone(), (literal!(", ")).clone());
            expstr = (ExpressionBasics::printExpStr(e.clone())?).clone();
            scopestr = stringAppendList(list![(crefstring.clone()).clone(), (literal!(" from Expression: ")).clone(), (expstr.clone()).clone()]);
            Error::addMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(scopestr.clone()).clone(), (literal!("BackendDAE object")).clone()])?;
            printcheckBackendDAEWithErrorMsg(res.clone(), wrongEqns.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEqnSizeError(mut inEqn: Arc<BackendDAE::Equation>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inEqn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                eqn @ Deref @ BackendDAE::Equation::EQUATION { source, scalar: e2, exp: e1, .. } => {
                    let mut t1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut eqnstr: ArcStr = arcstr::literal!("");
                    let mut t1str: ArcStr = arcstr::literal!("");
                    let mut t2str: ArcStr = arcstr::literal!("");
                    let mut tstr: ArcStr = arcstr::literal!("");
                    eqnstr = (BackendDump::equationString(eqn.clone())?).clone();
                    t1 = Expression::r#typeof(e1.clone())?;
                    t2 = Expression::r#typeof(e2.clone())?;
                    t1str = (TypesDump::unparseTypeNoAttr(t1.clone())?).clone();
                    t2str = (TypesDump::unparseTypeNoAttr(t2.clone())?).clone();
                    tstr = stringAppendList(list![(t1str.clone()).clone(), (literal!(" != ")).clone(), (t2str.clone()).clone()]);
                    Error::addSourceMessage(Error::EQUATION_TYPE_MISMATCH_ERROR.clone(), list![(eqnstr.clone()).clone(), (tstr.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { source, exp: e1, componentRef: cr, .. } => {
                    let mut t1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut eqnstr: ArcStr = arcstr::literal!("");
                    let mut t1str: ArcStr = arcstr::literal!("");
                    let mut t2str: ArcStr = arcstr::literal!("");
                    let mut tstr: ArcStr = arcstr::literal!("");
                    eqnstr = (BackendDump::equationString(eqn.clone())?).clone();
                    t1 = Expression::r#typeof(e1.clone())?;
                    t2 = ComponentReference::crefLastType(cr.clone())?;
                    t1str = (TypesDump::unparseTypeNoAttr(t1.clone())?).clone();
                    t2str = (TypesDump::unparseTypeNoAttr(t2.clone())?).clone();
                    tstr = stringAppendList(list![(t1str.clone()).clone(), (literal!(" != ")).clone(), (t2str.clone()).clone()]);
                    Error::addSourceMessage(Error::EQUATION_TYPE_MISMATCH_ERROR.clone(), list![(eqnstr.clone()).clone(), (tstr.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
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

pub fn checkBackendDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<(Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outExpCrefs: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
    let mut outWrongEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (outExpCrefs, outWrongEqns) = 'mc: {
        let __mc_input = inBackendDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE { UNIQUEIO: Deref @ metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: shared, .. } => {
                    let mut allvars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut expcrefs: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
                    let mut wrongEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    allvars = BackendVariable::mergeVariables(syst.orderedVars.clone(), shared.globalKnownVars.clone(), true)?;
                    (_, expcrefs) = traverseBackendDAEExpsVars(syst.orderedVars.clone(), (std::sync::Arc::new(checkBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>))> + 'static>), (allvars.clone(), metamodelica::nil()))?;
                    (_, expcrefs) = traverseBackendDAEExpsEqns(shared.removedEqs.clone(), (std::sync::Arc::new(checkBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>))> + 'static>), (allvars.clone(), expcrefs.clone()))?;
                    (_, expcrefs) = traverseBackendDAEExpsVars(shared.globalKnownVars.clone(), (std::sync::Arc::new(checkBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>))> + 'static>), (allvars.clone(), expcrefs.clone()))?;
                    (_, expcrefs) = traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), (std::sync::Arc::new(checkBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>))> + 'static>), (allvars.clone(), expcrefs.clone()))?;
                    (_, expcrefs) = traverseBackendDAEExpsEqns(syst.removedEqs.clone(), (std::sync::Arc::new(checkBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>))> + 'static>), (allvars.clone(), expcrefs.clone()))?;
                    (_, expcrefs) = traverseBackendDAEExpsEqns(shared.initialEqs.clone(), (std::sync::Arc::new(checkBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>))> + 'static>), (allvars.clone(), expcrefs.clone()))?;
                    wrongEqns = BackendEquation::traverseEquationArray(syst.orderedEqs.clone(), (std::sync::Arc::new(checkEquationSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), metamodelica::nil())?;
                    wrongEqns = BackendEquation::traverseEquationArray(shared.removedEqs.clone(), (std::sync::Arc::new(checkEquationSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), wrongEqns.clone())?;
                    wrongEqns = BackendEquation::traverseEquationArray(syst.removedEqs.clone(), (std::sync::Arc::new(checkEquationSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), wrongEqns.clone())?;
                    wrongEqns = BackendEquation::traverseEquationArray(shared.initialEqs.clone(), (std::sync::Arc::new(checkEquationSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), wrongEqns.clone())?;
                    Ok((expcrefs.clone(), wrongEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- BackendDAEUtil.checkBackendDAE failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExpCrefs, outWrongEqns))
}

fn checkBackendDAEExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (vars, lstExpCrefs)) => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let (_, (_, __pa0)) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(traversecheckBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>), (vars.clone(), metamodelica::nil()))?;
                    crefs = __pa0.clone();
                    Ok((exp.clone(), if (!(crefs.clone().is_empty())) {(vars.clone(), cons((exp.clone(), crefs.clone()), lstExpCrefs.clone()))} else {inTpl.clone()}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

fn traversecheckBackendDAEExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst, .. }, componentRef: cr }, _) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tp: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
                    expl = List::map1(varLst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone());
                    (_, tp) = Expression::traverseExpList(expl.clone(), (std::sync::Arc::new(traversecheckBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>), inTuple.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut tp: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (_, tp) = Expression::traverseExpBottomUp(e1.clone(), (std::sync::Arc::new(traversecheckBackendDAEExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>), inTuple.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::REDUCTION { iterators: riters, .. }, (vars, crefs)) => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut vars = (*vars).clone();
                    backendVars = List::map(riters.clone(), (std::sync::Arc::new(makeIterVariable) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>) -> Result<BackendDAE::Var> + 'static>));
                    vars = BackendVariable::addVars(backendVars.clone(), vars.clone());
                    Ok((e.clone(), (vars.clone(), crefs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _)) => {
                    BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, crefs)) => {
                    Ok((e.clone(), (vars.clone(), cons(cr.clone(), crefs.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

fn makeIterVariable(mut iter: Arc<DAE::ReductionIterator>) -> Result<BackendDAE::Var> {
    let mut backendVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut name: ArcStr = arcstr::literal!("");
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    name = (Expression::reductionIterName(iter.clone())?).clone();
    cr = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
    backendVar = BackendDAE::Var { varName: cr.clone(), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_INTEGER_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    Ok(backendVar)
}

fn checkEquationSize(mut inEq: Arc<BackendDAE::Equation>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (outEq, outEqs) = 'mc: {
        let __mc_input = (inEq.clone(), inEqs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, wrongEqns) => {
                    let mut wrongEqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut t1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut b: bool = false;
                    t1 = Expression::r#typeof(e1.clone())?;
                    t2 = Expression::r#typeof(e2.clone())?;
                    b = Expression::equalTypes(t1.clone(), t2.clone())?;
                    wrongEqns1 = List::consOnTrue(!(b.clone()), e.clone(), wrongEqns.clone());
                    Ok((e.clone(), wrongEqns1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e1, componentRef: cr, .. }, wrongEqns) => {
                    let mut wrongEqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut t1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut b: bool = false;
                    t1 = Expression::r#typeof(e1.clone())?;
                    t2 = ComponentReference::crefLastType(cr.clone())?;
                    b = Expression::equalTypes(t1.clone(), t2.clone())?;
                    wrongEqns1 = List::consOnTrue(!(b.clone()), e.clone(), wrongEqns.clone());
                    Ok((e.clone(), wrongEqns1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEq.clone(), inEqs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEq, outEqs))
}

pub fn checkAssertCondition(mut cond: Arc<DAE::Exp>, mut message: Arc<DAE::Exp>, mut level: Arc<DAE::Exp>, mut info: SourceInfo) -> Result<()> {
    if Flags::getConfigBool(Flags::CHECK_MODEL.clone())? {
        return Ok(());
    }
    let () = 'mc: {
        let __mc_input = info.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if !((!(Expression::isConstFalse(cond.clone())))) { bail!("guard") }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                ::match_deref::match_deref! { match &(level.clone()) {
                    Deref @ DAE::Exp::ENUM_LITERAL { index: 2, .. } => (),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut messageStr: ArcStr = arcstr::literal!("");
            let true = (Expression::isConstFalse(cond.clone())) else { bail!("pattern mismatch") };
            messageStr = (ExpressionBasics::printExpStr(message.clone())?).clone();
            Error::addSourceMessage(Error::ASSERT_CONSTANT_FALSE_ERROR.clone(), list![(messageStr.clone()).clone()], info.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// =============================================================================
// Util function at Backend using for lowering and other stuff
//
// =============================================================================
pub fn copyBackendDAE(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    outDAE = mapEqSystem(inDAE.clone(), (std::sync::Arc::new(copyEqSystemTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    assign_field!(outDAE.shared = copyBackendDAEShared(outDAE.shared.clone())?);
    Ok(outDAE)
}

pub fn copyEqSystemTraverser(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    outSystem = copyEqSystem(inSystem.clone())?;
    outShared = inShared.clone();
    Ok((outSystem, outShared))
}

pub fn copyEqSystem(mut inSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>> = None;
    let mut mt: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>> = None;
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    vars = BackendVariable::copyVariables(inSystem.orderedVars.clone());
    eqns = BackendEquation::copyEquationArray(inSystem.orderedEqs.clone());
    removedEqs = BackendEquation::copyEquationArray(inSystem.removedEqs.clone());
    m = AdjacencyMatrix::copyAdjacencyMatrix(inSystem.m.clone());
    mt = AdjacencyMatrix::copyAdjacencyMatrixT(inSystem.mT.clone());
    matching = copyMatching(inSystem.matching.clone())?;
    outSystem = Arc::new(BackendDAE::EqSystem { orderedVars: vars.clone(), orderedEqs: eqns.clone(), m: m.clone(), mT: mt.clone(), mapping: inSystem.mapping.clone(), matching: matching.clone(), stateSets: inSystem.stateSets.clone(), partitionKind: inSystem.partitionKind.clone(), removedEqs: removedEqs.clone() });
    Ok(outSystem)
}

pub fn copyEqSystems(mut inSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>> {
    let mut outSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    for mut e in &*inSystems.clone() {
        let mut e = e.clone();
        outSystems = cons(copyEqSystem(e.clone())?, outSystems.clone());
    }
    Ok(outSystems)
}

pub fn mergeEqSystems(mut System1: Arc<BackendDAE::EqSystem>, mut System2: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut System2: Arc<BackendDAE::EqSystem> = System2;
    assign_field!(
        System2.orderedEqs = BackendEquation::merge(System1.orderedEqs.clone(), System2.orderedEqs.clone())?,
        System2.orderedVars = BackendVariable::mergeVariables(System1.orderedVars.clone(), System2.orderedVars.clone(), true)?
    );
    Ok(System2)
}

pub fn copyBackendDAEShared(mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    outShared = (::match_deref::match_deref! { match &(inShared.clone()) {
        shared @ Deref @ BackendDAE::Shared { .. } => {
            let mut shared = (*shared).clone();
            assign_field!(
                shared.globalKnownVars = BackendVariable::copyVariables(shared.globalKnownVars.clone()),
                shared.externalObjects = BackendVariable::copyVariables(shared.externalObjects.clone()),
                shared.initialEqs = BackendEquation::copyEquationArray(shared.initialEqs.clone()),
                shared.removedEqs = BackendEquation::copyEquationArray(shared.removedEqs.clone())
            );
            shared.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outShared)
}

pub fn copyMatching(mut inMatching: Arc<BackendDAE::Matching>) -> Result<Arc<BackendDAE::Matching>> {
    let mut outMatching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    outMatching = (::match_deref::match_deref! { match &(inMatching.clone()) {
        Deref @ BackendDAE::Matching::NO_MATCHING => {
            Arc::new(crate::BackendDAE::Matching::NO_MATCHING)
        },
        Deref @ BackendDAE::Matching::MATCHING { comps, ass2, ass1 } => {
            let mut cass1: metamodelica::Array<i32>;
            let mut cass2: metamodelica::Array<i32>;
            cass1 = metamodelica::arrayFromVec(ass1.clone().borrow().clone());
            cass2 = metamodelica::arrayFromVec(ass2.clone().borrow().clone());
            Arc::new(BackendDAE::Matching::MATCHING { ass1: cass1.clone(), ass2: cass2.clone(), comps: comps.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMatching)
}

pub fn getCompsOfMatching(mut inMatching: Arc<BackendDAE::Matching>) -> Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> {
    let mut outComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    outComps = (::match_deref::match_deref! { match &(inMatching.clone()) {
        Deref @ BackendDAE::Matching::MATCHING { comps, .. } => {
            comps.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComps
}

pub fn addVarsToEqSystem(mut syst: Arc<BackendDAE::EqSystem>, mut varlst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    osyst = setEqSystVars(syst.clone(), BackendVariable::addVars(varlst.clone(), vars.clone()))?;
    Ok(osyst)
}

pub fn getSimIteratorSize(mut iters: Arc<metamodelica::List<BackendDAE::SimIterator>>) -> Result<i32> {
    let mut size: i32 = 1;
    let mut local_size: i32 = 0;
    for mut iter in &*iters.clone() {
        let mut iter = iter.clone();
        local_size = (match iter.clone() {
        BackendDAE::SimIterator::SIM_ITERATOR_RANGE { .. } => var_field!(iter.non_resizable_size, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone(),
        BackendDAE::SimIterator::SIM_ITERATOR_LIST { .. } => var_field!(iter.size, BackendDAE::SimIterator::SIM_ITERATOR_LIST).clone(),
        _ => bail!("match: no arm matched"),
    });
        size = size.clone() * local_size.clone();
    }
    Ok(size)
}

pub fn numberOfZeroCrossings(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<(i32, i32, i32, i32)> {
    let mut outNumZeroCrossings: i32 = 0;
    let mut outNumTimeEvents: i32 = 0;
    let mut outNumRelations: i32 = 0;
    let mut outNumMathEventFunctions: i32 = 0;
    let mut eventInfo: BackendDAE::EventInfo = inBackendDAE.shared.eventInfo.clone();
    outNumZeroCrossings = ZeroCrossings::length(eventInfo.zeroCrossings.clone())?;
    outNumTimeEvents = (eventInfo.timeEvents.clone().len() as i32);
    outNumRelations = DoubleEnded::length(eventInfo.relations.clone());
    outNumMathEventFunctions = eventInfo.numberMathEvents.clone();
    Ok((outNumZeroCrossings, outNumTimeEvents, outNumRelations, outNumMathEventFunctions))
}

pub fn numberOfDiscreteVars(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<i32> {
    let mut outNumDiscreteReal: i32 = 0;
    outNumDiscreteReal = countDiscreteVars(inBackendDAE.clone())?;
    Ok(outNumDiscreteReal)
}

fn countDiscreteVars(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<i32> {
    let mut outNumDiscreteVars: i32 = 0;
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut alias: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.shared.clone()) {
        Deref @ BackendDAE::Shared { aliasVars: __pa0, globalKnownVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    alias = __pa0.clone();
    globalKnownVars = __pa1.clone();
    outNumDiscreteVars = countDiscreteVars1(inDAE.eqs.clone());
    outNumDiscreteVars = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(countDiscreteVars3, BackendDAE::Var, i32)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<(BackendDAE::Var, i32)> + 'static>), outNumDiscreteVars.clone())?;
    outNumDiscreteVars = BackendVariable::traverseBackendDAEVars(alias.clone(), (std::sync::Arc::new(fnptr!(countDiscreteVars3, BackendDAE::Var, i32)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<(BackendDAE::Var, i32)> + 'static>), outNumDiscreteVars.clone())?;
    Ok(outNumDiscreteVars)
}

fn countDiscreteVars1(mut inEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> i32 {
    let mut outNumDiscreteVars: i32 = 0;
    outNumDiscreteVars = 0;
    outNumDiscreteVars = List::fold(inEqSystems.clone(), (std::sync::Arc::new(countDiscreteVars2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, i32) -> Result<i32> + 'static>), outNumDiscreteVars.clone());
    outNumDiscreteVars
}

fn countDiscreteVars2(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inNumDiscreteVars: i32) -> Result<i32> {
    let mut outNumDiscreteVars: i32 = 0;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(inEqSystem.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    outNumDiscreteVars = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(fnptr!(countDiscreteVars3, BackendDAE::Var, i32)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<(BackendDAE::Var, i32)> + 'static>), inNumDiscreteVars.clone())?;
    Ok(outNumDiscreteVars)
}

fn countDiscreteVars3(mut var: BackendDAE::Var, mut nDiscreteVars: i32) -> (BackendDAE::Var, i32) {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outCount: i32 = 0;
    (outVar, outCount) = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, varKind: BackendDAE::VarKind::DISCRETE, .. } => (var.clone(), nDiscreteVars.clone() + 1),
        _ => (var.clone(), nDiscreteVars.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outVar, outCount)
}

pub fn replaceCrefsWithValues(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, Arc<DAE::ComponentRef>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, Arc<DAE::ComponentRef>);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (vars, cr_orign)) => {
                    if !((!(ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr_orign.clone())?))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { bindExp: Some(__pa0), .. }, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(replaceCrefsWithValues) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>))> + 'static>), (vars.clone(), cr_orign.clone()))?;
                    Ok((e.clone(), (vars.clone(), cr_orign.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

pub fn makeExpType(mut inType: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = inType.clone();
    outType
}

pub fn hasExpContinuousParts(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables, mut inKnvars: BackendDAE::Variables) -> Result<bool> {
    let mut outBoolean: bool = false;
    let (_, (_, _, __pa0)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingContinuousExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (inVariables.clone(), inKnvars.clone(), false))?;
    outBoolean = __pa0.clone();
    Ok(outBoolean)
}

fn traversingContinuousExpFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, BackendDAE::Variables, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, globalKnownVars, _)) => {
                    let mut backendVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    backendVar = __pa0.clone();
                    let false = (BackendVariable::isVarDiscrete(backendVar.clone())) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), false, (vars.clone(), globalKnownVars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, (vars, globalKnownVars, _)) => {
                    Ok((inExp.clone(), false, (vars.clone(), globalKnownVars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, globalKnownVars, _)) => {
                    let mut backendVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    backendVar = __pa0.clone();
                    let true = (BackendVariable::isInput(backendVar.clone())) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), false, (vars.clone(), globalKnownVars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, _) => {
                    if !((stringEq((literal!("pre")).clone(), (name.clone()).clone()) || stringEq((literal!("change")).clone(), (name.clone()).clone()) || stringEq((literal!("ceil")).clone(), (name.clone()).clone()) || stringEq((literal!("floor")).clone(), (name.clone()).clone()))) { bail!("guard") }
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, (vars, globalKnownVars, _)) => {
                    Ok((inExp.clone(), false, (vars.clone(), globalKnownVars.clone(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

pub fn statesAndVarsExp(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (_, (_, __pa0)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingstatesAndVarsExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>), (inVariables.clone(), metamodelica::nil()))?;
    exps = __pa0.clone();
    Ok(exps)
}

pub fn traversingstatesAndVarsExpFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst, .. }, componentRef: cr }, (vars, _)) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut creexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    creexps = List::map1(varLst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone());
                    let (_, (_, __pa0)) = Expression::traverseExpListTopDown(creexps.clone(), (std::sync::Arc::new(traversingstatesAndVarsExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>), inTpl.clone())?;
                    res = __pa0.clone();
                    Ok((e.clone(), true, (vars.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, (vars, _)) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    let (_, (_, __pa1)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingstatesAndVarsExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>), inTpl.clone())?;
                    res = __pa1.clone();
                    Ok((e.clone(), true, (vars.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, expl)) => {
                    BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok((e.clone(), false, (vars.clone(), cons(e.clone(), expl.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, expl)) => {
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((e.clone(), false, (vars.clone(), cons(e.clone(), expl.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, _)) => {
                    BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

pub fn isLoopDependent(mut varExp: Arc<DAE::Exp>, mut iteratorExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut isDependent: bool = false;
    isDependent = 'mc: {
        let __mc_input = varExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut subscript_exprs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    subscripts = ComponentReferenceBasics::crefSubs(cr.clone())?;
                    subscript_exprs = List::map(subscripts.clone(), (std::sync::Arc::new(ExpressionBasics::subscriptIndexExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> + 'static>));
                    let true = (isLoopDependentHelper(subscript_exprs.clone(), iteratorExp.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { sub: subscripts, .. } => {
                    let mut subscript_exprs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    subscript_exprs = List::map(subscripts.clone(), (std::sync::Arc::new(Expression::getSubscriptExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> + 'static>));
                    Ok(isLoopDependentHelper(subscript_exprs.clone(), iteratorExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isDependent)
}

fn isLoopDependentHelper(mut subscripts: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iteratorExp: Arc<DAE::Exp>) -> bool {
    let mut isDependent: bool = false;
    for mut subscript in &*subscripts.clone() {
        let mut subscript = subscript.clone();
        if '__try0: {
            if unwrap_break_err!(Expression::expContains(subscript.clone(), iteratorExp.clone()), '__try0) {
                isDependent = true;
                return isDependent.clone();
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    isDependent
}

pub fn devectorizeArrayVar(mut arrayVar: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut newArrayVar: Arc<DAE::Exp>;
    newArrayVar = 'mc: {
        let __mc_input = arrayVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { sub: subs, exp: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: _ }, .. } } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    e = Expression::crefExp(cr.clone())?;
                    Ok(Expression::makeASUB(e.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { sub: subs, exp: Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: _ }, tail: _ }, .. } } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    e = Expression::crefExp(cr.clone())?;
                    Ok(Expression::makeASUB(e.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(arrayVar.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(newArrayVar)
}

pub fn explodeArrayVars(mut arrayVar: Arc<DAE::Exp>, mut iteratorExp: Arc<DAE::Exp>, mut rangeExpr: Arc<DAE::Exp>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut arrayElements: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    arrayElements = 'mc: {
        let __mc_input = arrayVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    let mut clonedElements: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut newElements: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut indices: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    indices = rangeExprs(rangeExpr.clone())?;
                    clonedElements = List::fill(arrayVar.clone(), (indices.clone().len() as i32));
                    newElements = generateArrayElements(clonedElements.clone(), indices.clone(), iteratorExp.clone())?;
                    Ok(newElements.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { .. }, .. } => {
                    let mut clonedElements: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut newElements: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut indices: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    indices = rangeExprs(rangeExpr.clone())?;
                    clonedElements = List::fill(arrayVar.clone(), (indices.clone().len() as i32));
                    newElements = generateArrayElements(clonedElements.clone(), indices.clone(), iteratorExp.clone())?;
                    Ok(newElements.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
                    let mut varExprs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut bvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (bvars, _) = BackendVariable::getVar(cref.clone(), vars.clone())?;
                    varExprs = List::map(bvars.clone(), (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>));
                    Ok(varExprs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, .. } => {
                    let mut varExprs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut bvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (bvars, _) = BackendVariable::getVar(cref.clone(), vars.clone())?;
                    varExprs = List::map(bvars.clone(), (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>));
                    Ok(varExprs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: daeExp, .. } => {
                    let mut varExprs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    varExprs = Expression::flattenArrayExpToList(daeExp.clone())?;
                    Ok(varExprs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(arrayElements)
}

fn rangeExprs(mut inRange: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outValues: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outValues = (::match_deref::match_deref! { match &(inRange.clone()) {
        Deref @ DAE::Exp::ARRAY { array: arrayElements, .. } => {
            arrayElements.clone()
        },
        Deref @ DAE::Exp::RANGE { .. } => {
            Expression::expandRange(inRange.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValues)
}

pub fn daeSize(mut inDAE: Arc<BackendDAE::BackendDAE>) -> i32 {
    let mut sz: i32 = 0;
    sz = ({
        let mut __acc: i32 = 0;
        for mut s in (inDAE.eqs.clone()).into_iter().cloned() {
            let __x = systemSize(s.clone());
            __acc += __x;
        }
        __acc
    });
    sz
}

pub fn systemSize(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> i32 {
    let mut outSize: i32 = BackendEquation::equationArraySize(inEqSystem.orderedEqs.clone()).unwrap();
    outSize
}

pub fn maxSizeOfEqSystems(mut inEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> i32 {
    let mut outSize: i32 = 0;
    let mut i: i32 = 0;
    for mut eqSyst in &*inEqSystems.clone() {
        let mut eqSyst = eqSyst.clone();
        i = systemSize(eqSyst.clone());
        if intGt(i.clone(), outSize.clone()) {
            outSize = i.clone();
        }
    }
    outSize
}

pub fn numOfComps(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<i32> {
    let mut num: i32 = 0;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inEqSystem.matching.clone()) {
        Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    num = (comps.clone().len() as i32);
    Ok(num)
}

pub fn equationArraySizeBDAE(mut inDAE: Arc<BackendDAE::BackendDAE>) -> i32 {
    let mut outSize: i32 = 0;
    outSize = List::applyAndFold(inDAE.eqs.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(equationArraySizeDAE, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<i32> + 'static>), 0);
    outSize
}

pub fn equationArraySizeDAE(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> i32 {
    let mut n: i32 = BackendEquation::getNumberOfEquations(inEqSystem.orderedEqs.clone());
    n
}

pub fn hasDAEMatching(mut inDAE: Arc<BackendDAE::BackendDAE>) -> bool {
    let mut b: bool = false;
    b = List::applyAndFold(inDAE.eqs.clone(), (std::sync::Arc::new(fnptr!(boolAnd, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(hasEqSystemMatching) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<bool> + 'static>), true);
    b
}

pub fn hasEqSystemMatching(mut dae: Arc<BackendDAE::EqSystem>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. } => true,
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::NO_MATCHING, .. } => false,
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

fn generateArrayElements(mut clones: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut indices: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iteratorExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut newElements: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    newElements = (::match_deref::match_deref! { match &((clones.clone(), indices.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: clone, tail: restClones }, Deref @ metamodelica::List::Cons { head: index, tail: restIndices }) => {
            let mut newElement: Arc<DAE::Exp>;
            let mut newElement2: Arc<DAE::Exp>;
            let mut elements: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (newElement, _) = Expression::replaceExp(clone.clone(), iteratorExp.clone(), index.clone())?;
            newElement2 = simplifySubscripts(newElement.clone())?;
            elements = generateArrayElements(restClones.clone(), restIndices.clone(), iteratorExp.clone())?;
            cons(newElement2.clone(), elements.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newElements)
}

fn simplifySubscripts(mut asub: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut maybeCref: Arc<DAE::Exp>;
    maybeCref = 'mc: {
        let __mc_input = asub.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: varIdent, identType: arrayType, subscriptLst: subscripts }, ty: varType } => {
                    let mut newCrefExp: Arc<DAE::Exp>;
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut subscripts = (*subscripts).clone();
                    subscripts = List::map(subscripts.clone(), (std::sync::Arc::new(simplifySubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Subscript>> + 'static>));
                    cref_ = ComponentReferenceBasics::makeCrefIdent((varIdent.clone()).clone(), arrayType.clone(), subscripts.clone());
                    newCrefExp = Expression::makeCrefExp(cref_.clone(), varType.clone())?;
                    Ok(newCrefExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: varIdent, identType: arrayType, subscriptLst: _ }, ty: varType }, sub: subscripts } => {
                    let mut subExprs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut subExprsSimplified: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut newCrefExp: Arc<DAE::Exp>;
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut subscripts = (*subscripts).clone();
                    subExprs = List::map(subscripts.clone(), (std::sync::Arc::new(Expression::getSubscriptExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> + 'static>));
                    ::match_deref::match_deref! { match &(List::select(subExprs.clone(), (std::sync::Arc::new(Expression::isNotConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    subExprsSimplified = ExpressionSimplify::simplifyList(subExprs.clone())?;
                    subscripts = List::map(subExprsSimplified.clone(), (std::sync::Arc::new(fnptr!(Expression::makeIndexSubscript, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Subscript>> + 'static>));
                    cref_ = ComponentReferenceBasics::makeCrefIdent((varIdent.clone()).clone(), arrayType.clone(), subscripts.clone());
                    newCrefExp = Expression::makeCrefExp(cref_.clone(), varType.clone())?;
                    Ok(newCrefExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(asub.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(maybeCref)
}

fn simplifySubscript(mut sub: Arc<DAE::Subscript>) -> Result<Arc<DAE::Subscript>> {
    let mut simplifiedSub: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    simplifiedSub = 'mc: {
        let __mc_input = sub.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Subscript::INDEX { exp: e } => {
                    let mut e1: Arc<DAE::Exp>;
                    (e1, _) = ExpressionSimplify::simplify(e.clone())?;
                    Ok(if (referenceEq(&e1.clone(),&e.clone())) {sub.clone()} else {Arc::new(DAE::Subscript::INDEX { exp: e.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(sub.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(simplifiedSub)
}

pub fn setTearingSelectAttribute(mut optComment: Option<Arc<SCode::Comment>>) -> Result<Option<BackendDAE::TearingSelect>> {
    let mut tearingSelect: Option<BackendDAE::TearingSelect> = None;
    let mut opt_anno: Option<Arc<SCode::Annotation>> = None;
    let mut anno: Arc<SCode::Annotation> = Arc::new(<SCode::Annotation as ::std::default::Default>::default());
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut opt_val: Option<Arc<Absyn::Exp>> = None;
    let mut val: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut name: ArcStr = arcstr::literal!("");
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    opt_anno = SCodeUtil::optCommentAnnotation(optComment.clone());
    if isNone(opt_anno.clone()) {
        return Ok(tearingSelect.clone());
    }
    let __pa0 = ::match_deref::match_deref! { match &(opt_anno.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    anno = __pa0.clone();
    r#mod = SCodeUtil::lookupAnnotation(anno.clone(), (literal!("__OpenModelica_tearingSelect")).clone())?;
    if SCodeUtil::isEmptyMod(r#mod.clone()) {
        r#mod = SCodeUtil::lookupAnnotation(anno.clone(), (literal!("tearingSelect")).clone())?;
        if !(SCodeUtil::isEmptyMod(r#mod.clone())) {
            Error::addSourceMessage(Error::DEPRECATED_EXPRESSION.clone(), list![(literal!("tearingSelect")).clone(), (literal!("__OpenModelica_tearingSelect")).clone()], SCodeUtil::getModifierInfo(r#mod.clone()))?;
        }
    }
    opt_val = SCodeUtil::getModifierBinding(r#mod.clone());
    if isNone(opt_val.clone()) {
        return Ok(tearingSelect.clone());
    }
    let __pa1 = ::match_deref::match_deref! { match &(opt_val.clone()) {
        Some(__pa1) => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    val = __pa1.clone();
    info = SCodeUtil::getModifierInfo(r#mod.clone());
    name = (getTearingSelectName(val.clone(), info.clone())?).clone();
    tearingSelect = lookupTearingSelectMember((name.clone()).clone());
    if isNone(tearingSelect.clone()) {
        Error::addSourceMessage(Error::UNKNOWN_ANNOTATION_VALUE.clone(), list![(Dump::printExpStr(val.clone())?).clone(), (literal!("__OpenModelica_tearingSelect")).clone()], info.clone())?;
    }
    Ok(tearingSelect)
}

fn getTearingSelectName(mut exp: Arc<Absyn::Exp>, mut info: SourceInfo) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name }, subscripts: Deref @ metamodelica::List::Nil, name: Deref @ "TearingSelect" } } => name.clone(),
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name } } => {
            Error::addSourceMessage(Error::DEPRECATED_EXPRESSION.clone(), list![(name.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TearingSelect.")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
            literal!("")
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(name)
}

fn lookupTearingSelectMember(mut name: ArcStr) -> Option<BackendDAE::TearingSelect> {
    let mut tearingSelect: Option<BackendDAE::TearingSelect> = None;
    tearingSelect = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "never" => Some(crate::BackendDAE::TearingSelect::NEVER),
        Deref @ "avoid" => Some(crate::BackendDAE::TearingSelect::AVOID),
        Deref @ "default" => Some(crate::BackendDAE::TearingSelect::DEFAULT),
        Deref @ "prefer" => Some(crate::BackendDAE::TearingSelect::PREFER),
        Deref @ "always" => Some(crate::BackendDAE::TearingSelect::ALWAYS),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    tearingSelect
}

pub fn setHideResultAttribute(mut comment: Option<Arc<SCode::Comment>>, mut inCref: Arc<DAE::ComponentRef>) -> Option<Arc<DAE::Exp>> {
    let mut hideResult: Option<Arc<DAE::Exp>> = None;
    let mut hr: Arc<DAE::Exp>;
    let mut ann: Arc<SCode::Annotation> = Arc::new(<SCode::Annotation as ::std::default::Default>::default());
    let mut val: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut crefRoot: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(comment.clone()) {
            Some(Deref @ SCode::Comment { annotation_: Some(__pa1), .. }) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        ann = __pa1.clone();
        let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(SCodeUtil::lookupAnnotationBinding(ann.clone(), (literal!("HideResult")).clone()), '__try0)) {
            Some(__pa2) => __pa2.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        val = __pa2.clone();
        hr = unwrap_break_err!(Expression::fromAbsynExp(val.clone()), '__try0);
        hideResult = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            (crefRoot, _) = unwrap_break_err!(ComponentReference::splitCrefLast(inCref.clone()), '__try0);
            (hr, _) = unwrap_break_err!(Expression::traverseExpBottomUp(hr.clone(), (std::sync::Arc::new(ComponentReference::joinCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)> + 'static>), crefRoot.clone()), '__try0);
            Some(hr.clone())
        },
        _ => Some(hr.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok::<_, anyhow::Error>((hideResult.clone(),))
    } {
        Ok((__try0_o0,)) => {
            hideResult = __try0_o0;
        }
        Err(_) => {
            hideResult = None;
        }
    }
    hideResult
}

/* ******************************************
   Functions that deals with BackendDAE as input
********************************************/
pub fn blockIsDynamic(mut lst: Arc<metamodelica::List<i32>>, mut arr: metamodelica::Array<i32>) -> bool {
    let mut outBoolean: bool = true;
    for mut x in &*lst.clone() {
        let mut x = x.clone();
        if arr.borrow()[(x.clone()-1) as usize].clone() != 0 {
            return outBoolean.clone();
        }
    }
    outBoolean = false;
    outBoolean
}

pub fn markStateEquations(mut syst: Arc<BackendDAE::EqSystem>, mut arr: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut outIntegerArray: metamodelica::Array<i32>;
    let mut statevarindx_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut v: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { m: Some(__pa0), orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    v = __pa1.clone();
    if Flags::getConfigEnum(Flags::SYM_SOLVER.clone())? > 0 {
        (_, statevarindx_lst) = BackendVariable::getAllVarIndicesFromVariables(v.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isAlgState, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
    } else {
        (_, statevarindx_lst) = BackendVariable::getAllVarIndicesFromVariables(v.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
    }
    eqns = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (statevarindx_lst.clone()).into_iter().cloned() {
            if !(ass1.clone().borrow()[(i.clone()-1) as usize].clone() > 0) { continue; }
            let __x = ass1.clone().borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outIntegerArray = markStateEquationsWork(eqns.clone(), m.clone(), ass1.clone(), arr.clone())?;
    Ok(outIntegerArray)
}

pub fn markZeroCrossingEquations(mut syst: Arc<BackendDAE::EqSystem>, mut inZeroCross: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, mut arr: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    type CheckEquationsVarsExpTopDownFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>;

    let mut outIntegerArray: metamodelica::Array<i32>;
    let mut varindx_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut v: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut tree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    let mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { m: Some(__pa0), orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    v = __pa1.clone();
    tree = AvlSetInt::new();
    func = Arc::new({ let __pe_b2 = v.clone(); move |__pe_a0, __pe_a1| Ok(BackendEquation::checkEquationsVarsExpTopDown(__pe_a0, __pe_a1, __pe_b2.clone())) });
    for mut zc in &*inZeroCross.clone() {
        let mut zc = zc.clone();
        tree = varsCollector(zc.relation_.clone(), tree.clone(), func.clone())?;
    }
    varindx_lst = AvlSetInt::listKeys(tree.clone(), metamodelica::nil());
    eqns = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (varindx_lst.clone()).into_iter().cloned() {
            if !(ass1.clone().borrow()[(i.clone()-1) as usize].clone() > 0) { continue; }
            let __x = ass1.clone().borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outIntegerArray = markStateEquationsWork(eqns.clone(), m.clone(), ass1.clone(), arr.clone())?;
    Ok(outIntegerArray)
}

fn varsCollector(mut exp: Arc<DAE::Exp>, mut tree: Arc<AvlSetInt::Tree>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>) -> Result<Arc<AvlSetInt::Tree>> {
    pub type CheckEquationsVarsExpTopDownFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>;

    let mut tree: Arc<AvlSetInt::Tree> = tree;
    tree = BackendEquation::expressionVarsIndexes(exp.clone(), tree.clone(), func.clone())?;
    Ok(tree)
}

fn markStateEquationsWork(mut inEqns: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut iMark: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oMark: metamodelica::Array<i32> = iMark.clone();
    let mut queue: Arc<metamodelica::List<i32>> = inEqns.clone();
    let mut j: i32 = 0;
    let mut eqn: i32 = 0;
    let mut len: i32 = (ass1.clone().borrow().len() as i32);
    let mut positiveAndUnmarked: bool = false;
    while !(queue.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(queue.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        eqn = __pa0.clone();
        queue = __pa1.clone();
        if oMark.borrow()[(eqn.clone()-1) as usize].clone() == 0 {
            {let _arr = oMark.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = 1; _arr};
            let __range2 = &*m.borrow()[(eqn.clone()-1) as usize].clone();
            for mut i in __range2 {
                let mut i = i.clone();
                if i.clone() > 0 && i.clone() <= len.clone() {
                    j = Dangerous::arrayGet(ass1.clone(), i.clone())?;
                    positiveAndUnmarked = if (j.clone() > 0) {oMark.clone().borrow()[(j.clone()-1) as usize].clone() == 0} else {false};
                    if positiveAndUnmarked.clone() {
                        queue = cons(j.clone(), queue.clone());
                    }
                }
            }
        }
    }
    Ok(oMark)
}

pub fn removeNegative(mut lst: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut lst_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    lst_1 = List::select(lst.clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
    lst_1
}

pub fn eqnsForVarWithStates(mut inAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inInteger: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outIntegerLst = 'mc: {
        let __mc_input = (inAdjacencyMatrixT.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut mt, mut n) = __mc_input.clone() else { bail!("nomatch") };
            let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut res_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            res = mt.borrow()[(n.clone()-1) as usize].clone();
            res_1 = List::map(res.clone(), Arc::new(fnptr!(intAbs, i32)));
            Ok(res_1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut indx) = __mc_input.clone() else { bail!("nomatch") };
            let mut s: ArcStr = arcstr::literal!("");
            println!("{}", (literal!("- BackendDAEUtil.eqnsForVarWithStates failed, indx= ")).clone());
            s = (intString(indx.clone())).clone();
            println!("{}", (s.clone()).clone());
            println!("{}", (literal!("\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIntegerLst)
}

pub fn varsInEqn(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut indx: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outIntegerLst = 'mc: {
        let __mc_input = indx.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(removeNegative(m.borrow()[(indx.clone()-1) as usize].clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut s: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAEUtil.varsInEqn failed, indx= ")); __mm_s.push_str(&*intString(indx.clone())); __mm_s.push_str(&*literal!("array length: ")); __mm_s.push_str(&*intString((m.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(s.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIntegerLst)
}

pub type TraverseIndicesTuple = (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);

pub fn setEvaluationStage(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let mut indicesDynamic: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut indicesDiscrete: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut indicesAlgebraic: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut traverseArgs: TraverseIndicesTuple;
    let mut assigndEqn: metamodelica::Array<i32>;
    let mut assigndVar: metamodelica::Array<i32>;
    let mut markedEqns: metamodelica::Array<i32>;
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut adjMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut adjMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut zeroCrossings: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let debug: bool = false;
    zeroCrossings = ZeroCrossings::toList(inBackendDAE.shared.eventInfo.zeroCrossings.clone());
    for mut eqSystem in &*inBackendDAE.eqs.clone() {
        let mut eqSystem = eqSystem.clone();
        if debug.clone() {
            BackendDump::printEqSystem(eqSystem.clone())?;
        }
        vars = eqSystem.orderedVars.clone();
        eqns = eqSystem.orderedEqs.clone();
        match '__try0: {
            let (__pa3, __pa1, __pa2) = ::match_deref::match_deref! { match &(eqSystem.matching.clone()) {
                __pa3 @ Deref @ BackendDAE::Matching::MATCHING { ass1: __pa1, ass2: __pa2, .. } => (__pa3.clone(), __pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            assigndVar = __pa1.clone();
            assigndEqn = __pa2.clone();
            matching = __pa3.clone();
            (eqSystem, adjMatrix, adjMatrixT) = unwrap_break_err!(getAdjacencyMatrixfromOption(eqSystem.clone(), crate::BackendDAE::IndexType::ABSOLUTE, Some(inBackendDAE.shared.functionTree.clone()), isInitializationDAE(inBackendDAE.shared.clone())), '__try0);
            traverseArgs = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
            traverseArgs = unwrap_break_err!(traverseEqSystemStrongComponents(eqSystem.clone(), (std::sync::Arc::new(collectEqnsIndexByKind) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), traverseArgs.clone()), '__try0);
            (indicesDynamic, indicesDiscrete, indicesAlgebraic) = traverseArgs.clone();
            if debug.clone() {
                println!("{}", (literal!("Dynamic equation indicies:\n")).clone());
                println!("{}", stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (indicesDynamic.clone()).into_iter().cloned() {
            let __x = intString(i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone()));
                println!("{}", (literal!("\n")).clone());
            }
            markedEqns = arrayCreate(BackendEquation::getNumberOfEquations(eqns.clone()), 0);
            markedEqns = unwrap_break_err!(markStateEquationsWork(indicesDynamic.clone(), adjMatrix.clone(), assigndVar.clone(), markedEqns.clone()), '__try0);
            eqns = unwrap_break_err!(setMarkedEqnsEvalStage(eqns.clone(), markedEqns.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::setEvalStageDynamic, BackendDAE::EvaluationStages)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>)), '__try0);
            markedEqns = arrayCreate(BackendEquation::getNumberOfEquations(eqns.clone()), 0);
            markedEqns = unwrap_break_err!(markZeroCrossingEquations(eqSystem.clone(), zeroCrossings.clone(), markedEqns.clone(), assigndVar.clone()), '__try0);
            eqns = unwrap_break_err!(setMarkedEqnsEvalStage(eqns.clone(), markedEqns.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::setEvalStageZeroCross, BackendDAE::EvaluationStages)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>)), '__try0);
            markedEqns = arrayCreate(BackendEquation::getNumberOfEquations(eqns.clone()), 0);
            markedEqns = unwrap_break_err!(markStateEquationsWork(indicesAlgebraic.clone(), adjMatrix.clone(), assigndVar.clone(), markedEqns.clone()), '__try0);
            eqns = unwrap_break_err!(setMarkedEqnsEvalStage(eqns.clone(), markedEqns.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::setEvalStageAlgebraic, BackendDAE::EvaluationStages)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>)), '__try0);
            markedEqns = arrayCreate(BackendEquation::getNumberOfEquations(eqSystem.removedEqs.clone()), 1);
            assign_field!(eqSystem.removedEqs = unwrap_break_err!(setMarkedEqnsEvalStage(eqSystem.removedEqs.clone(), markedEqns.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::setEvalStageDiscrete, BackendDAE::EvaluationStages)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>)), '__try0));
            markedEqns = arrayCreate(BackendEquation::getNumberOfEquations(eqns.clone()), 1);
            eqns = unwrap_break_err!(setMarkedEqnsEvalStage(eqns.clone(), markedEqns.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::setEvalStageDiscrete, BackendDAE::EvaluationStages)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>)), '__try0);
            Ok::<_, anyhow::Error>((eqns.clone(), markedEqns.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                eqns = __try0_o0;
                markedEqns = __try0_o1;
            }
            Err(_) => {
                markedEqns = arrayCreate(BackendEquation::getNumberOfEquations(eqns.clone()), 1);
                eqns = setMarkedEqnsEvalStage(eqns.clone(), markedEqns.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::setEvalStageAll, BackendDAE::EvaluationStages)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>))?;
            }
        }
        if debug.clone() {
            BackendDump::dumpEquationArray(eqns.clone(), (literal!("Updated equations")).clone())?;
        }
        newEqs = cons(eqSystem.clone(), newEqs.clone());
    }
    outBackendDAE = BackendDAE::DAE(newEqs.clone(), inBackendDAE.shared.clone())?;
    Ok(outBackendDAE)
}

fn setMarkedEqnsEvalStage(mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut markEqns: metamodelica::Array<i32>, mut func: Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    pub type setEvalStage = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>;

    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = eqns;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let __range0 = 1..=(markEqns.clone().borrow().len() as i32);
    for mut i in __range0 {
        if markEqns.borrow()[(i.clone()-1) as usize].clone() > 0 {
            eqn = BackendEquation::get(eqns.clone(), i.clone());
            eqn = BackendEquation::setEquationEvalStage(eqn.clone(), func.clone())?;
            eqns = BackendEquation::setAtIndex(eqns.clone(), i.clone(), eqn.clone())?;
        }
    }
    Ok(eqns)
}

fn collectEqnsIndexByKind(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut varIdxs: Arc<metamodelica::List<i32>>, mut eqnIdxs: Arc<metamodelica::List<i32>>, mut traverserArgs: TraverseIndicesTuple) -> Result<TraverseIndicesTuple> {
    let mut traverserArgs: TraverseIndicesTuple = traverserArgs;
    let mut indicesDynamic: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut indicesDiscrete: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut indicesAlgebraic: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (indicesDynamic, indicesDiscrete, indicesAlgebraic) = traverserArgs.clone();
    for mut v in &*inVars.clone() {
        let mut v = v.clone();
        if BackendVariable::isVarDiscrete(v.clone()) {
            indicesDiscrete = listAppend(eqnIdxs.clone(), indicesDiscrete.clone());
        }
    }
    for mut eq in &*inEqns.clone() {
        let mut eq = eq.clone();
        if BackendEquation::isDynamicEquation(eq.clone())? {
            indicesDynamic = listAppend(eqnIdxs.clone(), indicesDynamic.clone());
        }
        if BackendEquation::isAuxEquation(eq.clone())? {
            indicesAlgebraic = listAppend(eqnIdxs.clone(), indicesAlgebraic.clone());
        }
    }
    traverserArgs = (indicesDynamic.clone(), indicesDiscrete.clone(), indicesAlgebraic.clone());
    Ok(traverserArgs)
}

pub fn subscript2dCombinations(mut inExpSubscriptLstLst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>, mut inExpSubscriptLstLst2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>> {
    let mut outExpSubscriptLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
    outExpSubscriptLstLst = (::match_deref::match_deref! { match &((inExpSubscriptLstLst1.clone(), inExpSubscriptLstLst2.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: s1, tail: ss }, ss2) => {
            let mut lst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            let mut lst2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            lst1 = subscript2dCombinations2(s1.clone(), ss2.clone())?;
            lst2 = subscript2dCombinations(ss.clone(), ss2.clone())?;
            res = listAppend(lst1.clone(), lst2.clone());
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpSubscriptLstLst)
}

fn subscript2dCombinations2(mut inExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inExpSubscriptLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>> {
    let mut outExpSubscriptLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
    outExpSubscriptLstLst = (::match_deref::match_deref! { match &((inExpSubscriptLst.clone(), inExpSubscriptLstLst.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (ss, Deref @ metamodelica::List::Cons { head: s2, tail: ss2 }) => {
            let mut lst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            let mut elt1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            lst1 = subscript2dCombinations2(ss.clone(), ss2.clone())?;
            elt1 = listAppend(ss.clone(), s2.clone());
            cons(elt1.clone(), lst1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpSubscriptLstLst)
}

pub fn splitoutEquationAndVars(mut inNeededBlocks: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inEqnsNew: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVarsNew: BackendDAE::Variables) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    (outEqns, outVars) = (::match_deref::match_deref! { match &((inNeededBlocks.clone(), inEqnsNew.clone(), inVarsNew.clone())) {
        (Deref @ metamodelica::List::Nil, eqnsNew, varsNew) => {
            (eqnsNew.clone(), varsNew.clone())
        },
        (Deref @ metamodelica::List::Cons { head: comp, tail: rest }, eqnsNew, varsNew) => {
            let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqnsNew = (*eqnsNew).clone();
            let mut varsNew = (*varsNew).clone();
            (eqnsNew, varsNew) = splitoutEquationAndVars(rest.clone(), inEqns.clone(), inVars.clone(), eqnsNew.clone(), varsNew.clone())?;
            (eqn_lst, var_lst, _) = BackendDAETransform::getEquationAndSolvedVar(comp.clone(), inEqns.clone(), inVars.clone())?;
            eqnsNew = BackendEquation::addList(eqn_lst.clone(), eqnsNew.clone())?;
            varsNew = BackendVariable::addVars(var_lst.clone(), varsNew.clone());
            (eqnsNew.clone(), varsNew.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outEqns, outVars))
}

pub fn getStrongComponents(mut syst: Arc<BackendDAE::EqSystem>) -> Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> {
    let mut outComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    outComps = (::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: outComps, .. }, .. } => outComps.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComps
}

pub fn getFunctions(mut shared: Arc<BackendDAE::Shared>) -> Result<Arc<AvlTreePathFunction::Tree>> {
    let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let __pa0 = ::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { functionTree: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    functionTree = __pa0.clone();
    Ok(functionTree)
}

pub fn getGlobalKnownVarsFromShared(mut shared: Arc<BackendDAE::Shared>) -> Result<BackendDAE::Variables> {
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { globalKnownVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    globalKnownVars = __pa0.clone();
    Ok(globalKnownVars)
}

pub fn getExtraInfo(mut shared: Arc<BackendDAE::Shared>) -> Result<BackendDAE::ExtraInfo> {
    let mut einfo: BackendDAE::ExtraInfo = <BackendDAE::ExtraInfo as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { info: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    einfo = __pa0.clone();
    Ok(einfo)
}

pub fn reduceEqSystemsInDAE(mut inDAE: Arc<BackendDAE::BackendDAE>, mut iVarlst: Arc<metamodelica::List<BackendDAE::Var>>, mut makeMatching: bool, mut filterDiscretes: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    outDAE = BackendDAE::DAE(({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
        for mut syst in (systs.clone()).into_iter().cloned() {
            let __x = tryReduceEqSystem(syst.clone(), shared.clone(), iVarlst.clone(), filterDiscretes.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), shared.clone())?;
    if makeMatching.clone() {
        outDAE = transformBackendDAE(outDAE.clone(), Some((crate::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, crate::BackendDAE::EquationConstraints::EXACT)), None, None)?;
    }
    Ok(outDAE)
}

pub fn tryReduceEqSystem(mut iSyst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut iVarlst: Arc<metamodelica::List<BackendDAE::Var>>, mut filterDiscretes: bool) -> Arc<BackendDAE::EqSystem> {
    let mut oSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    match '__try0: {
        oSyst = unwrap_break_err!(reduceEqSystem(iSyst.clone(), shared.clone(), iVarlst.clone(), filterDiscretes.clone()), '__try0);
        Ok::<_, anyhow::Error>((oSyst.clone(),))
    } {
        Ok((__try0_o0,)) => {
            oSyst = __try0_o0;
        }
        Err(_) => {
            oSyst = iSyst.clone();
        }
    }
    oSyst
}

pub fn reduceEqSystem(mut iSyst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut iVarlst: Arc<metamodelica::List<BackendDAE::Var>>, mut filterDiscretes: bool) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut oSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut iVars: BackendDAE::Variables = BackendVariable::listVar(iVarlst.clone());
    let mut ordererdEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut arrEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut indx_lst_v: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut indx_lst_e: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut statevarindx_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut indx_arr: metamodelica::Array<i32>;
    let mut el: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vl: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    oSyst = (::match_deref::match_deref! { match &(iSyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2, ass1, .. }, orderedVars: vars, orderedEqs: ordererdEqs, .. } => {
            let mut syst = (*syst).clone();
            if Flags::getConfigEnum(Flags::SYM_SOLVER.clone())? > 0 {
                (_, statevarindx_lst) = BackendVariable::getAllVarIndicesFromVariables(vars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isAlgState, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
            } else {
                (_, statevarindx_lst) = BackendVariable::getAllVarIndicesFromVariables(vars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
            }
            indx_lst_v = BackendVariable::getVarIndexFromVariables(iVars.clone(), vars.clone())?;
            indx_lst_v = listAppend(indx_lst_v.clone(), statevarindx_lst.clone());
            indx_lst_e = List::map1r(indx_lst_v.clone(), Arc::new(arrayGet.clone()), ass1.clone());
            indx_arr = arrayCreate(equationArraySizeDAE(iSyst.clone()), 0);
            funcs = getFunctions(shared.clone())?;
            (_, m, _) = getAdjacencyMatrix(iSyst.clone(), crate::BackendDAE::IndexType::SPARSE, Some(funcs.clone()), isInitializationDAE(shared.clone()))?;
            indx_arr = markStateEquationsWork(indx_lst_e.clone(), m.clone(), ass1.clone(), indx_arr.clone())?;
            indx_lst_e = Array::foldIndex(indx_arr.clone(), (std::sync::Arc::new(fnptr!(translateArrayList, i32, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), metamodelica::nil())?;
            el = BackendEquation::getList(indx_lst_e.clone(), ordererdEqs.clone());
            if filterDiscretes.clone() {
                el = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            if !(!(BackendEquation::isDiscreteEquation(e.clone())?)) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
            arrEqs = BackendEquation::listEquation(el.clone())?;
            vl = BackendEquation::equationsVars(arrEqs.clone(), vars.clone())?;
            if filterDiscretes.clone() {
                vl = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (vl.clone()).into_iter().cloned() {
            if !(!(BackendVariable::isVarDiscrete(v.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
            assign_field!(
                syst.orderedVars = BackendVariable::listVar1(vl.clone()),
                syst.orderedEqs = arrEqs.clone(),
                syst.stateSets = metamodelica::nil()
            );
            clearEqSyst(syst.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oSyst)
}

pub fn introduceOutputRealDerivatives(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut currentSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut newCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut newEqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut daeVarsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    currentSystem = __pa0.clone();
    daeVarsLst = metamodelica::nil();
    newEqnlst = metamodelica::nil();
    for mut var in &*BackendVariable::varList(currentSystem.orderedVars.clone())? {
        let mut var = var.clone();
        if BackendVariable::isOutputVar(var.clone()) && BackendVariable::isRealVar(var.clone()) {
            newCref = ComponentReference::appendStringLastIdent((literal!("_der")).clone(), var.varName.clone())?;
            newCref = ComponentReference::prependStringCref((literal!("$")).clone(), newCref.clone())?;
            daeVarsLst = cons(BackendVariable::makeVar(newCref.clone()), daeVarsLst.clone());
            lhs = Expression::crefExp(newCref.clone())?;
            rhs = IndexReduction::makeder(BackendVariable::varExp(var.clone())?)?;
            newEqnlst = cons(BackendEquation::generateEquation(lhs.clone(), rhs.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone())?, newEqnlst.clone());
        }
    }
    currentSystem = BackendVariable::addVarsDAE(daeVarsLst.clone(), currentSystem.clone());
    assign_field!(currentSystem.orderedEqs = BackendEquation::merge(currentSystem.orderedEqs.clone(), BackendEquation::listEquation(newEqnlst.clone())?)?);
    outDAE = BackendDAE::DAE(list![currentSystem.clone()], inDAE.shared.clone())?;
    Ok(outDAE)
}

pub fn introduceOutputAliases(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    let mut systems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut returnSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut newVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut newEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newRemovedEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut newVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut newEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut topLevelOutputs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = HashSet::emptyHashSet();
    systems = dae.eqs.clone();
    for mut system in &*systems.clone() {
        let mut system = system.clone();
        eqs = system.orderedEqs.clone();
        vars = system.orderedVars.clone();
        removedEqs = system.removedEqs.clone();
        newVars = BackendVariable::emptyVarsSized(((intReal(BackendVariable::varsSize(vars.clone())?) * metamodelica::OrderedFloat(1.4_f64)).0 as i32));
        newEqns = metamodelica::nil();
        newRemovedEqs = metamodelica::nil();
        for mut v in &*BackendVariable::varList(vars.clone())? {
            let mut v = v.clone();
            if !(BackendVariable::isVarOnTopLevelAndOutput(v.clone())) {
                newVars = BackendVariable::addVar(v.clone(), newVars.clone())?;
            } else {
                cref = BackendVariable::varCref(v.clone())?;
                topLevelOutputs = BaseHashSet::add(cref.clone(), topLevelOutputs.clone())?;
                newCref = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::outputAliasPrefix)).clone(), cref.clone())?;
                newVar = BackendVariable::copyVarNewName(newCref.clone(), v.clone());
                newVar = BackendVariable::setVarDirection(newVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                newVar = BackendVariable::setVarKind(newVar.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                if BackendVariable::isRealVar(v.clone()) {
                    newVar = BackendVariable::setVarStateSelect(newVar.clone(), openmodelica_frontend_types::DAE::StateSelect::AVOID)?;
                }
                newVars = BackendVariable::addVar(newVar.clone(), newVars.clone())?;
                if BackendVariable::isStateVar(v.clone()) {
                    v = BackendVariable::setVarKind(v.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                }
                v = BackendVariable::removeFixedAttribute(v.clone())?;
                v = BackendVariable::removeStartAttribute(v.clone())?;
                newVars = BackendVariable::addVar(v.clone(), newVars.clone())?;
                newEqn = BackendEquation::generateEquation(Expression::crefToExp(cref.clone())?, Expression::crefToExp(newCref.clone())?, DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone())?;
                newEqns = cons(newEqn.clone(), newEqns.clone());
            }
        }
        traverseBackendDAEExpsEqns(eqs.clone(), (std::sync::Arc::new(introduceOutputAliases_eqs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), topLevelOutputs.clone())?;
        eqs = BackendEquation::addList(newEqns.clone(), eqs.clone())?;
        traverseBackendDAEExpsEqns(removedEqs.clone(), (std::sync::Arc::new(introduceOutputAliases_eqs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), topLevelOutputs.clone())?;
        removedEqs = BackendEquation::addList(newRemovedEqs.clone(), removedEqs.clone())?;
        assign_field!(
            system.orderedVars = newVars.clone(),
            system.orderedEqs = eqs.clone(),
            system.removedEqs = removedEqs.clone()
        );
        returnSysts = cons(system.clone(), returnSysts.clone());
    }
    returnSysts = returnSysts.clone().reverse();
    assign_field!(dae.eqs = returnSysts.clone());
    Ok(dae)
}

fn introduceOutputAliases_eqs(mut inExp: Arc<DAE::Exp>, mut inStates: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outStates: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    (outExp, outStates) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(introduceOutputAliases_eqs2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), inStates.clone())?;
    Ok((outExp, outStates))
}

fn introduceOutputAliases_eqs2(mut inExp: Arc<DAE::Exp>, mut inStates: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outStates: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = inStates.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (BaseHashSet::has(cr.clone(), inStates.clone())?) => {
            let mut newCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e1 = (*e1).clone();
            newCref = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::outputAliasPrefix)).clone(), cr.clone())?;
            assign_variant_field!(e1 => DAE::Exp::CREF; componentRef = newCref.clone());
            e1.clone()
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outStates))
}

fn translateArrayList(mut inElement: i32, mut inIndex: i32, mut inFoldArg: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outFoldArg: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outFoldArg = if (intEq(inElement.clone(), 1)) {cons(inIndex.clone(), inFoldArg.clone())} else {inFoldArg.clone()};
    outFoldArg
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn removeDiscreteAssignments(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStmts = 'mc: {
        let __mc_input = (inStmts.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: e, .. }, tail: rest }, vars) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    cref = Expression::expCref(e.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    let true = (BackendVariable::isVarDiscrete(v.clone())) else { bail!("pattern mismatch") };
                    xs = removeDiscreteAssignments(rest.clone(), vars.clone())?;
                    Ok(xs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: e, .. }, tail: rest }, vars) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    cref = Expression::expCref(e.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    let true = (BackendVariable::isVarDiscrete(v.clone())) else { bail!("pattern mismatch") };
                    xs = removeDiscreteAssignments(rest.clone(), vars.clone())?;
                    Ok(xs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { source, else_: algElse, statementLst: stmts, exp: e }, tail: rest }, vars) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut algElse = (*algElse).clone();
                    let mut stmts = (*stmts).clone();
                    stmts = removeDiscreteAssignments(stmts.clone(), vars.clone())?;
                    algElse = removediscreteAssingmentsElse(algElse.clone(), vars.clone())?;
                    xs = removeDiscreteAssignments(rest.clone(), vars.clone())?;
                    Ok(cons(Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: stmts.clone(), else_: algElse.clone(), source: source.clone() }), xs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_FOR { source, statementLst: stmts, range: e, iter: id1, iterIsArray: b1, type_: tp }, tail: rest }, vars) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts = (*stmts).clone();
                    stmts = removeDiscreteAssignments(stmts.clone(), vars.clone())?;
                    xs = removeDiscreteAssignments(rest.clone(), vars.clone())?;
                    Ok(cons(Arc::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: (id1.clone()).clone(), range: e.clone(), statementLst: stmts.clone(), source: source.clone() }), xs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHILE { source, statementLst: stmts, exp: e }, tail: rest }, vars) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts = (*stmts).clone();
                    stmts = removeDiscreteAssignments(stmts.clone(), vars.clone())?;
                    xs = removeDiscreteAssignments(rest.clone(), vars.clone())?;
                    Ok(cons(Arc::new(DAE::Statement::STMT_WHILE { exp: e.clone(), statementLst: stmts.clone(), source: source.clone() }), xs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: None, statementLst: stmts, initialCall, conditions, exp: e }, tail: rest }, vars) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts = (*stmts).clone();
                    stmts = removeDiscreteAssignments(stmts.clone(), vars.clone())?;
                    xs = removeDiscreteAssignments(rest.clone(), vars.clone())?;
                    Ok(cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts.clone(), elseWhen: None, source: source.clone() }), xs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: Some(ew), statementLst: stmts, initialCall, conditions, exp: e }, tail: rest }, vars) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut ew = (*ew).clone();
                    let mut stmts = (*stmts).clone();
                    stmts = removeDiscreteAssignments(stmts.clone(), vars.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(removeDiscreteAssignments(list![ew.clone()], vars.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ew = __pa0.clone();
                    xs = removeDiscreteAssignments(rest.clone(), vars.clone())?;
                    Ok(cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts.clone(), elseWhen: Some(ew.clone()), source: source.clone() }), xs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: stmt, tail: rest }, vars) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    xs = removeDiscreteAssignments(rest.clone(), vars.clone())?;
                    Ok(cons(stmt.clone(), xs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStmts)
}

fn removediscreteAssingmentsElse(mut inElse: Arc<DAE::Else>, mut inVars: BackendDAE::Variables) -> Result<Arc<DAE::Else>> {
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    outElse = (::match_deref::match_deref! { match &((inElse.clone(), inVars.clone())) {
        (Deref @ DAE::Else::NOELSE, _) => {
            Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE)
        },
        (Deref @ DAE::Else::ELSEIF { exp: e, statementLst: st, else_: el }, vars) => {
            let mut st = (*st).clone();
            let mut el = (*el).clone();
            el = removediscreteAssingmentsElse(el.clone(), vars.clone())?;
            st = removeDiscreteAssignments(st.clone(), vars.clone())?;
            Arc::new(DAE::Else::ELSEIF { exp: e.clone(), statementLst: st.clone(), else_: el.clone() })
        },
        (Deref @ DAE::Else::ELSE { statementLst: st }, vars) => {
            let mut st = (*st).clone();
            st = removeDiscreteAssignments(st.clone(), vars.clone())?;
            Arc::new(DAE::Else::ELSE { statementLst: st.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElse)
}

pub fn collateAlgorithm(mut inAlg: Arc<DAE::Algorithm>, mut infuncs: Option<Arc<AvlTreePathFunction::Tree>>) -> Result<Arc<DAE::Algorithm>> {
    let mut outAlg: Arc<DAE::Algorithm> = Arc::new(<DAE::Algorithm as ::std::default::Default>::default());
    outAlg = 'mc: {
        let __mc_input = inAlg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Algorithm { statementLst } => {
                    let mut statementLst = (*statementLst).clone();
                    (statementLst, _) = DAEUtil::traverseDAEStmts(statementLst.clone(), (std::sync::Arc::new(fnptr!(collateArrExpStmt, Arc<DAE::Exp>, Arc<DAE::Statement>, Option<Arc<AvlTreePathFunction::Tree>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Statement>, Option<Arc<AvlTreePathFunction::Tree>>) -> Result<(Arc<DAE::Exp>, Option<Arc<AvlTreePathFunction::Tree>>)> + 'static>), infuncs.clone())?;
                    Ok(Arc::new(DAE::Algorithm { statementLst: statementLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inAlg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAlg)
}

fn collateArrExpStmt(mut inExp: Arc<DAE::Exp>, mut inStmt: Arc<DAE::Statement>, mut funcs: Option<Arc<AvlTreePathFunction::Tree>>) -> (Arc<DAE::Exp>, Option<Arc<AvlTreePathFunction::Tree>>) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut oarg: Option<Arc<AvlTreePathFunction::Tree>> = funcs.clone();
    if '__try0: {
        (outExp, _) = unwrap_break_err!(Expression::traverseExpBottomUp(outExp.clone(), (std::sync::Arc::new(traversingcollateArrExpStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Statement>, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Statement>, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), (inStmt.clone(), funcs.clone())), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    (outExp, oarg)
}

fn traversingcollateArrExpStmt(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::Statement>, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Statement>, Option<Arc<AvlTreePathFunction::Tree>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<DAE::Statement>, Option<Arc<AvlTreePathFunction::Tree>>);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: _ }, tail: _ }, .. }, (Deref @ DAE::Statement::STMT_WHEN { .. }, _)) => {
                    Ok((e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }, tail: _ }, tail: _ }, .. }, (Deref @ DAE::Statement::STMT_WHEN { .. }, _)) => {
                    Ok((e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: _ }, .. }, (Deref @ DAE::Statement::STMT_WHEN { .. }, _)) => {
                    Ok((e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }, tail: _ }, .. }, (Deref @ DAE::Statement::STMT_WHEN { .. }, _)) => {
                    Ok((e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::CREF { .. }, tail: _ }, tail: _ }, .. }, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    e1_1 = Expression::expStripLastSubs(e1.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1_1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    let true = (ExpressionBasics::expEqual(e.clone(), e1_2.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1_1.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }, tail: _ }, tail: _ }, .. }, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    e1_1 = Expression::expStripLastSubs(e1.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1_1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    let true = (ExpressionBasics::expEqual(e.clone(), e1_2.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1_1.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::CREF { .. }, tail: _ }, .. }, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    e1_1 = Expression::expStripLastSubs(e1.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1_1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    let true = (ExpressionBasics::expEqual(e.clone(), e1_2.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1_1.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }, tail: _ }, .. }, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    e1_1 = Expression::expStripLastSubs(e1.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1_1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    let true = (ExpressionBasics::expEqual(e.clone(), e1_2.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1_1.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

pub fn collateArrExpList(mut iexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut optfunc: Option<Arc<AvlTreePathFunction::Tree>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outexpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outexpl = (::match_deref::match_deref! { match &(iexpl.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: e, tail: expl } => {
            let mut e1: Arc<DAE::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (e1, _) = collateArrExp(e.clone(), optfunc.clone())?;
            expl1 = collateArrExpList(expl.clone(), optfunc.clone())?;
            cons(e1.clone(), expl1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outexpl)
}

pub fn collateArrExp(mut inExp: Arc<DAE::Exp>, mut inFuncs: Option<Arc<AvlTreePathFunction::Tree>>) -> Result<(Arc<DAE::Exp>, Option<Arc<AvlTreePathFunction::Tree>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outFuncs: Option<Arc<AvlTreePathFunction::Tree>> = None;
    (outExp, outFuncs) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(traversingcollateArrExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<AvlTreePathFunction::Tree>>) -> Result<(Arc<DAE::Exp>, Option<Arc<AvlTreePathFunction::Tree>>)> + 'static>), inFuncs.clone())?;
    Ok((outExp, outFuncs))
}

fn traversingcollateArrExp(mut inExp: Arc<DAE::Exp>, mut inFuncs: Option<Arc<AvlTreePathFunction::Tree>>) -> Result<(Arc<DAE::Exp>, Option<Arc<AvlTreePathFunction::Tree>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut funcs: Option<Arc<AvlTreePathFunction::Tree>> = None;
    (outExp, funcs) = 'mc: {
        let __mc_input = (inExp.clone(), inFuncs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::CREF { .. }, tail: _ }, tail: _ }, .. }, funcs) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    e1_1 = Expression::expStripLastSubs(e1.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1_1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    let true = (ExpressionBasics::expEqual(e.clone(), e1_2.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1_1.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }, tail: _ }, tail: _ }, .. }, funcs) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    e1_1 = Expression::expStripLastSubs(e1.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1_1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    let true = (ExpressionBasics::expEqual(e.clone(), e1_2.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1_1.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::CREF { .. }, tail: _ }, .. }, funcs) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    e1_1 = Expression::expStripLastSubs(e1.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1_1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    let true = (ExpressionBasics::expEqual(e.clone(), e1_2.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1_1.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }, tail: _ }, .. }, funcs) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    e1_1 = Expression::expStripLastSubs(e1.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1_1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    let true = (ExpressionBasics::expEqual(e.clone(), e1_2.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1_1.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inFuncs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, funcs))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getEquationBlock(mut inInteger: i32, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>) -> Result<Arc<BackendDAE::StrongComponent>> {
    let mut outComp: Arc<BackendDAE::StrongComponent>;
    outComp = 'mc: {
        let __mc_input = (inInteger.clone(), inComps.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, Deref @ metamodelica::List::Cons { head: comp, tail: _ }) => {
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (elst, _) = BackendDAETransform::getEquationAndSolvedVarIndxes(comp.clone())?;
                    let true = (listMember(i.clone(), elst.clone())) else { bail!("pattern mismatch") };
                    Ok(comp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, Deref @ metamodelica::List::Cons { head: _, tail: comps }) => {
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    comp = getEquationBlock(i.clone(), comps.clone())?;
                    Ok(comp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComp)
}

/* *****************************************************************
 stuff to calculate adjacency matrix

 wbraun: It should be renames to Adjacency matrix, because
    adjacency matrix descibes the relation between knots and edges.
    In the sense it is used here is the relation between knots and
    knots of a bigraph.
******************************************************************/
pub fn adjacencyMatrix(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inEqSystem.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        eqns = __pa1.clone();
        vars = __pa2.clone();
        (outAdjacencyMatrix, outAdjacencyMatrixT) = unwrap_break_err!(adjacencyMatrixDispatch(vars.clone(), eqns.clone(), inIndexType.clone(), functionTree.clone(), isInitial.clone()), '__try0);
        Ok::<_, anyhow::Error>((eqns.clone(), outAdjacencyMatrix.clone(), outAdjacencyMatrixT.clone(), vars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            eqns = __try0_o0;
            outAdjacencyMatrix = __try0_o1;
            outAdjacencyMatrixT = __try0_o2;
            vars = __try0_o3;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAEUtil.adjacencyMatrix failed.")).clone()])?;
            bail!("fail");
        }
    }
    Ok((outAdjacencyMatrix, outAdjacencyMatrixT))
}

pub fn adjacencyMatrixMasked(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inIndexType: BackendDAE::IndexType, mut inMask: metamodelica::Array<bool>, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inEqSystem.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        eqns = __pa1.clone();
        vars = __pa2.clone();
        (outAdjacencyMatrix, outAdjacencyMatrixT) = unwrap_break_err!(adjacencyMatrixDispatchMasked(vars.clone(), eqns.clone(), inIndexType.clone(), inMask.clone(), functionTree.clone(), isInitial.clone()), '__try0);
        Ok::<_, anyhow::Error>((eqns.clone(), outAdjacencyMatrix.clone(), outAdjacencyMatrixT.clone(), vars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            eqns = __try0_o0;
            outAdjacencyMatrix = __try0_o1;
            outAdjacencyMatrixT = __try0_o2;
            vars = __try0_o3;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAEUtil.adjacencyMatrix failed.")).clone()])?;
            bail!("fail");
        }
    }
    Ok((outAdjacencyMatrix, outAdjacencyMatrixT))
}

pub fn adjacencyMatrixScalar(mut syst: Arc<BackendDAE::EqSystem>, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMapIncRowEqn: metamodelica::Array<i32>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        eqns = __pa1.clone();
        vars = __pa2.clone();
        ExpandableArray::compress(eqns.clone());
        (outAdjacencyMatrix, outAdjacencyMatrixT, outMapEqnIncRow, outMapIncRowEqn) = unwrap_break_err!(adjacencyMatrixDispatchScalar(vars.clone(), eqns.clone(), inIndexType.clone(), functionTree.clone(), isInitial.clone()), '__try0);
        Ok::<_, anyhow::Error>((eqns.clone(), outAdjacencyMatrix.clone(), outAdjacencyMatrixT.clone(), outMapEqnIncRow.clone(), outMapIncRowEqn.clone(), vars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            eqns = __try0_o0;
            outAdjacencyMatrix = __try0_o1;
            outAdjacencyMatrixT = __try0_o2;
            outMapEqnIncRow = __try0_o3;
            outMapIncRowEqn = __try0_o4;
            vars = __try0_o5;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAEUtil.adjacencyMatrixScalar failed.")).clone()])?;
            bail!("fail");
        }
    }
    Ok((outAdjacencyMatrix, outAdjacencyMatrixT, outMapEqnIncRow, outMapIncRowEqn))
}

fn applyIndexType(mut inLst: Arc<AvlSetInt::Tree>, mut inIndexType: BackendDAE::IndexType) -> Result<Arc<AvlSetInt::Tree>> {
    let mut outLst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    outLst = (match inIndexType.clone() {
        BackendDAE::IndexType::ABSOLUTE if (!(AvlSetInt::isEmpty(inLst.clone())) && AvlSetInt::smallestKey(inLst.clone())? < 0) => {
            outLst = Arc::new(crate::AvlSetInt::Tree::EMPTY);
            for mut key in &*AvlSetInt::listKeys(inLst.clone(), metamodelica::nil()) {
                let mut key = key.clone();
                outLst = AvlSetInt::add(outLst.clone(), intAbs(key.clone()))?;
            }
            outLst.clone()
        },
        _ => inLst.clone(),
    });
    Ok(outLst)
}

pub fn getIndexType(mut syst: Arc<BackendDAE::EqSystem>) -> Result<(BackendDAE::IndexType, bool, bool)> {
    let mut indexType: BackendDAE::IndexType = BackendDAE::IndexType::ABSOLUTE;
    let mut scalar: bool = false;
    let mut processed: bool = false;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.mapping.clone()) {
        Some((_, _, __pa0, __pa1, __pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    indexType = __pa0.clone();
    scalar = __pa1.clone();
    processed = __pa2.clone();
    Ok((indexType, scalar, processed))
}

pub fn hasIndexTypeSolvableAndUnprocessedScalar(mut syst: Arc<BackendDAE::EqSystem>) -> bool {
    let mut b: bool = false;
    let mut indexType: BackendDAE::IndexType = BackendDAE::IndexType::ABSOLUTE;
    let mut scalar: bool = false;
    let mut processed: bool = false;
    if '__try0: {
        (indexType, scalar, processed) = unwrap_break_err!(getIndexType(syst.clone()), '__try0);
        b = (match (indexType.clone(), scalar.clone(), processed.clone()) {
        (BackendDAE::IndexType::SOLVABLE, true, false) => true,
        _ => false,
    });
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    b
}

pub fn hasScalarAdjacencyMatrix(mut syst: Arc<BackendDAE::EqSystem>) -> Result<bool> {
    let mut b: bool = false;
    (_, b, _) = getIndexType(syst.clone())?;
    Ok(b)
}

pub fn setAnalyticalToStructuralProcessed(mut syst: Arc<BackendDAE::EqSystem>, mut processed: bool) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut mapArrayToScalar: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapScalarToArray: metamodelica::Array<i32>;
    let mut indexType: BackendDAE::IndexType = BackendDAE::IndexType::ABSOLUTE;
    let mut scalar: bool = false;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(syst.mapping.clone()) {
        Some((__pa0, __pa1, __pa2, __pa3, _)) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    mapArrayToScalar = __pa0.clone();
    mapScalarToArray = __pa1.clone();
    indexType = __pa2.clone();
    scalar = __pa3.clone();
    assign_field!(syst.mapping = Some((mapArrayToScalar.clone(), mapScalarToArray.clone(), indexType.clone(), scalar.clone(), processed.clone())));
    Ok(syst)
}

pub fn adjacencyMatrixDispatch(mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outAdjacencyArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut num_eqs: i32 = 0;
    let mut num_vars: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rowTree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    num_eqs = BackendEquation::getNumberOfEquations(inEqns.clone());
    num_vars = BackendVariable::varsSize(inVars.clone())?;
    outAdjacencyArray = arrayCreate(num_eqs.clone(), metamodelica::nil());
    outAdjacencyArrayT = arrayCreate(num_vars.clone(), metamodelica::nil());
    for mut idx in 1..=num_eqs.clone() {
        eq = BackendEquation::get(inEqns.clone(), idx.clone());
        (rowTree, _) = adjacencyRow(eq.clone(), inVars.clone(), inIndexType.clone(), functionTree.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone())?;
        row = AvlSetInt::listKeys(rowTree.clone(), metamodelica::nil());
        {let _arr = outAdjacencyArray.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = row.clone(); _arr};
        outAdjacencyArrayT = filladjacencyMatrixT(row.clone(), list![idx.clone()], outAdjacencyArrayT.clone())?;
    }
    Ok((outAdjacencyArray, outAdjacencyArrayT))
}

pub fn adjacencyMatrixDispatchMasked(mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inIndexType: BackendDAE::IndexType, mut inMask: metamodelica::Array<bool>, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outAdjacencyArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut num_eqs: i32 = 0;
    let mut num_vars: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rowTree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    num_eqs = BackendEquation::getNumberOfEquations(inEqns.clone());
    num_vars = BackendVariable::varsSize(inVars.clone())?;
    outAdjacencyArray = arrayCreate(num_eqs.clone(), metamodelica::nil());
    outAdjacencyArrayT = arrayCreate(num_vars.clone(), metamodelica::nil());
    for mut idx in 1..=num_eqs.clone() {
        if inMask.borrow()[(idx.clone()-1) as usize].clone() {
            eq = BackendEquation::get(inEqns.clone(), idx.clone());
            (rowTree, _) = adjacencyRow(eq.clone(), inVars.clone(), inIndexType.clone(), functionTree.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone())?;
            row = AvlSetInt::listKeys(rowTree.clone(), metamodelica::nil());
            {let _arr = outAdjacencyArray.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = row.clone(); _arr};
            outAdjacencyArrayT = filladjacencyMatrixT(row.clone(), list![idx.clone()], outAdjacencyArrayT.clone())?;
        }
    }
    Ok((outAdjacencyArray, outAdjacencyArrayT))
}

fn adjacencyMatrixDispatchScalar(mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut outAdjacencyArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut omapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut omapIncRowEqn: metamodelica::Array<i32>;
    let mut num_eqs: i32 = 0;
    let mut num_vars: i32 = 0;
    let mut size: i32 = 0;
    let mut num_rows: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut rowTree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut row_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut imap: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut iarr: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    num_eqs = BackendEquation::getNumberOfEquations(inEqns.clone());
    num_vars = BackendVariable::varsSize(inVars.clone())?;
    outAdjacencyArrayT = arrayCreate(num_vars.clone(), metamodelica::nil());
    omapEqnIncRow = arrayCreate(num_eqs.clone(), metamodelica::nil());
    for mut idx in 1..=num_eqs.clone() {
        eq = BackendEquation::get(inEqns.clone(), idx.clone());
        (rowTree, size) = adjacencyRow(eq.clone(), inVars.clone(), inIndexType.clone(), functionTree.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone())?;
        row = AvlSetInt::listKeys(rowTree.clone(), metamodelica::nil());
        row_indices = List::intRange2(num_rows.clone() + 1, num_rows.clone() + size.clone());
        num_rows = num_rows.clone() + size.clone();
        {let _arr = omapEqnIncRow.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = row_indices.clone(); _arr};
        imap = List::consN(size.clone(), idx.clone(), imap.clone());
        iarr = List::consN(size.clone(), row.clone(), iarr.clone());
        outAdjacencyArrayT = filladjacencyMatrixT(row.clone(), row_indices.clone(), outAdjacencyArrayT.clone())?;
    }
    outAdjacencyArray = List::listArrayReverse(iarr.clone())?;
    omapIncRowEqn = List::listArrayReverse(imap.clone())?;
    Ok((outAdjacencyArray, outAdjacencyArrayT, omapEqnIncRow, omapIncRowEqn))
}

fn filladjacencyMatrixT(mut eqns: Arc<metamodelica::List<i32>>, mut eqnsindxs: Arc<metamodelica::List<i32>>, mut inAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<i32>>> = inAdjacencyArrayT.clone();
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ei: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnsindxsNeg: Arc<metamodelica::List<i32>> = metamodelica::nil();
    eqnsindxsNeg = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (eqnsindxs.clone()).into_iter().cloned() {
            let __x = intNeg(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut v in &*eqns.clone() {
        let mut v = v.clone();
        if v.clone() < 0 {
            v = intAbs(v.clone());
            ei = eqnsindxsNeg.clone();
        } else {
            ei = eqnsindxs.clone();
        }
        row = listAppend(ei.clone(), inAdjacencyArrayT.clone().borrow()[(v.clone()-1) as usize].clone());
        {let _arr = outAdjacencyArrayT.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = row.clone(); _arr};
    }
    Ok(outAdjacencyArrayT)
}

pub fn adjacencyRow(mut inEquation: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut iRow: Arc<AvlSetInt::Tree>, mut isInitial: bool) -> Result<(Arc<AvlSetInt::Tree>, i32)> {
    let mut outIntegerLst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    let mut rowSize: i32 = 0;
    let mut whenIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inlinedEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    whenIntegerLst = 'mc: {
        let __mc_input = inIndexType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::IndexType::BASECLOCK_IDX = __mc_input.clone() else { bail!("nomatch") };
            let mut kind: BackendDAE::EquationKind = BackendDAE::EquationKind::AUX_EQUATION;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut i: i32 = 0;
            let mut varIxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let BackendDAE::EQUATION_ATTRIBUTES { kind: __pa0, .. } = (BackendEquation::getEquationAttributes(inEquation.clone())?) else { bail!("pattern mismatch") };
            kind = __pa0.clone();
            let BackendDAE::CLOCKED_EQUATION { clk: __pa1 } = (kind.clone()) else { bail!("pattern mismatch") };
            i = __pa1.clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            (_, varIxs) = BackendVariable::getVar(cr.clone(), vars.clone())?;
            Ok(varIxs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::IndexType::SUBCLOCK_IDX = __mc_input.clone() else { bail!("nomatch") };
            let mut kind: BackendDAE::EquationKind = BackendDAE::EquationKind::AUX_EQUATION;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut i: i32 = 0;
            let mut varIxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let BackendDAE::EQUATION_ATTRIBUTES { kind: __pa0, .. } = (BackendEquation::getEquationAttributes(inEquation.clone())?) else { bail!("pattern mismatch") };
            kind = __pa0.clone();
            let BackendDAE::CLOCKED_EQUATION { clk: __pa1 } = (kind.clone()) else { bail!("pattern mismatch") };
            i = __pa1.clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            (_, varIxs) = BackendVariable::getVar(cr.clone(), vars.clone())?;
            Ok(varIxs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::nil())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    (inlinedEquation, _) = BackendInline::inlineEq(inEquation.clone(), (functionTree.clone(), list![openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE]))?;
    (outIntegerLst, rowSize) = 'mc: {
        let __mc_input = inlinedEquation.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. } => {
                    let mut lst1: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone(), iRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    res = adjacencyRowExp(e2.clone(), vars.clone(), lst1.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    Ok((res.clone(), 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, size, .. } => {
                    let mut lst1: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone(), iRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    res = adjacencyRowExp(e2.clone(), vars.clone(), lst1.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    Ok((res.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, dimSize: dimsize, .. } => {
                    let mut lst1: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut size: i32 = 0;
                    size = if (Flags::isSet(Flags::NF_SCALARIZE.clone())?) {List::reduce(dimsize.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>))?} else {1};
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone(), iRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    res = adjacencyRowExp(e2.clone(), vars.clone(), lst1.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    Ok((res.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::FOR_EQUATION { iter: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: r#str, .. }, .. }, body: eqn, .. } => {
                    let mut eqn = (*eqn).clone();
                    (eqn, _) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(fnptr!(stripIterSub, Arc<DAE::Exp>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr) -> Result<(Arc<DAE::Exp>, bool, ArcStr)> + 'static>); move |__pe_a0, __pe_a2| Expression::traverseExpTopDown(__pe_a0, __pe_b1.clone(), __pe_a2) }), (r#str.clone()).clone())?;
                    Ok(adjacencyRow(eqn.clone(), vars.clone(), inIndexType.clone(), functionTree.clone(), iRow.clone(), isInitial.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, componentRef: cr, .. } => {
                    let mut lst1: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut expCref: Arc<DAE::Exp>;
                    expCref = Expression::crefExp(cr.clone())?;
                    lst1 = adjacencyRowExp(expCref.clone(), vars.clone(), iRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    res = adjacencyRowExp(e.clone(), vars.clone(), lst1.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    Ok((res.clone(), 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    res = adjacencyRowExp(e.clone(), vars.clone(), iRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
                    Ok((res.clone(), 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: we, size, .. } => {
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    res = adjacencyRowWhen(we.clone(), vars.clone(), inIndexType.clone(), functionTree.clone(), iRow.clone(), isInitial.clone())?;
                    Ok((res.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst }, size, .. } => {
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    res = traverseStmts(statementLst.clone(), Arc::new({ let __pe_b2 = vars.clone(); let __pe_b3 = functionTree.clone(); let __pe_b4 = inIndexType.clone(); let __pe_b5 = isInitial.clone(); move |__pe_a0, __pe_a1| adjacencyRowAlgorithm(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }), iRow.clone())?;
                    crefLst = CheckModel::algorithmStatementListOutputs(statementLst.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
                    for mut cr in &*crefLst.clone() {
                        let mut cr = cr.clone();
                        if '__try0: {
                            (varslst, p) = unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars.clone()), '__try0);
                            res = unwrap_break_err!(adjacencyRowExp1DiscreteOrArray(varslst.clone(), p.clone(), res.clone()), '__try0);
                            Ok::<(), anyhow::Error>(())
                        }.is_err() {
                        }
                        if '__try1: {
                            (varslst, p) = unwrap_break_err!(BackendVariable::getVar(ComponentReference::crefPrefixStart(cr.clone()), vars.clone()), '__try1);
                            res = unwrap_break_err!(adjacencyRowExp1DiscreteOrArray(varslst.clone(), p.clone(), res.clone()), '__try1);
                            Ok::<(), anyhow::Error>(())
                        }.is_err() {
                        }
                    }
                    Ok((res.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: eqns, eqnstrue: eqnslst, conditions: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut size: i32 = 0;
                    let mut eqns = (*eqns).clone();
                    if !(isInitial.clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(eqnslst.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        eqns = __pa0.clone();
                    }
                    (res, size) = adjacencyRowLst(eqns.clone(), vars.clone(), inIndexType.clone(), functionTree.clone(), iRow.clone(), isInitial.clone())?;
                    Ok((res.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: eqns, eqnstrue: eqnslst, conditions: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { .. }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut size: i32 = 0;
                    let mut eqns = (*eqns).clone();
                    if isInitial.clone() {
                        let __pa0 = ::match_deref::match_deref! { match &(eqnslst.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        eqns = __pa0.clone();
                    }
                    (res, size) = adjacencyRowLst(eqns.clone(), vars.clone(), inIndexType.clone(), functionTree.clone(), iRow.clone(), isInitial.clone())?;
                    Ok((res.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: eqns, eqnstrue: eqnslst, conditions: expl, .. } => {
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut size: i32 = 0;
                    res = adjacencyRow1(expl.clone(), (std::sync::Arc::new(adjacencyRowExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables, Arc<AvlSetInt::Tree>, Option<Arc<AvlTreePathFunction::Tree>>, BackendDAE::IndexType, bool) -> Result<Arc<AvlSetInt::Tree>> + 'static>), vars.clone(), iRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone());
                    (res, _) = adjacencyRowLstLst(eqnslst.clone(), vars.clone(), inIndexType.clone(), functionTree.clone(), res.clone(), isInitial.clone())?;
                    (res, size) = adjacencyRowLst(eqns.clone(), vars.clone(), inIndexType.clone(), functionTree.clone(), res.clone(), isInitial.clone())?;
                    Ok((res.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqnstr: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    eqnstr = (BackendDump::equationString(inEquation.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAEUtil.adjacencyRow failed for equation: ")); __mm_s.push_str(&*eqnstr.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    outIntegerLst = AvlSetInt::addList(outIntegerLst.clone(), whenIntegerLst.clone())?;
    Ok((outIntegerLst, rowSize))
}

fn stripIterSub(mut inExp: Arc<DAE::Exp>, mut inIter: ArcStr) -> (Arc<DAE::Exp>, bool, ArcStr) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outIter: ArcStr = inIter.clone();
    (outExp, cont) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { ty, componentRef: cr } => {
            (Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::crefStripIterSub(cr.clone(), (inIter.clone()).clone()), ty: ty.clone() }), false)
        },
        _ => {
            (inExp.clone(), true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outIter)
}

fn adjacencyRowLst(mut inEquation: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVariables: BackendDAE::Variables, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut inIntegerLst: Arc<AvlSetInt::Tree>, mut isInitial: bool) -> Result<(Arc<AvlSetInt::Tree>, i32)> {
    let mut outIntegerLst: Arc<AvlSetInt::Tree> = inIntegerLst.clone();
    let mut rowSize: i32 = 0;
    let mut size: i32 = 0;
    for mut eq in &*inEquation.clone() {
        let mut eq = eq.clone();
        (outIntegerLst, size) = adjacencyRow(eq.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outIntegerLst.clone(), isInitial.clone())?;
        rowSize = rowSize.clone() + size.clone();
    }
    Ok((outIntegerLst, rowSize))
}

fn adjacencyRowLstLst(mut inEquation: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inVariables: BackendDAE::Variables, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut inIntegerLst: Arc<AvlSetInt::Tree>, mut isInitial: bool) -> Result<(Arc<AvlSetInt::Tree>, i32)> {
    let mut outIntegerLst: Arc<AvlSetInt::Tree> = inIntegerLst.clone();
    let mut rowSize: i32 = 0;
    let mut size: i32 = 0;
    for mut eql in &*inEquation.clone() {
        let mut eql = eql.clone();
        (outIntegerLst, size) = adjacencyRowLst(eql.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outIntegerLst.clone(), isInitial.clone())?;
        rowSize = rowSize.clone() + size.clone();
    }
    Ok((outIntegerLst, rowSize))
}

fn adjacencyRowWhen(mut inEquation: Arc<BackendDAE::WhenEquation>, mut inVariables: BackendDAE::Variables, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut inRow: Arc<AvlSetInt::Tree>, mut isInitial: bool) -> Result<Arc<AvlSetInt::Tree>> {
    let mut outRow: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    outRow = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: oelsewe, whenStmtLst, condition: cond } => {
            let mut elsewe: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
            outRow = adjacencyRowExp(cond.clone(), inVariables.clone(), inRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowWhenOps(whenStmtLst.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outRow.clone(), isInitial.clone())?;
            if isSome(oelsewe.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oelsewe.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                elsewe = __pa0.clone();
                outRow = adjacencyRowWhen(elsewe.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outRow.clone(), isInitial.clone())?;
            }
            outRow.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRow)
}

fn adjacencyRowWhenOps(mut inWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut inVariables: BackendDAE::Variables, mut inIndexType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut inRow: Arc<AvlSetInt::Tree>, mut isInitial: bool) -> Result<Arc<AvlSetInt::Tree>> {
    let mut outRow: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    outRow = (::match_deref::match_deref! { match &(inWhenOps.clone()) {
        Deref @ metamodelica::List::Nil => {
            inRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e2, left: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, .. }, tail: rest } => {
            outRow = adjacencyRowExp(e2.clone(), inVariables.clone(), inRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowWhenOps(rest.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outRow.clone(), isInitial.clone())?;
            outRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e2, left: e1, .. }, tail: rest } => {
            outRow = adjacencyRowExp(e1.clone(), inVariables.clone(), inRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowExp(e2.clone(), inVariables.clone(), outRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowWhenOps(rest.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outRow.clone(), isInitial.clone())?;
            outRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { value: e2, stateVar: cr, .. }, tail: rest } => {
            let mut e1: Arc<DAE::Exp>;
            e1 = Expression::crefExp(cr.clone())?;
            outRow = adjacencyRowExp(e1.clone(), inVariables.clone(), inRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowExp(e2.clone(), inVariables.clone(), outRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowWhenOps(rest.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outRow.clone(), isInitial.clone())?;
            outRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSERT { message: e2, condition: e1, .. }, tail: rest } => {
            outRow = adjacencyRowExp(e1.clone(), inVariables.clone(), inRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowExp(e2.clone(), inVariables.clone(), outRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowWhenOps(rest.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outRow.clone(), isInitial.clone())?;
            outRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::TERMINATE { message: e1, .. }, tail: rest } => {
            outRow = adjacencyRowExp(e1.clone(), inVariables.clone(), inRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowWhenOps(rest.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outRow.clone(), isInitial.clone())?;
            outRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: e1, .. }, tail: rest } => {
            outRow = adjacencyRowExp(e1.clone(), inVariables.clone(), inRow.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
            outRow = adjacencyRowWhenOps(rest.clone(), inVariables.clone(), inIndexType.clone(), functionTree.clone(), outRow.clone(), isInitial.clone())?;
            outRow.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRow)
}

fn adjacencyRowAlgorithm(mut exp: Arc<DAE::Exp>, mut row: Arc<AvlSetInt::Tree>, mut inVariables: BackendDAE::Variables, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut inIndexType: BackendDAE::IndexType, mut isInitial: bool) -> Result<Arc<AvlSetInt::Tree>> {
    let mut row: Arc<AvlSetInt::Tree> = row;
    row = adjacencyRowExp(exp.clone(), inVariables.clone(), row.clone(), functionTree.clone(), inIndexType.clone(), isInitial.clone())?;
    Ok(row)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn adjacencyRow1<Type_a: Clone + 'static, Type_b: Clone + 'static, Type_c: Clone + 'static, Type_d: Clone + 'static, Type_e: Clone + 'static, Type_f: Clone + 'static>(mut inList: Arc<metamodelica::List<Type_a>>, mut inFunc: Arc<dyn ::std::ops::Fn(Type_a, Type_b, Type_c, Type_d, Type_e, Type_f) -> Result<Type_c> + 'static>, mut inArg: Type_b, mut inArg1: Type_c, mut inArg2: Type_d, mut inArg3: Type_e, mut inArg4: Type_f) -> Type_c {
    pub type FuncType<Type_a: Clone + 'static, Type_b: Clone + 'static, Type_c: Clone + 'static, Type_d: Clone + 'static, Type_e: Clone + 'static, Type_f: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_a, Type_b, Type_c, Type_d, Type_e, Type_f) -> Result<Type_c> + 'static>;

    let mut outArg1: Type_c;
    outArg1 = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Nil => {
            inArg1.clone()
        },
        Deref @ metamodelica::List::Cons { head: e1, tail: rest_e1 } => {
            let mut res: Type_c;
            let mut res1: Type_c;
            res = inFunc(e1.clone(), inArg.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone(), inArg4.clone()).unwrap();
            res1 = adjacencyRow1(rest_e1.clone(), inFunc.clone(), inArg.clone(), res.clone(), inArg2.clone(), inArg3.clone(), inArg4.clone());
            res1.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArg1
}

pub fn adjacencyRowExp(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables, mut inIntegerLst: Arc<AvlSetInt::Tree>, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut inIndexType: BackendDAE::IndexType, mut isInitial: bool) -> Result<Arc<AvlSetInt::Tree>> {
    let mut outIntegerLst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    outIntegerLst = (match inIndexType.clone() {
        BackendDAE::IndexType::SPARSE => {
            let mut vallst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingadjacencyRowExpFinderwithInput) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (inVariables.clone(), inIntegerLst.clone(), isInitial.clone()))?;
            vallst = __pa0.clone();
            vallst.clone()
        },
        BackendDAE::IndexType::SOLVABLE => {
            let mut vallst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let (_, (_, __pa0, _, _, _)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), (inVariables.clone(), inIntegerLst.clone(), Arc::new(openmodelica_ast_collections::AvlSetPath::Tree::EMPTY), isInitial.clone(), functionTree.clone()))?;
            vallst = __pa0.clone();
            vallst.clone()
        },
        BackendDAE::IndexType::BASECLOCK_IDX => {
            let mut vallst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpFinderBaseClock) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (inVariables.clone(), inIntegerLst.clone(), isInitial.clone()))?;
            vallst = __pa0.clone();
            vallst.clone()
        },
        BackendDAE::IndexType::SUBCLOCK_IDX => {
            let mut vallst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpFinderSubClock) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (inVariables.clone(), inIntegerLst.clone(), isInitial.clone()))?;
            vallst = __pa0.clone();
            vallst.clone()
        },
        _ => {
            let mut vallst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingadjacencyRowExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (inVariables.clone(), inIntegerLst.clone(), isInitial.clone()))?;
            vallst = __pa0.clone();
            vallst = applyIndexType(vallst.clone(), inIndexType.clone())?;
            vallst.clone()
        },
    });
    Ok(outIntegerLst)
}

pub fn traversingadjacencyRowExpSolvableFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { .. }, tpl) => {
                    Ok((inExp.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { .. }, tpl) => {
                    Ok((inExp.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, tpl) => {
                    Ok(traversingadjacencyRowIfExpSolvableFinder(inExp.clone(), tpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { .. }, tpl) => {
                    Ok((inExp.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { sub: subs, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, (vars, pa, visitedPaths, isInitial, ofunctionTree)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut e1: Arc<DAE::Exp>;
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut subs = (*subs).clone();
                    let mut pa = (*pa).clone();
                    explst = List::map(subs.clone(), (std::sync::Arc::new(Expression::getSubscriptExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> + 'static>));
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyList(explst.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0 @ Deref @ DAE::Exp::RANGE { .. }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut e in (extendRange(e1.clone(), vars.clone())?).into_iter().cloned() {
                    let __x = Arc::new(DAE::Subscript::INDEX { exp: e.clone() });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    crlst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
                    let __x = ComponentReference::subscriptCref(cr.clone(), list![s.clone()])?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    (varslst, p) = BackendVariable::getVarLst(crlst.clone(), vars.clone());
                    pa = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    Ok((inExp.clone(), false, (vars.clone(), pa.clone(), visitedPaths.clone(), isInitial.clone(), ofunctionTree.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: Deref @ metamodelica::List::Nil }, exp: e1 }, tpl) => {
                    let mut e1 = (*e1).clone();
                    let mut tpl = (*tpl).clone();
                    e1 = Expression::nthArrayExp(e1.clone(), i.clone())?;
                    (_, tpl) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), tpl.clone())?;
                    Ok((inExp.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { .. }, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TSUB { exp: e1, .. }, tpl) => {
                    let mut tpl = (*tpl).clone();
                    (_, tpl) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), tpl.clone())?;
                    Ok((inExp.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, visitedPaths, isInitial, ofunctionTree)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut p2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut pa = (*pa).clone();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    (_, p2) = BackendVariable::getVar(ComponentReference::crefPrefixStart(cr.clone()), vars.clone())?;
                    pa = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    pa = adjacencyRowExp1(varslst.clone(), p2.clone(), pa.clone(), 0)?;
                    Ok((inExp.clone(), true, (vars.clone(), pa.clone(), visitedPaths.clone(), isInitial.clone(), ofunctionTree.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, visitedPaths, isInitial, ofunctionTree)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut pa = (*pa).clone();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    pa = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    Ok((inExp.clone(), true, (vars.clone(), pa.clone(), visitedPaths.clone(), isInitial.clone(), ofunctionTree.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, pa, visitedPaths, isInitial, ofunctionTree)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut pa = (*pa).clone();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    pa = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 1)?;
                    Ok((inExp.clone(), false, (vars.clone(), pa.clone(), visitedPaths.clone(), isInitial.clone(), ofunctionTree.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: diffindx }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, pa, visitedPaths, isInitial, ofunctionTree)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut pa = (*pa).clone();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    pa = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), diffindx.clone())?;
                    Ok((inExp.clone(), false, (vars.clone(), pa.clone(), visitedPaths.clone(), isInitial.clone(), ofunctionTree.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, .. }, tpl) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    Ok((inExp.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, tpl) => {
                    let mut b: bool = false;
                    b = Flags::getConfigBool(Flags::DELAY_BREAK_LOOP.clone())? && ExpressionBasics::expEqual(e1.clone(), e2.clone())?;
                    Ok((inExp.clone(), !(b.clone()), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, (_, _, _, false, _)) => {
                    Ok(traversingadjacencyRowExpSolvableFinder(e1.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, (_, _, _, true, _)) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>);
                    (_, b1, tpl) = traversingadjacencyRowExpSolvableFinder(e1.clone(), inTpl.clone())?;
                    (_, b2, tpl) = traversingadjacencyRowExpSolvableFinder(e2.clone(), tpl.clone())?;
                    Ok((inExp.clone(), b1.clone() && b2.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, .. }, _) => {
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>);
                    (_, _, tpl) = traversingadjacencyRowExpSolvableFinder(e2.clone(), inTpl.clone())?;
                    Ok(traversingadjacencyRowExpSolvableFinder(e1.clone(), tpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { .. }, .. }, (vars, pa, visitedPaths, isInitial, ofunctionTree @ Some(functionTree))) => {
                    if !((!(AvlSetPath::hasKey(visitedPaths.clone(), var_field!((*inExp).path, DAE::Exp::CALL).clone())?))) { bail!("guard") }
                    let mut e1: Arc<DAE::Exp>;
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>);
                    (e1, _) = Inline::forceInlineCall(inExp.clone(), metamodelica::nil(), (Some(functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), Arc::new(openmodelica_ast_collections::AvlSetPath::Tree::EMPTY))?;
                    let false = (referenceEq(&inExp.clone(),&e1.clone())) else { bail!("pattern mismatch") };
                    (_, tpl) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), (vars.clone(), pa.clone(), AvlSetPath::add(visitedPaths.clone(), var_field!((*inExp).path, DAE::Exp::CALL).clone())?, isInitial.clone(), ofunctionTree.clone()))?;
                    Ok((inExp.clone(), false, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

fn traversingadjacencyRowIfExpSolvableFinder(mut e: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> {
    let mut e: Arc<DAE::Exp> = e;
    let mut cont: bool = false;
    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>) = tpl;
    tpl = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expElse, expThen, expCond } => {
                    if !((Expression::containsInitialCall(expCond.clone()))) { bail!("guard") }
                    let mut isInitial: bool = false;
                    let mut conditionTrue: bool = false;
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>) = tpl.clone();
                    (_, _, _, isInitial, _) = tpl.clone();
                    conditionTrue = (::match_deref::match_deref! { match &(expCond.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => isInitial.clone(),
        Deref @ DAE::Exp::LUNARY { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. }, operator: DAE::Operator::NOT { .. } } => !(isInitial.clone()),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    if conditionTrue.clone() {
                        (_, tpl) = Expression::traverseExpTopDown(expThen.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), tpl.clone())?;
                    } else {
                        (_, tpl) = Expression::traverseExpTopDown(expElse.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), tpl.clone())?;
                    }
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expElse, expThen, expCond } => {
                    let mut expCond = (*expCond).clone();
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>) = tpl.clone();
                    (expCond, _) = ExpressionSimplify::simplify(expCond.clone())?;
                    tpl = (::match_deref::match_deref! { match &(expCond.clone()) {
        Deref @ DAE::Exp::BCONST { bool: true } => {
                    (_, tpl) = Expression::traverseExpTopDown(expThen.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), tpl.clone())?;
                    tpl.clone()
        },
        Deref @ DAE::Exp::BCONST { bool: false } => {
                    (_, tpl) = Expression::traverseExpTopDown(expElse.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), tpl.clone())?;
                    tpl.clone()
        },
        _ => {
                    (_, tpl) = Expression::traverseExpTopDown(expThen.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), tpl.clone())?;
                    (_, tpl) = Expression::traverseExpTopDown(expElse.clone(), (std::sync::Arc::new(traversingadjacencyRowExpSolvableFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, Arc<AvlSetPath::Tree>, bool, Option<Arc<AvlTreePathFunction::Tree>>))> + 'static>), tpl.clone())?;
                    tpl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Error::addMessage(Error::GENERIC_ELAB_EXPRESSION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[BackendDAEUtil.traversingadjacencyRowIfExpSolvableFinder]: ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e.clone(), 0)?); __mm_s.push_str(&*literal!(": If-Expression could not be evaluated.")); ArcStr::from(__mm_s) }).clone()])?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((e, cont, tpl))
}

fn traversingadjacencyRowIfExp(mut e: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool), mut traFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> {
    pub type traverserFunction = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>;

    let mut e: Arc<DAE::Exp> = e;
    let mut cont: bool = false;
    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool) = tpl;
    tpl = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expElse, expThen, expCond } => {
                    if !((Expression::containsInitialCall(expCond.clone()))) { bail!("guard") }
                    let mut isInitial: bool = false;
                    let mut conditionTrue: bool = false;
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool) = tpl.clone();
                    (_, _, isInitial) = tpl.clone();
                    conditionTrue = (::match_deref::match_deref! { match &(expCond.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => isInitial.clone(),
        Deref @ DAE::Exp::LUNARY { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. }, operator: DAE::Operator::NOT { .. } } => !(isInitial.clone()),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    if conditionTrue.clone() {
                        (_, tpl) = Expression::traverseExpTopDown(expThen.clone(), traFunc.clone(), tpl.clone())?;
                    } else {
                        (_, tpl) = Expression::traverseExpTopDown(expElse.clone(), traFunc.clone(), tpl.clone())?;
                    }
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expElse, expThen, expCond } => {
                    let mut expCond = (*expCond).clone();
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool) = tpl.clone();
                    (expCond, _) = ExpressionSimplify::simplify(expCond.clone())?;
                    tpl = (::match_deref::match_deref! { match &(expCond.clone()) {
        Deref @ DAE::Exp::BCONST { bool: true } => {
                    (_, tpl) = Expression::traverseExpTopDown(expThen.clone(), traFunc.clone(), tpl.clone())?;
                    tpl.clone()
        },
        Deref @ DAE::Exp::BCONST { bool: false } => {
                    (_, tpl) = Expression::traverseExpTopDown(expElse.clone(), traFunc.clone(), tpl.clone())?;
                    tpl.clone()
        },
        _ => {
                    (_, tpl) = Expression::traverseExpTopDown(expCond.clone(), traFunc.clone(), tpl.clone())?;
                    (_, tpl) = Expression::traverseExpTopDown(expThen.clone(), traFunc.clone(), tpl.clone())?;
                    (_, tpl) = Expression::traverseExpTopDown(expElse.clone(), traFunc.clone(), tpl.clone())?;
                    tpl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Error::addMessage(Error::GENERIC_ELAB_EXPRESSION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[BackendDAEUtil.traversingadjacencyRowIfExp]: ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e.clone(), 0)?); __mm_s.push_str(&*literal!(": If-Expression could not be evaluated.")); ArcStr::from(__mm_s) }).clone()])?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((e, cont, tpl))
}

fn traversingAdjacencyRowIfExpEnhanced(mut e: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>), mut traFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> {
    pub type traverserFunction = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>;

    let mut e: Arc<DAE::Exp> = e;
    let mut cont: bool = false;
    let mut tpl: (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) = tpl;
    tpl = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expElse, expThen, expCond } => {
                    if !((Expression::containsInitialCall(expCond.clone()))) { bail!("guard") }
                    let mut isInitial: bool = false;
                    let mut conditionTrue: bool = false;
                    let mut tpl: (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) = tpl.clone();
                    (_, _, isInitial, _, _, _) = tpl.clone();
                    conditionTrue = (::match_deref::match_deref! { match &(expCond.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => isInitial.clone(),
        Deref @ DAE::Exp::LUNARY { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. }, operator: DAE::Operator::NOT { .. } } => !(isInitial.clone()),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    if conditionTrue.clone() {
                        (_, tpl) = Expression::traverseExpTopDown(expThen.clone(), traFunc.clone(), tpl.clone())?;
                    } else {
                        (_, tpl) = Expression::traverseExpTopDown(expElse.clone(), traFunc.clone(), tpl.clone())?;
                    }
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expElse, expThen, expCond } => {
                    let mut isInitial: bool = false;
                    let mut bs: bool = false;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut it: i32 = 0;
                    let mut mark: i32 = 0;
                    let mut at: metamodelica::Array<i32>;
                    let mut rowmark: metamodelica::Array<i32>;
                    let mut pa: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut bt: Arc<BinaryTree::BinTree> = Arc::new(<BinaryTree::BinTree as ::std::default::Default>::default());
                    let mut expCond = (*expCond).clone();
                    let mut tpl: (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) = tpl.clone();
                    (expCond, _) = ExpressionSimplify::simplify(expCond.clone())?;
                    tpl = (::match_deref::match_deref! { match &(expCond.clone()) {
        Deref @ DAE::Exp::BCONST { bool: true } => {
                    (_, tpl) = Expression::traverseExpTopDown(expThen.clone(), traFunc.clone(), tpl.clone())?;
                    tpl.clone()
        },
        Deref @ DAE::Exp::BCONST { bool: false } => {
                    (_, tpl) = Expression::traverseExpTopDown(expElse.clone(), traFunc.clone(), tpl.clone())?;
                    tpl.clone()
        },
        _ => {
                    (vars, bs, isInitial, it, at, pa) = tpl.clone();
                    mark = it.clone();
                    rowmark = at.clone();
                    let (_, (__pa0, _, _, _, _, __pa1)) = Expression::traverseExpTopDown(expThen.clone(), traFunc.clone(), (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa0.clone();
                    pa = __pa1.clone();
                    let (_, (__pa2, _, _, _, _, __pa3)) = Expression::traverseExpTopDown(expElse.clone(), traFunc.clone(), (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa2.clone();
                    pa = __pa3.clone();
                    let (_, (__pa4, _, _, _, _, __pa5)) = Expression::traverseExpTopDown(expCond.clone(), traFunc.clone(), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa4.clone();
                    pa = __pa5.clone();
                    (_, bt) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(getIfExpBranchVarOccurency) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<BinaryTree::BinTree>) -> Result<(Arc<DAE::Exp>, bool, Arc<BinaryTree::BinTree>)> + 'static>), BinaryTree::emptyBinTree().clone())?;
                    Expression::traverseExpTopDown(expThen.clone(), (std::sync::Arc::new(markBranchVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, metamodelica::Array<i32>, BackendDAE::Variables, Arc<BinaryTree::BinTree>)) -> Result<(Arc<DAE::Exp>, bool, (i32, metamodelica::Array<i32>, BackendDAE::Variables, Arc<BinaryTree::BinTree>))> + 'static>), (mark.clone(), rowmark.clone(), vars.clone(), bt.clone()))?;
                    Expression::traverseExpTopDown(expElse.clone(), (std::sync::Arc::new(markBranchVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, metamodelica::Array<i32>, BackendDAE::Variables, Arc<BinaryTree::BinTree>)) -> Result<(Arc<DAE::Exp>, bool, (i32, metamodelica::Array<i32>, BackendDAE::Variables, Arc<BinaryTree::BinTree>))> + 'static>), (mark.clone(), rowmark.clone(), vars.clone(), bt.clone()))?;
                    (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Error::addMessage(Error::GENERIC_ELAB_EXPRESSION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[BackendDAEUtil.traversingAdjacencyRowIfExpEnhanced]: ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e.clone(), 0)?); __mm_s.push_str(&*literal!(": If-Expression could not be evaluated.")); ArcStr::from(__mm_s) }).clone()])?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((e, cont, tpl))
}

pub fn traversingAdjacencyRowExpFinderBaseClock(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut p2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pa = (*pa).clone();
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    (_, p2) = BackendVariable::getVar(ComponentReference::crefPrefixStart(cr.clone()), vars.clone())?;
                    pa = AvlSetInt::addList(pa.clone(), p.clone())?;
                    pa = AvlSetInt::addList(pa.clone(), p2.clone())?;
                    Ok((inExp.clone(), true, (vars.clone(), pa.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok((inExp.clone(), true, (vars.clone(), AvlSetInt::addList(pa.clone(), p.clone())?, isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, _) => {
                    let mut outTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
                    (_, outTpl) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpFinderBaseClock) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), inTpl.clone())?;
                    Ok((inExp.clone(), false, outTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: e, .. } }, _) => {
                    let mut outTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
                    (_, outTpl) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpFinderBaseClock) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), inTpl.clone())?;
                    Ok((inExp.clone(), true, outTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { .. } }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, tpl) => {
                    Ok(traversingadjacencyRowIfExp(inExp.clone(), tpl.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpFinderBaseClock) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

pub fn traversingAdjacencyRowExpFinderSubClock(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut p2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    (_, p2) = BackendVariable::getVar(ComponentReference::crefPrefixStart(cr.clone()), vars.clone())?;
                    res = AvlSetInt::addList(pa.clone(), p.clone())?;
                    res = AvlSetInt::addList(res.clone(), p2.clone())?;
                    Ok((inExp.clone(), true, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = AvlSetInt::addList(pa.clone(), p.clone())?;
                    Ok((inExp.clone(), true, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, tpl) => {
                    Ok(traversingadjacencyRowIfExp(inExp.clone(), tpl.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpFinderSubClock) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

pub fn traversingadjacencyRowExpFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut p2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    (_, p2) = BackendVariable::getVar(ComponentReference::crefPrefixStart(cr.clone()), vars.clone())?;
                    res = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    res = adjacencyRowExp1(varslst.clone(), p2.clone(), res.clone(), 0)?;
                    Ok((e.clone(), true, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    Ok((e.clone(), true, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 1)?;
                    let (_, (_, __pa0, _)) = Expression::traverseExpTopDownCrefHelper(cr.clone(), (std::sync::Arc::new(traversingadjacencyRowExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (vars.clone(), res.clone(), isInitial.clone()))?;
                    res = __pa0.clone();
                    Ok((e.clone(), false, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::crefPrefixDer(cr.clone());
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 1)?;
                    let (_, (_, __pa0, _)) = Expression::traverseExpTopDownCrefHelper(cr.clone(), (std::sync::Arc::new(traversingadjacencyRowExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (vars.clone(), res.clone(), isInitial.clone()))?;
                    res = __pa0.clone();
                    Ok((e.clone(), false, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, _) => {
                    let mut b: bool = false;
                    b = Flags::getConfigBool(Flags::DELAY_BREAK_LOOP.clone())? && ExpressionBasics::expEqual(e1.clone(), e2.clone())?;
                    Ok((inExp.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, (_, _, false)) => {
                    Ok(traversingadjacencyRowExpFinder(e1.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, (_, _, true)) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
                    (_, b1, tpl) = traversingadjacencyRowExpFinder(e1.clone(), inTpl.clone())?;
                    (_, b2, tpl) = traversingadjacencyRowExpFinder(e2.clone(), tpl.clone())?;
                    Ok((inExp.clone(), b1.clone() && b2.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, .. }, _) => {
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
                    (_, _, tpl) = traversingadjacencyRowExpFinder(e2.clone(), inTpl.clone())?;
                    Ok(traversingadjacencyRowExpFinder(e1.clone(), tpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: Deref @ metamodelica::List::Nil }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    let mut pa = (*pa).clone();
                    cr = ComponentReference::subscriptCrefWithInt(cr.clone(), i.clone())?;
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    pa = adjacencyRowExp1(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    Ok((inExp.clone(), false, (vars.clone(), pa.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: Deref @ metamodelica::List::Nil }, exp: e1 }, (vars, _, isInitial)) => {
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut e1 = (*e1).clone();
                    e1 = Expression::nthArrayExp(e1.clone(), i.clone())?;
                    let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingadjacencyRowExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), inTpl.clone())?;
                    res = __pa0.clone();
                    Ok((inExp.clone(), false, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { .. }, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, tpl) => {
                    Ok(traversingadjacencyRowIfExp(inExp.clone(), tpl.clone(), (std::sync::Arc::new(traversingadjacencyRowExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTpl.clone()))
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
fn adjacencyRowExp1(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut inVarIndxLst: Arc<AvlSetInt::Tree>, mut diffindex: i32) -> Result<Arc<AvlSetInt::Tree>> {
    let mut outVarIndxLst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    outVarIndxLst = (::match_deref::match_deref! { match &((inVarLst.clone(), inIntegerLst.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            inVarIndxLst.clone()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) => {
            let mut vars: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut i1: i32 = 0;
            i1 = if (intGe(diffindex.clone(), 1)) {i.clone()} else {-(i.clone())};
            vars = AvlSetInt::add(inVarIndxLst.clone(), i1.clone())?;
            adjacencyRowExp1(rest.clone(), irest.clone(), vars.clone(), diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: diffidx, .. }, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) => {
            let mut vars: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut i1: i32 = 0;
            i1 = if (intGe(diffindex.clone(), diffidx.clone())) {i.clone()} else {-(i.clone())};
            vars = AvlSetInt::add(inVarIndxLst.clone(), i1.clone())?;
            adjacencyRowExp1(rest.clone(), irest.clone(), vars.clone(), diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) => {
            let mut vars: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            vars = AvlSetInt::add(inVarIndxLst.clone(), i.clone())?;
            adjacencyRowExp1(rest.clone(), irest.clone(), vars.clone(), diffindex.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVarIndxLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn adjacencyRowExp1DiscreteOrArray(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut inVarIndxLst: Arc<AvlSetInt::Tree>) -> Result<Arc<AvlSetInt::Tree>> {
    let mut outVarIndxLst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    outVarIndxLst = (::match_deref::match_deref! { match &((inVarLst.clone(), inIntegerLst.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            inVarIndxLst.clone()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) => {
            let mut vars: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            vars = AvlSetInt::add(inVarIndxLst.clone(), i.clone())?;
            adjacencyRowExp1DiscreteOrArray(rest.clone(), irest.clone(), vars.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, Deref @ metamodelica::List::Cons { head: _, tail: irest }) => {
            adjacencyRowExp1DiscreteOrArray(rest.clone(), irest.clone(), inVarIndxLst.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVarIndxLst)
}

pub fn traversingadjacencyRowExpFinderwithInput(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    cr = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(BackendDAE::partialDerivativeNamePrefix)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil(), cr.clone());
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1withInput(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    Ok((inExp.clone(), false, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1withInput(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    (varslst, p) = BackendVariable::getVar(ComponentReference::crefPrefixStart(cr.clone()), vars.clone())?;
                    res = adjacencyRowExp1withInput(varslst.clone(), p.clone(), res.clone(), 0)?;
                    Ok((inExp.clone(), true, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1withInput(varslst.clone(), p.clone(), pa.clone(), 0)?;
                    Ok((inExp.clone(), true, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1withInput(varslst.clone(), p.clone(), pa.clone(), 1)?;
                    Ok((inExp.clone(), false, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::crefPrefixDer(cr.clone());
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1withInput(varslst.clone(), p.clone(), pa.clone(), 1)?;
                    Ok((inExp.clone(), false, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, (vars, pa, isInitial)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    cr = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::previousNamePrefix)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil(), cr.clone());
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExp1withInput(varslst.clone(), p.clone(), pa.clone(), 1)?;
                    Ok((inExp.clone(), false, (vars.clone(), res.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, (_, _, false)) => {
                    Ok(traversingadjacencyRowExpFinderwithInput(e1.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, (_, _, true)) => {
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (_, b1, tpl) = traversingadjacencyRowExpFinderwithInput(e1.clone(), inTpl.clone())?;
                    (_, b2, tpl) = traversingadjacencyRowExpFinderwithInput(e2.clone(), tpl.clone())?;
                    Ok((inExp.clone(), b1.clone() && b2.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, .. }, _) => {
                    let mut tpl: (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool);
                    (_, _, tpl) = traversingadjacencyRowExpFinderwithInput(e2.clone(), inTpl.clone())?;
                    Ok(traversingadjacencyRowExpFinderwithInput(e1.clone(), tpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, tpl) => {
                    Ok(traversingadjacencyRowIfExp(inExp.clone(), tpl.clone(), (std::sync::Arc::new(traversingadjacencyRowExpFinderwithInput) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

fn adjacencyRowExp1withInput(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut vars: Arc<AvlSetInt::Tree>, mut diffindex: i32) -> Result<Arc<AvlSetInt::Tree>> {
    let mut outIntegerLst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    outIntegerLst = (::match_deref::match_deref! { match &((inVarLst.clone(), inIntegerLst.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            vars.clone()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DAE_AUX_VAR, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DAE_RESIDUAL_VAR, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::JAC_TMP_VAR, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(diffindex.clone() == 0 || AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE_DER, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::CLOCKED_STATE { .. }, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::ALG_STATE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_CONSTR, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_FCONSTR, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }) if (!(AvlSetInt::hasKey(vars.clone(), i.clone())?)) => {
            adjacencyRowExp1(rest.clone(), irest.clone(), AvlSetInt::add(vars.clone(), i.clone())?, diffindex.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            vars.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outIntegerLst)
}

pub fn updateAdjacencyMatrix(mut syst: Arc<BackendDAE::EqSystem>, mut inIndxType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut isInitial: bool) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    osyst = 'mc: {
        let __mc_input = syst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { mapping: Some(mapping), mT: Some(mt), m: Some(m), orderedEqs: daeeqns, orderedVars: vars, .. } => {
                    let mut mt = (*mt).clone();
                    let mut m = (*m).clone();
                    (m, mt) = updateAdjacencyMatrix1(vars.clone(), daeeqns.clone(), inIndxType.clone(), functionTree.clone(), m.clone(), mt.clone(), inIntegerLst.clone(), isInitial.clone())?;
                    Ok(setEqSystMatrices(syst.clone(), Some(m.clone()), Some(mt.clone()), Some(mapping.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAEUtil.updateAdjacencyMatrix failed")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(osyst)
}

fn updateAdjacencyMatrix1(mut vars: BackendDAE::Variables, mut daeeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inIndxType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (outAdjacencyMatrix, outAdjacencyMatrixT) = (::match_deref::match_deref! { match &(inIntegerLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            (m.clone(), mt.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: eqns } => {
            let mut m_1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut m_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_3: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut abse: i32 = 0;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut row: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut invars: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut outvars: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut oldvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            abse = intAbs(e.clone());
            eqn = BackendEquation::get(daeeqns.clone(), abse.clone());
            (row, _) = adjacencyRow(eqn.clone(), vars.clone(), inIndxType.clone(), functionTree.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone())?;
            oldvars = getOldVars(m.clone(), abse.clone());
            m_1 = Array::replaceAtWithFill(abse.clone(), AvlSetInt::listKeys(row.clone(), metamodelica::nil()), metamodelica::nil(), m.clone())?;
            (_, outvars, invars) = AvlSetInt::intersection(AvlSetInt::addList(Arc::new(crate::AvlSetInt::Tree::EMPTY), oldvars.clone())?, row.clone())?;
            mt_1 = removeValuefromMatrix(abse.clone(), AvlSetInt::listKeys(outvars.clone(), metamodelica::nil()), mt.clone())?;
            mt_2 = addValuetoMatrix(abse.clone(), AvlSetInt::listKeys(invars.clone(), metamodelica::nil()), mt_1.clone())?;
            (m_2, mt_3) = updateAdjacencyMatrix1(vars.clone(), daeeqns.clone(), inIndxType.clone(), functionTree.clone(), m_1.clone(), mt_2.clone(), eqns.clone(), isInitial.clone())?;
            (m_2.clone(), mt_3.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outAdjacencyMatrix, outAdjacencyMatrixT))
}

pub fn updateAdjacencyMatrixScalar(mut syst: Arc<BackendDAE::EqSystem>, mut inIndxType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut iMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMapIncRowEqn: metamodelica::Array<i32>, mut isInitial: bool) -> Result<(Arc<BackendDAE::EqSystem>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut oMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oMapIncRowEqn: metamodelica::Array<i32>;
    (osyst, oMapEqnIncRow, oMapIncRowEqn) = 'mc: {
        let __mc_input = syst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { mapping: Some((_, _, indexType, scalar, processed)), mT: Some(mt), m: Some(m), orderedEqs: daeeqns, orderedVars: vars, .. } => {
                    let mut oldsize: i32 = 0;
                    let mut newsize: i32 = 0;
                    let mut oldsize1: i32 = 0;
                    let mut newsize1: i32 = 0;
                    let mut deltasize: i32 = 0;
                    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut mt = (*mt).clone();
                    let mut m = (*m).clone();
                    oldsize = (iMapEqnIncRow.clone().borrow().len() as i32);
                    newsize = BackendEquation::getNumberOfEquations(daeeqns.clone());
                    mapEqnIncRow = Array::expand(newsize.clone() - oldsize.clone(), iMapEqnIncRow.clone(), metamodelica::nil())?;
                    oldsize1 = (iMapIncRowEqn.clone().borrow().len() as i32);
                    newsize1 = BackendEquation::equationArraySize(daeeqns.clone())?;
                    deltasize = newsize1.clone() - oldsize1.clone();
                    mapIncRowEqn = Array::expand(deltasize.clone(), iMapIncRowEqn.clone(), 0)?;
                    m = Array::expand(deltasize.clone(), m.clone(), metamodelica::nil())?;
                    mt = Array::expand(deltasize.clone(), mt.clone(), metamodelica::nil())?;
                    (m, mt, mapEqnIncRow, mapIncRowEqn) = updateAdjacencyMatrixScalar2(oldsize.clone() + 1, newsize.clone(), oldsize1.clone(), vars.clone(), daeeqns.clone(), m.clone(), mt.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inIndxType.clone(), functionTree.clone(), isInitial.clone())?;
                    eqns = List::removeOnTrue(oldsize.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), inIntegerLst.clone());
                    (m, mt, mapEqnIncRow, mapIncRowEqn) = updateAdjacencyMatrixScalar1(vars.clone(), daeeqns.clone(), m.clone(), mt.clone(), eqns.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inIndxType.clone(), functionTree.clone(), isInitial.clone())?;
                    Ok((setEqSystMatrices(syst.clone(), Some(m.clone()), Some(mt.clone()), Some((mapEqnIncRow.clone(), mapIncRowEqn.clone(), indexType.clone(), scalar.clone(), processed.clone())))?, mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAEUtil.updateAdjacencyMatrixScalar failed")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oMapEqnIncRow, oMapIncRowEqn))
}

fn updateAdjacencyMatrixScalar1(mut vars: BackendDAE::Variables, mut daeeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut iMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMapIncRowEqn: metamodelica::Array<i32>, mut inIndxType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oMapIncRowEqn: metamodelica::Array<i32>;
    (outAdjacencyMatrix, outAdjacencyMatrixT, oMapEqnIncRow, oMapIncRowEqn) = (::match_deref::match_deref! { match &(inIntegerLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            (m.clone(), mt.clone(), iMapEqnIncRow.clone(), iMapIncRowEqn.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: eqns } if (BackendEquation::has(daeeqns.clone(), intAbs(e.clone()))) => {
            let mut m_1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut m_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_3: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut abse: i32 = 0;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut row: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut invarsTree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut outvarsTree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut invars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut outvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut oldvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut scalarindxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            abse = intAbs(e.clone());
            eqn = BackendEquation::get(daeeqns.clone(), abse.clone());
            (row, _) = adjacencyRow(eqn.clone(), vars.clone(), inIndxType.clone(), functionTree.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone())?;
            scalarindxs = iMapEqnIncRow.borrow()[(abse.clone()-1) as usize].clone();
            oldvars = getOldVars(m.clone(), listHead(scalarindxs.clone())?);
            (_, outvarsTree, invarsTree) = AvlSetInt::intersection(AvlSetInt::addList(Arc::new(crate::AvlSetInt::Tree::EMPTY), oldvars.clone())?, row.clone())?;
            outvars = AvlSetInt::listKeys(outvarsTree.clone(), metamodelica::nil());
            invars = AvlSetInt::listKeys(invarsTree.clone(), metamodelica::nil());
            m_1 = List::fold1r(scalarindxs.clone(), Arc::new(arrayUpdate.clone()), AvlSetInt::listKeys(row.clone(), metamodelica::nil()), m.clone());
            mt_1 = List::fold1(scalarindxs.clone(), (std::sync::Arc::new(removeValuefromMatrix) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), outvars.clone(), mt.clone());
            mt_2 = List::fold1(scalarindxs.clone(), (std::sync::Arc::new(addValuetoMatrix) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), invars.clone(), mt_1.clone());
            (m_2, mt_3, mapEqnIncRow, mapIncRowEqn) = updateAdjacencyMatrixScalar1(vars.clone(), daeeqns.clone(), m_1.clone(), mt_2.clone(), eqns.clone(), iMapEqnIncRow.clone(), iMapIncRowEqn.clone(), inIndxType.clone(), functionTree.clone(), isInitial.clone())?;
            (m_2.clone(), mt_3.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: eqns } => {
            let mut m_1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut m_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt_3: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut abse: i32 = 0;
            let mut row: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut invarsTree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut outvarsTree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            let mut invars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut outvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut oldvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut scalarindxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            abse = intAbs(e.clone());
            row = Arc::new(crate::AvlSetInt::Tree::EMPTY);
            scalarindxs = iMapEqnIncRow.borrow()[(abse.clone()-1) as usize].clone();
            oldvars = getOldVars(m.clone(), listHead(scalarindxs.clone())?);
            (_, outvarsTree, invarsTree) = AvlSetInt::intersection(AvlSetInt::addList(Arc::new(crate::AvlSetInt::Tree::EMPTY), oldvars.clone())?, row.clone())?;
            outvars = AvlSetInt::listKeys(outvarsTree.clone(), metamodelica::nil());
            invars = AvlSetInt::listKeys(invarsTree.clone(), metamodelica::nil());
            m_1 = List::fold1r(scalarindxs.clone(), Arc::new(arrayUpdate.clone()), AvlSetInt::listKeys(row.clone(), metamodelica::nil()), m.clone());
            mt_1 = List::fold1(scalarindxs.clone(), (std::sync::Arc::new(removeValuefromMatrix) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), outvars.clone(), mt.clone());
            mt_2 = List::fold1(scalarindxs.clone(), (std::sync::Arc::new(addValuetoMatrix) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), invars.clone(), mt_1.clone());
            (m_2, mt_3, mapEqnIncRow, mapIncRowEqn) = updateAdjacencyMatrixScalar1(vars.clone(), daeeqns.clone(), m_1.clone(), mt_2.clone(), eqns.clone(), iMapEqnIncRow.clone(), iMapIncRowEqn.clone(), inIndxType.clone(), functionTree.clone(), isInitial.clone())?;
            (m_2.clone(), mt_3.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outAdjacencyMatrix, outAdjacencyMatrixT, oMapEqnIncRow, oMapIncRowEqn))
}

fn updateAdjacencyMatrixScalar2(mut index: i32, mut n: i32, mut size: i32, mut vars: BackendDAE::Variables, mut daeeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMapIncRowEqn: metamodelica::Array<i32>, mut inIndxType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oMapIncRowEqn: metamodelica::Array<i32>;
    (outAdjacencyMatrix, outAdjacencyMatrixT, oMapEqnIncRow, oMapIncRowEqn) = 'mc: {
        let __mc_input = functionTree.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((!(intGt(index.clone(), n.clone())) && BackendEquation::has(daeeqns.clone(), intAbs(index.clone())))) { bail!("guard") }
                    let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mt1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut abse: i32 = 0;
                    let mut rowsize: i32 = 0;
                    let mut new_size: i32 = 0;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut row: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut scalarindxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    abse = intAbs(index.clone());
                    eqn = BackendEquation::get(daeeqns.clone(), abse.clone());
                    rowsize = BackendEquation::equationSize(eqn.clone())?;
                    (row, _) = adjacencyRow(eqn.clone(), vars.clone(), inIndxType.clone(), functionTree.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone())?;
                    new_size = size.clone() + rowsize.clone();
                    scalarindxs = List::intRange2(size.clone() + 1, new_size.clone());
                    mapEqnIncRow = {let _arr = iMapEqnIncRow.clone(); _arr.borrow_mut()[(abse.clone()-1) as usize] = scalarindxs.clone(); _arr};
                    mapIncRowEqn = List::fold1r(scalarindxs.clone(), Arc::new(arrayUpdate.clone()), abse.clone(), iMapIncRowEqn.clone());
                    row_lst = AvlSetInt::listKeys(row.clone(), metamodelica::nil());
                    m1 = List::fold1r(scalarindxs.clone(), Arc::new(arrayUpdate.clone()), row_lst.clone(), m.clone());
                    mt1 = filladjacencyMatrixT(row_lst.clone(), scalarindxs.clone(), mt.clone())?;
                    (m1, mt1, mapEqnIncRow, mapIncRowEqn) = updateAdjacencyMatrixScalar2(index.clone() + 1, n.clone(), new_size.clone(), vars.clone(), daeeqns.clone(), m1.clone(), mt1.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inIndxType.clone(), functionTree.clone(), isInitial.clone())?;
                    Ok((m1.clone(), mt1.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((m.clone(), mt.clone(), iMapEqnIncRow.clone(), iMapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mt1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut abse: i32 = 0;
                    let mut rowsize: i32 = 0;
                    let mut new_size: i32 = 0;
                    let mut row: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut scalarindxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    abse = intAbs(index.clone());
                    rowsize = 1;
                    row = Arc::new(crate::AvlSetInt::Tree::EMPTY);
                    new_size = size.clone() + rowsize.clone();
                    scalarindxs = List::intRange2(size.clone() + 1, new_size.clone());
                    mapEqnIncRow = {let _arr = iMapEqnIncRow.clone(); _arr.borrow_mut()[(abse.clone()-1) as usize] = scalarindxs.clone(); _arr};
                    mapIncRowEqn = List::fold1r(scalarindxs.clone(), Arc::new(arrayUpdate.clone()), abse.clone(), iMapIncRowEqn.clone());
                    row_lst = AvlSetInt::listKeys(row.clone(), metamodelica::nil());
                    m1 = List::fold1r(scalarindxs.clone(), Arc::new(arrayUpdate.clone()), row_lst.clone(), m.clone());
                    mt1 = filladjacencyMatrixT(row_lst.clone(), scalarindxs.clone(), mt.clone())?;
                    (m1, mt1, mapEqnIncRow, mapIncRowEqn) = updateAdjacencyMatrixScalar2(index.clone() + 1, n.clone(), new_size.clone(), vars.clone(), daeeqns.clone(), m1.clone(), mt1.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inIndxType.clone(), functionTree.clone(), isInitial.clone())?;
                    Ok((m1.clone(), mt1.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAdjacencyMatrix, outAdjacencyMatrixT, oMapEqnIncRow, oMapIncRowEqn))
}

fn getOldVars(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut pos: i32) -> Arc<metamodelica::List<i32>> {
    let mut oldvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oldvars = if (pos.clone() <= (m.clone().borrow().len() as i32)) {m.borrow()[(pos.clone()-1) as usize].clone()} else {metamodelica::nil()};
    oldvars
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeValuefromMatrix(mut inValue: i32, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut inAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    outAdjacencyMatrixT = 'mc: {
        let __mc_input = (inValue.clone(), inIntegerLst.clone(), inAdjacencyMatrixT.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, mt) => {
                    Ok(mt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, Deref @ metamodelica::List::Cons { head: k, tail: keys }, mt) => {
                    let mut mt_1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mt_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mlst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut kabs: i32 = 0;
                    let mut v_1: i32 = 0;
                    kabs = intAbs(k.clone());
                    mlst = mt.borrow()[(kabs.clone()-1) as usize].clone();
                    v_1 = if (intGt(k.clone(), 0)) {v.clone()} else {-(v.clone())};
                    mlst1 = List::removeOnTrue(v_1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), mlst.clone());
                    mt_1 = {let _arr = mt.clone(); _arr.borrow_mut()[(kabs.clone()-1) as usize] = mlst1.clone(); _arr};
                    mt_2 = removeValuefromMatrix(v.clone(), keys.clone(), mt_1.clone())?;
                    Ok(mt_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, Deref @ metamodelica::List::Cons { head: _, tail: keys }, mt) => {
                    let mut mt_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    mt_2 = removeValuefromMatrix(v.clone(), keys.clone(), mt.clone())?;
                    Ok(mt_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    println!("{}", (literal!("- BackendDAE.removeValuefromMatrix failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAdjacencyMatrixT)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addValuetoMatrix(mut inValue: i32, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut inAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    outAdjacencyMatrixT = 'mc: {
        let __mc_input = (inValue.clone(), inIntegerLst.clone(), inAdjacencyMatrixT.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, mt) => {
                    Ok(mt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, Deref @ metamodelica::List::Cons { head: k, tail: keys }, mt) => {
                    let mut mt_1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mt_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut kabs: i32 = 0;
                    let mut v_1: i32 = 0;
                    kabs = intAbs(k.clone());
                    mlst = getOldVars(mt.clone(), kabs.clone());
                    v_1 = if (intGt(k.clone(), 0)) {v.clone()} else {-(v.clone())};
                    let false = (listMember(v_1.clone(), mlst.clone())) else { bail!("pattern mismatch") };
                    mt_1 = Array::replaceAtWithFill(kabs.clone(), cons(v_1.clone(), mlst.clone()), metamodelica::nil(), mt.clone())?;
                    mt_2 = addValuetoMatrix(v.clone(), keys.clone(), mt_1.clone())?;
                    Ok(mt_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, Deref @ metamodelica::List::Cons { head: _, tail: keys }, mt) => {
                    let mut mt_2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    mt_2 = addValuetoMatrix(v.clone(), keys.clone(), mt.clone())?;
                    Ok(mt_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    println!("{}", (literal!("- BackendDAE.addValuetoMatrix failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAdjacencyMatrixT)
}

pub fn getAdjacencyMatrixfromOptionForMapEqSystem(mut syst: Arc<BackendDAE::EqSystem>, mut inIndxType: BackendDAE::IndexType, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    funcs = getFunctions(shared.clone())?;
    (osyst, _, _) = getAdjacencyMatrixfromOption(syst.clone(), inIndxType.clone(), Some(funcs.clone()), isInitializationDAE(shared.clone()))?;
    oshared = shared.clone();
    Ok((osyst, oshared))
}

pub fn getAdjacencyMatrixfromOption(mut inSyst: Arc<BackendDAE::EqSystem>, mut inIndxType: BackendDAE::IndexType, mut inFunctionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(Arc<BackendDAE::EqSystem>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (outSyst, outM, outMT) = (::match_deref::match_deref! { match &(inSyst.clone()) {
        Deref @ BackendDAE::EqSystem { m: None, .. } => {
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mapping: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, BackendDAE::IndexType, bool, bool);
            (m, mT) = adjacencyMatrix(inSyst.clone(), inIndxType.clone(), inFunctionTree.clone(), isInitial.clone())?;
            mapping = getArrayAdjacencyMatrixMapping(ExpandableArray::getNumberOfElements(inSyst.orderedEqs.clone()), inIndxType.clone(), false);
            (setEqSystMatrices(inSyst.clone(), Some(m.clone()), Some(mT.clone()), Some(mapping.clone()))?, m.clone(), mT.clone())
        },
        Deref @ BackendDAE::EqSystem { mapping: None, mT: None, m: Some(m), orderedVars: v, .. } => {
            let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            mT = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), BackendVariable::varsSize(v.clone())?)?;
            (setEqSystMatrices(inSyst.clone(), Some(m.clone()), Some(mT.clone()), None)?, m.clone(), mT.clone())
        },
        Deref @ BackendDAE::EqSystem { mapping: Some(mapping), mT: None, m: Some(m), orderedVars: v, .. } => {
            let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            mT = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), BackendVariable::varsSize(v.clone())?)?;
            (setEqSystMatrices(inSyst.clone(), Some(m.clone()), Some(mT.clone()), Some(mapping.clone()))?, m.clone(), mT.clone())
        },
        Deref @ BackendDAE::EqSystem { mT: Some(mT), m: Some(m), .. } => {
            (inSyst.clone(), m.clone(), mT.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSyst, outM, outMT))
}

pub fn getArrayAdjacencyMatrixMapping(mut size: i32, mut indexType: BackendDAE::IndexType, mut scalar: bool) -> (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, BackendDAE::IndexType, bool, bool) {
    let mut mapping: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, BackendDAE::IndexType, bool, bool);
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>> = arrayCreate(size.clone(), list![-1]);
    let mut mapIncRowEqn: metamodelica::Array<i32> = arrayCreate(size.clone(), -1);
    for mut i in 1..=size.clone() {
        {
            let __cell0 = list![i.clone()];
            mapEqnIncRow.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
        }
        {
            let __cell1 = i.clone();
            mapIncRowEqn.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
        }
    }
    mapping = (mapEqnIncRow.clone(), mapIncRowEqn.clone(), indexType.clone(), scalar.clone(), false);
    mapping
}

pub fn getAdjacencyMatrix(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inIndxType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(Arc<BackendDAE::EqSystem>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapping: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, BackendDAE::IndexType, bool, bool);
    (outM, outMT) = adjacencyMatrix(inEqSystem.clone(), inIndxType.clone(), functionTree.clone(), isInitial.clone())?;
    mapping = getArrayAdjacencyMatrixMapping(ExpandableArray::getNumberOfElements(inEqSystem.orderedEqs.clone()), inIndxType.clone(), false);
    outEqSystem = setEqSystMatrices(inEqSystem.clone(), Some(outM.clone()), Some(outMT.clone()), Some(mapping.clone()))?;
    Ok((outEqSystem, outM, outMT))
}

pub fn getAdjacencyMatrixScalar(mut syst: Arc<BackendDAE::EqSystem>, mut inIndxType: BackendDAE::IndexType, mut functionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(Arc<BackendDAE::EqSystem>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMapIncRowEqn: metamodelica::Array<i32>;
    (outM, outMT, outMapEqnIncRow, outMapIncRowEqn) = adjacencyMatrixScalar(syst.clone(), inIndxType.clone(), functionTree.clone(), isInitial.clone())?;
    osyst = setEqSystMatrices(syst.clone(), Some(outM.clone()), Some(outMT.clone()), Some((outMapEqnIncRow.clone(), outMapIncRowEqn.clone(), inIndxType.clone(), true, false)))?;
    Ok((osyst, outM, outMT, outMapEqnIncRow, outMapIncRowEqn))
}

pub fn removedAdjacencyMatrix(mut inSyst: Arc<BackendDAE::EqSystem>, mut inIndxType: BackendDAE::IndexType, mut inFunctionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (outM, outMT) = adjacencyMatrixDispatch(inSyst.orderedVars.clone(), inSyst.removedEqs.clone(), inIndxType.clone(), inFunctionTree.clone(), isInitial.clone())?;
    Ok((outM, outMT))
}

pub fn removedAdjacencyMatrixMasked(mut inSyst: Arc<BackendDAE::EqSystem>, mut inIndxType: BackendDAE::IndexType, mut inMask: metamodelica::Array<bool>, mut inFunctionTree: Option<Arc<AvlTreePathFunction::Tree>>, mut isInitial: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (outM, outMT) = adjacencyMatrixDispatchMasked(inSyst.orderedVars.clone(), inSyst.removedEqs.clone(), inIndxType.clone(), inMask.clone(), inFunctionTree.clone(), isInitial.clone())?;
    Ok((outM, outMT))
}

fn traverseStmts<ArgT: Clone + 'static>(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<ArgT> + 'static>, mut extraArg: ArgT) -> Result<ArgT> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<ArgT> + 'static>;

    let mut extraArg: ArgT = extraArg;
    let mut e: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ew: Arc<DAE::Statement>;
    let mut id1: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut algElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    for mut stmt in &*inStmts.clone() {
        let mut stmt = stmt.clone();
        extraArg = 'mc: {
        let __mc_input = stmt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN { exp: e, exp1: e2, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = func(e.clone(), extraArg.clone())?;
                    extraArg = func(e2.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: e, expExpLst: expl1, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = func(e.clone(), extraArg.clone())?;
                    extraArg = List::fold(expl1.clone(), func.clone(), extraArg.clone());
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: e, lhs: e2, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = func(e.clone(), extraArg.clone())?;
                    extraArg = func(e2.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_IF { else_: algElse, statementLst: stmts, exp: e, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = traverseStmtsElse(algElse.clone(), func.clone(), extraArg.clone())?;
                    extraArg = traverseStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    extraArg = func(e.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FOR { statementLst: stmts, range: e, iter: id1, type_: tp, .. } => {
                    let mut stmts = (*stmts).clone();
                    let mut cr: Arc<DAE::ComponentRef> = cr.clone();
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = func(e.clone(), extraArg.clone())?;
                    cr = ComponentReferenceBasics::makeCrefIdent((id1.clone()).clone(), tp.clone(), metamodelica::nil());
                    (stmts, _) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>))> + 'static>), (cr.clone(), e.clone())));
                    extraArg = traverseStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_PARFOR { statementLst: stmts, range: e, iter: id1, type_: tp, .. } => {
                    let mut stmts = (*stmts).clone();
                    let mut cr: Arc<DAE::ComponentRef> = cr.clone();
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = func(e.clone(), extraArg.clone())?;
                    cr = ComponentReferenceBasics::makeCrefIdent((id1.clone()).clone(), tp.clone(), metamodelica::nil());
                    (stmts, _) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>))> + 'static>), (cr.clone(), e.clone())));
                    extraArg = traverseStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHILE { statementLst: stmts, exp: e, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = traverseStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    extraArg = func(e.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { elseWhen: None, statementLst: stmts, exp: e, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = traverseStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    extraArg = func(e.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { elseWhen: Some(ew), statementLst: stmts, exp: e, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = traverseStmts(list![ew.clone()], func.clone(), extraArg.clone())?;
                    extraArg = traverseStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    extraArg = func(e.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSERT { msg: e2, cond: e, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = func(e.clone(), extraArg.clone())?;
                    extraArg = func(e2.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TERMINATE { msg: e, .. } => {
                    Ok(func(e.clone(), extraArg.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_REINIT { value: e2, var: e, .. } => {
                    let mut extraArg: ArgT = extraArg.clone();
                    extraArg = func(e.clone(), extraArg.clone())?;
                    extraArg = func(e2.clone(), extraArg.clone())?;
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_NORETCALL { exp: e, .. } => {
                    Ok(func(e.clone(), extraArg.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_RETURN { .. } => {
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_BREAK { .. } => {
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_CONTINUE { .. } => {
                    Ok(extraArg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. } => {
                    Ok(traverseStmts(stmts.clone(), func.clone(), extraArg.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = (DAEDump::ppStatementStr(stmt.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackenddAEUtil.traverseStmts not implemented correctly: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    }
    Ok(extraArg)
}

fn traverseStmtsElse<Type_a: Clone + 'static>(mut inElse: Arc<DAE::Else>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<Type_a> + 'static>, mut iextraArg: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<Type_a> + 'static>;

    let mut oextraArg: Type_a;
    oextraArg = (::match_deref::match_deref! { match &((inElse.clone(), iextraArg.clone())) {
        (Deref @ DAE::Else::NOELSE, extraArg) => {
            extraArg.clone()
        },
        (Deref @ DAE::Else::ELSEIF { exp: e, statementLst: st, else_: el }, extraArg) => {
            let mut extraArg = (*extraArg).clone();
            extraArg = traverseStmtsElse(el.clone(), func.clone(), extraArg.clone())?;
            extraArg = func(e.clone(), extraArg.clone())?;
            traverseStmts(st.clone(), func.clone(), extraArg.clone())?
        },
        (Deref @ DAE::Else::ELSE { statementLst: st }, extraArg) => {
            traverseStmts(st.clone(), func.clone(), extraArg.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oextraArg)
}

pub fn adjacencyMatrixToSparsePattern(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut res: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut lst_1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    lst = Arc::new(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    lst_1 = List::mapList(lst.clone(), Arc::new(fnptr!(intAbs, i32)));
    lst_1 = List::map1(lst_1.clone(), (std::sync::Arc::new(List::sort) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    lst_1 = List::map1(lst_1.clone(), (std::sync::Arc::new(List::sortedUnique) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intGe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    res = metamodelica::arrayFromVec(lst_1.clone().into_iter().cloned().collect());
    res
}

/* *****************************************************************
 stuff to calculate enhanced Adjacency matrix

 The Adjacency matrix describes the relation between knots and
 knots of a bigraph. Additional information about the solvability
 of a variable are available.
******************************************************************/
pub fn getAdjacencyMatrixEnhancedScalar(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut trytosolve: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut outMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMapIncRowEqn: metamodelica::Array<i32>;
    let mut varsSolvedInWhenEqnsTupleList: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    (outAdjacencyMatrix, outAdjacencyMatrixT, outMapEqnIncRow, outMapIncRowEqn) = 'mc: {
        let __mc_input = (syst.clone(), shared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut arr: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
                    let mut arrT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
                    let mut numberOfEqs: i32 = 0;
                    let mut numberofVars: i32 = 0;
                    let mut rowmark: metamodelica::Array<i32>;
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut varsSolvedInWhenEqnsTupleList: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = varsSolvedInWhenEqnsTupleList.clone();
                    numberOfEqs = BackendEquation::getNumberOfEquations(eqns.clone());
                    numberofVars = BackendVariable::varsSize(vars.clone())?;
                    arrT = arrayCreate(numberofVars.clone(), metamodelica::nil());
                    rowmark = arrayCreate(numberofVars.clone(), 0);
                    (arr, arrT, mapEqnIncRow, mapIncRowEqn, varsSolvedInWhenEqnsTupleList) = adjacencyMatrixDispatchEnhancedScalar(vars.clone(), eqns.clone(), arrT.clone(), numberOfEqs.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
                    Ok((arr.clone(), arrT.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAEUtil.getAdjacencyMatrixEnhancedScalar failed")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    makeWhenEqnVarsUnsolvable(outAdjacencyMatrix.clone(), outAdjacencyMatrixT.clone(), varsSolvedInWhenEqnsTupleList.clone())?;
    Ok((outAdjacencyMatrix, outAdjacencyMatrixT, outMapEqnIncRow, outMapIncRowEqn))
}

pub fn makeWhenEqnVarsUnsolvable(mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut varsSolvedInWhenEqnsTupleList: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>) -> Result<()> {
    let mut eqn: i32 = 0;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut adjacencyRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut adjacencyRowT: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    for mut tpl in &*varsSolvedInWhenEqnsTupleList.clone() {
        let mut tpl = tpl.clone();
        (eqn, vars) = tpl.clone();
        for mut var in &*vars.clone() {
            let mut var = var.clone();
            adjacencyRowT = mt.borrow()[(var.clone()-1) as usize].clone();
            eqns = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut adjacencyElemT in (adjacencyRowT.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(adjacencyElemT.clone()) {
        (i, _, _) => {
            i.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (eqns, _) = List::deleteMemberOnTrue(eqn.clone(), eqns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            for mut e in &*eqns.clone() {
                let mut e = e.clone();
                adjacencyRow = m.borrow()[(e.clone()-1) as usize].clone();
                adjacencyRow = ({
        let mut __acc: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
        for mut adjacencyElem in (adjacencyRow.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(adjacencyElem.clone()) {
        (i, _, c) if (intEq(i.clone(), var.clone())) => {
            (i.clone(), crate::BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE, c.clone())
        },
        (_, _, _) => {
            adjacencyElem.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                {let _arr = m.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = adjacencyRow.clone(); _arr};
            }
            adjacencyRowT = mt.borrow()[(var.clone()-1) as usize].clone();
            adjacencyRowT = ({
        let mut __acc: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
        for mut adjacencyElemT in (adjacencyRowT.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(adjacencyElemT.clone()) {
        (i, _, c) if (!(intEq(i.clone(), eqn.clone()))) => {
            (i.clone(), crate::BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE, c.clone())
        },
        (_, _, _) => {
            adjacencyElemT.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            {let _arr = mt.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = adjacencyRowT.clone(); _arr};
        }
    }
    Ok(())
}

fn adjacencyMatrixDispatchEnhancedScalar(mut vars: BackendDAE::Variables, mut eqArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut adjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut numberOfEqs: i32, mut rowmark: metamodelica::Array<i32>, mut globalKnownVars: BackendDAE::Variables, mut trytosolve: bool, mut shared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>)> {
    let mut outAdjacencyArray: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut adjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> = adjacencyArrayT;
    let mut omapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut omapIncRowEqn: metamodelica::Array<i32>;
    let mut varsSolvedInWhenEqnsTupleListOut: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut inAdjacencyArray: Arc<metamodelica::List<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>> = metamodelica::nil();
    let mut mapEqnIncRow: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut imapIncRowEqn: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut rowSize: i32 = 0;
    let mut varsSolvedInWhenEqnsTuple: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut rowindxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i1 in 1..=numberOfEqs.clone() {
        e = BackendEquation::get(eqArr.clone(), i1.clone());
        (row, size, varsSolvedInWhenEqnsTuple) = adjacencyRowEnhanced(vars.clone(), e.clone(), i1.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
        rowindxs = List::intRange2(rowSize.clone() + 1, rowSize.clone() + size.clone());
        rowSize = rowSize.clone() + size.clone();
        imapIncRowEqn = List::consN(size.clone(), i1.clone(), imapIncRowEqn.clone());
        inAdjacencyArray = List::consN(size.clone(), row.clone(), inAdjacencyArray.clone());
        adjacencyArrayT = fillincAdjacencyMatrixTEnhanced(row.clone(), rowindxs.clone(), adjacencyArrayT.clone())?;
        varsSolvedInWhenEqnsTupleListOut = listAppend(varsSolvedInWhenEqnsTuple.clone(), varsSolvedInWhenEqnsTupleListOut.clone());
        mapEqnIncRow = cons(rowindxs.clone(), mapEqnIncRow.clone());
    }
    outAdjacencyArray = List::listArrayReverse(inAdjacencyArray.clone())?;
    omapEqnIncRow = List::listArrayReverse(mapEqnIncRow.clone())?;
    omapIncRowEqn = List::listArrayReverse(imapIncRowEqn.clone())?;
    Ok((outAdjacencyArray, adjacencyArrayT, omapEqnIncRow, omapIncRowEqn, varsSolvedInWhenEqnsTupleListOut))
}

pub fn getAdjacencyMatrixEnhanced(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut trytosolve: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>)> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut outAdjacencyMatrixT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    (outAdjacencyMatrix, outAdjacencyMatrixT) = 'mc: {
        let __mc_input = (syst.clone(), shared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut arr: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
                    let mut arrT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
                    let mut numberOfEqs: i32 = 0;
                    let mut numberofVars: i32 = 0;
                    let mut rowmark: metamodelica::Array<i32>;
                    numberOfEqs = BackendEquation::getNumberOfEquations(eqns.clone());
                    numberofVars = BackendVariable::varsSize(vars.clone())?;
                    arr = arrayCreate(BackendEquation::equationArraySize(eqns.clone())?, metamodelica::nil());
                    arrT = arrayCreate(numberofVars.clone(), metamodelica::nil());
                    rowmark = arrayCreate(numberofVars.clone(), 0);
                    (arr, arrT) = adjacencyMatrixDispatchEnhanced(vars.clone(), eqns.clone(), arr.clone(), arrT.clone(), 0, numberOfEqs.clone(), intLt(0, numberOfEqs.clone()), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
                    Ok((arr.clone(), arrT.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAEUtil.getAdjacencyMatrixEnhanced failed")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAdjacencyMatrix, outAdjacencyMatrixT))
}

fn adjacencyMatrixDispatchEnhanced(mut vars: BackendDAE::Variables, mut eqArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inAdjacencyArray: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut inAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut index: i32, mut numberOfEqs: i32, mut stop: bool, mut rowmark: metamodelica::Array<i32>, mut globalKnownVars: BackendDAE::Variables, mut trytosolve: bool, mut shared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>)> {
    let mut outAdjacencyArray: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut outAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    (outAdjacencyArray, outAdjacencyArrayT) = (match stop.clone() {
        false => {
            (inAdjacencyArray.clone(), inAdjacencyArrayT.clone())
        },
        true => {
            let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
            let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut iArr: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
            let mut iArrT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
            let mut i1: i32 = 0;
            i1 = index.clone() + 1;
            e = BackendEquation::get(eqArr.clone(), i1.clone());
            (row, _, _) = adjacencyRowEnhanced(vars.clone(), e.clone(), i1.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
            iArr = {let _arr = inAdjacencyArray.clone(); _arr.borrow_mut()[(i1.clone()-1) as usize] = row.clone(); _arr};
            iArrT = fillincAdjacencyMatrixTEnhanced(row.clone(), list![i1.clone()], inAdjacencyArrayT.clone())?;
            (iArr, iArrT) = adjacencyMatrixDispatchEnhanced(vars.clone(), eqArr.clone(), iArr.clone(), iArrT.clone(), i1.clone(), numberOfEqs.clone(), intLt(i1.clone(), numberOfEqs.clone()), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
            (iArr.clone(), iArrT.clone())
        },
    });
    Ok((outAdjacencyArray, outAdjacencyArrayT))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fillincAdjacencyMatrixTEnhanced(mut eqns: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut eqnsindxs: Arc<metamodelica::List<i32>>, mut inAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>> {
    let mut outAdjacencyArrayT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    outAdjacencyArrayT = 'mc: {
        let __mc_input = eqns.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inAdjacencyArrayT.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (v, solva, cons), tail: rest } => {
                    if !((intLt(0, v.clone()))) { bail!("guard") }
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut newrow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut mT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
                    row = inAdjacencyArrayT.borrow()[(v.clone()-1) as usize].clone();
                    newrow = List::map2(eqnsindxs.clone(), std::sync::Arc::new(fnptr!(Util::make3Tuple, _, _, _)), solva.clone(), cons.clone());
                    row = listAppend(newrow.clone(), row.clone());
                    mT = {let _arr = inAdjacencyArrayT.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = row.clone(); _arr};
                    Ok(fillincAdjacencyMatrixTEnhanced(rest.clone(), eqnsindxs.clone(), mT.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (v, solva, cons), tail: rest } => {
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut newrow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut vabs: i32 = 0;
                    let mut mT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
                    let mut eqnsindxs1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    vabs = intAbs(v.clone());
                    row = inAdjacencyArrayT.borrow()[(vabs.clone()-1) as usize].clone();
                    eqnsindxs1 = List::map(eqnsindxs.clone(), Arc::new(fnptr!(intNeg, i32)));
                    newrow = List::map2(eqnsindxs1.clone(), std::sync::Arc::new(fnptr!(Util::make3Tuple, _, _, _)), solva.clone(), cons.clone());
                    row = listAppend(newrow.clone(), row.clone());
                    mT = {let _arr = inAdjacencyArrayT.clone(); _arr.borrow_mut()[(vabs.clone()-1) as usize] = row.clone(); _arr};
                    Ok(fillincAdjacencyMatrixTEnhanced(rest.clone(), eqnsindxs.clone(), mT.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAEUtil.fillincAdjacencyMatrixTEnhanced failed")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAdjacencyArrayT)
}

fn adjacencyRowEnhanced(mut inVariables: BackendDAE::Variables, mut inEquation: Arc<BackendDAE::Equation>, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut globalKnownVars: BackendDAE::Variables, mut trytosolve: bool, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, i32, Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>)> {
    let mut outRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut varsSolvedInWhenEqnsTuple: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut isInitial: bool = false;
    isInitial = isInitializationDAE(shared.clone());
    (outRow, size) = 'mc: {
        let __mc_input = (inVariables.clone(), inEquation.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    lst = adjacencyRowExpEnhanced(e1.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), metamodelica::nil())?;
                    lst = adjacencyRowExpEnhanced(e2.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), lst.clone())?;
                    row = adjacencyRowEnhanced1(lst.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), metamodelica::nil(), trytosolve.clone(), 1, shared.clone())?;
                    Ok((row.clone(), 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, size, .. }) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    lst = adjacencyRowExpEnhanced(e1.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), metamodelica::nil())?;
                    lst = adjacencyRowExpEnhanced(e2.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), lst.clone())?;
                    row = adjacencyRowEnhanced1(lst.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), metamodelica::nil(), trytosolve.clone(), size.clone(), shared.clone())?;
                    Ok((row.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, dimSize: ds, .. }) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut size: i32 = size.clone();
                    lst = adjacencyRowExpEnhanced(e1.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), metamodelica::nil())?;
                    lst = adjacencyRowExpEnhanced(e2.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), lst.clone())?;
                    size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1);
                    row = adjacencyRowEnhanced1(lst.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), metamodelica::nil(), trytosolve.clone(), size.clone(), shared.clone())?;
                    Ok((row.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, componentRef: cr, .. }) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut expCref: Arc<DAE::Exp>;
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    expCref = Expression::crefExp(cr.clone())?;
                    lst = adjacencyRowExpEnhanced(expCref.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), metamodelica::nil())?;
                    lst = adjacencyRowExpEnhanced(e.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), lst.clone())?;
                    row = adjacencyRowEnhanced1(lst.clone(), expCref.clone(), e.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), metamodelica::nil(), trytosolve.clone(), 1, shared.clone())?;
                    Ok((row.clone(), 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    lst = adjacencyRowExpEnhanced(e.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), metamodelica::nil())?;
                    row = adjacencyRowEnhanced1(lst.clone(), e.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), metamodelica::nil(), trytosolve.clone(), 1, shared.clone())?;
                    Ok((row.clone(), 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: elsewe, size, .. }) => {
                    let mut varsSolvedInWhenEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut varsSolvedInWhenEqnsTuple: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = varsSolvedInWhenEqnsTuple.clone();
                    (row, varsSolvedInWhenEqns) = adjacencyRowWhenEnhanced(elsewe.clone(), mark.clone(), rowmark.clone(), vars.clone(), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), shared.clone())?;
                    varsSolvedInWhenEqnsTuple = list![(mark.clone(), varsSolvedInWhenEqns.clone())];
                    Ok((row.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::ALGORITHM { expand: crefExpand, source, alg, size, .. }) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut algoutCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    algoutCrefs = CheckModel::checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), crefExpand.clone())?;
                    row = adjacencyRowAlgorithmOutputs(algoutCrefs.clone(), vars.clone(), mark.clone(), rowmark.clone(), metamodelica::nil())?;
                    expl = Algorithm::getAllExps(alg.clone())?;
                    let (_, (_, _, _, __pa0)) = Expression::traverseExpList(expl.clone(), (std::sync::Arc::new(adjacencyRowAlgorithmInputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, i32, metamodelica::Array<i32>, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, i32, metamodelica::Array<i32>, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>))> + 'static>), (vars.clone(), mark.clone(), rowmark.clone(), row.clone()))?;
                    row = __pa0.clone();
                    Ok((row.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: eqnselse, eqnstrue: Deref @ metamodelica::List::Cons { head: eqns, tail: Deref @ metamodelica::List::Nil }, conditions: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut size: i32 = size.clone();
                    if isInitializationDAE(shared.clone()) {
                        (row, size) = adjacencyRowEnhancedEqnLst(eqns.clone(), inVariables.clone(), mark.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
                    } else {
                        (row, size) = adjacencyRowEnhancedEqnLst(eqnselse.clone(), inVariables.clone(), mark.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
                    }
                    Ok((row.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: eqnselse, eqnstrue: Deref @ metamodelica::List::Cons { head: eqns, tail: Deref @ metamodelica::List::Nil }, conditions: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { .. }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut size: i32 = size.clone();
                    if isInitializationDAE(shared.clone()) {
                        (row, size) = adjacencyRowEnhancedEqnLst(eqnselse.clone(), inVariables.clone(), mark.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
                    } else {
                        (row, size) = adjacencyRowEnhancedEqnLst(eqns.clone(), inVariables.clone(), mark.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
                    }
                    Ok((row.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: eqnselse, eqnstrue: eqnslst, conditions: expl, .. }) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lstall: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut row1: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
                    let mut size: i32 = size.clone();
                    lst = List::fold4(expl.clone(), (std::sync::Arc::new(adjacencyRowExpEnhanced) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables, i32, metamodelica::Array<i32>, bool, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), metamodelica::nil());
                    List::fold1(lst.clone(), (std::sync::Arc::new(markNegativ) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<i32> + 'static>), rowmark.clone(), mark.clone());
                    row1 = adjacencyRowEnhanced1(lst.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), metamodelica::nil(), trytosolve.clone(), 1, shared.clone())?;
                    (row, size) = adjacencyRowEnhancedEqnLst(eqnselse.clone(), vars.clone(), mark.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
                    lst = List::map(row.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)));
                    for mut eq in &*eqnslst.clone() {
                        let mut eq = eq.clone();
                        (lst, row, _) = adjacencyRowEnhancedEqnLstIfBranches(eq.clone(), vars.clone(), mark.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone(), (lst.clone(), row.clone(), size.clone()))?;
                    }
                    lstall = List::map(row.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)));
                    (_, lst, _) = List::intersection1OnTrue(lstall.clone(), lst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    List::fold1(lst.clone(), (std::sync::Arc::new(markNegativ) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<i32> + 'static>), rowmark.clone(), mark.clone());
                    row = listAppend(row1.clone(), row.clone());
                    Ok((row.clone(), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqnstr: ArcStr = arcstr::literal!("");
                    eqnstr = (BackendDump::equationString(inEquation.clone())?).clone();
                    eqnstr = stringAppendList(list![(literal!("BackendDAE.adjacencyRowEnhanced failed for eqn:\n")).clone(), (eqnstr.clone()).clone(), (literal!("\n")).clone()]);
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(eqnstr.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRow, size, varsSolvedInWhenEqnsTuple))
}

fn adjacencyRowEnhancedEqnLstIfBranches(mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVariables: BackendDAE::Variables, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut globalKnownVars: BackendDAE::Variables, mut trytosolve: bool, mut shared: Arc<BackendDAE::Shared>, mut intpl: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, i32)> {
    let mut outtpl: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, i32);
    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut iRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inLstAllBranch: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut iSize: i32 = 0;
    (inLstAllBranch, iRow, iSize) = intpl.clone();
    for mut eqn in &*iEqns.clone() {
        let mut eqn = eqn.clone();
        (row, size, _) = adjacencyRowEnhanced(inVariables.clone(), eqn.clone(), mark.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
        lst = List::map(row.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)));
        inLstAllBranch = List::intersectionOnTrue(lst.clone(), inLstAllBranch.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        iSize = iSize.clone() + size.clone();
        iRow = listAppend(row.clone(), iRow.clone());
    }
    outtpl = (inLstAllBranch.clone(), iRow.clone(), iSize.clone());
    Ok(outtpl)
}

fn adjacencyRowEnhancedEqnLst(mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVariables: BackendDAE::Variables, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut globalKnownVars: BackendDAE::Variables, mut trytosolve: bool, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, i32)> {
    let mut outRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut oSize: i32 = 0;
    let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    let mut size: i32 = 0;
    for mut eqn in &*iEqns.clone() {
        let mut eqn = eqn.clone();
        (row, size, _) = adjacencyRowEnhanced(inVariables.clone(), eqn.clone(), mark.clone(), rowmark.clone(), globalKnownVars.clone(), trytosolve.clone(), shared.clone())?;
        outRow = listAppend(row.clone(), outRow.clone());
        oSize = oSize.clone() + size.clone();
    }
    Ok((outRow, oSize))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn adjacencyRowAlgorithmOutputs(mut algOutputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inVariables: BackendDAE::Variables, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut iRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> {
    let mut outRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    outRow = (::match_deref::match_deref! { match &(algOutputs.clone()) {
        Deref @ metamodelica::List::Nil => {
            iRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: cr, tail: rest } => {
            let mut vindx: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
            (_, vindx) = BackendVariable::getVar(cr.clone(), inVariables.clone())?;
            row = adjacencyRowAlgorithmOutputs1(vindx.clone(), mark.clone(), rowmark.clone(), iRow.clone())?;
            adjacencyRowAlgorithmOutputs(rest.clone(), inVariables.clone(), mark.clone(), rowmark.clone(), row.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRow)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn adjacencyRowAlgorithmOutputs1(mut vindx: Arc<metamodelica::List<i32>>, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut iRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> {
    let mut outRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    outRow = (::match_deref::match_deref! { match &(vindx.clone()) {
        Deref @ metamodelica::List::Nil => {
            iRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: i, tail: rest } => {
            {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = mark.clone(); _arr};
            adjacencyRowAlgorithmOutputs1(rest.clone(), mark.clone(), rowmark.clone(), cons((i.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), iRow.clone()))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRow)
}

fn adjacencyRowAlgorithmInputs(mut inExp: Arc<DAE::Exp>, mut iTpl: (BackendDAE::Variables, i32, metamodelica::Array<i32>, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, i32, metamodelica::Array<i32>, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut oTpl: (BackendDAE::Variables, i32, metamodelica::Array<i32>, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>);
    (outExp, oTpl) = 'mc: {
        let __mc_input = (inExp.clone(), iTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, mark, rowmark, row)) => {
                    let mut vindx: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut row = (*row).clone();
                    (_, vindx) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    row = adjacencyRowAlgorithmInputs1(vindx.clone(), mark.clone(), rowmark.clone(), row.clone())?;
                    Ok((e.clone(), (vars.clone(), mark.clone(), rowmark.clone(), row.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), iTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, oTpl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn adjacencyRowAlgorithmInputs1(mut vindx: Arc<metamodelica::List<i32>>, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut iRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> {
    let mut outRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    outRow = (::match_deref::match_deref! { match &(vindx.clone()) {
        Deref @ metamodelica::List::Nil => {
            iRow.clone()
        },
        Deref @ metamodelica::List::Cons { head: i, tail: rest } if (!(intEq(intAbs(rowmark.borrow()[(i.clone()-1) as usize].clone()), mark.clone()))) => {
            {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = -(mark.clone()); _arr};
            adjacencyRowAlgorithmInputs1(rest.clone(), mark.clone(), rowmark.clone(), cons((i.clone(), crate::BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE, metamodelica::nil()), iRow.clone()))?
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            adjacencyRowAlgorithmInputs1(rest.clone(), mark.clone(), rowmark.clone(), iRow.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRow)
}

fn adjacencyRowWhenEnhanced(mut inEquation: Arc<BackendDAE::WhenEquation>, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables, mut iLst: Arc<metamodelica::List<i32>>, mut iRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, Arc<metamodelica::List<i32>>)> {
    let mut outRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = iRow.clone();
    let mut varsSolvedInWhenEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut condition: Arc<DAE::Exp>;
    let mut whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut oelsepart: Option<Arc<BackendDAE::WhenEquation>> = None;
    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut elsepart: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
    let mut isInitial: bool = false;
    isInitial = isInitializationDAE(shared.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: __pa0, whenStmtLst: __pa1, condition: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    oelsepart = __pa0.clone();
    whenStmtLst = __pa1.clone();
    condition = __pa2.clone();
    lst = adjacencyRowExpEnhanced(condition.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), iLst.clone())?;
    for mut rs in &*whenStmtLst.clone() {
        let mut rs = rs.clone();
        let () = (::match_deref::match_deref! { match &(rs.clone()) {
        BackendDAE::WhenOperator::ASSIGN { right, left: leftexp @ Deref @ DAE::Exp::CREF { componentRef: left, .. }, .. } => {
            let mut varIndx: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(left.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIndx = __pa0.clone();
            varsSolvedInWhenEqns = cons(varIndx.clone(), varsSolvedInWhenEqns.clone());
            lst = adjacencyRowExpEnhanced(right.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), lst.clone())?;
            List::fold1(lst.clone(), (std::sync::Arc::new(markNegativ) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<i32> + 'static>), rowmark.clone(), mark.clone());
            lst = adjacencyRowExpEnhanced(leftexp.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), lst.clone())?;
            outRow = adjacencyRowEnhanced1(lst.clone(), leftexp.clone(), right.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), outRow.clone(), false, 1, shared.clone())?;
            ()
        },
        BackendDAE::WhenOperator::ASSIGN { left: leftexp, right, .. } => {
            let mut varIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crefs = Expression::getAllCrefs(leftexp.clone())?;
            (_, varIdcs) = BackendVariable::getVarLst(crefs.clone(), vars.clone());
            varsSolvedInWhenEqns = listAppend(varIdcs.clone(), varsSolvedInWhenEqns.clone());
            lst = adjacencyRowExpEnhanced(right.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), lst.clone())?;
            List::fold1(lst.clone(), (std::sync::Arc::new(markNegativ) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<i32> + 'static>), rowmark.clone(), mark.clone());
            lst = adjacencyRowExpEnhanced(leftexp.clone(), vars.clone(), mark.clone(), rowmark.clone(), isInitial.clone(), lst.clone())?;
            outRow = adjacencyRowEnhanced1(lst.clone(), leftexp.clone(), right.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), outRow.clone(), false, 1, shared.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if isSome(oelsepart.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(oelsepart.clone()) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        elsepart = __pa3.clone();
        (outRow, _) = adjacencyRowWhenEnhanced(elsepart.clone(), mark.clone(), rowmark.clone(), vars.clone(), globalKnownVars.clone(), lst.clone(), outRow.clone(), shared.clone())?;
    }
    Ok((outRow, varsSolvedInWhenEqns))
}

fn markNegativ(mut indx: i32, mut rowmark: metamodelica::Array<i32>, mut mark: i32) -> Result<i32> {
    let mut oMark: i32 = 0;
    {let _arr = rowmark.clone(); _arr.borrow_mut()[(indx.clone()-1) as usize] = -(mark.clone()); _arr};
    oMark = mark.clone();
    Ok(oMark)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn adjacencyRowEnhanced1(mut lst: Arc<metamodelica::List<i32>>, mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut inRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut trytosolve: bool, mut size: i32, mut shared: Arc<BackendDAE::Shared>) -> Result<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> {
    let mut outRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>> = metamodelica::nil();
    outRow = 'mc: {
        let __mc_input = (lst.clone(), e1.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(inRow.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
                    if !((intGt(r.clone(), 0))) { bail!("guard") }
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let false = (intEq(rowmark.borrow()[(r.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varKind: BackendDAE::STATE { .. }, varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), r.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasDerCref(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varKind: BackendDAE::STATE { .. }, varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasDerCref(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crarr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    crarr = ComponentReferenceBasics::crefStripLastSubs(cr1.clone())?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), crarr.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::LUNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::NOT { ty: _ } }, _) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::UMINUS { ty: _ } }, _) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::UMINUS_ARR { ty: _ } }, _) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crarr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    crarr = ComponentReferenceBasics::crefStripLastSubs(cr1.clone())?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), crarr.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crarr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    crarr = ComponentReferenceBasics::crefStripLastSubs(cr1.clone())?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), crarr.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::LUNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::NOT { ty: _ } }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::UMINUS { ty: _ } }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::UMINUS_ARR { ty: _ } }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crarr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    crarr = ComponentReferenceBasics::crefStripLastSubs(cr1.clone())?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), crarr.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefPrefixOf(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (ComponentReferenceBasics::crefPrefixOf(cr.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path1 }, .. }, .. }, expLst: explst, path }, _) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let true = (AbsynUtil::pathEqual(path.clone(), path1.clone())) else { bail!("pattern mismatch") };
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (expCrefLstHasCref(explst.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path1 }, .. }, .. }, expLst: explst, path }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let true = (AbsynUtil::pathEqual(path.clone(), path1.clone())) else { bail!("pattern mismatch") };
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (expCrefLstHasCref(explst.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e1.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::RECORD { exps: explst, .. }, _) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (expCrefLstHasCref(explst.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, Deref @ DAE::Exp::RECORD { exps: explst, .. }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    let true = (expCrefLstHasCref(explst.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e1.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, Deref @ DAE::Exp::TUPLE { PR: explst }, Deref @ DAE::Exp::CALL { .. }) => {
                    let mut rabs: i32 = 0;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crexplst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut explst = (*explst).clone();
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr1 = __pa0.clone();
                    explst = List::flatten(List::map1(explst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpLstFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), None));
                    crlst = List::map(explst.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    crlst = List::flatten(List::map1(crlst.clone(), (std::sync::Arc::new(ComponentReference::expandCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), true));
                    crexplst = List::map(crlst.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                    let true = (expCrefLstHasCref(crexplst.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefNoPreorDer(e2.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_SOLVED, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _) => {
                    let mut de: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut e_derAlias: Arc<DAE::Exp>;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut solvab: BackendDAE::Solvability = BackendDAE::Solvability::SOLVABILITY_CONSTONE;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut solved: bool = false;
                    let mut derived: bool = false;
                    let mut cons: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
                    let 1 = (size.clone()) else { bail!("pattern mismatch") };
                    let true = (intGt(r.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intEq(rowmark.borrow()[(r.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varKind: BackendDAE::STATE { .. }, varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), r.clone())?) else { bail!("pattern mismatch") };
                    cr = __pa0.clone();
                    cr1 = ComponentReference::crefPrefixDer(cr.clone());
                    e = Expression::crefExp(cr.clone())?;
                    (e, _) = Expression::replaceExp(Expression::expSub(e1.clone(), e2.clone())?, Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![e.clone()], attr: DAE::callAttrBuiltinReal().clone() }), Expression::crefExp(cr1.clone())?)?;
                    e_derAlias = Expression::traverseExpDummy(e.clone(), (std::sync::Arc::new(replaceDerCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    (de, solved, derived, cons) = tryToSolveOrDerive(e_derAlias.clone(), cr1.clone(), vars.clone(), Some(shared.functionTree.clone()), trytosolve.clone())?;
                    if !(solved.clone()) {
                        (de, _) = ExpressionSimplify::simplify(de.clone())?;
                        (_, crlst) = Expression::traverseExpBottomUp(de.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinder, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
                        solvab = adjacencyRowEnhanced2(cr1.clone(), de.clone(), crlst.clone(), vars.clone(), globalKnownVars.clone())?;
                    } else {
                        if derived.clone() {
                            (de, _) = ExpressionSimplify::simplify(de.clone())?;
                            (_, crlst) = Expression::traverseExpBottomUp(de.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinder, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
                            solvab = adjacencyRowEnhanced2(cr1.clone(), de.clone(), crlst.clone(), vars.clone(), globalKnownVars.clone())?;
                            solvab = transformSolvabilityForCasualTearingSet(solvab.clone());
                        } else {
                            solvab = crate::BackendDAE::Solvability::SOLVABILITY_SOLVABLE;
                        }
                    }
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), solvab.clone(), cons.clone()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _) => {
                    let mut rabs: i32 = 0;
                    let mut de: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut e_derAlias: Arc<DAE::Exp>;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut solvab: BackendDAE::Solvability = BackendDAE::Solvability::SOLVABILITY_CONSTONE;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut solved: bool = false;
                    let mut derived: bool = false;
                    let mut cons: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
                    let 1 = (size.clone()) else { bail!("pattern mismatch") };
                    rabs = intAbs(r.clone());
                    let false = (intEq(rowmark.borrow()[(rabs.clone()-1) as usize].clone(), -(mark.clone()))) else { bail!("pattern mismatch") };
                    let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), rabs.clone())?) else { bail!("pattern mismatch") };
                    cr = __pa0.clone();
                    if CommonSubExpression::isCSECref(cr.clone()) {
                        solvab = crate::BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE;
                        cons = metamodelica::nil();
                    } else {
                        e = Expression::expSub(e1.clone(), e2.clone())?;
                        e_derAlias = Expression::traverseExpDummy(e.clone(), (std::sync::Arc::new(replaceDerCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                        (de, solved, derived, cons) = tryToSolveOrDerive(e_derAlias.clone(), cr.clone(), vars.clone(), Some(shared.functionTree.clone()), trytosolve.clone())?;
                        if !(solved.clone()) {
                            (de, _) = ExpressionSimplify::simplify(de.clone())?;
                            (_, crlst) = Expression::traverseExpTopDown(de.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
                            solvab = adjacencyRowEnhanced2(cr.clone(), de.clone(), crlst.clone(), vars.clone(), globalKnownVars.clone())?;
                        } else {
                            if derived.clone() {
                                        (de, _) = ExpressionSimplify::simplify(de.clone())?;
                                        (_, crlst) = Expression::traverseExpTopDown(de.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
                                        solvab = adjacencyRowEnhanced2(cr.clone(), de.clone(), crlst.clone(), vars.clone(), globalKnownVars.clone())?;
                                        solvab = transformSolvabilityForCasualTearingSet(solvab.clone());
                            } else {
                                        solvab = crate::BackendDAE::Solvability::SOLVABILITY_SOLVABLE;
                            }
                        }
                    }
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), solvab.clone(), cons.clone()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _) => {
                    Ok(adjacencyRowEnhanced1(rest.clone(), e1.clone(), e2.clone(), vars.clone(), globalKnownVars.clone(), mark.clone(), rowmark.clone(), cons((r.clone(), crate::BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE, metamodelica::nil()), inRow.clone()), trytosolve.clone(), size.clone(), shared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRow)
}

fn replaceDerCall(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr = (*cr).clone();
                    let mut outExp: Arc<DAE::Exp>;
                    v = BackendVariable::createAliasDerVar(cr.clone())?;
                    cr = BackendVariable::varCref(v.clone())?;
                    outExp = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() });
                    Ok(outExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEUtil.replaceDerCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn tryToSolveOrDerive(mut e: Arc<DAE::Exp>, mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables, mut functions: Option<Arc<AvlTreePathFunction::Tree>>, mut trytosolve1: bool) -> Result<(Arc<DAE::Exp>, bool, bool, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> {
    let mut f: Arc<DAE::Exp>;
    let mut solved: bool = false;
    let mut derived: bool = false;
    let mut outCons: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    let mut tp: Arc<DAE::Type> = Expression::r#typeof(e.clone())?;
    let mut trytosolve2: bool = stringEqual((Flags::getConfigString(Flags::TEARING_STRICTNESS.clone())?).clone(), (literal!("casual")).clone());
    let mut localCon: bool = false;
    let mut one: Arc<DAE::Exp>;
    let mut tmpEqn: Arc<DAE::Exp>;
    let mut solvedExp: Arc<DAE::Exp>;
    let mut con: Arc<DAE::Exp>;
    let mut eqnForNewVars: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVarsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut tmpVar: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut constraint: Arc<DAE::Constraint>;
    let mut constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    let debug: bool = false;
    if trytosolve1.clone() || trytosolve2.clone() {
        if '__try0: {
            (solvedExp, _, eqnForNewVars, newVarsCrefs) = unwrap_break_err!(ExpressionSolve::solve2(e.clone(), Expression::makeConstZero(tp.clone()), Expression::crefExp(cr.clone())?, functions.clone(), Some(1), true, false), '__try0);
            if debug.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Solve expression:\n")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("for variable: ")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(Expression::crefExp(cr.clone())?), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Solved expression:\n")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(solvedExp.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                ComponentReference::printComponentRefList(newVarsCrefs.clone());
                BackendDump::dumpEquationList(eqnForNewVars.clone(), (literal!("eqnForNewVars")).clone());
                unwrap_break_err!(ExpressionDump::dumpExp(solvedExp.clone()), '__try0);
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("listLength(eqnForNewVars): ")); __mm_s.push_str(&*intString((eqnForNewVars.clone().len() as i32))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            if trytosolve1.clone() {
                let (_, (__pa1, _)) = unwrap_break_err!(Expression::traverseExpTopDown(solvedExp.clone(), (std::sync::Arc::new(getConstraints) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::Constraint>>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::Constraint>>>, BackendDAE::Variables))> + 'static>), (metamodelica::nil(), vars.clone())), '__try0);
                constraints = __pa1.clone();
                for mut eqn in &*eqnForNewVars.clone() {
                    let mut eqn = eqn.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: __pa2, componentRef: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
                        _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    tmpEqn = __pa2.clone();
                    tmpVar = __pa3.clone();
                    let (_, (__pa4, _)) = unwrap_break_err!(Expression::traverseExpTopDown(tmpEqn.clone(), (std::sync::Arc::new(getConstraints) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::Constraint>>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::Constraint>>>, BackendDAE::Variables))> + 'static>), (constraints.clone(), vars.clone())), '__try0);
                    constraints = __pa4.clone();
                }
                for mut i in (1..=(constraints.clone().len() as i32)).rev() {
                    constraint = unwrap_break_err!((constraints.clone()).get(i.clone()), '__try0);
                    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(constraint.clone()) {
                        Deref @ DAE::Constraint::CONSTRAINT_DT { localCon: __pa5, constraint: __pa6 } => (__pa5.clone(), __pa6.clone()),
                        _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    localCon = __pa5.clone();
                    con = __pa6.clone();
                    for mut eqn in &*eqnForNewVars.clone() {
                        let mut eqn = eqn.clone();
                        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(eqn.clone()) {
                            Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: __pa7, componentRef: __pa8, .. } => (__pa7.clone(), __pa8.clone()),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        tmpEqn = __pa7.clone();
                        tmpVar = __pa8.clone();
                        con = unwrap_break_err!(Expression::replaceCrefBottomUp(con.clone(), tmpVar.clone(), tmpEqn.clone()), '__try0);
                    }
                    outCons = cons(Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: con.clone(), localCon: localCon.clone() }), outCons.clone());
                }
                if debug.clone() {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Constraints before substitution: ")); __mm_s.push_str(&*unwrap_break_err!(ExpressionDump::constraintDTlistToString(constraints.clone(), (literal!("\n")).clone()), '__try0)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Substituted expression:\n")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(solvedExp.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Constraints:")); __mm_s.push_str(&*unwrap_break_err!(ExpressionDump::constraintDTlistToString(outCons.clone(), (literal!("\n")).clone()), '__try0)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            solved = true;
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    match '__try9: {
        f = unwrap_break_err!(Differentiate::differentiateExpSolve(e.clone(), cr.clone(), functions.clone()), '__try9);
        f = (::match_deref::match_deref! { match &(f.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: one, operator: DAE::Operator::DIV { .. }, exp2: Deref @ DAE::Exp::CREF { .. } } if (unwrap_break_err!(Expression::isConst(one.clone()), '__try9) && !(Expression::isZero(one.clone()))) => one.clone(),
        _ => f.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        derived = true;
        Ok::<_, anyhow::Error>((f.clone(),))
    } {
        Ok((__try9_o0,)) => {
            f = __try9_o0;
        }
        Err(_) => {
            f = Expression::makeConstOne(tp.clone());
        }
    }
    if Expression::isZero(f.clone()) {
        bail!("fail");
    }
    let true = (solved.clone() || derived.clone()) else { bail!("pattern mismatch") };
    if debug.clone() {
        if solved.clone() {
            println!("{}", (literal!("[SOLVED] ")).clone());
        } else if derived.clone() {
            println!("{}", (literal!("[DERIVED] ")).clone());
        } else {
            println!("{}", (literal!("[?BROKEN?] ")).clone());
        }
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tryToSolveOrDerive ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(f.clone())?); __mm_s.push_str(&*literal!(" == ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(Expression::crefExp(cr.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((f, solved, derived, outCons))
}

pub fn isSolvable(mut solvability: BackendDAE::Solvability, mut strict: bool) -> bool {
    let mut solvable: bool = false;
    solvable = (match solvability.clone() {
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR => {
            false
        },
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE => {
            false
        },
        BackendDAE::Solvability::SOLVABILITY_CONST { b: mut b } => {
            b.clone()
        },
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: mut b } => {
            b.clone()
        },
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: mut b } => {
            b.clone() && !(strict.clone())
        },
        _ => {
            true
        },
    });
    solvable
}

fn getConstraints(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<DAE::Constraint>>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::Constraint>>>, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::Constraint>>>, BackendDAE::Variables);
    let mut inCons: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    (inCons, vars) = inTpl.clone();
    (outExp, outTpl) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp2: e, operator: DAE::Operator::DIV { .. }, .. } => {
            let mut rel: Arc<DAE::Exp>;
            let mut con: Arc<DAE::Constraint>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut localCon: bool = false;
            rel = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("abs")).clone() }), expLst: list![e.clone()], attr: DAE::callAttrBuiltinOther().clone() }), operator: DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1e-12_f64) }), index: -1, optionExpisASUB: None });
            (_, crlst) = Expression::traverseExpTopDown(rel.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            localCon = containAnyVarWithoutStates(crlst.clone(), vars.clone())?;
            con = Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: rel.clone(), localCon: localCon.clone() });
            (inExp.clone(), (cons(con.clone(), inCons.clone()), vars.clone()))
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } => {
            let mut rel: Arc<DAE::Exp>;
            let mut con: Arc<DAE::Constraint>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut localCon: bool = false;
            rel = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATEREQ { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            (_, crlst) = Expression::traverseExpTopDown(rel.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            localCon = containAnyVarWithoutStates(crlst.clone(), vars.clone())?;
            con = Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: rel.clone(), localCon: localCon.clone() });
            (inExp.clone(), (cons(con.clone(), inCons.clone()), vars.clone()))
        },
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::RCONST { real: __rlit_0 }, operator: DAE::Operator::POW { .. }, exp1: e } if __rlit_0.eq(&metamodelica::OrderedFloat((0.5) as f64)) => {
            let mut rel: Arc<DAE::Exp>;
            let mut con: Arc<DAE::Constraint>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut localCon: bool = false;
            rel = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATEREQ { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None });
            (_, crlst) = Expression::traverseExpTopDown(rel.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            localCon = containAnyVarWithoutStates(crlst.clone(), vars.clone())?;
            con = Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: rel.clone(), localCon: localCon.clone() });
            (inExp.clone(), (cons(con.clone(), inCons.clone()), vars.clone()))
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } => {
            let mut rel: Arc<DAE::Exp>;
            let mut con: Arc<DAE::Constraint>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut localCon: bool = false;
            rel = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1e-12_f64) }), index: -1, optionExpisASUB: None });
            (_, crlst) = Expression::traverseExpTopDown(rel.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            localCon = containAnyVarWithoutStates(crlst.clone(), vars.clone())?;
            con = Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: rel.clone(), localCon: localCon.clone() });
            (inExp.clone(), (cons(con.clone(), inCons.clone()), vars.clone()))
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. } => {
            let mut rel: Arc<DAE::Exp>;
            let mut con: Arc<DAE::Constraint>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut localCon: bool = false;
            rel = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1e-12_f64) }), index: -1, optionExpisASUB: None });
            (_, crlst) = Expression::traverseExpTopDown(rel.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            localCon = containAnyVarWithoutStates(crlst.clone(), vars.clone())?;
            con = Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: rel.clone(), localCon: localCon.clone() });
            (inExp.clone(), (cons(con.clone(), inCons.clone()), vars.clone()))
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, .. } => {
            let mut rel: Arc<DAE::Exp>;
            let mut con: Arc<DAE::Constraint>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut localCon: bool = false;
            rel = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("abs")).clone() }), expLst: list![e.clone()], attr: DAE::callAttrBuiltinOther().clone() }), operator: DAE::Operator::LESSEQ { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), index: -1, optionExpisASUB: None });
            (_, crlst) = Expression::traverseExpTopDown(rel.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            localCon = containAnyVarWithoutStates(crlst.clone(), vars.clone())?;
            con = Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: rel.clone(), localCon: localCon.clone() });
            (inExp.clone(), (cons(con.clone(), inCons.clone()), vars.clone()))
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, .. } => {
            let mut rel: Arc<DAE::Exp>;
            let mut con: Arc<DAE::Constraint>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut localCon: bool = false;
            rel = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("abs")).clone() }), expLst: list![e.clone()], attr: DAE::callAttrBuiltinOther().clone() }), operator: DAE::Operator::LESSEQ { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), index: -1, optionExpisASUB: None });
            (_, crlst) = Expression::traverseExpTopDown(rel.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            localCon = containAnyVarWithoutStates(crlst.clone(), vars.clone())?;
            con = Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: rel.clone(), localCon: localCon.clone() });
            (inExp.clone(), (cons(con.clone(), inCons.clone()), vars.clone()))
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, .. } => {
            let mut rel: Arc<DAE::Exp>;
            let mut con: Arc<DAE::Constraint>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut localCon: bool = false;
            rel = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::BINARY { exp2: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("floor")).clone() }), expLst: list![Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("abs")).clone() }), expLst: list![Arc::new(DAE::Exp::BINARY { exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp1: Arc::new(DAE::Exp::BINARY { exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(3.14159265358979_f64) }), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp1: e.clone() }) })], attr: DAE::callAttrBuiltinOther().clone() })], attr: DAE::callAttrBuiltinOther().clone() }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp1: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("abs")).clone() }), expLst: list![Arc::new(DAE::Exp::BINARY { exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp1: Arc::new(DAE::Exp::BINARY { exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(3.14159265358979_f64) }), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp1: e.clone() }) })], attr: DAE::callAttrBuiltinOther().clone() }) }), operator: DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1e-12_f64) }), index: -1, optionExpisASUB: None });
            (_, crlst) = Expression::traverseExpTopDown(rel.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinderNoPreDer, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
            localCon = containAnyVarWithoutStates(crlst.clone(), vars.clone())?;
            con = Arc::new(DAE::Constraint::CONSTRAINT_DT { constraint: rel.clone(), localCon: localCon.clone() });
            (inExp.clone(), (cons(con.clone(), inCons.clone()), vars.clone()))
        },
        _ => {
            (inExp.clone(), inTpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn getEqnAndVarsFromInnerEquation(mut innerEquation: BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> {
    let mut outEqn: i32 = 0;
    let mut outVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outCons: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    (outEqn, outVars, outCons) = (match innerEquation.clone() {
        BackendDAE::InnerEquation::INNEREQUATION { vars: mut vars, eqn: mut eqn } => {
            (eqn.clone(), vars.clone(), metamodelica::nil())
        },
        BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS { cons: mut cons, vars: mut vars, eqn: mut eqn } => {
            (eqn.clone(), vars.clone(), cons.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outEqn, outVars, outCons))
}

pub fn getEqnAndVarsFromInnerEquationLst(mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Constraint>>>>>)> {
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut allConstraints: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Constraint>>>>> = metamodelica::nil();
    let mut eqn: i32 = 0;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>> = metamodelica::nil();
    for mut innerEq in &*innerEquations.clone() {
        let mut innerEq = innerEq.clone();
        (eqn, vars, constraints) = getEqnAndVarsFromInnerEquation(innerEq.clone())?;
        eqns = cons(eqn.clone(), eqns.clone());
        if true /* isPresent not implemented in Rust */ {
            allVars = cons(vars.clone(), allVars.clone());
        }
        if true /* isPresent not implemented in Rust */ {
            allConstraints = cons(constraints.clone(), allConstraints.clone());
        }
    }
    Ok((eqns, allVars, allConstraints))
}

fn transformSolvabilityForCasualTearingSet(mut inSolvab: BackendDAE::Solvability) -> BackendDAE::Solvability {
    let mut outSolvab: BackendDAE::Solvability = BackendDAE::Solvability::SOLVABILITY_CONSTONE;
    outSolvab = (match inSolvab.clone() {
        BackendDAE::Solvability::SOLVABILITY_CONST { b: false } => BackendDAE::Solvability::SOLVABILITY_CONST { b: false },
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false } => BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false },
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: false } => BackendDAE::Solvability::SOLVABILITY_LINEAR { b: false },
        _ => crate::BackendDAE::Solvability::SOLVABILITY_SOLVABLE,
    });
    outSolvab
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn expCrefLstHasCref(mut iExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut outB: bool = false;
    outB = 'mc: {
        let __mc_input = iExpLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: rest } => {
                    let mut b: bool = false;
                    b = ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), inCr.clone())?;
                    b = if (!(b.clone())) {expCrefLstHasCref(rest.clone(), inCr.clone())?} else {b.clone()};
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outB)
}

fn adjacencyRowEnhanced2(mut cr: Arc<DAE::ComponentRef>, mut e: Arc<DAE::Exp>, mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<BackendDAE::Solvability> {
    let mut oSolvab: BackendDAE::Solvability = BackendDAE::Solvability::SOLVABILITY_CONSTONE;
    oSolvab = (::match_deref::match_deref! { match &(crlst.clone()) {
        Deref @ metamodelica::List::Nil => {
            let mut b1: bool = false;
            let mut b2: bool = false;
            b1 = Expression::isZeroOrAlmostZero(e.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }));
            b2 = Expression::isConstOne(e.clone()) || Expression::isConstMinusOne(e.clone());
            if (b2.clone()) {crate::BackendDAE::Solvability::SOLVABILITY_CONSTONE} else {BackendDAE::Solvability::SOLVABILITY_CONST { b: !(b1.clone()) }}
        },
        _ if (List::isMemberOnTrue(cr.clone(), crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))) => {
            crate::BackendDAE::Solvability::SOLVABILITY_NONLINEAR
        },
        _ => {
            let mut b1: bool = false;
            let mut b2: bool = false;
            b1 = containAnyVar(crlst.clone(), globalKnownVars.clone())?;
            b2 = containAnyVar(crlst.clone(), vars.clone())?;
            adjacencyRowEnhanced3(b1.clone(), b2.clone(), cr.clone(), e.clone(), crlst.clone(), vars.clone(), globalKnownVars.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSolvab)
}

fn adjacencyRowEnhanced3(mut b1: bool, mut b2: bool, mut cr: Arc<DAE::ComponentRef>, mut e: Arc<DAE::Exp>, mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<BackendDAE::Solvability> {
    let mut oSolvab: BackendDAE::Solvability = BackendDAE::Solvability::SOLVABILITY_CONSTONE;
    oSolvab = (match (b1.clone(), b2.clone()) {
        (true, true) => {
            let mut b: bool = false;
            let mut e1: Arc<DAE::Exp>;
            (e1, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(replaceVarWithValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), globalKnownVars.clone())?;
            (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
            b = !(Expression::isZeroOrAlmostZero(e1.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })));
            BackendDAE::Solvability::SOLVABILITY_LINEAR { b: b.clone() }
        },
        (false, _) => {
            let mut b: bool = false;
            b = !(Expression::isZeroOrAlmostZero(e.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })));
            BackendDAE::Solvability::SOLVABILITY_LINEAR { b: b.clone() }
        },
        (true, _) => {
            let mut b: bool = false;
            let mut b_1: bool = false;
            let mut e1: Arc<DAE::Exp>;
            let mut nominal: Arc<DAE::Exp>;
            (nominal, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(replaceVarWithNominal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), globalKnownVars.clone())?;
            (nominal, _) = ExpressionSimplify::simplify(nominal.clone())?;
            (e1, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(replaceVarWithValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), globalKnownVars.clone())?;
            (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
            b = !(Expression::isZeroOrAlmostZero(e1.clone(), nominal.clone()));
            b_1 = Expression::isConst(e1.clone())? || BackendVariable::isKnownAndParam(e1.clone(), globalKnownVars.clone())?;
            if (b_1.clone()) {BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: b.clone() }} else {BackendDAE::Solvability::SOLVABILITY_LINEAR { b: b.clone() }}
        },
        (_, _) => {
            let mut b: bool = false;
            b = !(Expression::isZeroOrAlmostZero(e.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })));
            BackendDAE::Solvability::SOLVABILITY_LINEAR { b: b.clone() }
        },
    });
    Ok(oSolvab)
}

fn replaceVarWithValue(mut inExp: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    (outExp, outVars) = 'mc: {
        let __mc_input = (inExp.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut e: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    e = BackendVariable::varBindExp(v.clone())?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(replaceVarWithValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), vars.clone())?;
                    Ok((e.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut e: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    let true = (BackendVariable::varFixed(v.clone())) else { bail!("pattern mismatch") };
                    e = BackendVariable::varBindExpStartValue(v.clone())?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(replaceVarWithValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), vars.clone())?;
                    Ok((e.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVars))
}

fn replaceVarWithNominal(mut inExp: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    (outExp, outVars) = 'mc: {
        let __mc_input = (inExp.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut nom: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    nom = BackendVariable::getVarNominalValue(v.clone());
                    (nom, _) = Expression::traverseExpBottomUp(nom.clone(), (std::sync::Arc::new(replaceVarWithNominal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), vars.clone())?;
                    Ok((nom.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVars))
}

fn adjacencyRowExpEnhanced(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut isInitial: bool, mut inRow: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (_, (_, _, _, _, _, __pa0)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (inVariables.clone(), false, isInitial.clone(), mark.clone(), rowmark.clone(), inRow.clone()))?;
    outRow = __pa0.clone();
    Ok(outRow)
}

fn traversingAdjacencyRowExpSolvableEnhancedFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { exp: e1, .. }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut vars = (*vars).clone();
                    let mut pa = (*pa).clone();
                    let (_, (__pa0, _, _, _, _, __pa1)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa0.clone();
                    pa = __pa1.clone();
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp2: e2, exp1: e1, .. }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut vars = (*vars).clone();
                    let mut pa = (*pa).clone();
                    let (_, (__pa0, _, _, _, _, __pa1)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa0.clone();
                    pa = __pa1.clone();
                    let (_, (__pa2, _, _, _, _, __pa3)) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa2.clone();
                    pa = __pa3.clone();
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp2: e2, exp1: e1, .. }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut vars = (*vars).clone();
                    let mut pa = (*pa).clone();
                    let (_, (__pa0, _, _, _, _, __pa1)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa0.clone();
                    pa = __pa1.clone();
                    let (_, (__pa2, _, _, _, _, __pa3)) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa2.clone();
                    pa = __pa3.clone();
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, (vars, bs, isInitial, it, at, pa)) => {
                    Ok(traversingAdjacencyRowIfExpEnhanced(inExp.clone(), (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone()), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { stop: e2, step: None, start: e1, .. }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut vars = (*vars).clone();
                    let mut pa = (*pa).clone();
                    let (_, (__pa0, _, _, _, _, __pa1)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa0.clone();
                    pa = __pa1.clone();
                    let (_, (__pa2, _, _, _, _, __pa3)) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa2.clone();
                    pa = __pa3.clone();
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { stop: e2, step: Some(e3), start: e1, .. }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut vars = (*vars).clone();
                    let mut pa = (*pa).clone();
                    let (_, (__pa0, _, _, _, _, __pa1)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa0.clone();
                    pa = __pa1.clone();
                    let (_, (__pa2, _, _, _, _, __pa3)) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa2.clone();
                    pa = __pa3.clone();
                    let (_, (__pa4, _, _, _, _, __pa5)) = Expression::traverseExpTopDown(e3.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), true, isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa4.clone();
                    pa = __pa5.clone();
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: Deref @ metamodelica::List::Nil }, exp: e1 }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut e1 = (*e1).clone();
                    let mut vars = (*vars).clone();
                    let mut pa = (*pa).clone();
                    e1 = Expression::nthArrayExp(e1.clone(), i.clone())?;
                    let (_, (__pa0, _, _, _, _, __pa1)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(traversingAdjacencyRowExpSolvableEnhancedFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>))> + 'static>), (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone()))?;
                    vars = __pa0.clone();
                    pa = __pa1.clone();
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), pa.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { .. }, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut mark: i32 = 0;
                    let mut rowmark: metamodelica::Array<i32>;
                    mark = it.clone();
                    rowmark = at.clone();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExpEnhanced1(varslst.clone(), p.clone(), pa.clone(), true, mark.clone(), rowmark.clone(), bs.clone())?;
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut mark: i32 = 0;
                    let mut rowmark: metamodelica::Array<i32>;
                    mark = it.clone();
                    rowmark = at.clone();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExpEnhanced1(varslst.clone(), p.clone(), pa.clone(), false, mark.clone(), rowmark.clone(), bs.clone())?;
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: _ }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, bs, isInitial, it, at, pa)) => {
                    let mut p: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varslst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut mark: i32 = 0;
                    let mut rowmark: metamodelica::Array<i32>;
                    mark = it.clone();
                    rowmark = at.clone();
                    (varslst, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    res = adjacencyRowExpEnhanced1(varslst.clone(), p.clone(), pa.clone(), false, mark.clone(), rowmark.clone(), bs.clone())?;
                    Ok((inExp.clone(), false, (vars.clone(), bs.clone(), isInitial.clone(), it.clone(), at.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, _) => {
                    let mut b: bool = false;
                    b = Flags::getConfigBool(Flags::DELAY_BREAK_LOOP.clone())? && ExpressionBasics::expEqual(e1.clone(), e2.clone())?;
                    Ok((inExp.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, (_, _, false, _, _, _)) => {
                    Ok(traversingAdjacencyRowExpSolvableEnhancedFinder(e1.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, (_, _, true, _, _, _)) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut tpl: (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>);
                    (_, b1, tpl) = traversingAdjacencyRowExpSolvableEnhancedFinder(e1.clone(), inTpl.clone())?;
                    (_, b2, tpl) = traversingAdjacencyRowExpSolvableEnhancedFinder(e2.clone(), tpl.clone())?;
                    Ok((inExp.clone(), b1.clone() && b2.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, .. }, _) => {
                    let mut tpl: (BackendDAE::Variables, bool, bool, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>);
                    (_, _, tpl) = traversingAdjacencyRowExpSolvableEnhancedFinder(e2.clone(), inTpl.clone())?;
                    Ok(traversingAdjacencyRowExpSolvableEnhancedFinder(e1.clone(), tpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

fn markBranchVars(mut inExp: Arc<DAE::Exp>, mut inTuple: (i32, metamodelica::Array<i32>, BackendDAE::Variables, Arc<BinaryTree::BinTree>)) -> Result<(Arc<DAE::Exp>, bool, (i32, metamodelica::Array<i32>, BackendDAE::Variables, Arc<BinaryTree::BinTree>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTuple: (i32, metamodelica::Array<i32>, BackendDAE::Variables, Arc<BinaryTree::BinTree>);
    (outExp, cont, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (mark, rowmark, vars, bt)) => {
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (backendVars, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    markBranchVars1(backendVars.clone(), ilst.clone(), mark.clone(), rowmark.clone(), bt.clone())?;
                    Ok((inExp.clone(), true, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTuple))
}

fn markBranchVars1(mut varlst: Arc<metamodelica::List<BackendDAE::Var>>, mut iIlst: Arc<metamodelica::List<i32>>, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut bt: Arc<BinaryTree::BinTree>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (varlst.clone(), iIlst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: _, tail: ilst }) => {
                    BinaryTree::treeGet(bt.clone(), cr.clone())?;
                    markBranchVars1(vlst.clone(), ilst.clone(), mark.clone(), rowmark.clone(), bt.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: vlst }, Deref @ metamodelica::List::Cons { head: i, tail: ilst }) => {
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = -(mark.clone()); _arr};
                    markBranchVars1(vlst.clone(), ilst.clone(), mark.clone(), rowmark.clone(), bt.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getIfExpBranchVarOccurency(mut inExp: Arc<DAE::Exp>, mut inBt: Arc<BinaryTree::BinTree>) -> Result<(Arc<DAE::Exp>, bool, Arc<BinaryTree::BinTree>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut bt: Arc<BinaryTree::BinTree> = Arc::new(<BinaryTree::BinTree as ::std::default::Default>::default());
    (outExp, cont, bt) = (::match_deref::match_deref! { match &((inExp.clone(), inBt.clone())) {
        (e @ Deref @ DAE::Exp::IFEXP { expElse: e2, expThen: e1, .. }, bt) => {
            let mut bt_then: Arc<BinaryTree::BinTree> = Arc::new(<BinaryTree::BinTree as ::std::default::Default>::default());
            let mut bt_else: Arc<BinaryTree::BinTree> = Arc::new(<BinaryTree::BinTree as ::std::default::Default>::default());
            let mut bt = (*bt).clone();
            (_, bt_then) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(getIfExpBranchVarOccurency) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<BinaryTree::BinTree>) -> Result<(Arc<DAE::Exp>, bool, Arc<BinaryTree::BinTree>)> + 'static>), BinaryTree::emptyBinTree().clone())?;
            (_, bt_else) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(getIfExpBranchVarOccurency) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<BinaryTree::BinTree>) -> Result<(Arc<DAE::Exp>, bool, Arc<BinaryTree::BinTree>)> + 'static>), BinaryTree::emptyBinTree().clone())?;
            bt = BinaryTree::binTreeintersection(bt_then.clone(), bt_else.clone(), bt.clone())?;
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::LUNARY { .. }, bt) => {
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::LBINARY { .. }, bt) => {
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::RELATION { .. }, bt) => {
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::RANGE { .. }, bt) => {
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::RANGE { .. }, bt) => {
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::ASUB { exp: e1, .. }, bt) => {
            let mut bt = (*bt).clone();
            (_, bt) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(getIfExpBranchVarOccurency) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<BinaryTree::BinTree>) -> Result<(Arc<DAE::Exp>, bool, Arc<BinaryTree::BinTree>)> + 'static>), bt.clone())?;
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, bt) => {
            let mut bt = (*bt).clone();
            bt = BinaryTree::treeAdd(bt.clone(), cr.clone(), 0)?;
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, bt) => {
            let mut bt = (*bt).clone();
            bt = BinaryTree::treeAdd(bt.clone(), cr.clone(), 0)?;
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, bt) => {
            let mut bt = (*bt).clone();
            bt = BinaryTree::treeAdd(bt.clone(), cr.clone(), 0)?;
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, bt) => {
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, bt) => {
            (e.clone(), false, bt.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, bt) => {
            let mut b: bool = false;
            b = Flags::getConfigBool(Flags::DELAY_BREAK_LOOP.clone())? && ExpressionBasics::expEqual(e1.clone(), e2.clone())?;
            (e.clone(), !(b.clone()), bt.clone())
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, _) => {
            getIfExpBranchVarOccurency(e1.clone(), inBt.clone())?
        },
        _ => {
            (inExp.clone(), true, inBt.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, bt))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn adjacencyRowExpEnhanced1(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut vars: Arc<metamodelica::List<i32>>, mut notinder: bool, mut mark: i32, mut rowmark: metamodelica::Array<i32>, mut unsolvable: bool) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outIntegerLst = 'mc: {
        let __mc_input = (inVarLst.clone(), inIntegerLst.clone(), notinder.clone(), unsolvable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(vars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, false, _) => {
                    if !((!(intEq(intAbs(rowmark.borrow()[(i.clone()-1) as usize].clone()), mark.clone())))) { bail!("guard") }
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), cons(i.clone(), vars.clone()), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, true, _) => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i1: i32 = 0;
                    i1 = -(i.clone());
                    if '__try0: {
                        unwrap_break_err!(List::getMemberOnTrue(i1.clone(), vars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), cons(i1.clone(), vars.clone()), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE_DER, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, _) => {
                    if !((!(intEq(intAbs(rowmark.borrow()[(i.clone()-1) as usize].clone()), mark.clone())))) { bail!("guard") }
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), cons(i.clone(), vars.clone()), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE_DER, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, true) => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    b = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), mark.clone());
                    b1 = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), -(mark.clone()));
                    b = b.clone() || b1.clone();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = List::consOnTrue(!(b.clone()), i.clone(), vars.clone());
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), res.clone(), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, _) => {
                    if !((!(intEq(intAbs(rowmark.borrow()[(i.clone()-1) as usize].clone()), mark.clone())))) { bail!("guard") }
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), cons(i.clone(), vars.clone()), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, true) => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    b = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), mark.clone());
                    b1 = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), -(mark.clone()));
                    b = b.clone() || b1.clone();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = List::consOnTrue(!(b.clone()), i.clone(), vars.clone());
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), res.clone(), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::ALG_STATE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, _) => {
                    if !((!(intEq(intAbs(rowmark.borrow()[(i.clone()-1) as usize].clone()), mark.clone())))) { bail!("guard") }
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), cons(i.clone(), vars.clone()), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::ALG_STATE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, true) => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    b = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), mark.clone());
                    b1 = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), -(mark.clone()));
                    b = b.clone() || b1.clone();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = List::consOnTrue(!(b.clone()), i.clone(), vars.clone());
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), res.clone(), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, _) => {
                    if !((!(intEq(intAbs(rowmark.borrow()[(i.clone()-1) as usize].clone()), mark.clone())))) { bail!("guard") }
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), cons(i.clone(), vars.clone()), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, true) => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    b = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), mark.clone());
                    b1 = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), -(mark.clone()));
                    b = b.clone() || b1.clone();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = List::consOnTrue(!(b.clone()), i.clone(), vars.clone());
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), res.clone(), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, _) => {
                    if !((!(intEq(intAbs(rowmark.borrow()[(i.clone()-1) as usize].clone()), mark.clone())))) { bail!("guard") }
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), cons(i.clone(), vars.clone()), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, true) => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    b = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), mark.clone());
                    b1 = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), -(mark.clone()));
                    b = b.clone() || b1.clone();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = List::consOnTrue(!(b.clone()), i.clone(), vars.clone());
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), res.clone(), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, _) => {
                    if !((!(intEq(intAbs(rowmark.borrow()[(i.clone()-1) as usize].clone()), mark.clone())))) { bail!("guard") }
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), cons(i.clone(), vars.clone()), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE, .. }, tail: rest }, Deref @ metamodelica::List::Cons { head: i, tail: irest }, _, true) => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    b = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), mark.clone());
                    b1 = intEq(rowmark.borrow()[(i.clone()-1) as usize].clone(), -(mark.clone()));
                    b = b.clone() || b1.clone();
                    {let _arr = rowmark.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = if (unsolvable.clone()) {-(mark.clone())} else {mark.clone()}; _arr};
                    res = List::consOnTrue(!(b.clone()), i.clone(), vars.clone());
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), res.clone(), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, Deref @ metamodelica::List::Cons { head: _, tail: irest }, _, _) => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    res = adjacencyRowExpEnhanced1(rest.clone(), irest.clone(), vars.clone(), notinder.clone(), mark.clone(), rowmark.clone(), unsolvable.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIntegerLst)
}

pub fn solvabilityWights(mut solva: BackendDAE::Solvability) -> Result<i32> {
    let mut i: i32 = 0;
    i = (match solva.clone() {
        BackendDAE::Solvability::SOLVABILITY_SOLVED => 1,
        BackendDAE::Solvability::SOLVABILITY_CONSTONE => 2,
        BackendDAE::Solvability::SOLVABILITY_CONST { .. } => 5,
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false } => 0,
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true } => 50,
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: false } => 0,
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: true } => 100,
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR => 500,
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE => 1000,
        _ => bail!("match: no arm matched"),
    });
    Ok(i)
}

pub fn solvabilityCMP(mut sa: BackendDAE::Solvability, mut sb: BackendDAE::Solvability) -> Result<bool> {
    let mut b: bool = false;
    b = (match (sa.clone(), sb.clone()) {
        (BackendDAE::Solvability::SOLVABILITY_SOLVED, BackendDAE::Solvability::SOLVABILITY_SOLVED) => false,
        (_, BackendDAE::Solvability::SOLVABILITY_SOLVED) => true,
        (BackendDAE::Solvability::SOLVABILITY_SOLVED, BackendDAE::Solvability::SOLVABILITY_CONSTONE) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONSTONE, BackendDAE::Solvability::SOLVABILITY_CONSTONE) => false,
        (_, BackendDAE::Solvability::SOLVABILITY_CONSTONE) => true,
        (BackendDAE::Solvability::SOLVABILITY_SOLVED, BackendDAE::Solvability::SOLVABILITY_CONST { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONSTONE, BackendDAE::Solvability::SOLVABILITY_CONST { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONST { .. }, BackendDAE::Solvability::SOLVABILITY_CONST { .. }) => false,
        (_, BackendDAE::Solvability::SOLVABILITY_CONST { .. }) => true,
        (BackendDAE::Solvability::SOLVABILITY_SOLVED, BackendDAE::Solvability::SOLVABILITY_PARAMETER { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONSTONE, BackendDAE::Solvability::SOLVABILITY_PARAMETER { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONST { .. }, BackendDAE::Solvability::SOLVABILITY_PARAMETER { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_PARAMETER { .. }, BackendDAE::Solvability::SOLVABILITY_PARAMETER { .. }) => false,
        (_, BackendDAE::Solvability::SOLVABILITY_PARAMETER { .. }) => true,
        (BackendDAE::Solvability::SOLVABILITY_SOLVED, BackendDAE::Solvability::SOLVABILITY_LINEAR { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONSTONE, BackendDAE::Solvability::SOLVABILITY_LINEAR { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONST { .. }, BackendDAE::Solvability::SOLVABILITY_LINEAR { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_PARAMETER { .. }, BackendDAE::Solvability::SOLVABILITY_LINEAR { .. }) => false,
        (BackendDAE::Solvability::SOLVABILITY_LINEAR { .. }, BackendDAE::Solvability::SOLVABILITY_LINEAR { .. }) => false,
        (_, BackendDAE::Solvability::SOLVABILITY_LINEAR { .. }) => true,
        (BackendDAE::Solvability::SOLVABILITY_SOLVED, BackendDAE::Solvability::SOLVABILITY_NONLINEAR) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONSTONE, BackendDAE::Solvability::SOLVABILITY_NONLINEAR) => false,
        (BackendDAE::Solvability::SOLVABILITY_CONST { .. }, BackendDAE::Solvability::SOLVABILITY_NONLINEAR) => false,
        (BackendDAE::Solvability::SOLVABILITY_PARAMETER { .. }, BackendDAE::Solvability::SOLVABILITY_NONLINEAR) => false,
        (BackendDAE::Solvability::SOLVABILITY_LINEAR { .. }, BackendDAE::Solvability::SOLVABILITY_NONLINEAR) => false,
        (BackendDAE::Solvability::SOLVABILITY_NONLINEAR, BackendDAE::Solvability::SOLVABILITY_NONLINEAR) => false,
        (_, BackendDAE::Solvability::SOLVABILITY_NONLINEAR) => true,
        (BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE, BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE) => false,
        (BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE, _) => true,
        _ => bail!("match: no arm matched"),
    });
    Ok(b)
}

pub fn getArrayEquationSub(mut Index: i32, mut inAD: Arc<metamodelica::List<Option<i32>>>, mut inList: Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>)>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Subscript>>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>)>>)> {
    let mut outSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut outList: Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>)>> = metamodelica::nil();
    (outSubs, outList) = 'mc: {
        let __mc_input = (Index.clone(), inAD.clone(), inList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, ad, Deref @ metamodelica::List::Nil) => {
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    let mut subslst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    subslst = Expression::dimensionSizesSubcriptsOpt(ad.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Expression::rangesToSubscripts(subslst.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    subs = __pa0.clone();
                    subslst1 = __pa1.clone();
                    Ok((subs.clone(), list![(i.clone(), subslst1.clone())]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, _, Deref @ metamodelica::List::Cons { head: (ie, Deref @ metamodelica::List::Cons { head: subs, tail: Deref @ metamodelica::List::Nil }), tail: rest }) => {
                    if !((intEq(i.clone(), ie.clone()))) { bail!("guard") }
                    Ok((subs.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, _, Deref @ metamodelica::List::Cons { head: (ie, Deref @ metamodelica::List::Cons { head: subs, tail: subslst }), tail: rest }) => {
                    if !((intEq(i.clone(), ie.clone()))) { bail!("guard") }
                    Ok((subs.clone(), cons((ie.clone(), subslst.clone()), rest.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, ad, Deref @ metamodelica::List::Cons { head: entry @ (ie, _), tail: rest }) => {
                    if !((!(intEq(i.clone(), ie.clone())))) { bail!("guard") }
                    let mut subs1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut entrylst: Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>)>> = metamodelica::nil();
                    (subs1, entrylst) = getArrayEquationSub(i.clone(), ad.clone(), rest.clone())?;
                    Ok((subs1.clone(), cons(entry.clone(), entrylst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- BackendDAE.getArrayEquationSub failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSubs, outList))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn containAnyVar(mut inExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inVariables: BackendDAE::Variables) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (inExpComponentRefLst.clone(), inVariables.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr, tail: _ }, vars) => {
                    BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: crefs }, vars) => {
                    Ok(containAnyVar(crefs.clone(), vars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBoolean)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn containAnyVarWithoutStates(mut inExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inVariables: BackendDAE::Variables) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (inExpComponentRefLst.clone(), inVariables.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr, tail: _ }, vars) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    let false = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: crefs }, vars) => {
                    Ok(containAnyVarWithoutStates(crefs.clone(), vars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBoolean)
}

pub fn getEqnSysRhs(mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVariables: BackendDAE::Variables, mut funcs: Option<Arc<AvlTreePathFunction::Tree>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ElementSource>>>)> {
    let mut outRhsExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outSources: Arc<metamodelica::List<Arc<DAE::ElementSource>>> = metamodelica::nil();
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    repl = makeZeroReplacements(inVariables.clone())?;
    (_, outRhsExps, outSources, _, _) = BackendEquation::traverseEquationArray(inEqns.clone(), (std::sync::Arc::new(equationToExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ElementSource>>>, Option<Arc<AvlTreePathFunction::Tree>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ElementSource>>>, Option<Arc<AvlTreePathFunction::Tree>>, BackendVarTransform::VariableReplacements))> + 'static>), (inVariables.clone(), metamodelica::nil(), metamodelica::nil(), funcs.clone(), repl.clone()))?;
    Ok((outRhsExps, outSources))
}

fn equationToExp(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ElementSource>>>, Option<Arc<AvlTreePathFunction::Tree>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ElementSource>>>, Option<Arc<AvlTreePathFunction::Tree>>, BackendVarTransform::VariableReplacements))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ElementSource>>>, Option<Arc<AvlTreePathFunction::Tree>>, BackendVarTransform::VariableReplacements);
    (outEq, outTpl) = 'mc: {
        let __mc_input = (inEq.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { source, exp: e, .. }, (v, explst, sources, funcs, repl)) => {
                    let mut rhs_exp: Arc<DAE::Exp>;
                    rhs_exp = getEqnsysRhsExp(e.clone(), v.clone(), funcs.clone(), Some(repl.clone()))?;
                    Ok((eqn.clone(), (v.clone(), cons(rhs_exp.clone(), explst.clone()), cons(source.clone(), sources.clone()), funcs.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { source, scalar: e2, exp: e1, .. }, (v, explst, sources, funcs, repl)) => {
                    let mut new_exp: Arc<DAE::Exp>;
                    let mut rhs_exp: Arc<DAE::Exp>;
                    let mut rhs_exp_1: Arc<DAE::Exp>;
                    let mut rhs_exp_2: Arc<DAE::Exp>;
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    rhs_exp = getEqnsysRhsExp(new_exp.clone(), v.clone(), funcs.clone(), Some(repl.clone()))?;
                    rhs_exp_1 = Expression::negate(rhs_exp.clone())?;
                    (rhs_exp_2, _) = ExpressionSimplify::simplify(rhs_exp_1.clone())?;
                    Ok((eqn.clone(), (v.clone(), cons(rhs_exp_2.clone(), explst.clone()), cons(source.clone(), sources.clone()), funcs.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { source, right: e2, left: e1, dimSize: ds, .. }, (v, explst, sources, funcs, repl)) => {
                    let mut new_exp: Arc<DAE::Exp>;
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    let mut explst = (*explst).clone();
                    let mut sources = (*sources).clone();
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    subslst = Expression::dimensionSizesSubscripts(ds.clone());
                    subslst = Expression::rangesToSubscripts(subslst.clone())?;
                    explst1 = List::map1r(subslst.clone(), (std::sync::Arc::new(Expression::applyExpSubscripts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Exp>> + 'static>), new_exp.clone());
                    explst1 = List::map3(explst1.clone(), (std::sync::Arc::new(getEqnsysRhsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, Option<BackendVarTransform::VariableReplacements>) -> Result<Arc<DAE::Exp>> + 'static>), v.clone(), funcs.clone(), Some(repl.clone()));
                    explst1 = List::map(explst1.clone(), (std::sync::Arc::new(Expression::negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    explst1 = ExpressionSimplify::simplifyList(explst1.clone())?;
                    explst = List::append_reverse(explst1.clone(), explst.clone());
                    sources = List::consN(BackendEquation::equationSize(eqn.clone())?, source.clone(), sources.clone());
                    Ok((eqn.clone(), (v.clone(), explst.clone(), sources.clone(), funcs.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { source, exp: e2, componentRef, .. }, (v, explst, sources, funcs, repl)) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut new_exp: Arc<DAE::Exp>;
                    let mut rhs_exp: Arc<DAE::Exp>;
                    let mut rhs_exp_1: Arc<DAE::Exp>;
                    let mut rhs_exp_2: Arc<DAE::Exp>;
                    e1 = Expression::crefExp(componentRef.clone())?;
                    new_exp = Expression::expSub(e1.clone(), e2.clone())?;
                    rhs_exp = getEqnsysRhsExp(new_exp.clone(), v.clone(), funcs.clone(), Some(repl.clone()))?;
                    rhs_exp_1 = Expression::negate(rhs_exp.clone())?;
                    (rhs_exp_2, _) = ExpressionSimplify::simplify(rhs_exp_1.clone())?;
                    Ok((eqn.clone(), (v.clone(), cons(rhs_exp_2.clone(), explst.clone()), cons(source.clone(), sources.clone()), funcs.clone(), repl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (BackendDump::equationString(eqn.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEUtil.equationToExp failed for complex equation: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], BackendEquation::equationInfo(eqn.clone())?)?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (BackendDump::equationString(eqn.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEUtil.equationToExp failed: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], BackendEquation::equationInfo(eqn.clone())?)?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEq, outTpl))
}

pub fn getEqnsysRhsExp(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables, mut funcs: Option<Arc<AvlTreePathFunction::Tree>>, mut oRepl: Option<BackendVarTransform::VariableReplacements>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (match oRepl.clone() {
        Some(mut repl) => {
            let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(getEqnsysRhsExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool))> + 'static>), (repl.clone(), inVariables.clone(), funcs.clone(), true))?) {
                (__pa0, (_, _, _, true)) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            outExp = __pa0.clone();
            (outExp, _) = ExpressionSimplify::simplify(outExp.clone())?;
            outExp.clone()
        },
        _ => {
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            repl = makeZeroReplacements(inVariables.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(getEqnsysRhsExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool))> + 'static>), (repl.clone(), inVariables.clone(), funcs.clone(), true))?) {
                (__pa0, (_, _, _, true)) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            outExp = __pa0.clone();
            (outExp, _) = ExpressionSimplify::simplify(outExp.clone())?;
            outExp.clone()
        },
    });
    Ok(outExp)
}

fn getEqnsysRhsExp1(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (e @ Deref @ DAE::Exp::CREF { .. }, (repl, _, _, _)) => {
            let mut e1: Arc<DAE::Exp>;
            let mut b1: bool = false;
            (e1, b1) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?;
            e1 = if (b1.clone()) {e1.clone()} else {e.clone()};
            (e1.clone(), false, inTpl.clone())
        },
        (Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: t, expElse: f }, (repl, vars, funcs, b)) => {
            let mut tpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool);
            let mut t = (*t).clone();
            let mut f = (*f).clone();
            let mut b = (*b).clone();
            let (_, (_, __pa0)) = Expression::traverseExpTopDown(cond.clone(), (std::sync::Arc::new(getEqnsysRhsExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool))> + 'static>), (vars.clone(), b.clone()))?;
            b = __pa0.clone();
            (t, tpl) = Expression::traverseExpTopDown(t.clone(), (std::sync::Arc::new(getEqnsysRhsExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool))> + 'static>), (repl.clone(), vars.clone(), funcs.clone(), b.clone()))?;
            (f, tpl) = Expression::traverseExpTopDown(f.clone(), (std::sync::Arc::new(getEqnsysRhsExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool))> + 'static>), tpl.clone())?;
            (Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: t.clone(), expElse: f.clone() }), false, tpl.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
            (e.clone(), true, inTpl.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (e.clone(), false, inTpl.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (e.clone(), false, inTpl.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: cond, tail: Deref @ metamodelica::List::Cons { head: t, tail: Deref @ metamodelica::List::Cons { head: f, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, _) => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut zero: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut tpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool);
            tp = Expression::r#typeof(e.clone())?;
            (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            e1 = Expression::expMul(cond.clone(), t.clone())?;
            e2 = Expression::expMul(cond.clone(), f.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: cond.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: zero.clone(), index: -1, optionExpisASUB: None }), expThen: e1.clone(), expElse: e2.clone() });
            (exp, tpl) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(getEqnsysRhsExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool))> + 'static>), inTpl.clone())?;
            (exp.clone(), false, tpl.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { expLst, .. }, (repl, vars, funcs, b)) => {
            let mut e = (*e).clone();
            let mut b = (*b).clone();
            let (_, (_, __pa0)) = Expression::traverseExpListTopDown(expLst.clone(), (std::sync::Arc::new(getEqnsysRhsExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool))> + 'static>), (vars.clone(), b.clone()))?;
            b = __pa0.clone();
            (e, b) = getEqnsysRhsExp3(b.clone(), e.clone(), (repl.clone(), vars.clone(), funcs.clone(), true))?;
            (e.clone(), false, (repl.clone(), vars.clone(), funcs.clone(), b.clone()))
        },
        (e, (_, _, _, b)) => {
            (e.clone(), b.clone(), inTpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn getEqnsysRhsExp3(mut b: bool, mut inExp: Arc<DAE::Exp>, mut iTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool)) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut oExp: Arc<DAE::Exp>;
    let mut notfound: bool = false;
    (oExp, notfound) = 'mc: {
        let __mc_input = (b.clone(), iTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, (_, _, funcs, _)) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut notfound: bool = notfound.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Inline::forceInlineExp(inExp.clone(), (funcs.clone(), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone())?) {
                        (__pa0, _, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    let (__pa1, (_, _, _, __pa2)) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(getEqnsysRhsExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendVarTransform::VariableReplacements, BackendDAE::Variables, Option<Arc<AvlTreePathFunction::Tree>>, bool))> + 'static>), iTpl.clone())?;
                    e = __pa1.clone();
                    notfound = __pa2.clone();
                    Ok((e.clone(), notfound.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oExp, notfound))
}

pub fn getEqnsysRhsExp2(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (BackendDAE::Variables, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _)) => {
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((inExp.clone(), false, (vars.clone(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, b)) => {
                    Ok((inExp.clone(), b.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

pub fn makeZeroReplacements(mut vars: BackendDAE::Variables) -> Result<BackendVarTransform::VariableReplacements> {
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    repl = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(makeZeroReplacement) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), BackendVarTransform::emptyReplacements())?;
    Ok(repl)
}

fn makeZeroReplacement(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> {
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    (var, repl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut var, mut repl) = __mc_input.clone() else { bail!("nomatch") };
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cr = BackendVariable::varCref(var.clone())?;
            repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), Expression::makeConstZero(ComponentReference::crefLastType(cr.clone())?), None)?;
            Ok((var.clone(), repl.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inVar.clone(), inRepl.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((var, repl))
}

/* ************************************************
 * traverseBackendDAE and stuff
 ************************************************/
pub fn traverseBackendDAEExps<Type_a: Clone + 'static>(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = 'mc: {
        let __mc_input = inBackendDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE { UNIQUEIO: systs, derivativeNamePrefix: shared, .. } => {
                    let mut outTypeA: Type_a;
                    outTypeA = List::fold1(systs.clone(), (std::sync::Arc::new(traverseBackendDAEExpsEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, _, _) -> Result<_> + 'static>), func.clone(), inTypeA.clone());
                    outTypeA = traverseBackendDAEExpsVars(shared.globalKnownVars.clone(), func.clone(), outTypeA.clone())?;
                    outTypeA = traverseBackendDAEExpsEqns(shared.initialEqs.clone(), func.clone(), outTypeA.clone())?;
                    outTypeA = traverseBackendDAEExpsEqns(shared.removedEqs.clone(), func.clone(), outTypeA.clone())?;
                    Ok(outTypeA.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut name: ArcStr = arcstr::literal!("");
                    (_, _, name) = System::dladdr(func.clone());
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("traverseBackendDAEExps failed for ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTypeA)
}

pub fn traverseBackendDAEExpsEqSystemJacobians<Type_a: Clone + 'static>(mut syst: Arc<BackendDAE::EqSystem>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = 'mc: {
        let __mc_input = syst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { stateSets, .. } => {
                    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
                    let mut arg: Type_a;
                    comps = getStrongComponents(syst.clone());
                    arg = traverseStrongComponentsJacobiansExp(comps.clone(), func.clone(), inTypeA.clone())?;
                    arg = traverseStateSetsJacobiansExp(stateSets.clone(), func.clone(), arg.clone())?;
                    Ok(arg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inTypeA.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTypeA)
}

pub fn traverseStrongComponentsJacobiansExp<Type_a: Clone + 'static>(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut arg: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut arg: Type_a = arg;
    for mut comp in &*inComps.clone() {
        let mut comp = comp.clone();
        arg = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, .. } => {
            traverseBackendDAEExpsJacobianEqn(jac.clone(), inFunc.clone(), arg.clone())?
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((bdae, _, _, _, _, _)), .. }, .. } => {
            traverseBackendDAEExps(bdae.clone(), inFunc.clone(), arg.clone())?
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((bdae, _, _, _, _, _)), .. }, .. }, .. } => {
            traverseBackendDAEExps(bdae.clone(), inFunc.clone(), arg.clone())?
        },
        _ => {
            arg.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(arg)
}

fn traverseBackendDAEExpsJacobianEqn<Type_a: Clone + 'static>(mut inJacEntry: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = (::match_deref::match_deref! { match &(inJacEntry.clone()) {
        Deref @ metamodelica::List::Nil => {
            inTypeA.clone()
        },
        Deref @ metamodelica::List::Cons { head: (_, _, eqn), tail: _ } => {
            let mut typeA: Type_a;
            typeA = traverseBackendDAEExpsOptEqn(Some(eqn.clone()), func.clone(), inTypeA.clone())?;
            typeA.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTypeA)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn traverseStateSetsJacobiansExp<Type_a: Clone + 'static>(mut inStateSets: Arc<metamodelica::List<BackendDAE::StateSet>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = (::match_deref::match_deref! { match &(inStateSets.clone()) {
        Deref @ metamodelica::List::Nil => {
            inTypeA.clone()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::StateSet { jacobian: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((bdae, _, _, _, _, _)), .. }, .. }, tail: rest } => {
            let mut arg: Type_a;
            arg = traverseBackendDAEExps(bdae.clone(), inFunc.clone(), inTypeA.clone())?;
            traverseStateSetsJacobiansExp(rest.clone(), inFunc.clone(), arg.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTypeA)
}

pub fn traverseBackendDAEExpsNoCopyWithUpdate<A: Clone + 'static>(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, A)> + 'static>, mut inTypeA: A) -> Result<A> {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, A)> + 'static>;

    let mut outTypeA: A;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut name: ArcStr = arcstr::literal!("");
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
            Deref @ DAE { UNIQUEIO: __pa1, derivativeNamePrefix: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        systs = __pa1.clone();
        shared = __pa2.clone();
        outTypeA = List::fold1(systs.clone(), (std::sync::Arc::new(traverseBackendDAEExpsEqSystemWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, _, _) -> Result<_> + 'static>), func.clone(), inTypeA.clone());
        outTypeA = unwrap_break_err!(traverseBackendDAEExpsVarsWithUpdate(shared.globalKnownVars.clone(), func.clone(), outTypeA.clone()), '__try0);
        outTypeA = unwrap_break_err!(traverseBackendDAEExpsEqns(shared.initialEqs.clone(), func.clone(), outTypeA.clone()), '__try0);
        outTypeA = unwrap_break_err!(traverseBackendDAEExpsEqns(shared.removedEqs.clone(), func.clone(), outTypeA.clone()), '__try0);
        Ok::<_, anyhow::Error>((outTypeA.clone(), shared.clone(), systs.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            outTypeA = __try0_o0;
            shared = __try0_o1;
            systs = __try0_o2;
        }
        Err(_) => {
            (_, _, name) = System::dladdr(func.clone());
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("traverseBackendDAEExpsNoCopyWithUpdate failed for ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok(outTypeA)
}

pub fn traverseBackendDAEExpsEqSystem<Type_a: Clone + 'static>(mut syst: Arc<BackendDAE::EqSystem>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = traverseBackendDAEExpsVars(syst.orderedVars.clone(), func.clone(), inTypeA.clone())?;
    outTypeA = traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), func.clone(), outTypeA.clone())?;
    outTypeA = traverseBackendDAEExpsEqns(syst.removedEqs.clone(), func.clone(), outTypeA.clone())?;
    Ok(outTypeA)
}

pub fn traverseBackendDAEExpsEqSystemWithUpdate<Type_a: Clone + 'static>(mut syst: Arc<BackendDAE::EqSystem>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = traverseBackendDAEExpsVarsWithUpdate(syst.orderedVars.clone(), func.clone(), inTypeA.clone())?;
    outTypeA = traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), func.clone(), outTypeA.clone())?;
    outTypeA = traverseBackendDAEExpsEqns(syst.removedEqs.clone(), func.clone(), outTypeA.clone())?;
    Ok(outTypeA)
}

pub fn traverseBackendDAEExpsVars<Type_a: Clone + 'static>(mut inVariables: BackendDAE::Variables, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = 'mc: {
        let __mc_input = inVariables.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Variables { varArr: BackendDAE::VariableArray { varOptArr: mut varOptArr, .. }, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut ext_arg_1: Type_a;
            ext_arg_1 = traverseArrayNoCopy(varOptArr.clone(), func.clone(), (std::sync::Arc::new(traverseBackendDAEExpsVar) as std::sync::Arc<dyn ::std::ops::Fn(Option<BackendDAE::Var>, _, _) -> Result<_> + 'static>), inTypeA.clone(), (varOptArr.clone().borrow().len() as i32))?;
            Ok(ext_arg_1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut name: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            (_, _, name) = System::dladdr(func.clone());
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAE.traverseBackendDAEExpsVars failed for ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTypeA)
}

pub fn traverseBackendDAEExpsVarsWithUpdate<Type_a: Clone + 'static>(mut inVariables: BackendDAE::Variables, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    outTypeA = 'mc: {
        let __mc_input = inVariables.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Variables { varArr: BackendDAE::VariableArray { varOptArr: mut varOptArr, .. }, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut ext_arg_1: Type_a;
            (_, ext_arg_1) = traverseArrayNoCopyWithUpdate(varOptArr.clone(), func.clone(), (std::sync::Arc::new(traverseBackendDAEExpsVarWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Option<BackendDAE::Var>, _, _) -> Result<_> + 'static>), inTypeA.clone(), (varOptArr.clone().borrow().len() as i32))?;
            Ok(ext_arg_1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut name: ArcStr = arcstr::literal!("");
            (_, _, name) = System::dladdr(func.clone());
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("traverseBackendDAEExpsVarsWithUpdate failed for ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTypeA)
}

pub fn traverseArrayNoCopy<ArrT: Clone + 'static, ElemT: Clone + 'static, ArgT: Clone + 'static>(mut inArray: metamodelica::Array<ArrT>, mut inElemFunc: Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, ArgT)> + 'static>, mut inArrayFunc: Arc<dyn ::std::ops::Fn(ArrT, Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, ArgT)> + 'static>, ArgT) -> Result<ArgT> + 'static>, mut inArg: ArgT, mut inLength: i32) -> Result<ArgT> {
    pub type ElemFuncType<ElemT: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, ArgT)> + 'static>;

    pub type ArrayFuncType<ArrT: Clone + 'static, ElemT: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ArrT, Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, ArgT)> + 'static>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT = inArg.clone();
    let true = (inLength.clone() <= (inArray.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
    for mut i in 1..=inLength.clone() {
        outArg = inArrayFunc(inArray.borrow()[(i.clone()-1) as usize].clone(), inElemFunc.clone(), outArg.clone())?;
    }
    Ok(outArg)
}

pub fn traverseArrayNoCopyWithStop<ArrT: Clone + 'static, ElemT: Clone + 'static, ArgT: Clone + 'static>(mut inArray: metamodelica::Array<ArrT>, mut inElemFunc: Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, bool, ArgT)> + 'static>, mut inArrayFunc: Arc<dyn ::std::ops::Fn(ArrT, Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, bool, ArgT)> + 'static>, ArgT) -> Result<(bool, ArgT)> + 'static>, mut inArg: ArgT, mut inLength: i32) -> Result<ArgT> {
    pub type ElemFuncType<ElemT: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, bool, ArgT)> + 'static>;

    pub type ArrayFuncType<ArrT: Clone + 'static, ElemT: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ArrT, Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, bool, ArgT)> + 'static>, ArgT) -> Result<(bool, ArgT)> + 'static>;

    let mut outArg: ArgT = inArg.clone();
    let mut cont: bool = false;
    let true = (inLength.clone() <= (inArray.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
    for mut i in 1..=inLength.clone() {
        (cont, outArg) = inArrayFunc(inArray.borrow()[(i.clone()-1) as usize].clone(), inElemFunc.clone(), outArg.clone())?;
        if !(cont.clone()) {
            break;
        }
    }
    Ok(outArg)
}

pub fn traverseArrayNoCopyWithUpdate<ArrT: Clone + 'static + PartialEq, ElemT: Clone + 'static, ArgT: Clone + 'static>(mut inArray: metamodelica::Array<ArrT>, mut inElemFunc: Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, ArgT)> + 'static>, mut inArrayFunc: Arc<dyn ::std::ops::Fn(ArrT, Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, ArgT)> + 'static>, ArgT) -> Result<(ArrT, ArgT)> + 'static>, mut inArg: ArgT, mut inLength: i32) -> Result<(metamodelica::Array<ArrT>, ArgT)> {
    pub type ElemFuncType<ElemT: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, ArgT)> + 'static>;

    pub type ArrayFuncType<ArrT: Clone + 'static, ElemT: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ArrT, Arc<dyn ::std::ops::Fn(ElemT, ArgT) -> Result<(ElemT, ArgT)> + 'static>, ArgT) -> Result<(ArrT, ArgT)> + 'static>;

    let mut outArray: metamodelica::Array<ArrT> = inArray.clone();
    let mut outArg: ArgT = inArg.clone();
    let mut e: ArrT;
    let mut new_e: ArrT;
    let true = (inLength.clone() <= (inArray.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
    for mut i in 1..=inLength.clone() {
        e = inArray.borrow()[(i.clone()-1) as usize].clone();
        (new_e, outArg) = inArrayFunc(e.clone(), inElemFunc.clone(), outArg.clone())?;
        if !(referenceEq(&e.clone(),&new_e.clone())) {
            {let _arr = outArray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = new_e.clone(); _arr};
        }
    }
    Ok((outArray, outArg))
}

fn traverseBackendDAEExpsVar<Type_a: Clone + 'static>(mut inVar: Option<BackendDAE::Var>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    (_, outTypeA) = traverseBackendDAEExpsVarWithUpdate(inVar.clone(), func.clone(), inTypeA.clone())?;
    Ok(outTypeA)
}

fn traverseBackendDAEExpsVarWithUpdate<Type_a: Clone + 'static>(mut inVar: Option<BackendDAE::Var>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(Option<BackendDAE::Var>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut ovar: Option<BackendDAE::Var> = None;
    let mut outTypeA: Type_a;
    (ovar, outTypeA) = 'mc: {
        let __mc_input = inVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok((None, inTypeA.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(BackendDAE::Var { varName: cref, varKind, varDirection, varParallelism, varType, bindExp: Some(e1), tplExp, arryDim: instdims, source, values: attr, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, unreplaceable, initNonlinear, encrypted }) => {
                    let mut e1_: Arc<DAE::Exp>;
                    let mut attr_: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut v: Option<BackendDAE::Var> = None;
                    (e1_, ext_arg_1) = func(e1.clone(), inTypeA.clone())?;
                    (attr_, ext_arg_2) = traverseBackendDAEVarAttr(attr.clone(), func.clone(), ext_arg_1.clone())?;
                    if referenceEq(&e1.clone(),&e1_.clone()) && referenceEq(&attr.clone(),&attr_.clone()) {
                        v = inVar.clone();
                    } else {
                        v = Some(BackendDAE::Var { varName: cref.clone(), varKind: varKind.clone(), varDirection: varDirection.clone(), varParallelism: varParallelism.clone(), varType: varType.clone(), bindExp: Some(e1_.clone()), tplExp: tplExp.clone(), arryDim: instdims.clone(), source: source.clone(), values: attr_.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: unreplaceable.clone(), initNonlinear: initNonlinear.clone(), encrypted: encrypted.clone() });
                    }
                    Ok((v.clone(), ext_arg_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(BackendDAE::Var { varName: cref, varKind, varDirection, varParallelism, varType, bindExp: None, tplExp, arryDim: instdims, source, values: attr, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, unreplaceable, initNonlinear, encrypted }) => {
                    let mut attr_: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut ext_arg_2: Type_a;
                    let mut v: Option<BackendDAE::Var> = None;
                    (attr_, ext_arg_2) = traverseBackendDAEVarAttr(attr.clone(), func.clone(), inTypeA.clone())?;
                    if referenceEq(&attr.clone(),&attr_.clone()) {
                        v = inVar.clone();
                    } else {
                        v = Some(BackendDAE::Var { varName: cref.clone(), varKind: varKind.clone(), varDirection: varDirection.clone(), varParallelism: varParallelism.clone(), varType: varType.clone(), bindExp: None, tplExp: tplExp.clone(), arryDim: instdims.clone(), source: source.clone(), values: attr_.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: unreplaceable.clone(), initNonlinear: initNonlinear.clone(), encrypted: encrypted.clone() });
                    }
                    Ok((v.clone(), ext_arg_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    (_, _, name) = System::dladdr(func.clone());
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAE.traverseBackendDAEExpsVar failed for ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((ovar, outTypeA))
}

pub fn traverseBackendDAEVarAttr<ExtraArgType: Clone + 'static>(mut attr: Option<Arc<DAE::VariableAttributes>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ExtraArgType) -> Result<(Arc<DAE::Exp>, ExtraArgType)> + 'static>, mut extraArg: ExtraArgType) -> Result<(Option<Arc<DAE::VariableAttributes>>, ExtraArgType)> {
    pub type FuncExpType<ExtraArgType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ExtraArgType) -> Result<(Arc<DAE::Exp>, ExtraArgType)> + 'static>;

    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    let mut outExtraArg: ExtraArgType;
    (outAttr, outExtraArg) = (::match_deref::match_deref! { match &(attr.clone()) {
        None => {
            (None, extraArg.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: dist, equationBound: eqbound, isProtected: p, finalPrefix: fin, startOrigin }) => {
            let mut q_: Option<Arc<DAE::Exp>> = None;
            let mut u_: Option<Arc<DAE::Exp>> = None;
            let mut du_: Option<Arc<DAE::Exp>> = None;
            let mut min_: Option<Arc<DAE::Exp>> = None;
            let mut max_: Option<Arc<DAE::Exp>> = None;
            let mut i_: Option<Arc<DAE::Exp>> = None;
            let mut f_: Option<Arc<DAE::Exp>> = None;
            let mut n_: Option<Arc<DAE::Exp>> = None;
            let mut eqbound_: Option<Arc<DAE::Exp>> = None;
            let mut dist_: Option<Arc<DAE::Distribution>> = None;
            let mut a: Option<Arc<DAE::VariableAttributes>> = None;
            (q_, outExtraArg) = Expression::traverseExpOpt(q.clone(), func.clone(), extraArg.clone())?;
            (u_, outExtraArg) = Expression::traverseExpOpt(u.clone(), func.clone(), outExtraArg.clone())?;
            (du_, outExtraArg) = Expression::traverseExpOpt(du.clone(), func.clone(), outExtraArg.clone())?;
            (min_, outExtraArg) = Expression::traverseExpOpt(min.clone(), func.clone(), outExtraArg.clone())?;
            (max_, outExtraArg) = Expression::traverseExpOpt(max.clone(), func.clone(), outExtraArg.clone())?;
            (i_, outExtraArg) = Expression::traverseExpOpt(i.clone(), func.clone(), outExtraArg.clone())?;
            (f_, outExtraArg) = Expression::traverseExpOpt(f.clone(), func.clone(), outExtraArg.clone())?;
            (n_, outExtraArg) = Expression::traverseExpOpt(n.clone(), func.clone(), outExtraArg.clone())?;
            (eqbound_, outExtraArg) = Expression::traverseExpOpt(eqbound.clone(), func.clone(), outExtraArg.clone())?;
            (dist_, outExtraArg) = traverseBackendDAEAttrDistribution(dist.clone(), func.clone(), outExtraArg.clone())?;
            if referenceEq(&q.clone(),&q_.clone()) && referenceEq(&u.clone(),&u_.clone()) && referenceEq(&du.clone(),&du_.clone()) && referenceEq(&min.clone(),&min_.clone()) && referenceEq(&max.clone(),&max_.clone()) && referenceEq(&i.clone(),&i_.clone()) && referenceEq(&f.clone(),&f_.clone()) && referenceEq(&n.clone(),&n_.clone()) && referenceEq(&eqbound.clone(),&eqbound_.clone()) && referenceEq(&dist.clone(),&dist_.clone()) {
                a = attr.clone();
            } else {
                a = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q_.clone(), unit: u_.clone(), displayUnit: du_.clone(), min: min_.clone(), max: max_.clone(), start: i_.clone(), fixed: f_.clone(), nominal: n_.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: dist_.clone(), equationBound: eqbound_.clone(), isProtected: p.clone(), finalPrefix: fin.clone(), startOrigin: startOrigin.clone() }));
            }
            (a.clone(), outExtraArg.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: dist, equationBound: eqbound, isProtected: p, finalPrefix: fin, startOrigin }) => {
            let mut q_: Option<Arc<DAE::Exp>> = None;
            let mut min_: Option<Arc<DAE::Exp>> = None;
            let mut max_: Option<Arc<DAE::Exp>> = None;
            let mut i_: Option<Arc<DAE::Exp>> = None;
            let mut f_: Option<Arc<DAE::Exp>> = None;
            let mut eqbound_: Option<Arc<DAE::Exp>> = None;
            let mut dist_: Option<Arc<DAE::Distribution>> = None;
            let mut a: Option<Arc<DAE::VariableAttributes>> = None;
            (q_, outExtraArg) = Expression::traverseExpOpt(q.clone(), func.clone(), extraArg.clone())?;
            (min_, outExtraArg) = Expression::traverseExpOpt(min.clone(), func.clone(), outExtraArg.clone())?;
            (max_, outExtraArg) = Expression::traverseExpOpt(max.clone(), func.clone(), outExtraArg.clone())?;
            (i_, outExtraArg) = Expression::traverseExpOpt(i.clone(), func.clone(), outExtraArg.clone())?;
            (f_, outExtraArg) = Expression::traverseExpOpt(f.clone(), func.clone(), outExtraArg.clone())?;
            (eqbound_, outExtraArg) = Expression::traverseExpOpt(eqbound.clone(), func.clone(), outExtraArg.clone())?;
            (dist_, outExtraArg) = traverseBackendDAEAttrDistribution(dist.clone(), func.clone(), outExtraArg.clone())?;
            if referenceEq(&q.clone(),&q_.clone()) && referenceEq(&min.clone(),&min_.clone()) && referenceEq(&max.clone(),&max_.clone()) && referenceEq(&i.clone(),&i_.clone()) && referenceEq(&f.clone(),&f_.clone()) && referenceEq(&eqbound.clone(),&eqbound_.clone()) && referenceEq(&dist.clone(),&dist_.clone()) {
                a = attr.clone();
            } else {
                a = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q_.clone(), min: min_.clone(), max: max_.clone(), start: i_.clone(), fixed: f_.clone(), uncertainOption: unc.clone(), distributionOption: dist_.clone(), equationBound: eqbound_.clone(), isProtected: p.clone(), finalPrefix: fin.clone(), startOrigin: startOrigin.clone() }));
            }
            (a.clone(), outExtraArg.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: i, fixed: f, equationBound: eqbound, isProtected: p, finalPrefix: fin, startOrigin }) => {
            let mut q_: Option<Arc<DAE::Exp>> = None;
            let mut i_: Option<Arc<DAE::Exp>> = None;
            let mut f_: Option<Arc<DAE::Exp>> = None;
            let mut eqbound_: Option<Arc<DAE::Exp>> = None;
            let mut a: Option<Arc<DAE::VariableAttributes>> = None;
            (q_, outExtraArg) = Expression::traverseExpOpt(q.clone(), func.clone(), extraArg.clone())?;
            (i_, outExtraArg) = Expression::traverseExpOpt(i.clone(), func.clone(), outExtraArg.clone())?;
            (f_, outExtraArg) = Expression::traverseExpOpt(f.clone(), func.clone(), outExtraArg.clone())?;
            (eqbound_, outExtraArg) = Expression::traverseExpOpt(eqbound.clone(), func.clone(), outExtraArg.clone())?;
            if referenceEq(&q.clone(),&q_.clone()) && referenceEq(&i.clone(),&i_.clone()) && referenceEq(&f.clone(),&f_.clone()) && referenceEq(&eqbound.clone(),&eqbound_.clone()) {
                a = attr.clone();
            } else {
                a = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q_.clone(), start: i_.clone(), fixed: f_.clone(), equationBound: eqbound_.clone(), isProtected: p.clone(), finalPrefix: fin.clone(), startOrigin: startOrigin.clone() }));
            }
            (a.clone(), outExtraArg.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: i, fixed: f, equationBound: eqbound, isProtected: p, finalPrefix: fin, startOrigin }) => {
            let mut q_: Option<Arc<DAE::Exp>> = None;
            let mut i_: Option<Arc<DAE::Exp>> = None;
            let mut f_: Option<Arc<DAE::Exp>> = None;
            let mut eqbound_: Option<Arc<DAE::Exp>> = None;
            let mut a: Option<Arc<DAE::VariableAttributes>> = None;
            (q_, outExtraArg) = Expression::traverseExpOpt(q.clone(), func.clone(), extraArg.clone())?;
            (i_, outExtraArg) = Expression::traverseExpOpt(i.clone(), func.clone(), outExtraArg.clone())?;
            (f_, outExtraArg) = Expression::traverseExpOpt(f.clone(), func.clone(), outExtraArg.clone())?;
            (eqbound_, outExtraArg) = Expression::traverseExpOpt(eqbound.clone(), func.clone(), outExtraArg.clone())?;
            if referenceEq(&q.clone(),&q_.clone()) && referenceEq(&i.clone(),&i_.clone()) && referenceEq(&f.clone(),&f_.clone()) && referenceEq(&eqbound.clone(),&eqbound_.clone()) {
                a = attr.clone();
            } else {
                a = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q_.clone(), start: i_.clone(), fixed: f_.clone(), equationBound: eqbound_.clone(), isProtected: p.clone(), finalPrefix: fin.clone(), startOrigin: startOrigin.clone() }));
            }
            (a.clone(), outExtraArg.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: i, fixed: f, equationBound: eqbound, isProtected: p, finalPrefix: fin, startOrigin }) => {
            let mut q_: Option<Arc<DAE::Exp>> = None;
            let mut min_: Option<Arc<DAE::Exp>> = None;
            let mut max_: Option<Arc<DAE::Exp>> = None;
            let mut i_: Option<Arc<DAE::Exp>> = None;
            let mut f_: Option<Arc<DAE::Exp>> = None;
            let mut eqbound_: Option<Arc<DAE::Exp>> = None;
            let mut a: Option<Arc<DAE::VariableAttributes>> = None;
            (q_, outExtraArg) = Expression::traverseExpOpt(q.clone(), func.clone(), extraArg.clone())?;
            (min_, outExtraArg) = Expression::traverseExpOpt(min.clone(), func.clone(), outExtraArg.clone())?;
            (max_, outExtraArg) = Expression::traverseExpOpt(max.clone(), func.clone(), outExtraArg.clone())?;
            (i_, outExtraArg) = Expression::traverseExpOpt(i.clone(), func.clone(), outExtraArg.clone())?;
            (f_, outExtraArg) = Expression::traverseExpOpt(f.clone(), func.clone(), outExtraArg.clone())?;
            (eqbound_, outExtraArg) = Expression::traverseExpOpt(eqbound.clone(), func.clone(), outExtraArg.clone())?;
            if referenceEq(&q.clone(),&q_.clone()) && referenceEq(&min.clone(),&min_.clone()) && referenceEq(&max.clone(),&max_.clone()) && referenceEq(&i.clone(),&i_.clone()) && referenceEq(&f.clone(),&f_.clone()) && referenceEq(&eqbound.clone(),&eqbound_.clone()) {
                a = attr.clone();
            } else {
                a = Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q_.clone(), min: min_.clone(), max: max_.clone(), start: i_.clone(), fixed: f_.clone(), equationBound: eqbound_.clone(), isProtected: p.clone(), finalPrefix: fin.clone(), startOrigin: startOrigin.clone() }));
            }
            (a.clone(), outExtraArg.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: _, finalPrefix: _ }) => {
            (attr.clone(), extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outAttr, outExtraArg))
}

fn traverseBackendDAEAttrDistribution<Type_a: Clone + 'static>(mut distOpt: Option<Arc<DAE::Distribution>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut extraArg: Type_a) -> Result<(Option<Arc<DAE::Distribution>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outDistOpt: Option<Arc<DAE::Distribution>> = None;
    let mut outExtraArg: Type_a;
    (outDistOpt, outExtraArg) = (::match_deref::match_deref! { match &((distOpt.clone(), extraArg.clone())) {
        (None, outExtraArg) => {
            (None, outExtraArg.clone())
        },
        (Some(Deref @ DAE::Distribution { name, params: arr, paramNames: sarr }), _) => {
            let mut name_: Arc<DAE::Exp>;
            let mut arr_: Arc<DAE::Exp>;
            let mut sarr_: Arc<DAE::Exp>;
            let mut d: Option<Arc<DAE::Distribution>> = None;
            (arr_, _) = Expression::extendArrExp(arr.clone(), false)?;
            (sarr_, _) = Expression::extendArrExp(sarr.clone(), false)?;
            (name_, outExtraArg) = Expression::traverseExpBottomUp(name.clone(), func.clone(), extraArg.clone())?;
            (arr_, outExtraArg) = Expression::traverseExpBottomUp(arr_.clone(), func.clone(), outExtraArg.clone())?;
            (sarr_, outExtraArg) = Expression::traverseExpBottomUp(sarr_.clone(), func.clone(), outExtraArg.clone())?;
            if referenceEq(&name.clone(),&name_.clone()) && referenceEq(&arr.clone(),&arr_.clone()) && referenceEq(&sarr.clone(),&sarr_.clone()) {
                d = distOpt.clone();
            } else {
                d = Some(Arc::new(DAE::Distribution { name: name_.clone(), params: arr_.clone(), paramNames: sarr_.clone() }));
            }
            (d.clone(), outExtraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outDistOpt, outExtraArg))
}

pub fn traverseBackendDAEExpsEqns<T: Clone + 'static>(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut extraArg: T) -> Result<T> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut extraArg: T = extraArg;
    let mut name: ArcStr = arcstr::literal!("");
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut eqn_new: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    if '__try0: {
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
            if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
                eqn = unwrap_break_err!(ExpandableArray::get(i.clone(), equationArray.clone()), '__try0);
                (eqn_new, extraArg) = unwrap_break_err!(BackendEquation::traverseExpsOfEquation(eqn.clone(), func.clone(), extraArg.clone()), '__try0);
                if !(referenceEq(&eqn.clone(),&eqn_new.clone())) {
                    unwrap_break_err!(ExpandableArray::update(i.clone(), eqn_new.clone(), equationArray.clone()), '__try0);
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            (_, _, name) = System::dladdr(func.clone());
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAE.traverseBackendDAEExpsEqns failed for ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
        }
        bail!("fail");
    }
    Ok(extraArg)
}

pub fn traverseBackendDAEExpsEqnsWithStop<T: Clone + 'static>(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut extraArg: T) -> Result<T> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut extraArg: T = extraArg;
    let mut name: ArcStr = arcstr::literal!("");
    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut continue_: bool = false;
    if '__try0: {
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
            if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
                e = unwrap_break_err!(ExpandableArray::get(i.clone(), equationArray.clone()), '__try0);
                (continue_, extraArg) = unwrap_break_err!(BackendEquation::traverseExpsOfEquation_WithStop(e.clone(), func.clone(), extraArg.clone()), '__try0);
                if !(continue_.clone()) {
                    break;
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            (_, _, name) = System::dladdr(func.clone());
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEUtil.traverseBackendDAEExpsEqnsWithStop failed for ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
        bail!("fail");
    }
    Ok(extraArg)
}

pub fn traverseBackendDAEExpsOptEqn<Type_a: Clone + 'static>(mut inEquation: Option<Arc<BackendDAE::Equation>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outTypeA: Type_a;
    (_, outTypeA) = traverseBackendDAEExpsOptEqnWithUpdate(inEquation.clone(), func.clone(), inTypeA.clone())?;
    Ok(outTypeA)
}

fn traverseBackendDAEExpsOptEqnWithUpdate<Type_a: Clone + 'static>(mut inEquation: Option<Arc<BackendDAE::Equation>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(Option<Arc<BackendDAE::Equation>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outEquation: Option<Arc<BackendDAE::Equation>> = None;
    let mut outTypeA: Type_a;
    (outEquation, outTypeA) = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Some(eqn1) => {
            let mut eqn2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut ext_arg_1: Type_a;
            (eqn2, ext_arg_1) = BackendEquation::traverseExpsOfEquation(eqn1.clone(), func.clone(), inTypeA.clone())?;
            (if (referenceEq(&eqn1.clone(),&eqn2.clone())) {inEquation.clone()} else {Some(eqn2.clone())}, ext_arg_1.clone())
        },
        _ => {
            (None, inTypeA.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEquation, outTypeA))
}

pub fn traverseAlgorithmExpsWithUpdate<Type_a: Clone + 'static>(mut inAlgorithm: Arc<DAE::Algorithm>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(Arc<DAE::Algorithm>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outAlgorithm: Arc<DAE::Algorithm> = Arc::new(<DAE::Algorithm as ::std::default::Default>::default());
    let mut outTypeA: Type_a;
    (outAlgorithm, outTypeA) = (::match_deref::match_deref! { match &(inAlgorithm.clone()) {
        Deref @ DAE::Algorithm { statementLst: stmts } => {
            let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut ext_arg_1: Type_a;
            let mut alg: Arc<DAE::Algorithm> = Arc::new(<DAE::Algorithm as ::std::default::Default>::default());
            (stmts1, ext_arg_1) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), func.clone(), inTypeA.clone());
            alg = if (referenceEq(&stmts.clone(),&stmts1.clone())) {inAlgorithm.clone()} else {Arc::new(DAE::Algorithm { statementLst: stmts1.clone() })};
            (alg.clone(), ext_arg_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outAlgorithm, outTypeA))
}

fn traverseZeroCrossingExps<Type_a: Clone + 'static>(mut iZeroCrossing: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a, mut iAcc: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>) -> Result<(Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut oZeroCrossing: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut outTypeA: Type_a;
    (oZeroCrossing, outTypeA) = (::match_deref::match_deref! { match &(iZeroCrossing.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iAcc.clone().reverse(), inTypeA.clone())
        },
        Deref @ metamodelica::List::Cons { head: zc @ BackendDAE::ZeroCrossing { .. }, tail: zeroCrossing } => {
            let mut relation1: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut zeroCrossing = (*zeroCrossing).clone();
            (relation1, arg) = Expression::traverseExpBottomUp(zc.relation_.clone(), func.clone(), inTypeA.clone())?;
            (zeroCrossing, arg) = traverseZeroCrossingExps(zeroCrossing.clone(), func.clone(), arg.clone(), cons(if (referenceEq(&relation1.clone(),&zc.relation_.clone())) {zc.clone()} else {BackendDAE::ZeroCrossing { index: zc.index.clone(), relation_: relation1.clone(), occurEquLst: zc.occurEquLst.clone(), iter: zc.iter.clone() }}, iAcc.clone()))?;
            (zeroCrossing.clone(), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oZeroCrossing, outTypeA))
}

/* ************************************************
 * Equation System Pipeline
 ************************************************/
pub fn getSolvedSystem(mut inDAE: Arc<BackendDAE::BackendDAE>, mut fileNamePrefix: ArcStr, mut strPreOptModules: Option<Arc<metamodelica::List<ArcStr>>>, mut strmatchingAlgorithm: Option<ArcStr>, mut strdaeHandler: Option<ArcStr>, mut strPostOptModules: Option<Arc<metamodelica::List<ArcStr>>>) -> Result<(Arc<BackendDAE::BackendDAE>, Arc<BackendDAE::BackendDAE>, Option<Arc<BackendDAE::BackendDAE>>, Option<BackendDAE::InlineData>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outSimDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut outInitDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut outInitDAE_lambda0_option: Option<Arc<BackendDAE::BackendDAE>> = None;
    let mut outInlineData: Option<BackendDAE::InlineData> = None;
    let mut outRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut dae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut simDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut outInitDAE_lambda0: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut preOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut postOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut daeHandler: (BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr);
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr);
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut numCheckpoints: i32 = 0;
    let mut oldSize: i32 = 0;
    let mut funcTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    numCheckpoints = ErrorExt::getNumCheckpoints();
    if '__try0: {
        StackOverflow::clearStacktraceMessages();
        preOptModules = unwrap_break_err!(getPreOptModules(strPreOptModules.clone()), '__try0);
        postOptModules = unwrap_break_err!(getPostOptModules(strPostOptModules.clone()), '__try0);
        matchingAlgorithm = unwrap_break_err!(getMatchingAlgorithm(strmatchingAlgorithm.clone()), '__try0);
        daeHandler = unwrap_break_err!(getIndexReductionMethod(strdaeHandler.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_DAE_LOW.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::OPT_DAE_DUMP.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpBackendDAE(inDAE.clone(), (literal!("dumpdaelow (before pre-optimization)")).clone()), '__try0);
            if unwrap_break_err!(Flags::isSet(Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone()), '__try0) {
                unwrap_break_err!(BackendDump::graphvizAdjacencyMatrix(inDAE.clone(), (literal!("dumpdaelow")).clone()), '__try0);
            }
        }
        dae = unwrap_break_err!(preOptimizeDAE(inDAE.clone(), preOptModules.clone()), '__try0);
        unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pre-optimization done (n=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", daeSize(dae.clone())))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try0);
        dae = unwrap_break_err!(causalizeDAE(dae.clone(), None, matchingAlgorithm.clone(), daeHandler.clone(), true), '__try0);
        unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("matching and sorting (n=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", daeSize(dae.clone())))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try0);
        dae = unwrap_break_err!(SynchronousFeatures::synchronousFeatures(dae.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::OPT_DAE_DUMP.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpBackendDAE(dae.clone(), (literal!("synchronousFeatures")).clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::GRAPHML.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpBipartiteGraphDAE(dae.clone(), (fileNamePrefix.clone()).clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::EVAL_OUTPUT_ONLY.clone()), '__try0) {
            oldSize = daeSize(dae.clone());
            dae = unwrap_break_err!(BackendDAEOptimize::evaluateOutputsOnly(dae.clone()), '__try0);
            unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evaluateOutputsOnly (n=")); __mm_s.push_str(&*intString(oldSize.clone())); __mm_s.push_str(&*literal!(" -> n=")); __mm_s.push_str(&*intString(daeSize(dae.clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try0);
        }
        dae = unwrap_break_err!(SymbolicJacobian::calculateStateSetsJacobians(dae.clone()), '__try0);
        (outInitDAE, outInitDAE_lambda0_option, outRemovedInitialEquationLst, globalKnownVars, dae) = unwrap_break_err!(Initialization::solveInitialSystem(dae.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::WARN_NO_NOMINAL.clone()), '__try0) {
            unwrap_break_err!(warnAboutIterationVariablesWithNoNominal(outInitDAE.clone()), '__try0);
        }
        simDAE = unwrap_break_err!(setFunctionTree(dae.clone(), getFunctions(outInitDAE.shared.clone())?), '__try0);
        simDAE = unwrap_break_err!(setDAEGlobalKnownVars(simDAE.clone(), globalKnownVars.clone()), '__try0);
        simDAE = unwrap_break_err!(BackendDAEOptimize::addInitialStmtsToAlgorithms(simDAE.clone(), false), '__try0);
        simDAE = unwrap_break_err!(Initialization::removeInitializationStuff(simDAE.clone()), '__try0);
        outInlineData = unwrap_break_err!(SymbolicImplicitSolver::symSolver(simDAE.clone()), '__try0);
        simDAE = unwrap_break_err!(postOptimizeDAE(simDAE.clone(), postOptModules.clone(), matchingAlgorithm.clone(), daeHandler.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::WARN_NO_NOMINAL.clone()), '__try0) {
            unwrap_break_err!(warnAboutIterationVariablesWithNoNominal(simDAE.clone()), '__try0);
        }
        simDAE = unwrap_break_err!(sortGlobalKnownVarsInDAE(simDAE.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("sort global known variables")).clone()), '__try0);
        funcTree = unwrap_break_err!(BackendDAEOptimize::copyRecordConstructorAndExternalObjConstructorDestructor(getFunctions(simDAE.shared.clone())?), '__try0);
        funcTree = unwrap_break_err!(BackendDAEOptimize::removeUnusedFunctions(outInitDAE.eqs.clone(), outInitDAE.shared.clone(), outRemovedInitialEquationLst.clone(), getFunctions(simDAE.shared.clone())?, funcTree.clone()), '__try0);
        if isSome(outInitDAE_lambda0_option.clone()) {
            let __pa1 = ::match_deref::match_deref! { match &(outInitDAE_lambda0_option.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            outInitDAE_lambda0 = __pa1.clone();
            funcTree = unwrap_break_err!(BackendDAEOptimize::removeUnusedFunctions(outInitDAE_lambda0.eqs.clone(), simDAE.shared.clone(), metamodelica::nil(), getFunctions(simDAE.shared.clone())?, funcTree.clone()), '__try0);
        }
        funcTree = unwrap_break_err!(BackendDAEOptimize::removeUnusedFunctions(simDAE.eqs.clone(), simDAE.shared.clone(), metamodelica::nil(), getFunctions(simDAE.shared.clone())?, funcTree.clone()), '__try0);
        outSimDAE = unwrap_break_err!(setFunctionTree(simDAE.clone(), funcTree.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("remove unused functions")).clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_INDX_DAE.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpBackendDAE(outSimDAE.clone(), (literal!("dumpindxdae")).clone()), '__try0);
            if unwrap_break_err!(Flags::isSet(Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone()), '__try0) {
                unwrap_break_err!(BackendDump::graphvizBackendDAE(outSimDAE.clone(), (literal!("dumpindxdae")).clone()), '__try0);
            }
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpCompShort(outSimDAE.clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_EQNINORDER.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpEqnsSolved(outSimDAE.clone(), (literal!("indxdae: eqns in order")).clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_LOOPS.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::DUMP_LOOPS_VERBOSE.clone()), '__try0) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BackendDump::BORDER)); __mm_s.push_str(&*literal!("\n\n Algbraic Loops (Simulation): \n\n")); __mm_s.push_str(&*arcstr::literal!(BackendDump::BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            unwrap_break_err!(BackendDump::dumpLoops(outSimDAE.clone()), '__try0);
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BackendDump::BORDER)); __mm_s.push_str(&*literal!("\n\n Algbraic Loops (Initialization): \n\n")); __mm_s.push_str(&*arcstr::literal!(BackendDump::BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            unwrap_break_err!(BackendDump::dumpLoops(outInitDAE.clone()), '__try0);
            if isSome(outInitDAE_lambda0_option.clone()) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BackendDump::BORDER)); __mm_s.push_str(&*literal!("\n\n Algbraic Loops (Initialization Lambda=0 (Homotopy)): \n\n")); __mm_s.push_str(&*arcstr::literal!(BackendDump::BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                unwrap_break_err!(BackendDump::dumpLoops(Util::getOption(outInitDAE_lambda0_option.clone())?), '__try0);
            }
        }
        unwrap_break_err!(checkBackendDAEWithErrorMsg(outSimDAE.clone()), '__try0);
        return Ok((outSimDAE.clone(), outInitDAE.clone(), outInitDAE_lambda0_option.clone(), outInlineData.clone(), outRemovedInitialEquationLst.clone()));
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        { let __v = None; openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v) };
        ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints.clone());
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow in ")); __mm_s.push_str(&*literal!("BackendDAEUtil.getSolvedSystem")); __mm_s.push_str(&*literal!("...\n")); __mm_s.push_str(&*stringDelimitList(StackOverflow::readableStacktraceMessages()?, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        StackOverflow::clearStacktraceMessages();
    }
    bail!("fail");
    Ok((outSimDAE, outInitDAE, outInitDAE_lambda0_option, outInlineData, outRemovedInitialEquationLst))
}

pub fn preOptimizeBackendDAE(mut inDAE: Arc<BackendDAE::BackendDAE>, mut strPreOptModules: Option<Arc<metamodelica::List<ArcStr>>>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut preOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    preOptModules = getPreOptModules(strPreOptModules.clone())?;
    outDAE = preOptimizeDAE(inDAE.clone(), preOptModules.clone())?;
    Ok(outDAE)
}

pub fn preOptimizeDAE(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inPreOptModules: Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut optModule: BackendDAEFunc::optimizationModule;
    let mut moduleStr: ArcStr = arcstr::literal!("");
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    execStat((literal!("prepare preOptimizeDAE")).clone())?;
    for mut preOptModule in &*inPreOptModules.clone() {
        let mut preOptModule = preOptModule.clone();
        (optModule, moduleStr) = preOptModule.clone();
        moduleStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*moduleStr.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*BackendDump::printBackendDAEType2String(inDAE.shared.backendDAEType.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        match '__try0: {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(optModule(outDAE.clone()), '__try0)) {
                Deref @ DAE { UNIQUEIO: __pa1, derivativeNamePrefix: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            systs = __pa1.clone();
            shared = __pa2.clone();
            (systs, shared) = unwrap_break_err!(filterEmptySystems(systs.clone(), shared.clone()), '__try0);
            outDAE = unwrap_break_err!(BackendDAE::DAE(systs.clone(), shared.clone()), '__try0);
            unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("preOpt ")); __mm_s.push_str(&*moduleStr.clone()); ArcStr::from(__mm_s) }).clone()), '__try0);
            if unwrap_break_err!(Flags::isSet(Flags::OPT_DAE_DUMP.clone()), '__try0) {
                unwrap_break_err!(BackendDump::dumpBackendDAE(outDAE.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pre-optimization module ")); __mm_s.push_str(&*moduleStr.clone()); ArcStr::from(__mm_s) }).clone()), '__try0);
            }
            Ok::<_, anyhow::Error>((outDAE.clone(), shared.clone(), systs.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                outDAE = __try0_o0;
                shared = __try0_o1;
                systs = __try0_o2;
            }
            Err(_) => {
                execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("preOpt ")); __mm_s.push_str(&*moduleStr.clone()); __mm_s.push_str(&*literal!(" <failed>")); ArcStr::from(__mm_s) }).clone())?;
                Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pre-optimization module ")); __mm_s.push_str(&*moduleStr.clone()); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone())?;
                bail!("fail");
            }
        }
    }
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        println!("{}", (literal!("pre-optimization done.\n")).clone());
    }
    Ok(outDAE)
}

pub fn transformBackendDAE(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inMatchingOptions: Option<(BackendDAE::IndexReduction, BackendDAE::EquationConstraints)>, mut strmatchingAlgorithm: Option<ArcStr>, mut strindexReductionMethod: Option<ArcStr>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr);
    let mut indexReductionMethod: (BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr);
    matchingAlgorithm = getMatchingAlgorithm(strmatchingAlgorithm.clone())?;
    indexReductionMethod = getIndexReductionMethod(strindexReductionMethod.clone())?;
    outDAE = causalizeDAE(inDAE.clone(), inMatchingOptions.clone(), matchingAlgorithm.clone(), indexReductionMethod.clone(), true)?;
    Ok(outDAE)
}

pub fn causalizeDAE(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inMatchingOptions: Option<(BackendDAE::IndexReduction, BackendDAE::EquationConstraints)>, mut matchingAlgorithm: (Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr), mut stateDeselection: (Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr, Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr), mut dolateinline: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut args: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>> = metamodelica::nil();
    let mut causalized: bool = false;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    (systs, shared, args, causalized) = mapCausalizeDAE(systs.clone(), shared.clone(), inMatchingOptions.clone(), matchingAlgorithm.clone(), stateDeselection.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
    outDAE = if (dolateinline.clone()) {BackendInline::lateInlineFunction(BackendDAE::DAE(systs.clone(), shared.clone())?)?} else {BackendDAE::DAE(systs.clone(), shared.clone())?};
    if causalized.clone() {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(stateDeselectionDAE(outDAE.clone(), args.clone(), stateDeselection.clone())) {
            Deref @ DAE { UNIQUEIO: __pa2, derivativeNamePrefix: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        systs = __pa2.clone();
        shared = __pa3.clone();
    }
    systs = mapSortEqnsDAE(systs.clone(), shared.clone())?;
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

fn mapCausalizeDAE(mut isysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut ishared: Arc<BackendDAE::Shared>, mut inMatchingOptions: Option<(BackendDAE::IndexReduction, BackendDAE::EquationConstraints)>, mut matchingAlgorithm: (Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr), mut stateDeselection: (Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr, Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr), mut acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut acc1: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>, mut iCausalized: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>, bool)> {
    let mut osysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut oargs: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>> = metamodelica::nil();
    let mut oCausalized: bool = false;
    (osysts, oshared, oargs, oCausalized) = (::match_deref::match_deref! { match &(isysts.clone()) {
        Deref @ metamodelica::List::Nil => {
            (acc.clone().reverse(), ishared.clone(), acc1.clone().reverse(), iCausalized.clone())
        },
        Deref @ metamodelica::List::Cons { head: syst, tail: systs } => {
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut arg: Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)> = None;
            let mut args: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>> = metamodelica::nil();
            let mut causalized: bool = false;
            let mut syst = (*syst).clone();
            let mut systs = (*systs).clone();
            (syst, shared, arg, causalized) = causalizeDAEWork(syst.clone(), ishared.clone(), inMatchingOptions.clone(), matchingAlgorithm.clone(), stateDeselection.clone(), iCausalized.clone())?;
            (systs, shared, args, causalized) = mapCausalizeDAE(systs.clone(), shared.clone(), inMatchingOptions.clone(), matchingAlgorithm.clone(), stateDeselection.clone(), cons(syst.clone(), acc.clone()), cons(arg.clone(), acc1.clone()), causalized.clone())?;
            (systs.clone(), shared.clone(), args.clone(), causalized.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osysts, oshared, oargs, oCausalized))
}

fn causalizeDAEWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMatchingOptions: Option<(BackendDAE::IndexReduction, BackendDAE::EquationConstraints)>, mut matchingAlgorithm: (Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr), mut stateDeselection: (Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr, Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr), mut iCausalized: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut oArg: Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)> = None;
    let mut oCausalized: bool = false;
    (osyst, oshared, oArg, oCausalized) = 'mc: {
        let __mc_input = (isyst.clone(), matchingAlgorithm.clone(), stateDeselection.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }, _, _) => {
                    Ok((isyst.clone(), ishared.clone(), None, iCausalized.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::NO_MATCHING, .. }, (matchingAlgorithmfunc, _), (sssHandler, _, _, _)) => {
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut match_opts: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints);
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
                    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    funcs = getFunctions(ishared.clone())?;
                    (syst, _, _, mapEqnIncRow, mapIncRowEqn) = getAdjacencyMatrixScalar(isyst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), isInitializationDAE(ishared.clone()))?;
                    match_opts = Util::getOptionOrDefault(inMatchingOptions.clone(), (crate::BackendDAE::IndexReduction::INDEX_REDUCTION, crate::BackendDAE::EquationConstraints::EXACT));
                    arg = IndexReduction::getStructurallySingularSystemHandlerArg(syst.clone(), ishared.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
                    nvars = BackendVariable::daenumVariables(syst.clone())?;
                    neqns = systemSize(syst.clone());
                    syst = Causalize::singularSystemCheck(nvars.clone(), neqns.clone(), syst.clone(), match_opts.clone(), matchingAlgorithm.clone(), arg.clone(), ishared.clone())?;
                    (syst, shared, arg) = matchingAlgorithmfunc(syst.clone(), ishared.clone(), false, match_opts.clone(), sssHandler.clone(), arg.clone())?;
                    Ok((syst.clone(), shared.clone(), Some(arg.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, mAmethodstr), (_, str1, _, _)) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Transformation Module ")); __mm_s.push_str(&*mAmethodstr.clone()); __mm_s.push_str(&*literal!(" index Reduction Method ")); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone();
                    if !(isInitializationDAE(ishared.clone())) {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, oArg, oCausalized))
}

fn stateDeselectionDAE(mut inDAE: Arc<BackendDAE::BackendDAE>, mut args: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>, mut stateDeselection: (Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr, Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)) -> Arc<BackendDAE::BackendDAE> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut methodstr: ArcStr = arcstr::literal!("");
    let mut sDfunc: BackendDAEFunc::stateDeselectionFunc;
    (_, _, sDfunc, methodstr) = stateDeselection.clone();
    outDAE = sDfunc(inDAE.clone(), args.clone()).unwrap();
    outDAE
}

fn mapSortEqnsDAE(mut inSystem: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>> {
    let mut outSystem: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    outSystem = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
        for mut syst in (inSystem.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. } => syst.clone(),
        _ => sortEqnsDAEWork(syst.clone(), inShared.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outSystem)
}

fn sortEqnsDAEWork(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    match '__try0: {
        funcs = unwrap_break_err!(getFunctions(inShared.clone()), '__try0);
        (syst, _, _, mapEqnIncRow, mapIncRowEqn) = unwrap_break_err!(getAdjacencyMatrixScalar(inSystem.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), isInitializationDAE(inShared.clone())), '__try0);
        (outSystem, _) = unwrap_break_err!(BackendDAETransform::strongComponentsScalar(syst.clone(), inShared.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SCC_GRAPHML.clone()), '__try0) {
            unwrap_break_err!(dumpStrongComponents(outSystem.clone(), inShared.clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((funcs.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), outSystem.clone(), syst.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            funcs = __try0_o0;
            mapEqnIncRow = __try0_o1;
            mapIncRowEqn = __try0_o2;
            outSystem = __try0_o3;
            syst = __try0_o4;
        }
        Err(_) => {
            Error::addInternalError((literal!("Transformation module sort components failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok(outSystem)
}

fn dumpStrongComponents(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ishared.clone()) {
        Deref @ BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix, .. }, .. } => {
            let mut fileName: ArcStr = arcstr::literal!("");
            let mut seqNo: i32 = 0;
            seqNo = System::tmpTickIndex(Global::backendDAE_fileSequence.clone());
            fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(seqNo.clone())); __mm_s.push_str(&*literal!("_Comps")); __mm_s.push_str(&*intString(systemSize(isyst.clone()))); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone();
            DumpGraphML::dumpSystem(isyst.clone(), ishared.clone(), None, (fileName.clone()).clone(), false)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn postOptimizeDAE(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inPostOptModules: Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>, mut inMatchingAlgorithm: (Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr), mut inDAEHandler: (Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr, Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut optModule: BackendDAEFunc::optimizationModule;
    let mut moduleStr: ArcStr = arcstr::literal!("");
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let debug: bool = false;
    execStat((literal!("prepare postOptimizeDAE")).clone())?;
    for mut postOptModule in &*inPostOptModules.clone() {
        let mut postOptModule = postOptModule.clone();
        (optModule, moduleStr) = postOptModule.clone();
        moduleStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*moduleStr.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*BackendDump::printBackendDAEType2String(inDAE.shared.backendDAEType.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        match '__try0: {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(optModule(outDAE.clone()), '__try0)) {
                Deref @ DAE { UNIQUEIO: __pa1, derivativeNamePrefix: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            systs = __pa1.clone();
            shared = __pa2.clone();
            (systs, shared) = unwrap_break_err!(filterEmptySystems(systs.clone(), shared.clone()), '__try0);
            outDAE = unwrap_break_err!(BackendDAE::DAE(systs.clone(), shared.clone()), '__try0);
            if debug.clone() {
                unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("postOpt ")); __mm_s.push_str(&*moduleStr.clone()); ArcStr::from(__mm_s) }).clone()), '__try0);
            }
            outDAE = unwrap_break_err!(causalizeDAE(outDAE.clone(), Some((crate::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, crate::BackendDAE::EquationConstraints::EXACT)), inMatchingAlgorithm.clone(), inDAEHandler.clone(), false), '__try0);
            unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("postOpt ")); __mm_s.push_str(&*if (debug.clone()) {literal!("causalize ")} else {literal!("")}); __mm_s.push_str(&*moduleStr.clone()); ArcStr::from(__mm_s) }).clone()), '__try0);
            if unwrap_break_err!(Flags::isSet(Flags::OPT_DAE_DUMP.clone()), '__try0) {
                unwrap_break_err!(BackendDump::dumpBackendDAE(outDAE.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("post-optimization module ")); __mm_s.push_str(&*moduleStr.clone()); ArcStr::from(__mm_s) }).clone()), '__try0);
            }
            Ok::<_, anyhow::Error>((outDAE.clone(), shared.clone(), systs.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                outDAE = __try0_o0;
                shared = __try0_o1;
                systs = __try0_o2;
            }
            Err(_) => {
                execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<failed> postOpt ")); __mm_s.push_str(&*moduleStr.clone()); ArcStr::from(__mm_s) }).clone())?;
                Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("post-optimization module ")); __mm_s.push_str(&*moduleStr.clone()); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone())?;
                bail!("fail");
            }
        }
    }
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        println!("{}", (literal!("post-optimization done.\n")).clone());
    }
    Ok(outDAE)
}

pub fn getSolvedSystemforJacobians(mut inDAE: Arc<BackendDAE::BackendDAE>, mut strPreOptModules: Arc<metamodelica::List<ArcStr>>, mut strMatchingAlgorithm: Option<ArcStr>, mut strDAEHandler: Option<ArcStr>, mut strPostOptModules: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut dae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut preOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut postOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut daeHandler: (BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr);
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr);
    preOptModules = selectOptModules(strPreOptModules.clone(), metamodelica::nil(), metamodelica::nil(), allPreOptimizationModules())?;
    postOptModules = selectOptModules(strPostOptModules.clone(), metamodelica::nil(), metamodelica::nil(), allPostOptimizationModules())?;
    matchingAlgorithm = getMatchingAlgorithm(strMatchingAlgorithm.clone())?;
    daeHandler = getIndexReductionMethod(strDAEHandler.clone())?;
    dae = preOptimizeDAE(inDAE.clone(), preOptModules.clone())?;
    dae = causalizeDAE(dae.clone(), Some((crate::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, crate::BackendDAE::EquationConstraints::EXACT)), matchingAlgorithm.clone(), daeHandler.clone(), true)?;
    execStat((literal!("causalizeDAE (first run)")).clone())?;
    outDAE = postOptimizeDAE(dae.clone(), postOptModules.clone(), matchingAlgorithm.clone(), daeHandler.clone())?;
    Ok(outDAE)
}

pub fn sortGlobalKnownVarsInDAE(mut backendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut backendDAE: Arc<BackendDAE::BackendDAE> = backendDAE;
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVars_sorted: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut parameterEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut paramSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut flatComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    globalKnownVars = backendDAE.shared.globalKnownVars.clone();
    parameterEqns = BackendEquation::emptyEqnsSized(BackendVariable::varsSize(globalKnownVars.clone())?);
    parameterEqns = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(createGlobalKnownVarsEquations) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> + 'static>), parameterEqns.clone())?;
    paramSystem = createEqSystem(globalKnownVars.clone(), parameterEqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (m, _) = adjacencyMatrix(paramSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, isInitializationDAE(backendDAE.shared.clone()))?;
    (ass1, ass2) = Matching::PerfectMatching(m.clone())?;
    comps = Sorting::Tarjan(m.clone(), ass1.clone(), (ass1.clone().borrow().len() as i32))?;
    flatComps = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut comp in (comps.clone()).into_iter().cloned() {
            let __x = Initialization::flattenParamComp(comp.clone(), globalKnownVars.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    globalKnownVars_sorted = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    for mut i in &*flatComps.clone() {
        let mut i = i.clone();
        var = BackendVariable::getVarAt(globalKnownVars.clone(), i.clone())?;
        globalKnownVars_sorted = BackendVariable::addVar(var.clone(), globalKnownVars_sorted.clone())?;
    }
    backendDAE = setDAEGlobalKnownVars(backendDAE.clone(), globalKnownVars_sorted.clone())?;
    execStat((literal!("sorting global known variables")).clone())?;
    Ok(backendDAE)
}

fn createGlobalKnownVarsEquations(mut var: BackendDAE::Var, mut parameterEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut var: BackendDAE::Var = var;
    let mut parameterEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = parameterEqns;
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    lhs = BackendVariable::varExp(var.clone())?;
    rhs = BackendVariable::varBindExpStartValueNoFail(var.clone())?;
    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() });
    parameterEqns = BackendEquation::add(eqn.clone(), parameterEqns.clone())?;
    Ok((var, parameterEqns))
}

pub fn getEqnIndexArray(mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut eqIdxArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut idx: i32 = 1;
    let mut idx2: i32 = 0;
    let mut size: i32 = 0;
    let mut eqIdxLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    eqIdxArray = arrayCreate(BackendEquation::getNumberOfEquations(eqs.clone()), metamodelica::nil());
    for mut eq in &*BackendEquation::equationList(eqs.clone()) {
        let mut eq = eq.clone();
        size = BackendEquation::equationSize(BackendEquation::get(eqs.clone(), idx.clone()))?;
        eqIdxLst = List::map1(List::intRange(size.clone()), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), idx2.clone());
        {let _arr = eqIdxArray.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = eqIdxLst.clone(); _arr};
        idx = idx.clone() + 1;
        idx2 = size.clone() + idx2.clone();
    }
    Ok(eqIdxArray)
}

pub fn analyticalToStructuralSingularity(mut comp: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut syst: Arc<BackendDAE::EqSystem>, mut changed: bool, mut init: bool) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, bool)> {
    let mut ass1: metamodelica::Array<i32> = ass1;
    let mut ass2: metamodelica::Array<i32> = ass2;
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut changed: bool = changed;
    let mut mapArrayToScalar: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapScalarToArray: metamodelica::Array<i32>;
    let mut loopEqs: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, (i32, i32))>> = metamodelica::nil();
    let mut loopVars: Arc<metamodelica::List<(BackendDAE::Var, i32)>> = metamodelica::nil();
    let mut tmp_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut linJac: Arc<SymbolicJacobian::LinearJacobian::LinearJacobian> = Arc::new(<SymbolicJacobian::LinearJacobian::LinearJacobian as ::std::default::Default>::default());
    if (comp.clone().len() as i32) > 1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.mapping.clone()) {
            Some((__pa0, __pa1, _, _, _)) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        mapArrayToScalar = __pa0.clone();
        mapScalarToArray = __pa1.clone();
        for mut eqnIndex in &*comp.clone() {
            let mut eqnIndex = eqnIndex.clone();
            if (mapArrayToScalar.borrow()[(mapScalarToArray.borrow()[(eqnIndex.clone()-1) as usize].clone()-1) as usize].clone().len() as i32) == 1 {
                tmp_eq = BackendEquation::get(syst.orderedEqs.clone(), mapScalarToArray.borrow()[(eqnIndex.clone()-1) as usize].clone());
                loopEqs = cons((tmp_eq.clone(), (mapScalarToArray.borrow()[(eqnIndex.clone()-1) as usize].clone(), eqnIndex.clone())), loopEqs.clone());
            }
            loopVars = cons((BackendVariable::getVarAt(syst.orderedVars.clone(), ass1.borrow()[(eqnIndex.clone()-1) as usize].clone())?, ass1.borrow()[(eqnIndex.clone()-1) as usize].clone()), loopVars.clone());
        }
        if !(loopEqs.clone().is_empty()) && (loopEqs.clone().len() as i32) <= Flags::getConfigInt(Flags::MAX_SIZE_ASSC.clone())? {
            if '__try2: {
                linJac = unwrap_break_err!(SymbolicJacobian::LinearJacobian::generate(loopEqs.clone(), loopVars.clone(), ass1.clone()), '__try2);
                if !(SymbolicJacobian::LinearJacobian::emptyOrSingle(linJac.clone())) {
                    if unwrap_break_err!(Flags::isSet(Flags::DUMP_ASSC.clone()), '__try2) {
                        println!("{}", (unwrap_break_err!(SymbolicJacobian::LinearJacobian::toString(linJac.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Original (initial: ")); __mm_s.push_str(&*boolString(init.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try2)).clone());
                    }
                    linJac = SymbolicJacobian::LinearJacobian::solve(linJac.clone());
                    if unwrap_break_err!(Flags::isSet(Flags::DUMP_ASSC.clone()), '__try2) {
                        println!("{}", (unwrap_break_err!(SymbolicJacobian::LinearJacobian::toString(linJac.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Solved (initial: ")); __mm_s.push_str(&*boolString(init.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try2)).clone());
                    }
                    changed = changed.clone() || SymbolicJacobian::LinearJacobian::anyChanges(linJac.clone());
                    (ass1, ass2, syst) = unwrap_break_err!(SymbolicJacobian::LinearJacobian::resolveASSC(linJac.clone(), ass1.clone(), ass2.clone(), syst.clone(), init.clone()), '__try2);
                }
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
    }
    Ok((ass1, ass2, syst, changed))
}

/* ************************************************
 * index reduction method Selection
 ************************************************/
pub fn getIndexReductionMethodString() -> Result<ArcStr> {
    let mut strIndexReductionMethod: ArcStr = arcstr::literal!("");
    strIndexReductionMethod = (Config::getIndexReductionMethod()?).clone();
    Ok(strIndexReductionMethod)
}

pub fn getIndexReductionMethod(mut ostrIndexReductionMethod: Option<ArcStr>) -> Result<(Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr, Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)> {
    let mut IndexReductionMethod: (BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr);
    let mut allIndexReductionMethods: Arc<metamodelica::List<(BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr)>> = metamodelica::nil();
    let mut strIndexReductionMethod: ArcStr = arcstr::literal!("");
    allIndexReductionMethods = list![((std::sync::Arc::new(IndexReduction::failIfIndexReduction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("none"), (std::sync::Arc::new(fnptr!(IndexReduction::noStateDeselection, Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("none")), ((std::sync::Arc::new(IndexReduction::pantelidesIndexReduction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("Pantelides"), (std::sync::Arc::new(fnptr!(IndexReduction::noStateDeselection, Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("uode")), ((std::sync::Arc::new(IndexReduction::pantelidesIndexReduction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("Pantelides"), (std::sync::Arc::new(IndexReduction::dynamicStateSelection) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dynamicStateSelection")), ((std::sync::Arc::new(IndexReduction::pantelidesIndexReduction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("Pantelides"), (std::sync::Arc::new(IndexReduction::dynamicStateSelection) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dummyDerivatives"))];
    strIndexReductionMethod = (getIndexReductionMethodString()?).clone();
    strIndexReductionMethod = (Util::getOptionOrDefault(ostrIndexReductionMethod.clone(), (strIndexReductionMethod.clone()).clone())).clone();
    IndexReductionMethod = selectIndexReductionMethod((strIndexReductionMethod.clone()).clone(), allIndexReductionMethods.clone())?;
    Ok(IndexReductionMethod)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn selectIndexReductionMethod<Type_a: Clone + 'static, Type_b: Clone + 'static>(mut strIndexReductionMethod: ArcStr, mut inIndexReductionMethods: Arc<metamodelica::List<(Type_a, ArcStr, Type_b, ArcStr)>>) -> Result<(Type_a, ArcStr, Type_b, ArcStr)> {
    let mut outIndexReductionMethod: (Type_a, ArcStr, Type_b, ArcStr);
    outIndexReductionMethod = 'mc: {
        let __mc_input = inIndexReductionMethods.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: method @ (_, _, _, name), tail: _ } => {
                    if !((stringEqual((strIndexReductionMethod.clone()).clone(), (name.clone()).clone()))) { bail!("guard") }
                    Ok(method.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: methods } => {
                    let mut method: (Type_a, ArcStr, Type_b, ArcStr);
                    method = selectIndexReductionMethod((strIndexReductionMethod.clone()).clone(), methods.clone())?;
                    Ok(method.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = stringAppendList(list![(literal!("Selection of Index Reduction Method ")).clone(), (strIndexReductionMethod.clone()).clone(), (literal!(" failed.")).clone()]);
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIndexReductionMethod)
}

/* ************************************************
 * matching Algorithm Selection
 ************************************************/
pub fn getMatchingAlgorithmString() -> Result<ArcStr> {
    let mut strMatchingAlgorithm: ArcStr = arcstr::literal!("");
    strMatchingAlgorithm = (Config::getMatchingAlgorithm()?).clone();
    Ok(strMatchingAlgorithm)
}

pub fn getMatchingAlgorithm(mut ostrMatchingAlgorithm: Option<ArcStr>) -> Result<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, ArcStr)> {
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr);
    let mut allMatchingAlgorithms: Arc<metamodelica::List<(BackendDAEFunc::matchingAlgorithmFunc, ArcStr)>> = metamodelica::nil();
    let mut strMatchingAlgorithm: ArcStr = arcstr::literal!("");
    allMatchingAlgorithms = list![((std::sync::Arc::new(Matching::BFSB) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("BFSB")), ((std::sync::Arc::new(Matching::DFSB) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("DFSB")), ((std::sync::Arc::new(Matching::MC21A) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("MC21A")), ((std::sync::Arc::new(Matching::PF) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("PF")), ((std::sync::Arc::new(Matching::PFPlus) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("PFPlus")), ((std::sync::Arc::new(Matching::HK) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("HK")), ((std::sync::Arc::new(Matching::HKDW) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("HKDW")), ((std::sync::Arc::new(Matching::ABMP) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("ABMP")), ((std::sync::Arc::new(Matching::PR_FIFO_FAIR) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("PR")), ((std::sync::Arc::new(Matching::DFSBExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("DFSBExt")), ((std::sync::Arc::new(Matching::BFSBExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("BFSBExt")), ((std::sync::Arc::new(Matching::MC21AExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("MC21AExt")), ((std::sync::Arc::new(Matching::PFExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("PFExt")), ((std::sync::Arc::new(Matching::PFPlusExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("PFPlusExt")), ((std::sync::Arc::new(Matching::HKExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("HKExt")), ((std::sync::Arc::new(Matching::HKDWExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("HKDWExt")), ((std::sync::Arc::new(Matching::ABMPExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("ABMPExt")), ((std::sync::Arc::new(Matching::PR_FIFO_FAIRExternal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("PRExt")), ((std::sync::Arc::new(Matching::BBMatching) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), BackendDAEFunc::StructurallySingularSystemHandlerFunc, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), literal!("BB"))];
    strMatchingAlgorithm = (getMatchingAlgorithmString()?).clone();
    strMatchingAlgorithm = (Util::getOptionOrDefault(ostrMatchingAlgorithm.clone(), (strMatchingAlgorithm.clone()).clone())).clone();
    matchingAlgorithm = selectMatchingAlgorithm((strMatchingAlgorithm.clone()).clone(), allMatchingAlgorithms.clone())?;
    Ok(matchingAlgorithm)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn selectMatchingAlgorithm<Type_a: Clone + 'static>(mut strMatchingAlgorithm: ArcStr, mut inMatchingAlgorithms: Arc<metamodelica::List<(Type_a, ArcStr)>>) -> Result<(Type_a, ArcStr)> {
    let mut outMatchingAlgorithm: (Type_a, ArcStr);
    outMatchingAlgorithm = 'mc: {
        let __mc_input = inMatchingAlgorithms.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: method @ (_, name), tail: _ } => {
                    if !((stringEqual((strMatchingAlgorithm.clone()).clone(), (name.clone()).clone()))) { bail!("guard") }
                    Ok(method.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: methods } => {
                    let mut method: (Type_a, ArcStr);
                    method = selectMatchingAlgorithm((strMatchingAlgorithm.clone()).clone(), methods.clone())?;
                    Ok(method.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = stringAppendList(list![(literal!("Selection of Matching Algorithm ")).clone(), (strMatchingAlgorithm.clone()).clone(), (literal!(" failed.")).clone()]);
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMatchingAlgorithm)
}

// =============================================================================
// Optimization module selection
//
// =============================================================================
pub fn allPreOptimizationModules() -> Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>> {
    let mut allPreOptimizationModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = list![((std::sync::Arc::new(introduceOutputRealDerivatives) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("introduceOutputRealDerivatives")), ((std::sync::Arc::new(introduceOutputAliases) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("introduceOutputAliases")), ((std::sync::Arc::new(DataReconciliation::newExtractionAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dataReconciliation")), ((std::sync::Arc::new(DataReconciliation::extractBoundaryCondition) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dataReconciliationBoundaryConditions")), ((std::sync::Arc::new(DataReconciliation::stateEstimation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dataReconciliationStateEstimation")), ((std::sync::Arc::new(DynamicOptimization::createDynamicOptimization) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("createDynamicOptimization")), ((std::sync::Arc::new(BackendInline::normalInlineFunction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("normalInlineFunction")), ((std::sync::Arc::new(EvaluateParameter::evaluateParameters) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("evaluateParameters")), ((std::sync::Arc::new(RemoveSimpleEquations::removeVerySimpleEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeVerySimpleEquations")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyIfEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyIfEquations")), ((std::sync::Arc::new(BackendDAEOptimize::expandDerOperator) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("expandDerOperator")), ((std::sync::Arc::new(BackendDAEOptimize::removeLocalKnownVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeLocalKnownVars")), ((std::sync::Arc::new(CommonSubExpression::wrapFunctionCalls) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("wrapFunctionCalls")), ((std::sync::Arc::new(SynchronousFeatures::clockPartitioning) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("clockPartitioning")), ((std::sync::Arc::new(IndexReduction::findStateOrder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("findStateOrder")), ((std::sync::Arc::new(BackendDAEOptimize::introduceDerAlias) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("introduceDerAlias")), ((std::sync::Arc::new(DynamicOptimization::inputDerivativesForDynOpt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("inputDerivativesForDynOpt")), ((std::sync::Arc::new(BackendDAEOptimize::replaceEdgeChange) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("replaceEdgeChange")), ((std::sync::Arc::new(InlineArrayEquations::inlineArrayEqn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("inlineArrayEqn")), ((std::sync::Arc::new(BackendDAEOptimize::sortEqnsVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("sortEqnsVars")), ((std::sync::Arc::new(BackendDAEOptimize::removeEqualRHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeEqualRHS")), ((std::sync::Arc::new(RemoveSimpleEquations::removeSimpleEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeSimpleEquations")), ((std::sync::Arc::new(CommonSubExpression::commonSubExpressionReplacement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("comSubExp")), ((std::sync::Arc::new(fnptr!(ResolveLoops::resolveLoops, Arc<BackendDAE::BackendDAE>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("resolveLoops")), ((std::sync::Arc::new(fnptr!(EvaluateFunctions::evalFunctions, Arc<BackendDAE::BackendDAE>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("evalFunc")), ((std::sync::Arc::new(FindZeroCrossings::encapsulateWhenConditions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("encapsulateWhenConditions")), ((std::sync::Arc::new(BackendDAEOptimize::removeProtectedParameters) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeProtectedParameters")), ((std::sync::Arc::new(BackendDAEOptimize::removeUnusedParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeUnusedParameter")), ((std::sync::Arc::new(BackendDAEOptimize::removeUnusedVariables) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeUnusedVariables")), ((std::sync::Arc::new(BackendDAEOptimize::residualForm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("residualForm")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyAllExpressions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyAllExpressions")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyInStream) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyInStream")), ((std::sync::Arc::new(BackendDump::dumpDAE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dumpDAE")), ((std::sync::Arc::new(XMLDump::dumpDAEXML) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dumpDAEXML")), ((std::sync::Arc::new(BackendDAETransform::collapseArrayExpressions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("collapseArrayExpressions"))];
    allPreOptimizationModules
}

pub fn allPostOptimizationModules() -> Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>> {
    let mut allPostOptimizationModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = list![((std::sync::Arc::new(BackendInline::lateInlineFunction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("lateInlineFunction")), ((std::sync::Arc::new(DynamicOptimization::simplifyConstraints) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyConstraints")), ((std::sync::Arc::new(CommonSubExpression::wrapFunctionCalls) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("wrapFunctionCalls")), ((std::sync::Arc::new(CommonSubExpression::cseBinary) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("cseBinary")), ((std::sync::Arc::new(BackendDAEOptimize::replaceDerCalls) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("replaceDerCalls")), ((std::sync::Arc::new(OnRelaxation::relaxSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("relaxSystem")), ((std::sync::Arc::new(InlineArrayEquations::inlineArrayEqn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("inlineArrayEqn")), ((std::sync::Arc::new(SymbolicJacobian::constantLinearSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("constantLinearSystem")), ((std::sync::Arc::new(BackendDAEOptimize::simplifysemiLinear) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifysemiLinear")), ((std::sync::Arc::new(ResolveLoops::solveLinearSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("solveLinearSystem")), ((std::sync::Arc::new(BackendDAEOptimize::addedScaledVars_states) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("addScaledVars_states")), ((std::sync::Arc::new(BackendDAEOptimize::addedScaledVars_inputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("addScaledVars_inputs")), ((std::sync::Arc::new(RemoveSimpleEquations::removeSimpleEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeSimpleEquations")), ((std::sync::Arc::new(BackendDAEOptimize::inlineFunctionInLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("forceInlineFunctionInLoops")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyComplexFunction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyComplexFunction")), ((std::sync::Arc::new(ExpressionSolve::solveSimpleEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("solveSimpleEquations")), ((std::sync::Arc::new(ResolveLoops::reshuffling_post) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("reshufflePost")), ((std::sync::Arc::new(DynamicOptimization::reduceDynamicOptimization) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("reduceDynamicOptimization")), ((std::sync::Arc::new(Tearing::tearingSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("tearingSystem")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyLoops")), ((std::sync::Arc::new(Tearing::recursiveTearing) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("recursiveTearing")), ((std::sync::Arc::new(HpcOmEqSystems::partitionLinearTornSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("partlintornsystem")), ((std::sync::Arc::new(BackendDAEOptimize::countOperations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("countOperations")), ((std::sync::Arc::new(SymbolicJacobian::inputDerivativesUsed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("inputDerivativesUsed")), ((std::sync::Arc::new(DynamicOptimization::removeLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("extendDynamicOptimization")), ((std::sync::Arc::new(BackendDAEOptimize::addTimeAsState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("addTimeAsState")), ((std::sync::Arc::new(fnptr!(SymbolicJacobian::calculateStrongComponentJacobians, Arc<BackendDAE::BackendDAE>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("calculateStrongComponentJacobians")), ((std::sync::Arc::new(SymbolicJacobian::calculateStateSetsJacobians) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("calculateStateSetsJacobians")), ((std::sync::Arc::new(SymbolicJacobian::symbolicJacobian) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("symbolicJacobian")), ((std::sync::Arc::new(SymbolicJacobian::generateSymbolicSensitivities) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("generateSymbolicSensitivities")), ((std::sync::Arc::new(SymbolicJacobian::generateSymbolicLinearizationPast) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("generateSymbolicLinearization")), ((std::sync::Arc::new(BackendDAEOptimize::removeConstants) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("removeConstants")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyTimeIndepFuncCalls) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyTimeIndepFuncCalls")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyAllExpressions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyAllExpressions")), ((std::sync::Arc::new(BackendDAEOptimize::hets) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("hets")), ((std::sync::Arc::new(FindZeroCrossings::findZeroCrossings) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("findZeroCrossings")), ((std::sync::Arc::new(BackendDump::dumpComponentsGraphStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dumpComponentsGraphStr")), ((std::sync::Arc::new(BackendDump::dumpDAE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dumpDAE")), ((std::sync::Arc::new(XMLDump::dumpDAEXML) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("dumpDAEXML")), ((std::sync::Arc::new(BackendDAETransform::collapseArrayExpressions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("collapseArrayExpressions")), ((std::sync::Arc::new(DAEMode::createDAEmodeBDAE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("createDAEmodeBDAE")), ((std::sync::Arc::new(SymbolicJacobian::symbolicJacobianDAE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("symbolicJacobianDAE")), ((std::sync::Arc::new(setEvaluationStage) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("setEvaluationStage"))];
    allPostOptimizationModules
}

fn allInitOptimizationModules() -> Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>> {
    let mut allInitOptimizationModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = list![((std::sync::Arc::new(Initialization::replaceHomotopyWithSimplified) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("replaceHomotopyWithSimplified")), ((std::sync::Arc::new(InlineArrayEquations::inlineArrayEqn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("inlineArrayEqn")), ((std::sync::Arc::new(SymbolicJacobian::constantLinearSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("constantLinearSystem")), ((std::sync::Arc::new(BackendDAEOptimize::inlineHomotopy) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("inlineHomotopy")), ((std::sync::Arc::new(BackendDAEOptimize::inlineFunctionInLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("forceInlineFunctionInLoops")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyComplexFunction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyComplexFunction")), ((std::sync::Arc::new(CommonSubExpression::wrapFunctionCalls) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("wrapFunctionCalls")), ((std::sync::Arc::new(DynamicOptimization::reduceDynamicOptimization) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("reduceDynamicOptimization")), ((std::sync::Arc::new(Tearing::tearingSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("tearingSystem")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyLoops")), ((std::sync::Arc::new(Tearing::recursiveTearing) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("recursiveTearing")), ((std::sync::Arc::new(ExpressionSolve::solveSimpleEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("solveSimpleEquations")), ((std::sync::Arc::new(BackendDAEOptimize::generateHomotopyComponents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("generateHomotopyComponents")), ((std::sync::Arc::new(fnptr!(SymbolicJacobian::calculateStrongComponentJacobians, Arc<BackendDAE::BackendDAE>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("calculateStrongComponentJacobians")), ((std::sync::Arc::new(BackendDAEOptimize::simplifyAllExpressions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("simplifyAllExpressions")), ((std::sync::Arc::new(SymbolicJacobian::inputDerivativesUsed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("inputDerivativesUsed")), ((std::sync::Arc::new(DynamicOptimization::removeLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("extendDynamicOptimization")), ((std::sync::Arc::new(BackendDAETransform::collapseArrayExpressions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>), literal!("collapseArrayExpressions"))];
    allInitOptimizationModules
}

pub fn getPreOptModulesString() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strPreOptModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strPreOptModules = Config::getPreOptModules()?;
    Ok(strPreOptModules)
}

fn deprecatedDebugFlag(mut inFlag: Flags::DebugFlag, mut inModuleList: Arc<metamodelica::List<ArcStr>>, mut inModule: ArcStr, mut inPhase: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outModuleList: Arc<metamodelica::List<ArcStr>> = inModuleList.clone();
    if Flags::isSet(inFlag.clone())? {
        outModuleList = cons((inModule.clone()).clone(), inModuleList.clone());
        Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Deprecated debug flag -d=")); __mm_s.push_str(&*FlagsUtil::debugFlagName(inFlag.clone())?); __mm_s.push_str(&*literal!(" detected. Use --")); __mm_s.push_str(&*inPhase.clone()); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*inModule.clone()); __mm_s.push_str(&*literal!(" instead.")); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(outModuleList)
}

fn deprecatedConfigFlag(mut inFlag: Flags::ConfigFlag, mut inModuleList: Arc<metamodelica::List<ArcStr>>, mut inModule: ArcStr, mut inPhase: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outModuleList: Arc<metamodelica::List<ArcStr>> = inModuleList.clone();
    if Flags::getConfigBool(inFlag.clone())? {
        outModuleList = cons((inModule.clone()).clone(), inModuleList.clone());
        Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Deprecated flag --")); __mm_s.push_str(&*FlagsUtil::configFlagName(inFlag.clone())?); __mm_s.push_str(&*literal!(" detected. Use --")); __mm_s.push_str(&*inPhase.clone()); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*inModule.clone()); __mm_s.push_str(&*literal!(" instead.")); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(outModuleList)
}

pub fn getPreOptModules(mut inPreOptModules: Option<Arc<metamodelica::List<ArcStr>>>) -> Result<Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>> {
    let mut outPreOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut preOptModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut enabledModules: Arc<metamodelica::List<ArcStr>> = Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())?;
    let mut disabledModules: Arc<metamodelica::List<ArcStr>> = Flags::getConfigStringList(Flags::PRE_OPT_MODULES_SUB.clone())?;
    preOptModules = getPreOptModulesString()?;
    preOptModules = Util::getOptionOrDefault(inPreOptModules.clone(), preOptModules.clone());
    if isSome(openmodelica_util::Globals::isInStream.with(|__root| __root.borrow().clone())) {
        enabledModules = cons((literal!("simplifyInStream")).clone(), enabledModules.clone());
    }
    if Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())? {
        enabledModules = deprecatedDebugFlag(Flags::SORT_EQNS_AND_VARS.clone(), enabledModules.clone(), (literal!("sortEqnsVars")).clone(), (literal!("preOptModules+")).clone())?;
        if Config::acceptOptimicaGrammar()? || Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())? {
            enabledModules = cons((literal!("inputDerivativesForDynOpt")).clone(), enabledModules.clone());
            enabledModules = cons((literal!("createDynamicOptimization")).clone(), enabledModules.clone());
        }
        if Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())? == literal!("causal") || Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())? == literal!("none") {
            disabledModules = cons((literal!("removeSimpleEquations")).clone(), disabledModules.clone());
        }
        if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
            disabledModules = cons((literal!("inlineArrayEqn")).clone(), disabledModules.clone());
        }
    }
    if !(Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?) && !(enabledModules.clone().is_empty()) {
        Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It's not possible to combine following flags: --preOptModules+=... and --")); __mm_s.push_str(&*FlagsUtil::configFlagName(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?); __mm_s.push_str(&*literal!("=false")); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    if !(Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?) && !(disabledModules.clone().is_empty()) {
        Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It's not possible to combine following flags: --postOptModules-=... and --")); __mm_s.push_str(&*FlagsUtil::configFlagName(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?); __mm_s.push_str(&*literal!("=false")); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    outPreOptModules = selectOptModules(preOptModules.clone(), enabledModules.clone(), disabledModules.clone(), allPreOptimizationModules())?;
    Ok(outPreOptModules)
}

pub fn getPostOptModulesString() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strpostOptModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strpostOptModules = Config::getPostOptModules()?;
    Ok(strpostOptModules)
}

pub fn getPostOptModules(mut inPostOptModules: Option<Arc<metamodelica::List<ArcStr>>>) -> Result<Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>> {
    let mut outPostOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut postOptModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut enabledModules: Arc<metamodelica::List<ArcStr>> = Flags::getConfigStringList(Flags::POST_OPT_MODULES_ADD.clone())?;
    let mut disabledModules: Arc<metamodelica::List<ArcStr>> = Flags::getConfigStringList(Flags::POST_OPT_MODULES_SUB.clone())?;
    postOptModules = getPostOptModulesString()?;
    postOptModules = Util::getOptionOrDefault(inPostOptModules.clone(), postOptModules.clone());
    if Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())? {
        if Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())? {
            enabledModules = cons((literal!("simplifyConstraints")).clone(), enabledModules.clone());
        }
        if !(Flags::getConfigString(Flags::LOOP2CON.clone())? == literal!("none")) {
            enabledModules = cons((literal!("extendDynamicOptimization")).clone(), enabledModules.clone());
        }
        if Flags::getConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone())? || Config::acceptOptimicaGrammar()? || Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())? {
            enabledModules = cons((literal!("generateSymbolicLinearization")).clone(), enabledModules.clone());
        }
        if Flags::getConfigInt(Flags::SIMPLIFY_LOOPS.clone())? > 0 {
            enabledModules = cons((literal!("simplifyLoops")).clone(), enabledModules.clone());
        }
        if Flags::getConfigString(Flags::HETS.clone())? != literal!("none") {
            enabledModules = cons((literal!("hets")).clone(), enabledModules.clone());
        }
        if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
            enabledModules = cons((literal!("countOperations")).clone(), enabledModules.clone());
        }
        if 1 < Flags::getConfigInt(Flags::MAX_SIZE_FOR_SOLVE_LINIEAR_SYSTEM.clone())? {
            enabledModules = cons((literal!("solveLinearSystem")).clone(), enabledModules.clone());
        }
        if Flags::isSet(Flags::RESHUFFLE_POST.clone())? {
            enabledModules = cons((literal!("reshufflePost")).clone(), enabledModules.clone());
        }
        if Flags::getConfigInt(Flags::RTEARING.clone())? > 0 {
            enabledModules = cons((literal!("recursiveTearing")).clone(), enabledModules.clone());
        }
        if Flags::getConfigInt(Flags::PARTLINTORN.clone())? > 0 {
            enabledModules = cons((literal!("partlintornsystem")).clone(), enabledModules.clone());
        }
        if Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())? == literal!("none") || Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())? == literal!("fastAcausal") || Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())? == literal!("allAcausal") {
            disabledModules = cons((literal!("removeSimpleEquations")).clone(), disabledModules.clone());
        }
        if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
            disabledModules = cons((literal!("inlineArrayEqn")).clone(), disabledModules.clone());
        }
    }
    if !(Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?) && !(enabledModules.clone().is_empty()) {
        Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It's not possible to combine following flags: --postOptModules+=... and --")); __mm_s.push_str(&*FlagsUtil::configFlagName(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?); __mm_s.push_str(&*literal!("=false")); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    if !(Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?) && !(disabledModules.clone().is_empty()) {
        Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It's not possible to combine following flags: --postOptModules-=... and --")); __mm_s.push_str(&*FlagsUtil::configFlagName(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?); __mm_s.push_str(&*literal!("=false")); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    outPostOptModules = selectOptModules(postOptModules.clone(), enabledModules.clone(), disabledModules.clone(), allPostOptimizationModules())?;
    Ok(outPostOptModules)
}

pub fn getInitOptModules(mut inInitOptModules: Option<Arc<metamodelica::List<ArcStr>>>, mut inEnabledModules: Arc<metamodelica::List<ArcStr>>, mut inDisabledModules: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>> {
    let mut outInitOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut initOptModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut enabledModules: Arc<metamodelica::List<ArcStr>> = Flags::getConfigStringList(Flags::INIT_OPT_MODULES_ADD.clone())?;
    let mut disabledModules: Arc<metamodelica::List<ArcStr>> = Flags::getConfigStringList(Flags::INIT_OPT_MODULES_SUB.clone())?;
    initOptModules = Config::getInitOptModules()?;
    initOptModules = Util::getOptionOrDefault(inInitOptModules.clone(), initOptModules.clone());
    if Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())? {
        if Flags::getConfigInt(Flags::SIMPLIFY_LOOPS.clone())? > 0 {
            enabledModules = cons((literal!("simplifyLoops")).clone(), enabledModules.clone());
        }
        if Flags::getConfigInt(Flags::RTEARING.clone())? > 0 {
            enabledModules = cons((literal!("recursiveTearing")).clone(), enabledModules.clone());
        }
    }
    if !(Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?) && !(enabledModules.clone().is_empty()) {
        Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It's not possible to combine following flags: --initOptModules+=... and --")); __mm_s.push_str(&*FlagsUtil::configFlagName(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?); __mm_s.push_str(&*literal!("=false")); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    if !(Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?) && !(disabledModules.clone().is_empty()) {
        Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It's not possible to combine following flags: --initOptModules-=... and --")); __mm_s.push_str(&*FlagsUtil::configFlagName(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?); __mm_s.push_str(&*literal!("=false")); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    outInitOptModules = selectOptModules(initOptModules.clone(), enabledModules.clone(), disabledModules.clone(), allInitOptimizationModules())?;
    initOptModules = List::map(outInitOptModules.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)));
    outInitOptModules = selectOptModules(initOptModules.clone(), inEnabledModules.clone(), inDisabledModules.clone(), allInitOptimizationModules())?;
    Ok(outInitOptModules)
}

fn selectOptModules(mut inStrOptModules: Arc<metamodelica::List<ArcStr>>, mut inEnabledModules: Arc<metamodelica::List<ArcStr>>, mut inDisabledModules: Arc<metamodelica::List<ArcStr>>, mut inOptModules: Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>) -> Result<Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>> {
    let mut outOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut forceOrdering: bool = Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?;
    let mut name: ArcStr = arcstr::literal!("");
    let mut numModules: i32 = (inOptModules.clone().len() as i32);
    let mut activeModules: metamodelica::Array<bool> = arrayCreate(numModules.clone(), false);
    let mut index: i32 = 0;
    let mut maxIndex: i32 = -1;
    if forceOrdering.clone() {
        for mut name in &*inStrOptModules.clone() {
            let mut name = name.clone();
            for mut index in &*getModuleIndexes((name.clone()).clone(), inOptModules.clone())? {
                let mut index = index.clone();
                if index.clone() < maxIndex.clone() {
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Specified ordering will be ignored. Use --")); __mm_s.push_str(&*FlagsUtil::configFlagName(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?); __mm_s.push_str(&*literal!("=false to override module ordering.")); ArcStr::from(__mm_s) }).clone())?;
                    maxIndex = numModules.clone();
                } else {
                    maxIndex = intMax(maxIndex.clone(), index.clone());
                }
                {
                    let __cell0 = true;
                    activeModules.clone().borrow_mut()[(index.clone()-1) as usize] = __cell0;
                }
            }
        }
        for mut name in &*inEnabledModules.clone() {
            let mut name = name.clone();
            for mut index in &*getModuleIndexes((name.clone()).clone(), inOptModules.clone())? {
                let mut index = index.clone();
                {
                    let __cell1 = true;
                    activeModules.clone().borrow_mut()[(index.clone()-1) as usize] = __cell1;
                }
            }
        }
        for mut name in &*inDisabledModules.clone() {
            let mut name = name.clone();
            for mut index in &*getModuleIndexes((name.clone()).clone(), inOptModules.clone())? {
                let mut index = index.clone();
                {
                    let __cell2 = false;
                    activeModules.clone().borrow_mut()[(index.clone()-1) as usize] = __cell2;
                }
            }
        }
        for mut i in 1..=numModules.clone() {
            if activeModules.borrow()[(i.clone()-1) as usize].clone() {
                outOptModules = cons((inOptModules.clone()).get(i.clone())?, outOptModules.clone());
            }
        }
    } else {
        for mut name in &*inStrOptModules.clone() {
            let mut name = name.clone();
            outOptModules = cons(selectOptModules1((name.clone()).clone(), inOptModules.clone())?, outOptModules.clone());
        }
    }
    outOptModules = outOptModules.clone().reverse();
    Ok(outOptModules)
}

fn getModuleIndexes(mut inModuleName: ArcStr, mut inModuleList: Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outIndexes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut index: i32 = 1;
    for mut module in &*inModuleList.clone() {
        let mut module = module.clone();
        (_, name) = module.clone();
        if stringEqual((inModuleName.clone()).clone(), (name.clone()).clone()) {
            outIndexes = cons(index.clone(), outIndexes.clone());
        }
        index = index.clone() + 1;
    }
    if outIndexes.clone().is_empty() {
        Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*inModuleName.clone()); __mm_s.push_str(&*literal!("' is not a valid optimization module. Please check the flags carefully.")); ArcStr::from(__mm_s) }).clone())?;
        bail!("fail");
    }
    Ok(outIndexes)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn selectOptModules1(mut strOptModule: ArcStr, mut inOptModules: Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>) -> Result<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)> {
    let mut outOptModule: (BackendDAEFunc::optimizationModule, ArcStr);
    outOptModule = (::match_deref::match_deref! { match &(inOptModules.clone()) {
        Deref @ metamodelica::List::Cons { head: module @ (_, name), tail: _ } if (stringEqual((name.clone()).clone(), (strOptModule.clone()).clone())) => {
            module.clone()
        },
        Deref @ metamodelica::List::Cons { head: (_, name), tail: rest } if (!(stringEqual((name.clone()).clone(), (strOptModule.clone()).clone()))) => {
            selectOptModules1((strOptModule.clone()).clone(), rest.clone())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Selection of optimization module ")); __mm_s.push_str(&*strOptModule.clone()); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOptModule)
}

pub fn isInitOptModuleActivated(mut initOptModule: ArcStr, mut activatedInitOptModules: Arc<metamodelica::List<(Arc<dyn ::std::ops::Fn(Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> + 'static>, ArcStr)>>) -> Result<bool> {
    let mut isActivated: bool = false;
    let mut modules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = activatedInitOptModules.clone();
    let mut s: ArcStr = arcstr::literal!("");
    if modules.clone().is_empty() {
        modules = getInitOptModules(None, metamodelica::nil(), metamodelica::nil())?;
    }
    for mut module in &*modules.clone() {
        let mut module = module.clone();
        (_, s) = module.clone();
        if stringEqual((s.clone()).clone(), (initOptModule.clone()).clone()) {
            isActivated = true;
            return Ok(isActivated.clone());
        }
    }
    Ok(isActivated)
}

/* ************************************************
 * traverse BackendDAE equation systems
 ************************************************/
pub fn mapEqSystem1<A: Clone + 'static>(mut dae: Arc<BackendDAE::BackendDAE>, mut func: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, A, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>, mut a: A) -> Result<Arc<BackendDAE::BackendDAE>> {
    pub type Function<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, A, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>;

    let mut odae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    (systs, shared) = List::map1Fold(systs.clone(), func.clone(), a.clone(), shared.clone());
    (systs, shared) = filterEmptySystems(systs.clone(), shared.clone())?;
    odae = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(odae)
}

pub fn mapEqSystemAndFold<B: Clone + 'static>(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, B) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, B)> + 'static>, mut initialExtra: B) -> Result<(Arc<BackendDAE::BackendDAE>, B)> {
    pub type Function<B: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, B) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, B)> + 'static>;

    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut outExtra: B;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    (systs, shared, outExtra) = List::mapFold2(systs.clone(), inFunc.clone(), shared.clone(), initialExtra.clone());
    (systs, shared) = filterEmptySystems(systs.clone(), shared.clone())?;
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok((outDAE, outExtra))
}

pub fn foldEqSystem<B: Clone + 'static>(mut dae: Arc<BackendDAE::BackendDAE>, mut func: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, B) -> Result<B> + 'static>, mut initialExtra: B) -> Result<B> {
    pub type Function<B: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, B) -> Result<B> + 'static>;

    let mut extra: B;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    extra = List::fold1(systs.clone(), func.clone(), shared.clone(), initialExtra.clone());
    (_, shared) = filterEmptySystems(systs.clone(), shared.clone())?;
    Ok(extra)
}

pub fn mapEqSystem(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>) -> Result<Arc<BackendDAE::BackendDAE>> {
    pub type Function = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>;

    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    (systs, shared) = List::mapFold(systs.clone(), inFunc.clone(), shared.clone());
    (systs, shared) = filterEmptySystems(systs.clone(), shared.clone())?;
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

pub fn nonEmptySystem(mut syst: Arc<BackendDAE::EqSystem>) -> Result<bool> {
    let mut nonEmpty: bool = false;
    nonEmpty = BackendVariable::varsSize(syst.orderedVars.clone())? != 0 || BackendEquation::getNumberOfEquations(syst.removedEqs.clone()) != 0;
    Ok(nonEmpty)
}

fn filterEmptySystems(mut inSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>)> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    for mut e in &*inSysts.clone() {
        let mut e = e.clone();
        (reqns, outSysts) = filterEmptySystem(e.clone(), reqns.clone(), outSysts.clone())?;
    }
    if outSysts.clone().is_empty() {
        outSysts = list![createEqSystem(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendEquation::emptyEqns(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns())];
    } else {
        outSysts = Dangerous::listReverseInPlace(outSysts.clone());
    }
    assign_field!(outShared.removedEqs = BackendEquation::addList(reqns.clone(), outShared.removedEqs.clone())?);
    Ok((outSysts, outShared))
}

fn filterEmptySystem(mut inSyst: Arc<BackendDAE::EqSystem>, mut reqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>)> {
    let mut reqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = reqs;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = systs;
    if BackendVariable::varsSize(inSyst.orderedVars.clone())? != 0 || isClockedSyst(inSyst.clone()) && BackendEquation::getNumberOfEquations(inSyst.removedEqs.clone()) != 0 {
        systs = cons(inSyst.clone(), systs.clone());
    } else {
        reqs = listAppend(BackendEquation::equationList(inSyst.removedEqs.clone()), reqs.clone());
    }
    Ok((reqs, systs))
}

pub fn getAllVarLst(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { shared: BackendDAE::Shared { globalKnownVars: __pa0, .. }, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    globalKnownVars = __pa0.clone();
    eqs = __pa1.clone();
    varLst = List::flatten(List::map(cons(globalKnownVars.clone(), List::map(eqs.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::daeVars, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<BackendDAE::Variables> + 'static>))), (std::sync::Arc::new(BackendVariable::varList) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>)));
    Ok(varLst)
}

pub fn isClockedSyst(mut inSyst: Arc<BackendDAE::EqSystem>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(inSyst.clone()) {
        Deref @ BackendDAE::EqSystem { partitionKind: BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub fn getAlgorithms(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<metamodelica::Array<Arc<DAE::Algorithm>>> {
    let mut algs: metamodelica::Array<Arc<DAE::Algorithm>>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut alglst: Arc<metamodelica::List<Arc<DAE::Algorithm>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    alglst = List::fold(systs.clone(), (std::sync::Arc::new(collectAlgorithmsFromEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<Arc<DAE::Algorithm>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Algorithm>>>> + 'static>), metamodelica::nil());
    algs = metamodelica::arrayFromVec(alglst.clone().into_iter().cloned().collect());
    Ok(algs)
}

fn collectAlgorithmsFromEqSystem(mut syst: Arc<BackendDAE::EqSystem>, mut alglst: Arc<metamodelica::List<Arc<DAE::Algorithm>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Algorithm>>>> {
    let mut oalglst: Arc<metamodelica::List<Arc<DAE::Algorithm>>> = metamodelica::nil();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let __pa0 = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqns = __pa0.clone();
    oalglst = BackendEquation::traverseEquationArray(eqns.clone(), (std::sync::Arc::new(fnptr!(collectAlgorithms, Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::Algorithm>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::Algorithm>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::Algorithm>>>)> + 'static>), alglst.clone())?;
    Ok(oalglst)
}

fn collectAlgorithms(mut inEq: Arc<BackendDAE::Equation>, mut inAlgs: Arc<metamodelica::List<Arc<DAE::Algorithm>>>) -> (Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::Algorithm>>>) {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut algs: Arc<metamodelica::List<Arc<DAE::Algorithm>>> = metamodelica::nil();
    (outEq, algs) = (::match_deref::match_deref! { match &((inEq.clone(), inAlgs.clone())) {
        (Deref @ BackendDAE::Equation::ALGORITHM { alg, .. }, algs) => {
            (inEq.clone(), cons(alg.clone(), algs.clone()))
        },
        _ => {
            (inEq.clone(), inAlgs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outEq, algs)
}

// =============================================================================
// section for getConditionList
//
// =============================================================================
pub fn getConditionList(mut inCondition: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool)> {
    let mut outConditionVarList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outInitialCall: bool = false;
    (outConditionVarList, outInitialCall) = (::match_deref::match_deref! { match &(inCondition.clone()) {
        Deref @ DAE::Exp::ARRAY { array: conditionList, .. } => {
            let mut conditionVarList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut initialCall: bool = false;
            (conditionVarList, initialCall) = getConditionList1(conditionList.clone(), metamodelica::nil(), false)?;
            (conditionVarList.clone(), initialCall.clone())
        },
        _ => {
            let mut conditionVarList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut initialCall: bool = false;
            (conditionVarList, initialCall) = getConditionList1(list![inCondition.clone()], metamodelica::nil(), false)?;
            (conditionVarList.clone(), initialCall.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outConditionVarList, outInitialCall))
}

fn getConditionList1(mut inConditionList: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inConditionVarList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inInitialCall: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool)> {
    let mut outConditionVarList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outInitialCall: bool = false;
    (outConditionVarList, outInitialCall) = (::match_deref::match_deref! { match &(inConditionList.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inConditionVarList.clone(), inInitialCall.clone())
        },
        Deref @ metamodelica::List::Cons { head: exp, tail: conditionList } if (Expression::isConst(exp.clone())?) => {
            let mut conditionVarList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut initialCall: bool = false;
            (conditionVarList, initialCall) = getConditionList1(conditionList.clone(), inConditionVarList.clone(), inInitialCall.clone())?;
            (conditionVarList.clone(), initialCall.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. }, tail: conditionList } => {
            let mut conditionVarList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut initialCall: bool = false;
            (conditionVarList, initialCall) = getConditionList1(conditionList.clone(), inConditionVarList.clone(), true)?;
            (conditionVarList.clone(), initialCall.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: conditionList } => {
            let mut conditionVarList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut initialCall: bool = false;
            (conditionVarList, initialCall) = getConditionList1(conditionList.clone(), cons(componentRef.clone(), inConditionVarList.clone()), inInitialCall.clone())?;
            (conditionVarList.clone(), initialCall.clone())
        },
        Deref @ metamodelica::List::Cons { head: exp, tail: _ } => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function getConditionList1 failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outConditionVarList, outInitialCall))
}

pub fn isArrayComp(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isArray: bool = false;
    isArray = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isArray
}

pub fn isWhenComp(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isWhen: bool = false;
    isWhen = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWhen
}

pub fn isSingleEquationComp(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isWhen: bool = false;
    isWhen = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWhen
}

pub fn isLinearEqSystemComp(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isWhen: bool = false;
    isWhen = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_LINEAR, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWhen
}

pub fn isNonLinearEqSystemComp(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isWhen: bool = false;
    isWhen = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NONLINEAR, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWhen
}

pub fn isLinearTornSystemComp(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isWhen: bool = false;
    isWhen = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: true, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWhen
}

pub fn isNonLinearTornSystemComp(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isWhen: bool = false;
    isWhen = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWhen
}

pub fn extendRange(mut inRangeExp: Arc<DAE::Exp>, mut inKnVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut start: Arc<DAE::Exp>;
    let mut step: Arc<DAE::Exp>;
    let mut stop: Arc<DAE::Exp>;
    let mut ostep: Option<Arc<DAE::Exp>> = None;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if '__try0: {
        let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inRangeExp.clone()) {
            Deref @ DAE::Exp::RANGE { stop: __pa1, step: __pa2, start: __pa3, ty: __pa4 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        stop = __pa1.clone();
        ostep = __pa2.clone();
        start = __pa3.clone();
        ty = __pa4.clone();
        start = unwrap_break_err!(evalExp(start.clone(), inKnVariables.clone()), '__try0);
        stop = unwrap_break_err!(evalExp(stop.clone(), inKnVariables.clone()), '__try0);
        if isSome(ostep.clone()) {
            let __pa5 = ::match_deref::match_deref! { match &(ostep.clone()) {
                Some(__pa5) => __pa5.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            step = __pa5.clone();
            ostep = Some(unwrap_break_err!(evalExp(step.clone(), inKnVariables.clone()), '__try0));
        }
        outExpLst = unwrap_break_err!(Expression::expandRange(Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: start.clone(), step: ostep.clone(), stop: stop.clone() })), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::trace((literal!("BackendDAECreate.extendRange failed. Maybe some ZeroCrossing are not supported\n")).clone())?;
        }
    }
    Ok(outExpLst)
}

pub fn evalExp(mut inExp: Arc<DAE::Exp>, mut inKnVariables: BackendDAE::Variables) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { .. } => {
            let mut e: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(var_field!((*inExp).componentRef, DAE::Exp::CREF).clone(), inKnVariables.clone())?) {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { bindExp: Some(__pa0), .. }, tail: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            e.clone()
        },
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD { ty: Deref @ DAE::Type::T_INTEGER { .. } }, .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            e1 = evalExp(var_field!((*inExp).exp1, DAE::Exp::BINARY).clone(), inKnVariables.clone())?;
            e2 = evalExp(var_field!((*inExp).exp2, DAE::Exp::BINARY).clone(), inKnVariables.clone())?;
            Arc::new(DAE::Exp::ICONST { integer: Expression::expInt(e1.clone())? + Expression::expInt(e2.clone())? })
        },
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::SUB { ty: Deref @ DAE::Type::T_INTEGER { .. } }, .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            e1 = evalExp(var_field!((*inExp).exp1, DAE::Exp::BINARY).clone(), inKnVariables.clone())?;
            e2 = evalExp(var_field!((*inExp).exp2, DAE::Exp::BINARY).clone(), inKnVariables.clone())?;
            Arc::new(DAE::Exp::ICONST { integer: Expression::expInt(e1.clone())? - Expression::expInt(e2.clone())? })
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn expInt(mut inExp: Arc<DAE::Exp>, mut inKnVariables: BackendDAE::Variables) -> Result<i32> {
    let mut i: i32 = 0;
    i = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: i2 } => {
            i2.clone()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { index: i2, .. } => {
            i2.clone()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut i2: i32 = 0;
            let mut e: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inKnVariables.clone())?) {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { bindExp: Some(__pa0), .. }, tail: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            i2 = expInt(e.clone(), inKnVariables.clone())?;
            i2.clone()
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::ADD { ty: Deref @ DAE::Type::T_INTEGER { .. } }, exp1: e1 } => {
            let mut i1: i32 = 0;
            let mut i2: i32 = 0;
            i1 = expInt(e1.clone(), inKnVariables.clone())?;
            i2 = expInt(e2.clone(), inKnVariables.clone())?;
            i = i1.clone() + i2.clone();
            i.clone()
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::SUB { ty: Deref @ DAE::Type::T_INTEGER { .. } }, exp1: e1 } => {
            let mut i1: i32 = 0;
            let mut i2: i32 = 0;
            i1 = expInt(e1.clone(), inKnVariables.clone())?;
            i2 = expInt(e2.clone(), inKnVariables.clone())?;
            i = i1.clone() - i2.clone();
            i.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(i)
}

pub fn createEqSystem(mut inVars: BackendDAE::Variables, mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inStateSets: Arc<metamodelica::List<BackendDAE::StateSet>>, mut inPartitionKind: BackendDAE::BaseClockPartitionKind, mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Arc<BackendDAE::EqSystem> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    outSyst = Arc::new(BackendDAE::EqSystem { orderedVars: inVars.clone(), orderedEqs: inEqs.clone(), m: None, mT: None, mapping: None, matching: Arc::new(crate::BackendDAE::Matching::NO_MATCHING), stateSets: inStateSets.clone(), partitionKind: inPartitionKind.clone(), removedEqs: removedEqs.clone() });
    outSyst
}

pub fn createEmptyShared(mut backendDAEType: BackendDAE::BackendDAEType, mut ei: BackendDAE::ExtraInfo, mut cache: FCore::Cache, mut graph: FCore::Graph) -> Result<Arc<BackendDAE::Shared>> {
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    shared = Arc::new(BackendDAE::Shared { globalKnownVars: BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), localKnownVars: BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), externalObjects: BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), aliasVars: BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), initialEqs: BackendEquation::emptyEqns(), removedEqs: BackendEquation::emptyEqns(), constraints: metamodelica::nil(), classAttrs: metamodelica::nil(), cache: cache.clone(), graph: graph.clone(), functionTree: AvlTreePathFunction::new(), eventInfo: emptyEventInfo()?, extObjClasses: metamodelica::nil(), backendDAEType: backendDAEType.clone(), symjacs: metamodelica::nil(), info: ei.clone(), partitionsInfo: emptyPartitionsInfo(), daeModeData: BackendDAE::emptyDAEModeData().clone(), dataReconciliationData: None, timeInterval: None });
    Ok(shared)
}

pub fn emptyPartitionsInfo() -> BackendDAE::PartitionsInfo {
    let mut partitionsInfo: BackendDAE::PartitionsInfo = <BackendDAE::PartitionsInfo as ::std::default::Default>::default();
    let mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>;
    let mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>;
    basePartitions = arrayCreate(0, BackendDAE::BasePartition { clock: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK), nSubClocks: 0 });
    subPartitions = arrayCreate(0, BackendDAE::SubPartition { clock: BackendDAE::DEFAULT_SUBCLOCK.clone(), holdEvents: false, prevVars: metamodelica::nil() });
    partitionsInfo = BackendDAE::PartitionsInfo { basePartitions: basePartitions.clone(), subPartitions: subPartitions.clone() };
    partitionsInfo
}

pub fn makeSingleEquationComp(mut eqIdx: i32, mut varIdx: i32) -> Arc<BackendDAE::StrongComponent> {
    let mut comp: Arc<BackendDAE::StrongComponent>;
    comp = Arc::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx.clone(), var: varIdx.clone() });
    comp
}

pub fn getAliasVars(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<BackendDAE::Variables> {
    let mut outAliasVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { shared: BackendDAE::Shared { aliasVars: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outAliasVars = __pa0.clone();
    Ok(outAliasVars)
}

pub fn getGlobalKnownVarsFromDAE(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<BackendDAE::Variables> {
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { shared: BackendDAE::Shared { globalKnownVars: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    globalKnownVars = __pa0.clone();
    Ok(globalKnownVars)
}

pub fn setVars(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inVars: BackendDAE::Variables) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 }, derivativeNamePrefix: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    syst = __pa0.clone();
    systs = __pa1.clone();
    shared = __pa2.clone();
    syst = setEqSystVars(syst.clone(), inVars.clone())?;
    outDAE = BackendDAE::DAE(cons(syst.clone(), systs.clone()), shared.clone())?;
    Ok(outDAE)
}

pub fn setEqs(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 }, derivativeNamePrefix: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    syst = __pa0.clone();
    systs = __pa1.clone();
    shared = __pa2.clone();
    syst = setEqSystEqs(syst.clone(), inEqs.clone());
    outDAE = BackendDAE::DAE(cons(syst.clone(), systs.clone()), shared.clone())?;
    Ok(outDAE)
}

pub fn setAliasVars(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inAliasVars: BackendDAE::Variables) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    shared = setSharedAliasVars(shared.clone(), inAliasVars.clone())?;
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

pub fn setDAEGlobalKnownVars(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inGlobalKnownVars: BackendDAE::Variables) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    shared = setSharedGlobalKnownVars(shared.clone(), inGlobalKnownVars.clone());
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

pub fn setFunctionTree(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    shared = setSharedFunctionTree(shared.clone(), inFunctionTree.clone())?;
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

pub fn setEqSystEqs(mut inSyst: Arc<BackendDAE::EqSystem>, mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Arc<BackendDAE::EqSystem> {
    let mut syst: Arc<BackendDAE::EqSystem> = inSyst.clone();
    assign_field!(syst.orderedEqs = inEqs.clone());
    syst
}

pub fn setEqSystVars(mut inSyst: Arc<BackendDAE::EqSystem>, mut inVars: BackendDAE::Variables) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    outSyst = (::match_deref::match_deref! { match &(inSyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { .. } => {
            let mut syst = (*syst).clone();
            assign_field!(syst.orderedVars = inVars.clone());
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSyst)
}

pub fn setEqSystMatrices(mut inSyst: Arc<BackendDAE::EqSystem>, mut m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>, mut mT: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>, mut mapping: Option<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, BackendDAE::IndexType, bool, bool)>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    outSyst = (::match_deref::match_deref! { match &(inSyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { .. } => {
            let mut syst = (*syst).clone();
            assign_field!(
                syst.m = m.clone(),
                syst.mT = mT.clone(),
                syst.mapping = mapping.clone()
            );
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSyst)
}

pub fn clearEqSyst(mut inSyst: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
    let mut partitionKind: BackendDAE::BaseClockPartitionKind = BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inSyst.clone()) {
        Deref @ BackendDAE::EqSystem { removedEqs: __pa0, partitionKind: __pa1, stateSets: __pa2, orderedEqs: __pa3, orderedVars: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    removedEqs = __pa0.clone();
    partitionKind = __pa1.clone();
    stateSets = __pa2.clone();
    eqs = __pa3.clone();
    vars = __pa4.clone();
    outSyst = Arc::new(BackendDAE::EqSystem { partitionKind: partitionKind.clone(), stateSets: stateSets.clone(), matching: Arc::new(crate::BackendDAE::Matching::NO_MATCHING), removedEqs: removedEqs.clone(), mapping: None, mT: None, m: None, orderedEqs: eqs.clone(), orderedVars: vars.clone() });
    Ok(outSyst)
}

pub fn setEqSystMatching(mut inSyst: Arc<BackendDAE::EqSystem>, mut matching: Arc<BackendDAE::Matching>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    outSyst = (::match_deref::match_deref! { match &(inSyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { .. } => {
            let mut syst = (*syst).clone();
            assign_field!(syst.matching = matching.clone());
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSyst)
}

pub fn setEqSystStateSets(mut inSyst: Arc<BackendDAE::EqSystem>, mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    outSyst = (::match_deref::match_deref! { match &(inSyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { .. } => {
            let mut syst = (*syst).clone();
            assign_field!(syst.stateSets = stateSets.clone());
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSyst)
}

pub fn setEqSystRemovedEqns(mut inSyst: Arc<BackendDAE::EqSystem>, mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Arc<BackendDAE::EqSystem> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = inSyst.clone();
    assign_field!(outSyst.removedEqs = removedEqs.clone());
    outSyst
}

pub fn setSharedRemovedEqns(mut inShared: Arc<BackendDAE::Shared>, mut inRemovedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    outShared = (::match_deref::match_deref! { match &(inShared.clone()) {
        shared @ Deref @ BackendDAE::Shared { .. } => {
            let mut shared = (*shared).clone();
            assign_field!(shared.removedEqs = inRemovedEqs.clone());
            shared.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outShared)
}

pub fn setSharedInitialEqns(mut inShared: Arc<BackendDAE::Shared>, mut initialEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    outShared = (::match_deref::match_deref! { match &(inShared.clone()) {
        shared @ Deref @ BackendDAE::Shared { .. } => {
            let mut shared = (*shared).clone();
            assign_field!(shared.initialEqs = initialEqs.clone());
            shared.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outShared)
}

pub fn setSharedSymJacs(mut inShared: Arc<BackendDAE::Shared>, mut symjacs: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    outShared = (::match_deref::match_deref! { match &(inShared.clone()) {
        shared @ Deref @ BackendDAE::Shared { .. } => {
            let mut shared = (*shared).clone();
            assign_field!(shared.symjacs = symjacs.clone());
            shared.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outShared)
}

pub fn getSharedSymJacs(mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>> {
    let mut outSymjacs: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>> = metamodelica::nil();
    outSymjacs = (::match_deref::match_deref! { match &(inShared.clone()) {
        shared @ Deref @ BackendDAE::Shared { .. } => {
            shared.symjacs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSymjacs)
}

pub fn setSharedFunctionTree(mut inShared: Arc<BackendDAE::Shared>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    outShared = (::match_deref::match_deref! { match &(inShared.clone()) {
        shared @ Deref @ BackendDAE::Shared { .. } => {
            let mut shared = (*shared).clone();
            assign_field!(shared.functionTree = inFunctionTree.clone());
            shared.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outShared)
}

pub fn setSharedEventInfo(mut inShared: Arc<BackendDAE::Shared>, mut eventInfo: BackendDAE::EventInfo) -> Arc<BackendDAE::Shared> {
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    assign_field!(outShared.eventInfo = eventInfo.clone());
    outShared
}

pub fn setSharedGlobalKnownVars(mut inShared: Arc<BackendDAE::Shared>, mut globalKnownVars: BackendDAE::Variables) -> Arc<BackendDAE::Shared> {
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    assign_field!(outShared.globalKnownVars = globalKnownVars.clone());
    outShared
}

pub fn setSharedAliasVars(mut inShared: Arc<BackendDAE::Shared>, mut aliasVars: BackendDAE::Variables) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    outShared = (::match_deref::match_deref! { match &(inShared.clone()) {
        shared @ Deref @ BackendDAE::Shared { .. } => {
            let mut shared = (*shared).clone();
            assign_field!(shared.aliasVars = aliasVars.clone());
            shared.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outShared)
}

pub fn setSharedOptimica(mut inShared: Arc<BackendDAE::Shared>, mut constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut classAttrs: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    outShared = (::match_deref::match_deref! { match &(inShared.clone()) {
        shared @ Deref @ BackendDAE::Shared { .. } => {
            let mut shared = (*shared).clone();
            assign_field!(
                shared.constraints = constraints.clone(),
                shared.classAttrs = classAttrs.clone()
            );
            shared.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outShared)
}

pub fn collapseOrderedEqs(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    eqsLst = List::fold(inDAE.eqs.clone(), (std::sync::Arc::new(fnptr!(collapseRemovedEqs1, Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> + 'static>), metamodelica::nil());
    outEqns = BackendEquation::listEquation(listAppend(eqsLst.clone(), BackendEquation::equationList(inDAE.shared.removedEqs.clone())))?;
    Ok(outEqns)
}

pub fn collapseRemovedEqs(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    eqsLst = List::fold(inDAE.eqs.clone(), (std::sync::Arc::new(fnptr!(collapseRemovedEqs1, Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> + 'static>), metamodelica::nil());
    outEqns = BackendEquation::listEquation(listAppend(eqsLst.clone(), BackendEquation::equationList(inDAE.shared.removedEqs.clone())))?;
    Ok(outEqns)
}

fn collapseRemovedEqs1(mut inSyst: Arc<BackendDAE::EqSystem>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEqns = if (isClockedSyst(inSyst.clone())) {inEqns.clone()} else {listAppend(BackendEquation::equationList(inSyst.removedEqs.clone()), inEqns.clone())};
    outEqns
}

pub fn emptyEventInfo() -> Result<BackendDAE::EventInfo> {
    let mut info: BackendDAE::EventInfo = <BackendDAE::EventInfo as ::std::default::Default>::default();
    info = BackendDAE::EventInfo { timeEvents: metamodelica::nil(), zeroCrossings: ZeroCrossings::new()?, relations: DoubleEnded::fromList(metamodelica::nil())?, samples: ZeroCrossings::new()?, numberMathEvents: 0 };
    Ok(info)
}

pub fn getSubClock(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Option<BackendDAE::SubClock> {
    let mut outSubClock: Option<BackendDAE::SubClock> = None;
    outSubClock = (match inSyst.partitionKind.clone() {
        BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { subPartIdx: mut idx } => {
            Some(inShared.partitionsInfo.subPartitions.borrow()[(idx.clone()-1) as usize].clock.clone())
        },
        _ => {
            None
        },
    });
    outSubClock
}

pub fn componentsEqual(mut comp1: Arc<BackendDAE::StrongComponent>, mut comp2: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isEqual: bool = false;
    isEqual = (::match_deref::match_deref! { match &((comp1.clone(), comp2.clone())) {
        (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: i2, eqn: i1 }, Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: j2, eqn: j1 }) => {
            intEq(i1.clone(), j1.clone()) && intEq(i2.clone(), j2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: l2, eqns: l1, .. }, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: k2, eqns: k1, .. }) => {
            List::isEqualOnTrue(l1.clone(), k1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)) && List::isEqualOnTrue(l2.clone(), k2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: l1, eqn: i1 }, Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: k1, eqn: j1 }) => {
            intEq(i1.clone(), j1.clone()) && List::isEqualOnTrue(l1.clone(), k1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: l1, eqn: i1 }, Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: k1, eqn: j1 }) => {
            intEq(i1.clone(), j1.clone()) && List::isEqualOnTrue(l1.clone(), k1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: l1, eqn: i1 }, Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: k1, eqn: j1 }) => {
            intEq(i1.clone(), j1.clone()) && List::isEqualOnTrue(l1.clone(), k1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: l1, eqn: i1 }, Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: k1, eqn: j1 }) => {
            intEq(i1.clone(), j1.clone()) && List::isEqualOnTrue(l1.clone(), k1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: l1, eqn: i1 }, Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: k1, eqn: j1 }) => {
            intEq(i1.clone(), j1.clone()) && List::isEqualOnTrue(l1.clone(), k1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations: l3, residualequations: l2, tearingvars: l1, .. }, .. }, Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations: k3, residualequations: k2, tearingvars: k1, .. }, .. }) => {
            List::isEqualOnTrue(l1.clone(), k1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)) && List::isEqualOnTrue(l2.clone(), k2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)) && List::isEqualOnTrue(l3.clone(), k3.clone(), (std::sync::Arc::new(fnptr!(innerEquationsEqual, BackendDAE::InnerEquation, BackendDAE::InnerEquation)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation, BackendDAE::InnerEquation) -> Result<bool> + 'static>))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEqual
}

fn innerEquationsEqual(mut innerEquation1: BackendDAE::InnerEquation, mut innerEquation2: BackendDAE::InnerEquation) -> bool {
    let mut isEqual: bool = false;
    isEqual = (match (innerEquation1.clone(), innerEquation2.clone()) {
        (BackendDAE::InnerEquation::INNEREQUATION { vars: ref l1, eqn: mut i1 }, BackendDAE::InnerEquation::INNEREQUATION { vars: ref l2, eqn: mut i2 }) => {
            intEq(i1.clone(), i2.clone()) && List::isEqualOnTrue(l1.clone(), l2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        (BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS { vars: ref l1, eqn: mut i1, .. }, BackendDAE::InnerEquation::INNEREQUATIONCONSTRAINTS { vars: ref l2, eqn: mut i2, .. }) => {
            intEq(i1.clone(), i2.clone()) && List::isEqualOnTrue(l1.clone(), l2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        _ => {
            false
        },
    });
    isEqual
}

pub fn causalizeVarBindSystem(mut varLstIn: Arc<metamodelica::List<BackendDAE::Var>>, mut isInitial: bool) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut nVars: i32 = 0;
    let mut nEqs: i32 = 0;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut bindExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    bindExps = List::map(varLstIn.clone(), (std::sync::Arc::new(BackendVariable::varBindExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>));
    eqs = List::threadMap2(List::map(varLstIn.clone(), (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>)), bindExps.clone(), (std::sync::Arc::new(BackendEquation::generateEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone());
    (m, mT) = adjacencyMatrixDispatch(BackendVariable::listVar1(varLstIn.clone()), BackendEquation::listEquation(eqs.clone())?, crate::BackendDAE::IndexType::ABSOLUTE, None, isInitial.clone())?;
    nVars = (varLstIn.clone().len() as i32);
    nEqs = (eqs.clone().len() as i32);
    ass1 = arrayCreate(nVars.clone(), -1);
    ass2 = arrayCreate(nEqs.clone(), -1);
    Matching::matchingExternalsetAdjacencyMatrix(nVars.clone(), nEqs.clone(), m.clone());
    BackendDAEEXT::matching(nVars.clone(), nEqs.clone(), 5, -1, metamodelica::OrderedFloat(0.0_f64), 1);
    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
    comps = Sorting::TarjanTransposed(mT.clone(), ass2.clone())?;
    Ok((comps, ass1, ass2))
}

pub fn traverseEqSystemStrongComponents<Type_a: Clone + 'static>(mut syst: Arc<BackendDAE::EqSystem>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Type_a) -> Result<Type_a> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Type_a) -> Result<Type_a> + 'static>;

    let mut outTypeA: Type_a = inTypeA.clone();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut varArr: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqnArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    match '__try0: {
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa1, orderedVars: __pa2, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa3, .. }, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        eqnArr = __pa1.clone();
        varArr = __pa2.clone();
        comps = __pa3.clone();
        for mut component in &*comps.clone() {
            let mut component = component.clone();
            (vars, varIdxs, eqns, eqnIdxs) = unwrap_break_err!(getStrongComponentVarsAndEquations(component.clone(), varArr.clone(), eqnArr.clone()), '__try0);
            outTypeA = unwrap_break_err!(func(eqns.clone(), vars.clone(), varIdxs.clone(), eqnIdxs.clone(), outTypeA.clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((comps.clone(), eqnArr.clone(), varArr.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            comps = __try0_o0;
            eqnArr = __try0_o1;
            varArr = __try0_o2;
        }
        Err(_) => {
            (_, _, name) = System::dladdr(func.clone());
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEUtil.traverseEqSystemStrongComponents failed ")); __mm_s.push_str(&*literal!("with function:\n")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok(outTypeA)
}

pub fn getStrongComponentsVarsAndEquations(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut varArr: BackendDAE::Variables, mut eqArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>)> {
    let mut varsOut: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varIdxsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqIdxsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut vIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        (vars, vIdxs, eqs, eIdxs) = getStrongComponentVarsAndEquations(comp.clone(), varArr.clone(), eqArr.clone())?;
        varsOut = listAppend(vars.clone(), varsOut.clone());
        varIdxsOut = listAppend(vIdxs.clone(), varIdxsOut.clone());
        eqsOut = listAppend(eqs.clone(), eqsOut.clone());
        eqIdxsOut = listAppend(eIdxs.clone(), eqIdxsOut.clone());
    }
    varsOut = varsOut.clone().reverse();
    varIdxsOut = varIdxsOut.clone().reverse();
    eqsOut = eqsOut.clone().reverse();
    eqIdxsOut = eqIdxsOut.clone().reverse();
    Ok((varsOut, varIdxsOut, eqsOut, eqIdxsOut))
}

pub fn getStrongComponentVarsAndEquations(mut comp: Arc<BackendDAE::StrongComponent>, mut varArr: BackendDAE::Variables, mut eqArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>)> {
    let mut varsOut: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqIdcxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (varsOut, varIdxs, eqsOut, eqIdcxs) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: vidx, eqn: eidx } => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            var = BackendVariable::getVarAt(varArr.clone(), vidx.clone())?;
            eq = BackendEquation::get(eqArr.clone(), eidx.clone());
            (list![var.clone()], list![vidx.clone()], list![eq.clone()], list![eidx.clone()])
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: vidxs, eqns: eidxs, .. } => {
            let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            vars = List::map1(vidxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varArr.clone());
            eqs = BackendEquation::getList(eidxs.clone(), eqArr.clone());
            (vars.clone(), vidxs.clone(), eqs.clone(), eidxs.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: vidxs, eqn: eidx } => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            vars = List::map1(vidxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varArr.clone());
            eq = BackendEquation::get(eqArr.clone(), eidx.clone());
            (vars.clone(), vidxs.clone(), list![eq.clone()], list![eidx.clone()])
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: vidxs, eqn: eidx } => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            vars = List::map1(vidxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varArr.clone());
            eq = BackendEquation::get(eqArr.clone(), eidx.clone());
            (vars.clone(), vidxs.clone(), list![eq.clone()], list![eidx.clone()])
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: vidxs, eqn: eidx } => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            vars = List::map1(vidxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varArr.clone());
            eq = BackendEquation::get(eqArr.clone(), eidx.clone());
            (vars.clone(), vidxs.clone(), list![eq.clone()], list![eidx.clone()])
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: vidxs, eqn: eidx } => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            vars = List::map1(vidxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varArr.clone());
            eq = BackendEquation::get(eqArr.clone(), eidx.clone());
            (vars.clone(), vidxs.clone(), list![eq.clone()], list![eidx.clone()])
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: vidxs, eqn: eidx } => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            vars = List::map1(vidxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varArr.clone());
            eq = BackendEquation::get(eqArr.clone(), eidx.clone());
            (vars.clone(), vidxs.clone(), list![eq.clone()], list![eidx.clone()])
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, tearingvars: vidxs, residualequations: eidxs, .. }, .. } => {
            let mut otherEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherVarsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut vidxs = (*vidxs).clone();
            let mut eidxs = (*eidxs).clone();
            (otherEqns, otherVarsLst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
            otherVars = List::flatten(otherVarsLst.clone());
            eidxs = listAppend(otherEqns.clone(), eidxs.clone());
            vidxs = listAppend(otherVars.clone(), vidxs.clone());
            vars = List::map1(vidxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varArr.clone());
            eqs = BackendEquation::getList(eidxs.clone(), eqArr.clone());
            (vars.clone(), vidxs.clone(), eqs.clone(), eidxs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((varsOut, varIdxs, eqsOut, eqIdcxs))
}

pub fn getStrongComponentEquations(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut eqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    eqsOut = metamodelica::nil();
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        (_, _, eqLst, _) = getStrongComponentVarsAndEquations(comp.clone(), vars.clone(), eqs.clone())?;
        eqsOut = listAppend(eqLst.clone(), eqsOut.clone());
    }
    Ok(eqsOut)
}

pub fn isFuncCallWithNoDerAnnotation(mut eq: Arc<BackendDAE::Equation>, mut functionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut isFuncCallWithNoDerAnno: bool = false;
    let mut noDerivativeInputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let (_, (_, __pa0)) = BackendEquation::traverseExpsOfEquation(eq.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(isFuncCallWithNoDerAnnotation1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, bool, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>); move |__pe_a0, __pe_a2| Expression::traverseExpTopDown(__pe_a0, __pe_b1.clone(), __pe_a2) }), (functionTree.clone(), metamodelica::nil()))?;
    noDerivativeInputs = __pa0.clone();
    isFuncCallWithNoDerAnno = !(noDerivativeInputs.clone().is_empty());
    Ok((isFuncCallWithNoDerAnno, noDerivativeInputs))
}

pub fn isFuncCallWithNoDerAnnotation1(mut expIn: Arc<DAE::Exp>, mut tplIn: (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, bool, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> {
    let mut expOut: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut tplOut: (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
    (expOut, cont, tplOut) = 'mc: {
        let __mc_input = (expIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst, path, .. }, (functionTree, crefsIn)) => {
                    let mut inputPos: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mapper: DAE::FunctionDefinition;
                    let mut noDerivativeInputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut conditionRefs: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = metamodelica::nil();
                    let mut expLst = (*expLst).clone();
                    (mapper, _) = Differentiate::getFunctionMapper(path.clone(), functionTree.clone())?;
                    let DAE::FUNCTION_DER_MAPPER { conditionRefs: __pa0, .. } = (mapper.clone()) else { bail!("pattern mismatch") };
                    conditionRefs = __pa0.clone();
                    inputPos = getNoDerivativeInputPosition(conditionRefs.clone());
                    expLst = List::map1(inputPos.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    expLst = List::filter1OnTrue(expLst.clone(), (std::sync::Arc::new(isNotFunctionCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<bool> + 'static>), functionTree.clone());
                    noDerivativeInputs = List::flatten(List::map(expLst.clone(), (std::sync::Arc::new(Expression::getAllCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>)));
                    Ok((expIn.clone(), true, (functionTree.clone(), listAppend(noDerivativeInputs.clone(), crefsIn.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((expIn.clone(), true, tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((expOut, cont, tplOut))
}

pub fn isNotFunctionCall(mut inExp: Arc<DAE::Exp>, mut funcsIn: Arc<AvlTreePathFunction::Tree>) -> Result<bool> {
    let mut outIsNoCall: bool = false;
    outIsNoCall = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, .. } => {
                    let mut func: DAE::Function;
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(funcsIn.clone(), path.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    func = __pa0.clone();
                    Ok(DAEUtil::getFunctionElements(func.clone())?.is_empty())
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
    Ok(outIsNoCall)
}

fn getNoDerivativeInputPosition(mut conds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>>) -> Arc<metamodelica::List<i32>> {
    let mut IdxsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut c in &*conds.clone() {
        let mut c = c.clone();
        IdxsOut = (match c.clone() {
        (mut idx, DAE::derivativeCond::NO_DERIVATIVE { binding: _ }) => {
            cons(idx.clone(), IdxsOut.clone())
        },
        _ => {
            IdxsOut.clone()
        },
    });
    }
    IdxsOut
}

pub fn checkAdjacencyMatrixSolvability(mut syst: Arc<BackendDAE::EqSystem>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut isInitial: bool) -> Result<()> {
    let mut varSize: i32 = 0;
    let mut eqnSize: i32 = 0;
    let mut v: i32 = 0;
    let mut eq: i32 = 0;
    let mut count: i32 = 0;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mTOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut solvedVars: metamodelica::Array<i32>;
    let mut solvedEqs: metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>;
    let mut names: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut errors: i32 = 0;
    let mut numSolved: i32 = 0;
    let mut eqSize: i32 = 0;
    let mut lenInfos: i32 = 0;
    let mut lenVars: i32 = 0;
    let mut cont: bool = true;
    let mut varsArray: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut eqsArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut _equation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let debug: bool = false;
    let alwaysCheck: bool = false;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut r#str: ArcStr = arcstr::literal!("");
    varsArray = syst.orderedVars.clone();
    eqsArray = syst.orderedEqs.clone();
    varSize = BackendVariable::varsSize(varsArray.clone())?;
    eqnSize = BackendEquation::equationArraySize(eqsArray.clone())?;
    if varSize.clone() != eqnSize.clone() {
        Error::addMessage(if (varSize.clone() > eqnSize.clone()) {Error::UNDERDET_EQN_SYSTEM.clone()} else {Error::OVERDET_EQN_SYSTEM.clone()}, list![ArcStr::from(::std::format!("{}", eqnSize.clone())), ArcStr::from(::std::format!("{}", varSize.clone()))])?;
    } else if !(alwaysCheck.clone()) {
        return Ok(());
    }
    (_, mOrig, mTOrig) = getAdjacencyMatrixfromOption(syst.clone(), crate::BackendDAE::IndexType::ABSOLUTE, Some(functionTree.clone()), isInitial.clone())?;
    m = metamodelica::arrayFromVec(mOrig.clone().borrow().clone());
    mT = metamodelica::arrayFromVec(mTOrig.clone().borrow().clone());
    solvedVars = arrayCreate((mT.clone().borrow().len() as i32), 0);
    solvedEqs = arrayCreate((m.clone().borrow().len() as i32), metamodelica::nil());
    let __range0 = 1..=(m.clone().borrow().len() as i32);
    for mut i in __range0 {
        _equation = BackendEquation::get(eqsArray.clone(), i.clone());
        info = BackendEquation::equationInfo(_equation.clone())?;
        eqSize = BackendEquation::equationSize(_equation.clone())?;
        count = (m.clone().borrow()[(i.clone()-1) as usize].clone().len() as i32);
        if eqSize.clone() > count.clone() {
            r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut j in (m.clone().borrow()[(i.clone()-1) as usize].clone()).into_iter().cloned() {
            let __x = ComponentReferenceBasics::printComponentRefStr(BackendVariable::varCref(BackendVariable::getVarAt(varsArray.clone(), j.clone())?)?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone());
            Error::addSourceMessage(Error::EQUATION_NOT_SOLVABLE_DIFFERENT_COUNT.clone(), list![(BackendDump::equationString(_equation.clone())?).clone(), ArcStr::from(::std::format!("{}", eqSize.clone())), ArcStr::from(::std::format!("{}", count.clone())), (r#str.clone()).clone()], info.clone())?;
            bail!("fail");
        }
    }
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Got adjacency matrix ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", varSize.clone()))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (m.clone().borrow().len() as i32)))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", eqnSize.clone()))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (mT.clone().borrow().len() as i32)))); __mm_s.push_str(&*literal!("...\n")); ArcStr::from(__mm_s) }).clone());
    }
    count = 0;
    while cont.clone() && count.clone() < 1000 {
        cont = false;
        count = count.clone() + 1;
        let __range1 = 1..=(m.clone().borrow().len() as i32);
        for mut i in __range1 {
            if !(solvedEqs.clone().borrow()[(i.clone()-1) as usize].clone().is_empty()) {
                continue;
            }
            _equation = BackendEquation::get(eqsArray.clone(), i.clone());
            info = BackendEquation::equationInfo(_equation.clone())?;
            eqSize = BackendEquation::equationSize(_equation.clone())?;
            vars = m.clone().borrow()[(i.clone()-1) as usize].clone();
            lenVars = (vars.clone().len() as i32);
            if eqSize.clone() == 0 {
                {let _arr = solvedEqs.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = list![(DAE::emptyCref().clone(), 0)]; _arr};
                continue;
            }
            if lenVars.clone() <= eqSize.clone() {
                if lenVars.clone() < eqSize.clone() {
                    variableDoesNotFitInEquation(i.clone(), vars.clone(), mOrig.clone(), eqsArray.clone(), varsArray.clone(), solvedVars.clone())?;
                    errors = errors.clone() + 1;
                    if lenVars.clone() == 0 {
                        {let _arr = solvedEqs.clone(); let _val = cons((DAE::emptyCref().clone(), 0), solvedEqs.clone().borrow()[(i.clone()-1) as usize].clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
                    }
                }
                for mut v in &*vars.clone() {
                    let mut v = v.clone();
                    var = BackendVariable::getVarAt(varsArray.clone(), v.clone())?;
                    {let _arr = solvedEqs.clone(); let _val = cons((var.varName.clone(), v.clone()), solvedEqs.clone().borrow()[(i.clone()-1) as usize].clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
                    {let _arr = solvedVars.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = i.clone(); _arr};
                    eqs = mT.clone().borrow()[(v.clone()-1) as usize].clone();
                    for mut eq in &*eqs.clone() {
                        let mut eq = eq.clone();
                        {let _arr = m.clone(); let _val = List::setDifference(m.clone().borrow()[(eq.clone()-1) as usize].clone(), vars.clone())?; _arr.borrow_mut()[(eq.clone()-1) as usize] = _val; _arr};
                    }
                    numSolved = numSolved.clone() + 1;
                }
                cont = true;
            }
        }
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of equations solved: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", numSolved.clone() - errors.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of errors: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", errors.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        let __range2 = 1..=(mT.clone().borrow().len() as i32);
        for mut i in __range2 {
            if 0 != solvedVars.clone().borrow()[(i.clone()-1) as usize].clone() {
                continue;
            }
            eqs = mT.clone().borrow()[(i.clone()-1) as usize].clone();
            var = BackendVariable::getVarAt(varsArray.clone(), i.clone())?;
            info = var.source.info.clone();
            if eqs.clone().is_empty() {
                Error::addSourceMessage(Error::VAR_NO_REMAINING_EQN.clone(), list![(ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?).clone(), stringAppendList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut eq in (mTOrig.clone().borrow()[(i.clone()-1) as usize].clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  Equation ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", eq.clone()))); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*BackendDump::equationString(BackendEquation::get(eqsArray.clone(), eq.clone()))?); __mm_s.push_str(&*literal!(", which needs to solve for ")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut tpl in (solvedEqs.clone().borrow()[(eq.clone()-1) as usize].clone()).into_iter().cloned() {
            let __x = ComponentReferenceBasics::printComponentRefStr(Util::tuple21(tpl.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))], info.clone())?;
                {let _arr = solvedVars.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = -1; _arr};
            } else if (eqs.clone().len() as i32) == 1 {
                let __pa3 = ::match_deref::match_deref! { match &(eqs.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                eq = __pa3.clone();
                eqSize = BackendEquation::equationSize(BackendEquation::get(eqsArray.clone(), eq.clone()))?;
                names = solvedEqs.clone().borrow()[(eq.clone()-1) as usize].clone();
                lenInfos = (names.clone().len() as i32);
                {let _arr = solvedVars.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = eq.clone(); _arr};
                {let _arr = solvedEqs.clone(); _arr.borrow_mut()[(eq.clone()-1) as usize] = cons((var.varName.clone(), i.clone()), names.clone()); _arr};
                if lenInfos.clone() >= eqSize.clone() {
                    variableDoesNotFitInEquation(eq.clone(), list![i.clone()], mOrig.clone(), eqsArray.clone(), varsArray.clone(), solvedVars.clone())?;
                    errors = errors.clone() + 1;
                }
                vars = m.clone().borrow()[(eq.clone()-1) as usize].clone();
                if lenInfos.clone() + 1 >= eqSize.clone() {
                    for mut v in &*vars.clone() {
                        let mut v = v.clone();
                        {let _arr = mT.clone(); let _val = List::setDifference(mT.clone().borrow()[(v.clone()-1) as usize].clone(), eqs.clone())?; _arr.borrow_mut()[(v.clone()-1) as usize] = _val; _arr};
                    }
                }
                numSolved = numSolved.clone() + 1;
                cont = true;
            }
        }
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of equations solved: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", numSolved.clone() - errors.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of errors: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", errors.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    let true = (0 == errors.clone()) else { bail!("pattern mismatch") };
    if alwaysCheck.clone() && varSize.clone() == eqnSize.clone() {
        return Ok(());
    }
    if debug.clone() {
        let __range5 = 1..=(mT.clone().borrow().len() as i32);
        for mut i in __range5 {
            var = BackendVariable::getVarAt(varsArray.clone(), i.clone())?;
            if 0 == solvedVars.clone().borrow()[(i.clone()-1) as usize].clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Remaining unsolved variable:")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
        }
        let __range6 = 1..=(m.clone().borrow().len() as i32);
        for mut i in __range6 {
            _equation = BackendEquation::get(eqsArray.clone(), i.clone());
            eqnSize = BackendEquation::equationSize(_equation.clone())?;
            count = (solvedEqs.clone().borrow()[(i.clone()-1) as usize].clone().len() as i32);
            if eqnSize.clone() != count.clone() {
                vars = m.clone().borrow()[(i.clone()-1) as usize].clone();
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Remaining vars: ")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut j in (vars.clone()).into_iter().cloned() {
            let __x = ArcStr::from(::std::format!("{}", j.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                if count.clone() > 0 {
                    r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (solvedEqs.clone().borrow()[(i.clone()-1) as usize].clone()).into_iter().cloned() {
            let __x = ComponentReferenceBasics::printComponentRefStr(Util::tuple21(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Remaining equation (already solved ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*BackendDump::equationString(_equation.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                } else {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Remaining equation: ")); __mm_s.push_str(&*BackendDump::equationString(_equation.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
        }
    }
    bail!("fail");
    Ok(())
}

fn variableDoesNotFitInEquation(mut eq: i32, mut vars: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqsArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut varsArray: BackendDAE::Variables, mut solvedVars: metamodelica::Array<i32>) -> Result<()> {
    let mut _equation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut varsInOrig: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqsInOrig: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqsString: ArcStr = arcstr::literal!("");
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut eqSize: i32 = 0;
    _equation = BackendEquation::get(eqsArray.clone(), eq.clone());
    info = BackendEquation::equationInfo(_equation.clone())?;
    eqSize = BackendEquation::equationSize(_equation.clone())?;
    varsInOrig = List::setDifference(m.clone().borrow()[(eq.clone()-1) as usize].clone(), vars.clone())?;
    eqsInOrig = List::sortedUnique(List::sort(({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut myVar in (varsInOrig.clone()).into_iter().cloned() {
            if !(solvedVars.clone().borrow()[(myVar.clone()-1) as usize].clone() > 0) { continue; }
            let __x = solvedVars.clone().borrow()[(myVar.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    eqsString = stringAppendList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut eqNum in (eqsInOrig.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n    Equation ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", eqNum.clone()))); __mm_s.push_str(&*literal!(" (size: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", BackendEquation::equationSize(BackendEquation::get(eqsArray.clone(), eqNum.clone()))?))); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*BackendDump::equationString(BackendEquation::get(eqsArray.clone(), eqNum.clone()))?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    Error::addSourceMessage(Error::EQN_NO_SPACE_TO_SOLVE.clone(), list![ArcStr::from(::std::format!("{}", eq.clone())), ArcStr::from(::std::format!("{}", eqSize.clone())), (BackendDump::equationString(_equation.clone())?).clone(), (getVariableNamesForErrorMessage(varsArray.clone(), vars.clone())?).clone(), (getVariableNamesForErrorMessage(varsArray.clone(), varsInOrig.clone())?).clone(), (eqsString.clone()).clone()], info.clone())?;
    Ok(())
}

fn getVariableNamesForErrorMessage(mut varsArray: BackendDAE::Variables, mut vars: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut names: ArcStr = arcstr::literal!("");
    if vars.clone().is_empty() {
        names = (literal!("")).clone();
    } else {
        names = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = ComponentReferenceBasics::printComponentRefStr(BackendVariable::varCref(BackendVariable::getVarAt(varsArray.clone(), v.clone())?)?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone();
    }
    Ok(names)
}

// =============================================================================
// warn about iteration variables with no nominal attribute
//
// =============================================================================
fn warnAboutIterationVariablesWithNoNominal(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut daeTypeStr: ArcStr = arcstr::literal!("");
    let mut compKind: ArcStr = arcstr::literal!("");
    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    daeTypeStr = (BackendDump::printBackendDAEType2String(inDAE.shared.backendDAEType.clone())?).clone();
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        let __pa0 = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        comps = __pa0.clone();
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            (compKind, vlst) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NONLINEAR, vars: vlst, .. } => ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("nonlinear equation system in the ")); __mm_s.push_str(&*daeTypeStr.clone()); __mm_s.push_str(&*literal!(" DAE:")); ArcStr::from(__mm_s) }, vlst.clone()),
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_GENERIC, vars: vlst, .. } => ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("equation system with analytic Jacobian in the ")); __mm_s.push_str(&*daeTypeStr.clone()); __mm_s.push_str(&*literal!(" DAE:")); ArcStr::from(__mm_s) }, vlst.clone()),
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC, vars: vlst, .. } => ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("equation system without analytic Jacobian in the ")); __mm_s.push_str(&*daeTypeStr.clone()); __mm_s.push_str(&*literal!(" DAE:")); ArcStr::from(__mm_s) }, vlst.clone()),
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, .. }, .. } => ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("torn nonlinear equation system in the ")); __mm_s.push_str(&*daeTypeStr.clone()); __mm_s.push_str(&*literal!(" DAE:")); ArcStr::from(__mm_s) }, vlst.clone()),
        _ => (literal!(""), metamodelica::nil()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(vlst.clone().is_empty()) {
                vars = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), syst.orderedVars.clone());
                vars = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            if !(!(BackendVariable::varHasNominalValue(v.clone()))) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                if !(vars.clone().is_empty()) {
                    Error::addCompilerWarning((BackendDump::varListStringIndented(vars.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Iteration variables with no nominal value in ")); __mm_s.push_str(&*compKind.clone()); ArcStr::from(__mm_s) }).clone())).clone())?;
                }
            }
        }
    }
    Ok(())
}

pub fn getLinearfromJacType(mut jacType: BackendDAE::JacobianType) -> Result<bool> {
    let mut linear: bool = false;
    linear = (match jacType.clone() {
        BackendDAE::JacobianType::JAC_CONSTANT => true,
        BackendDAE::JacobianType::JAC_LINEAR => true,
        BackendDAE::JacobianType::JAC_NONLINEAR => false,
        BackendDAE::JacobianType::JAC_NO_ANALYTIC => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(linear)
}

pub fn containsHomotopyCall(mut inExp: Arc<DAE::Exp>, mut inHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHomotopy: bool = false;
    (outExp, outHomotopy) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(fnptr!(containsHomotopyCall2, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), inHomotopy.clone())?;
    Ok((outExp, outHomotopy))
}

fn containsHomotopyCall2(mut inExp: Arc<DAE::Exp>, mut inHomotopy: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut cont: bool = false;
    let mut outHomotopy: bool = false;
    (outExp, outHomotopy, cont) = (::match_deref::match_deref! { match &((inExp.clone(), inHomotopy.clone())) {
        (_, true) => (inExp.clone(), true, false),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, _) => (inExp.clone(), true, false),
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "__HOM_LAMBDA", .. }, .. }, _) => (inExp.clone(), true, false),
        _ => (inExp.clone(), inHomotopy.clone(), true),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outHomotopy)
}

pub fn doIndexReduction(mut opt: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints)) -> bool {
    let mut b: bool = false;
    b = (match opt.clone() {
        (BackendDAE::IndexReduction::INDEX_REDUCTION, _) => true,
        _ => false,
    });
    b
}

pub fn markNonlinearIterationVariables(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    assign_field!(dae.eqs = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
        for mut syst in (dae.eqs.clone()).into_iter().cloned() {
            let __x = markNonlinearIterationVariablesEqSystem(syst.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    Ok(dae)
}

fn markNonlinearIterationVariablesEqSystem(mut syst: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    syst = ({
        let mut set: Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(ComponentReference::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), 13);
        (::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, .. } => {
            for mut comp in &*comps.clone() {
                let mut comp = comp.clone();
                markNonlinearIterationVariablesStrongComponent(comp.clone(), set.clone())?;
            }
            let (syst.orderedVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(syst.orderedVars.clone(), (std::sync::Arc::new(markNonlinearIterationVariable) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>>) -> Result<(BackendDAE::Var, Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>>)> + 'static>), set.clone())?;
            syst.clone()
        },
        _ => {
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(syst)
}

fn markNonlinearIterationVariablesStrongComponent(mut comp: Arc<BackendDAE::StrongComponent>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>>) -> Result<()> {
    let mut nonlinear_iteration_vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    nonlinear_iteration_vars = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, strictTearingSet: BackendDAE::TearingSet { jac, .. }, .. } => {
            SymbolicJacobian::getNonLinearVariables(jac.clone())?
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_GENERIC, jac, .. } => {
            SymbolicJacobian::getNonLinearVariables(jac.clone())?
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut var in &*nonlinear_iteration_vars.clone() {
        let mut var = var.clone();
        UnorderedSet::add(var.varName.clone(), set.clone())?;
    }
    Ok(())
}

fn markNonlinearIterationVariable(mut var: BackendDAE::Var, mut set: Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>>) -> Result<(BackendDAE::Var, Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>>)> {
    let mut var: BackendDAE::Var = var;
    let mut set: Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>> = set;
    if UnorderedSet::contains(var.varName.clone(), set.clone())? {
        var = BackendVariable::setVarInitNonlinear(var.clone(), true);
    }
    Ok((var, set))
}

