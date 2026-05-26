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
use crate::SimCode;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference::writeCref;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::Expression;
use openmodelica_frontend_dump::ExpressionBasics::printExpStr as expStr;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::File::Escape::JSON;
use openmodelica_util::File;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn serializeParMod(mut code: SimCode::SimCode, mut withOperations: bool) -> Result<ArcStr> {
    let mut fileName: ArcStr = arcstr::literal!("");
    let (true, __pa0) = (serializeParModWork(code.clone(), withOperations.clone())?) else { bail!("pattern mismatch") };
    fileName = __pa0.clone();
    Ok(fileName)
}

fn serializeParModWork(mut code: SimCode::SimCode, mut withOperations: bool) -> Result<(bool, ArcStr)> {
    let mut success: bool = false;
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut file: File::File = File::File(File::noReference())?;
    let mut mi: SimCode::ModelInfo;
    let mut vars: SimCodeVar::SimVars;
    match '__try0: {
        let SimCode::SIMCODE { modelInfo: ref __pa2 @ SimCode::MODELINFO { vars: ref __pa1, .. }, .. } = (code.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        vars = __pa1.clone();
        mi = __pa2.clone();
        fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*code.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_ode.json")); ArcStr::from(__mm_s) }).clone();
        File::open(file.clone(), (fileName.clone()).clone(), File::Mode::Write.clone());
        File::write(file.clone(), (literal!("{\"format\":\"ParModelica task system info\",\"version\":1,\n\"info\":{\"name\":")).clone());
        unwrap_break_err!(serializePath(file.clone(), mi.name.clone()), '__try0);
        File::write(file.clone(), (literal!(",\"description\":\"")).clone());
        File::writeEscape(file.clone(), (mi.description.clone()).clone(), JSON.clone());
        File::write(file.clone(), (literal!("\"},\n\"ode-equations\":[")).clone());
        File::write(file.clone(), (literal!("{\"eqIndex\":0,\"tag\":\"dummy\"}")).clone());
        for mut eq in &*unwrap_break_err!(SimCodeUtil::sortEqSystems(List::flatten(code.odeEquations.clone())), '__try0) {
            let mut eq = eq.clone();
            unwrap_break_err!(serializeEquation(file.clone(), eq.clone(), (literal!("regular")).clone(), withOperations.clone(), 0, false, 0), '__try0);
        }
        File::write(file.clone(), (literal!("\n]\n}")).clone());
        success = true;
        Ok::<_, anyhow::Error>((fileName.clone(), mi.clone(), success.clone(), vars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            fileName = __try0_o0;
            mi = __try0_o1;
            success = __try0_o2;
            vars = __try0_o3;
        }
        Err(_) => {
            Error::addInternalError((literal!("SerializeTaskSystemInfo.serializeParModWork failed")).clone(), metamodelica::sourceInfo!())?;
            success = false;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((success, fileName))
}

fn serializeEquation(mut file: File::File, mut eq: Arc<SimCode::SimEqSystem>, mut section: ArcStr, mut withOperations: bool, mut parent: i32, mut first: bool, mut assign_type: i32) -> Result<bool> {
    let mut success: bool = false;
    if !(first.clone()) {
        File::write(file.clone(), (literal!(",")).clone());
    }
    success = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_RESIDUAL).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\",\"tag\":\"residual\",\"uses\":[")).clone());
            serializeUses(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_RESIDUAL).clone(), true)?)?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_RESIDUAL).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_RESIDUAL).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if assign_type.clone() == 1 {
                File::write(file.clone(), (literal!("\",\"tag\":\"torn\",\"defines\":[\"")).clone());
            } else if assign_type.clone() == 2 {
                File::write(file.clone(), (literal!("\",\"tag\":\"jacobian\",\"defines\":[\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"assign\",\"defines\":[\"")).clone());
            }
            writeCref(file.clone(), var_field!((*eq).cref, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeUses(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), true)?)?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if assign_type.clone() == 1 {
                File::write(file.clone(), (literal!("\",\"tag\":\"torn\",\"defines\":[\"")).clone());
            } else if assign_type.clone() == 2 {
                File::write(file.clone(), (literal!("\",\"tag\":\"jacobian\",\"defines\":[\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"assign\",\"defines\":[\"")).clone());
            }
            writeCref(file.clone(), var_field!((*eq).cref, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeUses(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone(), true)?)?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if assign_type.clone() == 1 {
                File::write(file.clone(), (literal!("\",\"tag\":\"torn\",\"defines\":[\"")).clone());
            } else if assign_type.clone() == 2 {
                File::write(file.clone(), (literal!("\",\"tag\":\"jacobian\",\"defines\":[\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"assign\",\"defines\":[\"")).clone());
            }
            writeCref(file.clone(), Expression::expCref(var_field!((*eq).lhs, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone())?, escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeUses(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone(), true)?)?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: None, lSystem: lSystem @ Deref @ SimCode::LinearSystem { .. }, .. } => {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut jeqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut constantEqns: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            i = (lSystem.beqs.clone().len() as i32);
            j = (lSystem.simJac.clone().len() as i32);
            eqs = SimCodeUtil::sortEqSystems(lSystem.residual.clone())?;
            jeqs = (::match_deref::match_deref! { match &(lSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), lSystem.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if lSystem.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(lSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeUses(file.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (lSystem.vars.clone()).into_iter().cloned() {
            let __x = (match v.clone() {
        SimCodeVar::SimVar { .. } => v.name.clone(),
        _ => bail!("match: no arm matched"),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
            File::write(file.clone(), (literal!("],\"equation\":[{\"size\":")).clone());
            File::write(file.clone(), (intString(i.clone())).clone());
            if i.clone() != 0 {
                File::write(file.clone(), (literal!(",\"density\":")).clone());
                File::writeReal(file.clone(), metamodelica::OrderedFloat((j.clone() / (i.clone() * i.clone())) as f64), (literal!("%.2f")).clone());
            }
            File::write(file.clone(), (literal!(",\"A\":[")).clone());
            serializeList1(file.clone(), lSystem.simJac.clone(), withOperations.clone(), Arc::new(serializeLinearCell));
            File::write(file.clone(), (literal!("],\"b\":[")).clone());
            serializeList(file.clone(), lSystem.beqs.clone(), Arc::new(serializeExp));
            File::write(file.clone(), (literal!("]}]")).clone());
            File::write(file.clone(), (literal!(",\n\"internal-equations\":[")).clone());
            if !(eqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), true, if (lSystem.tornSystem.clone()) {1} else {0})?;
                for mut e in &*listRest(eqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), false, if (lSystem.tornSystem.clone()) {1} else {0})?;
                }
            }
            File::write(file.clone(), (literal!("\n]")).clone());
            File::write(file.clone(), (literal!(",\n\"jacobian-equations\":[")).clone());
            if !(jeqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), true, 2)?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), false, 2)?;
                }
            }
            File::write(file.clone(), (literal!("\n]}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: Some(atL @ Deref @ SimCode::LinearSystem { .. }), lSystem: lSystem @ Deref @ SimCode::LinearSystem { .. }, .. } => {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut jeqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut constantEqns: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            i = (lSystem.beqs.clone().len() as i32);
            j = (lSystem.simJac.clone().len() as i32);
            eqs = SimCodeUtil::sortEqSystems(lSystem.residual.clone())?;
            if !(eqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), true, if (lSystem.tornSystem.clone()) {1} else {0})?;
                for mut e in &*listRest(eqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), false, if (lSystem.tornSystem.clone()) {1} else {0})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(lSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), true, 2)?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), false, 2)?;
                }
            }
            if eqs.clone().is_empty() && jeqs.clone().is_empty() {
                File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            } else {
                File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            }
            File::writeInt(file.clone(), lSystem.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if lSystem.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem-dynamic\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system-dynamic\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(lSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeUses(file.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (lSystem.vars.clone()).into_iter().cloned() {
            let __x = (match v.clone() {
        SimCodeVar::SimVar { .. } => v.name.clone(),
        _ => bail!("match: no arm matched"),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
            File::write(file.clone(), (literal!("],\"equation\":[{\"size\":")).clone());
            File::write(file.clone(), (intString(i.clone())).clone());
            if i.clone() != 0 {
                File::write(file.clone(), (literal!(",\"density\":")).clone());
                File::writeReal(file.clone(), metamodelica::OrderedFloat((j.clone() / (i.clone() * i.clone())) as f64), (literal!("%.2f")).clone());
            }
            File::write(file.clone(), (literal!(",\"A\":[")).clone());
            serializeList1(file.clone(), lSystem.simJac.clone(), withOperations.clone(), Arc::new(serializeLinearCell));
            File::write(file.clone(), (literal!("],\"b\":[")).clone());
            serializeList(file.clone(), lSystem.beqs.clone(), Arc::new(serializeExp));
            File::write(file.clone(), (literal!("]}]},")).clone());
            i = (atL.beqs.clone().len() as i32);
            j = (atL.simJac.clone().len() as i32);
            eqs = SimCodeUtil::sortEqSystems(atL.residual.clone())?;
            if !(eqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), atL.index.clone(), true, if (atL.tornSystem.clone()) {1} else {0})?;
                for mut e in &*listRest(eqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), atL.index.clone(), false, if (atL.tornSystem.clone()) {1} else {0})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(atL.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), atL.index.clone(), true, 2)?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), atL.index.clone(), false, 2)?;
                }
            }
            if eqs.clone().is_empty() && jeqs.clone().is_empty() {
                File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            } else {
                File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            }
            File::writeInt(file.clone(), atL.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if atL.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(atL.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeUses(file.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (atL.vars.clone()).into_iter().cloned() {
            let __x = (match v.clone() {
        SimCodeVar::SimVar { .. } => v.name.clone(),
        _ => bail!("match: no arm matched"),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
            File::write(file.clone(), (literal!("],\"equation\":[{\"size\":")).clone());
            File::write(file.clone(), (intString(i.clone())).clone());
            if i.clone() != 0 {
                File::write(file.clone(), (literal!(",\"density\":")).clone());
                File::writeReal(file.clone(), metamodelica::OrderedFloat((j.clone() / (i.clone() * i.clone())) as f64), (literal!("%.2f")).clone());
            }
            File::write(file.clone(), (literal!(",\"A\":[")).clone());
            serializeList1(file.clone(), atL.simJac.clone(), withOperations.clone(), Arc::new(serializeLinearCell));
            File::write(file.clone(), (literal!("],\"b\":[")).clone());
            serializeList(file.clone(), atL.beqs.clone(), Arc::new(serializeExp));
            File::write(file.clone(), (literal!("]}]}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::List::Cons { head: stmt, tail: _ }, .. } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_ALGORITHM).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*section.clone()); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            (crefs, crefs2) = Expression::extractUniqueCrefsFromStatmentS(var_field!((*eq).statements, SimCode::SimEqSystem::SES_ALGORITHM).clone());
            serializeUses(file.clone(), crefs.clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            serializeUses(file.clone(), crefs2.clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeList(file.clone(), var_field!((*eq).statements, SimCode::SimEqSystem::SES_ALGORITHM).clone(), Arc::new(serializeStatement));
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), Algorithm::getStatementSource(stmt.clone())?, withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { statements: Deref @ metamodelica::List::Cons { head: stmt, tail: _ }, .. } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_INVERSE_ALGORITHM).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*section.clone()); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            (crefs, crefs2) = Expression::extractUniqueCrefsFromStatmentS(var_field!((*eq).statements, SimCode::SimEqSystem::SES_INVERSE_ALGORITHM).clone());
            serializeUses(file.clone(), crefs.clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            serializeUses(file.clone(), crefs2.clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeList(file.clone(), var_field!((*eq).statements, SimCode::SimEqSystem::SES_INVERSE_ALGORITHM).clone(), Arc::new(serializeStatement));
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), Algorithm::getStatementSource(stmt.clone())?, withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: None, nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { .. }, .. } => {
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut jeqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut constantEqns: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            eqs = SimCodeUtil::sortEqSystems(nlSystem.eqs.clone())?;
            jeqs = (::match_deref::match_deref! { match &(nlSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), nlSystem.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if nlSystem.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(nlSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeUses(file.clone(), nlSystem.crefs.clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[[")).clone());
            serializeList(file.clone(), eqs.clone(), Arc::new(serializeEquationIndex));
            File::write(file.clone(), (literal!("],[")).clone());
            serializeList(file.clone(), jeqs.clone(), Arc::new(serializeEquationIndex));
            File::write(file.clone(), (literal!("]]")).clone());
            File::write(file.clone(), (literal!(",\n\"internal-equations\":[")).clone());
            if !(eqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), true, if (nlSystem.tornSystem.clone()) {1} else {0})?;
                for mut e in &*listRest(eqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), false, if (nlSystem.tornSystem.clone()) {1} else {0})?;
                }
            }
            File::write(file.clone(), (literal!("\n]")).clone());
            File::write(file.clone(), (literal!(",\n\"jacobian-equations\":[")).clone());
            if !(jeqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), true, 2)?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), false, 2)?;
                }
            }
            File::write(file.clone(), (literal!("\n]}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: Some(atNL @ Deref @ SimCode::NonlinearSystem { .. }), nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { .. }, .. } => {
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut jeqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut constantEqns: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            eqs = SimCodeUtil::sortEqSystems(nlSystem.eqs.clone())?;
            serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), true, if (nlSystem.tornSystem.clone()) {1} else {0})?;
            for mut e in &*listRest(eqs.clone())? {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), false, if (nlSystem.tornSystem.clone()) {1} else {0})?;
            }
            jeqs = (::match_deref::match_deref! { match &(nlSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), true, 2)?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), false, 2)?;
                }
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), nlSystem.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if nlSystem.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(nlSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeUses(file.clone(), nlSystem.crefs.clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[[")).clone());
            serializeList(file.clone(), eqs.clone(), Arc::new(serializeEquationIndex));
            File::write(file.clone(), (literal!("],[")).clone());
            serializeList(file.clone(), jeqs.clone(), Arc::new(serializeEquationIndex));
            File::write(file.clone(), (literal!("]]},")).clone());
            eqs = SimCodeUtil::sortEqSystems(atNL.eqs.clone())?;
            serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), atNL.index.clone(), true, if (atNL.tornSystem.clone()) {1} else {0})?;
            for mut e in &*listRest(eqs.clone())? {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), atNL.index.clone(), false, if (atNL.tornSystem.clone()) {1} else {0})?;
            }
            jeqs = (::match_deref::match_deref! { match &(atNL.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), atNL.index.clone(), true, 2)?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), atNL.index.clone(), false, 2)?;
                }
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), atNL.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if atNL.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(atNL.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeUses(file.clone(), atNL.crefs.clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[[")).clone());
            serializeList(file.clone(), eqs.clone(), Arc::new(serializeEquationIndex));
            File::write(file.clone(), (literal!("],[")).clone());
            serializeList(file.clone(), jeqs.clone(), Arc::new(serializeEquationIndex));
            File::write(file.clone(), (literal!("]]}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { .. } => {
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            eqs = listAppend(List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).ifbranches, SimCode::SimEqSystem::SES_IFEQUATION).clone()).into_iter().cloned() {
            let __x = Util::tuple22(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), var_field!((*eq).elsebranch, SimCode::SimEqSystem::SES_IFEQUATION).clone());
            serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), 0, true, 0)?;
            for mut e in &*listRest(eqs.clone())? {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), 0, false, 0)?;
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_IFEQUATION).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\",\"tag\":\"if-equation\",\"display\":\"if-equation\",\"equation\":[")).clone());
            serializeList(file.clone(), var_field!((*eq).ifbranches, SimCode::SimEqSystem::SES_IFEQUATION).clone(), Arc::new(serializeIfBranch));
            File::write(file.clone(), (literal!(",")).clone());
            serializeIfBranch(file.clone(), (Arc::new(DAE::Exp::BCONST { bool: true }), var_field!((*eq).elsebranch, SimCode::SimEqSystem::SES_IFEQUATION).clone()))?;
            File::write(file.clone(), (literal!("]}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_MIXED { .. } => {
            serializeEquation(file.clone(), var_field!((*eq).cont, SimCode::SimEqSystem::SES_MIXED).clone(), (section.clone()).clone(), withOperations.clone(), 0, true, 0)?;
            for mut e in &*var_field!((*eq).discEqs, SimCode::SimEqSystem::SES_MIXED).clone() {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), 0, false, 0)?;
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_MIXED).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\",\"tag\":\"container\",\"display\":\"mixed\",\"defines\":[")).clone());
            serializeUses(file.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (var_field!((*eq).discVars, SimCode::SimEqSystem::SES_MIXED).clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeEquationIndex(file.clone(), var_field!((*eq).cont, SimCode::SimEqSystem::SES_MIXED).clone())?;
            for mut e1 in &*var_field!((*eq).discEqs, SimCode::SimEqSystem::SES_MIXED).clone() {
                let mut e1 = e1.clone();
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquationIndex(file.clone(), e1.clone())?;
            }
            File::write(file.clone(), (literal!("]}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { .. } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_WHEN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            for mut whenOps in &*var_field!((*eq).whenStmtLst, SimCode::SimEqSystem::SES_WHEN).clone() {
                let mut whenOps = whenOps.clone();
                let _ = (match whenOps.clone() {
        mut whenOp @ BackendDAE::WhenOperator::ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\",\"defines\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.left, BackendDAE::WhenOperator::ASSIGN).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            serializeUses(file.clone(), List::union(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.right, BackendDAE::WhenOperator::ASSIGN).clone(), true)?))?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.right, BackendDAE::WhenOperator::ASSIGN).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        mut whenOp @ BackendDAE::WhenOperator::REINIT { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\",\"defines\":[")).clone());
            serializeCref(file.clone(), var_field!(whenOp.stateVar, BackendDAE::WhenOperator::REINIT).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            serializeUses(file.clone(), List::union(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.value, BackendDAE::WhenOperator::REINIT).clone(), true)?))?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.value, BackendDAE::WhenOperator::REINIT).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        mut whenOp @ BackendDAE::WhenOperator::ASSERT { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\"")).clone());
            File::write(file.clone(), (literal!(",\"uses\":[")).clone());
            crefs = listAppend(Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.condition, BackendDAE::WhenOperator::ASSERT).clone(), true)?, Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.message, BackendDAE::WhenOperator::ASSERT).clone(), true)?);
            serializeUses(file.clone(), List::union(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), crefs.clone()))?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::ASSERT).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        mut whenOp @ BackendDAE::WhenOperator::TERMINATE { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\"")).clone());
            File::write(file.clone(), (literal!(",\"uses\":[")).clone());
            serializeUses(file.clone(), List::union(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.message, BackendDAE::WhenOperator::TERMINATE).clone(), true)?))?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::TERMINATE).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        mut whenOp @ BackendDAE::WhenOperator::NORETCALL { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\"")).clone());
            File::write(file.clone(), (literal!(",\"uses\":[")).clone());
            serializeUses(file.clone(), List::union(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.exp, BackendDAE::WhenOperator::NORETCALL).clone(), true)?))?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.exp, BackendDAE::WhenOperator::NORETCALL).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
            }
            let _ = (::match_deref::match_deref! { match &(var_field!((*eq).elseWhen, SimCode::SimEqSystem::SES_WHEN).clone()) {
        Some(e) => {
            if SimCodeUtil::simEqSystemIndex(e.clone())? != 0 {
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), 0, false, 0)?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            true
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_FOR_LOOP).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if assign_type.clone() == 1 {
                File::write(file.clone(), (literal!("\",\"tag\":\"torn\",\"defines\":[\"")).clone());
            } else if assign_type.clone() == 2 {
                File::write(file.clone(), (literal!("\",\"tag\":\"jacobian\",\"defines\":[\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"assign\",\"defines\":[\"")).clone());
            }
            writeCref(file.clone(), var_field!((*eq).cref, SimCode::SimEqSystem::SES_FOR_LOOP).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeUses(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_FOR_LOOP).clone(), true)?)?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_FOR_LOOP).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_FOR_LOOP).clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            true
        },
        Deref @ SimCode::SimEqSystem::SES_ALIAS { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_ALIAS).clone(), (literal!("%d")).clone());
            File::write(file.clone(), (literal!(",\"tag\":\"alias\",\"equation\":[")).clone());
            File::writeInt(file.clone(), var_field!((*eq).aliasOf, SimCode::SimEqSystem::SES_ALIAS).clone(), (literal!("%d")).clone());
            File::write(file.clone(), (literal!("],\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\"}")).clone());
            true
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("serializeEquation failed: ")); __mm_s.push_str(&*anyString(eq.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(success)
}

fn serializeLinearCell(mut file: File::File, mut cell: (i32, i32, Arc<SimCode::SimEqSystem>), mut withOperations: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(cell.clone()) {
        (i, j, eq @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. }) => {
            File::write(file.clone(), (literal!("{\"row\":")).clone());
            File::write(file.clone(), (intString(i.clone())).clone());
            File::write(file.clone(), (literal!(",\"column\":")).clone());
            File::write(file.clone(), (intString(j.clone())).clone());
            File::write(file.clone(), (literal!(",\"exp\":\"")).clone());
            File::writeEscape(file.clone(), (expStr(eq.exp.clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\",\"source\":")).clone());
            serializeSource(file.clone(), eq.source.clone(), withOperations.clone());
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("SerializeTaskSystemInfo.serializeLinearCell failed. Expected only SES_RESIDUAL as input.")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn serializeUses(mut file: File::File, mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(crefs.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: cr, tail: Deref @ metamodelica::List::Nil } => {
            File::write(file.clone(), (literal!("\"")).clone());
            writeCref(file.clone(), cr.clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"")).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: cr, tail: rest } => {
            File::write(file.clone(), (literal!("\"")).clone());
            writeCref(file.clone(), cr.clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\",")).clone());
            serializeUses(file.clone(), rest.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn serializeStatement(mut file: File::File, mut stmt: Arc<DAE::Statement>) -> Result<()> {
    File::write(file.clone(), (literal!("\"")).clone());
    File::writeEscape(file.clone(), (System::trim((DAEDump::ppStatementStr(stmt.clone())?).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\"")).clone());
    Ok(())
}

fn serializeList<ArgType: Clone + 'static>(mut file: File::File, mut lst: Arc<metamodelica::List<ArgType>>, mut func: Arc<dyn ::std::ops::Fn(File::File, ArgType) -> Result<()> + 'static>) -> () {
    pub type FuncType<ArgType: Clone> = fn(File::File, ArgType) -> Result<()>;

    let _ = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: a, tail: Deref @ metamodelica::List::Nil } => {
            func(file.clone(), a.clone()).unwrap();
            ()
        },
        Deref @ metamodelica::List::Cons { head: a, tail: rest } => {
            func(file.clone(), a.clone()).unwrap();
            File::write(file.clone(), (literal!(",")).clone());
            serializeList(file.clone(), rest.clone(), func.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn serializeList1<ArgType: Clone + 'static, Extra: Clone + 'static>(mut file: File::File, mut lst: Arc<metamodelica::List<ArgType>>, mut extra: Extra, mut func: Arc<dyn ::std::ops::Fn(File::File, ArgType, Extra) -> Result<()> + 'static>) -> () {
    pub type FuncType<ArgType: Clone, Extra: Clone> = fn(File::File, ArgType, Extra) -> Result<()>;

    let _ = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: a, tail: Deref @ metamodelica::List::Nil } => {
            func(file.clone(), a.clone(), extra.clone()).unwrap();
            ()
        },
        Deref @ metamodelica::List::Cons { head: a, tail: rest } => {
            func(file.clone(), a.clone(), extra.clone()).unwrap();
            File::write(file.clone(), (literal!(",")).clone());
            serializeList1(file.clone(), rest.clone(), extra.clone(), func.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn serializeExp(mut file: File::File, mut exp: Arc<DAE::Exp>) -> Result<()> {
    File::write(file.clone(), (literal!("\"")).clone());
    File::writeEscape(file.clone(), (expStr(exp.clone())?).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\"")).clone());
    Ok(())
}

fn serializeCref(mut file: File::File, mut cr: Arc<DAE::ComponentRef>) -> Result<()> {
    File::write(file.clone(), (literal!("\"")).clone());
    writeCref(file.clone(), cr.clone(), escape=JSON.clone())?;
    File::write(file.clone(), (literal!("\"")).clone());
    Ok(())
}

fn serializeString(mut file: File::File, mut string: ArcStr) -> () {
    File::write(file.clone(), (literal!("\"")).clone());
    File::writeEscape(file.clone(), (string.clone()).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\"")).clone());
    ()
}

fn serializePath(mut file: File::File, mut path: Arc<Absyn::Path>) -> Result<()> {
    let mut p: Arc<Absyn::Path> = path.clone();
    let mut b: bool = true;
    File::write(file.clone(), (literal!("\"")).clone());
    while b.clone() {
        (p, b) = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            File::writeEscape(file.clone(), (var_field!((*p).name, Absyn::Path::IDENT).clone()).clone(), JSON.clone());
            (p.clone(), false)
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            File::writeEscape(file.clone(), (var_field!((*p).name, Absyn::Path::QUALIFIED).clone()).clone(), JSON.clone());
            File::write(file.clone(), (literal!(".")).clone());
            (var_field!((*p).path, Absyn::Path::QUALIFIED).clone(), true)
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => (var_field!((*p).path, Absyn::Path::FULLYQUALIFIED).clone(), true),
        _ => bail!("match: no arm matched"),
    } });
    }
    File::write(file.clone(), (literal!("\"")).clone());
    Ok(())
}

fn serializeEquationIndex(mut file: File::File, mut eq: Arc<SimCode::SimEqSystem>) -> Result<()> {
    File::writeInt(file.clone(), SimCodeUtil::simEqSystemIndex(eq.clone())?, (literal!("%d")).clone());
    Ok(())
}

fn serializeIfBranch(mut file: File::File, mut branch: (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)) -> Result<()> {
    let mut exp: Arc<DAE::Exp>;
    let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    (exp, eqs) = branch.clone();
    File::write(file.clone(), (literal!("[")).clone());
    serializeExp(file.clone(), exp.clone())?;
    File::write(file.clone(), (literal!(",")).clone());
    serializeList(file.clone(), eqs.clone(), Arc::new(serializeEquationIndex));
    File::write(file.clone(), (literal!("]")).clone());
    Ok(())
}

fn serializeSource(mut file: File::File, mut source: Arc<DAE::ElementSource>, mut withOperations: bool) -> () {
    let mut info: SourceInfo;
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut typeLst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut partOfLst: Arc<metamodelica::List<Absyn::Within>> = metamodelica::nil();
    let mut instance: Arc<DAE::ComponentPrefix> = Arc::new(DAE::ComponentPrefix::NOCOMPPRE);
    let mut i: i32 = 0;
    let mut operations: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    File::write(file.clone(), (literal!("{}")).clone());
    ()
}

