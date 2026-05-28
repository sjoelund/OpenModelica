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
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Sorting;
use crate::SymbolicJacobian;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// strongComponents and stuff
//
// =============================================================================
pub fn strongComponentsScalar(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>)> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    (outSystem, outComps) = 'mc: {
        let __mc_input = inSystem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2, ass1, .. }, mT: Some(mt), .. } => {
                    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
                    let mut markarray: metamodelica::Array<i32>;
                    let mut comps_m: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut syst = (*syst).clone();
                    let mut ass1 = (*ass1).clone();
                    comps_m = Sorting::TarjanTransposed(mt.clone(), ass2.clone())?;
                    markarray = arrayCreate(BackendEquation::getNumberOfEquations(inSystem.orderedEqs.clone()), -1);
                    comps = analyseStrongComponentsScalar(comps_m.clone(), inSystem.clone(), inShared.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), 1, markarray.clone())?;
                    GCExt::free(markarray.clone());
                    ass1 = varAssignmentNonScalar(ass1.clone(), mapIncRowEqn.clone());
                    syst = Arc::new(BackendDAE::EqSystem { orderedVars: syst.orderedVars.clone(), orderedEqs: syst.orderedEqs.clone(), m: None, mT: None, mapping: None, matching: Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() }), stateSets: syst.stateSets.clone(), partitionKind: syst.partitionKind.clone(), removedEqs: syst.removedEqs.clone() });
                    Ok((syst.clone(), comps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function strongComponentsScalar failed (sorting strong components)")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSystem, outComps))
}

pub fn eqnAssignmentNonScalar(mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass2: metamodelica::Array<i32>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outAcc: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let __range0 = 1..=(mapEqnIncRow.clone().borrow().len() as i32);
    for mut i in __range0 {
        elst = mapEqnIncRow.borrow()[(i.clone()-1) as usize].clone();
        vlst = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (elst.clone()).into_iter().cloned() {
            if !(ass2.clone().borrow()[(e.clone()-1) as usize].clone() > 0) { continue; }
            let __x = ass2.clone().borrow()[(e.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        acc = cons(vlst.clone(), acc.clone());
    }
    outAcc = List::listArrayReverse(acc.clone())?;
    Ok(outAcc)
}

pub fn varAssignmentNonScalar(mut ass1: metamodelica::Array<i32>, mut mapIncRowEqn: metamodelica::Array<i32>) -> metamodelica::Array<i32> {
    let mut outAcc: metamodelica::Array<i32>;
    outAcc = metamodelica::arrayCreate((ass1.clone().borrow().len() as i32), -1);
    let __range0 = 1..=(ass1.clone().borrow().len() as i32);
    for mut i in __range0 {
        Dangerous::arrayUpdate(outAcc.clone(), i.clone(), if (Dangerous::arrayGet(ass1.clone(), i.clone()).unwrap() > 0) {mapIncRowEqn.borrow()[(Dangerous::arrayGet(ass1.clone(), i.clone()).unwrap()-1) as usize].clone()} else {-1}).unwrap();
    }
    outAcc
}

fn analyseStrongComponentsScalar(mut inComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut inAss1: metamodelica::Array<i32>, mut inAss2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut imark: i32, mut markarray: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut acomp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut mark: i32 = imark.clone();
    for mut comp in &*inComps.clone() {
        let mut comp = comp.clone();
        (acomp, mark) = analyseStrongComponentScalar(comp.clone(), syst.clone(), shared.clone(), inAss1.clone(), inAss2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), mark.clone(), markarray.clone())?;
        outComps = listAppend(acomp.clone(), outComps.clone());
    }
    outComps = Dangerous::listReverseInPlace(outComps.clone());
    Ok(outComps)
}

fn analyseStrongComponentScalar(mut inComp: Arc<metamodelica::List<i32>>, mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut inAss1: metamodelica::Array<i32>, mut inAss2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut imark: i32, mut markarray: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, i32)> {
    let mut outComp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut omark: i32 = imark.clone() + 1;
    let mut comp: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        eqns = __pa1.clone();
        vars = __pa2.clone();
        vlst = List::map1r(inComp.clone(), Arc::new(arrayGet.clone()), inAss2.clone());
        vlst = List::select1(vlst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
        varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
        comp = List::map1r(inComp.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
        comp = List::fold2(comp.clone(), (std::sync::Arc::new(uniqueComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), imark.clone(), markarray.clone(), metamodelica::nil());
        eqn_lst = List::map1r(comp.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), eqns.clone());
        outComp = unwrap_break_err!(analyseStrongComponentBlock(comp.clone(), eqn_lst.clone(), varlst.clone(), vlst.clone(), syst.clone(), shared.clone(), mapEqnIncRow.clone()), '__try0);
        Ok::<_, anyhow::Error>((comp.clone(), eqn_lst.clone(), eqns.clone(), outComp.clone(), varlst.clone(), vars.clone(), vlst.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6)) => {
            comp = __try0_o0;
            eqn_lst = __try0_o1;
            eqns = __try0_o2;
            outComp = __try0_o3;
            varlst = __try0_o4;
            vars = __try0_o5;
            vlst = __try0_o6;
        }
        Err(_) => {
            Error::addInternalError((literal!("function analyseStrongComponentScalar failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok((outComp, omark))
}

fn uniqueComp(mut c: i32, mut mark: i32, mut markarray: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = iAcc.clone();
    if mark.clone() != markarray.borrow()[(c.clone()-1) as usize].clone() {
        {let _arr = markarray.clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = mark.clone(); _arr};
        oAcc = cons(c.clone(), iAcc.clone());
    }
    Ok(oAcc)
}

fn analyseStrongComponentBlock(mut inComp: Arc<metamodelica::List<i32>>, mut inEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inVarindxLst: Arc<metamodelica::List<i32>>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>> {
    let mut outComp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    outComp = 'mc: {
        let __mc_input = (inComp.clone(), inEqnLst.clone(), inVarLst.clone(), inVarindxLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compelem, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { .. }, tail: Deref @ metamodelica::List::Nil }, _, varindxs) => {
                    Ok(list![Arc::new(BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compelem, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, var_lst, varindxs) => {
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    crlst = List::map(var_lst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    b1 = List::applyAndFold(crlst.clone(), (std::sync::Arc::new(fnptr!(boolAnd, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(ComponentReference::isArrayElement, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), true);
                    if !(b1.clone()) {
                        expLst = List::map(crlst.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                        let true = (List::exist1(inEqnLst.clone(), (std::sync::Arc::new(crefsAreArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<bool> + 'static>), expLst.clone())) else { bail!("pattern mismatch") };
                    }
                    Ok(list![Arc::new(BackendDAE::StrongComponent::SINGLEARRAY { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compelem, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::IF_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, _, varindxs) => {
                    Ok(list![Arc::new(BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compelem, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, _, varindxs) => {
                    Ok(list![Arc::new(BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compelem, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, _, varindxs) => {
                    Ok(list![Arc::new(BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: compelem.clone(), vars: varindxs.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compelem, tail: Deref @ metamodelica::List::Nil }, _, _, Deref @ metamodelica::List::Cons { head: v, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(list![Arc::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: compelem.clone(), var: v.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comp, eqn_lst, var_lst, varindxs) => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut vars_1: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqn_lst1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut var_lst_1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut eqns_1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut jac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>> = None;
                    let mut jac_tp: BackendDAE::JacobianType = BackendDAE::JacobianType::JAC_CONSTANT;
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut jacConstant: bool = false;
                    let mut mixedSystem: bool = false;
                    let true = (BackendVariable::hasContinuousVar(var_lst.clone())) else { bail!("pattern mismatch") };
                    eqn_lst1 = BackendEquation::replaceDerOpInEquationList(eqn_lst.clone())?;
                    var_lst_1 = List::map(var_lst.clone(), (std::sync::Arc::new(fnptr!(transformXToXd, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<BackendDAE::Var> + 'static>));
                    vars_1 = BackendVariable::listVar1(var_lst_1.clone());
                    eqns_1 = BackendEquation::listEquation(eqn_lst1.clone())?;
                    (mixedSystem, _) = BackendEquation::iterationVarsinRelations(eqn_lst1.clone(), vars_1.clone())?;
                    if !(Flags::isSet(Flags::DISABLE_JACSCC.clone())?) {
                        syst = BackendDAEUtil::createEqSystem(vars_1.clone(), eqns_1.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                        (m, mt) = BackendDAEUtil::adjacencyMatrix(syst.clone(), crate::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                        (jac, shared) = SymbolicJacobian::calculateJacobian(vars_1.clone(), eqns_1.clone(), m.clone(), true, ishared.clone())?;
                        (jac_tp, jacConstant) = SymbolicJacobian::analyzeJacobian(vars_1.clone(), eqns_1.clone(), jac.clone())?;
                        if jacConstant.clone() && isSome(jac.clone()) {
                            let true = (analyzeConstantJacobian(Util::getOption(jac.clone())?, (mt.clone().borrow().len() as i32), var_lst.clone(), eqn_lst.clone(), shared.clone())?) else { bail!("pattern mismatch") };
                        }
                    } else {
                        jac = None;
                        jac_tp = crate::BackendDAE::JacobianType::JAC_NO_ANALYTIC;
                    }
                    Ok(list![Arc::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: comp.clone(), vars: varindxs.clone(), jac: Arc::new(BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: jac.clone() }), jacType: jac_tp.clone(), mixedSystem: mixedSystem.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comp, eqn_lst, var_lst, _) => {
                    if !((BackendEquation::allAlgorithmsLst(eqn_lst.clone()))) { bail!("guard") }
                    let mut ass2: metamodelica::Array<i32>;
                    let mut indxdisc_var: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut algorithmComp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
                    let true = (BackendVariable::hasDiscreteVar(var_lst.clone())) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::hasContinuousVar(var_lst.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(isyst.matching.clone()) {
                        Deref @ BackendDAE::Matching::MATCHING { ass1: _, ass2: __pa0, comps: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ass2 = __pa0.clone();
                    algorithmComp = metamodelica::nil();
                    for mut c in &*comp.clone() {
                        let mut c = c.clone();
                        indxdisc_var = metamodelica::nil();
                        let __range1 = &*mapEqnIncRow.borrow()[(c.clone()-1) as usize].clone();
                        for mut j in __range1 {
                            let mut j = j.clone();
                            indxdisc_var = cons(ass2.borrow()[(j.clone()-1) as usize].clone(), indxdisc_var.clone());
                        }
                        algorithmComp = cons(Arc::new(BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: c.clone(), vars: indxdisc_var.clone() }), algorithmComp.clone());
                    }
                    Ok(algorithmComp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, eqn_lst, var_lst, _) => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut slst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (BackendVariable::hasDiscreteVar(var_lst.clone())) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::hasContinuousVar(var_lst.clone())) else { bail!("pattern mismatch") };
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAETransform.analyseStrongComponentBlock")); __mm_s.push_str(&*literal!(" failed (Purely discrete algebraic loops cannot be solved by iterative processes. Try to break them open using the delay() operator.)\n")); ArcStr::from(__mm_s) }).clone();
                    crlst = List::map(var_lst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    slst = List::map(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>));
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*stringDelimitList(slst.clone(), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
                    slst = List::map(eqn_lst.clone(), (std::sync::Arc::new(BackendDump::equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>));
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(slst.clone(), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
                    Error::addInternalError((msg.clone()).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, eqn_lst, var_lst, _) => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut slst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAETransform.analyseStrongComponentBlock")); __mm_s.push_str(&*literal!(" failed\nvariables:\n  ")); ArcStr::from(__mm_s) }).clone();
                    crlst = List::map(var_lst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    slst = List::map(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>));
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*stringDelimitList(slst.clone(), (literal!("\n  ")).clone())); ArcStr::from(__mm_s) }).clone();
                    slst = List::map(eqn_lst.clone(), (std::sync::Arc::new(BackendDump::equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>));
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!("\nequations:\n  ")); __mm_s.push_str(&*stringDelimitList(slst.clone(), (literal!("\n  ")).clone())); ArcStr::from(__mm_s) }).clone();
                    Error::addInternalError((msg.clone()).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function analyseStrongComponentBlock failed")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComp)
}

fn crefsAreArray(mut eqIn: Arc<BackendDAE::Equation>, mut crefLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<bool> {
    let mut isUnsolvable: bool = false;
    isUnsolvable = 'mc: {
        let __mc_input = eqIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: Deref @ DAE::Exp::ARRAY { array: expLst, .. }, .. } => {
                    let mut expLst = (*expLst).clone();
                    (_, _, expLst) = List::intersection1OnTrue(expLst.clone(), crefLst.clone(), (std::sync::Arc::new(ExpressionBasics::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
                    Ok(expLst.clone().is_empty())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: Deref @ DAE::Exp::ARRAY { array: expLst, .. }, .. } => {
                    let mut expLst = (*expLst).clone();
                    (_, _, expLst) = List::intersection1OnTrue(expLst.clone(), crefLst.clone(), (std::sync::Arc::new(ExpressionBasics::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
                    Ok(expLst.clone().is_empty())
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
    Ok(isUnsolvable)
}

fn analyzeConstantJacobian(mut inJac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut inSize: i32, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<bool> {
    let mut outValid: bool = true;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut info: i32 = 0;
    let mut infoStr: ArcStr = arcstr::literal!("");
    let mut syst: ArcStr = arcstr::literal!("");
    let mut varnames: ArcStr = arcstr::literal!("");
    let mut varname: ArcStr = arcstr::literal!("");
    let mut rhsStr: ArcStr = arcstr::literal!("");
    let mut jacStr: ArcStr = arcstr::literal!("");
    let mut eqnstr: ArcStr = arcstr::literal!("");
    let mut beqs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rhsVals: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut jacVals: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    jacVals = SymbolicJacobian::evaluateConstantJacobian(inSize.clone(), inJac.clone());
    rhsVals = List::fill(metamodelica::OrderedFloat(0.0_f64), inSize.clone());
    (_, info) = System::dgesv(jacVals.clone(), rhsVals.clone())?;
    if info.clone() < 0 {
        varnames = stringDelimitList(List::mapMap(inVars.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!(" ;\n  ")).clone());
        eqns = BackendEquation::listEquation(inEqns.clone())?;
        vars = BackendVariable::listVar1(inVars.clone());
        funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
        (beqs, _) = BackendDAEUtil::getEqnSysRhs(eqns.clone(), vars.clone(), Some(funcs.clone()))?;
        beqs = beqs.clone().reverse();
        rhsStr = stringDelimitList(List::map(beqs.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(" ;\n  ")).clone());
        jacStr = stringDelimitList(List::map1(List::mapList(jacVals.clone(), (std::sync::Arc::new(fnptr!(realString, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<ArcStr> + 'static>)), (std::sync::Arc::new(fnptr!(stringDelimitList, Arc<metamodelica::List<ArcStr>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<ArcStr>>, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" , ")).clone()), (literal!(" ;\n  ")).clone());
        eqnstr = (BackendDump::dumpEqnsStr(inEqns.clone())?).clone();
        syst = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*eqnstr.clone()); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*jacStr.clone()); __mm_s.push_str(&*literal!("] * [")); __mm_s.push_str(&*varnames.clone()); __mm_s.push_str(&*literal!("] = [")); __mm_s.push_str(&*rhsStr.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        Error::addMessage(Error::LINEAR_SYSTEM_INVALID.clone(), list![(literal!("LAPACK/dgesv")).clone(), (syst.clone()).clone()])?;
        outValid = false;
    } else if info.clone() > 0 {
        varname = (ComponentReferenceBasics::printComponentRefStr(BackendVariable::varCref((inVars.clone()).get(info.clone())?)?)?).clone();
        infoStr = (intString(info.clone())).clone();
        varnames = stringDelimitList(List::mapMap(inVars.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!(" ;\n  ")).clone());
        eqns = BackendEquation::listEquation(inEqns.clone())?;
        vars = BackendVariable::listVar1(inVars.clone());
        funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
        (beqs, _) = BackendDAEUtil::getEqnSysRhs(eqns.clone(), vars.clone(), Some(funcs.clone()))?;
        beqs = beqs.clone().reverse();
        rhsStr = stringDelimitList(List::map(beqs.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(" ;\n  ")).clone());
        jacStr = stringDelimitList(List::map1(List::mapList(jacVals.clone(), (std::sync::Arc::new(fnptr!(realString, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<ArcStr> + 'static>)), (std::sync::Arc::new(fnptr!(stringDelimitList, Arc<metamodelica::List<ArcStr>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<ArcStr>>, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" , ")).clone()), (literal!(" ;\n  ")).clone());
        eqnstr = (BackendDump::dumpEqnsStr(inEqns.clone())?).clone();
        syst = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*eqnstr.clone()); __mm_s.push_str(&*literal!("\n[\n  ")); __mm_s.push_str(&*jacStr.clone()); __mm_s.push_str(&*literal!("\n]\n  *\n[\n  ")); __mm_s.push_str(&*varnames.clone()); __mm_s.push_str(&*literal!("\n]\n  =\n[\n  ")); __mm_s.push_str(&*rhsStr.clone()); __mm_s.push_str(&*literal!("\n]")); ArcStr::from(__mm_s) }).clone();
        Error::addMessage(Error::LINEAR_SYSTEM_SINGULAR.clone(), list![(syst.clone()).clone(), (infoStr.clone()).clone(), (varname.clone()).clone()])?;
    }
    Ok(outValid)
}

fn transformXToXd(mut inVar: BackendDAE::Var) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    if BackendVariable::isStateVar(inVar.clone()) {
        outVar.varName = ComponentReference::crefPrefixDer(inVar.varName.clone());
        outVar.varKind = crate::BackendDAE::VarKind::STATE_DER;
        outVar.unreplaceable = false;
    }
    outVar
}

pub fn getEquationAndSolvedVar(mut inComp: Arc<BackendDAE::StrongComponent>, mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVariables: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outEquation: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outVar: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outIndex: i32 = 0;
    (outEquation, outVar, outIndex) = (::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: v, eqn: e } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            eqn = BackendEquation::get(inEquationArray.clone(), e.clone());
            var = BackendVariable::getVarAt(inVariables.clone(), v.clone())?;
            (list![eqn.clone()], list![var.clone()], e.clone())
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: vlst, eqns: elst, .. } => {
            let mut e: i32 = 0;
            let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            eqnlst = BackendEquation::getList(elst.clone(), inEquationArray.clone());
            varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVariables.clone());
            e = listHead(elst.clone())?;
            (eqnlst.clone(), varlst.clone(), e.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: vlst, eqn: e } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            eqn = BackendEquation::get(inEquationArray.clone(), e.clone());
            varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVariables.clone());
            (list![eqn.clone()], varlst.clone(), e.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: vlst, eqn: e } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            eqn = BackendEquation::get(inEquationArray.clone(), e.clone());
            varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVariables.clone());
            (list![eqn.clone()], varlst.clone(), e.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: vlst, eqn: e } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            eqn = BackendEquation::get(inEquationArray.clone(), e.clone());
            varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVariables.clone());
            (list![eqn.clone()], varlst.clone(), e.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: vlst, eqn: e } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            eqn = BackendEquation::get(inEquationArray.clone(), e.clone());
            varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVariables.clone());
            (list![eqn.clone()], varlst.clone(), e.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: vlst, eqn: e } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            eqn = BackendEquation::get(inEquationArray.clone(), e.clone());
            varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVariables.clone());
            (list![eqn.clone()], varlst.clone(), e.clone())
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: elst, tearingvars: vlst, .. }, .. } => {
            let mut e: i32 = 0;
            let mut otherEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherVarsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnlst1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut varlst1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            eqnlst = BackendEquation::getList(elst.clone(), inEquationArray.clone());
            varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVariables.clone());
            (otherEqns, otherVarsLst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
            otherVars = List::flatten(otherVarsLst.clone());
            eqnlst1 = BackendEquation::getList(otherEqns.clone(), inEquationArray.clone());
            varlst1 = List::map1r(otherVars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVariables.clone());
            e = listHead(elst.clone())?;
            (listAppend(eqnlst.clone(), eqnlst1.clone()), listAppend(varlst.clone(), varlst1.clone()), e.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("BackendDAETransform.getEquationAndSolvedVar failed!")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEquation, outVar, outIndex))
}

pub fn getEquationAndSolvedVarIndxes(mut inComp: Arc<BackendDAE::StrongComponent>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outEquation: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outVar: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outEquation, outVar) = 'mc: {
        let __mc_input = inComp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: v, eqn: e } => {
                    Ok((list![e.clone()], list![v.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: vlst, eqns: elst, .. } => {
                    Ok((elst.clone(), vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: vlst, eqn: e } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: vlst, eqn: e } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: vlst, eqn: e } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: vlst, eqn: e } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: vlst, eqn: e } => {
                    Ok((list![e.clone()], vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: elst, tearingvars: vlst, .. }, .. } => {
                    let mut elst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vlst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut elst = (*elst).clone();
                    let mut vlst = (*vlst).clone();
                    (elst1, vLstLst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
                    vlst1 = List::flatten(vLstLst.clone());
                    elst = listAppend(elst1.clone(), elst.clone());
                    vlst = listAppend(vlst1.clone(), vlst.clone());
                    Ok((elst.clone(), vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("BackendDAETransform.getEquationAndSolvedVarIndxes failed!")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquation, outVar))
}

// =============================================================================
// traverseBackendDAEExps stuff
//
// =============================================================================
pub fn traverseBackendDAEExpsEqnWithSymbolicOperation<Type_a: Clone + 'static>(mut inEquation: Arc<BackendDAE::Equation>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a))> + 'static>, mut inTypeA: Type_a) -> Result<(Arc<BackendDAE::Equation>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a))> + 'static>;

    let mut outEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTypeA: Type_a;
    (outEquation, outTypeA) = 'mc: {
        let __mc_input = inEquation.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: e2, exp: e1 } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    e1_1 = __pa0.clone();
                    ops = __pa1.clone();
                    ext_arg_1 = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = func(e2.clone(), (ops.clone(), ext_arg_1.clone()))?;
                    e2_1 = __pa3.clone();
                    ops = __pa4.clone();
                    ext_arg_2 = __pa5.clone();
                    source = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1.clone(), scalar: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { recordSize, attr: eqAttr, source, right: e2, left: e1, dimSize } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    e1_1 = __pa0.clone();
                    ops = __pa1.clone();
                    ext_arg_1 = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = func(e2.clone(), (ops.clone(), ext_arg_1.clone()))?;
                    e2_1 = __pa3.clone();
                    ops = __pa4.clone();
                    ext_arg_2 = __pa5.clone();
                    source = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
                    Ok((Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), ext_arg_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::FOR_EQUATION { attr: eqAttr, source, body: eqn, stop, start, iter } => {
                    let mut eqn = (*eqn).clone();
                    let mut outTypeA: Type_a;
                    (eqn, outTypeA) = traverseBackendDAEExpsEqnWithSymbolicOperation(eqn.clone(), func.clone(), inTypeA.clone())?;
                    Ok((Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: iter.clone(), start: start.clone(), stop: stop.clone(), body: eqn.clone(), source: source.clone(), attr: eqAttr.clone() }), outTypeA.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: eqAttr, source, exp: e2, componentRef: cr } => {
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
                    let mut ext_arg_1: Type_a;
                    let mut source = (*source).clone();
                    e1 = Expression::crefExp(cr.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?) {
                        (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, (__pa1, __pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr1 = __pa0.clone();
                    ops = __pa1.clone();
                    ext_arg_1 = __pa2.clone();
                    let (__pa3, (__pa4, _)) = func(e2.clone(), (ops.clone(), ext_arg_1.clone()))?;
                    e2_1 = __pa3.clone();
                    ops = __pa4.clone();
                    source = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
                    Ok((Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr1.clone(), exp: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: eqAttr, source, exp: e1 } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
                    let mut ext_arg_1: Type_a;
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    e1_1 = __pa0.clone();
                    ops = __pa1.clone();
                    ext_arg_1 = __pa2.clone();
                    source = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
                    Ok((Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1_1.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { attr: eqAttr, expand: crefExpand, source, alg: Deref @ DAE::Algorithm { statementLst }, size } => {
                    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
                    let mut ext_arg_1: Type_a;
                    let mut source = (*source).clone();
                    let mut statementLst = (*statementLst).clone();
                    let (__pa0, (__pa1, __pa2)) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), func.clone(), (metamodelica::nil(), inTypeA.clone()));
                    statementLst = __pa0.clone();
                    ops = __pa1.clone();
                    ext_arg_1 = __pa2.clone();
                    source = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
                    Ok((Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: statementLst.clone() }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), ext_arg_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: eqAttr, source, whenEquation: Deref @ BackendDAE::WhenEquation { elsewhenPart: oelsepart, whenStmtLst, condition: cond }, size } => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut elsepartRes: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
                    let mut elsepart: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
                    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut ext_arg_3: Type_a;
                    let mut source = (*source).clone();
                    let mut oelsepart = (*oelsepart).clone();
                    let mut whenStmtLst = (*whenStmtLst).clone();
                    let mut cond = (*cond).clone();
                    (whenStmtLst, ext_arg_1) = traverseBackendDAEExpsWhenOperatorWithSymbolicOperation(whenStmtLst.clone(), func.clone(), inTypeA.clone())?;
                    let (__pa0, (__pa1, __pa2)) = func(cond.clone(), (metamodelica::nil(), ext_arg_1.clone()))?;
                    cond = __pa0.clone();
                    ops = __pa1.clone();
                    ext_arg_2 = __pa2.clone();
                    source = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
                    if isSome(oelsepart.clone()) {
                        let __pa3 = ::match_deref::match_deref! { match &(oelsepart.clone()) {
                            Some(__pa3) => __pa3.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        elsepart = __pa3.clone();
                        let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(traverseBackendDAEExpsEqnWithSymbolicOperation(Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: elsepart.clone(), source: source.clone(), attr: eqAttr.clone() }), func.clone(), ext_arg_2.clone())?) {
                            (Deref @ BackendDAE::Equation::WHEN_EQUATION { source: __pa4, whenEquation: __pa5, .. }, __pa6) => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        source = __pa4.clone();
                        elsepartRes = __pa5.clone();
                        ext_arg_3 = __pa6.clone();
                        oelsepart = Some(elsepartRes.clone());
                    } else {
                        oelsepart = None;
                        ext_arg_3 = ext_arg_2.clone();
                    }
                    eqn = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: oelsepart.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    Ok((eqn.clone(), ext_arg_3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: eqAttr, source, right: e2, left: e1, size } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
                    let mut ext_arg_1: Type_a;
                    let mut ext_arg_2: Type_a;
                    let mut source = (*source).clone();
                    let (__pa0, (__pa1, __pa2)) = func(e1.clone(), (metamodelica::nil(), inTypeA.clone()))?;
                    e1_1 = __pa0.clone();
                    ops = __pa1.clone();
                    ext_arg_1 = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = func(e2.clone(), (ops.clone(), ext_arg_1.clone()))?;
                    e2_1 = __pa3.clone();
                    ops = __pa4.clone();
                    ext_arg_2 = __pa5.clone();
                    source = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
                    Ok((Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { attr: eqAttr, source, eqnsfalse: eqns, eqnstrue: eqnslst, conditions: expl } => {
                    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
                    let mut ext_arg_1: Type_a;
                    let mut source = (*source).clone();
                    let mut eqns = (*eqns).clone();
                    let mut eqnslst = (*eqnslst).clone();
                    let mut expl = (*expl).clone();
                    let (__pa0, (__pa1, __pa2)) = traverseBackendDAEExpsLstEqnWithSymbolicOperation(expl.clone(), func.clone(), (metamodelica::nil(), inTypeA.clone()), metamodelica::nil());
                    expl = __pa0.clone();
                    ops = __pa1.clone();
                    ext_arg_1 = __pa2.clone();
                    source = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone());
                    (eqnslst, ext_arg_1) = traverseBackendDAEExpsEqnLstLstWithSymbolicOperation(eqnslst.clone(), func.clone(), ext_arg_1.clone(), metamodelica::nil())?;
                    (eqns, ext_arg_1) = traverseBackendDAEExpsEqnLstWithSymbolicOperation(eqns.clone(), func.clone(), ext_arg_1.clone(), metamodelica::nil())?;
                    Ok((Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: expl.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: eqAttr.clone() }), ext_arg_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function traverseBackendDAEExpsEqnWithSymbolicOperation failed")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquation, outTypeA))
}

fn traverseBackendDAEExpsLstEqnWithSymbolicOperation<Type_a: Clone + 'static>(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a, mut iAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> (Arc<metamodelica::List<Arc<DAE::Exp>>>, Type_a) {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypeA: Type_a;
    (outExps, outTypeA) = (::match_deref::match_deref! { match &(inExps.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iAcc.clone().reverse(), inTypeA.clone())
        },
        Deref @ metamodelica::List::Cons { head: exp, tail: rest } => {
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut arg: Type_a;
            let mut exp = (*exp).clone();
            (exp, arg) = func(exp.clone(), inTypeA.clone()).unwrap();
            (exps, arg) = traverseBackendDAEExpsLstEqnWithSymbolicOperation(rest.clone(), func.clone(), arg.clone(), cons(exp.clone(), iAcc.clone()));
            (exps.clone(), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExps, outTypeA)
}

pub fn traverseBackendDAEExpsEqnLstWithSymbolicOperation<Type_a: Clone + 'static>(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a))> + 'static>, mut inTypeA: Type_a, mut iAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a))> + 'static>;

    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outTypeA: Type_a;
    (outEqns, outTypeA) = (::match_deref::match_deref! { match &(inEqns.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iAcc.clone().reverse(), inTypeA.clone())
        },
        Deref @ metamodelica::List::Cons { head: eqn, tail: rest } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut arg: Type_a;
            let mut eqn = (*eqn).clone();
            (eqn, arg) = traverseBackendDAEExpsEqnWithSymbolicOperation(eqn.clone(), func.clone(), inTypeA.clone())?;
            (eqns, arg) = traverseBackendDAEExpsEqnLstWithSymbolicOperation(rest.clone(), func.clone(), arg.clone(), cons(eqn.clone(), iAcc.clone()))?;
            (eqns.clone(), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEqns, outTypeA))
}

fn traverseBackendDAEExpsEqnLstLstWithSymbolicOperation<Type_a: Clone + 'static>(mut inEqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a))> + 'static>, mut inTypeA: Type_a, mut iAcc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, Type_a))> + 'static>;

    let mut outEqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
    let mut outTypeA: Type_a;
    (outEqns, outTypeA) = (::match_deref::match_deref! { match &(inEqns.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iAcc.clone().reverse(), inTypeA.clone())
        },
        Deref @ metamodelica::List::Cons { head: eqn, tail: rest } => {
            let mut eqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
            let mut arg: Type_a;
            let mut eqn = (*eqn).clone();
            (eqn, arg) = traverseBackendDAEExpsEqnLstWithSymbolicOperation(eqn.clone(), func.clone(), inTypeA.clone(), metamodelica::nil())?;
            (eqnslst, arg) = traverseBackendDAEExpsEqnLstLstWithSymbolicOperation(rest.clone(), func.clone(), arg.clone(), cons(eqn.clone(), iAcc.clone()))?;
            (eqnslst.clone(), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEqns, outTypeA))
}

fn traverseBackendDAEExpsWhenOperatorWithSymbolicOperation<ArgT: Clone + 'static>(mut inStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, ArgT)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, ArgT))> + 'static>, mut inArg: ArgT) -> Result<(Arc<metamodelica::List<BackendDAE::WhenOperator>>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, ArgT)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, ArgT))> + 'static>;

    let mut outStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut outArg: ArgT = inArg.clone();
    for mut rs in &*inStmtLst.clone() {
        let mut rs = rs.clone();
        rs = (match rs.clone() {
        BackendDAE::WhenOperator::ASSIGN { left: ref lhs, right: ref cond, source: ref src } => {
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut lhs = lhs.clone();
            let mut cond = cond.clone();
            let mut src = src.clone();
            let (__pa0, (__pa1, __pa2)) = func(cond.clone(), (metamodelica::nil(), inArg.clone()))?;
            cond = __pa0.clone();
            ops = __pa1.clone();
            outArg = __pa2.clone();
            let (__pa3, (__pa4, __pa5)) = func(lhs.clone(), (ops.clone(), outArg.clone()))?;
            lhs = __pa3.clone();
            ops = __pa4.clone();
            outArg = __pa5.clone();
            src = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), src.clone());
            BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: cond.clone(), source: src.clone() }
        },
        BackendDAE::WhenOperator::REINIT { stateVar: ref cr, value: ref cond, source: ref src } => {
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut cr = cr.clone();
            let mut cond = cond.clone();
            let mut src = src.clone();
            let (__pa0, (__pa1, __pa2)) = func(cond.clone(), (metamodelica::nil(), inArg.clone()))?;
            cond = __pa0.clone();
            ops = __pa1.clone();
            outArg = __pa2.clone();
            let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr.clone())?, (ops.clone(), outArg.clone()))?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa3, .. }, (__pa4, __pa5)) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa3.clone();
            ops = __pa4.clone();
            outArg = __pa5.clone();
            src = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), src.clone());
            BackendDAE::WhenOperator::REINIT { stateVar: cr.clone(), value: cond.clone(), source: src.clone() }
        },
        BackendDAE::WhenOperator::ASSERT { condition: ref cond, message: ref msg, level: mut level, source: ref src } => {
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut cond = cond.clone();
            let mut src = src.clone();
            let (__pa0, (__pa1, __pa2)) = func(cond.clone(), (metamodelica::nil(), inArg.clone()))?;
            cond = __pa0.clone();
            ops = __pa1.clone();
            outArg = __pa2.clone();
            src = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), src.clone());
            BackendDAE::WhenOperator::ASSERT { condition: cond.clone(), message: msg.clone(), level: level.clone(), source: src.clone() }
        },
        BackendDAE::WhenOperator::NORETCALL { exp: mut exp, source: ref src } => {
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut exp = exp.clone();
            let mut src = src.clone();
            let (__pa0, (__pa1, __pa2)) = Expression::traverseExpBottomUp(exp.clone(), func.clone(), (metamodelica::nil(), outArg.clone()))?;
            exp = __pa0.clone();
            ops = __pa1.clone();
            outArg = __pa2.clone();
            src = List::foldr(ops.clone(), (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), src.clone());
            BackendDAE::WhenOperator::NORETCALL { exp: exp.clone(), source: src.clone() }
        },
        _ => {
            rs.clone()
        },
    });
        outStmtLst = cons(rs.clone(), outStmtLst.clone());
    }
    outStmtLst = outStmtLst.clone().reverse();
    Ok((outStmtLst, outArg))
}

pub fn collapseArrayExpressions(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    for mut syst in &*dae.eqs.clone() {
        let mut syst = syst.clone();
        BackendEquation::traverseEquationArray_WithUpdate(syst.orderedEqs.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(collapseArrayCrefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>); move |__pe_a0, __pe_a2| traverseBackendDAEExpsEqnWithSymbolicOperation(__pe_a0, __pe_b1.clone(), __pe_a2) }), 0)?;
        BackendEquation::traverseEquationArray_WithUpdate(syst.removedEqs.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(collapseArrayCrefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>); move |__pe_a0, __pe_a2| traverseBackendDAEExpsEqnWithSymbolicOperation(__pe_a0, __pe_b1.clone(), __pe_a2) }), 0)?;
    }
    Ok(dae)
}

pub fn collapseArrayCrefExp<T: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, T)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, T))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, T);
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    let mut t: T;
    (ops, t) = inTpl.clone();
    (outExp, t) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(collapseArrayCrefExpWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), t.clone())?;
    if !(ExpressionBasics::expEqual(inExp.clone(), outExp.clone())?) {
        outTpl = (cons(Arc::new(DAE::SymbolicOperation::SIMPLIFY { before: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: inExp.clone() }), after: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: outExp.clone() }) }), ops.clone()), t.clone());
    } else {
        outTpl = inTpl.clone();
    }
    Ok((outExp, outTpl))
}

fn collapseArrayCrefExpWork<T: Clone + 'static>(mut e: Arc<DAE::Exp>, mut t: T) -> Result<(Arc<DAE::Exp>, bool, T)> {
    let mut e: Arc<DAE::Exp> = e;
    let mut cont: bool = false;
    let mut t: T = t;
    (e, cont) = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { .. } => {
                    Ok((collapseArrayCrefExpWork2(e.clone())?, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { .. } => {
                    Ok((collapseArrayCrefExpWork2(e.clone())?, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((e.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((e, cont, t))
}

fn collapseArrayCrefExpWork2(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut e: Arc<DAE::Exp> = e;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut ds: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut len: i32 = 0;
    let mut exp_count: i32 = 0;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut exp1: Arc<DAE::Exp>;
    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut ndim: i32 = 0;
    (dims, ty) = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::MATRIX { ty: ty @ Deref @ DAE::Type::T_ARRAY { dims, .. }, .. } => (dims.clone(), ty.clone()),
        Deref @ DAE::Exp::ARRAY { ty: ty @ Deref @ DAE::Type::T_ARRAY { dims, .. }, .. } => (dims.clone(), ty.clone()),
        _ => bail!("match: no arm matched"),
    } });
    let () = (::match_deref::match_deref! { match &(Types::arrayElementType(ty.clone())) {
        Deref @ DAE::Type::T_COMPLEX { .. } => bail!("fail"),
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ds = Expression::dimensionsSizes(dims.clone());
    ndim = (ds.clone().len() as i32);
    len = ({
        let mut __acc: i32 = 1;
        for mut i in (ds.clone()).into_iter().cloned() {
            let __x = i.clone();
            __acc *= __x;
        }
        __acc
    });
    let true = (len.clone() > 0) else { bail!("pattern mismatch") };
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Expression::flattenArrayExpToList(e.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp1 = __pa0.clone();
    exps = __pa1.clone();
    let __pa2 = ::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: __pa2, .. } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr1 = __pa2.clone();
    subs = ComponentReference::crefLastSubs(cr1.clone())?;
    let true = (ndim.clone() == (subs.clone().len() as i32)) else { bail!("pattern mismatch") };
    let true = ((subs.clone().len() as i32) == (ComponentReferenceBasics::crefSubs(cr1.clone())?.len() as i32)) else { bail!("pattern mismatch") };
    for mut sub in &*subs.clone() {
        let mut sub = sub.clone();
        ::match_deref::match_deref! { match &(sub.clone()) {
            Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } } => (),
            _ => bail!("pattern mismatch"),
        } };
    }
    exp_count = (exps.clone().len() as i32) + 1;
    let true = (exp_count.clone() == len.clone()) else { bail!("pattern mismatch") };
    dims = TypesDump::getDimensions(ComponentReference::crefLastType(cr1.clone())?);
    let true = (exp_count.clone() == ({
        let mut __acc: i32 = 1;
        for mut i in (Expression::dimensionsSizes(dims.clone())).into_iter().cloned() {
            let __x = i.clone();
            __acc *= __x;
        }
        __acc
    })) else { bail!("pattern mismatch") };
    for mut exp in &*exps.clone() {
        let mut exp = exp.clone();
        let __pa4 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { componentRef: __pa4, .. } => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        cr2 = __pa4.clone();
        let true = (ndim.clone() == (ComponentReference::crefLastSubs(cr2.clone())?.len() as i32)) else { bail!("pattern mismatch") };
        let true = (ComponentReferenceBasics::crefEqualWithoutSubs(cr1.clone(), cr2.clone())) else { bail!("pattern mismatch") };
        let true = (1 == ComponentReferenceBasics::crefCompareIntSubscript(cr2.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
        cr1 = cr2.clone();
    }
    e = Expression::makeCrefExp(ComponentReferenceBasics::crefStripLastSubs(cr1.clone())?, ty.clone())?;
    Ok(e)
}

