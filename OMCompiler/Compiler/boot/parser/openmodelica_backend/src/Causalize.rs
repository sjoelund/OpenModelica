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
use crate::BackendDAEFunc;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::DumpGraphML;
use crate::Matching;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

pub type DAEHandler = (BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr);

/* ****************************************
 Singular System check
 *****************************************/
pub fn singularSystemCheck(mut nvars: i32, mut neqns: i32, mut isyst: Arc<BackendDAE::EqSystem>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr), mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), mut ishared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    outSyst = 'mc: {
        let __mc_input = (nvars.clone(), neqns.clone(), isyst.clone(), inMatchingOptions.clone(), matchingAlgorithm.clone(), arg.clone(), ishared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (_, BackendDAE::EquationConstraints::ALLOW_UNDERCONSTRAINED), _, _, _) => {
                    Ok(singularSystemCheck1(nvars.clone(), neqns.clone(), isyst.clone(), crate::BackendDAE::EquationConstraints::ALLOW_UNDERCONSTRAINED, matchingAlgorithm.clone(), arg.clone(), ishared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (_, BackendDAE::EquationConstraints::EXACT), _, _, _) => {
                    let true = (intEq(nvars.clone(), neqns.clone())) else { bail!("pattern mismatch") };
                    Ok(singularSystemCheck1(nvars.clone(), neqns.clone(), isyst.clone(), crate::BackendDAE::EquationConstraints::EXACT, matchingAlgorithm.clone(), arg.clone(), ishared.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (_, BackendDAE::EquationConstraints::EXACT), _, _, _) => {
                    let mut esize_str: ArcStr = arcstr::literal!("");
                    let mut vsize_str: ArcStr = arcstr::literal!("");
                    let true = (intGt(nvars.clone(), neqns.clone())) else { bail!("pattern mismatch") };
                    esize_str = (intString(neqns.clone())).clone();
                    vsize_str = (intString(nvars.clone())).clone();
                    Error::addMessage(Error::UNDERDET_EQN_SYSTEM.clone(), list![(esize_str.clone()).clone(), (vsize_str.clone()).clone()])?;
                    BackendDAEUtil::checkAdjacencyMatrixSolvability(isyst.clone(), ishared.functionTree.clone(), BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut esize_str: ArcStr = arcstr::literal!("");
                    let mut vsize_str: ArcStr = arcstr::literal!("");
                    let true = (intLt(nvars.clone(), neqns.clone())) else { bail!("pattern mismatch") };
                    esize_str = (intString(neqns.clone())).clone();
                    vsize_str = (intString(nvars.clone())).clone();
                    Error::addMessage(Error::OVERDET_EQN_SYSTEM.clone(), list![(esize_str.clone()).clone(), (vsize_str.clone()).clone()])?;
                    BackendDAEUtil::checkAdjacencyMatrixSolvability(isyst.clone(), ishared.functionTree.clone(), BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Causalize.singularSystemCheck failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSyst)
}

//protected import BackendDAETransform;
fn singularSystemCheck1(mut nVars: i32, mut nEqns: i32, mut iSyst: Arc<BackendDAE::EqSystem>, mut eqnConstr: BackendDAE::EquationConstraints, mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr), mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), mut iShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = iSyst.clone();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut matchingFunc: BackendDAEFunc::matchingAlgorithmFunc;
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut indexType: BackendDAE::IndexType = BackendDAE::IndexType::ABSOLUTE;
    let mut scalar: bool = false;
    let mut processed: bool = false;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(iSyst.clone()) {
        Deref @ BackendDAE::EqSystem { mapping: Some((__pa0, __pa1, __pa2, __pa3, __pa4)), mT: Some(__pa5), m: Some(__pa6), .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    mapEqnIncRow = __pa0.clone();
    mapIncRowEqn = __pa1.clone();
    indexType = __pa2.clone();
    scalar = __pa3.clone();
    processed = __pa4.clone();
    mT = __pa5.clone();
    m = __pa6.clone();
    (matchingFunc, _) = matchingAlgorithm.clone();
    m = AdjacencyMatrix::absAdjacencyMatrix(m.clone());
    mT = AdjacencyMatrix::absAdjacencyMatrix(mT.clone());
    syst = BackendDAEUtil::setEqSystMatrices(iSyst.clone(), Some(m.clone()), Some(mT.clone()), Some((mapEqnIncRow.clone(), mapIncRowEqn.clone(), crate::BackendDAE::IndexType::ABSOLUTE, scalar.clone(), processed.clone())))?;
    assign_field!(syst.matching = Arc::new(crate::BackendDAE::Matching::NO_MATCHING));
    let (__pa9, __pa7, __pa8) = ::match_deref::match_deref! { match &(matchingFunc(syst.clone(), iShared.clone(), true, (crate::BackendDAE::IndexReduction::INDEX_REDUCTION, eqnConstr.clone()), (std::sync::Arc::new(foundSingularSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), arg.clone())?) {
        (__pa9 @ Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2: __pa7, ass1: __pa8, .. }, .. }, _, _) => (__pa9.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ass2 = __pa7.clone();
    ass1 = __pa8.clone();
    syst = __pa9.clone();
    assign_field!(outSyst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() }));
    (_, ass1, ass2) = BackendVariable::traverseBackendDAEVars(outSyst.orderedVars.clone(), (std::sync::Arc::new(freeStateAssignments) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(BackendDAE::Var, (i32, metamodelica::Array<i32>, metamodelica::Array<i32>))> + 'static>), (1, ass1.clone(), ass2.clone()))?;
    Ok(outSyst)
}

fn freeStateAssignments(mut inVar: BackendDAE::Var, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(BackendDAE::Var, (i32, metamodelica::Array<i32>, metamodelica::Array<i32>))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    (outVar, outTpl) = (match (inVar.clone(), inTpl.clone()) {
        (mut var @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, (mut index, mut ass1, mut ass2)) => {
            let mut e: i32 = 0;
            e = ass1.borrow()[(index.clone()-1) as usize].clone();
            ass1 = {let _arr = ass1.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = -1; _arr};
            ass2 = {let _arr = ass2.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = -1; _arr};
            (var.clone(), (index.clone() + 1, ass1.clone(), ass2.clone()))
        },
        (mut var, (mut index, mut ass1, mut ass2)) => {
            (var.clone(), (index.clone() + 1, ass1.clone(), ass2.clone()))
        },
    });
    Ok((outVar, outTpl))
}

fn foundSingularSystem(mut eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut actualEqn: i32, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut changedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut actualEqn: i32 = actualEqn;
    let mut isyst: Arc<BackendDAE::EqSystem> = isyst;
    let mut ishared: Arc<BackendDAE::Shared> = ishared;
    let mut inAssignments1: metamodelica::Array<i32> = inAssignments1;
    let mut inAssignments2: metamodelica::Array<i32> = inAssignments2;
    let mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = inArg;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut n: i32 = 0;
    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut eqn_str: ArcStr = arcstr::literal!("");
    let mut var_str: ArcStr = arcstr::literal!("");
    if !(eqns.clone().is_empty()) {
        (_, _, _, mapIncRowEqn, _) = inArg.clone();
        n = BackendDAEUtil::systemSize(isyst.clone());
        unmatched = List::flatten(eqns.clone());
        unmatched1 = List::map1r(unmatched.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
        unmatched1 = List::uniqueIntN(unmatched1.clone(), (mapIncRowEqn.clone().borrow().len() as i32))?;
        eqn_str = (BackendDump::dumpMarkedEqns(isyst.clone(), List::sort(unmatched1.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?)?).clone();
        vars = Matching::getUnassigned(n.clone(), inAssignments2.clone(), metamodelica::nil());
        vars = List::fold1(unmatched.clone(), (std::sync::Arc::new(fnptr!(getAssignedVars, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), inAssignments1.clone(), vars.clone());
        var_str = (BackendDump::dumpMarkedVars(isyst.clone(), List::sort(vars.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?)?).clone();
        source = BackendEquation::markedEquationSource(isyst.clone(), listHead(unmatched1.clone())?)?;
        info = ElementSource::getElementSourceFileInfo(source.clone());
        Error::addSourceMessage(if (BackendDAEUtil::isInitializationDAE(ishared.clone())) {Error::STRUCTURAL_SINGULAR_INITIAL_SYSTEM.clone()} else {Error::STRUCT_SINGULAR_SYSTEM.clone()}, list![(eqn_str.clone()).clone(), (var_str.clone()).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((changedEqns, actualEqn, isyst, ishared, inAssignments1, inAssignments2, inArg))
}

fn getAssignedVars(mut e: i32, mut ass: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut b: bool = false;
    i = ass.borrow()[(e.clone()-1) as usize].clone();
    b = intGt(i.clone(), 0);
    oAcc = List::consOnTrue(b.clone(), i.clone(), iAcc.clone());
    oAcc
}

