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
// two functions "addLabelToExpList" and "addLabelToExpListForSubstitution", were commented in old version
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::BackendDAE;
use crate::BackendVarTransform;
use crate::Differentiate;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub const LABELNAME: &'static str = "label";

pub fn buildLabels(mut inEquationLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut inModelInfo: SimCode::ModelInfo, mut reduceList: Arc<metamodelica::List<i32>>, mut inArgs: Arc<Absyn::FunctionArgs>) -> Result<(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, SimCode::ModelInfo)> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut outModelInfo: SimCode::ModelInfo;
    (outEquationLst, outModelInfo) = 'mc: {
        let __mc_input = (inEquationLst.clone(), inModelInfo.clone(), reduceList.clone(), inArgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqns, modelInfo @ SimCode::ModelInfo { varInfo: varInfo @ SimCode::VarInfo { .. }, .. }, _, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: exp_list }, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
                    let mut eqns_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut labels_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels_2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut p: i32 = 0;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut modelInfo = (*modelInfo).clone();
                    let mut varInfo = (*varInfo).clone();
                    repl = meanValueReplacements(modelInfo.vars.clone(), exp_list.clone())?;
                    let (__pa0, __pa1, (_, __pa2), __pa3) = addLabelToEquations(eqns.clone(), modelInfo.vars.clone(), (0, varInfo.numParams.clone()), reduceList.clone(), repl.clone())?;
                    eqns_1 = __pa0.clone();
                    vars_1 = __pa1.clone();
                    p = __pa2.clone();
                    labels_1 = __pa3.clone();
                    labels_2 = listAppend(modelInfo.labels.clone(), labels_1.clone());
                    if varInfo.numParams.clone() != p.clone() {
                        todo!("unhandled field-assign shape: varInfo.numParams");
                        todo!("unhandled field-assign shape: modelInfo.varInfo");
                    }
                    todo!("unhandled field-assign shape: modelInfo.labels");
                    if !(referenceEq(&modelInfo.vars.clone(),&vars_1.clone())) {
                        todo!("unhandled field-assign shape: modelInfo.vars");
                    }
                    Ok((eqns_1.clone(), modelInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqns, modelInfo @ SimCode::ModelInfo { varInfo: varInfo @ SimCode::VarInfo { .. }, .. }, _, _) => {
                    let mut eqns_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut labels_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels_2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut p: i32 = 0;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut modelInfo = (*modelInfo).clone();
                    let mut varInfo = (*varInfo).clone();
                    repl = BackendVarTransform::emptyReplacements();
                    let (__pa0, __pa1, (_, __pa2), __pa3) = addLabelToEquations(eqns.clone(), modelInfo.vars.clone(), (0, varInfo.numParams.clone()), reduceList.clone(), repl.clone())?;
                    eqns_1 = __pa0.clone();
                    vars_1 = __pa1.clone();
                    p = __pa2.clone();
                    labels_1 = __pa3.clone();
                    labels_2 = listAppend(modelInfo.labels.clone(), labels_1.clone());
                    if varInfo.numParams.clone() != p.clone() {
                        todo!("unhandled field-assign shape: varInfo.numParams");
                        todo!("unhandled field-assign shape: modelInfo.varInfo");
                    }
                    todo!("unhandled field-assign shape: modelInfo.labels");
                    if !(referenceEq(&modelInfo.vars.clone(),&vars_1.clone())) {
                        todo!("unhandled field-assign shape: modelInfo.vars");
                    }
                    Ok((eqns_1.clone(), modelInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquationLst, outModelInfo))
}

pub fn reduceTerms(mut inEquationLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut inModelInfo: SimCode::ModelInfo, mut inArgs: Arc<Absyn::FunctionArgs>) -> Result<(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, SimCode::ModelInfo)> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut outModelInfo: SimCode::ModelInfo;
    (outEquationLst, outModelInfo) = ({
        let mut reduceListStr: ArcStr = literal!("");
        (::match_deref::match_deref! { match &((inEquationLst.clone(), inModelInfo.clone(), inArgs.clone())) {
        (eqns, modelInfo, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: inNamedArgList, args: inExpArgList }) => {
            let mut reduceList: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut outExpList: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut modelInfo_1: SimCode::ModelInfo;
            let mut eqns = (*eqns).clone();
            (_, outExpList) = AbsynUtil::getNamedFuncArgNamesAndValues(inNamedArgList.clone());
            reduceListStr = (System::stringReplace((ExpressionBasics::printExpStr(Expression::fromAbsynExp((outExpList.clone()).get(1)?)?)?).clone(), (literal!("\"")).clone(), (literal!("")).clone())?).clone();
            reduceList = StringDelimit2Int((reduceListStr.clone()).clone(), (literal!(",")).clone())?;
            (eqns, modelInfo_1) = buildLabels(eqns.clone(), modelInfo.clone(), reduceList.clone(), Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { argNames: inNamedArgList.clone(), args: inExpArgList.clone() }))?;
            (eqns.clone(), modelInfo_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok((outEquationLst, outModelInfo))
}

fn meanValueReplacements(mut inVarLst: SimCodeVar::SimVars, mut exp_list: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<BackendVarTransform::VariableReplacements> {
    let mut outVarRepl: BackendVarTransform::VariableReplacements;
    outVarRepl = (::match_deref::match_deref! { match &((inVarLst.clone(), exp_list.clone())) {
        (SimCodeVar::SimVars { stateVars: states, boolAlgVars: boolAlg, intAlgVars: intAlg, algVars: alg, .. }, _) => {
            let mut listVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
            let mut listVars1: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
            let mut listVars2: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
            let mut repl: BackendVarTransform::VariableReplacements;
            repl = BackendVarTransform::emptyReplacements();
            listVars1 = listAppend(alg.clone(), intAlg.clone());
            listVars2 = listAppend(listVars1.clone(), boolAlg.clone());
            listVars = listAppend(listVars2.clone(), states.clone());
            repl = meanValueReplacements2(repl.clone(), listVars.clone(), exp_list.clone())?;
            repl.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVarRepl)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn meanValueReplacements2(mut inVarRepl: BackendVarTransform::VariableReplacements, mut inVarList: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut inValuesList: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<BackendVarTransform::VariableReplacements> {
    let mut outVarRepl: BackendVarTransform::VariableReplacements;
    outVarRepl = 'mc: {
        let __mc_input = (inVarRepl.clone(), inVarList.clone(), inValuesList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::REAL { value }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut repl = (*repl).clone();
                    repl = BackendVarTransform::addReplacement(repl.clone(), DAE::crefTime().clone(), Arc::new(DAE::Exp::RCONST { real: stringReal((value.clone()).clone())? }), None)?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("Add replacement for time\n")).clone())?;
                    }
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name, .. }, tail: restVar }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::REAL { value }, tail: restVal }) => {
                    let mut repl = (*repl).clone();
                    repl = BackendVarTransform::addReplacement(repl.clone(), name.clone(), Arc::new(DAE::Exp::RCONST { real: stringReal((value.clone()).clone())? }), None)?;
                    repl = meanValueReplacements2(repl.clone(), restVar.clone(), restVal.clone())?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add replacement for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(name.clone())?); __mm_s.push_str(&*literal!(" by ")); __mm_s.push_str(&*value.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name, .. }, tail: restVar }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::INTEGER { value: value2 }, tail: restVal }) => {
                    let mut value: ArcStr = arcstr::literal!("");
                    let mut repl = (*repl).clone();
                    value = (intString(value2.clone())).clone();
                    repl = BackendVarTransform::addReplacement(repl.clone(), name.clone(), Arc::new(DAE::Exp::RCONST { real: stringReal((value.clone()).clone())? }), None)?;
                    repl = meanValueReplacements2(repl.clone(), restVar.clone(), restVal.clone())?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add replacement for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(name.clone())?); __mm_s.push_str(&*literal!(" by ")); __mm_s.push_str(&*value.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name, .. }, tail: restVar }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::REAL { value }, op: Absyn::Operator::UMINUS }, tail: restVal }) => {
                    let mut repl = (*repl).clone();
                    repl = BackendVarTransform::addReplacement(repl.clone(), name.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, exp: Arc::new(DAE::Exp::RCONST { real: stringReal((value.clone()).clone())? }) }), None)?;
                    repl = meanValueReplacements2(repl.clone(), restVar.clone(), restVal.clone())?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add replacement for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(name.clone())?); __mm_s.push_str(&*literal!(" by -")); __mm_s.push_str(&*value.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name, .. }, tail: restVar }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::INTEGER { value: value2 }, op: Absyn::Operator::UMINUS }, tail: restVal }) => {
                    let mut value: ArcStr = arcstr::literal!("");
                    let mut repl = (*repl).clone();
                    value = (intString(value2.clone())).clone();
                    repl = BackendVarTransform::addReplacement(repl.clone(), name.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, exp: Arc::new(DAE::Exp::RCONST { real: stringReal((value.clone()).clone())? }) }), None)?;
                    repl = meanValueReplacements2(repl.clone(), restVar.clone(), restVal.clone())?;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add replacement for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(name.clone())?); __mm_s.push_str(&*literal!(" by -")); __mm_s.push_str(&*value.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("Add no replacement\n")).clone())?;
                    }
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (repl, Deref @ metamodelica::List::Cons { head: _, tail: restVar }, Deref @ metamodelica::List::Cons { head: _, tail: restVal }) => {
                    let mut repl = (*repl).clone();
                    repl = meanValueReplacements2(repl.clone(), restVar.clone(), restVal.clone())?;
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarRepl)
}

fn addLabelToEquations(mut inEquationLst1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut inVarLst: SimCodeVar::SimVars, mut inIndex: (i32, i32), mut reduceList: Arc<metamodelica::List<i32>>, mut inVarRepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>)> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outEquationLst, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (inEquationLst1.clone(), inVarLst.clone(), inIndex.clone(), reduceList.clone(), inVarRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, vars, idx, _, _) => {
                    Ok((metamodelica::nil(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: i, res_index: res_i, exp: e, source, eqAttr }, tail: es }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace residuals\n")).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExp(e.clone(), vars.clone(), idx.clone(), true, reduceList.clone(), inVarRepl.clone())?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(es.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(SimCode::SimEqSystem::SES_RESIDUAL { index: i.clone(), res_index: res_i.clone(), exp: e2.clone(), source: source.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: i, cref: cr, exp: e, source, eqAttr }, tail: es }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace simple assignments\n")).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExp(e.clone(), vars.clone(), idx.clone(), true, reduceList.clone(), inVarRepl.clone())?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(es.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: i.clone(), cref: cr.clone(), exp: e2.clone(), source: source.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: i, statements, eqAttr }, tail: es }, vars, idx, _, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut statements2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace algorithms\n")).clone())?;
                    }
                    (statements2, vars_1, idx2, labels) = addLabelToAlgorithms(statements.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(es.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(SimCode::SimEqSystem::SES_ALGORITHM { index: i.clone(), statements: statements2.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: i, partOfMixed: partOfLinear, tornSystem, vars: varsLin, beqs: b, simJac: A, residual, jacobianMatrix, sources: sourcelist, indexLinearSystem: idxLS, nUnknowns: nUnknownsLS, partOfJac }, alternativeTearing: None, eqAttr }, tail: es }, vars, idx, _, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut A2: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace linear equation systems\n")).clone())?;
                    }
                    (A2, vars_1, idx2, labels) = addLabelToLinearEquationSystems(A.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(es.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(SimCode::SimEqSystem::SES_LINEAR { lSystem: Arc::new(SimCode::LinearSystem { index: i.clone(), partOfMixed: partOfLinear.clone(), tornSystem: tornSystem.clone(), vars: varsLin.clone(), beqs: b.clone(), simJac: A2.clone(), residual: residual.clone(), jacobianMatrix: jacobianMatrix.clone(), sources: sourcelist.clone(), indexLinearSystem: idxLS.clone(), nUnknowns: nUnknownsLS.clone(), partOfJac: partOfJac.clone() }), alternativeTearing: None, eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { clockIndex, jacobianMatrix, nUnknowns: nUnknownsNLS, indexNonLinearSystem: idxNLS, crefs, eqs: nl, index: i, .. }, alternativeTearing: None, eqAttr }, tail: es }, vars, idx, _, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut nl_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace non-linear equation systems\n")).clone())?;
                    }
                    (nl_1, vars_1, idx2, labels) = addLabelToEquations(nl.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(es.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Arc::new(SimCode::NonlinearSystem { clockIndex: clockIndex.clone(), tornSystem: false, mixedSystem: false, homotopySupport: false, jacobianMatrix: jacobianMatrix.clone(), nUnknowns: nUnknownsNLS.clone(), indexNonLinearSystem: idxNLS.clone(), crefs: crefs.clone(), eqs: nl_1.clone(), index: i.clone() }), alternativeTearing: None, eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_MIXED { index: i, cont, discVars, discEqs: disc, indexMixedSystem: indexSys, eqAttr }, tail: es }, vars, idx, _, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut cont_1: Arc<SimCode::SimEqSystem>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace mixed equation systems\n")).clone())?;
                    }
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLabelToEquations(list![cont.clone()], vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cont_1 = __pa0.clone();
                    vars_1 = __pa1.clone();
                    idx2 = __pa2.clone();
                    labels = __pa3.clone();
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(es.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(SimCode::SimEqSystem::SES_MIXED { index: i.clone(), cont: cont_1.clone(), discVars: discVars.clone(), discEqs: disc.clone(), indexMixedSystem: indexSys.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_WHEN { index: i, conditions, initialCall, whenStmtLst, elseWhen: None, source, eqAttr }, tail: es }, vars, idx, _, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace when equations without else statement\n")).clone())?;
                    }
                    (es_1, vars_1, idx2, labels) = addLabelToEquations(es.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((cons(Arc::new(SimCode::SimEqSystem::SES_WHEN { index: i.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), whenStmtLst: whenStmtLst.clone(), elseWhen: None, source: source.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_WHEN { index: i, conditions, initialCall, whenStmtLst, elseWhen: Some(elsePart), source, eqAttr }, tail: es }, vars, idx, _, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut elsePart = (*elsePart).clone();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace when equations with else statement\n")).clone())?;
                    }
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLabelToEquations(list![elsePart.clone()], vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    elsePart = __pa0.clone();
                    vars_1 = __pa1.clone();
                    idx2 = __pa2.clone();
                    labels = __pa3.clone();
                    (es_1, vars_2, idx3, labels2) = addLabelToEquations(es.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(SimCode::SimEqSystem::SES_WHEN { index: i.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), whenStmtLst: whenStmtLst.clone(), elseWhen: Some(elsePart.clone()), source: source.clone(), eqAttr: eqAttr.clone() }), es_1.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq, tail: es }, vars, idx, _, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut idx2: (i32, i32);
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace unknown equations\n")).clone())?;
                    }
                    (es_1, vars_1, idx2, labels) = addLabelToEquations(es.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((cons(eq.clone(), es_1.clone()), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquationLst, outVarLst, outIndex, outStringList))
}

fn addLabelToAlgorithms(mut inStatements: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inVarLst: SimCodeVar::SimVars, mut inIndex: (i32, i32), mut reduceList: Arc<metamodelica::List<i32>>, mut inVarRepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>)> {
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outStatements, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (inStatements.clone(), inVarLst.clone(), inIndex.clone(), reduceList.clone(), inVarRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, vars, idx, _, _) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace empty algorithm\n")).clone())?;
                    }
                    Ok((metamodelica::nil(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { type_: ty, exp1: e1, exp: e, source }, tail: rest }, vars, idx, _, _) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut e2: Arc<DAE::Exp>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace assignment algorithm\n")).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExp(e.clone(), vars.clone(), idx.clone(), true, reduceList.clone(), inVarRepl.clone())?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(rest.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: ty.clone(), exp1: e1.clone(), exp: e2.clone(), source: source.clone() }), rest2.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { exp: e, statementLst: stmtLst, else_, source }, tail: rest }, vars, idx, _, _) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtLst2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace if algorithm\n")).clone())?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(stmtLst.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(rest.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: stmtLst2.clone(), else_: else_.clone(), source: source.clone() }), rest2.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_FOR { type_: ty, iterIsArray, iter, range: e, statementLst: stmtLst, source }, tail: rest }, vars, idx, _, _) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtLst2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace for algorithm\n")).clone())?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(stmtLst.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(rest.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(DAE::Statement::STMT_FOR { type_: ty.clone(), iterIsArray: iterIsArray.clone(), iter: (iter.clone()).clone(), range: e.clone(), statementLst: stmtLst2.clone(), source: source.clone() }), rest2.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHILE { exp: e, statementLst: stmtLst, source }, tail: rest }, vars, idx, _, _) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtLst2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace while algorithm\n")).clone())?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(stmtLst.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(rest.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(DAE::Statement::STMT_WHILE { exp: e.clone(), statementLst: stmtLst2.clone(), source: source.clone() }), rest2.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmtLst, elseWhen: None, source }, tail: rest }, vars, idx, _, _) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut rest2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtLst2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace when algorithm without else statement\n")).clone())?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(stmtLst.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (rest2, vars_2, idx3, labels2) = addLabelToAlgorithms(rest.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmtLst2.clone(), elseWhen: None, source: source.clone() }), rest2.clone()), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmtLst, elseWhen: Some(elseWhen), source }, tail: rest }, vars, idx, _, _) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut rest2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtLst2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels5: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut elseWhen2: Arc<DAE::Statement>;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace when algorithm with else statement\n")).clone())?;
                    }
                    (stmtLst2, vars_1, idx2, labels) = addLabelToAlgorithms(stmtLst.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLabelToAlgorithms(list![elseWhen.clone()], vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    elseWhen2 = __pa0.clone();
                    vars_2 = __pa1.clone();
                    idx3 = __pa2.clone();
                    labels2 = __pa3.clone();
                    (rest2, vars_3, idx4, labels3) = addLabelToAlgorithms(rest.clone(), vars_2.clone(), idx3.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmtLst2.clone(), elseWhen: Some(elseWhen2.clone()), source: source.clone() }), rest2.clone()), vars_3.clone(), idx4.clone(), labels5.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: stmt, tail: rest }, vars, idx, _, _) => {
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut rest2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("---Replace other algorithm\n")).clone())?;
                    }
                    (rest2, vars_1, idx2, labels) = addLabelToAlgorithms(rest.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((cons(stmt.clone(), rest2.clone()), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStatements, outVarLst, outIndex, outStringList))
}

/* Fatima
protected function addLabelToElse
"helper function for labeling else part"
  input  DAE.Else inElse;
  input  SimCodeVar.SimVars inVarLst;
  input  tuple<Integer,Integer> inIndex;
  input  list <Integer> reduceList;
  input  BackendVarTransform.VariableReplacements inVarRepl;
  output DAE.Else outElse;
  output SimCodeVar.SimVars outVarLst;
  output tuple<Integer,Integer> outIndex;
  output list<String> outStringList;
  algorithm
  (outElse,outVarLst,outIndex,outStringList) := matchcontinue (inElse,inVarLst,inIndex,reduceList,inVarRepl)
    local
      SimCodeVar.SimVars vars,vars_1,vars_2,vars_3;
      tuple <Integer,Integer> idx,idx2,idx3,idx4;
      SimCode.SimEqSystem el,el2;
      list<DAE.Statement> rest,rest2,stmtLst,stmtLst2;
      list<String> labels,labels2,labels3,labels4,labels5;
      DAE.Else else_,else2;
      DAE.Exp e;
    case(DAE.NOELSE(),vars,idx,reduceList,inVarRepl) then (DAE.NOELSE(),vars,idx,{});
    case(DAE.ELSEIF(e,stmtLst,else_),vars,idx,reduceList,inVarRepl)
      algorithm
        ////Debug.fcall(Flags.CPP,print,"---Replace elseif with else\n" );
        (stmtLst2,vars_1,idx2,labels) = addLabelToAlgorithms(stmtLst,vars,idx,reduceList,inVarRepl);
        (else2,vars_2,idx3,labels2) = addLabelToElse(else_,vars_1,idx2,reduceList,inVarRepl);
        labels3=listAppend(labels,labels2);
      then
        (DAE.ELSEIF(e,stmtLst2,else2),vars_2,idx3,labels3);
    case(DAE.ELSE(stmtLst),vars,idx,reduceList,inVarRepl)
      algorithm
        //Debug.fcall(Flags.CPP,print,"---Replace else\n" );
        (stmtLst2,vars_1,idx2,labels) = addLabelToAlgorithms(stmtLst,vars,idx,reduceList,inVarRepl);
      then
        (DAE.ELSE(stmtLst2),vars_1,idx2,labels) ;
  end matchcontinue;
end addLabelToElse;
*/
fn addLabelToLinearEquationSystems(mut inLinear: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>, mut inVarLst: SimCodeVar::SimVars, mut inIndex: (i32, i32), mut reduceList: Arc<metamodelica::List<i32>>, mut inVarRepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>)> {
    let mut outLinear: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>> = metamodelica::nil();
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outLinear, outVarLst, outIndex, outStringList) = (::match_deref::match_deref! { match &((inLinear.clone(), inVarLst.clone(), inIndex.clone(), reduceList.clone(), inVarRepl.clone())) {
        (Deref @ metamodelica::List::Nil, vars, idx, _, _) => {
            (metamodelica::nil(), vars.clone(), idx.clone(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: (i, j, el), tail: rest }, vars, idx, _, _) => {
            let mut vars_1: SimCodeVar::SimVars;
            let mut vars_2: SimCodeVar::SimVars;
            let mut idx2: (i32, i32);
            let mut idx3: (i32, i32);
            let mut el2: Arc<SimCode::SimEqSystem>;
            let mut rest2: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>> = metamodelica::nil();
            let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLabelToEquations(list![el.clone()], vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            el2 = __pa0.clone();
            vars_1 = __pa1.clone();
            idx2 = __pa2.clone();
            labels = __pa3.clone();
            (rest2, vars_2, idx3, labels2) = addLabelToLinearEquationSystems(rest.clone(), vars_1.clone(), idx2.clone(), reduceList.clone(), inVarRepl.clone())?;
            labels3 = listAppend(labels.clone(), labels2.clone());
            (cons((i.clone(), j.clone(), el2.clone()), rest2.clone()), vars_2.clone(), idx3.clone(), labels3.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outLinear, outVarLst, outIndex, outStringList))
}

fn addLabelToExp(mut inExp1: Arc<DAE::Exp>, mut inVarLst: SimCodeVar::SimVars, mut inIntdex: (i32, i32), mut add: bool, mut reduceList: Arc<metamodelica::List<i32>>, mut inVarRepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIntdex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outExp, outVarLst, outIntdex, outStringList) = 'mc: {
        let __mc_input = (inExp1.clone(), inVarLst.clone(), inIntdex.clone(), add.clone(), reduceList.clone(), inVarRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut vars: SimCodeVar::SimVars;
                    let mut idx: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    ::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REDUCTION_METHOD.clone())?) {
                        Deref @ "deletion" => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (e, vars, idx, labels) = addLabelToExpForDeletion(inExp1.clone(), inVarLst.clone(), inIntdex.clone(), add.clone(), reduceList.clone())?;
                    Ok((e.clone(), vars.clone(), idx.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut vars: SimCodeVar::SimVars;
                    let mut idx: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    ::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REDUCTION_METHOD.clone())?) {
                        Deref @ "substitution" => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (e, vars, idx, labels, _) = addLabelToExpForSubstitution(inExp1.clone(), inVarLst.clone(), inIntdex.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e.clone(), vars.clone(), idx.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut vars: SimCodeVar::SimVars;
                    let mut idx: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    ::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REDUCTION_METHOD.clone())?) {
                        Deref @ "linearization" => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (e, vars, idx, labels) = addLabelToExpForLinearization(inExp1.clone(), inVarLst.clone(), inIntdex.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e.clone(), vars.clone(), idx.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVarLst, outIntdex, outStringList))
}

fn addLabelToExpForDeletion(mut inExp1: Arc<DAE::Exp>, mut inVarLst: SimCodeVar::SimVars, mut inIntdex: (i32, i32), mut add: bool, mut reduceList: Arc<metamodelica::List<i32>>) -> Result<(Arc<DAE::Exp>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIntdex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outExp, outVarLst, outIntdex, outStringList) = 'mc: {
        let __mc_input = (inExp1.clone(), inVarLst.clone(), inIntdex.clone(), add.clone(), reduceList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::ADD { .. }, exp1: e1 }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels5: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to add exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2.clone(), true, reduceList.clone())?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), false, idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    } else {
                        (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), add.clone(), idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    }
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4.clone(), labels5.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::SUB { .. }, exp1: e1 }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels5: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sub exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2.clone(), true, reduceList.clone())?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), false, idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    } else {
                        (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), add.clone(), idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    }
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4.clone(), labels5.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::MUL { .. }, exp1: e1 }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels5: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to mul exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), false, reduceList.clone())?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2.clone(), false, reduceList.clone())?;
                    (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), add.clone(), idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4.clone(), labels5.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::DIV { .. }, exp1: e1 }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to div exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    Ok((Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() }), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::POW { .. }, exp1: e1 }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels5: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to pow exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2.clone(), true, reduceList.clone())?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), false, idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    } else {
                        (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), add.clone(), idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    }
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4.clone(), labels5.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { exp: e1, operator: op }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to unary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    Ok((Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1_1.clone() }), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { .. }, vars, idx, _, _) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Not Implemented: Add label to relation ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 }, vars, idx, _, _) => {
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3_1: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to if exp")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e2.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e3_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e3.clone(), vars_1.clone(), idx2.clone(), true, reduceList.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() }), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("add no label to pre arguments\n")).clone())?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add.clone(), idx.clone(), vars.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars_1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("add no label to edge arguments\n")).clone())?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add.clone(), idx.clone(), vars.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars_1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("add no label to change arguments\n")).clone())?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add.clone(), idx.clone(), vars.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars_1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("add no label to sample arguments\n")).clone())?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add.clone(), idx.clone(), vars.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars_1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("add no label for no event arguments\n")).clone())?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add.clone(), idx.clone(), vars.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars_1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" } }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels5: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to max exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2.clone(), true, reduceList.clone())?;
                    (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }), expLst: list![e1_1.clone(), e2_1.clone()], attr: attr.clone() }), add.clone(), idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4.clone(), labels5.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" } }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut vars_3: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut idx4: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels5: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to min exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e2_1, vars_2, idx3, labels2) = addLabelToExpForDeletion(e2.clone(), vars_1.clone(), idx2.clone(), true, reduceList.clone())?;
                    (e3, vars_3, idx4, labels3) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("min")).clone() }), expLst: list![e1_1.clone(), e2_1.clone()], attr: attr.clone() }), add.clone(), idx3.clone(), vars_2.clone(), reduceList.clone())?;
                    labels4 = listAppend(labels.clone(), labels2.clone());
                    labels5 = listAppend(labels4.clone(), labels3.clone());
                    Ok((e3.clone(), vars_3.clone(), idx4.clone(), labels5.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to abs exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_2, idx3, labels2) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("abs")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), false, idx2.clone(), vars_1.clone(), reduceList.clone())?;
                    } else {
                        (e3, vars_2, idx3, labels2) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("abs")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), add.clone(), idx2.clone(), vars_1.clone(), reduceList.clone())?;
                    }
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sqrt exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    if Flags::getConfigBool(Flags::DISABLE_EXTRA_LABELING.clone())? {
                        (e3, vars_2, idx3, labels2) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sqrt")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), false, idx2.clone(), vars_1.clone(), reduceList.clone())?;
                    } else {
                        (e3, vars_2, idx3, labels2) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sqrt")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), add.clone(), idx2.clone(), vars_1.clone(), reduceList.clone())?;
                    }
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sin")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to cos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e3, vars_2, idx3, labels2) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("cos")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), add.clone(), idx2.clone(), vars_1.clone(), reduceList.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("asin")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to cos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e3, vars_2, idx3, labels2) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("acos")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), add.clone(), idx2.clone(), vars_1.clone(), reduceList.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" } }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to tan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("tan")).clone() }), expLst: list![e1_1.clone()], attr: attr.clone() }), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" } }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to atan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("atan")).clone() }), expLst: list![e1_1.clone()], attr: attr.clone() }), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut vars_2: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to exp exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    (e3, vars_2, idx3, labels2) = addOneLabel(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("exp")).clone() }), expLst: list![e2.clone()], attr: attr.clone() }), add.clone(), idx2.clone(), vars_1.clone(), reduceList.clone())?;
                    labels3 = listAppend(labels.clone(), labels2.clone());
                    Ok((e3.clone(), vars_2.clone(), idx3.clone(), labels3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" } }, vars, idx, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to div exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e1_1, vars_1, idx2, labels) = addLabelToExpForDeletion(e1.clone(), vars.clone(), idx.clone(), true, reduceList.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("div")).clone() }), expLst: list![e1_1.clone(), e2.clone()], attr: attr.clone() }), vars_1.clone(), idx2.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: expl, path }, vars, idx, _, _) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to other call function ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expl.clone(), attr: attr.clone() }), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: valueR }, vars, idx, _, _) => {
                    if !((valueR.clone() == metamodelica::OrderedFloat(0.0_f64))) { bail!("guard") }
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("Add no label to const 0.0\n")).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RCONST { real: _ }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to real const variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add.clone(), idx.clone(), vars.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars_1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: valueI }, vars, idx, _, _) => {
                    if !((valueI.clone() == 0)) { bail!("guard") }
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace((literal!("Add no label to const 0\n")).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::ICONST { integer: 0 }), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ICONST { integer: _ }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to integer const variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add.clone(), idx.clone(), vars.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars_1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::SCONST { string: _ }, vars, idx, _, _) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to string const variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BCONST { bool: _ }, vars, idx, _, _) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to boolean const variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, vars, idx, _, _) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to string variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, vars, idx, _, _) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add no label to boolean variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars_1, idx1, labels) = addOneLabel(e.clone(), add.clone(), idx.clone(), vars.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars_1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, vars, idx, _, _) => {
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to unknown expression ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVarLst, outIntdex, outStringList))
}

/*
protected function addLabelToExpList
"function that adds labels to expression lists"
  input list<Expression.Exp> inExpLst;
  input SimCodeVar.SimVars inVarLst;
  input tuple<Integer,Integer> inIndex;
  input list<Integer> reduceList;
  output list<Expression.Exp>  outExpLst;
  output SimCodeVar.SimVars outVarLst;
  output tuple<Integer,Integer> outIndex;
  output list<String> outStringList;
algorithm
  (outExpLst,outVarLst,outIndex,outStringList):=
  matchcontinue (inExpLst,inVarLst,inIndex,reduceList)
    local
      Expression.Exp e,e_1,e_2,e1;
      tuple<Integer,Integer> idx1,idx2,idx3;
      list<Expression.Exp> er,er2;
      SimCodeVar.SimVars vars,vars_1,vars_2;
      list<String> labels,labels2,labels3;
      BackendVarTransform.VariableReplacements repl;
    case ({},vars,idx1,reduceList) then ({},vars,idx1,{});
    case ((e1 :: er),vars,idx1,reduceList)
      algorithm
        repl=BackendVarTransform.emptyReplacements();
        (e_1,vars_1,idx2,labels) = addLabelToExp(e1,vars,idx1,true,reduceList,repl);
        (er2,vars_2,idx3,labels2) = addLabelToExpList(er, vars_1, idx2,reduceList);
        labels3=listAppend(labels,labels2);
      then
        (e_1::er2,vars_2,idx3,labels3);
  end matchcontinue;
end addLabelToExpList;
*/
fn addOneLabel(mut inExp1: Arc<DAE::Exp>, mut add: bool, mut inIndex: (i32, i32), mut inVarLst: SimCodeVar::SimVars, mut reduceList: Arc<metamodelica::List<i32>>) -> Result<(Arc<DAE::Exp>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outExp, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (inExp1.clone(), add.clone(), inIndex.clone(), inVarLst.clone(), reduceList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, true, (i, p), vars, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut i_1: i32 = 0;
                    let true = (Flags::getConfigBool(Flags::REDUCE_TERMS.clone())?) else { bail!("pattern mismatch") };
                    let _ = List::getMember(i.clone(), reduceList.clone())?;
                    e2 = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), e.clone())?;
                    i_1 = i.clone() + 1;
                    Ok((e2.clone(), vars.clone(), (i_1.clone(), p.clone()), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, true, (i, p), vars, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut i_1: i32 = 0;
                    let true = (Flags::getConfigBool(Flags::REDUCE_TERMS.clone())?) else { bail!("pattern mismatch") };
                    e2 = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), e.clone())?;
                    i_1 = i.clone() + 1;
                    Ok((e2.clone(), vars.clone(), (i_1.clone(), p.clone()), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, true, (i, p), vars, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut name1: ArcStr = arcstr::literal!("");
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut p_1: i32 = 0;
                    let mut i_1: i32 = 0;
                    (vars_1, name) = createLabelVar(vars.clone(), p.clone(), i.clone())?;
                    name1 = (stringAppend((name.clone()).clone(), (literal!("_1")).clone())).clone();
                    e2 = multiply(e.clone(), (name1.clone()).clone())?;
                    p_1 = p.clone() + 2;
                    i_1 = i.clone() + 1;
                    Ok((e2.clone(), vars_1.clone(), (i_1.clone(), p_1.clone()), list![(name.clone()).clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, false, (i, p), vars, _) => {
                    Ok((e.clone(), vars.clone(), (i.clone(), p.clone()), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVarLst, outIndex, outStringList))
}

fn addLabelToExpForLinearization(mut inExp1: Arc<DAE::Exp>, mut inVarLst: SimCodeVar::SimVars, mut inIndex: (i32, i32), mut reduceList: Arc<metamodelica::List<i32>>, mut inVarRepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outExp, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (inExp1.clone(), inVarLst.clone(), inIndex.clone(), reduceList.clone(), inVarRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::POW { ty: tp }, exp1: e1 }, vars, idx, _, _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e1.clone())?) else { bail!("pattern mismatch") };
                    let false = (Expression::expHasCrefs(e2.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to pow exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e3, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: e2.clone() }), vars1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::POW { ty: tp }, exp1: e1 }, vars, idx, _, _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut e6: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let false = (Expression::expHasCrefs(e1.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::expHasCrefs(e2.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to pow exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e3, vars1, idx1, labels) = addLabelToExpForLinearization(e2.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e4 = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: e3.clone() });
                    e5 = linearizeExp(e4.clone(), e3.clone(), vars.clone(), inVarRepl.clone())?;
                    (e6, vars2, idx2, labels1) = addTwoLabels(e4.clone(), e5.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e6.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 }, vars, idx, _, _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to binary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e3, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (e4, vars2, idx2, labels1) = addLabelToExpForLinearization(e2.clone(), vars1.clone(), idx1.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op.clone(), exp2: e4.clone() }), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { exp: e1, operator: op }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to unary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e2.clone() }), vars1.clone(), idx1.clone(), labels.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 }, vars, idx, _, _) => {
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to if exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e4, vars1, idx1, labels) = addLabelToExpForLinearization(e2.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addLabelToExpForLinearization(e3.clone(), vars1.clone(), idx1.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e4.clone(), expElse: e5.clone() }), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sin")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to cos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("cos")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to tan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("tan")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to asin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("asin")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to acos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("acos")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to atan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("atan")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to exp exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("exp")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to log exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("log")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" } }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sqrt exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addLabelToExpForLinearization(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    e3 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sqrt")).clone() }), expLst: list![e2.clone()], attr: attr.clone() });
                    e4 = linearizeExp(e3.clone(), e2.clone(), vars.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1) = addTwoLabels(e3.clone(), e4.clone(), true, vars1.clone(), idx1.clone(), reduceList.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((e5.clone(), vars2.clone(), idx2.clone(), labels2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, vars, idx, _, _) => {
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVarLst, outIndex, outStringList))
}

fn addTwoLabels(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut label: bool, mut inVarLst: SimCodeVar::SimVars, mut inIndex: (i32, i32), mut reduceList: Arc<metamodelica::List<i32>>) -> Result<(Arc<DAE::Exp>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outExp, outVarLst, outIndex, outStringList) = 'mc: {
        let __mc_input = (inExp1.clone(), inExp2.clone(), label.clone(), inVarLst.clone(), inIndex.clone(), reduceList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, true, vars, (i, p), _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut i_1: i32 = 0;
                    let true = (Flags::getConfigBool(Flags::REDUCE_TERMS.clone())?) else { bail!("pattern mismatch") };
                    let _ = List::getMember(i.clone(), reduceList.clone())?;
                    e3 = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), e1.clone())?;
                    e4 = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), e2.clone())?;
                    e5 = Expression::expAdd(e3.clone(), e4.clone())?;
                    i_1 = i.clone() + 1;
                    Ok((e5.clone(), vars.clone(), (i_1.clone(), p.clone()), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, true, vars, (i, p), _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut i_1: i32 = 0;
                    let true = (Flags::getConfigBool(Flags::REDUCE_TERMS.clone())?) else { bail!("pattern mismatch") };
                    e3 = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), e1.clone())?;
                    e4 = Expression::expMul(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), e2.clone())?;
                    e5 = Expression::expAdd(e3.clone(), e4.clone())?;
                    i_1 = i.clone() + 1;
                    Ok((e5.clone(), vars.clone(), (i_1.clone(), p.clone()), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, true, vars, (i, p), _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut i_1: i32 = 0;
                    let mut p_1: i32 = 0;
                    let mut vars_1: SimCodeVar::SimVars;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut name1: ArcStr = arcstr::literal!("");
                    let mut name2: ArcStr = arcstr::literal!("");
                    (vars_1, name) = createLabelVar(vars.clone(), p.clone(), i.clone())?;
                    name1 = (stringAppend((name.clone()).clone(), (literal!("_1")).clone())).clone();
                    name2 = (stringAppend((name.clone()).clone(), (literal!("_2")).clone())).clone();
                    e3 = multiply(e1.clone(), (name1.clone()).clone())?;
                    e4 = multiply(e2.clone(), (name2.clone()).clone())?;
                    e5 = Expression::expAdd(e3.clone(), e4.clone())?;
                    p_1 = p.clone() + 2;
                    i_1 = i.clone() + 1;
                    Ok((e5.clone(), vars_1.clone(), (i_1.clone(), p_1.clone()), list![(name.clone()).clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, _, false, vars, (i, p), _) => {
                    Ok((e1.clone(), vars.clone(), (i.clone(), p.clone()), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVarLst, outIndex, outStringList))
}

fn linearizeExp(mut inExp: Arc<DAE::Exp>, mut source: Arc<DAE::Exp>, mut inVarLst: SimCodeVar::SimVars, mut inVarRepl: BackendVarTransform::VariableReplacements) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), source.clone(), inVarLst.clone(), inVarRepl.clone())) {
        (e1, e2, _, repl) => {
            let mut e: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            let mut e4: Arc<DAE::Exp>;
            let mut e5: Arc<DAE::Exp>;
            let mut e6: Arc<DAE::Exp>;
            let mut tmpExp: Arc<DAE::Exp>;
            let mut replExp: Arc<DAE::Exp>;
            let mut tmp: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (replExp, _) = BackendVarTransform::replaceExp(e2.clone(), repl.clone(), None)?;
            (e, _) = Expression::replaceExp(e1.clone(), e2.clone(), replExp.clone())?;
            tmp = ComponentReferenceBasics::makeCrefIdent((literal!("linVar")).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
            tmpExp = Expression::crefExp(tmp.clone())?;
            (e3, _) = Expression::replaceExp(e1.clone(), e2.clone(), tmpExp.clone())?;
            e4 = Differentiate::differentiateExpSolve(e3.clone(), tmp.clone(), None)?;
            (e5, _) = Expression::replaceExp(e4.clone(), tmpExp.clone(), replExp.clone())?;
            e6 = Expression::expAdd(e.clone(), Expression::expMul(e5.clone(), Expression::expSub(e2.clone(), replExp.clone())?)?)?;
            e6.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn addLabelToExpForSubstitution(mut inExp1: Arc<DAE::Exp>, mut inVarLst: SimCodeVar::SimVars, mut inIndex: (i32, i32), mut reduceList: Arc<metamodelica::List<i32>>, mut inVarRepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, SimCodeVar::SimVars, (i32, i32), Arc<metamodelica::List<ArcStr>>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVarLst: SimCodeVar::SimVars;
    let mut outIndex: (i32, i32);
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut substitute: bool = false;
    (outExp, outVarLst, outIndex, outStringList, substitute) = 'mc: {
        let __mc_input = (inExp1.clone(), inVarLst.clone(), inIndex.clone(), reduceList.clone(), inVarRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 }, vars, idx, _, _) => {
                    let mut ex: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut vars3: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs1: bool = false;
                    let mut subs2: bool = false;
                    let mut subs3: bool = false;
                    let mut subs4: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ex = __pa0.clone();
                    (e3, vars1, idx1, labels, subs1) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (e4, vars2, idx2, labels1, subs2) = addLabelToExpForSubstitution(e2.clone(), vars1.clone(), idx1.clone(), reduceList.clone(), inVarRepl.clone())?;
                    subs3 = boolAnd(subs1.clone(), subs2.clone());
                    (e5, vars3, idx3, labels2) = addTwoLabels(Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op.clone(), exp2: e4.clone() }), ex.clone(), subs3.clone(), vars2.clone(), idx2.clone(), reduceList.clone())?;
                    subs4 = boolOr(subs1.clone(), subs2.clone());
                    labels3 = listAppend(labels.clone(), labels1.clone());
                    labels4 = listAppend(labels3.clone(), labels2.clone());
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to binary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((e5.clone(), vars3.clone(), idx3.clone(), labels4.clone(), subs4.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { exp: e1, operator: op }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to unary exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e2.clone() }), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 }, vars, idx, _, _) => {
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to if exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e4, vars1, idx1, labels, _) = addLabelToExpForSubstitution(e2.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (e5, vars2, idx2, labels1, _) = addLabelToExpForSubstitution(e3.clone(), vars1.clone(), idx1.clone(), reduceList.clone(), inVarRepl.clone())?;
                    labels2 = listAppend(labels.clone(), labels1.clone());
                    Ok((Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e4.clone(), expElse: e5.clone() }), vars2.clone(), idx2.clone(), labels2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" } }, vars, idx, _, _) => {
                    let mut ex: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut vars3: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs1: bool = false;
                    let mut subs2: bool = false;
                    let mut subs3: bool = false;
                    let mut subs4: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ex = __pa0.clone();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to max exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e3, vars1, idx1, labels, subs1) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (e4, vars2, idx2, labels1, subs2) = addLabelToExpForSubstitution(e2.clone(), vars1.clone(), idx1.clone(), reduceList.clone(), inVarRepl.clone())?;
                    subs3 = boolAnd(subs1.clone(), subs2.clone());
                    (e5, vars3, idx3, labels2) = addTwoLabels(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }), expLst: list![e3.clone(), e4.clone()], attr: attr.clone() }), ex.clone(), subs3.clone(), vars2.clone(), idx2.clone(), reduceList.clone())?;
                    subs4 = boolOr(subs1.clone(), subs2.clone());
                    labels3 = listAppend(labels.clone(), labels1.clone());
                    labels4 = listAppend(labels3.clone(), labels2.clone());
                    Ok((e5.clone(), vars3.clone(), idx3.clone(), labels4.clone(), subs4.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" } }, vars, idx, _, _) => {
                    let mut ex: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut vars3: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs1: bool = false;
                    let mut subs2: bool = false;
                    let mut subs3: bool = false;
                    let mut subs4: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ex = __pa0.clone();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to min exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e3, vars1, idx1, labels, subs1) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (e4, vars2, idx2, labels1, subs2) = addLabelToExpForSubstitution(e2.clone(), vars1.clone(), idx1.clone(), reduceList.clone(), inVarRepl.clone())?;
                    subs3 = boolAnd(subs1.clone(), subs2.clone());
                    (e5, vars3, idx3, labels2) = addTwoLabels(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("min")).clone() }), expLst: list![e3.clone(), e4.clone()], attr: attr.clone() }), ex.clone(), subs3.clone(), vars2.clone(), idx2.clone(), reduceList.clone())?;
                    subs4 = boolOr(subs1.clone(), subs2.clone());
                    labels3 = listAppend(labels.clone(), labels1.clone());
                    labels4 = listAppend(labels3.clone(), labels2.clone());
                    Ok((e5.clone(), vars3.clone(), idx3.clone(), labels4.clone(), subs4.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to abs exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sqrt exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to sin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to cos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to tan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to asin exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to acos exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, .. }, vars, idx, _, _) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (_, true) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to atan exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, vars, idx, _, _) => {
                    let mut ex: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ex = __pa0.clone();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to exp exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(ex.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels, subs) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" } }, vars, idx, _, _) => {
                    let mut ex: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut e5: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut vars2: SimCodeVar::SimVars;
                    let mut vars3: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut idx2: (i32, i32);
                    let mut idx3: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut labels4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subs1: bool = false;
                    let mut subs2: bool = false;
                    let mut subs3: bool = false;
                    let mut subs4: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ex = __pa0.clone();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to div exp ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e3, vars1, idx1, labels, subs1) = addLabelToExpForSubstitution(e1.clone(), vars.clone(), idx.clone(), reduceList.clone(), inVarRepl.clone())?;
                    (e4, vars2, idx2, labels1, subs2) = addLabelToExpForSubstitution(e2.clone(), vars1.clone(), idx1.clone(), reduceList.clone(), inVarRepl.clone())?;
                    subs3 = boolAnd(subs1.clone(), subs2.clone());
                    (e5, vars3, idx3, labels2) = addTwoLabels(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("div")).clone() }), expLst: list![e3.clone(), e4.clone()], attr: attr.clone() }), ex.clone(), subs3.clone(), vars2.clone(), idx2.clone(), reduceList.clone())?;
                    subs4 = boolOr(subs1.clone(), subs2.clone());
                    labels3 = listAppend(labels.clone(), labels1.clone());
                    labels4 = listAppend(labels3.clone(), labels2.clone());
                    Ok((e5.clone(), vars3.clone(), idx3.clone(), labels4.clone(), subs4.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: Deref @ DAE::Type::T_INTEGER { varLst: _ } }, vars, idx, _, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to integer variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addTwoLabels(e.clone(), e1.clone(), true, vars.clone(), idx.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: _, ty: Deref @ DAE::Type::T_REAL { varLst: _ } }, vars, idx, _, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars1: SimCodeVar::SimVars;
                    let mut idx1: (i32, i32);
                    let mut labels: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(substituteExp(e.clone(), inVarRepl.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Add label to real variable ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (e2, vars1, idx1, labels) = addTwoLabels(e.clone(), e1.clone(), true, vars.clone(), idx.clone(), reduceList.clone())?;
                    Ok((e2.clone(), vars1.clone(), idx1.clone(), labels.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, vars, idx, _, _) => {
                    Ok((e.clone(), vars.clone(), idx.clone(), metamodelica::nil(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVarLst, outIndex, outStringList, substitute))
}

/*
protected function addLabelToExpListForSubstitution
"function that adds labels to expressions for substitution"
  input list<Expression.Exp> inExpLst;
  input SimCodeVar.SimVars inVarLst;
  input tuple<Integer,Integer> inIndex;
  input list<Integer> reduceList;
  input BackendVarTransform.VariableReplacements inVarRepl;
  output list<Expression.Exp> outExpLst;
  output SimCodeVar.SimVars outVarLst;
  output tuple<Integer,Integer> outIndex;
  output list<String> outStringList;
algorithm
  (outExpLst,outVarLst,outIndex,outStringList):=matchcontinue(inExpLst,inVarLst,inIndex,reduceList,inVarRepl)
    local
      Expression.Exp e,e_1,e_2,e1;
      tuple<Integer,Integer> idx1,idx2,idx3;
      list<Expression.Exp> er,er2;
      SimCodeVar.SimVars vars,vars_1,vars_2;
      list<String> labels,labels2,labels3;
      BackendVarTransform.VariableReplacements repl;
    case ({},vars,idx1,reduceList,repl) then ({},vars,idx1,{});
    case ((e1 :: er),vars,idx1,reduceList,repl)
      algorithm
        (e_1,vars_1,idx2,labels) = addLabelToExpForSubstitution(e1,vars,idx1,reduceList,repl);
        (er2,vars_2,idx3,labels2) = addLabelToExpListForSubstitution(er, vars_1, idx2,reduceList,repl);
        labels3=listAppend(labels,labels2);
      then
        (e_1::er2,vars_2,idx3,labels3);
  end matchcontinue;

end addLabelToExpListForSubstitution;
*/
fn substituteExp(mut inExp: Arc<DAE::Exp>, mut inVarRepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut replPerformed: bool = false;
    (outExp, replPerformed) = (::match_deref::match_deref! { match &((inExp.clone(), inVarRepl.clone())) {
        (e, repl) => {
            let mut e1: Arc<DAE::Exp>;
            (e1, replPerformed) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?;
            (e1.clone(), replPerformed.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, replPerformed))
}

fn multiply(mut inExp: Arc<DAE::Exp>, mut inString: ArcStr) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), inString.clone())) {
        (e, name) => {
            let mut e2: Arc<DAE::Exp>;
            e2 = Expression::expMul(Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() }), e.clone())?;
            if Flags::isSet(Flags::REDUCE_DAE.clone())? {
                Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("generate label  ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(" for term ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            e2.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn createLabelVar(mut inVariables: SimCodeVar::SimVars, mut inInteger: i32, mut inInteger2: i32) -> Result<(SimCodeVar::SimVars, ArcStr)> {
    let mut outVariables: SimCodeVar::SimVars;
    let mut outString: ArcStr = arcstr::literal!("");
    (outVariables, outString) = (match (inVariables.clone(), inInteger.clone(), inInteger2.clone()) {
        (SimCodeVar::SimVars { stateVars: ref states, derivativeVars: ref derVar, algVars: ref alg, discreteAlgVars: ref disAlg, intAlgVars: ref intAlg, boolAlgVars: ref boolAlg, inputVars: ref inVar, outputVars: ref outVar, aliasVars: ref algAlias, intAliasVars: ref intAlias, boolAliasVars: ref boolAlias, paramVars: ref param, intParamVars: ref intParam, boolParamVars: ref boolParam, stringAlgVars: ref stringAlg, stringParamVars: ref stringParam, stringAliasVars: ref stringAlias, extObjVars: ref extObjVar, constVars: ref r#const, intConstVars: ref intConst, boolConstVars: ref boolConst, stringConstVars: ref stringConst, jacobianVars: ref jacobianVar, seedVars: ref seedVar, realOptimizeConstraintsVars: ref realOptConst, realOptimizeFinalConstraintsVars: ref realOptFinalConst, sensitivityVars: ref sensVar, dataReconSetcVars: ref setcVar, dataReconinputVars: ref datareconinputvar, dataReconSetBVars: ref setBVar }, mut p, mut i) => {
            let mut simVar_1: SimCodeVar::SimVar;
            let mut simVar_2: SimCodeVar::SimVar;
            let mut param_1: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
            let mut param_2: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
            let mut name: ArcStr = arcstr::literal!("");
            let mut name1: ArcStr = arcstr::literal!("");
            let mut name2: ArcStr = arcstr::literal!("");
            let mut indexStr: ArcStr = arcstr::literal!("");
            let mut param = param.clone();
            indexStr = (intString(i.clone())).clone();
            name = (stringAppend((arcstr::literal!(LABELNAME)).clone(), (indexStr.clone()).clone())).clone();
            name1 = (stringAppend((name.clone()).clone(), (literal!("_1")).clone())).clone();
            name2 = (stringAppend((name.clone()).clone(), (literal!("_2")).clone())).clone();
            simVar_1 = SimCodeVar::SimVar { name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name1.clone()).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: crate::BackendDAE::VarKind::PARAM, comment: (literal!("")).clone(), unit: (literal!("")).clone(), displayUnit: (literal!("")).clone(), index: p.clone(), minValue: None, maxValue: None, initialValue: Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })), nominalValue: None, isFixed: true, type_: DAE::T_REAL_DEFAULT().clone(), isDiscrete: false, arrayCref: None, aliasvar: crate::SimCodeVar::AliasVariable::NOALIAS, source: DAE::emptyElementSource().clone(), causality: Some(crate::SimCodeVar::Causality::LOCAL), variable_index: None, fmi_index: None, numArrayElement: metamodelica::nil(), isValueChangeable: false, isProtected: false, hideResult: None, isEncrypted: false, inputIndex: None, initNonlinear: false, matrixName: None, variability: None, initial_: None, exportVar: None, relativeQuantity: false };
            param = param.clone().reverse();
            param_1 = cons(simVar_1.clone(), param.clone());
            p = p.clone() + 1;
            simVar_2 = SimCodeVar::SimVar { name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name2.clone()).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: crate::BackendDAE::VarKind::PARAM, comment: (literal!("")).clone(), unit: (literal!("")).clone(), displayUnit: (literal!("")).clone(), index: p.clone(), minValue: None, maxValue: None, initialValue: Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })), nominalValue: None, isFixed: true, type_: DAE::T_REAL_DEFAULT().clone(), isDiscrete: false, arrayCref: None, aliasvar: crate::SimCodeVar::AliasVariable::NOALIAS, source: DAE::emptyElementSource().clone(), causality: Some(crate::SimCodeVar::Causality::LOCAL), variable_index: None, fmi_index: None, numArrayElement: metamodelica::nil(), isValueChangeable: false, isProtected: false, hideResult: None, isEncrypted: false, inputIndex: None, initNonlinear: false, matrixName: None, variability: None, initial_: None, exportVar: None, relativeQuantity: false };
            param_2 = cons(simVar_2.clone(), param_1.clone());
            param_2 = param_2.clone().reverse();
            (SimCodeVar::SimVars { stateVars: states.clone(), derivativeVars: derVar.clone(), algVars: alg.clone(), discreteAlgVars: disAlg.clone(), intAlgVars: intAlg.clone(), boolAlgVars: boolAlg.clone(), inputVars: inVar.clone(), outputVars: outVar.clone(), aliasVars: algAlias.clone(), intAliasVars: intAlias.clone(), boolAliasVars: boolAlias.clone(), paramVars: param_2.clone(), intParamVars: intParam.clone(), boolParamVars: boolParam.clone(), stringAlgVars: stringAlg.clone(), stringParamVars: stringParam.clone(), stringAliasVars: stringAlias.clone(), extObjVars: extObjVar.clone(), constVars: r#const.clone(), intConstVars: intConst.clone(), boolConstVars: boolConst.clone(), stringConstVars: stringConst.clone(), jacobianVars: jacobianVar.clone(), seedVars: seedVar.clone(), realOptimizeConstraintsVars: realOptConst.clone(), realOptimizeFinalConstraintsVars: realOptFinalConst.clone(), sensitivityVars: sensVar.clone(), dataReconSetcVars: setcVar.clone(), dataReconinputVars: datareconinputvar.clone(), dataReconSetBVars: setBVar.clone() }, name.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outVariables, outString))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeReduceList(mut expLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inList: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &((expLst.clone(), inList.clone())) {
        (Deref @ metamodelica::List::Nil, lst) => {
            lst.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::INTEGER { value: v }, tail: expLstRest }, lst) => {
            let mut i: i32 = 0;
            let mut lst2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut lst3: Arc<metamodelica::List<i32>> = metamodelica::nil();
            i = v.clone();
            lst2 = listAppend(lst.clone(), list![i.clone()]);
            lst3 = makeReduceList(expLstRest.clone(), lst2.clone())?;
            lst3.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

fn StringDelimit2Int(mut inString: ArcStr, mut inDelim: ArcStr) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = (inString.clone(), inDelim.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut v, mut delim) = __mc_input.clone() else { bail!("nomatch") };
            let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut lst2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            lst = Util::stringSplitAtChar((v.clone()).clone(), (delim.clone()).clone())?;
            lst2 = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut s in (lst.clone()).into_iter().cloned() {
            let __x = stringInt((s.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Ok(lst2.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::nil())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

pub fn createBackendLabelVars(mut modelInfo: SimCode::ModelInfo) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut labelList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    labelList = (match modelInfo.clone() {
        SimCode::ModelInfo { labels: mut labels, varInfo: SimCode::VarInfo { numParams: mut numParams, .. }, .. } => {
            let mut list1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            list1 = createBackendLabelVars2(labels.clone(), numParams.clone())?;
            list1.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(labelList)
}

fn createBackendLabelVars2(mut inLabels: Arc<metamodelica::List<ArcStr>>, mut inIndex: i32) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &((inLabels.clone(), inIndex.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: name, tail: rest }, p) => {
            let mut name1: ArcStr = arcstr::literal!("");
            let mut name2: ArcStr = arcstr::literal!("");
            let mut var1: BackendDAE::Var;
            let mut var2: BackendDAE::Var;
            let mut list1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut list2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut list3: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut p = (*p).clone();
            name1 = (stringAppend((name.clone()).clone(), (literal!("_1")).clone())).clone();
            name2 = (stringAppend((name.clone()).clone(), (literal!("_2")).clone())).clone();
            var1 = BackendDAE::Var { varName: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name1.clone()).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: crate::BackendDAE::VarKind::PARAM, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })), arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None })), tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            p = p.clone() + 1;
            var2 = BackendDAE::Var { varName: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name2.clone()).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: crate::BackendDAE::VarKind::PARAM, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: Some(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })), arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None })), tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            list1 = list![var1.clone(), var2.clone()];
            p = p.clone() + 1;
            list2 = createBackendLabelVars2(rest.clone(), p.clone())?;
            list3 = listAppend(list1.clone(), list2.clone());
            list3.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

