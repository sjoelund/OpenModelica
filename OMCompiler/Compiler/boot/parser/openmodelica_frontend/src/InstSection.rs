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

use crate::Algorithm;
use crate::Ceval;
use crate::ComponentReference;
use crate::ConnectUtil;
use crate::ConnectionGraph;
use crate::DAEDump;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionSimplify;
use crate::ExpressionSimplifyTypes;
use crate::FCore;
use crate::FGraph;
use crate::InnerOuter;
use crate::Inst;
use crate::InstDAE;
use crate::InstFunction;
use crate::InstTypes;
use crate::InstUtil;
use crate::Lookup;
use crate::Patternm;
use crate::PrefixUtil;
use crate::Static;
use crate::Types;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

pub const alwaysUnroll: bool = true;

pub fn instEquation(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inEquation: Arc<SCode::Equation>, mut inImpl: bool, mut unrollForLoops: bool, mut inGraph: ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = instEquationCommon(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), inEquation.clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, inImpl.clone(), inGraph.clone())?;
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

pub fn instInitialEquation(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inEquation: Arc<SCode::Equation>, mut inImpl: bool, mut unrollForLoops: bool, mut inGraph: ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = instEquationCommon(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), inEquation.clone(), openmodelica_frontend_types::SCode::Initial::INITIAL, inImpl.clone(), inGraph.clone())?;
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn instEquationCommon(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inEquation: Arc<SCode::Equation>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inGraph: ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut errorCount: i32 = Error::getNumErrorMessages();
    let _ = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut state: ClassInf::State;
            let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
            let mut outSets: DAE::Connect::Sets;
            let mut outGraph: ConnectionGraph::ConnectionGraph;
            let mut outCache: FCore::Cache = outCache.clone();
            let mut outEnv: FCore::Graph;
            let mut outState: ClassInf::State;
            let mut outDae: DAE::DAElist;
            state = ClassInfUtil::trans(inState.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_EQUATION)?;
            (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = instEquationCommonWork(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), state.clone(), inEquation.clone(), inInitial.clone(), inImpl.clone(), inGraph.clone(), Arc::new(DAE::SymbolicOperation::FLATTEN { scode: inEquation.clone(), dae: None }))?;
            outDae = DAEUtil::traverseDAE(outDae.clone(), AvlTreePathFunction::Tree::EMPTY()?, Arc::new(Expression::traverseSubexpressionsHelper), (ExpressionSimplify::simplifyWork, ExpressionSimplifyTypes::optionSimplifyOnly.clone()));
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut s: ArcStr = arcstr::literal!("");
            if '__try0: {
                let _ = unwrap_break_err!(ClassInfUtil::trans(inState.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_EQUATION), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            s = (ClassInfUtil::printStateStr(inState.clone())).clone();
            Error::addSourceMessage(Error::EQUATION_TRANSITION_FAILURE.clone(), list![(s.clone()).clone()], SCodeUtil::getEquationInfo(inEquation.clone())?)?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut s: ArcStr = arcstr::literal!("");
            let true = (errorCount.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::EQUATION_GENERIC_FAILURE.clone(), list![(s.clone()).clone()], SCodeUtil::getEquationInfo(inEquation.clone())?)?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn instEquationCommonWork(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inEquation: Arc<SCode::Equation>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inGraph: ConnectionGraph::ConnectionGraph, mut inFlattenOp: Arc<DAE::SymbolicOperation>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets = inSets.clone();
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph.clone();
    (outDae, outState) = 'mc: {
        let __mc_input = inEquation.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_CONNECT { info, crefRight: rhs_acr, crefLeft: lhs_acr, .. } => {
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outState: ClassInf::State;
                    let mut outSets: DAE::Connect::Sets = outSets.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outDae: DAE::DAElist;
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    if SCodeUtil::isInitial(inInitial.clone()) {
                        Error::addSourceMessage(Error::CONNECT_IN_INITIAL_EQUATION.clone(), metamodelica::nil(), info.clone())?;
                        bail!("fail");
                    }
                    (outCache, outEnv, outIH, outSets, outDae, outGraph) = instConnect(outCache.clone(), outEnv.clone(), outIH.clone(), outSets.clone(), inPrefix.clone(), lhs_acr.clone(), rhs_acr.clone(), inImpl.clone(), inGraph.clone(), info.clone())?;
                    outState = instEquationCommonCiTrans(inState.clone(), inInitial.clone())?;
                    Ok((outDae.clone(), outState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_EQUALS { comment, info, expRight: rhs_aexp, expLeft: lhs_aexp } => {
                    let mut lhs_exp: Arc<DAE::Exp>;
                    let mut rhs_exp: Arc<DAE::Exp>;
                    let mut lhs_prop: DAE::Properties;
                    let mut rhs_prop: DAE::Properties;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut outDae: DAE::DAElist;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outState: ClassInf::State;
                    checkTupleCallEquationMessage(lhs_aexp.clone(), rhs_aexp.clone(), info.clone())?;
                    (outCache, lhs_exp, lhs_prop) = Static::elabExpLHS(inCache.clone(), inEnv.clone(), lhs_aexp.clone(), inImpl.clone(), true, inPrefix.clone(), info.clone())?;
                    (outCache, rhs_exp, rhs_prop) = Static::elabExp(inCache.clone(), inEnv.clone(), rhs_aexp.clone(), inImpl.clone(), true, inPrefix.clone(), info.clone())?;
                    (outCache, lhs_exp, lhs_prop) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), lhs_exp.clone(), lhs_prop.clone(), inImpl.clone(), info.clone())?;
                    (outCache, rhs_exp, rhs_prop) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), rhs_exp.clone(), rhs_prop.clone(), inImpl.clone(), info.clone())?;
                    (outCache, lhs_exp, rhs_exp, lhs_prop) = condenseArrayEquation(outCache.clone(), inEnv.clone(), lhs_aexp.clone(), rhs_aexp.clone(), lhs_exp.clone(), rhs_exp.clone(), lhs_prop.clone(), rhs_prop.clone(), inImpl.clone(), inPrefix.clone(), info.clone())?;
                    (outCache, lhs_exp) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), lhs_exp.clone(), inPrefix.clone())?;
                    (outCache, rhs_exp) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), rhs_exp.clone(), inPrefix.clone())?;
                    source = makeEqSource(info.clone(), inEnv.clone(), inPrefix.clone(), inFlattenOp.clone())?;
                    source = ElementSource::addCommentToSource(source.clone(), Some(comment.clone()));
                    outDae = instEqEquation(lhs_exp.clone(), lhs_prop.clone(), rhs_exp.clone(), rhs_prop.clone(), source.clone(), inInitial.clone(), inImpl.clone(), Absyn::dummyInfo.clone())?;
                    outState = instEquationCommonCiTrans(inState.clone(), inInitial.clone())?;
                    Ok((outDae.clone(), outState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_IF { info, elseBranch: else_branch, thenBranch: branches, .. } => {
                    let mut prop: DAE::Properties;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut rest_branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
                    let mut ell: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
                    let mut el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outDae: DAE::DAElist;
                    let mut outState: ClassInf::State;
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outSets: DAE::Connect::Sets = outSets.clone();
                    (outCache, expl, props) = Static::elabExpList(outCache.clone(), outEnv.clone(), var_field!((*inEquation).condition, SCode::Equation::EQ_IF).clone(), inImpl.clone(), true, inPrefix.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT.clone())?;
                    prop = Types::propsAnd(props.clone())?;
                    checkIfConditionTypes(prop.clone(), var_field!((*inEquation).condition, SCode::Equation::EQ_IF).clone(), props.clone(), info.clone())?;
                    match '__try0: {
                        rest_branches = branches.clone();
                        eql = else_branch.clone();
                        for mut cond in &*expl.clone() {
                            let mut cond = cond.clone();
                            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(props.clone()) {
                                        Deref @ metamodelica::List::Cons { head: DAE::Properties::PROP { constFlag: __pa1, .. }, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                                        _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                            } };
                            c = __pa1.clone();
                            props = __pa2.clone();
                            let true = (Types::isParameterOrConstant(c.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                            (outCache, val) = unwrap_break_err!(Ceval::ceval(outCache.clone(), outEnv.clone(), cond.clone(), inImpl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0), '__try0);
                            let true = (unwrap_break_err!(checkIfConditionBinding(val.clone(), info.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                            if unwrap_break_err!(ValuesUtil::valueBool(val.clone()), '__try0) {
                                        eql = unwrap_break_err!(listHead(rest_branches.clone()), '__try0);
                                        break;
                            }
                            rest_branches = unwrap_break_err!(listRest(rest_branches.clone()), '__try0);
                        }
                        outCache = unwrap_break_err!(InstUtil::popStructuralParameters(outCache.clone(), inPrefix.clone()), '__try0);
                        (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = unwrap_break_err!(Inst::instList(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), (if (SCodeUtil::isInitial(inInitial.clone())) { (Arc::new(instInitialEquation) as _) } else { (Arc::new(instEquation) as _) }), eql.clone(), inImpl.clone(), alwaysUnroll.clone(), inGraph.clone()), '__try0);
                        Ok::<_, anyhow::Error>((outCache.clone(), outDae.clone(), outEnv.clone(), outIH.clone(), outState.clone()))
                    } {
                        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
                            outCache = __try0_o0;
                            outDae = __try0_o1;
                            outEnv = __try0_o2;
                            outIH = __try0_o3;
                            outState = __try0_o4;
                        }
                        Err(_) => {
                            (outCache, expl) = PrefixUtil::prefixExpList(outCache.clone(), inEnv.clone(), inIH.clone(), expl.clone(), inPrefix.clone())?;
                            source = makeEqSource(info.clone(), inEnv.clone(), inPrefix.clone(), inFlattenOp.clone())?;
                            if SCodeUtil::isInitial(inInitial.clone()) {
                                        (outCache, outEnv, outIH, outState, ell) = instInitialIfEqBranches(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), branches.clone(), inImpl.clone(), metamodelica::nil())?;
                                        (outCache, outEnv, outIH, outState, el) = instInitialIfEqBranch(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), outState.clone(), else_branch.clone(), inImpl.clone())?;
                                        outDae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIAL_IF_EQUATION { condition1: expl.clone(), equations2: ell.clone(), equations3: el.clone(), source: source.clone() })] };
                            } else {
                                        (outCache, outEnv, outIH, outState, ell) = instIfEqBranches(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), branches.clone(), inImpl.clone(), metamodelica::nil())?;
                                        (outCache, outEnv, outIH, outState, el) = instIfEqBranch(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), outState.clone(), else_branch.clone(), inImpl.clone())?;
                                        outDae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::IF_EQUATION { condition1: expl.clone(), equations2: ell.clone(), equations3: el.clone(), source: source.clone() })] };
                            }
                        }
                    }
                    Ok((outDae.clone(), outState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_WHEN { info, .. } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut cond_exp: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut el2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut else_when: Option<Arc<DAE::Element>> = None;
                    let mut outState: ClassInf::State;
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outDae: DAE::DAElist;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    if SCodeUtil::isInitial(inInitial.clone()) {
                        Error::addSourceMessageAndFail(Error::INITIAL_WHEN.clone(), metamodelica::nil(), info.clone())?;
                    }
                    (outCache, outEnv, outIH, cond_exp, el, outGraph) = instWhenEqBranch(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), (var_field!((*inEquation).condition, SCode::Equation::EQ_WHEN).clone(), var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_WHEN).clone()), inImpl.clone(), alwaysUnroll.clone(), inGraph.clone(), info.clone())?;
                    source = makeEqSource(info.clone(), inEnv.clone(), inPrefix.clone(), inFlattenOp.clone())?;
                    else_when = None;
                    for mut branch in &*var_field!((*inEquation).elseBranches, SCode::Equation::EQ_WHEN).clone().reverse() {
                        let mut branch = branch.clone();
                        (outCache, outEnv, outIH, exp, el2, outGraph) = instWhenEqBranch(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), branch.clone(), inImpl.clone(), alwaysUnroll.clone(), outGraph.clone(), info.clone())?;
                        else_when = Some(Arc::new(DAE::Element::WHEN_EQUATION { condition: exp.clone(), equations: el2.clone(), elsewhen_: else_when.clone(), source: source.clone() }));
                    }
                    outState = instEquationCommonCiTrans(inState.clone(), inInitial.clone())?;
                    outDae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::WHEN_EQUATION { condition: cond_exp.clone(), equations: el.clone(), elsewhen_: else_when.clone(), source: source.clone() })] };
                    Ok((outDae.clone(), outState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_FOR { info, .. } => {
                    let mut range_aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp: Arc<DAE::Exp>;
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut iter_crefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env: FCore::Graph;
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outSets: DAE::Connect::Sets = outSets.clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outDae: DAE::DAElist;
                    let mut outState: ClassInf::State;
                    let mut outCache: FCore::Cache = outCache.clone();
                    if isSome(var_field!((*inEquation).range, SCode::Equation::EQ_FOR).clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(var_field!((*inEquation).range, SCode::Equation::EQ_FOR).clone()) {
                            Some(__pa0) => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        range_aexp = __pa0.clone();
                        let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Static::elabExp(outCache.clone(), inEnv.clone(), range_aexp.clone(), inImpl.clone(), true, inPrefix.clone(), info.clone())?) {
                            (__pa1, __pa2, DAE::Properties::PROP { constFlag: __pa3, type_: Deref @ DAE::Type::T_ARRAY { ty: __pa4, .. } }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        outCache = __pa1.clone();
                        exp = __pa2.clone();
                        c = __pa3.clone();
                        ty = __pa4.clone();
                    } else {
                        iter_crefs = SCodeUtil::findIteratorIndexedCrefsInEquations(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_FOR).clone(), (var_field!((*inEquation).index, SCode::Equation::EQ_FOR).clone()).clone(), metamodelica::nil());
                        let (__pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(Static::deduceIterationRange((var_field!((*inEquation).index, SCode::Equation::EQ_FOR).clone()).clone(), iter_crefs.clone(), inEnv.clone(), outCache.clone(), info.clone())?) {
                            (__pa6, DAE::Properties::PROP { constFlag: __pa7, type_: Deref @ DAE::Type::T_ARRAY { ty: __pa8, .. } }, __pa9) => (__pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        exp = __pa6.clone();
                        c = __pa7.clone();
                        ty = __pa8.clone();
                        outCache = __pa9.clone();
                        range_aexp = Arc::new(Absyn::Exp::STRING { value: (literal!("Internal error: generated implicit range could not be evaluated.")).clone() });
                    }
                    env = addForLoopScope(inEnv.clone(), (var_field!((*inEquation).index, SCode::Equation::EQ_FOR).clone()).clone(), ty.clone(), openmodelica_frontend_types::SCode::Variability::VAR, Some(c.clone()))?;
                    if '__try11: {
                        (outCache, val) = unwrap_break_err!(Ceval::ceval(outCache.clone(), inEnv.clone(), exp.clone(), inImpl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0), '__try11);
                        Ok::<(), anyhow::Error>(())
                    }.is_err() {
                        if Flags::getConfigBool(Flags::CHECK_MODEL.clone())? {
                            val = Arc::new(Values::Value::ARRAY { valueLst: list![Arc::new(Values::Value::INTEGER { integer: 1 })], dimLst: list![1] });
                        } else {
                            Error::addSourceMessageAndFail(Error::NON_PARAMETER_ITERATOR_RANGE.clone(), list![(Dump::printExpStr(range_aexp.clone())?).clone()], info.clone())?;
                        }
                    }
                    (outCache, outDae, outSets, outGraph) = unroll(outCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), (var_field!((*inEquation).index, SCode::Equation::EQ_FOR).clone()).clone(), ty.clone(), val.clone(), var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_FOR).clone(), inInitial.clone(), inImpl.clone(), inGraph.clone())?;
                    outState = instEquationCommonCiTrans(inState.clone(), inInitial.clone())?;
                    Ok((outDae.clone(), outState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_ASSERT { info, .. } => {
                    let mut cond_exp: Arc<DAE::Exp>;
                    let mut msg_exp: Arc<DAE::Exp>;
                    let mut level_exp: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist;
                    (outCache, cond_exp) = instOperatorArg(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inEquation).condition, SCode::Equation::EQ_ASSERT).clone(), inImpl.clone(), DAE::T_BOOL_DEFAULT.clone(), (literal!("assert")).clone(), (literal!("condition")).clone(), 1, info.clone())?;
                    (outCache, msg_exp) = instOperatorArg(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inEquation).message, SCode::Equation::EQ_ASSERT).clone(), inImpl.clone(), DAE::T_STRING_DEFAULT.clone(), (literal!("assert")).clone(), (literal!("message")).clone(), 2, info.clone())?;
                    (outCache, level_exp) = instOperatorArg(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inEquation).level, SCode::Equation::EQ_ASSERT).clone(), inImpl.clone(), DAE::T_ASSERTIONLEVEL.clone(), (literal!("assert")).clone(), (literal!("level")).clone(), 3, info.clone())?;
                    source = makeEqSource(info.clone(), inEnv.clone(), inPrefix.clone(), inFlattenOp.clone())?;
                    if SCodeUtil::isInitial(inInitial.clone()) {
                        outDae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIAL_ASSERT { condition: cond_exp.clone(), message: msg_exp.clone(), level: level_exp.clone(), source: source.clone() })] };
                    } else {
                        outDae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::ASSERT { condition: cond_exp.clone(), message: msg_exp.clone(), level: level_exp.clone(), source: source.clone() })] };
                    }
                    Ok((outDae.clone(), inState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_TERMINATE { info, .. } => {
                    let mut msg_exp: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist;
                    (outCache, msg_exp) = instOperatorArg(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inEquation).message, SCode::Equation::EQ_TERMINATE).clone(), inImpl.clone(), DAE::T_STRING_DEFAULT.clone(), (literal!("terminate")).clone(), (literal!("message")).clone(), 1, info.clone())?;
                    source = makeEqSource(info.clone(), inEnv.clone(), inPrefix.clone(), inFlattenOp.clone())?;
                    if SCodeUtil::isInitial(inInitial.clone()) {
                        outDae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIAL_TERMINATE { message: msg_exp.clone(), source: source.clone() })] };
                    } else {
                        outDae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::TERMINATE { message: msg_exp.clone(), source: source.clone() })] };
                    }
                    Ok((outDae.clone(), inState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_REINIT { info, cref: Deref @ Absyn::Exp::CREF { componentRef: acr }, .. } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut cr_exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cr_prop: DAE::Properties;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist;
                    let (__pa0, __pa3, __pa1, __pa2, __pa4) = ::match_deref::match_deref! { match &(Static::elabCrefNoEval(outCache.clone(), inEnv.clone(), acr.clone(), inImpl.clone(), false, inPrefix.clone(), info.clone())?) {
                        (__pa0, __pa3 @ Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: __pa2 }, __pa4, _) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    outCache = __pa0.clone();
                    cr = __pa1.clone();
                    ty = __pa2.clone();
                    cr_exp = __pa3.clone();
                    cr_prop = __pa4.clone();
                    let true = (checkReinitType(ty.clone(), cr_prop.clone(), cr.clone(), info.clone())?) else { bail!("pattern mismatch") };
                    (outCache, exp, prop) = Static::elabExp(outCache.clone(), inEnv.clone(), var_field!((*inEquation).expReinit, SCode::Equation::EQ_REINIT).clone(), inImpl.clone(), true, inPrefix.clone(), info.clone())?;
                    (outCache, exp, prop) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), exp.clone(), prop.clone(), inImpl.clone(), info.clone())?;
                    (exp, _) = Types::matchProp(exp.clone(), prop.clone(), cr_prop.clone(), true)?;
                    (outCache, cr_exp, exp, cr_prop) = condenseArrayEquation(outCache.clone(), inEnv.clone(), var_field!((*inEquation).cref, SCode::Equation::EQ_REINIT).clone(), var_field!((*inEquation).expReinit, SCode::Equation::EQ_REINIT).clone(), cr_exp.clone(), exp.clone(), cr_prop.clone(), prop.clone(), inImpl.clone(), inPrefix.clone(), info.clone())?;
                    (outCache, cr_exp) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), cr_exp.clone(), inPrefix.clone())?;
                    (outCache, exp) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), exp.clone(), inPrefix.clone())?;
                    source = makeEqSource(info.clone(), inEnv.clone(), inPrefix.clone(), inFlattenOp.clone())?;
                    let DAE::DAE { elementLst: __pa5 } = (instEqEquation(cr_exp.clone(), cr_prop.clone(), exp.clone(), prop.clone(), source.clone(), inInitial.clone(), inImpl.clone(), Absyn::dummyInfo.clone())?) else { bail!("pattern mismatch") };
                    el = __pa5.clone();
                    el = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
                    let __x = makeDAEArrayEqToReinitForm(e.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    outDae = DAE::DAElist { elementLst: el.clone() };
                    Ok((outDae.clone(), inState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_NORETCALL { info, .. } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outDae: DAE::DAElist;
                    let mut outState: ClassInf::State;
                    let mut outSets: DAE::Connect::Sets = outSets.clone();
                    if isConnectionsOperator(var_field!((*inEquation).exp, SCode::Equation::EQ_NORETCALL).clone()) {
                        (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = handleConnectionsOperators(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), inEquation.clone(), inInitial.clone(), inImpl.clone(), inGraph.clone(), inFlattenOp.clone())?;
                    } else {
                        (outCache, exp, _) = Static::elabExp(inCache.clone(), inEnv.clone(), var_field!((*inEquation).exp, SCode::Equation::EQ_NORETCALL).clone(), inImpl.clone(), false, inPrefix.clone(), info.clone())?;
                        (outCache, exp) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), exp.clone(), inPrefix.clone())?;
                        source = makeEqSource(info.clone(), inEnv.clone(), inPrefix.clone(), inFlattenOp.clone())?;
                        outDae = instEquationNoRetCallVectorization(exp.clone(), inInitial.clone(), source.clone())?;
                        outState = inState.clone();
                    }
                    Ok((outDae.clone(), outState.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstSection.instEquationCommonWork failed for eqn: ")).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(" in scope: ")); __mm_s.push_str(&*FGraph::getGraphNameStr(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn makeEqSource(mut inInfo: SourceInfo, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inFlattenOp: Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> {
    let mut outSource: Arc<DAE::ElementSource>;
    outSource = ElementSource::createElementSource(inInfo.clone(), FGraph::getScopePath(inEnv.clone())?, inPrefix.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
    outSource = ElementSource::addSymbolicTransformation(outSource.clone(), inFlattenOp.clone())?;
    Ok(outSource)
}

fn checkIfConditionTypes(mut inAccumProp: DAE::Properties, mut inConditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inProperties: Arc<metamodelica::List<DAE::Properties>>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inAccumProp.clone()) {
        DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. } => {
            ()
        },
        _ => {
            let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut exp_str: ArcStr = arcstr::literal!("");
            let mut ty_str: ArcStr = arcstr::literal!("");
            props = inProperties.clone();
            for mut cond in &*inConditions.clone() {
                let mut cond = cond.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(props.clone()) {
                    Deref @ metamodelica::List::Cons { head: DAE::Properties::PROP { type_: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                ty = __pa0.clone();
                props = __pa1.clone();
                if !(Types::isScalarBoolean(ty.clone())) {
                    exp_str = (Dump::printExpStr(cond.clone())?).clone();
                    ty_str = (TypesDump::unparseTypeNoAttr(ty.clone())?).clone();
                    Error::addSourceMessageAndFail(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(exp_str.clone()).clone(), (ty_str.clone()).clone()], inInfo.clone())?;
                }
            }
            Error::addInternalError((literal!("InstSection.checkIfConditionTypes failed to find non-Boolean condition.")).clone(), inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkIfConditionBinding(mut inValues: Arc<Values::Value>, mut inInfo: SourceInfo) -> Result<bool> {
    let mut outHasBindings: bool = false;
    let mut empty_val: Option<Arc<Values::Value>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    empty_val = ValuesUtil::containsEmpty(inValues.clone());
    if isSome(empty_val.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(empty_val.clone()) {
            Some(Deref @ Values::Value::EMPTY { name: __pa0, .. }) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa0.clone();
        Error::addSourceMessage(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone(), list![(name.clone()).clone()], inInfo.clone())?;
        outHasBindings = false;
    } else {
        outHasBindings = true;
    }
    Ok(outHasBindings)
}

fn instOperatorArg(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inArg: Arc<Absyn::Exp>, mut inImpl: bool, mut inExpectedType: Arc<DAE::Type>, mut inOperatorName: ArcStr, mut inArgName: ArcStr, mut inArgIndex: i32, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outArg: Arc<DAE::Exp>;
    let mut props: DAE::Properties;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outCache, outArg, props) = Static::elabExp(inCache.clone(), inEnv.clone(), inArg.clone(), inImpl.clone(), true, inPrefix.clone(), inInfo.clone())?;
    ty = Types::getPropType(props.clone())?;
    if !(Types::subtype(ty.clone(), inExpectedType.clone(), true)?) {
        Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(intString(inArgIndex.clone())).clone(), (inOperatorName.clone()).clone(), (inArgName.clone()).clone(), (Dump::printExpStr(inArg.clone())?).clone(), (TypesDump::unparseTypeNoAttr(ty.clone())?).clone(), (TypesDump::unparseType(inExpectedType.clone())?).clone()], inInfo.clone())?;
    }
    (outCache, outArg, _) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), outArg.clone(), props.clone(), inImpl.clone(), inInfo.clone())?;
    (outCache, outArg) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), outArg.clone(), inPrefix.clone())?;
    Ok((outCache, outArg))
}

fn isConnectionsOperator(mut inExp: Arc<Absyn::Exp>) -> bool {
    let mut yes: bool = false;
    yes = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::List::Nil } }, .. } => {
            listMember((id.clone()).clone(), list![(literal!("root")).clone(), (literal!("potentialRoot")).clone(), (literal!("branch")).clone(), (literal!("uniqueRoot")).clone()])
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    yes
}

fn handleConnectionsOperators(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inEquation: Arc<SCode::Equation>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inGraph: ConnectionGraph::ConnectionGraph, mut flattenOp: Arc<DAE::SymbolicOperation>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), inEquation.clone(), inInitial.clone(), inImpl.clone(), inGraph.clone(), flattenOp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil }, function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "root", subscripts: Deref @ metamodelica::List::Nil } }, .. }, info, .. }, _, _, graph, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, _, _))) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    s = (SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    Error::addSourceMessage(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO.clone(), list![(s.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil }, function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "root", subscripts: Deref @ metamodelica::List::Nil } }, .. }, info, .. }, _, _, graph, _) => {
                    let mut cr_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _, _))) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    cr_ = __pa1.clone();
                    (cache, cr_) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), cr_.clone())?;
                    graph = ConnectionGraph::addDefiniteRoot(graph.clone(), cr_.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs, function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "potentialRoot", subscripts: Deref @ metamodelica::List::Nil } }, .. }, info, .. }, _, _, graph, _) => {
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    (cr, _) = potentialRootArguments(functionArgs.clone(), info.clone(), pre.clone(), inEquation.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, _, _))) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    s = (SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    Error::addSourceMessage(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO.clone(), list![(s.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs, function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "potentialRoot", subscripts: Deref @ metamodelica::List::Nil } }, .. }, info, .. }, _, _, graph, _) => {
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut ipriority: i32 = 0;
                    let mut cr_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    (cr, ipriority) = potentialRootArguments(functionArgs.clone(), info.clone(), pre.clone(), inEquation.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _, _))) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    cr_ = __pa1.clone();
                    (cache, cr_) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), cr_.clone())?;
                    graph = ConnectionGraph::addPotentialRoot(graph.clone(), cr_.clone(), intReal(ipriority.clone()))?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs, function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "uniqueRoot", subscripts: Deref @ metamodelica::List::Nil } }, .. }, info, .. }, _, _, graph, _) => {
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    (cr, _) = uniqueRootArguments(functionArgs.clone(), info.clone(), pre.clone(), inEquation.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, _, _))) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    s = (SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    Error::addSourceMessage(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO.clone(), list![(s.clone()).clone()], info.clone())?;
                    Error::addSourceMessage(Error::NON_STANDARD_OPERATOR.clone(), list![(literal!("Connections.uniqueRoot")).clone()], info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs, function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "uniqueRoot", subscripts: Deref @ metamodelica::List::Nil } }, .. }, info, .. }, _, _, graph, _) => {
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut msg: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut msg_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    (cr, msg) = uniqueRootArguments(functionArgs.clone(), info.clone(), pre.clone(), inEquation.clone())?;
                    (cache, exp, _) = Static::elabExp(cache.clone(), env.clone(), Arc::new(Absyn::Exp::CREF { componentRef: cr.clone() }), false, true, pre.clone(), info.clone())?;
                    (cache, msg_1, _) = Static::elabExp(cache.clone(), env.clone(), msg.clone(), false, false, pre.clone(), info.clone())?;
                    (cache, exp) = PrefixUtil::prefixExp(cache.clone(), env.clone(), ih.clone(), exp.clone(), pre.clone())?;
                    (cache, msg_1) = PrefixUtil::prefixExp(cache.clone(), env.clone(), ih.clone(), msg_1.clone(), pre.clone())?;
                    graph = ConnectionGraph::addUniqueRoots(graph.clone(), exp.clone(), msg_1.clone())?;
                    Error::addSourceMessage(Error::NON_STANDARD_OPERATOR.clone(), list![(literal!("Connections.uniqueRoot")).clone()], info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr2 }, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil }, function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "branch", subscripts: Deref @ metamodelica::List::Nil } }, .. }, info, .. }, _, _, graph, _) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut cr1_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cr2_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr1.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, _, _))) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e_1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr2.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa2, Some((__pa3, _, _))) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    e_2 = __pa3.clone();
                    b1 = Types::isZeroLengthArray(Expression::r#typeof(e_1.clone())?);
                    b2 = Types::isZeroLengthArray(Expression::r#typeof(e_2.clone())?);
                    if boolOr(b1.clone(), b2.clone()) {
                        s = (SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?).clone();
                        Error::addSourceMessage(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO.clone(), list![(s.clone()).clone()], info.clone())?;
                    } else {
                        let __pa4 = ::match_deref::match_deref! { match &(e_1.clone()) {
                            Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: _ } => __pa4.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        cr1_ = __pa4.clone();
                        let __pa5 = ::match_deref::match_deref! { match &(e_2.clone()) {
                            Deref @ DAE::Exp::CREF { componentRef: __pa5, ty: _ } => __pa5.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        cr2_ = __pa5.clone();
                        (cache, cr1_) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), cr1_.clone())?;
                        (cache, cr2_) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), cr2_.clone())?;
                        graph = ConnectionGraph::addBranch(graph.clone(), cr1_.clone(), cr2_.clone())?;
                    }
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, _, eqn, _, _, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    s = (SCodeDump::equationStr(eqn.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    Debug::trace((literal!("- handleConnectionsOperators failed for eqn: ")).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" in scope:")); __mm_s.push_str(&*FGraph::getGraphNameStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn potentialRootArguments(mut inFunctionArgs: Arc<Absyn::FunctionArgs>, mut info: SourceInfo, mut inPrefix: DAE::Prefix, mut inEquation: Arc<SCode::Equation>) -> Result<(Arc<Absyn::ComponentRef>, i32)> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut outPriority: i32 = 0;
    (outCref, outPriority) = 'mc: {
        let __mc_input = inFunctionArgs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil } => {
                    Ok((cr.clone(), 0))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::INTEGER { value: p }, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil } => {
                    Ok((cr.clone(), p.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "priority", argValue: Deref @ Absyn::Exp::INTEGER { value: p } }, tail: Deref @ metamodelica::List::Nil } } => {
                    Ok((cr.clone(), p.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s1 = (SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s2 = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
                    Error::addSourceMessage(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCref, outPriority))
}

fn uniqueRootArguments(mut inFunctionArgs: Arc<Absyn::FunctionArgs>, mut info: SourceInfo, mut inPrefix: DAE::Prefix, mut inEquation: Arc<SCode::Equation>) -> Result<(Arc<Absyn::ComponentRef>, Arc<Absyn::Exp>)> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut outMessage: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (outCref, outMessage) = 'mc: {
        let __mc_input = inFunctionArgs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil } => {
                    Ok((cr.clone(), Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Cons { head: msg, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil } => {
                    Ok((cr.clone(), msg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "message", argValue: msg }, tail: Deref @ metamodelica::List::Nil } } => {
                    Ok((cr.clone(), msg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s1 = (SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s2 = (PrefixUtil::printPrefixStr3(inPrefix.clone())?).clone();
                    Error::addSourceMessage(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCref, outMessage))
}

fn checkReinitType(mut inType: Arc<DAE::Type>, mut inProperties: DAE::Properties, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<bool> {
    let mut outSucceeded: bool = false;
    outSucceeded = 'mc: {
        let __mc_input = inProperties.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cref_str: ArcStr = arcstr::literal!("");
            let mut ty_str: ArcStr = arcstr::literal!("");
            ty = Types::arrayElementType(inType.clone());
            let false = (Types::isReal(ty.clone())) else { bail!("pattern mismatch") };
            cref_str = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
            ty_str = (TypesDump::unparseType(ty.clone())?).clone();
            Error::addSourceMessage(Error::REINIT_MUST_BE_REAL.clone(), list![(cref_str.clone()).clone(), (ty_str.clone()).clone()], inInfo.clone())?;
            Ok(false)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP { constFlag: mut cnst, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut cref_str: ArcStr = arcstr::literal!("");
            let mut cnst_str: ArcStr = arcstr::literal!("");
            let false = (Types::isVar(cnst.clone())) else { bail!("pattern mismatch") };
            cnst_str = (TypesDump::unparseConst(cnst.clone())?).clone();
            cref_str = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
            Error::addSourceMessage(Error::REINIT_MUST_BE_VAR.clone(), list![(cref_str.clone()).clone(), (cnst_str.clone()).clone()], inInfo.clone())?;
            Ok(false)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSucceeded)
}

fn checkTupleCallEquationMessage(mut left: Arc<Absyn::Exp>, mut right: Arc<Absyn::Exp>, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((AbsynUtil::stripCommentExpressions(left.clone(), false)?, AbsynUtil::stripCommentExpressions(right.clone(), false)?)) {
        (Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _) => {
            ()
        },
        (Deref @ Absyn::Exp::TUPLE { expressions: crefs }, Deref @ Absyn::Exp::CALL { .. }) => {
            let mut left_str: ArcStr = arcstr::literal!("");
            let mut right_str: ArcStr = arcstr::literal!("");
            if !(List::all(crefs.clone(), Arc::new(fnptr!(AbsynUtil::isCref, Arc<Absyn::Exp>)))) {
                left_str = (Dump::printExpStr(left.clone())?).clone();
                right_str = (Dump::printExpStr(right.clone())?).clone();
                Error::addSourceMessageAndFail(Error::TUPLE_ASSIGN_CREFS_ONLY.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*left_str.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*right_str.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
            }
            ()
        },
        (Deref @ Absyn::Exp::TUPLE { .. }, _) => {
            let mut left_str: ArcStr = arcstr::literal!("");
            let mut right_str: ArcStr = arcstr::literal!("");
            left_str = (Dump::printExpStr(left.clone())?).clone();
            right_str = (Dump::printExpStr(right.clone())?).clone();
            Error::addSourceMessage(Error::TUPLE_ASSIGN_FUNCALL_ONLY.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*left_str.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*right_str.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn instEquationNoRetCallVectorization(mut exp: Arc<DAE::Exp>, mut initial_: SCode::Initial, mut source: Arc<DAE::ElementSource>) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist;
    dae = (match initial_.clone() {
        SCode::Initial::NON_INITIAL => DAE::DAElist { elementLst: list![Arc::new(DAE::Element::NORETCALL { exp: exp.clone(), source: source.clone() })] },
        SCode::Initial::INITIAL => DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIAL_NORETCALL { exp: exp.clone(), source: source.clone() })] },
        _ => bail!("match: no arm matched"),
    });
    Ok(dae)
}

fn makeDAEArrayEqToReinitForm(mut inEq: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    outEqn = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, scalar: e, source } => {
            Arc::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr1, exp: e, source } => {
            Arc::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source } => {
            let mut e2: Arc<DAE::Exp>;
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            t = ComponentReference::crefLastType(cr2.clone())?;
            e2 = Expression::makeCrefExp(cr2.clone(), t.clone())?;
            Arc::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e2.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::ARRAY_EQUATION { source, array: e, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, .. } => {
            Arc::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e.clone(), source: source.clone() })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("Failure in: makeDAEArrayEqToReinitForm")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

fn condenseArrayEquation(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ie1: Arc<Absyn::Exp>, mut ie2: Arc<Absyn::Exp>, mut elabedE1: Arc<DAE::Exp>, mut elabedE2: Arc<DAE::Exp>, mut iprop: DAE::Properties, mut iprop2: DAE::Properties, mut r#impl: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outE1: Arc<DAE::Exp>;
    let mut outE2: Arc<DAE::Exp>;
    let mut oprop: DAE::Properties;
    (outCache, outE1, outE2, oprop) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), ie1.clone(), ie2.clone(), elabedE1.clone(), elabedE2.clone(), iprop.clone(), iprop2.clone(), r#impl.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e1, e2, _, _, prop, prop2, _, pre, _) => {
                    let mut b3: bool = false;
                    let mut b4: bool = false;
                    let mut elabedE1_2: Arc<DAE::Exp>;
                    let mut elabedE2_2: Arc<DAE::Exp>;
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let mut e1 = (*e1).clone();
                    let mut prop = (*prop).clone();
                    let mut prop2 = (*prop2).clone();
                    let true = (Flags::getConfigBool(Flags::CONDENSE_ARRAYS.clone())?) else { bail!("pattern mismatch") };
                    b3 = Types::isPropTupleArray(prop.clone())?;
                    b4 = Types::isPropTupleArray(prop2.clone())?;
                    let true = (boolOr(b3.clone(), b4.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::containFunctioncall(elabedE2.clone())) else { bail!("pattern mismatch") };
                    (e1, prop) = expandTupleEquationWithWild(e1.clone(), prop2.clone(), prop.clone())?;
                    (cache, elabedE1_2, prop1) = Static::elabExpLHS(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
                    (cache, elabedE1_2, prop1) = Ceval::cevalIfConstant(cache.clone(), env.clone(), elabedE1_2.clone(), prop1.clone(), r#impl.clone(), info.clone())?;
                    (cache, elabedE2_2, prop2) = Static::elabExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
                    (cache, elabedE2_2, prop2) = Ceval::cevalIfConstant(cache.clone(), env.clone(), elabedE2_2.clone(), prop2.clone(), r#impl.clone(), info.clone())?;
                    Ok((cache.clone(), elabedE1_2.clone(), elabedE2_2.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, _, _, prop, _, _, _, _) => {
                    Ok((cache.clone(), elabedE1.clone(), elabedE2.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outE1, outE2, oprop))
}

fn expandTupleEquationWithWild(mut inExp: Arc<Absyn::Exp>, mut propCall: DAE::Properties, mut propTuple: DAE::Properties) -> Result<(Arc<Absyn::Exp>, DAE::Properties)> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut oprop: DAE::Properties;
    (outExp, oprop) = 'mc: {
        let __mc_input = (inExp.clone(), propCall.clone(), propTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::TUPLE { expressions: aexpl }, DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { names, types: typeList }, .. }, DAE::Properties::PROP_TUPLE { tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: tupleConst }, type_: Deref @ DAE::Type::T_TUPLE { types: lst, .. } }) => {
                    let mut aexpl2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut fillValue: i32 = 0;
                    let mut lst2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tupleConst2: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
                    fillValue = (typeList.clone().len() as i32) - (aexpl.clone().len() as i32);
                    lst2 = List::fill(DAE::T_ANYTYPE_DEFAULT.clone(), fillValue.clone());
                    aexpl2 = List::fill(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(openmodelica_ast::Absyn::ComponentRef::WILD) }), fillValue.clone());
                    tupleConst2 = List::fill(Arc::new(DAE::TupleConst::SINGLE_CONST { r#const: openmodelica_frontend_types::DAE::Const::C_VAR }), fillValue.clone());
                    aexpl2 = listAppend(aexpl.clone(), aexpl2.clone());
                    lst2 = listAppend(lst.clone(), lst2.clone());
                    tupleConst2 = listAppend(tupleConst.clone(), tupleConst2.clone());
                    Ok((Arc::new(Absyn::Exp::TUPLE { expressions: aexpl2.clone() }), DAE::Properties::PROP_TUPLE { type_: Arc::new(DAE::Type::T_TUPLE { types: lst2.clone(), names: names.clone() }), tupleConst: Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: tupleConst2.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: typeList, names }, .. }, DAE::Properties::PROP { type_: propType, constFlag: tconst }) => {
                    let mut aexpl: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut aexpl2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut fillValue: i32 = 0;
                    let mut lst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tupleConst: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
                    let mut tupleConst2: Arc<metamodelica::List<Arc<DAE::TupleConst>>> = metamodelica::nil();
                    fillValue = (typeList.clone().len() as i32) - 1;
                    aexpl2 = List::fill(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(openmodelica_ast::Absyn::ComponentRef::WILD) }), fillValue.clone());
                    lst2 = List::fill(DAE::T_ANYTYPE_DEFAULT.clone(), fillValue.clone());
                    tupleConst2 = List::fill(Arc::new(DAE::TupleConst::SINGLE_CONST { r#const: openmodelica_frontend_types::DAE::Const::C_VAR }), fillValue.clone());
                    aexpl = cons(inExp.clone(), aexpl2.clone());
                    lst = cons(propType.clone(), lst2.clone());
                    tupleConst = cons(Arc::new(DAE::TupleConst::SINGLE_CONST { r#const: tconst.clone() }), tupleConst2.clone());
                    Ok((Arc::new(Absyn::Exp::TUPLE { expressions: aexpl.clone() }), DAE::Properties::PROP_TUPLE { type_: Arc::new(DAE::Type::T_TUPLE { types: lst.clone(), names: names.clone() }), tupleConst: Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: tupleConst.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    if !((!(Types::isPropTuple(propCall.clone())?))) { bail!("guard") }
                    Ok((inExp.clone(), propTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- expandTupleEquationWithWild failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, oprop))
}

fn instEquationCommonCiTrans(mut inState: ClassInf::State, mut inInitial: SCode::Initial) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = (match inInitial.clone() {
        SCode::Initial::NON_INITIAL => ClassInfUtil::trans(inState.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_EQUATION)?,
        _ => inState.clone(),
    });
    Ok(outState)
}

fn unroll(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inIdent: Ident, mut inIteratorType: Arc<DAE::Type>, mut inValue: Arc<Values::Value>, mut inEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inInitial: SCode::Initial, mut inImplicit: bool, mut inGraph: ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, DAE::DAElist, DAE::Connect::Sets, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets = inSets.clone();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph.clone();
    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut env: FCore::Graph;
    let mut ci_state: ClassInf::State = inState.clone();
    let mut daes: Arc<metamodelica::List<DAE::DAElist>> = metamodelica::nil();
    let mut dae: DAE::DAElist;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(inValue.clone()) {
            Deref @ Values::Value::ARRAY { valueLst: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        values = __pa1.clone();
        for mut val in &*values.clone() {
            let mut val = val.clone();
            env = unwrap_break_err!(FGraph::openScope(inEnv.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (arcstr::literal!(FCore::forScopeName)).clone(), None), '__try0);
            env = unwrap_break_err!(FGraph::addForIterator(env.clone(), (inIdent.clone()).clone(), inIteratorType.clone(), Arc::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::CONST, Some(openmodelica_frontend_types::DAE::Const::C_CONST)), '__try0);
            (outCache, _, _, dae, outSets, ci_state, outGraph) = unwrap_break_err!(Inst::instList(outCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), outSets.clone(), ci_state.clone(), (if (SCodeUtil::isInitial(inInitial.clone())) { (Arc::new(instInitialEquation) as _) } else { (Arc::new(instEquation) as _) }), inEquations.clone(), inImplicit.clone(), alwaysUnroll.clone(), outGraph.clone()), '__try0);
            daes = cons(dae.clone(), daes.clone());
        }
        outDae = List::fold(daes.clone(), Arc::new(DAEUtil::joinDaes), DAE::emptyDae.clone());
        Ok::<_, anyhow::Error>((outDae.clone(), values.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outDae = __try0_o0;
            values = __try0_o1;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstSection.unroll failed: ")); __mm_s.push_str(&*ValuesDump::valString(inValue.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail");
        }
    }
    Ok((outCache, outDae, outSets, outGraph))
}

fn addForLoopScope(mut env: FCore::Graph, mut iterName: Ident, mut iterType: Arc<DAE::Type>, mut iterVariability: SCode::Variability, mut constOfForIteratorRange: Option<DAE::Const>) -> Result<FCore::Graph> {
    let mut newEnv: FCore::Graph;
    newEnv = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (arcstr::literal!(FCore::forScopeName)).clone(), None)?;
    newEnv = FGraph::addForIterator(newEnv.clone(), (iterName.clone()).clone(), iterType.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), iterVariability.clone(), constOfForIteratorRange.clone())?;
    Ok(newEnv)
}

fn addParForLoopScope(mut env: FCore::Graph, mut iterName: Ident, mut iterType: Arc<DAE::Type>, mut iterVariability: SCode::Variability, mut constOfForIteratorRange: Option<DAE::Const>) -> Result<FCore::Graph> {
    let mut newEnv: FCore::Graph;
    newEnv = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (arcstr::literal!(FCore::parForScopeName)).clone(), None)?;
    newEnv = FGraph::addForIterator(newEnv.clone(), (iterName.clone()).clone(), iterType.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), iterVariability.clone(), constOfForIteratorRange.clone())?;
    Ok(newEnv)
}

pub fn instEqEquation(mut inExp1: Arc<DAE::Exp>, mut inProperties2: DAE::Properties, mut inExp3: Arc<DAE::Exp>, mut inProperties4: DAE::Properties, mut source: Arc<DAE::ElementSource>, mut inInitial5: SCode::Initial, mut inImplicit: bool, mut extraInfo: SourceInfo) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = (inExp1.clone(), inProperties2.clone(), inExp3.clone(), inProperties4.clone(), source.clone(), inInitial5.clone(), inImplicit.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1 @ Deref @ DAE::Exp::CREF { .. }, p1 @ DAE::Properties::PROP { type_: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, e2, p2 @ DAE::Properties::PROP { constFlag: c, .. }, _, initial_, _) => {
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut e1 = (*e1).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e2.clone(), p2.clone(), p1.clone(), true)?) {
                        (__pa0, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2_1 = __pa0.clone();
                    t_1 = __pa1.clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
                    dae = instEqEquation2(e1.clone(), e2_1.clone(), t_1.clone(), c.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP { .. }, e2, p2 @ DAE::Properties::PROP { constFlag: c, .. }, _, initial_, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut e2 = (*e2).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e1.clone(), p1.clone(), p2.clone(), false)?) {
                        (__pa0, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    t_1 = __pa1.clone();
                    (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    dae = instEqEquation2(e1_1.clone(), e2.clone(), t_1.clone(), c.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP { .. }, e2, p2 @ DAE::Properties::PROP { constFlag: c, .. }, _, initial_, _) => {
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut e1 = (*e1).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e2.clone(), p2.clone(), p1.clone(), true)?) {
                        (__pa0, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2_1 = __pa0.clone();
                    t_1 = __pa1.clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
                    dae = instEqEquation2(e1.clone(), e2_1.clone(), t_1.clone(), c.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP_TUPLE { .. }, e2, p2 @ DAE::Properties::PROP_TUPLE { tupleConst: tp, .. }, _, initial_, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut e2 = (*e2).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e1.clone(), p1.clone(), p2.clone(), false)?) {
                        (__pa0, DAE::Properties::PROP_TUPLE { type_: __pa1, tupleConst: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    t_1 = __pa1.clone();
                    (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    c = Types::propTupleAllConst(tp.clone())?;
                    dae = instEqEquation2(e1_1.clone(), e2.clone(), t_1.clone(), c.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP_TUPLE { .. }, e2, p2 @ DAE::Properties::PROP_TUPLE { tupleConst: tp, .. }, _, initial_, _) => {
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut e1 = (*e1).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e2.clone(), p2.clone(), p1.clone(), true)?) {
                        (__pa0, DAE::Properties::PROP_TUPLE { type_: __pa1, tupleConst: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2_1 = __pa0.clone();
                    t_1 = __pa1.clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
                    c = Types::propTupleAllConst(tp.clone())?;
                    dae = instEqEquation2(e1.clone(), e2_1.clone(), t_1.clone(), c.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1 @ Deref @ DAE::Exp::CREF { .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ENUMERATION { .. }, .. }, e2, DAE::Properties::PROP { constFlag: c, type_: t @ Deref @ DAE::Type::T_ENUMERATION { .. } }, _, initial_, _) => {
                    let mut dae: DAE::DAElist;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    dae = instEqEquation2(e1.clone(), e2.clone(), t.clone(), c.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP { .. }, e2, DAE::Properties::PROP_TUPLE { .. }, _, initial_, _) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut p2: DAE::Properties;
                    let mut c: DAE::Const = DAE::Const::C_CONST;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    p2 = Types::propTupleFirstProp(inProperties4.clone())?;
                    let DAE::PROP { constFlag: __pa0, .. } = (p2.clone()) else { bail!("pattern mismatch") };
                    c = __pa0.clone();
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(Types::matchProp(e1.clone(), p1.clone(), p2.clone(), false)?) {
                        (__pa1, DAE::Properties::PROP { type_: __pa2, .. }) => (__pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa1.clone();
                    t_1 = __pa2.clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    e2 = Arc::new(DAE::Exp::TSUB { exp: e2.clone(), ix: 1, ty: t_1.clone() });
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    dae = instEqEquation2(e1.clone(), e2.clone(), t_1.clone(), c.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Properties::PROP { type_: t1, .. }, e2, DAE::Properties::PROP { type_: t2, .. }, _, _, _) => {
                    let mut e1_str: ArcStr = arcstr::literal!("");
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut e2_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo;
                    e1_str = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    t1_str = (TypesDump::unparseTypeNoAttr(t1.clone())?).clone();
                    e2_str = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    t2_str = (TypesDump::unparseTypeNoAttr(t2.clone())?).clone();
                    s1 = stringAppendList(list![(e1_str.clone()).clone(), (literal!("=")).clone(), (e2_str.clone()).clone()]);
                    s2 = stringAppendList(list![(t1_str.clone()).clone(), (literal!("=")).clone(), (t2_str.clone()).clone()]);
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Types::typeErrorSanityCheck((t1_str.clone()).clone(), (t2_str.clone()).clone(), info.clone())?;
                    Error::addMultiSourceMessage(Error::EQUATION_TYPE_MISMATCH_ERROR.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], if (extraInfo.fileName.clone() == literal!("")) {list![info.clone()]} else {list![extraInfo.clone(), info.clone()]})?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDae)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn instEqEquation2(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inType3: Arc<DAE::Type>, mut inConst: DAE::Const, mut source: Arc<DAE::ElementSource>, mut inInitial4: SCode::Initial) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = (inExp1.clone(), inExp2.clone(), inType3.clone(), inConst.clone(), source.clone(), inInitial4.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_INTEGER { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_REAL { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_STRING { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_BOOL { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_CLOCK { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, e2, Deref @ DAE::Type::T_ENUMERATION { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeDefine(cr.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, Deref @ DAE::Type::T_ENUMERATION { .. }, _, _, initial_) => {
                    Ok(makeDaeDefine(cr.clone(), e1.clone(), source.clone(), initial_.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_ENUMERATION { .. }, _, _, initial_) => {
                    Ok(makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, tt @ Deref @ DAE::Type::T_ARRAY { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = instArrayEquation(e1.clone(), e2.clone(), tt.clone(), inConst.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: exps1 }, e2, Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    let mut e1: Arc<DAE::Exp>;
                    let mut exps1 = (*exps1).clone();
                    exps1 = List::map(exps1.clone(), Arc::new(Expression::emptyToWild));
                    checkNoDuplicateAssignments(exps1.clone(), ElementSource::getElementSourceFileInfo(source.clone()))?;
                    e1 = Arc::new(DAE::Exp::TUPLE { PR: exps1.clone() });
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_TUPLE { .. }, _, _, initial_) => {
                    if !((!(Expression::isTuple(e1.clone())))) { bail!("guard") }
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_METALIST { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_METATUPLE { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_METAOPTION { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_METAUNIONTYPE { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tt, .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = instEqEquation2(e1.clone(), e2.clone(), tt.clone(), inConst.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_COMPLEX { varLst: vs, .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    let mut exps1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exps2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    exps1 = Expression::splitRecord(e1.clone(), inType3.clone())?;
                    exps2 = Expression::splitRecord(e2.clone(), inType3.clone())?;
                    tys = List::map(vs.clone(), Arc::new(Types::getVarType));
                    dae = instEqEquation2List(exps1.clone(), exps2.clone(), tys.clone(), inConst.clone(), source.clone(), initial_.clone(), metamodelica::nil())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, tt @ Deref @ DAE::Type::T_COMPLEX { .. }, _, _, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = instComplexEquation(e1.clone(), e2.clone(), tt.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstSection.instEqEquation2 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDae)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn instEqEquation2List(mut inExps1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExps2: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTypes3: Arc<metamodelica::List<Arc<DAE::Type>>>, mut r#const: DAE::Const, mut source: Arc<DAE::ElementSource>, mut initial_: SCode::Initial, mut acc: Arc<metamodelica::List<DAE::DAElist>>) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((inExps1.clone(), inExps2.clone(), inTypes3.clone(), r#const.clone(), source.clone(), initial_.clone(), acc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            DAEUtil::joinDaeLst(acc.clone().reverse())?
        },
        (Deref @ metamodelica::List::Cons { head: exp1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: exp2, tail: rest2 }, Deref @ metamodelica::List::Cons { head: ty, tail: rest3 }, _, _, _, _) => {
            let mut res: DAE::DAElist;
            res = instEqEquation2(exp1.clone(), exp2.clone(), ty.clone(), r#const.clone(), source.clone(), initial_.clone())?;
            instEqEquation2List(rest1.clone(), rest2.clone(), rest3.clone(), r#const.clone(), source.clone(), initial_.clone(), cons(res.clone(), acc.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDae)
}

pub fn makeDaeEquation(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>, mut inInitial3: SCode::Initial) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone(), inSource.clone(), inInitial3.clone())) {
        (e1, e2, source, SCode::Initial::NON_INITIAL) => {
            let mut elt: Arc<DAE::Element>;
            let mut source = (*source).clone();
            elt = Arc::new(DAE::Element::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone() });
            source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
            DAE::DAElist { elementLst: list![Arc::new(DAE::Element::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone() })] }
        },
        (e1, e2, source, SCode::Initial::INITIAL) => {
            let mut elt: Arc<DAE::Element>;
            let mut source = (*source).clone();
            elt = Arc::new(DAE::Element::INITIALEQUATION { exp1: e1.clone(), exp2: e2.clone(), source: source.clone() });
            source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
            DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIALEQUATION { exp1: e1.clone(), exp2: e2.clone(), source: source.clone() })] }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDae)
}

fn makeDaeDefine(mut inComponentRef: Arc<DAE::ComponentRef>, mut inExp: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((inComponentRef.clone(), inExp.clone(), source.clone(), inInitial.clone())) {
        (cr, e2, _, SCode::Initial::NON_INITIAL) => {
            DAE::DAElist { elementLst: list![Arc::new(DAE::Element::DEFINE { componentRef: cr.clone(), exp: e2.clone(), source: source.clone() })] }
        },
        (cr, e2, _, SCode::Initial::INITIAL) => {
            DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIALDEFINE { componentRef: cr.clone(), exp: e2.clone(), source: source.clone() })] }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDae)
}

fn instArrayEquation(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>, mut inConst: DAE::Const, mut inSource: Arc<DAE::ElementSource>, mut initial_: SCode::Initial) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist;
    dae = 'mc: {
        let __mc_input = (lhs.clone(), rhs.clone(), tp.clone(), inConst.clone(), inSource.clone(), initial_.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, source, SCode::Initial::INITIAL) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut ds: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut elt: Arc<DAE::Element>;
                    let mut source = (*source).clone();
                    b1 = Expression::containVectorFunctioncall(lhs.clone());
                    b2 = Expression::containVectorFunctioncall(rhs.clone());
                    let true = (boolOr(b1.clone(), b2.clone())) else { bail!("pattern mismatch") };
                    ds = TypesDump::getDimensions(tp.clone());
                    elt = Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() });
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, source, SCode::Initial::NON_INITIAL) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut ds: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut elt: Arc<DAE::Element>;
                    let mut source = (*source).clone();
                    b1 = Expression::containVectorFunctioncall(lhs.clone());
                    b2 = Expression::containVectorFunctioncall(rhs.clone());
                    let true = (boolOr(b1.clone(), b2.clone())) else { bail!("pattern mismatch") };
                    ds = TypesDump::getDimensions(tp.clone());
                    elt = Arc::new(DAE::Element::ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() });
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, ty: t }, _, _, _) => {
                    let mut lhs_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut rhs_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut lhs_idxs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut rhs_idxs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dae: DAE::DAElist;
                    let false = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(lhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    lhs_dim = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(Expression::r#typeof(rhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs_dim = __pa1.clone();
                    lhs_idxs = expandArrayDimension(lhs_dim.clone(), lhs.clone())?;
                    rhs_idxs = expandArrayDimension(rhs_dim.clone(), rhs.clone())?;
                    dae = instArrayElEq(lhs.clone(), rhs.clone(), t.clone(), inConst.clone(), lhs_idxs.clone(), rhs_idxs.clone(), inSource.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, ty: t }, _, _, _) => {
                    let mut lhs_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut rhs_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut lhs_idxs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut rhs_idxs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dae: DAE::DAElist;
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let true = (Expression::dimensionKnown(dim.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(lhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    lhs_dim = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(Expression::r#typeof(rhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs_dim = __pa1.clone();
                    lhs_idxs = expandArrayDimension(lhs_dim.clone(), lhs.clone())?;
                    rhs_idxs = expandArrayDimension(rhs_dim.clone(), rhs.clone())?;
                    dae = instArrayElEq(lhs.clone(), rhs.clone(), t.clone(), inConst.clone(), lhs_idxs.clone(), rhs_idxs.clone(), inSource.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, .. }, _, source, _) => {
                    let mut b: bool = false;
                    let mut ds: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut elt: Arc<DAE::Element>;
                    let mut source = (*source).clone();
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let true = (Expression::dimensionKnown(dim.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isRange(lhs.clone()) || Expression::isRange(rhs.clone()) || Expression::isReduction(lhs.clone()) || Expression::isReduction(rhs.clone())) else { bail!("pattern mismatch") };
                    ds = TypesDump::getDimensions(tp.clone());
                    b = SCodeUtil::isInitial(initial_.clone());
                    elt = if (b.clone()) {Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })} else {Arc::new(DAE::Element::ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })};
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    elt = if (b.clone()) {Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })} else {Arc::new(DAE::Element::ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })};
                    Ok(DAE::DAElist { elementLst: list![elt.clone()] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, ty: t }, _, _, _) => {
                    let mut lhs_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut rhs_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut lhs_idxs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut rhs_idxs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut dae: DAE::DAElist;
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let false = (Expression::dimensionKnown(dim.clone())) else { bail!("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(lhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    lhs_dim = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(Expression::r#typeof(rhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs_dim = __pa1.clone();
                    lhs_idxs = expandArrayDimension(lhs_dim.clone(), lhs.clone())?;
                    rhs_idxs = expandArrayDimension(rhs_dim.clone(), rhs.clone())?;
                    dae = instArrayElEq(lhs.clone(), rhs.clone(), t.clone(), inConst.clone(), lhs_idxs.clone(), rhs_idxs.clone(), inSource.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil }, .. }, _, source, SCode::Initial::INITIAL) => {
                    let mut elt: Arc<DAE::Element>;
                    let mut source = (*source).clone();
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    elt = Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })], exp: lhs.clone(), array: rhs.clone(), source: source.clone() });
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })], exp: lhs.clone(), array: rhs.clone(), source: source.clone() })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil }, .. }, _, source, SCode::Initial::NON_INITIAL) => {
                    let mut elt: Arc<DAE::Element>;
                    let mut source = (*source).clone();
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    elt = Arc::new(DAE::Element::ARRAY_EQUATION { dimension: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })], exp: lhs.clone(), array: rhs.clone(), source: source.clone() });
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::ARRAY_EQUATION { dimension: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })], exp: lhs.clone(), array: rhs.clone(), source: source.clone() })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil }, .. }, _, _, _) => {
                    let mut lhs_str: ArcStr = arcstr::literal!("");
                    let mut rhs_str: ArcStr = arcstr::literal!("");
                    let mut eq_str: ArcStr = arcstr::literal!("");
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let false = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    lhs_str = (ExpressionBasics::printExpStr(lhs.clone())?).clone();
                    rhs_str = (ExpressionBasics::printExpStr(rhs.clone())?).clone();
                    eq_str = stringAppendList(list![(lhs_str.clone()).clone(), (literal!("=")).clone(), (rhs_str.clone()).clone()]);
                    Error::addSourceMessage(Error::INST_ARRAY_EQ_UNKNOWN_SIZE.clone(), list![(eq_str.clone()).clone()], ElementSource::getElementSourceFileInfo(inSource.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstSection.instArrayEquation failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(dae)
}

fn instArrayElEq(mut inLhsExp: Arc<DAE::Exp>, mut inRhsExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inConst: DAE::Const, mut inLhsIndices: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inRhsIndices: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inSource: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial) -> Result<DAE::DAElist> {
    let mut outDAE: DAE::DAElist = DAE::emptyDae.clone();
    let mut rhs_idx: Arc<DAE::Exp>;
    let mut rhs_idxs: Arc<metamodelica::List<Arc<DAE::Exp>>> = inRhsIndices.clone().reverse();
    let mut dae: DAE::DAElist;
    for mut lhs_idx in &*inLhsIndices.clone().reverse() {
        let mut lhs_idx = lhs_idx.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rhs_idxs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        rhs_idx = __pa0.clone();
        rhs_idxs = __pa1.clone();
        dae = instEqEquation2(lhs_idx.clone(), rhs_idx.clone(), inType.clone(), inConst.clone(), inSource.clone(), inInitial.clone())?;
        outDAE = DAEUtil::joinDaes(dae.clone(), outDAE.clone())?;
    }
    Ok(outDAE)
}

fn unrollForLoop(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inIterator: ArcStr, mut inRange: Arc<DAE::Exp>, mut inRangeProps: DAE::Properties, mut inBody: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut inStatement: Arc<SCode::Statement>, mut inInfo: SourceInfo, mut inSource: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inUnrollLoops: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut range: Arc<DAE::Exp>;
    let mut env: FCore::Graph;
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut r#str: ArcStr = arcstr::literal!("");
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Types::getPropType(inRangeProps.clone()), '__try0)) {
            Deref @ DAE::Type::T_ARRAY { ty: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        ty = __pa1.clone();
        c = unwrap_break_err!(Types::getPropConst(inRangeProps.clone()), '__try0);
        let true = (Types::isParameterOrConstant(c.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        env = unwrap_break_err!(addForLoopScope(inEnv.clone(), (inIterator.clone()).clone(), ty.clone(), openmodelica_frontend_types::SCode::Variability::VAR, Some(c.clone())), '__try0);
        (outCache, val) = unwrap_break_err!(Ceval::ceval(inCache.clone(), env.clone(), inRange.clone(), inImpl.clone(), Absyn::Msg::MSG { info: inInfo.clone() }, 0), '__try0);
        (outCache, outStatements) = unwrap_break_err!(loopOverRange(inCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), (inIterator.clone()).clone(), val.clone(), inBody.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
        Ok::<_, anyhow::Error>((c.clone(), env.clone(), outCache.clone(), outStatements.clone(), ty.clone(), val.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            c = __try0_o0;
            env = __try0_o1;
            outCache = __try0_o2;
            outStatements = __try0_o3;
            ty = __try0_o4;
            val = __try0_o5;
        }
        Err(_) => {
            Error::addSourceMessageAndFail(Error::UNROLL_LOOP_CONTAINING_WHEN.clone(), list![(SCodeDump::statementStr(inStatement.clone(), SCodeDump::defaultOptions.clone())?).clone()], inInfo.clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((outCache, outStatements))
}

fn instForStatement(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inForStatement: Arc<SCode::Statement>, mut inSource: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inUnrollLoops: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut iterator: ArcStr = arcstr::literal!("");
    let mut oarange: Option<Arc<Absyn::Exp>> = None;
    let mut arange: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut range: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let mut info: SourceInfo;
    let mut iter_crefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inForStatement.clone()) {
        Deref @ SCode::Statement::ALG_FOR { info: __pa0, forBody: __pa1, range: __pa2, index: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    body = __pa1.clone();
    oarange = __pa2.clone();
    iterator = __pa3.clone();
    if isSome(oarange.clone()) {
        let __pa4 = ::match_deref::match_deref! { match &(oarange.clone()) {
            Some(__pa4) => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        arange = __pa4.clone();
        (outCache, range, prop) = Static::elabExp(inCache.clone(), inEnv.clone(), arange.clone(), inImpl.clone(), true, inPrefix.clone(), info.clone())?;
    } else {
        iter_crefs = SCodeUtil::findIteratorIndexedCrefsInStatements(body.clone(), (iterator.clone()).clone(), metamodelica::nil());
        (range, prop, outCache) = Static::deduceIterationRange((iterator.clone()).clone(), iter_crefs.clone(), inEnv.clone(), inCache.clone(), info.clone())?;
    }
    if containsWhenStatements(body.clone())? {
        (outCache, outStatements) = unrollForLoop(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), (iterator.clone()).clone(), range.clone(), prop.clone(), body.clone(), inForStatement.clone(), info.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone())?;
    } else {
        (outCache, outStatements) = instForStatement_dispatch(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), (iterator.clone()).clone(), range.clone(), prop.clone(), body.clone(), info.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone())?;
    }
    Ok((outCache, outStatements))
}

fn instForStatement_dispatch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inIterator: ArcStr, mut inRange: Arc<DAE::Exp>, mut inRangeProps: DAE::Properties, mut inBody: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut inInfo: SourceInfo, mut inSource: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inUnrollLoops: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut env: FCore::Graph;
    let mut source: Arc<DAE::ElementSource>;
    let mut range: Arc<DAE::Exp>;
    c = Types::getPropConst(inRangeProps.clone())?;
    if Types::isParameterOrConstant(c.clone()) {
        if '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Ceval::ceval(outCache.clone(), inEnv.clone(), inRange.clone(), inImpl.clone(), Absyn::Msg::MSG { info: inInfo.clone() }, 0), '__try0)) {
                (__pa1, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, .. }) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            outCache = __pa1.clone();
            outStatements = metamodelica::nil();
            return Ok((outCache, outStatements));
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    ty = Types::getPropType(inRangeProps.clone())?;
    ty = getIteratorType(ty.clone(), (inIterator.clone()).clone(), inInfo.clone())?;
    (outCache, range) = Ceval::cevalRangeIfConstant(outCache.clone(), inEnv.clone(), inRange.clone(), inRangeProps.clone(), inImpl.clone(), inInfo.clone())?;
    (outCache, range) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), range.clone(), inPrefix.clone())?;
    env = addForLoopScope(inEnv.clone(), (inIterator.clone()).clone(), ty.clone(), openmodelica_frontend_types::SCode::Variability::VAR, Some(c.clone()))?;
    (outCache, outStatements) = instStatements(outCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), inBody.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone())?;
    source = ElementSource::addElementSourceFileInfo(inSource.clone(), inInfo.clone());
    outStatements = list![Algorithm::makeFor((inIterator.clone()).clone(), range.clone(), inRangeProps.clone(), outStatements.clone(), source.clone())?];
    Ok((outCache, outStatements))
}

fn instComplexEquation(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>, mut source: Arc<DAE::ElementSource>, mut initial_: SCode::Initial) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist;
    dae = 'mc: {
        let __mc_input = (lhs.clone(), rhs.clone(), tp.clone(), source.clone(), initial_.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut dae: DAE::DAElist;
                    let true = (Types::isRecord(tp.clone())) else { bail!("pattern mismatch") };
                    dae = makeComplexDaeEquation(lhs.clone(), rhs.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut dae: DAE::DAElist;
                    let true = (Types::isExternalObject(tp.clone())) else { bail!("pattern mismatch") };
                    dae = makeDaeEquation(lhs.clone(), rhs.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut dae: DAE::DAElist;
                    dae = makeComplexDaeEquation(lhs.clone(), rhs.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo;
                    let false = (Types::isRecord(tp.clone())) else { bail!("pattern mismatch") };
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); ArcStr::from(__mm_s) }).clone();
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Error::addSourceMessage(Error::ILLEGAL_EQUATION_TYPE.clone(), list![(s.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(dae)
}

fn makeComplexDaeEquation(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut initial_: SCode::Initial) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist;
    dae = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone(), source.clone(), initial_.clone())) {
        (_, _, _, SCode::Initial::NON_INITIAL) => DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMPLEX_EQUATION { lhs: lhs.clone(), rhs: rhs.clone(), source: source.clone() })] },
        (_, _, _, SCode::Initial::INITIAL) => DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: lhs.clone(), rhs: rhs.clone(), source: source.clone() })] },
        _ => bail!("match: no arm matched"),
    } });
    Ok(dae)
}

pub fn instAlgorithm(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inAlgorithm: Arc<SCode::AlgorithmSection>, mut inImpl: bool, mut unrollForLoops: bool, mut inGraph: ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), inAlgorithm.clone(), inImpl.clone(), unrollForLoops.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::AlgorithmSection { statements }, r#impl, _, graph) => {
                    let mut statements_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut source: Arc<DAE::ElementSource>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ci_state = (*ci_state).clone();
                    ci_state = ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_ALGORITHM)?;
                    source = ElementSource::createElementSource(Absyn::dummyInfo.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    (cache, statements_1) = instStatements(cache.clone(), env.clone(), ih.clone(), pre.clone(), ci_state.clone(), statements.clone(), source.clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, r#impl.clone(), unrollForLoops.clone())?;
                    (statements_1, _) = DAEUtil::traverseDAEEquationsStmts(statements_1.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (ExpressionSimplify::simplifyWork, ExpressionSimplifyTypes::optionSimplifyOnly.clone()));
                    dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: statements_1.clone() }), source: source.clone() })] };
                    Ok((cache.clone(), env.clone(), ih.clone(), dae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, ci_state, Deref @ SCode::AlgorithmSection { statements: Deref @ metamodelica::List::Cons { head: stmt, tail: _ } }, _, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo;
                    if '__try0: {
                        let _ = unwrap_break_err!(ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_ALGORITHM), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s = (ClassInfUtil::printStateStr(ci_state.clone())).clone();
                    info = SCodeUtil::getStatementInfo(stmt.clone())?;
                    Error::addSourceMessage(Error::ALGORITHM_TRANSITION_FAILURE.clone(), list![(s.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- InstSection.instAlgorithm failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

pub fn instInitialAlgorithm(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inAlgorithm: Arc<SCode::AlgorithmSection>, mut inImpl: bool, mut unrollForLoops: bool, mut inGraph: ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), inAlgorithm.clone(), inImpl.clone(), unrollForLoops.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::AlgorithmSection { statements }, r#impl, _, graph) => {
                    let mut statements_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut source: Arc<DAE::ElementSource>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    source = ElementSource::createElementSource(Absyn::dummyInfo.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    (cache, statements_1) = instStatements(cache.clone(), env.clone(), ih.clone(), pre.clone(), ci_state.clone(), statements.clone(), source.clone(), openmodelica_frontend_types::SCode::Initial::INITIAL, r#impl.clone(), unrollForLoops.clone())?;
                    (statements_1, _) = DAEUtil::traverseDAEEquationsStmts(statements_1.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (ExpressionSimplify::simplifyWork, ExpressionSimplifyTypes::optionSimplifyOnly.clone()));
                    dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::INITIALALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: statements_1.clone() }), source: source.clone() })] };
                    Ok((cache.clone(), env.clone(), ih.clone(), dae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstSection.instInitialAlgorithm failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

pub fn instConstraint(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inConstraints: SCode::ConstraintSection, mut inImpl: bool) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist, ClassInf::State)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outDae: DAE::DAElist;
    let mut outState: ClassInf::State;
    (outCache, outEnv, outDae, outState) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPrefix.clone(), inState.clone(), inConstraints.clone(), inImpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, mut pre, mut ci_state, SCode::ConstraintSection { constraints: mut constraints }, mut r#impl) = __mc_input.clone() else { bail!("nomatch") };
            let mut constraints_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut source: Arc<DAE::ElementSource>;
            let mut dae: DAE::DAElist;
            ci_state = ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_ALGORITHM)?;
            source = ElementSource::createElementSource(Absyn::dummyInfo.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
            (cache, constraints_1, _) = Static::elabExpList(cache.clone(), env.clone(), constraints.clone(), r#impl.clone(), true, pre.clone(), Absyn::dummyInfo.clone(), DAE::T_UNKNOWN_DEFAULT.clone())?;
            dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::CONSTRAINT { constraints: Arc::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: constraints_1.clone() }), source: source.clone() })] };
            Ok((cache.clone(), env.clone(), dae.clone(), ci_state.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- InstSection.instConstraints failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outDae, outState))
}

pub fn instStatements(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inStatements: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut inSource: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial, mut inImpl: bool, mut unrollForLoops: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut stmtsl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
    for mut stmt in &*inStatements.clone() {
        let mut stmt = stmt.clone();
        (outCache, stmts) = instStatement(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), stmt.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), unrollForLoops.clone())?;
        stmtsl = cons(stmts.clone(), stmtsl.clone());
    }
    outStatements = List::flattenReverse(stmtsl.clone());
    Ok((outCache, outStatements))
}

fn instExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inExp: Arc<Absyn::Exp>, mut inImpl: bool, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = Static::elabExp(inCache.clone(), inEnv.clone(), inExp.clone(), inImpl.clone(), true, inPrefix.clone(), inInfo.clone())?;
    (outCache, outExp, outProperties) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), outExp.clone(), outProperties.clone(), inImpl.clone(), inInfo.clone())?;
    (outCache, outExp) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), outExp.clone(), inPrefix.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn instStatement(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inStatement: Arc<SCode::Statement>, mut inSource: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inUnrollLoops: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut num_errors: i32 = Error::getNumErrorMessages();
    match '__try0: {
        outStatements = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { .. } => {
            (outCache, outStatements) = unwrap_break_err!(instAssignment(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inStatement.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone(), num_errors.clone()), '__try0);
            outStatements.clone()
        },
        Deref @ SCode::Statement::ALG_IF { info, .. } => {
            let mut cond_exp: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut cond_prop: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut if_branch: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut else_branch: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut branch: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut else_if_branches: Arc<metamodelica::List<(Arc<DAE::Exp>, DAE::Properties, Arc<metamodelica::List<Arc<DAE::Statement>>>)>> = metamodelica::nil();
            let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut sstmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
            let mut source: Arc<DAE::ElementSource>;
            (outCache, cond_exp, cond_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inStatement).boolExpr, SCode::Statement::ALG_IF).clone(), inImpl.clone(), info.clone()), '__try0);
            (outCache, if_branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), var_field!((*inStatement).trueBranch, SCode::Statement::ALG_IF).clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
            else_if_branches = metamodelica::nil();
            for mut else_if in &*var_field!((*inStatement).elseIfBranch, SCode::Statement::ALG_IF).clone() {
                let mut else_if = else_if.clone();
                (aexp, sstmts) = else_if.clone();
                (outCache, exp, prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), aexp.clone(), inImpl.clone(), info.clone()), '__try0);
                (outCache, branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), sstmts.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
                else_if_branches = cons((exp.clone(), prop.clone(), branch.clone()), else_if_branches.clone());
            }
            else_if_branches = else_if_branches.clone().reverse();
            (outCache, else_branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), var_field!((*inStatement).elseBranch, SCode::Statement::ALG_IF).clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            unwrap_break_err!(Algorithm::makeIf(cond_exp.clone(), cond_prop.clone(), if_branch.clone(), else_if_branches.clone(), else_branch.clone(), source.clone()), '__try0)
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            (outCache, outStatements) = unwrap_break_err!(instForStatement(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), inStatement.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
            outStatements.clone()
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            (outCache, outStatements) = unwrap_break_err!(instParForStatement(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), inStatement.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
            outStatements.clone()
        },
        Deref @ SCode::Statement::ALG_WHILE { info, .. } => {
            let mut cond_exp: Arc<DAE::Exp>;
            let mut cond_prop: DAE::Properties;
            let mut branch: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut source: Arc<DAE::ElementSource>;
            (outCache, cond_exp, cond_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inStatement).boolExpr, SCode::Statement::ALG_WHILE).clone(), inImpl.clone(), info.clone()), '__try0);
            (outCache, branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), var_field!((*inStatement).whileBody, SCode::Statement::ALG_WHILE).clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            list![unwrap_break_err!(Algorithm::makeWhile(cond_exp.clone(), cond_prop.clone(), branch.clone(), source.clone()), '__try0)]
        },
        Deref @ SCode::Statement::ALG_WHEN_A { info, .. } => {
            let mut cond_exp: Arc<DAE::Exp>;
            let mut cond_prop: DAE::Properties;
            let mut branch: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut sstmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
            let mut source: Arc<DAE::ElementSource>;
            let mut when_stmt_opt: Option<Arc<DAE::Statement>> = None;
            let mut when_stmt: Arc<DAE::Statement>;
            if ClassInfUtil::isFunction(inState.clone()) {
                unwrap_break_err!(Error::addSourceMessageAndFail(Error::FUNCTION_ELEMENT_WRONG_KIND.clone(), list![(literal!("when")).clone()], info.clone()), '__try0);
            }
            unwrap_break_err!(checkWhenAlgorithm(inStatement.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            when_stmt_opt = None;
            for mut b in &*var_field!((*inStatement).branches, SCode::Statement::ALG_WHEN_A).clone().reverse() {
                let mut b = b.clone();
                (aexp, sstmts) = b.clone();
                (outCache, cond_exp, cond_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), aexp.clone(), inImpl.clone(), info.clone()), '__try0);
                (outCache, branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), sstmts.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
                when_stmt = unwrap_break_err!(Algorithm::makeWhenA(cond_exp.clone(), cond_prop.clone(), branch.clone(), when_stmt_opt.clone(), source.clone()), '__try0);
                when_stmt_opt = Some(when_stmt.clone());
            }
            list![when_stmt.clone()]
        },
        Deref @ SCode::Statement::ALG_ASSERT { info, .. } => {
            let mut cond_exp: Arc<DAE::Exp>;
            let mut msg_exp: Arc<DAE::Exp>;
            let mut level_exp: Arc<DAE::Exp>;
            let mut cond_prop: DAE::Properties;
            let mut msg_prop: DAE::Properties;
            let mut level_prop: DAE::Properties;
            let mut source: Arc<DAE::ElementSource>;
            (outCache, cond_exp, cond_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inStatement).condition, SCode::Statement::ALG_ASSERT).clone(), inImpl.clone(), info.clone()), '__try0);
            (outCache, msg_exp, msg_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inStatement).message, SCode::Statement::ALG_ASSERT).clone(), inImpl.clone(), info.clone()), '__try0);
            (outCache, level_exp, level_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inStatement).level, SCode::Statement::ALG_ASSERT).clone(), inImpl.clone(), info.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            unwrap_break_err!(Algorithm::makeAssert(cond_exp.clone(), msg_exp.clone(), level_exp.clone(), cond_prop.clone(), msg_prop.clone(), level_prop.clone(), source.clone()), '__try0)
        },
        Deref @ SCode::Statement::ALG_TERMINATE { info, .. } => {
            let mut msg_exp: Arc<DAE::Exp>;
            let mut msg_prop: DAE::Properties;
            let mut source: Arc<DAE::ElementSource>;
            (outCache, msg_exp, msg_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inStatement).message, SCode::Statement::ALG_TERMINATE).clone(), inImpl.clone(), info.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            unwrap_break_err!(Algorithm::makeTerminate(msg_exp.clone(), msg_prop.clone(), source.clone()), '__try0)
        },
        Deref @ SCode::Statement::ALG_REINIT { info, .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut cr_exp: Arc<DAE::Exp>;
            let mut prop: DAE::Properties;
            let mut cr_prop: DAE::Properties;
            let mut source: Arc<DAE::ElementSource>;
            (outCache, cr_exp, cr_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inStatement).cref, SCode::Statement::ALG_REINIT).clone(), inImpl.clone(), info.clone()), '__try0);
            (outCache, exp, prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), var_field!((*inStatement).newValue, SCode::Statement::ALG_REINIT).clone(), inImpl.clone(), info.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            unwrap_break_err!(Algorithm::makeReinit(cr_exp.clone(), exp.clone(), cr_prop.clone(), prop.clone(), source.clone()), '__try0)
        },
        Deref @ SCode::Statement::ALG_NORETCALL { info, .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut source: Arc<DAE::ElementSource>;
            (outCache, exp, _) = unwrap_break_err!(Static::elabExp(outCache.clone(), inEnv.clone(), var_field!((*inStatement).exp, SCode::Statement::ALG_NORETCALL).clone(), inImpl.clone(), true, inPrefix.clone(), info.clone()), '__try0);
            unwrap_break_err!(checkValidNoRetcall(exp.clone(), info.clone()), '__try0);
            (outCache, exp) = unwrap_break_err!(PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), exp.clone(), inPrefix.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            if (Expression::isTuple(exp.clone())) {metamodelica::nil()} else {list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: source.clone() })]}
        },
        Deref @ SCode::Statement::ALG_BREAK { info, .. } => {
            let mut source: Arc<DAE::ElementSource>;
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            list![Arc::new(DAE::Statement::STMT_BREAK { source: source.clone() })]
        },
        Deref @ SCode::Statement::ALG_CONTINUE { info, .. } => {
            let mut source: Arc<DAE::ElementSource>;
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            list![Arc::new(DAE::Statement::STMT_CONTINUE { source: source.clone() })]
        },
        Deref @ SCode::Statement::ALG_RETURN { info, .. } => {
            let mut source: Arc<DAE::ElementSource>;
            if !(ClassInfUtil::isFunction(inState.clone())) {
                unwrap_break_err!(Error::addSourceMessageAndFail(Error::RETURN_OUTSIDE_FUNCTION.clone(), metamodelica::nil(), info.clone()), '__try0);
            }
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            list![Arc::new(DAE::Statement::STMT_RETURN { source: source.clone() })]
        },
        Deref @ SCode::Statement::ALG_FAILURE { info, .. } => {
            let mut branch: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut source: Arc<DAE::ElementSource>;
            let true = (unwrap_break_err!(Config::acceptMetaModelicaGrammar(), '__try0)) else { bail!("pattern mismatch") };
            (outCache, branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), var_field!((*inStatement).stmts, SCode::Statement::ALG_FAILURE).clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            list![Arc::new(DAE::Statement::STMT_FAILURE { body: branch.clone(), source: source.clone() })]
        },
        Deref @ SCode::Statement::ALG_TRY { info, .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut if_branch: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut else_branch: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut source: Arc<DAE::ElementSource>;
            let mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
            let true = (unwrap_break_err!(Config::acceptMetaModelicaGrammar(), '__try0)) else { bail!("pattern mismatch") };
            (outCache, if_branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), var_field!((*inStatement).body, SCode::Statement::ALG_TRY).clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
            (outCache, else_branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), var_field!((*inStatement).elseBody, SCode::Statement::ALG_TRY).clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone()), '__try0);
            source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
            cases = list![Arc::new(DAE::MatchCase { patterns: metamodelica::nil(), patternGuard: None, localDecls: metamodelica::nil(), body: if_branch.clone(), result: Some(Arc::new(DAE::Exp::TUPLE { PR: metamodelica::nil() })), resultInfo: info.clone(), jump: 0, info: info.clone() }), Arc::new(DAE::MatchCase { patterns: metamodelica::nil(), patternGuard: None, localDecls: metamodelica::nil(), body: else_branch.clone(), result: Some(Arc::new(DAE::Exp::TUPLE { PR: metamodelica::nil() })), resultInfo: info.clone(), jump: 0, info: info.clone() })];
            exp = Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: if (unwrap_break_err!(SCodeUtil::commentHasBooleanNamedAnnotation(var_field!((*inStatement).comment, SCode::Statement::ALG_TRY).clone(), (literal!("__OpenModelica_stackOverflowCheckpoint")).clone()), '__try0)) {openmodelica_frontend_types::DAE::MatchType::TRY_STACKOVERFLOW} else {openmodelica_frontend_types::DAE::MatchType::MATCHCONTINUE}, inputs: metamodelica::nil(), aliases: metamodelica::nil(), localDecls: metamodelica::nil(), cases: cases.clone(), et: DAE::T_NORETCALL_DEFAULT.clone() });
            list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: source.clone() })]
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok::<_, anyhow::Error>((outStatements.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outStatements = __try0_o0;
        }
        Err(_) => {
            let true = (num_errors.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
            Error::addSourceMessageAndFail(Error::STATEMENT_GENERIC_FAILURE.clone(), list![(SCodeDump::statementStr(inStatement.clone(), SCodeDump::defaultOptions.clone())?).clone()], SCodeUtil::getStatementInfo(inStatement.clone())?)?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((outCache, outStatements))
}

fn makeAssignment(mut inLhs: Arc<DAE::Exp>, mut inLhsProps: DAE::Properties, mut inRhs: Arc<DAE::Exp>, mut inRhsProps: DAE::Properties, mut inAttributes: Arc<DAE::Attributes>, mut inInitial: SCode::Initial, mut inSource: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Statement>> {
    let mut outStatement: Arc<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &((inLhs.clone(), inLhsProps.clone(), inRhs.clone(), inRhsProps.clone(), inAttributes.clone(), inInitial.clone(), inSource.clone())) {
        (_, DAE::Properties::PROP { .. }, Deref @ DAE::Exp::CALL { .. }, DAE::Properties::PROP_TUPLE { .. }, _, _, _) => {
            let mut wild_props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
            let mut wild_count: i32 = 0;
            let mut wilds: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut wildCrefExp: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(Types::propTuplePropList(inRhsProps.clone())?) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            wild_props = __pa0.clone();
            wild_count = (wild_props.clone().len() as i32);
            wildCrefExp = Expression::makeCrefExp(Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD), DAE::T_UNKNOWN_DEFAULT.clone())?;
            wilds = List::fill(wildCrefExp.clone(), wild_count.clone());
            wild_props = List::fill(DAE::Properties::PROP { type_: DAE::T_ANYTYPE_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, wild_count.clone());
            Algorithm::makeTupleAssignment(cons(inLhs.clone(), wilds.clone()), cons(inLhsProps.clone(), wild_props.clone()), inRhs.clone(), inRhsProps.clone(), inInitial.clone(), inSource.clone())?
        },
        _ => {
            Algorithm::makeAssignment(inLhs.clone(), inLhsProps.clone(), inRhs.clone(), inRhsProps.clone(), inAttributes.clone(), inInitial.clone(), inSource.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStatement)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn containsWhenStatements(mut statementList: Arc<metamodelica::List<Arc<SCode::Statement>>>) -> Result<bool> {
    let mut hasWhenStatements: bool = false;
    hasWhenStatements = 'mc: {
        let __mc_input = statementList.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Statement::ALG_WHEN_A { .. }, tail: _ } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Statement::ALG_IF { elseBranch: eb, elseIfBranch: eib, trueBranch: tb, .. }, tail: rest } => {
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut b3: bool = false;
                    let mut b4: bool = false;
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut slst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>> = metamodelica::nil();
                    b1 = containsWhenStatements(tb.clone())?;
                    b2 = containsWhenStatements(eb.clone())?;
                    slst = List::map(eib.clone(), Arc::new(fnptr!(Util::tuple22, _)));
                    blst = List::map(slst.clone(), Arc::new(containsWhenStatements));
                    b3 = List::reduce(cons(false, blst.clone()), Arc::new(fnptr!(boolOr, bool, bool)))?;
                    b4 = containsWhenStatements(rest.clone())?;
                    b = List::reduce(list![b1.clone(), b2.clone(), b3.clone(), b4.clone()], Arc::new(fnptr!(boolOr, bool, bool)))?;
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Statement::ALG_FOR { forBody: lst, .. }, tail: rest } => {
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    b1 = containsWhenStatements(lst.clone())?;
                    b2 = containsWhenStatements(rest.clone())?;
                    b = boolOr(b1.clone(), b2.clone());
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Statement::ALG_PARFOR { parforBody: lst, .. }, tail: rest } => {
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    b1 = containsWhenStatements(lst.clone())?;
                    b2 = containsWhenStatements(rest.clone())?;
                    b = boolOr(b1.clone(), b2.clone());
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Statement::ALG_WHILE { whileBody: lst, .. }, tail: rest } => {
                    let mut b: bool = false;
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    b1 = containsWhenStatements(lst.clone())?;
                    b2 = containsWhenStatements(rest.clone())?;
                    b = boolOr(b1.clone(), b2.clone());
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(containsWhenStatements(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(hasWhenStatements)
}

fn loopOverRange(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut ci_state: ClassInf::State, mut inIdent: Ident, mut inValue: Arc<Values::Value>, mut inAlgItmLst: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut source: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial, mut inImpl: bool, mut unrollForLoops: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    (outCache, outStatements) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), ci_state.clone(), inIdent.clone(), inValue.clone(), inAlgItmLst.clone(), source.clone(), inInitial.clone(), inImpl.clone(), unrollForLoops.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, _, _, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, .. }, _, _, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, _, i, Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::List::Cons { head: dim, tail: dims }, valueLst: Deref @ metamodelica::List::Cons { head: fst, tail: rest } }, algs, _, initial_, r#impl, _) => {
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut dim = (*dim).clone();
                    let mut dims = (*dims).clone();
                    dim = dim.clone() - 1;
                    dims = cons(dim.clone(), dims.clone());
                    env_1 = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (arcstr::literal!(FCore::forScopeName)).clone(), None)?;
                    env_2 = FGraph::addForIterator(env_1.clone(), (i.clone()).clone(), DAE::T_INTEGER_DEFAULT.clone(), Arc::new(DAE::Binding::VALBOUND { valBound: fst.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::CONST, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
                    (cache, stmts1) = instStatements(cache.clone(), env_2.clone(), ih.clone(), pre.clone(), ci_state.clone(), algs.clone(), source.clone(), initial_.clone(), r#impl.clone(), unrollForLoops.clone())?;
                    (cache, stmts2) = loopOverRange(cache.clone(), env.clone(), ih.clone(), pre.clone(), ci_state.clone(), (i.clone()).clone(), Arc::new(Values::Value::ARRAY { valueLst: rest.clone(), dimLst: dims.clone() }), algs.clone(), source.clone(), initial_.clone(), r#impl.clone(), unrollForLoops.clone())?;
                    stmts = listAppend(stmts1.clone(), stmts2.clone());
                    Ok((cache.clone(), stmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, v, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstSection.loopOverRange failed to loop over range: ")); __mm_s.push_str(&*ValuesDump::valString(v.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outStatements))
}

fn rangeExpression(mut inTuple: (Arc<Absyn::ComponentRef>, i32)) -> Arc<Absyn::Exp> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    outExp = (::match_deref::match_deref! { match &(inTuple.clone()) {
        (acref, dimNum) => {
            let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            e = Arc::new(Absyn::Exp::RANGE { start: Arc::new(Absyn::Exp::INTEGER { value: 1 }), step: None, stop: Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("size")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![Arc::new(Absyn::Exp::CREF { componentRef: acref.clone() }), Arc::new(Absyn::Exp::INTEGER { value: dimNum.clone() })], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }) });
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn instIfEqBranch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inImpl: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outState: ClassInf::State;
    let mut outEquations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    checkForConnectInIfBranch(inEquations.clone());
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Inst::instList(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), Connect::emptySet.clone(), inState.clone(), Arc::new(instEquation), inEquations.clone(), inImpl.clone(), alwaysUnroll.clone(), ConnectionGraph::EMPTY.clone())?) {
        (__pa0, __pa1, __pa2, DAE::DAElist { elementLst: __pa3 }, _, __pa4, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    outEnv = __pa1.clone();
    outIH = __pa2.clone();
    outEquations = __pa3.clone();
    outState = __pa4.clone();
    Ok((outCache, outEnv, outIH, outState, outEquations))
}

fn instIfEqBranches(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inBranches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>, mut inImpl: bool, mut inAccumEqs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, ClassInf::State, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outState: ClassInf::State;
    let mut outEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
    (outCache, outEnv, outIH, outState, outEquations) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), inBranches.clone(), inImpl.clone(), inAccumEqs.clone())) {
        (cache, env, ih, _, state, Deref @ metamodelica::List::Cons { head: seq, tail: rest_seq }, _, _) => {
            let mut deq: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut ih = (*ih).clone();
            let mut state = (*state).clone();
            (cache, env, ih, state, deq) = instIfEqBranch(cache.clone(), env.clone(), ih.clone(), inPrefix.clone(), state.clone(), seq.clone(), inImpl.clone())?;
            (cache, env, ih, state, branches) = instIfEqBranches(cache.clone(), env.clone(), ih.clone(), inPrefix.clone(), state.clone(), rest_seq.clone(), inImpl.clone(), cons(deq.clone(), inAccumEqs.clone()))?;
            (cache.clone(), env.clone(), ih.clone(), state.clone(), branches.clone())
        },
        (_, _, _, _, _, Deref @ metamodelica::List::Nil, _, _) => {
            (inCache.clone(), inEnv.clone(), inIH.clone(), inState.clone(), inAccumEqs.clone().reverse())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outState, outEquations))
}

fn instInitialIfEqBranch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inImpl: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outState: ClassInf::State;
    let mut outEquations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    checkForConnectInIfBranch(inEquations.clone());
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Inst::instList(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), Connect::emptySet.clone(), inState.clone(), Arc::new(instInitialEquation), inEquations.clone(), inImpl.clone(), alwaysUnroll.clone(), ConnectionGraph::EMPTY.clone())?) {
        (__pa0, __pa1, __pa2, DAE::DAElist { elementLst: __pa3 }, _, __pa4, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    outEnv = __pa1.clone();
    outIH = __pa2.clone();
    outEquations = __pa3.clone();
    outState = __pa4.clone();
    Ok((outCache, outEnv, outIH, outState, outEquations))
}

fn instInitialIfEqBranches(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inBranches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>, mut inImpl: bool, mut inAccumEqs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, ClassInf::State, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outState: ClassInf::State;
    let mut outEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
    (outCache, outEnv, outIH, outState, outEquations) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), inBranches.clone(), inImpl.clone(), inAccumEqs.clone())) {
        (cache, env, ih, _, state, Deref @ metamodelica::List::Cons { head: seq, tail: rest_seq }, _, _) => {
            let mut deq: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut ih = (*ih).clone();
            let mut state = (*state).clone();
            (cache, env, ih, state, deq) = instInitialIfEqBranch(cache.clone(), env.clone(), ih.clone(), inPrefix.clone(), state.clone(), seq.clone(), inImpl.clone())?;
            (cache, env, ih, state, branches) = instInitialIfEqBranches(cache.clone(), env.clone(), ih.clone(), inPrefix.clone(), state.clone(), rest_seq.clone(), inImpl.clone(), cons(deq.clone(), inAccumEqs.clone()))?;
            (cache.clone(), env.clone(), ih.clone(), state.clone(), branches.clone())
        },
        (_, _, _, _, _, Deref @ metamodelica::List::Nil, _, _) => {
            (inCache.clone(), inEnv.clone(), inIH.clone(), inState.clone(), inAccumEqs.clone().reverse())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outState, outEquations))
}

fn checkForConnectInIfBranch(mut inEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> () {
    List::map_0(inEquations.clone(), Arc::new(checkForConnectInIfBranch2));
    ()
}

fn checkForConnectInIfBranch2(mut inEquation: Arc<SCode::Equation>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ SCode::Equation::EQ_CONNECT { info, crefRight: cr2, crefLeft: cr1, .. } => {
            let mut cr1_str: ArcStr = arcstr::literal!("");
            let mut cr2_str: ArcStr = arcstr::literal!("");
            cr1_str = (Dump::printComponentRefStr(cr1.clone())?).clone();
            cr2_str = (Dump::printComponentRefStr(cr2.clone())?).clone();
            Error::addSourceMessage(Error::CONNECT_IN_IF.clone(), list![(cr1_str.clone()).clone(), (cr2_str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        Deref @ SCode::Equation::EQ_FOR { eEquationLst: eqs, .. } => {
            checkForConnectInIfBranch(eqs.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn instElseIfs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPre: DAE::Prefix, mut ci_state: ClassInf::State, mut inElseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>, mut source: Arc<DAE::ElementSource>, mut initial_: SCode::Initial, mut inImpl: bool, mut unrollForLoops: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<(Arc<DAE::Exp>, DAE::Properties, Arc<metamodelica::List<Arc<DAE::Statement>>>)>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outElseIfBranches: Arc<metamodelica::List<(Arc<DAE::Exp>, DAE::Properties, Arc<metamodelica::List<Arc<DAE::Statement>>>)>> = metamodelica::nil();
    (outCache, outElseIfBranches) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), ci_state.clone(), inElseIfBranches.clone(), source.clone(), initial_.clone(), inImpl.clone(), unrollForLoops.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, _, Deref @ metamodelica::List::Cons { head: (e, l), tail: tail }, _, _, r#impl, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut tail_1: Arc<metamodelica::List<(Arc<DAE::Exp>, DAE::Properties, Arc<metamodelica::List<Arc<DAE::Statement>>>)>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, e_1, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), env.clone(), ih.clone(), e_1.clone(), pre.clone())?;
                    (cache, stmts) = instStatements(cache.clone(), env.clone(), ih.clone(), pre.clone(), ci_state.clone(), l.clone(), source.clone(), initial_.clone(), r#impl.clone(), unrollForLoops.clone())?;
                    (cache, tail_1) = instElseIfs(cache.clone(), env.clone(), ih.clone(), pre.clone(), ci_state.clone(), tail.clone(), source.clone(), initial_.clone(), r#impl.clone(), unrollForLoops.clone(), info.clone())?;
                    Ok((cache.clone(), cons((e_2.clone(), prop.clone(), stmts.clone()), tail_1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstSection.instElseIfs failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outElseIfBranches))
}

fn instWhenEqBranch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), mut inImpl: bool, mut inUnrollLoops: bool, mut inGraph: ConnectionGraph::ConnectionGraph, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outCondition: Arc<DAE::Exp>;
    let mut outEquations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut cond: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut prop: DAE::Properties;
    let mut aexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut dexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut dexp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut isClock: bool = false;
    (cond, body) = inBranch.clone();
    isClock = false;
    outCondition = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Absyn::Exp::ARRAY { arrayExp: aexps } => {
            dexps = metamodelica::nil();
            for mut aexp in &*aexps.clone() {
                let mut aexp = aexp.clone();
                (outCache, dexp, prop) = instExp(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), aexp.clone(), inImpl.clone(), info.clone())?;
                ty = Types::getPropType(prop.clone())?;
                dexp = checkWhenCondition(dexp.clone(), ty.clone(), aexp.clone(), info.clone())?;
                dexps = cons(dexp.clone(), dexps.clone());
            }
            Expression::makeArray(dexps.clone().reverse(), DAE::T_BOOL_DEFAULT.clone(), true)
        },
        _ => {
            (outCache, dexp, prop) = instExp(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), cond.clone(), inImpl.clone(), info.clone())?;
            ty = Types::getPropType(prop.clone())?;
            if Types::isClockOrSubTypeClock(ty.clone())? {
                isClock = true;
            } else {
                dexp = checkWhenCondition(dexp.clone(), ty.clone(), cond.clone(), info.clone())?;
            }
            dexp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(isClock.clone()) {
        List::map_0(body.clone(), Arc::new(checkForNestedWhenInEq));
    }
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Inst::instList(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), Arc::new(instEquation), body.clone(), inImpl.clone(), alwaysUnroll.clone(), inGraph.clone())?) {
        (__pa0, __pa1, __pa2, DAE::DAElist { elementLst: __pa3 }, _, _, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    outEnv = __pa1.clone();
    outIH = __pa2.clone();
    outEquations = __pa3.clone();
    outGraph = __pa4.clone();
    Ok((outCache, outEnv, outIH, outCondition, outEquations, outGraph))
}

fn checkWhenCondition(mut exp: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>, mut aexp: Arc<Absyn::Exp>, mut info: SourceInfo) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut tyEl: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    match '__try0: {
        if Types::isArray(ty.clone()) {
            tyEl = Types::arrayElementType(ty.clone());
        } else {
            tyEl = ty.clone();
        }
        (exp, _) = unwrap_break_err!(Types::matchType(exp.clone(), tyEl.clone(), DAE::T_BOOL_DEFAULT.clone(), false), '__try0);
        Ok::<_, anyhow::Error>((exp.clone(), tyEl.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            exp = __try0_o0;
            tyEl = __try0_o1;
        }
        Err(_) => {
            Error::addSourceMessage(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(Dump::printExpStr(aexp.clone())?).clone(), (TypesDump::unparseType(ty.clone())?).clone()], info.clone())?;
            bail!("fail");
        }
    }
    if Config::languageStandardAtLeast(Config::LanguageStandard::_3_2.clone())? {
        let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => (),
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" } }, .. } => (),
        _ => {
            if Expression::expHasInitial(exp.clone())? {
                Error::addSourceMessage(Error::INITIAL_CALL_WARNING.clone(), list![(Dump::printExpStr(aexp.clone())?).clone()], info.clone())?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(exp)
}

fn instConnect(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inSets: DAE::Connect::Sets, mut inPrefix: DAE::Prefix, mut inComponentRefLeft: Arc<Absyn::ComponentRef>, mut inComponentRefRight: Arc<Absyn::ComponentRef>, mut inImplicit: bool, mut inGraph: ConnectionGraph::ConnectionGraph, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Connect::Sets, DAE::DAElist, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), inPrefix.clone(), inComponentRefLeft.clone(), inComponentRefRight.clone(), inImplicit.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, _, c1, c2, _, graph) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let true = (AbsynUtil::crefEqual(c1.clone(), c2.clone())) else { bail!("pattern mismatch") };
                    s1 = (Dump::printComponentRefStr(c1.clone())?).clone();
                    s2 = (Dump::printComponentRefStr(c1.clone())?).clone();
                    Error::addSourceMessage(Error::SAME_CONNECT_INSTANCE.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::emptyDae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, c2, r#impl, graph) => {
                    let mut c1_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut attr1: Arc<DAE::Attributes>;
                    let mut attr2: Arc<DAE::Attributes>;
                    let mut ct1: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut f1: DAE::Connect::Face = DAE::Connect::Face::INSIDE;
                    let mut f2: DAE::Connect::Face = DAE::Connect::Face::INSIDE;
                    let mut dae: DAE::DAElist;
                    let mut io1: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut io2: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut vt1: SCode::Variability = SCode::Variability::CONST;
                    let mut vt2: SCode::Variability = SCode::Variability::CONST;
                    let mut del1: bool = false;
                    let mut del2: bool = false;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    (cache, c1_2, attr1, ct1, vt1, io1, f1, ty1, del1) = instConnector(cache.clone(), env.clone(), ih.clone(), c1.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, c2_2, attr2, _, vt2, io2, f2, ty2, del2) = instConnector(cache.clone(), env.clone(), ih.clone(), c2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    if del1.clone() || del2.clone() {
                        dae = DAE::emptyDae.clone();
                    } else if Types::isExpandableConnector(ty1.clone()) || Types::isExpandableConnector(ty2.clone()) {
                        bail!("fail");
                    } else {
                        checkConnectTypes(c1_2.clone(), ty1.clone(), f1.clone(), attr1.clone(), c2_2.clone(), ty2.clone(), f2.clone(), attr2.clone(), info.clone())?;
                        (cache, _, ih, sets, dae, graph) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1_2.clone(), f1.clone(), ty1.clone(), vt1.clone(), c2_2.clone(), f2.clone(), ty2.clone(), vt2.clone(), ct1.clone(), io1.clone(), io2.clone(), graph.clone(), info.clone())?;
                        sets = ConnectUtil::increaseConnectRefCount(c1_2.clone(), c2_2.clone(), sets.clone())?;
                    }
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, c2, r#impl, graph) => {
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    ErrorExt::setCheckpoint((literal!("expandableConnectors")).clone());
                    let true = (System::getHasExpandableConnectors()) else { bail!("pattern mismatch") };
                    (cache, env, ih, sets, dae, graph) = connectExpandableConnectors(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), r#impl.clone(), graph.clone(), info.clone())?;
                    ErrorExt::rollBack((literal!("expandableConnectors")).clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, pre, c1, c2, _, _) => {
                    let mut subs1: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut subs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut crefs1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    ErrorExt::rollBack((literal!("expandableConnectors")).clone());
                    subs1 = AbsynUtil::getSubsFromCref(c1.clone(), true, true)?;
                    crefs1 = AbsynUtil::getCrefsFromSubs(subs1.clone(), true, true)?;
                    subs2 = AbsynUtil::getSubsFromCref(c2.clone(), true, true)?;
                    crefs2 = AbsynUtil::getCrefsFromSubs(subs2.clone(), true, true)?;
                    s1 = (Dump::printComponentRefStr(c1.clone())?).clone();
                    s2 = (Dump::printComponentRefStr(c2.clone())?).clone();
                    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    checkConstantVariability(crefs1.clone(), cache.clone(), env.clone(), (s1.clone()).clone(), pre.clone(), info.clone())?;
                    checkConstantVariability(crefs2.clone(), cache.clone(), env.clone(), (s1.clone()).clone(), pre.clone(), info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, _, _, _, _, graph) => {
                    if !((Config::getGraphicsExpMode()?)) { bail!("guard") }
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::emptyDae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, c1, c2, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstSection.instConnect failed for: connect(")); __mm_s.push_str(&*Dump::printComponentRefStr(c1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Dump::printComponentRefStr(c2.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn instConnector(mut inCache: FCore::Cache, mut env: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut connectorCref: Arc<Absyn::ComponentRef>, mut r#impl: bool, mut prefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>, Arc<DAE::Attributes>, Arc<DAE::ConnectorType>, SCode::Variability, Absyn::InnerOuter, DAE::Connect::Face, Arc<DAE::Type>, bool)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outAttr: Arc<DAE::Attributes>;
    let mut connectorType: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    let mut variability: SCode::Variability = SCode::Variability::CONST;
    let mut innerOuter: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut face: DAE::Connect::Face = DAE::Connect::Face::INSIDE;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut deleted: bool = false;
    let mut status: FCore::Status = FCore::Status::CLS_FULL;
    let mut is_expandable: bool = false;
    outCref = ComponentReference::toExpCref(connectorCref.clone())?;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupConnectorVar(env.clone(), outCref.clone(), true)?) {
        (Deref @ DAE::Attributes { innerOuter: __pa0, variability: __pa1, connectorType: __pa2, .. }, __pa3, __pa4, __pa5) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    innerOuter = __pa0.clone();
    variability = __pa1.clone();
    connectorType = __pa2.clone();
    ty = __pa3.clone();
    status = __pa4.clone();
    is_expandable = __pa5.clone();
    deleted = FCore::isDeletedComp(status.clone());
    if deleted.clone() || is_expandable.clone() {
        face = openmodelica_frontend_types::DAE::Connect::Face::NO_FACE;
        outAttr = DAE::dummyAttrVar.clone();
    } else {
        let (__pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(Static::elabCrefNoEval(inCache.clone(), env.clone(), connectorCref.clone(), r#impl.clone(), false, prefix.clone(), info.clone())?) {
            (__pa6, Deref @ DAE::Exp::CREF { componentRef: __pa7, .. }, DAE::Properties::PROP { type_: __pa8, .. }, __pa9) => (__pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
            _ => bail!("pattern mismatch"),
        } };
        outCache = __pa6.clone();
        outCref = __pa7.clone();
        ty = __pa8.clone();
        outAttr = __pa9.clone();
        (outCache, outCref) = Static::canonCref(outCache.clone(), env.clone(), outCref.clone(), r#impl.clone())?;
        validConnector(ty.clone(), outCref.clone(), info.clone())?;
        face = ConnectUtil::componentFace(env.clone(), outCref.clone())?;
        ty = sortConnectorType(ty.clone())?;
    }
    Ok((outCache, outCref, outAttr, connectorType, variability, innerOuter, face, ty, deleted))
}

fn sortConnectorType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty, dims } => {
            let mut ty = (*ty).clone();
            ty = sortConnectorType(ty.clone())?;
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() })
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ci_state, varLst: vars, equalityConstraint: ec, .. } => {
            let mut vars = (*vars).clone();
            vars = List::sort(vars.clone(), Arc::new(connectorCompGt))?;
            Arc::new(DAE::Type::T_COMPLEX { complexClassType: ci_state.clone(), varLst: vars.clone(), equalityConstraint: ec.clone(), usedExternally: var_field!((*inType).usedExternally, DAE::Type::T_COMPLEX).clone() })
        },
        _ => {
            inType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn connectorCompGt(mut inVar1: Arc<DAE::Var>, mut inVar2: Arc<DAE::Var>) -> Result<bool> {
    let mut outGt: bool = false;
    let mut id1: ArcStr = arcstr::literal!("");
    let mut id2: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inVar1.clone()) {
        Deref @ DAE::Var { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    id1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(inVar2.clone()) {
        Deref @ DAE::Var { name: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    id2 = __pa1.clone();
    outGt = 1 == stringCompare((id1.clone()).clone(), (id2.clone()).clone());
    Ok(outGt)
}

fn checkConstantVariability(mut inrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut cache: FCore::Cache, mut env: FCore::Graph, mut affectedConnector: ArcStr, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inrefs.clone(), cache.clone(), env.clone(), affectedConnector.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr, tail: refs }, _, _, _, pre, _) => {
                    let mut prop: DAE::Properties;
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (_, Some((_, __pa0, _))) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    prop = __pa0.clone();
                    r#const = Types::propertiesListToConst(list![prop.clone()])?;
                    let true = (Types::isParameterOrConstant(r#const.clone())) else { bail!("pattern mismatch") };
                    checkConstantVariability(refs.clone(), cache.clone(), env.clone(), (affectedConnector.clone()).clone(), pre.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr, tail: _ }, _, _, _, pre, _) => {
                    let mut prop: DAE::Properties;
                    let mut r#const: DAE::Const = DAE::Const::C_CONST;
                    let mut s1: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (_, Some((_, __pa0, _))) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    prop = __pa0.clone();
                    r#const = Types::propertiesListToConst(list![prop.clone()])?;
                    let false = (Types::isParameterOrConstant(r#const.clone())) else { bail!("pattern mismatch") };
                    s1 = (Dump::printComponentRefStr(cr.clone())?).clone();
                    Error::addSourceMessage(Error::CONNECTOR_ARRAY_NONCONSTANT.clone(), list![(affectedConnector.clone()).clone(), (s1.clone()).clone()], info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn connectExpandableConnectors(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inSets: DAE::Connect::Sets, mut inPrefix: DAE::Prefix, mut inComponentRefLeft: Arc<Absyn::ComponentRef>, mut inComponentRefRight: Arc<Absyn::ComponentRef>, mut inImpl: bool, mut inGraph: ConnectionGraph::ConnectionGraph, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Connect::Sets, DAE::DAElist, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), inPrefix.clone(), inComponentRefLeft.clone(), inComponentRefRight.clone(), inImpl.clone(), inGraph.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, c2, r#impl, graph, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c1_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut attr1: Arc<DAE::Attributes>;
                    let mut attr2: Arc<DAE::Attributes>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut variables1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut variables2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut variablesUnion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    c1_1 = __pa1.clone();
                    attr1 = __pa2.clone();
                    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa3, Some((Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: _ }, _, __pa5))) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    c2_1 = __pa4.clone();
                    attr2 = __pa5.clone();
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (cache, c2_2) = Static::canonCref(cache.clone(), env.clone(), c2_1.clone(), r#impl.clone())?;
                    (attr1, ty1, _, _) = Lookup::lookupConnectorVar(env.clone(), c1_2.clone(), true)?;
                    (attr2, ty2, _, _) = Lookup::lookupConnectorVar(env.clone(), c2_2.clone(), true)?;
                    ::match_deref::match_deref! { match &(attr1.clone()) {
                        Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::POTENTIAL, .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(attr2.clone()) {
                        Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::POTENTIAL, .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let true = (Types::isExpandableConnector(ty1.clone())) else { bail!("pattern mismatch") };
                    let true = (Types::isExpandableConnector(ty2.clone())) else { bail!("pattern mismatch") };
                    (_, _, _, _, _, _, _, env1, _) = Lookup::lookupVar(cache.clone(), env.clone(), c1_2.clone())?;
                    (_, _, _, _, _, _, _, env2, _) = Lookup::lookupVar(cache.clone(), env.clone(), c2_2.clone())?;
                    variables1 = FGraph::getVariablesFromGraphScope(env1.clone())?;
                    variables2 = FGraph::getVariablesFromGraphScope(env2.clone())?;
                    variablesUnion = List::union(variables1.clone(), variables2.clone());
                    variablesUnion = List::sort(variablesUnion.clone(), Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)))?;
                    (cache, env, ih, sets, dae, graph) = connectExpandableVariables(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), variablesUnion.clone(), r#impl.clone(), graph.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, c2, r#impl, graph, _) => {
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, None) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa1, Some((Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, _, _))) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa1.clone();
                    (cache, env, ih, sets, dae, graph) = connectExpandableConnectors(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c2.clone(), c1.clone(), r#impl.clone(), graph.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, pre, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, c2, r#impl, _, _) => {
                    let mut cache = (*cache).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, None) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error: The marked virtual expandable component reference in connect([")); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(pre.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Dump::printComponentRefStr(c1.clone())?); __mm_s.push_str(&*literal!("], ")); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(pre.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Dump::printComponentRefStr(c2.clone())?); __mm_s.push_str(&*literal!("); should be qualified, i.e. expandableConnectorName.virtualName!\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1 @ Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, c2, r#impl, graph, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c1_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut attr2: Arc<DAE::Attributes>;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ct2: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut envExpandable: FCore::Graph;
                    let mut envComponent: FCore::Graph;
                    let mut envComponentEmpty: FCore::Graph;
                    let mut c1_prefix: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut io2: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut vt2: SCode::Variability = SCode::Variability::CONST;
                    let mut prl2: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
                    let mut componentName: ArcStr = arcstr::literal!("");
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut variablesUnion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut vis2: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut arrDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut daeDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, None) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa1, Some((Deref @ DAE::Exp::CREF { componentRef: __pa2, ty: _ }, _, __pa3))) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa1.clone();
                    c2_1 = __pa2.clone();
                    attr2 = __pa3.clone();
                    (cache, c2_2) = Static::canonCref(cache.clone(), env.clone(), c2_1.clone(), r#impl.clone())?;
                    (attr2, ty2, _, _) = Lookup::lookupConnectorVar(env.clone(), c2_2.clone(), true)?;
                    let (__pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(attr2.clone()) {
                        Deref @ DAE::Attributes { connectorType: __pa4, parallelism: __pa5, variability: __pa6, direction: _, innerOuter: __pa7, visibility: __pa8 } => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ct2 = __pa4.clone();
                    prl2 = __pa5.clone();
                    vt2 = __pa6.clone();
                    io2 = __pa7.clone();
                    vis2 = __pa8.clone();
                    c1_prefix = AbsynUtil::crefStripLast(c1.clone())?;
                    let (__pa9, __pa10) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1_prefix.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa9, Some((Deref @ DAE::Exp::CREF { componentRef: __pa10, ty: _ }, _, _))) => (__pa9.clone(), __pa10.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa9.clone();
                    c1_1 = __pa10.clone();
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (_, ty1, _, _) = Lookup::lookupConnectorVar(env.clone(), c1_2.clone(), true)?;
                    let true = (Types::isExpandableConnector(ty1.clone())) else { bail!("pattern mismatch") };
                    c1_2 = ComponentReferenceBasics::crefStripLastSubs(c1_2.clone())?;
                    (_, attr, ty, binding, cnstForRange, _, _, envExpandable, _) = Lookup::lookupVar(cache.clone(), env.clone(), c1_2.clone())?;
                    (_, _, _, _, _, _, _, envComponent, _) = Lookup::lookupVar(cache.clone(), env.clone(), c2_2.clone())?;
                    variablesUnion = FGraph::getVariablesFromGraphScope(envComponent.clone())?;
                    let true = ((variablesUnion.clone().len() as i32) > 1) else { bail!("pattern mismatch") };
                    componentName = (AbsynUtil::crefGetLastIdent(c1.clone())?).clone();
                    envComponentEmpty = FGraph::removeComponentsFromScope(envComponent.clone())?;
                    daeDims = TypesDump::getDimensions(ty2.clone());
                    arrDims = List::map(daeDims.clone(), Arc::new(Expression::unelabDimension));
                    envExpandable = FGraph::cloneLastScopeRef(envExpandable.clone())?;
                    envExpandable = FGraph::mkComponentNode(envExpandable.clone(), Arc::new(DAE::Var { name: (componentName.clone()).clone(), attributes: Arc::new(DAE::Attributes { connectorType: ct2.clone(), parallelism: prl2.clone(), variability: vt2.clone(), direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: io2.clone(), visibility: vis2.clone() }), ty: ty2.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(SCode::Element::COMPONENT { name: (componentName.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: arrDims.clone(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_TYPED, envComponentEmpty.clone())?;
                    env = updateEnvComponentsOnQualPath(cache.clone(), env.clone(), c1_2.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), envExpandable.clone())?;
                    (cache, env, ih, sets, dae, graph) = connectExpandableVariables(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), variablesUnion.clone(), r#impl.clone(), graph.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1 @ Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, c2, r#impl, graph, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c1_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c1p: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2p: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut attr1: Arc<DAE::Attributes>;
                    let mut attr2: Arc<DAE::Attributes>;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ct1: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
                    let mut ct2: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut daeExpandable: DAE::DAElist;
                    let mut envExpandable: FCore::Graph;
                    let mut envComponent: FCore::Graph;
                    let mut envComponentEmpty: FCore::Graph;
                    let mut c1_prefix: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut io1: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut io2: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut vt1: SCode::Variability = SCode::Variability::CONST;
                    let mut vt2: SCode::Variability = SCode::Variability::CONST;
                    let mut prl1: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
                    let mut prl2: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
                    let mut componentName: ArcStr = arcstr::literal!("");
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cnstForRange: Option<DAE::Const> = None;
                    let mut state: ClassInf::State;
                    let mut variablesUnion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut source: Arc<DAE::ElementSource>;
                    let mut vis1: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut vis2: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut arrDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut daeDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, None) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa1, Some((Deref @ DAE::Exp::CREF { componentRef: __pa2, ty: _ }, _, __pa3))) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa1.clone();
                    c2_1 = __pa2.clone();
                    attr2 = __pa3.clone();
                    (cache, c2_2) = Static::canonCref(cache.clone(), env.clone(), c2_1.clone(), r#impl.clone())?;
                    (attr2, ty2, _, _) = Lookup::lookupConnectorVar(env.clone(), c2_2.clone(), true)?;
                    let (__pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(attr2.clone()) {
                        Deref @ DAE::Attributes { connectorType: __pa4, parallelism: __pa5, variability: __pa6, direction: _, innerOuter: __pa7, visibility: __pa8 } => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ct2 = __pa4.clone();
                    prl2 = __pa5.clone();
                    vt2 = __pa6.clone();
                    io2 = __pa7.clone();
                    vis2 = __pa8.clone();
                    c1_prefix = AbsynUtil::crefStripLast(c1.clone())?;
                    let (__pa9, __pa10) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1_prefix.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa9, Some((Deref @ DAE::Exp::CREF { componentRef: __pa10, ty: _ }, _, _))) => (__pa9.clone(), __pa10.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa9.clone();
                    c1_1 = __pa10.clone();
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (attr1, ty1, _, _) = Lookup::lookupConnectorVar(env.clone(), c1_2.clone(), true)?;
                    let true = (Types::isExpandableConnector(ty1.clone())) else { bail!("pattern mismatch") };
                    c1_2 = ComponentReferenceBasics::crefStripLastSubs(c1_2.clone())?;
                    (_, attr, ty, binding, cnstForRange, _, _, envExpandable, _) = Lookup::lookupVar(cache.clone(), env.clone(), c1_2.clone())?;
                    (_, _, _, _, _, _, _, envComponent, _) = Lookup::lookupVar(cache.clone(), env.clone(), c2_2.clone())?;
                    variablesUnion = FGraph::getVariablesFromGraphScope(envComponent.clone())?;
                    let false = ((variablesUnion.clone().len() as i32) > 1) else { bail!("pattern mismatch") };
                    componentName = (AbsynUtil::crefGetLastIdent(c1.clone())?).clone();
                    envComponentEmpty = FGraph::removeComponentsFromScope(envComponent.clone())?;
                    daeDims = TypesDump::getDimensions(ty2.clone());
                    arrDims = List::map(daeDims.clone(), Arc::new(Expression::unelabDimension));
                    envExpandable = FGraph::mkComponentNode(envExpandable.clone(), Arc::new(DAE::Var { name: (componentName.clone()).clone(), attributes: Arc::new(DAE::Attributes { connectorType: ct2.clone(), parallelism: prl2.clone(), variability: vt2.clone(), direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: io2.clone(), visibility: vis2.clone() }), ty: ty2.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(SCode::Element::COMPONENT { name: (componentName.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: arrDims.clone(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_TYPED, envComponentEmpty.clone())?;
                    env = updateEnvComponentsOnQualPath(cache.clone(), env.clone(), c1_2.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), envExpandable.clone())?;
                    let (__pa11, __pa12) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa11, Some((Deref @ DAE::Exp::CREF { componentRef: __pa12, ty: _ }, _, _))) => (__pa11.clone(), __pa12.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa11.clone();
                    c1_1 = __pa12.clone();
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (attr1, ty1, _, _) = Lookup::lookupConnectorVar(env.clone(), c1_2.clone(), true)?;
                    let (__pa13, __pa14, __pa15, __pa16, __pa17) = ::match_deref::match_deref! { match &(attr1.clone()) {
                        Deref @ DAE::Attributes { connectorType: __pa13, parallelism: __pa14, variability: __pa15, direction: _, innerOuter: __pa16, visibility: __pa17 } => (__pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone(), __pa17.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ct1 = __pa13.clone();
                    prl1 = __pa14.clone();
                    vt1 = __pa15.clone();
                    io1 = __pa16.clone();
                    vis1 = __pa17.clone();
                    (cache, env, ih, sets, dae, graph) = instConnect(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), r#impl.clone(), graph.clone(), info.clone())?;
                    state = ClassInf::State::CONNECTOR { path: Arc::new(Absyn::Path::IDENT { name: (literal!("expandable connector")).clone() }), isExpandable: true };
                    (cache, c1p) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1_2.clone())?;
                    (cache, c2p) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c2_2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (c1p.clone(), c2p.clone()))?;
                    (cache, c1_2) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1_2.clone())?;
                    daeDims = TypesDump::getDimensions(ty1.clone());
                    arrDims = List::map(daeDims.clone(), Arc::new(Expression::unelabDimension));
                    daeExpandable = generateExpandableDAE(cache.clone(), env.clone(), envExpandable.clone(), c1_2.clone(), state.clone(), ty1.clone(), SCode::Attributes { arrayDims: arrDims.clone(), connectorType: DAEUtil::toSCodeConnectorType(ct1.clone())?, parallelism: prl1.clone(), variability: vt1.clone(), direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, vis1.clone(), io1.clone(), source.clone())?;
                    dae = DAEUtil::joinDaes(dae.clone(), daeExpandable.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, pre, c1, c2, r#impl, _, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c1_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _, _))) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    c1_1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa2, Some((Deref @ DAE::Exp::CREF { componentRef: __pa3, ty: _ }, _, _))) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    c2_1 = __pa3.clone();
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (cache, c2_2) = Static::canonCref(cache.clone(), env.clone(), c2_1.clone(), r#impl.clone())?;
                    (_, ty1, _, _) = Lookup::lookupConnectorVar(env.clone(), c1_2.clone(), true)?;
                    (_, ty2, _, _) = Lookup::lookupConnectorVar(env.clone(), c2_2.clone(), true)?;
                    let false = (Types::isExpandableConnector(ty1.clone())) else { bail!("pattern mismatch") };
                    let false = (Types::isExpandableConnector(ty2.clone())) else { bail!("pattern mismatch") };
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn generateExpandableDAE(mut inCache: FCore::Cache, mut inParentEnv: FCore::Graph, mut inClassEnv: FCore::Graph, mut cref: Arc<DAE::ComponentRef>, mut state: ClassInf::State, mut ty: Arc<DAE::Type>, mut attrs: SCode::Attributes, mut vis: SCode::Visibility, mut io: Absyn::InnerOuter, mut source: Arc<DAE::ElementSource>) -> Result<DAE::DAElist> {
    let mut outDAE: DAE::DAElist;
    outDAE = (::match_deref::match_deref! { match &((inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), cref.clone(), state.clone(), ty.clone(), attrs.clone(), vis.clone(), io.clone(), source.clone())) {
        (_, _, _, _, _, _, _, _, _, _) => {
            let mut daeDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut daeExpandable: DAE::DAElist;
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            daeDims = TypesDump::getDimensions(ty.clone());
            let _ = List::map(daeDims.clone(), Arc::new(Expression::unelabDimension));
            if daeDims.clone().is_empty() {
                daeExpandable = InstDAE::daeDeclare(inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), cref.clone(), state.clone(), ty.clone(), attrs.clone(), vis.clone(), None, metamodelica::nil(), None, None, Some(Arc::new(SCode::Comment { annotation_: None, comment: Some((literal!("virtual variable in expandable connector")).clone()) })), io.clone(), openmodelica_frontend_types::SCode::Final::NOT_FINAL, source.clone(), true)?;
            } else {
                crefs = ComponentReference::expandCref(cref.clone(), false)?;
                daeExpandable = daeDeclareList(inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), crefs.clone().reverse(), state.clone(), ty.clone(), attrs.clone(), vis.clone(), io.clone(), source.clone(), DAE::emptyDae.clone())?;
            }
            daeExpandable.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn daeDeclareList(mut inCache: FCore::Cache, mut inParentEnv: FCore::Graph, mut inClassEnv: FCore::Graph, mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut state: ClassInf::State, mut ty: Arc<DAE::Type>, mut attrs: SCode::Attributes, mut vis: SCode::Visibility, mut io: Absyn::InnerOuter, mut source: Arc<DAE::ElementSource>, mut acc: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDAE: DAE::DAElist;
    outDAE = (::match_deref::match_deref! { match &((inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), crefs.clone(), state.clone(), ty.clone(), attrs.clone(), vis.clone(), io.clone(), source.clone(), acc.clone())) {
        (_, _, _, Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
            acc.clone()
        },
        (_, _, _, Deref @ metamodelica::List::Cons { head: cref, tail: lst }, _, _, _, _, _, _, _) => {
            let mut daeExpandable: DAE::DAElist;
            daeExpandable = InstDAE::daeDeclare(inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), cref.clone(), state.clone(), ty.clone(), attrs.clone(), vis.clone(), None, metamodelica::nil(), None, None, Some(Arc::new(SCode::Comment { annotation_: None, comment: Some((literal!("virtual variable in expandable connector")).clone()) })), io.clone(), openmodelica_frontend_types::SCode::Final::NOT_FINAL, source.clone(), true)?;
            daeExpandable = DAEUtil::joinDaes(daeExpandable.clone(), acc.clone())?;
            daeExpandable = daeDeclareList(inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), lst.clone(), state.clone(), ty.clone(), attrs.clone(), vis.clone(), io.clone(), source.clone(), daeExpandable.clone())?;
            daeExpandable.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn updateEnvComponentsOnQualPath(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut virtualExpandableCref: Arc<DAE::ComponentRef>, mut virtualExpandableAttr: Arc<DAE::Attributes>, mut virtualExpandableTy: Arc<DAE::Type>, mut virtualExpandableBinding: Arc<DAE::Binding>, mut virtualExpandableCnstForRange: Option<DAE::Const>, mut virtualExpandableEnv: FCore::Graph) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), virtualExpandableCref.clone(), virtualExpandableAttr.clone(), virtualExpandableTy.clone(), virtualExpandableBinding.clone(), virtualExpandableCnstForRange.clone(), virtualExpandableEnv.clone())) {
        (_, topEnv, Deref @ DAE::ComponentRef::CREF_IDENT { ident: currentName, .. }, veAttr, veTy, veBinding, veCnstForRange, veEnv) => {
            let mut updatedEnv: FCore::Graph;
            let mut realEnv: FCore::Graph;
            let mut forLoopScope: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
            (realEnv, forLoopScope) = FGraph::splitGraphScope(topEnv.clone())?;
            updatedEnv = FGraph::updateComp(realEnv.clone(), Arc::new(DAE::Var { name: (currentName.clone()).clone(), attributes: veAttr.clone(), ty: veTy.clone(), binding: veBinding.clone(), bind_from_outside: false, constOfForIteratorRange: veCnstForRange.clone() }), crate::FCore::Status::VAR_TYPED, veEnv.clone())?;
            updatedEnv = FGraph::pushScope(updatedEnv.clone(), forLoopScope.clone())?;
            updatedEnv.clone()
        },
        (cache, topEnv, veCref @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, veAttr, veTy, veBinding, veCnstForRange, veEnv) => {
            let mut qualCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut currentAttr: Arc<DAE::Attributes>;
            let mut currentTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut currentBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let mut currentCnstForRange: Option<DAE::Const> = None;
            let mut updatedEnv: FCore::Graph;
            let mut currentEnv: FCore::Graph;
            let mut realEnv: FCore::Graph;
            let mut forLoopScope: Arc<metamodelica::List<metamodelica::Array<FCore::Node>>> = metamodelica::nil();
            let mut currentName: ArcStr = arcstr::literal!("");
            currentName = (ComponentReferenceBasics::crefLastIdent(veCref.clone())?).clone();
            qualCref = ComponentReference::crefStripLastIdent(veCref.clone())?;
            qualCref = ComponentReferenceBasics::crefStripLastSubs(qualCref.clone())?;
            (_, currentAttr, currentTy, currentBinding, currentCnstForRange, _, _, currentEnv, _) = Lookup::lookupVar(cache.clone(), topEnv.clone(), qualCref.clone())?;
            (realEnv, forLoopScope) = FGraph::splitGraphScope(currentEnv.clone())?;
            currentEnv = FGraph::updateComp(realEnv.clone(), Arc::new(DAE::Var { name: (currentName.clone()).clone(), attributes: veAttr.clone(), ty: veTy.clone(), binding: veBinding.clone(), bind_from_outside: false, constOfForIteratorRange: veCnstForRange.clone() }), crate::FCore::Status::VAR_TYPED, veEnv.clone())?;
            currentEnv = FGraph::pushScope(currentEnv.clone(), forLoopScope.clone())?;
            updatedEnv = updateEnvComponentsOnQualPath(cache.clone(), topEnv.clone(), qualCref.clone(), currentAttr.clone(), currentTy.clone(), currentBinding.clone(), currentCnstForRange.clone(), currentEnv.clone())?;
            updatedEnv.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

fn connectExpandableVariables(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inSets: DAE::Connect::Sets, mut inPrefix: DAE::Prefix, mut inComponentRefLeft: Arc<Absyn::ComponentRef>, mut inComponentRefRight: Arc<Absyn::ComponentRef>, mut inVariablesUnion: Arc<metamodelica::List<ArcStr>>, mut inImpl: bool, mut inGraph: ConnectionGraph::ConnectionGraph, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Connect::Sets, DAE::DAElist, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), inPrefix.clone(), inComponentRefLeft.clone(), inComponentRefRight.clone(), inVariablesUnion.clone(), inImpl.clone(), inGraph.clone(), info.clone())) {
        (cache, env, ih, sets, _, _, _, Deref @ metamodelica::List::Nil, _, graph, _) => {
            (cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::emptyDae.clone(), graph.clone())
        },
        (cache, env, ih, sets, pre, c1, c2, Deref @ metamodelica::List::Cons { head: name, tail: names }, r#impl, graph, _) => {
            let mut dae: DAE::DAElist;
            let mut dae1: DAE::DAElist;
            let mut dae2: DAE::DAElist;
            let mut c1_full: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut c2_full: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut ih = (*ih).clone();
            let mut sets = (*sets).clone();
            let mut graph = (*graph).clone();
            c1_full = AbsynUtil::joinCrefs(c1.clone(), Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() }))?;
            c2_full = AbsynUtil::joinCrefs(c2.clone(), Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() }))?;
            (cache, env, ih, sets, dae1, graph) = instConnect(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1_full.clone(), c2_full.clone(), r#impl.clone(), graph.clone(), info.clone())?;
            (cache, env, ih, sets, dae2, graph) = connectExpandableVariables(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), names.clone(), r#impl.clone(), graph.clone(), info.clone())?;
            dae = DAEUtil::joinDaes(dae1.clone(), dae2.clone())?;
            (cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn getStateFromType(mut ty: Arc<DAE::Type>) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: state, .. } => {
            state.clone()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: state, .. } => {
            state.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outState)
}

fn isConnectorType(mut ty: Arc<DAE::Type>) -> bool {
    let mut isConnector: bool = false;
    isConnector = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: false }, .. } => true,
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: false }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConnector
}

fn flipDirection(mut inDir: Absyn::Direction) -> Result<Absyn::Direction> {
    let mut outDir: Absyn::Direction = Absyn::Direction::BIDIR;
    outDir = (match inDir.clone() {
        Absyn::Direction::INPUT => openmodelica_ast::Absyn::Direction::OUTPUT,
        Absyn::Direction::OUTPUT => openmodelica_ast::Absyn::Direction::INPUT,
        Absyn::Direction::BIDIR => openmodelica_ast::Absyn::Direction::BIDIR,
        _ => bail!("match: no arm matched"),
    });
    Ok(outDir)
}

fn validConnector(mut inType: Arc<DAE::Type>, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inType.clone(), inCref.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_STRING { .. }, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_BOOL { .. }, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { .. }, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_CLOCK { .. }, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: state, .. }, _, _) => {
                    ClassInfUtil::valid(state.clone(), SCode::Restriction::R_CONNECTOR { isExpandable: false })?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: state, .. }, _, _) => {
                    ClassInfUtil::valid(state.clone(), SCode::Restriction::R_CONNECTOR { isExpandable: true })?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: state, .. }, _, _) => {
                    ClassInfUtil::valid(state.clone(), SCode::Restriction::R_CONNECTOR { isExpandable: false })?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: state, .. }, _, _) => {
                    ClassInfUtil::valid(state.clone(), SCode::Restriction::R_CONNECTOR { isExpandable: true })?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: tp, .. }, _, _) => {
                    validConnector(tp.clone(), inCref.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (ConnectUtil::isExpandable(inCref.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
                    Error::addSourceMessage(Error::INVALID_CONNECTOR_TYPE.clone(), list![(r#str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn checkConnectTypes(mut inLhsCref: Arc<DAE::ComponentRef>, mut inLhsType: Arc<DAE::Type>, mut inLhsFace: DAE::Connect::Face, mut inLhsAttributes: Arc<DAE::Attributes>, mut inRhsCref: Arc<DAE::ComponentRef>, mut inRhsType: Arc<DAE::Type>, mut inRhsFace: DAE::Connect::Face, mut inRhsAttributes: Arc<DAE::Attributes>, mut inInfo: SourceInfo) -> Result<()> {
    let mut lhs_ct: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    let mut rhs_ct: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    let mut lhs_dir: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut rhs_dir: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut lhs_io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut rhs_io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut lhs_vis: SCode::Visibility = SCode::Visibility::PROTECTED;
    let mut rhs_vis: SCode::Visibility = SCode::Visibility::PROTECTED;
    ComponentReference::checkCrefSubscriptsBounds(inLhsCref.clone(), inInfo.clone())?;
    ComponentReference::checkCrefSubscriptsBounds(inRhsCref.clone(), inInfo.clone())?;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inLhsAttributes.clone()) {
        Deref @ DAE::Attributes { visibility: __pa0, innerOuter: __pa1, direction: __pa2, connectorType: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhs_vis = __pa0.clone();
    lhs_io = __pa1.clone();
    lhs_dir = __pa2.clone();
    lhs_ct = __pa3.clone();
    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(inRhsAttributes.clone()) {
        Deref @ DAE::Attributes { visibility: __pa4, innerOuter: __pa5, direction: __pa6, connectorType: __pa7, .. } => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    rhs_vis = __pa4.clone();
    rhs_io = __pa5.clone();
    rhs_dir = __pa6.clone();
    rhs_ct = __pa7.clone();
    checkConnectTypesType(inLhsType.clone(), inRhsType.clone(), inLhsCref.clone(), inRhsCref.clone(), inInfo.clone())?;
    checkConnectTypesFlowStream(lhs_ct.clone(), rhs_ct.clone(), inLhsCref.clone(), inRhsCref.clone(), inInfo.clone())?;
    checkConnectTypesDirection(lhs_dir.clone(), inLhsFace.clone(), lhs_vis.clone(), rhs_dir.clone(), inRhsFace.clone(), rhs_vis.clone(), inLhsCref.clone(), inRhsCref.clone(), inInfo.clone())?;
    checkConnectTypesInnerOuter(lhs_io.clone(), rhs_io.clone(), inLhsCref.clone(), inRhsCref.clone(), inInfo.clone())?;
    Ok(())
}

fn checkConnectTypesType(mut inLhsType: Arc<DAE::Type>, mut inRhsType: Arc<DAE::Type>, mut inLhsCref: Arc<DAE::ComponentRef>, mut inRhsCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inLhsType.clone(), inRhsType.clone(), inLhsCref.clone(), inRhsCref.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let true = (Types::equivtypesOrRecordSubtypeOf(inLhsType.clone(), inRhsType.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut t1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cs1: ArcStr = arcstr::literal!("");
                    let mut cs2: ArcStr = arcstr::literal!("");
                    let mut cref_str1: ArcStr = arcstr::literal!("");
                    let mut cref_str2: ArcStr = arcstr::literal!("");
                    t1 = Types::arrayElementType(inLhsType.clone());
                    t2 = Types::arrayElementType(inRhsType.clone());
                    let false = (Types::equivtypesOrRecordSubtypeOf(t1.clone(), t2.clone())?) else { bail!("pattern mismatch") };
                    (_, cs1) = TypesDump::printConnectorTypeStr(t1.clone())?;
                    (_, cs2) = TypesDump::printConnectorTypeStr(t2.clone())?;
                    cref_str1 = (ComponentReferenceBasics::printComponentRefStr(inLhsCref.clone())?).clone();
                    cref_str2 = (ComponentReferenceBasics::printComponentRefStr(inRhsCref.clone())?).clone();
                    Error::addSourceMessage(Error::CONNECT_INCOMPATIBLE_TYPES.clone(), list![(cref_str1.clone()).clone(), (cref_str2.clone()).clone(), (cref_str1.clone()).clone(), (cs1.clone()).clone(), (cref_str2.clone()).clone(), (cs2.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut cref_str1: ArcStr = arcstr::literal!("");
                    let mut cref_str2: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut dims1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    dims1 = TypesDump::getDimensions(inLhsType.clone());
                    dims2 = TypesDump::getDimensions(inRhsType.clone());
                    let false = (List::isEqualOnTrue(dims1.clone(), dims2.clone(), Arc::new(Expression::dimensionsEqual))) else { bail!("pattern mismatch") };
                    let false = (dims1.clone().is_empty() && dims2.clone().is_empty()) else { bail!("pattern mismatch") };
                    cref_str1 = (ComponentReferenceBasics::printComponentRefStr(inLhsCref.clone())?).clone();
                    cref_str2 = (ComponentReferenceBasics::printComponentRefStr(inRhsCref.clone())?).clone();
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionBasics::dimensionsString(dims1.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    str2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionBasics::dimensionsString(dims2.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::CONNECTOR_ARRAY_DIFFERENT.clone(), list![(cref_str1.clone()).clone(), (cref_str2.clone()).clone(), (str1.clone()).clone(), (str2.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn checkConnectTypesFlowStream(mut inLhsConnectorType: Arc<DAE::ConnectorType>, mut inRhsConnectorType: Arc<DAE::ConnectorType>, mut inLhsCref: Arc<DAE::ComponentRef>, mut inRhsCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inLhsConnectorType.clone(), inRhsConnectorType.clone(), inLhsCref.clone(), inRhsCref.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let true = (DAEUtil::connectorTypeEqual(inLhsConnectorType.clone(), inRhsConnectorType.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cref_str1: ArcStr = arcstr::literal!("");
                    let mut cref_str2: ArcStr = arcstr::literal!("");
                    let mut pre_str1: ArcStr = arcstr::literal!("");
                    let mut pre_str2: ArcStr = arcstr::literal!("");
                    let mut err_strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    cref_str1 = (ComponentReferenceBasics::printComponentRefStr(inLhsCref.clone())?).clone();
                    cref_str2 = (ComponentReferenceBasics::printComponentRefStr(inRhsCref.clone())?).clone();
                    pre_str1 = (DAEUtil::connectorTypeStr(inLhsConnectorType.clone())?).clone();
                    pre_str2 = (DAEUtil::connectorTypeStr(inRhsConnectorType.clone())?).clone();
                    err_strl = if (DAEUtil::potentialBool(inLhsConnectorType.clone())) {list![(pre_str2.clone()).clone(), (cref_str2.clone()).clone(), (cref_str1.clone()).clone()]} else {list![(pre_str1.clone()).clone(), (cref_str1.clone()).clone(), (cref_str2.clone()).clone()]};
                    Error::addSourceMessage(Error::CONNECT_PREFIX_MISMATCH.clone(), err_strl.clone(), inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn checkConnectTypesDirection(mut inLhsDirection: Absyn::Direction, mut inLhsFace: DAE::Connect::Face, mut inLhsVisibility: SCode::Visibility, mut inRhsDirection: Absyn::Direction, mut inRhsFace: DAE::Connect::Face, mut inRhsVisibility: SCode::Visibility, mut inLhsCref: Arc<DAE::ComponentRef>, mut inRhsCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inLhsDirection.clone(), inLhsFace.clone(), inLhsVisibility.clone(), inRhsDirection.clone(), inRhsFace.clone(), inRhsVisibility.clone(), inLhsCref.clone(), inRhsCref.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _) => {
                    let false = (isSignalSource(inLhsDirection.clone(), inLhsFace.clone(), inLhsVisibility.clone()) && isSignalSource(inRhsDirection.clone(), inRhsFace.clone(), inRhsVisibility.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cref_str1: ArcStr = arcstr::literal!("");
                    let mut cref_str2: ArcStr = arcstr::literal!("");
                    cref_str1 = (ComponentReferenceBasics::printComponentRefStr(inLhsCref.clone())?).clone();
                    cref_str2 = (ComponentReferenceBasics::printComponentRefStr(inRhsCref.clone())?).clone();
                    Error::addSourceMessage(Error::CONNECT_TWO_SOURCES.clone(), list![(cref_str1.clone()).clone(), (cref_str2.clone()).clone()], inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn isSignalSource(mut inDirection: Absyn::Direction, mut inFace: DAE::Connect::Face, mut inVisibility: SCode::Visibility) -> bool {
    let mut outIsSignal: bool = false;
    outIsSignal = (match (inDirection.clone(), inFace.clone(), inVisibility.clone()) {
        (Absyn::Direction::OUTPUT, DAE::Connect::Face::INSIDE, _) => true,
        (Absyn::Direction::INPUT, DAE::Connect::Face::OUTSIDE, SCode::Visibility::PUBLIC) => true,
        _ => false,
    });
    outIsSignal
}

fn checkConnectTypesInnerOuter(mut inLhsIO: Absyn::InnerOuter, mut inRhsIO: Absyn::InnerOuter, mut inLhsCref: Arc<DAE::ComponentRef>, mut inRhsCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inLhsIO.clone(), inRhsIO.clone(), inLhsCref.clone(), inRhsCref.clone(), inInfo.clone())) {
        (Absyn::InnerOuter::OUTER, Absyn::InnerOuter::OUTER, _, _, _) => {
            let mut cref_str1: ArcStr = arcstr::literal!("");
            let mut cref_str2: ArcStr = arcstr::literal!("");
            cref_str1 = (ComponentReferenceBasics::printComponentRefStr(inLhsCref.clone())?).clone();
            cref_str2 = (ComponentReferenceBasics::printComponentRefStr(inRhsCref.clone())?).clone();
            Error::addSourceMessage(Error::CONNECT_OUTER_OUTER.clone(), list![(cref_str1.clone()).clone(), (cref_str2.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn connectComponents(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inSets: DAE::Connect::Sets, mut inPrefix3: DAE::Prefix, mut cr1: Arc<DAE::ComponentRef>, mut inFace5: DAE::Connect::Face, mut inType6: Arc<DAE::Type>, mut vt1: SCode::Variability, mut cr2: Arc<DAE::ComponentRef>, mut inFace8: DAE::Connect::Face, mut inType9: Arc<DAE::Type>, mut vt2: SCode::Variability, mut inConnectorType: Arc<DAE::ConnectorType>, mut io1: Absyn::InnerOuter, mut io2: Absyn::InnerOuter, mut inGraph: ConnectionGraph::ConnectionGraph, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Connect::Sets, DAE::DAElist, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), inPrefix3.clone(), cr1.clone(), inFace5.clone(), inType6.clone(), vt1.clone(), cr2.clone(), inFace8.clone(), inType9.clone(), vt2.clone(), inConnectorType.clone(), io1.clone(), io2.clone(), inGraph.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, _, _, c2, f2, _, _, ct, _, _, graph, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut sets = (*sets).clone();
                    let false = (DAEUtil::streamBool(ct.clone())) else { bail!("pattern mismatch") };
                    let true = (InnerOuter::outerConnection(io1.clone(), io2.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(PrefixUtil::prefixExp(cache.clone(), env.clone(), ih.clone(), Expression::crefExp(c1.clone())?, pre.clone())?) {
                        (__pa0, Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    c1_1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(PrefixUtil::prefixExp(cache.clone(), env.clone(), ih.clone(), Expression::crefExp(c2.clone())?, pre.clone())?) {
                        (__pa2, Deref @ DAE::Exp::CREF { componentRef: __pa3, ty: _ }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    c2_1 = __pa3.clone();
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (c1_1.clone(), c2_1.clone()))?;
                    sets = ConnectUtil::addOuterConnection(pre.clone(), sets.clone(), c1_1.clone(), c2_1.clone(), io1.clone(), io2.clone(), f1.clone(), f2.clone(), source.clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::emptyDae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, _, t1, _, c2, _, t2, _, Deref @ DAE::ConnectorType::POTENTIAL, _, _, graph, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut source: Arc<DAE::ElementSource>;
                    let mut crefExp1: Arc<DAE::Exp>;
                    let mut crefExp2: Arc<DAE::Exp>;
                    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut const1: DAE::Const = DAE::Const::C_CONST;
                    let mut const2: DAE::Const = DAE::Const::C_CONST;
                    let mut lhsl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut rhsl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let true = (SCodeUtil::isParameterOrConst(vt1.clone()) && SCodeUtil::isParameterOrConst(vt2.clone())) else { bail!("pattern mismatch") };
                    let true = (Types::basicType(Types::arrayElementType(t1.clone()))) else { bail!("pattern mismatch") };
                    let true = (Types::basicType(Types::arrayElementType(t2.clone()))) else { bail!("pattern mismatch") };
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (c1_1.clone(), c2_1.clone()))?;
                    crefExp1 = Expression::crefExp(c1_1.clone())?;
                    crefExp2 = Expression::crefExp(c2_1.clone())?;
                    const1 = Types::variabilityToConst(vt1.clone())?;
                    const2 = Types::variabilityToConst(vt2.clone())?;
                    (cache, crefExp1, _) = Ceval::cevalIfConstant(cache.clone(), env.clone(), crefExp1.clone(), DAE::Properties::PROP { type_: t1.clone(), constFlag: const1.clone() }, true, info.clone())?;
                    (cache, crefExp2, _) = Ceval::cevalIfConstant(cache.clone(), env.clone(), crefExp2.clone(), DAE::Properties::PROP { type_: t2.clone(), constFlag: const2.clone() }, true, info.clone())?;
                    lhsl = Expression::arrayElements(crefExp1.clone())?;
                    rhsl = Expression::arrayElements(crefExp2.clone())?;
                    elts = List::threadMap1(lhsl.clone(), rhsl.clone(), Arc::new(generateConnectAssert), source.clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::DAElist { elementLst: elts.clone() }, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, t1, _, c2, f2, t2, _, _, _, _, graph, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut sets_1: DAE::Connect::Sets;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let true = (Types::basicType(t1.clone())) else { bail!("pattern mismatch") };
                    let true = (Types::basicType(t2.clone())) else { bail!("pattern mismatch") };
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (c1_1.clone(), c2_1.clone()))?;
                    sets_1 = ConnectUtil::addConnection(sets.clone(), c1.clone(), f1.clone(), c2.clone(), f2.clone(), inConnectorType.clone(), source.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), DAE::emptyDae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, _, c2, f2, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _, ct @ Deref @ DAE::ConnectorType::POTENTIAL, _, _, graph, _) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    ::match_deref::match_deref! { match &(Types::arrayElementType(t1.clone())) {
                        Deref @ DAE::Type::T_COMPLEX { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Types::arrayElementType(t2.clone())) {
                        Deref @ DAE::Type::T_COMPLEX { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let true = (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    let _ = Expression::dimensionSize(dim1.clone())?;
                    crefs1 = ComponentReference::expandCref(c1.clone(), false)?;
                    crefs2 = ComponentReference::expandCref(c2.clone(), false)?;
                    (cache, _, ih, sets_1, dae, graph) = connectArrayComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), crefs1.clone(), f1.clone(), t1.clone(), vt1.clone(), io1.clone(), crefs2.clone(), f2.clone(), t2.clone(), vt2.clone(), io2.clone(), ct.clone(), graph.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, _, c2, f2, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _, ct @ Deref @ DAE::ConnectorType::POTENTIAL, _, _, graph, _) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    ::match_deref::match_deref! { match &(Types::arrayElementType(t1.clone())) {
                        Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Types::arrayElementType(t2.clone())) {
                        Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let true = (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    let _ = Expression::dimensionSize(dim1.clone())?;
                    crefs1 = ComponentReference::expandCref(c1.clone(), false)?;
                    crefs2 = ComponentReference::expandCref(c2.clone(), false)?;
                    (cache, _, ih, sets_1, dae, graph) = connectArrayComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), crefs1.clone(), f1.clone(), t1.clone(), vt1.clone(), io1.clone(), crefs2.clone(), f2.clone(), t2.clone(), vt2.clone(), io2.clone(), ct.clone(), graph.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, t1 @ Deref @ DAE::Type::T_ARRAY { .. }, _, c2, f2, t2 @ Deref @ DAE::Type::T_ARRAY { .. }, _, ct, _, _, graph, _) => {
                    let mut c1p: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2p: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut sets_1: DAE::Connect::Sets;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    dims = TypesDump::getDimensions(t1.clone());
                    dims2 = TypesDump::getDimensions(t2.clone());
                    let true = (List::isEqualOnTrue(dims.clone(), dims2.clone(), Arc::new(Expression::dimensionsKnownAndEqual))) else { bail!("pattern mismatch") };
                    (cache, c1p) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1.clone())?;
                    (cache, c2p) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (c1p.clone(), c2p.clone()))?;
                    sets_1 = ConnectUtil::addArrayConnection(sets.clone(), c1.clone(), f1.clone(), c2.clone(), f2.clone(), source.clone(), ct.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), DAE::emptyDae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, t1 @ Deref @ DAE::Type::T_COMPLEX { equalityConstraint: Some((fpath1, idim1, inlineType1)), .. }, _, c2, f2, t2 @ Deref @ DAE::Type::T_COMPLEX { equalityConstraint: Some((_, _, _)), .. }, _, ct @ Deref @ DAE::ConnectorType::POTENTIAL, _, _, graph @ ConnectionGraph::ConnectionGraph { updateGraph: true, .. }, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut sets_1: DAE::Connect::Sets;
                    let mut equalityConstraintFunctionReturnType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut zeroVector: Arc<DAE::Exp>;
                    let mut crefExp1: Arc<DAE::Exp>;
                    let mut crefExp2: Arc<DAE::Exp>;
                    let mut breakDAEElements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut equalityConstraintFunction: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut fpath1 = (*fpath1).clone();
                    let mut graph = (*graph).clone();
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c2.clone())?;
                    (cache, env, ih, sets_1, dae, _) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), t1.clone(), vt1.clone(), c2.clone(), f2.clone(), t2.clone(), vt2.clone(), ct.clone(), io1.clone(), io2.clone(), ConnectionGraph::NOUPDATE_EMPTY.clone(), info.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (c1_1.clone(), c2_1.clone()))?;
                    zeroVector = Expression::makeRealArrayOfZeros(idim1.clone());
                    crefExp1 = Expression::crefExp(c1_1.clone())?;
                    crefExp2 = Expression::crefExp(c2_1.clone())?;
                    equalityConstraintFunctionReturnType = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: idim1.clone() })] });
                    source = ElementSource::addAdditionalComment(source.clone(), (literal!(" equation generated by overconstrained connection graph breaking")).clone())?;
                    breakDAEElements = list![Arc::new(DAE::Element::ARRAY_EQUATION { dimension: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: idim1.clone() })], exp: zeroVector.clone(), array: Arc::new(DAE::Exp::CALL { path: fpath1.clone(), expLst: list![crefExp1.clone(), crefExp2.clone()], attr: Arc::new(DAE::CallAttributes { ty: equalityConstraintFunctionReturnType.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: inlineType1.clone(), tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), source: source.clone() })];
                    graph = ConnectionGraph::addConnection(graph.clone(), c1_1.clone(), c2_1.clone(), breakDAEElements.clone())?;
                    (cache, equalityConstraintFunction, env) = Lookup::lookupClass(cache.clone(), env.clone(), fpath1.clone(), None)?;
                    (cache, fpath1) = Inst::makeFullyQualified(cache.clone(), env.clone(), fpath1.clone())?;
                    cache = FCore::addCachedInstFuncGuard(cache.clone(), fpath1.clone())?;
                    (cache, env, ih) = InstFunction::implicitFunctionInstantiation(cache.clone(), env.clone(), ih.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, equalityConstraintFunction.clone(), metamodelica::nil())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some((fpath1, idim1, inlineType1)), complexType: t1, .. }, _, c2, f2, Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some((_, _, _)), complexType: t2, .. }, _, ct @ Deref @ DAE::ConnectorType::POTENTIAL, _, _, graph @ ConnectionGraph::ConnectionGraph { updateGraph: true, .. }, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut sets_1: DAE::Connect::Sets;
                    let mut equalityConstraintFunctionReturnType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut zeroVector: Arc<DAE::Exp>;
                    let mut crefExp1: Arc<DAE::Exp>;
                    let mut crefExp2: Arc<DAE::Exp>;
                    let mut breakDAEElements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut equalityConstraintFunction: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut fpath1 = (*fpath1).clone();
                    let mut graph = (*graph).clone();
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c2.clone())?;
                    (cache, env, ih, sets_1, dae, _) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), t1.clone(), vt1.clone(), c2.clone(), f2.clone(), t2.clone(), vt2.clone(), ct.clone(), io1.clone(), io2.clone(), ConnectionGraph::NOUPDATE_EMPTY.clone(), info.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (c1_1.clone(), c2_1.clone()))?;
                    zeroVector = Expression::makeRealArrayOfZeros(idim1.clone());
                    crefExp1 = Expression::crefExp(c1_1.clone())?;
                    crefExp2 = Expression::crefExp(c2_1.clone())?;
                    equalityConstraintFunctionReturnType = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: idim1.clone() })] });
                    source = ElementSource::addAdditionalComment(source.clone(), (literal!(" equation generated by overconstrained connection graph breaking")).clone())?;
                    breakDAEElements = list![Arc::new(DAE::Element::ARRAY_EQUATION { dimension: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: idim1.clone() })], exp: zeroVector.clone(), array: Arc::new(DAE::Exp::CALL { path: fpath1.clone(), expLst: list![crefExp1.clone(), crefExp2.clone()], attr: Arc::new(DAE::CallAttributes { ty: equalityConstraintFunctionReturnType.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: inlineType1.clone(), tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), source: source.clone() })];
                    graph = ConnectionGraph::addConnection(graph.clone(), ComponentReferenceBasics::crefStripLastSubs(c1_1.clone())?, ComponentReferenceBasics::crefStripLastSubs(c2_1.clone())?, breakDAEElements.clone())?;
                    (cache, equalityConstraintFunction, env) = Lookup::lookupClass(cache.clone(), env.clone(), fpath1.clone(), None)?;
                    (cache, fpath1) = Inst::makeFullyQualified(cache.clone(), env.clone(), fpath1.clone())?;
                    cache = FCore::addCachedInstFuncGuard(cache.clone(), fpath1.clone())?;
                    (cache, env, ih) = InstFunction::implicitFunctionInstantiation(cache.clone(), env.clone(), ih.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, equalityConstraintFunction.clone(), metamodelica::nil())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: bc_tp1, .. }, _, c2, f2, t2, _, ct, _, _, graph, _) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    (cache, _, ih, sets_1, dae, graph) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), bc_tp1.clone(), vt1.clone(), c2.clone(), f2.clone(), t2.clone(), vt2.clone(), ct.clone(), io1.clone(), io2.clone(), graph.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, t1, _, c2, f2, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: bc_tp2, .. }, _, ct, _, _, graph, _) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    (cache, _, ih, sets_1, dae, graph) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), t1.clone(), vt1.clone(), c2.clone(), f2.clone(), bc_tp2.clone(), vt2.clone(), ct.clone(), io1.clone(), io2.clone(), graph.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_COMPLEX { varLst: Deref @ metamodelica::List::Nil, complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, _, c2, f2, Deref @ DAE::Type::T_COMPLEX { varLst: Deref @ metamodelica::List::Nil, complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, _, _, _, _, graph, _) => {
                    let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut sets_1: DAE::Connect::Sets;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (c1_1.clone(), c2_1.clone()))?;
                    sets_1 = ConnectUtil::addConnection(sets.clone(), c1.clone(), f1.clone(), c2.clone(), f2.clone(), inConnectorType.clone(), source.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), DAE::emptyDae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_COMPLEX { varLst: l1, .. }, _, c2, f2, Deref @ DAE::Type::T_COMPLEX { varLst: l2, .. }, _, ct, _, _, graph, _) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    (cache, _, ih, sets_1, dae, graph) = connectVars(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), l1.clone(), vt1.clone(), c2.clone(), f2.clone(), l2.clone(), vt2.clone(), ct.clone(), io1.clone(), io2.clone(), graph.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, pre, c1, _, t1, _, c2, _, t2, _, _, _, _, _, _) => {
                    let mut c1_str: ArcStr = arcstr::literal!("");
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut c2_str: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    (cache, _) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c1.clone())?;
                    (cache, _) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), c2.clone())?;
                    c1_str = (ComponentReferenceBasics::printComponentRefStr(c1.clone())?).clone();
                    t1_str = (TypesDump::unparseType(t1.clone())?).clone();
                    c2_str = (ComponentReferenceBasics::printComponentRefStr(c2.clone())?).clone();
                    t2_str = (TypesDump::unparseType(t2.clone())?).clone();
                    c1_str = stringAppendList(list![(literal!("\n")).clone(), (c1_str.clone()).clone(), (literal!(" type:\n")).clone(), (t1_str.clone()).clone()]);
                    c2_str = stringAppendList(list![(literal!("\n")).clone(), (c2_str.clone()).clone(), (literal!(" type:\n")).clone(), (t2_str.clone()).clone()]);
                    Error::addSourceMessage(Error::INVALID_CONNECTOR_VARIABLE.clone(), list![(c1_str.clone()).clone(), (c2_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstSection.connectComponents failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn generateConnectAssert(mut inLhsExp: Arc<DAE::Exp>, mut inRhsExp: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Element>> {
    let mut outAssert: Arc<DAE::Element>;
    let mut exp: Arc<DAE::Exp>;
    exp = Arc::new(DAE::Exp::RELATION { exp1: inLhsExp.clone(), operator: DAE::Operator::EQUAL { ty: DAE::T_BOOL_DEFAULT.clone() }, exp2: inRhsExp.clone(), index: -1, optionExpisASUB: None });
    (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
    outAssert = Arc::new(DAE::Element::ASSERT { condition: exp.clone(), message: Arc::new(DAE::Exp::SCONST { string: (literal!("automatically generated from connect")).clone() }), level: DAE::ASSERTIONLEVEL_ERROR.clone(), source: inSource.clone() });
    Ok(outAssert)
}

fn connectArrayComponents(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inSets: DAE::Connect::Sets, mut inPrefix: DAE::Prefix, mut inLhsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inLhsFace: DAE::Connect::Face, mut inLhsType: Arc<DAE::Type>, mut inLhsVar: SCode::Variability, mut inLhsIO: Absyn::InnerOuter, mut inRhsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inRhsFace: DAE::Connect::Face, mut inRhsType: Arc<DAE::Type>, mut inRhsVar: SCode::Variability, mut inRhsIO: Absyn::InnerOuter, mut inConnectorType: Arc<DAE::ConnectorType>, mut inGraph: ConnectionGraph::ConnectionGraph, mut inInfo: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Connect::Sets, DAE::DAElist, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), inPrefix.clone(), inLhsCrefs.clone(), inLhsFace.clone(), inLhsType.clone(), inLhsVar.clone(), inLhsIO.clone(), inRhsCrefs.clone(), inRhsFace.clone(), inRhsType.clone(), inRhsVar.clone(), inRhsIO.clone(), inConnectorType.clone(), inGraph.clone(), inInfo.clone())) {
        (_, _, _, _, _, Deref @ metamodelica::List::Cons { head: lhs, tail: rest_lhs }, _, _, _, _, Deref @ metamodelica::List::Cons { head: rhs, tail: rest_rhs }, _, _, _, _, _, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut ih: InstanceHierarchy = metamodelica::nil();
            let mut sets: DAE::Connect::Sets;
            let mut dae1: DAE::DAElist;
            let mut dae2: DAE::DAElist;
            let mut graph: ConnectionGraph::ConnectionGraph;
            (cache, env, ih, sets, dae1, graph) = connectComponents(inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), inPrefix.clone(), lhs.clone(), inLhsFace.clone(), inLhsType.clone(), inLhsVar.clone(), rhs.clone(), inRhsFace.clone(), inRhsType.clone(), inRhsVar.clone(), inConnectorType.clone(), inLhsIO.clone(), inRhsIO.clone(), inGraph.clone(), inInfo.clone())?;
            (cache, env, ih, sets, dae2, graph) = connectArrayComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), inPrefix.clone(), rest_lhs.clone(), inLhsFace.clone(), inLhsType.clone(), inLhsVar.clone(), inLhsIO.clone(), rest_rhs.clone(), inRhsFace.clone(), inRhsType.clone(), inRhsVar.clone(), inRhsIO.clone(), inConnectorType.clone(), graph.clone(), inInfo.clone())?;
            dae1 = DAEUtil::joinDaes(dae1.clone(), dae2.clone())?;
            (cache.clone(), env.clone(), ih.clone(), sets.clone(), dae1.clone(), graph.clone())
        },
        _ => {
            (inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), DAE::emptyDae.clone(), inGraph.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn connectVars(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inSets: DAE::Connect::Sets, mut inPrefix: DAE::Prefix, mut inComponentRef3: Arc<DAE::ComponentRef>, mut inFace4: DAE::Connect::Face, mut inTypesVarLst5: Arc<metamodelica::List<Arc<DAE::Var>>>, mut vt1: SCode::Variability, mut inComponentRef6: Arc<DAE::ComponentRef>, mut inFace7: DAE::Connect::Face, mut inTypesVarLst8: Arc<metamodelica::List<Arc<DAE::Var>>>, mut vt2: SCode::Variability, mut inConnectorType: Arc<DAE::ConnectorType>, mut io1: Absyn::InnerOuter, mut io2: Absyn::InnerOuter, mut inGraph: ConnectionGraph::ConnectionGraph, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Connect::Sets, DAE::DAElist, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), inPrefix.clone(), inComponentRef3.clone(), inFace4.clone(), inTypesVarLst5.clone(), vt1.clone(), inComponentRef6.clone(), inFace7.clone(), inTypesVarLst8.clone(), vt2.clone(), inConnectorType.clone(), io1.clone(), io2.clone(), inGraph.clone(), info.clone())) {
        (cache, env, ih, sets, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _, graph, _) => {
            (cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::emptyDae.clone(), graph.clone())
        },
        (cache, env, ih, sets, _, c1, f1, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty: ty1, attributes: attr1 @ Deref @ DAE::Attributes { variability: vta, connectorType: ct, .. }, name: n, .. }, tail: xs1 }, _, c2, f2, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty: ty2, attributes: attr2 @ Deref @ DAE::Attributes { variability: vtb, .. }, .. }, tail: xs2 }, _, _, _, _, graph, _) => {
            let mut sets_1: DAE::Connect::Sets;
            let mut sets_2: DAE::Connect::Sets;
            let mut c1_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut c2_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dae: DAE::DAElist;
            let mut dae2: DAE::DAElist;
            let mut dae_1: DAE::DAElist;
            let mut ty_2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            let mut ih = (*ih).clone();
            let mut ct = (*ct).clone();
            let mut graph = (*graph).clone();
            ty_2 = Types::simplifyType(ty1.clone())?;
            ct = propagateConnectorType(inConnectorType.clone(), ct.clone());
            c1_1 = ComponentReference::crefPrependIdent(c1.clone(), (n.clone()).clone(), metamodelica::nil(), ty_2.clone())?;
            c2_1 = ComponentReference::crefPrependIdent(c2.clone(), (n.clone()).clone(), metamodelica::nil(), ty_2.clone())?;
            checkConnectTypes(c1_1.clone(), ty1.clone(), f1.clone(), attr1.clone(), c2_1.clone(), ty2.clone(), f2.clone(), attr2.clone(), info.clone())?;
            (cache, _, ih, sets_1, dae, graph) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), inPrefix.clone(), c1_1.clone(), f1.clone(), ty1.clone(), vta.clone(), c2_1.clone(), f2.clone(), ty2.clone(), vtb.clone(), ct.clone(), io1.clone(), io2.clone(), graph.clone(), info.clone())?;
            (cache, _, ih, sets_2, dae2, graph) = connectVars(cache.clone(), env.clone(), ih.clone(), sets_1.clone(), inPrefix.clone(), c1.clone(), f1.clone(), xs1.clone(), vt1.clone(), c2.clone(), f2.clone(), xs2.clone(), vt2.clone(), inConnectorType.clone(), io1.clone(), io2.clone(), graph.clone(), info.clone())?;
            dae_1 = DAEUtil::joinDaes(dae.clone(), dae2.clone())?;
            (cache.clone(), env.clone(), ih.clone(), sets_2.clone(), dae_1.clone(), graph.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn propagateConnectorType(mut inConnectorType: Arc<DAE::ConnectorType>, mut inSubConnectorType: Arc<DAE::ConnectorType>) -> Arc<DAE::ConnectorType> {
    let mut outSubConnectorType: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    outSubConnectorType = (::match_deref::match_deref! { match &((inConnectorType.clone(), inSubConnectorType.clone())) {
        (Deref @ DAE::ConnectorType::POTENTIAL, _) => inSubConnectorType.clone(),
        _ => inConnectorType.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSubConnectorType
}

fn expandArrayDimension(mut inDim: Arc<DAE::Dimension>, mut inArray: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpl = 'mc: {
        let __mc_input = (inDim.clone(), inArray.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::ARRAY { array: outExpl, .. }) => {
                    Ok(outExpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_INTEGER { integer: sz }, _) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ints: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    ints = List::intRange(sz.clone());
                    expl = List::map1(ints.clone(), Arc::new(makeAsubIndex), inArray.clone());
                    Ok(expl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_BOOLEAN, _) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expl = list![ExpressionSimplify::simplify1(Expression::makeASUB(inArray.clone(), list![Arc::new(DAE::Exp::BCONST { bool: false })])?)?, ExpressionSimplify::simplify1(Expression::makeASUB(inArray.clone(), list![Arc::new(DAE::Exp::BCONST { bool: true })])?)?];
                    Ok(expl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_ENUM { literals: ls, enumTypeName: name, .. }, _) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expl = makeEnumLiteralIndices(name.clone(), ls.clone(), 1, inArray.clone())?;
                    Ok(expl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN, _) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ints: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    ints = List::intRange(1);
                    expl = List::map1(ints.clone(), Arc::new(makeAsubIndex), inArray.clone());
                    Ok(expl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpl)
}

fn makeAsubIndex(mut index: i32, mut expr: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut asub: Arc<DAE::Exp>;
    (asub, _) = ExpressionSimplify::simplify1(Expression::makeASUB(expr.clone(), list![Arc::new(DAE::Exp::ICONST { integer: index.clone() })])?)?;
    Ok(asub)
}

fn makeEnumLiteralIndices(mut enumTypeName: Arc<Absyn::Path>, mut enumLiterals: Arc<metamodelica::List<ArcStr>>, mut enumIndex: i32, mut expr: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut enumIndices: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    enumIndices = (::match_deref::match_deref! { match &((enumTypeName.clone(), enumLiterals.clone(), enumIndex.clone(), expr.clone())) {
        (_, Deref @ metamodelica::List::Nil, _, _) => {
            metamodelica::nil()
        },
        (_, Deref @ metamodelica::List::Cons { head: l, tail: ls }, _, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut enum_type_name: Arc<Absyn::Path>;
            let mut index: i32 = 0;
            enum_type_name = AbsynUtil::joinPaths(enumTypeName.clone(), Arc::new(Absyn::Path::IDENT { name: (l.clone()).clone() }))?;
            e = Arc::new(DAE::Exp::ENUM_LITERAL { name: enum_type_name.clone(), index: enumIndex.clone() });
            (e, _) = ExpressionSimplify::simplify1(Expression::makeASUB(expr.clone(), list![e.clone()])?)?;
            e = if (Expression::isCref(e.clone())) {Expression::unliftExp(e.clone())?} else {e.clone()};
            index = enumIndex.clone() + 1;
            expl = makeEnumLiteralIndices(enumTypeName.clone(), ls.clone(), index.clone(), expr.clone())?;
            cons(e.clone(), expl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(enumIndices)
}

fn getVectorizedCref(mut crefOrArray: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut cref: Arc<DAE::Exp>;
    cref = (::match_deref::match_deref! { match &(crefOrArray.clone()) {
        cref @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => {
            cref.clone()
        },
        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: t }, tail: _ } } => {
            let mut crefExp: Arc<DAE::Exp>;
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            crefExp = Expression::makeCrefExp(cr.clone(), t.clone())?;
            crefExp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

fn checkWhenAlgorithm(mut inWhenAlgorithm: Arc<SCode::Statement>) -> Result<()> {
    let true = (checkForReinitInWhenInitialAlg(inWhenAlgorithm.clone())?) else { bail!("pattern mismatch") };
    checkForNestedWhenInStatements(inWhenAlgorithm.clone())?;
    Ok(())
}

fn checkForReinitInWhenInitialAlg(mut inWhenAlgorithm: Arc<SCode::Statement>) -> Result<bool> {
    let mut outOK: bool = false;
    outOK = 'mc: {
        let __mc_input = inWhenAlgorithm.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_WHEN_A { info, branches: Deref @ metamodelica::List::Cons { head: (exp, algs), tail: _ }, .. } => {
                    let true = (AbsynUtil::expContainsInitial(exp.clone())?) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::algorithmsContainReinit(algs.clone())) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::REINIT_IN_WHEN_INITIAL.clone(), metamodelica::nil(), info.clone())?;
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
    Ok(outOK)
}

fn checkForNestedWhenInStatements(mut inWhenAlgorithm: Arc<SCode::Statement>) -> Result<()> {
    let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
    let mut info: SourceInfo;
    let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inWhenAlgorithm.clone()) {
        Deref @ SCode::Statement::ALG_WHEN_A { info: __pa0, branches: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    branches = __pa1.clone();
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        (_, body) = branch.clone();
        if containsWhenStatements(body.clone())? {
            Error::addSourceMessageAndFail(Error::NESTED_WHEN.clone(), metamodelica::nil(), info.clone())?;
        }
    }
    Ok(())
}

fn checkWhenEquation(mut inWhenEq: Arc<SCode::Equation>) -> Result<()> {
    let true = (checkForReinitInWhenInitialEq(inWhenEq.clone())?) else { bail!("pattern mismatch") };
    checkForNestedWhenInEquation(inWhenEq.clone())?;
    Ok(())
}

fn checkForReinitInWhenInitialEq(mut inWhenEq: Arc<SCode::Equation>) -> Result<bool> {
    let mut outOK: bool = false;
    outOK = 'mc: {
        let __mc_input = inWhenEq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_WHEN { info, eEquationLst: el, condition: exp, .. } => {
                    let true = (AbsynUtil::expContainsInitial(exp.clone())?) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::equationsContainReinit(el.clone())) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::REINIT_IN_WHEN_INITIAL.clone(), metamodelica::nil(), info.clone())?;
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
    Ok(outOK)
}

fn checkForNestedWhenInEquation(mut inWhenEq: Arc<SCode::Equation>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inWhenEq.clone()) {
        Deref @ SCode::Equation::EQ_WHEN { elseBranches: tpl_el, eEquationLst: eqs, .. } => {
            let mut eqs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
            checkForNestedWhenInEqList(eqs.clone());
            eqs_lst = List::map(tpl_el.clone(), Arc::new(fnptr!(Util::tuple22, _)));
            List::map_0(eqs_lst.clone(), Arc::new(fnptr!(checkForNestedWhenInEqList, Arc<metamodelica::List<Arc<SCode::Equation>>>)));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn checkForNestedWhenInEqList(mut inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> () {
    List::map_0(inEqs.clone(), Arc::new(checkForNestedWhenInEq));
    ()
}

fn checkForNestedWhenInEq(mut inEq: Arc<SCode::Equation>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ SCode::Equation::EQ_WHEN { info, .. } => {
            Error::addSourceMessage(Error::NESTED_WHEN.clone(), metamodelica::nil(), info.clone())?;
            bail!("fail")
        },
        Deref @ SCode::Equation::EQ_IF { elseBranch: eqs, thenBranch: eqs_lst, .. } => {
            List::map_0(eqs_lst.clone(), Arc::new(fnptr!(checkForNestedWhenInEqList, Arc<metamodelica::List<Arc<SCode::Equation>>>)));
            checkForNestedWhenInEqList(eqs.clone());
            ()
        },
        Deref @ SCode::Equation::EQ_FOR { eEquationLst: eqs, .. } => {
            checkForNestedWhenInEqList(eqs.clone());
            ()
        },
        Deref @ SCode::Equation::EQ_EQUALS { .. } => {
            ()
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            ()
        },
        Deref @ SCode::Equation::EQ_CONNECT { info, crefRight: cr2, crefLeft: cr1, .. } => {
            let mut cr1_str: ArcStr = arcstr::literal!("");
            let mut cr2_str: ArcStr = arcstr::literal!("");
            cr1_str = (Dump::printComponentRefStr(cr1.clone())?).clone();
            cr2_str = (Dump::printComponentRefStr(cr2.clone())?).clone();
            Error::addSourceMessage(Error::CONNECT_IN_WHEN.clone(), list![(cr1_str.clone()).clone(), (cr2_str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        Deref @ SCode::Equation::EQ_ASSERT { .. } => {
            ()
        },
        Deref @ SCode::Equation::EQ_TERMINATE { .. } => {
            ()
        },
        Deref @ SCode::Equation::EQ_REINIT { .. } => {
            ()
        },
        Deref @ SCode::Equation::EQ_NORETCALL { .. } => {
            ()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- InstSection.checkForNestedWhenInEq failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn instAssignment(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPre: DAE::Prefix, mut alg: Arc<SCode::Statement>, mut source: Arc<DAE::ElementSource>, mut initial_: SCode::Initial, mut r#impl: bool, mut unrollForLoops: bool, mut numError: i32) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    (outCache, stmts) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), ih.clone(), inPre.clone(), alg.clone(), source.clone(), initial_.clone(), r#impl.clone(), unrollForLoops.clone(), numError.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, pre, Deref @ SCode::Statement::ALG_ASSIGN { info, value, assignComponent: var, .. }, _, _, _, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut eprop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts.clone();
                    (cache, e_1, eprop) = Static::elabExp(cache.clone(), env.clone(), value.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, stmts) = instAssignment2(cache.clone(), env.clone(), ih.clone(), pre.clone(), var.clone(), value.clone(), e_1.clone(), eprop.clone(), info.clone(), ElementSource::addAnnotation(source.clone(), var_field!((*alg).comment, SCode::Statement::ALG_ASSIGN).clone()), initial_.clone(), r#impl.clone(), unrollForLoops.clone(), numError.clone())?;
                    Ok((cache.clone(), stmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, pre, Deref @ SCode::Statement::ALG_ASSIGN { info, value, .. }, _, _, _, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (numError.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(Static::elabExp(cache.clone(), env.clone(), value.clone(), r#impl.clone(), true, pre.clone(), info.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    r#str = (Dump::unparseAlgorithmStr(SCodeUtil::statementToAlgorithmItem(alg.clone())?)?).clone();
                    Error::addSourceMessage(Error::ASSIGN_RHS_ELABORATION.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, stmts))
}

fn instAssignment2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPre: DAE::Prefix, mut var: Arc<Absyn::Exp>, mut inRhs: Arc<Absyn::Exp>, mut value: Arc<DAE::Exp>, mut props: DAE::Properties, mut info: SourceInfo, mut inSource: Arc<DAE::ElementSource>, mut initial_: SCode::Initial, mut inImpl: bool, mut unrollForLoops: bool, mut numError: i32) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut varNoComment: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut inRhsNoComment: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    varNoComment = AbsynUtil::stripCommentExpressions(var.clone(), false)?;
    inRhsNoComment = AbsynUtil::stripCommentExpressions(inRhs.clone(), false)?;
    let _ = (::match_deref::match_deref! { match &(varNoComment.clone()) {
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Cons { head: lhs, tail: Deref @ metamodelica::List::Nil } } => {
            (outCache, stmts) = instAssignment2(inCache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), lhs.clone(), inRhsNoComment.clone(), value.clone(), props.clone(), info.clone(), inSource.clone(), initial_.clone(), inImpl.clone(), unrollForLoops.clone(), numError.clone())?;
            return Ok((outCache, stmts));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outCache, stmts) = 'mc: {
        let __mc_input = (inCache.clone(), varNoComment.clone(), value.clone(), props.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: cr }, e_1, _) => {
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut attr: Arc<DAE::Attributes>;
                    let mut lhs_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut rhs_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Static::elabCrefNoEval(cache.clone(), inEnv.clone(), cr.clone(), inImpl.clone(), false, inPre.clone(), info.clone())?) {
                        (__pa0, __pa2 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: __pa1 }, _, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    t = __pa1.clone();
                    lhs = __pa2.clone();
                    attr = __pa3.clone();
                    ::match_deref::match_deref! { match &(t.clone()) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs = e_1.clone();
                    Static::checkAssignmentToInput(varNoComment.clone(), attr.clone(), inEnv.clone(), false, info.clone())?;
                    let __pa5 = ::match_deref::match_deref! { match &(Expression::r#typeof(lhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa5, tail: _ }, .. } => __pa5.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    lhs_dim = __pa5.clone();
                    let __pa6 = ::match_deref::match_deref! { match &(Expression::r#typeof(rhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa6, tail: _ }, .. } => __pa6.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs_dim = __pa6.clone();
                    ::match_deref::match_deref! { match &(expandArrayDimension(lhs_dim.clone(), lhs.clone())?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(expandArrayDimension(rhs_dim.clone(), rhs.clone())?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: cr }, e_1, eprop) => {
                    let mut ce: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ce_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cprop: DAE::Properties;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut stmt: Arc<DAE::Statement>;
                    let mut lt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut rt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut attr: Arc<DAE::Attributes>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Static::elabCrefNoEval(cache.clone(), inEnv.clone(), cr.clone(), inImpl.clone(), false, inPre.clone(), info.clone())?) {
                        (__pa0, Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: __pa2 }, __pa3, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ce = __pa1.clone();
                    t = __pa2.clone();
                    cprop = __pa3.clone();
                    attr = __pa4.clone();
                    Static::checkAssignmentToInput(varNoComment.clone(), attr.clone(), inEnv.clone(), false, info.clone())?;
                    (cache, ce_1) = Static::canonCref(cache.clone(), inEnv.clone(), ce.clone(), inImpl.clone())?;
                    (cache, ce_1) = PrefixUtil::prefixCrefInnerOuter(cache.clone(), inEnv.clone(), inIH.clone(), ce_1.clone(), inPre.clone())?;
                    (cache, t) = PrefixUtil::prefixExpressionsInType(cache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), t.clone())?;
                    lt = Types::getPropType(cprop.clone())?;
                    (cache, lt) = PrefixUtil::prefixExpressionsInType(cache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), lt.clone())?;
                    cprop = Types::setPropType(cprop.clone(), lt.clone())?;
                    (cache, e_1, eprop) = Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl.clone(), info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv.clone(), inIH.clone(), e_1.clone(), inPre.clone())?;
                    rt = Types::getPropType(eprop.clone())?;
                    (cache, rt) = PrefixUtil::prefixExpressionsInType(cache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), rt.clone())?;
                    eprop = Types::setPropType(eprop.clone(), rt.clone())?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = makeAssignment(Expression::makeCrefExp(ce_1.clone(), t.clone())?, cprop.clone(), e_2.clone(), eprop.clone(), attr.clone(), initial_.clone(), source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, e2 @ Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "der", .. }, .. }, e_1, eprop) => {
                    let mut cprop: DAE::Properties;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut e2_2: Arc<DAE::Exp>;
                    let mut e2_2_2: Arc<DAE::Exp>;
                    let mut stmt: Arc<DAE::Statement>;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    (cache, _, cprop, attr) = Static::elabCrefNoEval(cache.clone(), inEnv.clone(), cr.clone(), inImpl.clone(), false, inPre.clone(), info.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabExp(cache.clone(), inEnv.clone(), e2.clone(), inImpl.clone(), true, inPre.clone(), info.clone())?) {
                        (__pa0, __pa1 @ Deref @ DAE::Exp::CALL { .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e2_2 = __pa1.clone();
                    (cache, e2_2_2) = PrefixUtil::prefixExp(cache.clone(), inEnv.clone(), inIH.clone(), e2_2.clone(), inPre.clone())?;
                    (cache, e_1, eprop) = Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl.clone(), info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv.clone(), inIH.clone(), e_1.clone(), inPre.clone())?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = makeAssignment(e2_2_2.clone(), cprop.clone(), e_2.clone(), eprop.clone(), attr.clone(), initial_.clone(), source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: cr }, e_1, eprop) => {
                    let mut cprop: DAE::Properties;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut cre: Arc<DAE::Exp>;
                    let mut cre2: Arc<DAE::Exp>;
                    let mut stmt: Arc<DAE::Statement>;
                    let mut attr: Arc<DAE::Attributes>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    (cache, cre, cprop, attr) = Static::elabCrefNoEval(cache.clone(), inEnv.clone(), cr.clone(), inImpl.clone(), false, inPre.clone(), info.clone())?;
                    Static::checkAssignmentToInput(varNoComment.clone(), attr.clone(), inEnv.clone(), false, info.clone())?;
                    (cache, cre2) = PrefixUtil::prefixExp(cache.clone(), inEnv.clone(), inIH.clone(), cre.clone(), inPre.clone())?;
                    (cache, e_1, eprop) = Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl.clone(), info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv.clone(), inIH.clone(), e_1.clone(), inPre.clone())?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = makeAssignment(cre2.clone(), cprop.clone(), e_2.clone(), eprop.clone(), attr.clone(), initial_.clone(), source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: expl }, e_1, eprop) => {
                    let mut e_2: Arc<DAE::Exp>;
                    let mut stmt: Arc<DAE::Statement>;
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expl_2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cprops: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut attrs: Arc<metamodelica::List<Arc<DAE::Attributes>>> = metamodelica::nil();
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    let true = (List::all(expl.clone(), Arc::new(fnptr!(AbsynUtil::isCref, Arc<Absyn::Exp>)))) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl.clone(), info.clone())?) {
                        (__pa0, __pa1 @ Deref @ DAE::Exp::CALL { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e_1 = __pa1.clone();
                    eprop = __pa2.clone();
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv.clone(), inIH.clone(), e_1.clone(), inPre.clone())?;
                    (cache, expl_1, cprops, attrs) = Static::elabExpCrefNoEvalList(cache.clone(), inEnv.clone(), expl.clone(), inImpl.clone(), false, inPre.clone(), info.clone())?;
                    Static::checkAssignmentToInputs(expl.clone(), attrs.clone(), inEnv.clone(), info.clone())?;
                    checkNoDuplicateAssignments(expl_1.clone(), info.clone())?;
                    (cache, expl_2) = PrefixUtil::prefixExpList(cache.clone(), inEnv.clone(), inIH.clone(), expl_1.clone(), inPre.clone())?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = Algorithm::makeTupleAssignment(expl_2.clone(), cprops.clone(), e_2.clone(), eprop.clone(), initial_.clone(), source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: expl }, e_1, eprop) => {
                    let mut e_2: Arc<DAE::Exp>;
                    let mut stmt: Arc<DAE::Statement>;
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expl_2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cprops: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut attrs: Arc<metamodelica::List<Arc<DAE::Attributes>>> = metamodelica::nil();
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let true = (List::all(expl.clone(), Arc::new(fnptr!(AbsynUtil::isCref, Arc<Absyn::Exp>)))) else { bail!("pattern mismatch") };
                    let true = (Types::isTuple(Types::getPropType(eprop.clone())?)) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl.clone(), info.clone())?) {
                        (__pa0, __pa1 @ Deref @ DAE::Exp::MATCHEXPRESSION { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e_1 = __pa1.clone();
                    eprop = __pa2.clone();
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv.clone(), inIH.clone(), e_1.clone(), inPre.clone())?;
                    (cache, expl_1, cprops, attrs) = Static::elabExpCrefNoEvalList(cache.clone(), inEnv.clone(), expl.clone(), inImpl.clone(), false, inPre.clone(), info.clone())?;
                    Static::checkAssignmentToInputs(expl.clone(), attrs.clone(), inEnv.clone(), info.clone())?;
                    checkNoDuplicateAssignments(expl_1.clone(), info.clone())?;
                    (cache, expl_2) = PrefixUtil::prefixExpList(cache.clone(), inEnv.clone(), inIH.clone(), expl_1.clone(), inPre.clone())?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = Algorithm::makeTupleAssignment(expl_2.clone(), cprops.clone(), e_2.clone(), eprop.clone(), initial_.clone(), source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, left, e_1, prop) => {
                    let mut stmt: Arc<DAE::Statement>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut pattern: Arc<DAE::Pattern> = Arc::new(DAE::Pattern::PAT_WILD);
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    ty = Types::getPropType(prop.clone())?;
                    (e_1, ty) = Types::convertTupleToMetaTuple(e_1.clone(), ty.clone())?;
                    (cache, pattern) = Patternm::elabPatternCheckDuplicateBindings(cache.clone(), inEnv.clone(), left.clone(), ty.clone(), info.clone())?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = if (Types::isEmptyOrNoRetcall(ty.clone())) {Arc::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() })} else {Arc::new(DAE::Statement::STMT_ASSIGN { type_: DAE::T_UNKNOWN_DEFAULT.clone(), exp1: Arc::new(DAE::Exp::PATTERN { pattern: pattern.clone() }), exp: e_1.clone(), source: source.clone() })};
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: expl }, e_1, eprop) => {
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expl_2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cprops: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut eprops: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut attrs: Arc<metamodelica::List<Arc<DAE::Attributes>>> = metamodelica::nil();
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts.clone();
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl.clone(), info.clone())?) {
                        (__pa0, __pa2 @ Deref @ DAE::Exp::TUPLE { PR: __pa1 }, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    expl_1 = __pa1.clone();
                    e_1 = __pa2.clone();
                    eprop = __pa3.clone();
                    (cache, expl_2, cprops, attrs) = Static::elabExpCrefNoEvalList(cache.clone(), inEnv.clone(), expl.clone(), inImpl.clone(), false, inPre.clone(), info.clone())?;
                    Static::checkAssignmentToInputs(expl.clone(), attrs.clone(), inEnv.clone(), info.clone())?;
                    checkNoDuplicateAssignments(expl_2.clone(), info.clone())?;
                    (cache, expl_2) = PrefixUtil::prefixExpList(cache.clone(), inEnv.clone(), inIH.clone(), expl_2.clone(), inPre.clone())?;
                    eprops = Types::propTuplePropList(eprop.clone())?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmts = Algorithm::makeAssignmentsList(expl_2.clone(), cprops.clone(), expl_1.clone(), eprops.clone(), DAE::dummyAttrVar.clone(), initial_.clone(), source.clone())?;
                    Ok((cache.clone(), stmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ Absyn::Exp::TUPLE { expressions: expl }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let false = (List::all(expl.clone(), Arc::new(fnptr!(AbsynUtil::isCref, Arc<Absyn::Exp>)))) else { bail!("pattern mismatch") };
                    s = (Dump::printExpStr(e.clone())?).clone();
                    Error::addSourceMessage(Error::TUPLE_ASSIGN_CREFS_ONLY.clone(), list![(s.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, e1 @ Deref @ Absyn::Exp::TUPLE { expressions: expl }, _, prop2) => {
                    let mut prop1: DAE::Properties;
                    let mut e_1: Arc<DAE::Exp>;
                    let mut lt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut rt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut lhs_str: ArcStr = arcstr::literal!("");
                    let mut rhs_str: ArcStr = arcstr::literal!("");
                    let mut lt_str: ArcStr = arcstr::literal!("");
                    let mut rt_str: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    ::match_deref::match_deref! { match &(inRhsNoComment.clone()) {
                        Deref @ Absyn::Exp::CALL { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let true = (List::all(expl.clone(), Arc::new(fnptr!(AbsynUtil::isCref, Arc<Absyn::Exp>)))) else { bail!("pattern mismatch") };
                    (cache, e_1, prop1) = Static::elabExpLHS(cache.clone(), inEnv.clone(), e1.clone(), inImpl.clone(), false, inPre.clone(), info.clone())?;
                    lt = Types::getPropType(prop1.clone())?;
                    rt = Types::getPropType(prop2.clone())?;
                    let false = (Types::subtype(lt.clone(), rt.clone(), true)?) else { bail!("pattern mismatch") };
                    lhs_str = (ExpressionBasics::printExpStr(e_1.clone())?).clone();
                    rhs_str = (Dump::printExpStr(inRhs.clone())?).clone();
                    lt_str = (TypesDump::unparseTypeNoAttr(lt.clone())?).clone();
                    rt_str = (TypesDump::unparseTypeNoAttr(rt.clone())?).clone();
                    Types::typeErrorSanityCheck((lt_str.clone()).clone(), (rt_str.clone()).clone(), info.clone())?;
                    Error::addSourceMessage(Error::ASSIGN_TYPE_MISMATCH_ERROR.clone(), list![(lhs_str.clone()).clone(), (rhs_str.clone()).clone(), (lt_str.clone()).clone(), (rt_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Exp::TUPLE { expressions: expl }, e_1, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (List::all(expl.clone(), Arc::new(fnptr!(AbsynUtil::isCref, Arc<Absyn::Exp>)))) else { bail!("pattern mismatch") };
                    if '__try0: {
                        ::match_deref::match_deref! { match &(inRhsNoComment.clone()) {
                            Deref @ Absyn::Exp::CALL { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s = (ExpressionBasics::printExpStr(e_1.clone())?).clone();
                    Error::addSourceMessage(Error::TUPLE_ASSIGN_FUNCALL_ONLY.clone(), list![(s.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let true = (numError.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
                    s1 = (Dump::printExpStr(var.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(value.clone())?).clone();
                    Error::addSourceMessage(Error::ASSIGN_UNKNOWN_ERROR.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, stmts))
}

fn checkNoDuplicateAssignments(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut info: SourceInfo) -> Result<()> {
    let mut exp: Arc<DAE::Exp>;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = inExps.clone();
    while !(exps.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
        exps = __pa1.clone();
        if Expression::isWild(exp.clone()) {
            continue;
        } else if listMember(exp.clone(), exps.clone()) {
            Error::addSourceMessage(Error::DUPLICATE_DEFINITION.clone(), list![(ExpressionBasics::printExpStr(exp.clone())?).clone()], info.clone())?;
            bail!("fail");
        }
    }
    Ok(())
}

fn getIteratorType(mut ty: Arc<DAE::Type>, mut id: ArcStr, mut info: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut oty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    oty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (TypesDump::unparseType(ty.clone())?).clone();
            Error::addSourceMessage(Error::ITERATOR_NON_ARRAY.clone(), list![(id.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        Deref @ DAE::Type::T_ARRAY { ty: oty, .. } => {
            oty.clone()
        },
        Deref @ DAE::Type::T_METALIST { ty: oty } => {
            Types::boxIfUnboxedType(oty.clone())?
        },
        Deref @ DAE::Type::T_METAARRAY { ty: oty } => {
            Types::boxIfUnboxedType(oty.clone())?
        },
        Deref @ DAE::Type::T_METATYPE { ty: oty } => {
            getIteratorType(var_field!((*ty).ty, DAE::Type::T_METATYPE).clone(), (id.clone()).clone(), info.clone())?
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (TypesDump::unparseType(ty.clone())?).clone();
            Error::addSourceMessage(Error::ITERATOR_NON_ARRAY.clone(), list![(id.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oty)
}

fn instParForStatement(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inForStatement: Arc<SCode::Statement>, mut inSource: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inUnrollLoops: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut iterator: ArcStr = arcstr::literal!("");
    let mut oarange: Option<Arc<Absyn::Exp>> = None;
    let mut arange: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut range: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let mut info: SourceInfo;
    let mut iter_crefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inForStatement.clone()) {
        Deref @ SCode::Statement::ALG_PARFOR { info: __pa0, parforBody: __pa1, range: __pa2, index: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    body = __pa1.clone();
    oarange = __pa2.clone();
    iterator = __pa3.clone();
    if isSome(oarange.clone()) {
        let __pa4 = ::match_deref::match_deref! { match &(oarange.clone()) {
            Some(__pa4) => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        arange = __pa4.clone();
        (outCache, range, prop) = Static::elabExp(inCache.clone(), inEnv.clone(), arange.clone(), inImpl.clone(), true, inPrefix.clone(), info.clone())?;
    } else {
        iter_crefs = SCodeUtil::findIteratorIndexedCrefsInStatements(body.clone(), (iterator.clone()).clone(), metamodelica::nil());
        (range, prop, outCache) = Static::deduceIterationRange((iterator.clone()).clone(), iter_crefs.clone(), inEnv.clone(), inCache.clone(), info.clone())?;
    }
    if containsWhenStatements(body.clone())? {
        (outCache, outStatements) = unrollForLoop(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), (iterator.clone()).clone(), range.clone(), prop.clone(), body.clone(), inForStatement.clone(), info.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone())?;
    } else {
        (outCache, outStatements) = instParForStatement_dispatch(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), (iterator.clone()).clone(), range.clone(), prop.clone(), body.clone(), info.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone())?;
    }
    Ok((outCache, outStatements))
}

fn instParForStatement_dispatch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inIterator: ArcStr, mut inRange: Arc<DAE::Exp>, mut inRangeProps: DAE::Properties, mut inBody: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut inInfo: SourceInfo, mut inSource: Arc<DAE::ElementSource>, mut inInitial: SCode::Initial, mut inImpl: bool, mut inUnrollLoops: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut env: FCore::Graph;
    let mut source: Arc<DAE::ElementSource>;
    let mut loop_prl_vars: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
    let mut parfor_iter: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut range: Arc<DAE::Exp>;
    c = Types::getPropConst(inRangeProps.clone())?;
    if Types::isParameterOrConstant(c.clone()) {
        if '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Ceval::ceval(outCache.clone(), inEnv.clone(), inRange.clone(), inImpl.clone(), Absyn::Msg::MSG { info: inInfo.clone() }, 0), '__try0)) {
                (__pa1, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, .. }) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            outCache = __pa1.clone();
            outStatements = metamodelica::nil();
            return Ok((outCache, outStatements));
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    ty = Types::getPropType(inRangeProps.clone())?;
    ty = getIteratorType(ty.clone(), (inIterator.clone()).clone(), inInfo.clone())?;
    (outCache, range) = Ceval::cevalRangeIfConstant(outCache.clone(), inEnv.clone(), inRange.clone(), inRangeProps.clone(), inImpl.clone(), inInfo.clone())?;
    (outCache, range) = PrefixUtil::prefixExp(outCache.clone(), inEnv.clone(), inIH.clone(), range.clone(), inPrefix.clone())?;
    env = addParForLoopScope(inEnv.clone(), (inIterator.clone()).clone(), ty.clone(), openmodelica_frontend_types::SCode::Variability::VAR, Some(c.clone()))?;
    (outCache, outStatements) = instStatements(outCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), inState.clone(), inBody.clone(), inSource.clone(), inInitial.clone(), inImpl.clone(), inUnrollLoops.clone())?;
    loop_prl_vars = collectParallelVariables(metamodelica::nil(), outStatements.clone())?;
    parfor_iter = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (inIterator.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() });
    (loop_prl_vars, _) = List::deleteMemberOnTrue(parfor_iter.clone(), loop_prl_vars.clone(), Arc::new(fnptr!(crefInfoListCrefsEqual, Arc<DAE::ComponentRef>, (Arc<DAE::ComponentRef>, SourceInfo))))?;
    List::map2_0(loop_prl_vars.clone(), Arc::new(isCrefParGlobalOrForIterator), outCache.clone(), env.clone());
    source = ElementSource::addElementSourceFileInfo(inSource.clone(), inInfo.clone());
    outStatements = list![Algorithm::makeParFor((inIterator.clone()).clone(), range.clone(), inRangeProps.clone(), outStatements.clone(), loop_prl_vars.clone(), source.clone())?];
    Ok((outCache, outStatements))
}

fn isCrefParGlobalOrForIterator(mut inCrefInfo: (Arc<DAE::ComponentRef>, SourceInfo), mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inCrefInfo.clone(), inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((cref, _), _, _) => {
                    let mut prl: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
                    let mut isParglobal: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupVar(inCache.clone(), inEnv.clone(), cref.clone())?) {
                        (_, Deref @ DAE::Attributes { parallelism: __pa0, .. }, _, _, _, _, _, _, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    prl = __pa0.clone();
                    isParglobal = SCodeUtil::parallelismEqual(prl.clone(), openmodelica_frontend_types::SCode::Parallelism::PARGLOBAL);
                    let true = (isParglobal.clone()) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((cref, info), _, _) => {
                    let mut errorString: ArcStr = arcstr::literal!("");
                    errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Component '")); __mm_s.push_str(&*AbsynUtil::pathString(ComponentReference::crefToPath(cref.clone())?, (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' is used in a parallel for loop.")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel for loops can only contain references to parglobal variables.")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn crefInfoListCrefsEqual(mut inFoundCref: Arc<DAE::ComponentRef>, mut inCrefInfos: (Arc<DAE::ComponentRef>, SourceInfo)) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inFoundCref.clone(), inCrefInfos.clone())) {
        (_, (cref1, _)) => {
            ComponentReferenceBasics::crefEqualWithoutSubs(cref1.clone(), inFoundCref.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn collectParallelVariables(mut inCrefInfos: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>>, mut inStatments: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>>> {
    let mut outCrefInfos: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
    outCrefInfos = 'mc: {
        let __mc_input = (inCrefInfos.clone(), inStatments.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(inCrefInfos.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { type_: _, exp1, exp: exp2, source: Deref @ DAE::ElementSource { info, .. } }, tail: restStmts }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone(), exp2.clone()], info.clone())?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), restStmts.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_FOR { source: Deref @ DAE::ElementSource { info, .. }, statementLst: stmtList, range: exp1, iter, type_: iterType, .. }, tail: restStmts }) => {
                    let mut foundCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone()], info.clone())?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), stmtList.clone())?;
                    foundCref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iter.clone()).clone(), identType: iterType.clone(), subscriptLst: metamodelica::nil() });
                    (crefInfoList, _) = List::deleteMemberOnTrue(foundCref.clone(), crefInfoList.clone(), Arc::new(fnptr!(crefInfoListCrefsEqual, Arc<DAE::ComponentRef>, (Arc<DAE::ComponentRef>, SourceInfo))))?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), restStmts.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { exp: exp1, statementLst: stmtList, else_: _, source: Deref @ DAE::ElementSource { info, .. } }, tail: restStmts }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone()], info.clone())?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), stmtList.clone())?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), restStmts.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHILE { exp: exp1, statementLst: stmtList, source: Deref @ DAE::ElementSource { info, .. } }, tail: restStmts }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone()], info.clone())?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), stmtList.clone())?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), restStmts.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: _, tail: restStmts }) => {
                    Ok(collectParallelVariables(crefInfoList.clone(), restStmts.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefInfos)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn collectParallelVariablesinExps(mut inCrefInfos: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>>, mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>>> {
    let mut outCrefInfos: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
    outCrefInfos = 'mc: {
        let __mc_input = (inCrefInfos.clone(), inExps.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(inCrefInfos.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: foundCref, ty: _ }, tail: restExps }, _) => {
                    let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut alreadyInList: bool = false;
                    let mut crefInfoList = (*crefInfoList).clone();
                    alreadyInList = List::isMemberOnTrue(foundCref.clone(), crefInfoList.clone(), Arc::new(fnptr!(crefInfoListCrefsEqual, Arc<DAE::ComponentRef>, (Arc<DAE::ComponentRef>, SourceInfo))));
                    crefInfoList = if (alreadyInList.clone()) {crefInfoList.clone()} else {cons((foundCref.clone(), inInfo.clone()), crefInfoList.clone())};
                    let __pa0 = ::match_deref::match_deref! { match &(foundCref.clone()) {
                        Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    subscriptLst = __pa0.clone();
                    crefInfoList = collectParallelVariablesInSubscriptList(crefInfoList.clone(), subscriptLst.clone(), inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ASUB { exp: exp1, sub: subs }, tail: restExps }, _) => {
                    let mut expLst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crefInfoList = (*crefInfoList).clone();
                    expLst1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), cons(exp1.clone(), expLst1.clone()), inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1, operator: _, exp2 }, tail: restExps }, _) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone(), exp2.clone()], inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { operator: _, exp: exp1 }, tail: restExps }, _) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone()], inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LBINARY { exp1, operator: _, exp2 }, tail: restExps }, _) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone(), exp2.clone()], inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LUNARY { operator: _, exp: exp1 }, tail: restExps }, _) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone()], inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RANGE { ty: _, start: exp1, step: Some(exp2), stop: exp3 }, tail: restExps }, _) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone(), exp2.clone(), exp3.clone()], inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RANGE { ty: _, start: exp1, step: None, stop: exp3 }, tail: restExps }, _) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone(), exp3.clone()], inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CAST { ty: _, exp: exp1 }, tail: restExps }, _) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone()], inInfo.clone())?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: _, tail: restExps }, _) => {
                    Ok(collectParallelVariablesinExps(crefInfoList.clone(), restExps.clone(), inInfo.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefInfos)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn collectParallelVariablesInSubscriptList(mut inCrefInfos: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>>, mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>>> {
    let mut outCrefInfos: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
    outCrefInfos = 'mc: {
        let __mc_input = (inCrefInfos.clone(), inSubscriptLst.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(inCrefInfos.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: exp1 }, tail: restSubs }, _) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), list![exp1.clone()], inInfo.clone())?;
                    crefInfoList = collectParallelVariablesInSubscriptList(crefInfoList.clone(), restSubs.clone(), inInfo.clone())?;
                    Ok(crefInfoList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::List::Cons { head: _, tail: restSubs }, _) => {
                    Ok(collectParallelVariablesInSubscriptList(crefInfoList.clone(), restSubs.clone(), inInfo.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefInfos)
}

fn checkValidNoRetcall(mut exp: Arc<DAE::Exp>, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((exp.clone(), info.clone())) {
        (Deref @ DAE::Exp::CALL { .. }, _) => {
            ()
        },
        (Deref @ DAE::Exp::REDUCTION { .. }, _) => {
            ()
        },
        (Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::List::Nil }, _) => {
            ()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            Error::addSourceMessage(Error::NORETCALL_INVALID_EXP.clone(), list![(r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

