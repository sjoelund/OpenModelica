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
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendInline;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Differentiate;
use crate::InlineArrayEquations;
use crate::Matching;
use crate::Sorting;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashTable2;
use openmodelica_frontend::HashTable3;
use openmodelica_frontend::HashTableCG;
use openmodelica_frontend::HashTableCrIntToExp;
use openmodelica_frontend::Inline;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// Pantelides index reduction method .
// see:
// C Pantelides, The Consistent Initialization of Differential-Algebraic Systems, SIAM J. Sci. and Stat. Comput. Volume 9, Issue 2, pp. 213–231 (March 1988)
// Soares, R. de P.; Secchi, A. R.: Direct Initialisation and Solution of High-Index DAESystems. in Proceedings of the European Symbosium on Computer Aided Process Engineering - 15, Barcelona, Spain,
// =============================================================================
pub fn pantelidesIndexReduction(mut inEqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inActualEqn: i32, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut changedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut continueEqn: i32 = 0;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut oass1: metamodelica::Array<i32>;
    let mut oass2: metamodelica::Array<i32>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    let mut markarr: metamodelica::Array<i32>;
    let mut size: i32 = 0;
    let mut newsize: i32 = 0;
    let mut eqns_1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut unassignedStates: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut unassignedEqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    if inEqns.clone().is_empty() {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("- IndexReduction.pantelidesIndexReduction called with empty list of equations!")).clone()])?;
        if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
            println!("{}", (literal!("Index reduction done.\n")).clone());
        }
        bail!("fail");
    }
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::OPT_DAE_DUMP.clone()), '__try0) {
            println!("{}", (literal!("\n\nIndex reduction:\n")).clone());
        }
        ErrorExt::setCheckpoint((literal!("Pantelides")).clone());
        (eqns_1, unassignedStates, unassignedEqns, _) = unwrap_break_err!(minimalStructurallySingularSystem(inEqns.clone(), inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), inArg.clone()), '__try0);
        size = BackendDAEUtil::systemSize(inSystem.clone());
        ErrorExt::delCheckpoint((literal!("Pantelides")).clone());
        ErrorExt::setCheckpoint((literal!("Pantelides")).clone());
        markarr = arrayCreate(size.clone(), -1);
        (osyst, oshared, oass1, oass2, outArg, _) = unwrap_break_err!(pantelidesIndexReduction1(unassignedStates.clone(), unassignedEqns.clone(), inEqns.clone(), eqns_1.clone(), inActualEqn.clone(), inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), 1, markarr.clone(), inArg.clone(), metamodelica::nil()), '__try0);
        ErrorExt::rollBack((literal!("Pantelides")).clone());
        ErrorExt::setCheckpoint((literal!("Pantelides")).clone());
        newsize = BackendDAEUtil::systemSize(osyst.clone());
        changedEqns = if (newsize.clone() > size.clone()) {List::intRange2(size.clone() + 1, newsize.clone())} else {metamodelica::nil()};
        (changedEqns, continueEqn) = getChangedEqnsAndLowest(newsize.clone(), oass2.clone(), changedEqns.clone(), size.clone());
        ErrorExt::delCheckpoint((literal!("Pantelides")).clone());
        if unwrap_break_err!(Flags::isSet(Flags::OPT_DAE_DUMP.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpEqSystemShort(osyst.clone(), (literal!("pantelidesIndexReduction")).clone()), '__try0);
            println!("{}", (literal!("Index reduction done.\n")).clone());
        }
        Ok::<_, anyhow::Error>((changedEqns.clone(), continueEqn.clone(), eqns_1.clone(), markarr.clone(), newsize.clone(), oass1.clone(), oass2.clone(), oshared.clone(), osyst.clone(), outArg.clone(), size.clone(), unassignedEqns.clone(), unassignedStates.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10, __try0_o11, __try0_o12)) => {
            changedEqns = __try0_o0;
            continueEqn = __try0_o1;
            eqns_1 = __try0_o2;
            markarr = __try0_o3;
            newsize = __try0_o4;
            oass1 = __try0_o5;
            oass2 = __try0_o6;
            oshared = __try0_o7;
            osyst = __try0_o8;
            outArg = __try0_o9;
            size = __try0_o10;
            unassignedEqns = __try0_o11;
            unassignedStates = __try0_o12;
        }
        Err(_) => {
            ErrorExt::delCheckpoint((literal!("Pantelides")).clone());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("- IndexReduction.pantelidesIndexReduction failed!")).clone()])?;
            if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
                println!("{}", (literal!("Index reduction done.\n")).clone());
            }
            bail!("fail");
        }
    }
    Ok((changedEqns, continueEqn, osyst, oshared, oass1, oass2, outArg))
}

pub fn failIfIndexReduction(mut inEqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inActualEqn: i32, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut changedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inActualEqn: i32 = inActualEqn;
    let mut inSystem: Arc<BackendDAE::EqSystem> = inSystem;
    let mut inShared: Arc<BackendDAE::Shared> = inShared;
    let mut inAssignments1: metamodelica::Array<i32> = inAssignments1;
    let mut inAssignments2: metamodelica::Array<i32> = inAssignments2;
    let mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = inArg;
    Error::addCompilerError((literal!("Structurally singular system detected, but no index reduction method has been selected.")).clone())?;
    bail!("fail");
    Ok((changedEqns, inActualEqn, inSystem, inShared, inAssignments1, inAssignments2, inArg))
}

fn getChangedEqnsAndLowest(mut index: i32, mut ass2: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>, mut iLowest: i32) -> (Arc<metamodelica::List<i32>>, i32) {
    let mut oAcc: Arc<metamodelica::List<i32>> = iAcc.clone();
    let mut oLowest: i32 = iLowest.clone();
    for mut i in (1..=index.clone()).rev() {
        oAcc = List::consOnTrue(intLt(ass2.borrow()[(i.clone()-1) as usize].clone(), 1), i.clone(), oAcc.clone());
        oLowest = i.clone();
    }
    (oAcc, oLowest)
}

fn pantelidesIndexReduction1(mut unassignedStates: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut unassignedEqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut alleqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iEqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut actualEqn: i32, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut mark: i32, mut markarr: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), mut iNotDiffableMSS: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    let mut oNotDiffableMSS: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    (osyst, oshared, outAssignments1, outAssignments2, outArg, oNotDiffableMSS) = (::match_deref::match_deref! { match &((unassignedStates.clone(), unassignedEqns.clone(), alleqns.clone(), iEqns.clone())) {
        (_, _, _, Deref @ metamodelica::List::Nil) => {
            let mut ass1: metamodelica::Array<i32>;
            let mut ass2: metamodelica::Array<i32>;
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
            (syst, shared, ass1, ass2, arg) = handleundifferntiableMSSLst(iNotDiffableMSS.clone(), inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), inArg.clone())?;
            (syst.clone(), shared.clone(), ass1.clone(), ass2.clone(), arg.clone(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: states, tail: statelst }, Deref @ metamodelica::List::Cons { head: ueqns, tail: ueqnsrest }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnsrest }, Deref @ metamodelica::List::Cons { head: eqns_1, tail: eqnsrest_1 }) => {
            let mut ass1: metamodelica::Array<i32>;
            let mut ass2: metamodelica::Array<i32>;
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
            let mut notDiffableMSS: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
            (syst, shared, ass1, ass2, arg, notDiffableMSS) = pantelidesIndexReductionMSS(states.clone(), ueqns.clone(), eqns.clone(), eqns_1.clone(), actualEqn.clone(), inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), mark.clone(), markarr.clone(), inArg.clone(), iNotDiffableMSS.clone())?;
            (syst, shared, ass1, ass2, arg, notDiffableMSS) = pantelidesIndexReduction1(statelst.clone(), ueqnsrest.clone(), eqnsrest.clone(), eqnsrest_1.clone(), actualEqn.clone(), syst.clone(), shared.clone(), ass1.clone(), ass2.clone(), mark.clone(), markarr.clone(), arg.clone(), notDiffableMSS.clone())?;
            (syst.clone(), shared.clone(), ass1.clone(), ass2.clone(), arg.clone(), notDiffableMSS.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("- IndexReduction.pantelidesIndexReduction1 failed! Use -d=bltdump to get more information.")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared, outAssignments1, outAssignments2, outArg, oNotDiffableMSS))
}

fn pantelidesIndexReductionMSS(mut unassignedStates: Arc<metamodelica::List<i32>>, mut unassignedEqns: Arc<metamodelica::List<i32>>, mut alleqns: Arc<metamodelica::List<i32>>, mut MSSSeqs: Arc<metamodelica::List<i32>>, mut actualEqn: i32, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut mark: i32, mut markarr: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), mut iNotDiffableMSS: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    let mut oNotDiffableMSS: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    (osyst, oshared, outAssignments1, outAssignments2, outArg, oNotDiffableMSS) = 'mc: {
        let __mc_input = (unassignedStates.clone(), unassignedEqns.clone(), alleqns.clone(), MSSSeqs.clone(), actualEqn.clone(), inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), mark.clone(), markarr.clone(), inArg.clone(), iNotDiffableMSS.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _, Deref @ BackendDAE::EqSystem { orderedEqs: eqnsarray, orderedVars: vars, .. }, _, _, _, _, _, (so, orgEqnsLst, mapEqnIncRow, mapIncRowEqn, noofeqns), _) => {
                    let mut MSSSeqs1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut orgEqnsLst1: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
                    let mut syst: Arc<BackendDAE::EqSystem>;
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut eqnstpl: Arc<metamodelica::List<(i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
                    let mut notDiffableMSS: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
                    let mut mapEqnIncRow = (*mapEqnIncRow).clone();
                    let mut mapIncRowEqn = (*mapIncRowEqn).clone();
                    MSSSeqs1 = List::map1r(MSSSeqs.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
                    MSSSeqs1 = List::uniqueIntN(MSSSeqs1.clone(), (mapIncRowEqn.clone().borrow().len() as i32))?;
                    MSSSeqs1 = List::select1(MSSSeqs1.clone(), (std::sync::Arc::new(fnptr!(intLe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), noofeqns.clone());
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("##############--MSSS--##############\n")); __mm_s.push_str(&*literal!("Indices of constraint equations: ")); ArcStr::from(__mm_s) }).clone());
                        BackendDump::debuglst(MSSSeqs1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(" ")).clone(), (literal!("\n")).clone());
                        println!("{}", (literal!("\n")).clone());
                    }
                    (eqnstpl, shared) = differentiateEqnsLst(MSSSeqs1.clone(), vars.clone(), eqnsarray.clone(), inShared.clone())?;
                    (syst, shared, ass1, ass2, orgEqnsLst1, mapEqnIncRow, mapIncRowEqn, notDiffableMSS) = differentiateEqns(eqnstpl.clone(), MSSSeqs1.clone(), unassignedStates.clone(), unassignedEqns.clone(), inSystem.clone(), shared.clone(), inAssignments1.clone(), inAssignments2.clone(), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), iNotDiffableMSS.clone())?;
                    Ok((syst.clone(), shared.clone(), ass1.clone(), ass2.clone(), (so.clone(), orgEqnsLst1.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), noofeqns.clone()), notDiffableMSS.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("- IndexReduction.pantelidesIndexReductionMSS failed! Use -d=bltdump to get more information.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outAssignments1, outAssignments2, outArg, oNotDiffableMSS))
}

fn eqnstplDebugString(mut tpl: (i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>)) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    if isSome(Util::tuple32(tpl.clone())) {
        s = (literal!("")).clone();
    } else {
        s = (BackendDump::equationString(Util::getOption(Util::tuple32(tpl.clone()))?)?).clone();
    }
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Original Eq ")); __mm_s.push_str(&*intString(Util::tuple31(tpl.clone()))); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\n\t-->")); __mm_s.push_str(&*BackendDump::equationString(Util::tuple33(tpl.clone()))?); __mm_s.push_str(&*literal!("")); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

pub fn minimalStructurallySingularSystem(mut inEqnsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<i32>>)> {
    let mut outEqnsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut outStateIndxs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut outunassignedEqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut discEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unassignedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnslst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stateindxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut statemark: metamodelica::Array<i32>;
    let mut size: i32 = 0;
    let mut b: bool = false;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { m: Some(__pa0), orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    eqns = __pa1.clone();
    vars = __pa2.clone();
    size = BackendVariable::varsSize(vars.clone())?;
    statemark = arrayCreate(size.clone(), -1);
    unassignedEqns = List::flatten(inEqnsLst.clone());
    stateindxs = List::fold2(unassignedEqns.clone(), (std::sync::Arc::new(fnptr!(statesInEquations, i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), (m.clone(), statemark.clone(), 0), inAssignments1.clone(), metamodelica::nil());
    (unassignedEqns, eqnslst, discEqns) = List::fold3(unassignedEqns.clone(), (std::sync::Arc::new(unassignedContinuesEqns) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), vars.clone(), inAssignments2.clone(), m.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
    b = intGe((stateindxs.clone().len() as i32), (unassignedEqns.clone().len() as i32));
    singularSystemError(b.clone(), stateindxs.clone(), unassignedEqns.clone(), eqnslst.clone(), syst.clone(), shared.clone(), inAssignments1.clone(), inAssignments2.clone(), inArg.clone())?;
    (outEqnsLst, outStateIndxs, outunassignedEqns, discEqns) = minimalStructurallySingularSystemMSS(inEqnsLst.clone(), syst.clone(), shared.clone(), inAssignments1.clone(), inAssignments2.clone(), inArg.clone(), statemark.clone(), 1, m.clone(), vars.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
    Ok((outEqnsLst, outStateIndxs, outunassignedEqns, discEqns))
}

fn minimalStructurallySingularSystemMSS(mut inEqnsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), mut statemark: metamodelica::Array<i32>, mut mark: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut inEqnsLstAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inStateIndxsAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inUnassEqnsAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inDiscEqnsAcc: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<i32>>)> {
    let mut outEqnsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut outStateIndxs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut outUnassEqnsAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut outDiscEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outEqnsLst, outStateIndxs, outUnassEqnsAcc, outDiscEqns) = (::match_deref::match_deref! { match &(inEqnsLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inEqnsLstAcc.clone(), inStateIndxsAcc.clone(), inUnassEqnsAcc.clone(), inDiscEqnsAcc.clone())
        },
        Deref @ metamodelica::List::Cons { head: ilst, tail: rest } => {
            let mut unassignedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqnsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut discEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut stateIndxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b: bool = false;
            (unassignedEqns, eqnsLst, discEqns) = List::fold3(ilst.clone(), (std::sync::Arc::new(unassignedContinuesEqns) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), vars.clone(), inAssignments2.clone(), m.clone(), (metamodelica::nil(), metamodelica::nil(), inDiscEqnsAcc.clone()));
            stateIndxs = List::fold2(ilst.clone(), (std::sync::Arc::new(fnptr!(statesInEquations, i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), (m.clone(), statemark.clone(), mark.clone()), inAssignments1.clone(), metamodelica::nil());
            b = intGe((stateIndxs.clone().len() as i32), (unassignedEqns.clone().len() as i32));
            singularSystemError(b.clone(), stateIndxs.clone(), unassignedEqns.clone(), eqnsLst.clone(), inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), inArg.clone())?;
            (outEqnsLst, outStateIndxs, outUnassEqnsAcc, outDiscEqns) = minimalStructurallySingularSystemMSS(rest.clone(), inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), inArg.clone(), statemark.clone(), mark.clone() + 1, m.clone(), vars.clone(), cons(eqnsLst.clone(), inEqnsLstAcc.clone()), cons(stateIndxs.clone(), inStateIndxsAcc.clone()), cons(unassignedEqns.clone(), inUnassEqnsAcc.clone()), discEqns.clone())?;
            (outEqnsLst.clone(), outStateIndxs.clone(), outUnassEqnsAcc.clone(), outDiscEqns.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEqnsLst, outStateIndxs, outUnassEqnsAcc, outDiscEqns))
}

fn singularSystemError(mut b: bool, mut unassignedStates: Arc<metamodelica::List<i32>>, mut unassignedEqns: Arc<metamodelica::List<i32>>, mut eqns: Arc<metamodelica::List<i32>>, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((b.clone(), eqns.clone(), inArg.clone())) {
        (true, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
            ()
        },
        (_, Deref @ metamodelica::List::Nil, (_, _, _, mapIncRowEqn, _)) => {
            let mut eqns1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut syst: Arc<BackendDAE::EqSystem>;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                println!("{}", (literal!("Reduce Index failed! Found empty set of continuous equations.\nmarked equations:\n")).clone());
            }
            eqns1 = List::map1r(eqns.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
            eqns1 = List::uniqueIntN(eqns1.clone(), (mapIncRowEqn.clone().borrow().len() as i32))?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                println!("{}", (BackendDump::dumpMarkedEqns(inSystem.clone(), eqns1.clone())?).clone());
            }
            syst = BackendDAEUtil::setEqSystMatching(inSystem.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: inAssignments1.clone(), ass2: inAssignments2.clone(), comps: metamodelica::nil() }))?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                BackendDump::printBackendDAE(BackendDAE::DAE(list![syst.clone()], inShared.clone())?)?;
            }
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("IndexReduction.pantelidesIndexReduction failed! Found empty set of continuous equations. Use -d=bltdump to get more information.")).clone()])?;
            bail!("fail")
        },
        (false, Deref @ metamodelica::List::Cons { head: _, tail: _ }, (_, _, _, mapIncRowEqn, _)) => {
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqns1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut syst: Arc<BackendDAE::EqSystem>;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                println!("{}", (literal!("Reduce Index failed! System is structurally singular and cannot be handled because the number of unassigned continuous equations is larger than the number of states.\nmarked equations:\n")).clone());
                BackendDump::debuglst(eqns.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(" ")).clone(), (literal!("\n")).clone());
            }
            eqns1 = List::map1r(eqns.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
            eqns1 = List::uniqueIntN(eqns1.clone(), (mapIncRowEqn.clone().borrow().len() as i32))?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                println!("{}", (BackendDump::dumpMarkedEqns(inSystem.clone(), eqns1.clone())?).clone());
                println!("{}", (literal!("\n\nunassigned states:\n")).clone());
            }
            varlst = List::map1r(unassignedStates.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), BackendVariable::daeVars(inSystem.clone()));
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                BackendDump::printVarList(varlst.clone());
            }
            syst = BackendDAEUtil::setEqSystMatching(inSystem.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: inAssignments1.clone(), ass2: inAssignments2.clone(), comps: metamodelica::nil() }))?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                BackendDump::printBackendDAE(BackendDAE::DAE(list![syst.clone()], inShared.clone())?)?;
            }
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("IndexReduction.pantelidesIndexReduction failed! System is structurally singular and cannot be handled because the number of unassigned equations is larger than the number of states. Use -d=bltdump to get more information.")).clone()])?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn unassignedContinuesEqns(mut eindx: i32, mut vars: BackendDAE::Variables, mut ass2: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inFold: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outFold: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    outFold = 'mc: {
        let __mc_input = inFold.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (unassignedEqns, eqnsLst, discEqns) => {
                    let mut vindx: i32 = 0;
                    let mut varlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut ba: bool = false;
                    let mut unassignedEqns = (*unassignedEqns).clone();
                    let mut eqnsLst = (*eqnsLst).clone();
                    let mut discEqns = (*discEqns).clone();
                    vindx = ass2.borrow()[(eindx.clone()-1) as usize].clone();
                    ba = intLt(vindx.clone(), 1);
                    varlst = m.borrow()[(eindx.clone()-1) as usize].clone();
                    varlst = List::map(varlst.clone(), Arc::new(intAbs.clone()));
                    vlst = List::map1r(varlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    b = List::all(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isVarDiscrete, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
                    eqnsLst = List::consOnTrue(!(b.clone()), eindx.clone(), eqnsLst.clone());
                    unassignedEqns = List::consOnTrue(ba.clone() && !(b.clone()), eindx.clone(), unassignedEqns.clone());
                    discEqns = List::consOnTrue(b.clone(), eindx.clone(), discEqns.clone());
                    Ok((unassignedEqns.clone(), eqnsLst.clone(), discEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (unassignedEqns, eqnsLst, discEqns) => {
                    let mut vindx: i32 = 0;
                    vindx = ass2.borrow()[(eindx.clone()-1) as usize].clone();
                    let false = (intGt(vindx.clone(), 0)) else { bail!("pattern mismatch") };
                    Ok((cons(eindx.clone(), unassignedEqns.clone()), cons(eindx.clone(), eqnsLst.clone()), discEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFold)
}

fn statesInEquations(mut eindx: i32, mut inTpl: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32), mut ass1: metamodelica::Array<i32>, mut inStateLst: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outStateLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut statemark: metamodelica::Array<i32>;
    let mut mark: i32 = 0;
    (m, statemark, mark) = inTpl.clone();
    vars = List::removeOnTrue(0, (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), m.borrow()[(eindx.clone()-1) as usize].clone());
    vars = List::map(vars.clone(), Arc::new(intAbs.clone()));
    vars = List::removeOnTrue((statemark.clone(), mark.clone()), (std::sync::Arc::new(fnptr!(isMarked, (metamodelica::Array<i32>, i32), i32)) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Array<i32>, i32), i32) -> Result<bool> + 'static>), vars.clone());
    let _ = List::fold1(vars.clone(), (std::sync::Arc::new(markTrue) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), mark.clone(), statemark.clone());
    outStateLst = listAppend(inStateLst.clone(), vars.clone());
    outStateLst
}

fn isMarked(mut ass: (metamodelica::Array<i32>, i32), mut indx: i32) -> bool {
    let mut b: bool = false;
    let mut arr: metamodelica::Array<i32>;
    let mut mark: i32 = 0;
    (arr, mark) = ass.clone();
    b = intEq(arr.borrow()[(intAbs(indx.clone())-1) as usize].clone(), mark.clone());
    b
}

fn markTrue(mut indx: i32, mut mark: i32, mut arr: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut arr: metamodelica::Array<i32> = arr;
    let _ = {let _arr = arr.clone(); _arr.borrow_mut()[(intAbs(indx.clone())-1) as usize] = mark.clone(); _arr};
    Ok(arr)
}

fn collectVarEqns(mut varIdcsIn: Arc<metamodelica::List<i32>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut numVars: i32, mut numEqs: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eqIdcsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varIdx: i32 = 0;
    let mut eqIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut varIdx in &*varIdcsIn.clone() {
        let mut varIdx = varIdx.clone();
        if intLt(varIdx.clone(), numVars.clone()) {
            eqIdcs = List::map(mT.borrow()[(varIdx.clone()-1) as usize].clone(), Arc::new(intAbs.clone()));
            eqIdcsOut = listAppend(eqIdcs.clone(), eqIdcsOut.clone());
        }
    }
    eqIdcsOut = List::uniqueIntN(eqIdcsOut.clone(), numEqs.clone())?;
    Ok(eqIdcsOut)
}

fn searchDerivativesExp(mut inExp: Arc<DAE::Exp>, mut tpl: (Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<i32>>, BackendDAE::Variables);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), tpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (ilst, vars)) => {
                    let mut i1lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut ilst = (*ilst).clone();
                    (_, i1lst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    ilst = List::fold1(i1lst.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ilst.clone());
                    Ok((e.clone(), (ilst.clone(), vars.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

fn differentiateEqnsLst(mut inEqns: Arc<metamodelica::List<i32>>, mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<(i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>)>>, Arc<BackendDAE::Shared>)> {
    let mut outEqnTpl: Arc<metamodelica::List<(i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut oShared: Arc<BackendDAE::Shared>;
    let mut e: i32 = 0;
    let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqTplOpt: Option<(i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>)> = None;
    outEqnTpl = metamodelica::nil();
    oShared = inShared.clone();
    eqs = inEqns.clone();
    while !(eqs.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        eqs = __pa1.clone();
        (eqTplOpt, oShared) = differentiateEqnsLst1(e.clone(), vars.clone(), eqns.clone(), oShared.clone())?;
        if isSome(eqTplOpt.clone()) {
            outEqnTpl = cons(Util::getOption(eqTplOpt.clone())?, outEqnTpl.clone());
        } else {
            outEqnTpl = metamodelica::nil();
            oShared = inShared.clone();
            return Ok((outEqnTpl, oShared));
        }
    }
    Ok((outEqnTpl, oShared))
}

fn differentiateEqnsLst1(mut eqIdx: i32, mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Option<(i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>)>, Arc<BackendDAE::Shared>)> {
    let mut oEqTpl: Option<(i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>)> = None;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut diffEqn: Option<Arc<BackendDAE::Equation>> = None;
    eqn = BackendEquation::get(eqns.clone(), eqIdx.clone());
    if BackendEquation::isDifferentiated(eqn.clone())? {
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            BackendDump::debugStrEqnStr((literal!("Skip already differentiated equation\n")).clone(), eqn.clone(), (literal!("\n")).clone())?;
        }
        oEqTpl = Some((eqIdx.clone(), None, eqn.clone()));
        oshared = inShared.clone();
    } else {
        (diffEqn, oshared) = Differentiate::differentiateEquationTime(eqn.clone(), vars.clone(), inShared.clone())?;
        eqn = BackendEquation::markDifferentiated(eqn.clone())?;
        if isSome(diffEqn.clone()) {
            oEqTpl = Some((eqIdx.clone(), diffEqn.clone(), eqn.clone()));
        } else {
            oEqTpl = None;
            oshared = inShared.clone();
        }
    }
    Ok((oEqTpl, oshared))
}

fn replaceDifferentiatedEqns(mut inEqnTplLst: Arc<metamodelica::List<(i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>)>>, mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imapIncRowEqn: metamodelica::Array<i32>, mut inChangedVars: Arc<metamodelica::List<i32>>, mut inOrgEqns: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>)> {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outChangedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outOrgEqns: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut eqIdx: i32 = 0;
    let mut changedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqOrig: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut eqDiff: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut eqTpl: (i32, Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Equation>);
    outVars = vars.clone();
    outEqns = eqns.clone();
    outChangedVars = inChangedVars.clone();
    outOrgEqns = inOrgEqns.clone();
    for mut eqTpl in &*inEqnTplLst.clone() {
        let mut eqTpl = eqTpl.clone();
        if isSome(Util::tuple32(eqTpl.clone())) {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(eqTpl.clone()) {
                (__pa0, Some(__pa1), __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eqIdx = __pa0.clone();
            eqDiff = __pa1.clone();
            eqOrig = __pa2.clone();
            (eqDiff, _) = BackendEquation::traverseExpsOfEquation(eqDiff.clone(), (std::sync::Arc::new(replaceStateOrderExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), outVars.clone())?;
            let (__pa3, (_, (__pa4, __pa5, __pa6, _, _, _))) = BackendEquation::traverseExpsOfEquation(eqDiff.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(changeDerVariablesToStatesFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>))> + 'static>), (outVars.clone(), outEqns.clone(), outChangedVars.clone(), eqIdx.clone(), imapIncRowEqn.clone(), mt.clone())))?;
            eqDiff = __pa3.clone();
            outVars = __pa4.clone();
            outEqns = __pa5.clone();
            outChangedVars = __pa6.clone();
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                debugdifferentiateEqns((eqOrig.clone(), eqDiff.clone(), eqIdx.clone()))?;
            }
            outEqns = BackendEquation::setAtIndex(outEqns.clone(), eqIdx.clone(), eqDiff.clone())?;
            outOrgEqns = addOrgEqn(eqIdx.clone(), eqOrig.clone(), outOrgEqns.clone())?;
        }
    }
    Ok((outVars, outEqns, outChangedVars, outOrgEqns))
}

fn replaceStateOrderExp(mut inExp: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> {
    let mut e: Arc<DAE::Exp>;
    let mut vars: BackendDAE::Variables;
    (e, vars) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(replaceStateOrderExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, bool, BackendDAE::Variables)> + 'static>), inVars.clone())?;
    Ok((e, vars))
}

fn replaceStateOrderExpFinder(mut inExp: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, bool, BackendDAE::Variables)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outVars: BackendDAE::Variables;
    (outExp, cont, outVars) = 'mc: {
        let __mc_input = (inExp.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, vars) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let (BackendDAE::VAR { varKind: BackendDAE::STATE { derName: Some(__pa0), .. }, .. }, _) = (BackendVariable::getVarSingle(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                    dcr = __pa0.clone();
                    e = Expression::crefExp(dcr.clone())?;
                    Ok((e.clone(), false, vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: index }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" } }, vars) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let true = (intEq(index.clone(), 2)) else { bail!("pattern mismatch") };
                    let (BackendDAE::VAR { varKind: BackendDAE::STATE { derName: Some(__pa0), .. }, .. }, _) = (BackendVariable::getVarSingle(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                    dcr = __pa0.clone();
                    e = Expression::crefExp(dcr.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![e.clone()], attr: attr.clone() }), false, vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" } }, vars) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let (BackendDAE::VAR { varKind: BackendDAE::STATE { derName: Some(__pa0), .. }, .. }, _) = (BackendVariable::getVarSingle(cr.clone(), vars.clone())?) else { bail!("pattern mismatch") };
                    dcr = __pa0.clone();
                    e = Expression::crefExp(dcr.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![e.clone()], attr: attr.clone() }), false, vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, vars) => {
                    Ok((e.clone(), true, vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outVars))
}

fn statesWithUnusedDerivative(mut state: i32, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oAcc = 'mc: {
        let __mc_input = state.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (List::all(mt.borrow()[(state.clone()-1) as usize].clone(), Arc::new(todo!("PARTEVALFUNCTION of intLt: named args do not match any formal: ["i2"]")))) else { bail!("pattern mismatch") };
            Ok(cons(state.clone(), iAcc.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iAcc.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oAcc)
}

fn isStateonIndex(mut index: i32, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut b: bool = false;
    let mut v: BackendDAE::Var;
    v = BackendVariable::getVarAt(vars.clone(), index.clone())?;
    b = BackendVariable::isStateVar(v.clone());
    Ok(b)
}

fn handleundifferntiableMSSLst(mut iNotDiffableMSS: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inAss1: metamodelica::Array<i32>, mut inAss2: metamodelica::Array<i32>, mut iArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut oArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outAss1, outAss2, oArg) = (::match_deref::match_deref! { match &((iNotDiffableMSS.clone(), inSystem.clone(), inShared.clone(), inAss1.clone(), inAss2.clone(), iArg.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            (inSystem.clone(), inShared.clone(), inAss1.clone(), inAss2.clone(), iArg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: (eqns, unassignedStates, unassignedEqns), tail: notDiffableMSS }, Deref @ BackendDAE::EqSystem { mT: Some(mt), orderedVars: v, .. }, _, _, _, (so, orgEqnsLst, mapEqnIncRow, mapIncRowEqn, noofeqns)) => {
            let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
            let mut ass1: metamodelica::Array<i32>;
            let mut ass2: metamodelica::Array<i32>;
            let mut so = (*so).clone();
            let mut orgEqnsLst = (*orgEqnsLst).clone();
            let mut mapEqnIncRow = (*mapEqnIncRow).clone();
            let mut mapIncRowEqn = (*mapIncRowEqn).clone();
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                println!("{}", (literal!("not differentiable minimal singular subset:\n")).clone());
                println!("{}", (literal!("unassignedEqns:\n")).clone());
                BackendDump::debuglst(unassignedEqns.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(", ")).clone(), (literal!("\n")).clone());
                println!("{}", (literal!("unassignedStates:\n")).clone());
                BackendDump::debuglst(unassignedStates.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(", ")).clone(), (literal!("\n")).clone());
            }
            ilst = List::fold1(unassignedStates.clone(), (std::sync::Arc::new(statesWithUnusedDerivative) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mt.clone(), metamodelica::nil());
            ilst = List::select1(ilst.clone(), (std::sync::Arc::new(isStateonIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<bool> + 'static>), v.clone());
            let (_, (__pa0, _)) = BackendDAEUtil::traverseBackendDAEExpsEqns(BackendEquation::getInitialEqnsFromShared(inShared.clone()), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(searchDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables))> + 'static>), (ilst.clone(), v.clone())))?;
            ilst = __pa0.clone();
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                println!("{}", (literal!("states without used derivative:\n")).clone());
                BackendDump::debuglst(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(", ")).clone(), (literal!("\n")).clone());
            }
            (syst, shared, ass1, ass2, so, orgEqnsLst, mapEqnIncRow, mapIncRowEqn) = handleundifferntiableMSS(intLe((ilst.clone().len() as i32), (unassignedEqns.clone().len() as i32)), ilst.clone(), eqns.clone(), unassignedStates.clone(), unassignedEqns.clone(), inSystem.clone(), inShared.clone(), inAss1.clone(), inAss2.clone(), so.clone(), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
            (syst, shared, ass1, ass2, arg) = handleundifferntiableMSSLst(notDiffableMSS.clone(), syst.clone(), shared.clone(), ass1.clone(), ass2.clone(), (so.clone(), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), noofeqns.clone()))?;
            (syst.clone(), shared.clone(), ass1.clone(), ass2.clone(), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((osyst, oshared, outAss1, outAss2, oArg))
}

fn handleundifferntiableMSS(mut b: bool, mut statesWithUnusedDer: Arc<metamodelica::List<i32>>, mut inEqns: Arc<metamodelica::List<i32>>, mut unassignedStates: Arc<metamodelica::List<i32>>, mut unassignedEqns: Arc<metamodelica::List<i32>>, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inAss1: metamodelica::Array<i32>, mut inAss2: metamodelica::Array<i32>, mut inStateOrd: BackendDAE::StateOrder, mut inOrgEqnsLst: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, mut imapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imapIncRowEqn: metamodelica::Array<i32>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut outStateOrd: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    let mut outOrgEqnsLst: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut omapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut omapIncRowEqn: metamodelica::Array<i32>;
    (osyst, oshared, outAss1, outAss2, outStateOrd, outOrgEqnsLst, omapEqnIncRow, omapIncRowEqn) = 'mc: {
        let __mc_input = (b.clone(), statesWithUnusedDer.clone(), inEqns.clone(), unassignedStates.clone(), unassignedEqns.clone(), inSystem.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, syst @ Deref @ BackendDAE::EqSystem { mT: Some(_), m: Some(_), .. }) => {
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eqnslst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eqnslst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut funcs; // TODO: local with unresolved type
                    let mut syst = (*syst).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::fold1(inEqns.clone(), (std::sync::Arc::new(replaceFinalVars) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, BackendVarTransform::VariableReplacements)> + 'static>), BackendVariable::daeGlobalKnownVars(inShared.clone()), (syst.orderedEqs.clone(), metamodelica::nil(), BackendVarTransform::emptyReplacements()))) {
                        (__pa0, __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqns = __pa0.clone();
                    eqnslst = __pa1.clone();
                    todo!("unhandled field-assign shape: syst.orderedEqs");
                    eqnslst1 = List::flatten(List::map1r(eqnslst.clone(), Arc::new(arrayGet.clone()), imapEqnIncRow.clone()));
                    ilst = List::map1r(eqnslst1.clone(), Arc::new(arrayGet.clone()), inAss2.clone());
                    ilst = List::select1(ilst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    ass2 = List::fold1r(eqnslst1.clone(), Arc::new(arrayUpdate.clone()), -1, inAss2.clone());
                    ass1 = List::fold1r(ilst.clone(), Arc::new(arrayUpdate.clone()), -1, inAss1.clone());
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", (literal!("Replaced final Parameter in Eqns\n")).clone());
                        println!("{}", (literal!("Update Adjacency Matrix: ")).clone());
                        BackendDump::debuglst(eqnslst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(" ")).clone(), (literal!("\n")).clone());
                    }
                    funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
                    (syst, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::updateAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), eqnslst.clone(), imapEqnIncRow.clone(), imapIncRowEqn.clone(), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                    Ok((syst.clone(), inShared.clone(), ass1.clone(), ass2.clone(), inStateOrd.clone(), inOrgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _, _, _, syst @ Deref @ BackendDAE::EqSystem { mT: Some(mt), m: Some(m), orderedVars: v, .. }) => {
                    let mut eqnslst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut funcs; // TODO: local with unresolved type
                    let mut syst = (*syst).clone();
                    varlst = List::map1r(statesWithUnusedDer.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), v.clone());
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", (literal!("Change varKind to algebraic for\n")).clone());
                        BackendDump::printVarList(varlst.clone());
                    }
                    varlst = BackendVariable::setVarsKind(varlst.clone(), crate::BackendDAE::VarKind::VARIABLE);
                    todo!("unhandled field-assign shape: syst.orderedVars");
                    eqnslst1 = collectVarEqns(statesWithUnusedDer.clone(), mt.clone(), (mt.clone().borrow().len() as i32), (m.clone().borrow().len() as i32))?;
                    eqnslst1 = List::map1r(eqnslst1.clone(), Arc::new(arrayGet.clone()), imapIncRowEqn.clone());
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", (literal!("Update Adjacency Matrix: ")).clone());
                        BackendDump::debuglst(eqnslst1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(" ")).clone(), (literal!("\n")).clone());
                    }
                    funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
                    (syst, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::updateAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), eqnslst1.clone(), imapEqnIncRow.clone(), imapIncRowEqn.clone(), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                    Ok((syst.clone(), inShared.clone(), inAss1.clone(), inAss2.clone(), inStateOrd.clone(), inOrgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, Deref @ metamodelica::List::Cons { head: i, tail: ilst }, _, _, _, syst @ Deref @ BackendDAE::EqSystem { mT: Some(mt), m: Some(m), orderedVars: v, .. }) => {
                    let mut eqnslst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut var: BackendDAE::Var;
                    let mut funcs; // TODO: local with unresolved type
                    let mut syst = (*syst).clone();
                    var = BackendVariable::getVarAt(v.clone(), i.clone())?;
                    varlst = list![var.clone()];
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", (literal!("Change varKind to algebraic for\n")).clone());
                        BackendDump::printVarList(varlst.clone());
                    }
                    varlst = BackendVariable::setVarsKind(varlst.clone(), crate::BackendDAE::VarKind::VARIABLE);
                    todo!("unhandled field-assign shape: syst.orderedVars");
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        varlst = List::map1r(ilst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), v.clone());
                        println!("{}", (literal!("Other Candidates are\n")).clone());
                        BackendDump::printVarList(varlst.clone());
                    }
                    eqnslst1 = collectVarEqns(list![i.clone()], mt.clone(), (mt.clone().borrow().len() as i32), (m.clone().borrow().len() as i32))?;
                    eqnslst1 = List::map1r(eqnslst1.clone(), Arc::new(arrayGet.clone()), imapIncRowEqn.clone());
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", (literal!("Update Adjacency Matrix: ")).clone());
                        BackendDump::debuglst(eqnslst1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!(" ")).clone(), (literal!("\n")).clone());
                    }
                    funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
                    (syst, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::updateAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), eqnslst1.clone(), imapEqnIncRow.clone(), imapIncRowEqn.clone(), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                    Ok((syst.clone(), inShared.clone(), inAss1.clone(), inAss2.clone(), inStateOrd.clone(), inOrgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _, _, Deref @ BackendDAE::EqSystem { mT: Some(mt), m: Some(_), orderedVars: v, .. }) => {
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut syst: Arc<BackendDAE::EqSystem>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut outAss2: metamodelica::Array<i32>;
                    let mut outAss1: metamodelica::Array<i32>;
                    let mut oshared: Arc<BackendDAE::Shared>;
                    let mut outOrgEqnsLst: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
                    let mut omapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut omapIncRowEqn: metamodelica::Array<i32>;
                    let mut outStateOrd: BackendDAE::StateOrder = outStateOrd.clone();
                    ilst = Matching::getUnassigned(BackendVariable::varsSize(v.clone())?, inAss1.clone(), metamodelica::nil());
                    ilst = List::fold1(ilst.clone(), (std::sync::Arc::new(statesWithUnusedDerivative) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mt.clone(), metamodelica::nil());
                    varlst = List::map1r(ilst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), v.clone());
                    let (_, (__pa0, _)) = BackendDAEUtil::traverseBackendDAEExpsEqns(BackendEquation::getInitialEqnsFromShared(inShared.clone()), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(searchDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables))> + 'static>), (ilst.clone(), v.clone())))?;
                    ilst = __pa0.clone();
                    ::match_deref::match_deref! { match &(ilst.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: _ } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("All unassignedStates without Derivative: ")); __mm_s.push_str(&*stringDelimitList(List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        BackendDump::printVarList(varlst.clone());
                    }
                    (syst, oshared, outAss1, outAss2, outStateOrd, outOrgEqnsLst, omapEqnIncRow, omapIncRowEqn) = handleundifferntiableMSS(intLe((ilst.clone().len() as i32), (unassignedEqns.clone().len() as i32)), ilst.clone(), inEqns.clone(), unassignedStates.clone(), unassignedEqns.clone(), inSystem.clone(), inShared.clone(), inAss1.clone(), inAss2.clone(), inStateOrd.clone(), inOrgEqnsLst.clone(), imapEqnIncRow.clone(), imapIncRowEqn.clone())?;
                    Ok((syst.clone(), oshared.clone(), outAss1.clone(), outAss2.clone(), outStateOrd.clone(), outOrgEqnsLst.clone(), omapEqnIncRow.clone(), omapIncRowEqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, Deref @ BackendDAE::EqSystem { mT: Some(_), m: Some(_), orderedVars: v, .. }) => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    varlst = List::map1r(unassignedStates.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), v.clone());
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", (literal!("unassignedStates\n")).clone());
                        BackendDump::printVarList(varlst.clone());
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outAss1, outAss2, outStateOrd, outOrgEqnsLst, omapEqnIncRow, omapIncRowEqn))
}

fn replaceFinalVars(mut e: i32, mut vars: BackendDAE::Variables, mut inTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, BackendVarTransform::VariableReplacements)> {
    let mut outTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, BackendVarTransform::VariableReplacements);
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut changedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut b: bool = false;
    let mut repl: BackendVarTransform::VariableReplacements;
    (eqns, changedEqns, repl) = inTpl.clone();
    eqn = BackendEquation::get(eqns.clone(), e.clone());
    let (__pa0, (_, __pa1, __pa2)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(replaceFinalVarsEqn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements))> + 'static>), (vars.clone(), false, repl.clone()))?;
    eqn = __pa0.clone();
    b = __pa1.clone();
    repl = __pa2.clone();
    eqns = if (b.clone()) {BackendEquation::setAtIndex(eqns.clone(), e.clone(), eqn.clone())?} else {eqns.clone()};
    changedEqns = List::consOnTrue(b.clone(), e.clone(), changedEqns.clone());
    outTpl = (eqns.clone(), changedEqns.clone(), repl.clone());
    Ok(outTpl)
}

fn replaceFinalVarsEqn(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements))> {
    let mut e: Arc<DAE::Exp>;
    let mut tpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements);
    let mut vars: BackendDAE::Variables;
    let mut b: bool = false;
    let mut repl: BackendVarTransform::VariableReplacements;
    let (__pa0, ref __pa2 @ (_, ref __pa1, _)) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(replaceFinalVarsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements))> + 'static>), inTpl.clone())?;
    e = __pa0.clone();
    b = __pa1.clone();
    tpl = __pa2.clone();
    (e, _) = ExpressionSimplify::condsimplify(b.clone(), e.clone())?;
    Ok((e, tpl))
}

fn replaceFinalVarsExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _, repl)) => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut e2: Arc<DAE::Exp>;
                    let mut repl = (*repl).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (__pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    vlst = __pa0.clone();
                    let (__pa1, true) = (List::fold20(vlst.clone(), (std::sync::Arc::new(replaceFinalVarsGetExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements, bool) -> Result<(BackendVarTransform::VariableReplacements, bool)> + 'static>), repl.clone(), false)) else { bail!("pattern mismatch") };
                    repl = __pa1.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?) {
                        (__pa2, true) => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa2.clone();
                    Ok((e2.clone(), (vars.clone(), true, repl.clone())))
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

fn replaceFinalVarsGetExp(mut inVar: BackendDAE::Var, mut repl: BackendVarTransform::VariableReplacements, mut b: bool) -> Result<(BackendVarTransform::VariableReplacements, bool)> {
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut b: bool = b;
    (repl, b) = 'mc: {
        let __mc_input = inVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { bindExp: Some(exp), varName: cr, .. } => {
                    if !((BackendVariable::isFinalVar(inVar.clone()))) { bail!("guard") }
                    let mut repl1: BackendVarTransform::VariableReplacements;
                    repl1 = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), exp.clone(), None)?;
                    Ok((repl1.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { values, bindExp: None, varName: cr, .. } => {
                    if !((BackendVariable::isFinalVar(inVar.clone()))) { bail!("guard") }
                    let mut repl1: BackendVarTransform::VariableReplacements;
                    let mut exp: Arc<DAE::Exp>;
                    exp = DAEUtil::getStartAttrFail(values.clone())?;
                    repl1 = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), exp.clone(), None)?;
                    Ok((repl1.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((repl.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((repl, b))
}

pub fn getStructurallySingularSystemHandlerArg(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)> {
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut dht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    let mut so: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut count: i32 = 0;
    if Config::getIndexReductionMethod()? == literal!("uode") {
        so = crate::BackendDAE::StateOrder::NOSTATEORDER;
    } else {
        count = ((8 / 3 * BackendVariable::getNumStateVarFromVariables(inSystem.orderedVars.clone())?).0.floor() as i32);
        if count.clone() == 0 {
            so = crate::BackendDAE::StateOrder::NOSTATEORDER;
        } else {
            ht = HashTableCG::emptyHashTableSized(count.clone());
            dht = HashTable3::emptyHashTableSized(count.clone());
            so = BackendDAE::StateOrder::STATEORDER { hashTable: ht.clone(), invHashTable: dht.clone() };
        }
    }
    eqns = BackendEquation::getEqnsFromEqSystem(inSystem.clone());
    outArg = (so.clone(), arrayCreate(BackendEquation::getNumberOfEquations(eqns.clone()), metamodelica::nil()), mapEqnIncRow.clone(), mapIncRowEqn.clone(), BackendEquation::getNumberOfEquations(eqns.clone()));
    Ok(outArg)
}

// =============================================================================
// No State deselection Method.
// use the index 1/0 system as it is
// =============================================================================
pub fn noStateDeselection(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inArgs: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Arc<BackendDAE::BackendDAE> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = inDAE.clone();
    outDAE
}

// =============================================================================
// dynamic state selection method
// see
// - Mattsson, S.E.; Söderlind, G.: A new technique for solving high-index differential-algebraic equations using dummy derivatives, Computer-Aided Control System Design, 1992. (CACSD),1992 IEEE Symposium on , pp.218-224, 17-19 Mar 1992
// - Mattsson, S.E.; Olsson, H; Elmqviste, H. Dynamic Selection of States in Dymola. In: Proceedings of the Modelica Workshop 2000, Lund, Sweden, Modelica Association, 23-24 Oct. 2000.
// - Mattsson, S.; Söderlind, G.: Index reduction in differential-Algebraic equations using dummy derivatives, SIAM J. Sci. Comput. 14, 677-692, 1993.
// =============================================================================
pub fn dynamicStateSelection(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inArgs: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    ht = HashTableCrIntToExp::emptyHashTable();
    (systs, shared, ht) = dynamicStateSelection_mapEqsystem(systs.clone(), shared.clone(), inArgs.clone(), 1, ht.clone())?;
    if intGt(BaseHashTable::hashTableCurrentSize(ht.clone()), 0) {
        (systs, shared) = List::map1Fold(systs.clone(), (std::sync::Arc::new(replaceDummyDerivatives) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)), Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>), ht.clone(), shared.clone());
    }
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

fn dynamicStateSelection_mapEqsystem(mut isysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inShared: Arc<BackendDAE::Shared>, mut iargs: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>>, mut setIndex: i32, mut iHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> {
    let mut osysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut oshared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)) = iHt.clone();
    let mut syst_: Arc<BackendDAE::EqSystem>;
    let mut oarg: Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)> = None;
    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    let mut args: Arc<metamodelica::List<Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>>> = iargs.clone();
    let mut index: i32 = setIndex.clone();
    for mut syst in &*isysts.clone() {
        let mut syst = syst.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        oarg = __pa0.clone();
        args = __pa1.clone();
        if isSome(oarg.clone()) {
            let Some(__pa2) = (oarg.clone()) else { bail!("pattern mismatch") };
            arg = __pa2.clone();
            (syst_, oshared, oHt, index) = dynamicStateSelectionWork(syst.clone(), oshared.clone(), arg.clone(), oHt.clone(), index.clone())?;
            osysts = cons(syst_.clone(), osysts.clone());
        } else {
            osysts = cons(syst.clone(), osysts.clone());
        }
    }
    osysts = metamodelica::Dangerous::listReverseInPlace(osysts.clone());
    Ok((osysts, oshared, oHt))
}

fn countStateCandidates(mut inVar: BackendDAE::Var, mut inCount: i32) -> (BackendDAE::Var, i32) {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outCount: i32 = 0;
    outCount = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: 1, .. }, .. } => {
            let mut statecount: i32 = 0;
            let mut b: bool = false;
            b = BackendVariable::varStateSelectAlways(inVar.clone());
            statecount = if (!(b.clone())) {inCount.clone() + 1} else {inCount.clone()};
            statecount.clone()
        },
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, .. } => {
            let mut statecount: i32 = 0;
            let mut b: bool = false;
            b = BackendVariable::varStateSelectAlways(inVar.clone());
            statecount = if (b.clone()) {inCount.clone() + 1} else {inCount.clone()};
            statecount.clone()
        },
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: None, index: diffcount, .. }, .. } => {
            let mut statecount: i32 = 0;
            let mut b: bool = false;
            statecount = diffcount.clone() + inCount.clone();
            b = BackendVariable::varStateSelectAlways(inVar.clone());
            statecount = if (b.clone()) {statecount.clone() - 1} else {statecount.clone()};
            statecount.clone()
        },
        _ => {
            inCount.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outVar, outCount)
}

fn countStateCandidatesWithNever(mut inVar: BackendDAE::Var, mut inCount: i32) -> (BackendDAE::Var, i32) {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outCount: i32 = 0;
    outCount = (::match_deref::match_deref! { match &(inVar.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: 1, .. }, .. } => {
            let mut statecount: i32 = 0;
            let mut b: bool = false;
            b = BackendVariable::varStateSelectNever(inVar.clone());
            statecount = if (b.clone()) {inCount.clone() + 1} else {inCount.clone()};
            statecount.clone()
        },
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, .. } => {
            let mut statecount: i32 = 0;
            let mut b: bool = false;
            b = BackendVariable::varStateSelectNever(inVar.clone());
            statecount = if (!(b.clone())) {inCount.clone() + 1} else {inCount.clone()};
            statecount.clone()
        },
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: None, index: diffcount, .. }, .. } => {
            let mut statecount: i32 = 0;
            let mut b: bool = false;
            statecount = diffcount.clone() + inCount.clone();
            b = BackendVariable::varStateSelectNever(inVar.clone());
            statecount = if (!(b.clone())) {statecount.clone() - 1} else {statecount.clone()};
            statecount.clone()
        },
        _ => {
            inCount.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outVar, outCount)
}

fn countOrgEqns(mut inOrgEqns: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, mut iCount: i32) -> Result<i32> {
    let mut oCount: i32 = iCount.clone();
    let mut orgeqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut orgEqn: (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    let mut size: i32 = 0;
    let mut numEqs: i32 = 0;
    let mut e: i32 = 0;
    numEqs = (inOrgEqns.clone().borrow().len() as i32);
    for mut e in 1..=numEqs.clone() {
        orgeqns = inOrgEqns.clone().borrow()[(e.clone()-1) as usize].clone();
        size = BackendEquation::equationLstSize(orgeqns.clone())?;
        oCount = oCount.clone() + size.clone();
    }
    Ok(oCount)
}

fn replaceDerStatesStatesExp(mut inExp: Arc<DAE::Exp>, mut inOrder: BackendDAE::StateOrder) -> Result<(Arc<DAE::Exp>, BackendDAE::StateOrder)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outOrder: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    (outExp, outOrder) = 'mc: {
        let __mc_input = (inExp.clone(), inOrder.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, so) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    dcr = getStateOrder(cr.clone(), so.clone())?;
                    e = Expression::crefExp(dcr.clone())?;
                    Ok((e.clone(), so.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inOrder.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outOrder))
}

fn highestOrderDerivatives(mut v: BackendDAE::Variables, mut iSo: BackendDAE::StateOrder) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendDAE::StateOrder)> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oSo: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    (oSo, _, outVars) = BackendVariable::traverseBackendDAEVars(v.clone(), (std::sync::Arc::new(traversinghighestOrderDerivativesFinder) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::StateOrder, BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(BackendDAE::Var, (BackendDAE::StateOrder, BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>))> + 'static>), (iSo.clone(), v.clone(), metamodelica::nil()))?;
    Ok((outVars, oSo))
}

fn traversinghighestOrderDerivativesFinder(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::StateOrder, BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(BackendDAE::Var, (BackendDAE::StateOrder, BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (BackendDAE::StateOrder, BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: None, .. }, .. }, (so, vars, varlst)) => {
                    Ok((v.clone(), (so.clone(), vars.clone(), cons(v.clone(), varlst.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(dcr), .. }, varName: cr, .. }, (so, vars, varlst)) => {
                    let mut b: bool = false;
                    let mut so = (*so).clone();
                    let mut varlst = (*varlst).clone();
                    b = BackendVariable::isState(dcr.clone(), vars.clone())?;
                    varlst = List::consOnTrue(!(b.clone()), v.clone(), varlst.clone());
                    so = addStateOrder(cr.clone(), dcr.clone(), so.clone())?;
                    Ok((v.clone(), (so.clone(), vars.clone(), varlst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn getVar(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables) -> Result<BackendDAE::Var> {
    let mut v: BackendDAE::Var;
    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
    Ok(v)
}

/// Level,nStates,nStateCandidates,nUnassignedEquations,StateCandidates,ConstraintEqns,OtherVars,OtherEqns
pub type StateSets = Arc<metamodelica::List<(i32, i32, i32, i32, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)>>;

fn reduceStateSets(mut iTplLst: StateSets, mut idummyStates: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut odummyStates: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    if !(iTplLst.clone().is_empty()) {
        odummyStates = reduceStateSets2(iTplLst.clone())?;
    } else {
        odummyStates = idummyStates.clone();
    }
    Ok(odummyStates)
}

fn reduceStateSets2(mut iTplLst: StateSets) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut dummyStates: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tpl: (i32, i32, i32, i32, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    let mut rang: i32 = 0;
    let mut nStateCandidates: i32 = 0;
    let mut nUnassignedEquations: i32 = 0;
    let mut stateCandidates: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    for mut tpl in &*iTplLst.clone() {
        let mut tpl = tpl.clone();
        (_, _, nStateCandidates, nUnassignedEquations, stateCandidates, _, _, _) = tpl.clone();
        rang = nStateCandidates.clone() - nUnassignedEquations.clone();
        (_, stateCandidates) = List::split(stateCandidates.clone(), rang.clone())?;
        dummyStates = listAppend(stateCandidates.clone(), dummyStates.clone());
    }
    Ok(dummyStates)
}

fn addStateSets(mut iTplLst: StateSets, mut iSetIndex: i32, mut inSystem: Arc<BackendDAE::EqSystem>) -> Result<(i32, Arc<BackendDAE::EqSystem>)> {
    let mut oSetIndex: i32 = iSetIndex.clone();
    let mut oSystem: Arc<BackendDAE::EqSystem>;
    (oSetIndex, oSystem) = (::match_deref::match_deref! { match &((iTplLst.clone(), inSystem.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (iSetIndex.clone(), inSystem.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, syst) => {
            let mut setIndex: i32 = 0;
            let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut vars: BackendDAE::Variables;
            let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
            let mut syst = (*syst).clone();
            (setIndex, vars, eqs, stateSets) = generateStateSets(iTplLst.clone(), iSetIndex.clone(), syst.orderedVars.clone(), syst.orderedEqs.clone(), syst.stateSets.clone())?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = eqs.clone(),
                syst.stateSets = stateSets.clone()
            );
            (setIndex.clone(), syst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oSetIndex, oSystem))
}

fn generateStateSets(mut iTplLst: StateSets, mut iSetIndex: i32, mut iVars: BackendDAE::Variables, mut iEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iStateSets: Arc<metamodelica::List<BackendDAE::StateSet>>) -> Result<(i32, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::StateSet>>)> {
    let mut oSetIndex: i32 = iSetIndex.clone();
    let mut oVars: BackendDAE::Variables = iVars.clone();
    let mut oEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = iEqns.clone();
    let mut oStateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = iStateSets.clone();
    let mut tpl: (i32, i32, i32, i32, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    let mut setVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut aVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varJ: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut otherVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut stateCandidates: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut crset: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crA: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut set: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crJ: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tyExpCrStates: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut rang: i32 = 0;
    let mut nStates: i32 = 0;
    let mut nStateCandidates: i32 = 0;
    let mut nUnassignedEquations: i32 = 0;
    let mut setIndex: i32 = 0;
    let mut level: i32 = 0;
    let mut recordSize: Option<i32> = None;
    let mut expcrA: Arc<DAE::Exp>;
    let mut mulAstates: Arc<DAE::Exp>;
    let mut mulAdstates: Arc<DAE::Exp>;
    let mut expset: Arc<DAE::Exp>;
    let mut expderset: Arc<DAE::Exp>;
    let mut expsetstart: Arc<DAE::Exp>;
    let mut expcrstates: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expcrdstates: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expcrset: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expcrdset: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expcrstatesstart: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut crstates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut op: DAE::Operator;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut deqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut cEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut b: bool = false;
    for mut tpl in &*iTplLst.clone() {
        let mut tpl = tpl.clone();
        (level, _, nStateCandidates, nUnassignedEquations, stateCandidates, cEqnsLst, otherVars, oEqnLst) = tpl.clone();
        rang = nStateCandidates.clone() - nUnassignedEquations.clone();
        b = intGt(rang.clone(), 1);
        (_, crset, setVars, crA, aVars, tp, crJ, varJ) = getSetVars(oSetIndex.clone(), rang.clone(), nStateCandidates.clone(), nUnassignedEquations.clone(), level.clone())?;
        expcrstates = List::map(stateCandidates.clone(), (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>));
        crstates = List::map(stateCandidates.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
        expcrstatesstart = List::map(crstates.clone(), (std::sync::Arc::new(makeStartExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
        expcrdstates = List::map(expcrstates.clone(), (std::sync::Arc::new(makeder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
        expcrset = List::map(crset.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
        expcrdset = List::map(expcrset.clone(), (std::sync::Arc::new(makeder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
        expcrA = Expression::crefExp(crA.clone())?;
        expcrA = Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: expcrA.clone() });
        tyExpCrStates = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStateCandidates.clone() })] });
        op = if (b.clone()) {DAE::Operator::MUL_MATRIX_PRODUCT { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: rang.clone() })] }) }} else {DAE::Operator::MUL_SCALAR_PRODUCT { ty: DAE::T_REAL_DEFAULT().clone() }};
        mulAstates = Arc::new(DAE::Exp::BINARY { exp1: expcrA.clone(), operator: op.clone(), exp2: Arc::new(DAE::Exp::ARRAY { ty: tyExpCrStates.clone(), scalar: true, array: expcrstates.clone() }) });
        (mulAstates, _) = Expression::extendArrExp(mulAstates.clone(), false)?;
        mulAdstates = Arc::new(DAE::Exp::BINARY { exp1: expcrA.clone(), operator: op.clone(), exp2: Arc::new(DAE::Exp::ARRAY { ty: tyExpCrStates.clone(), scalar: true, array: expcrdstates.clone() }) });
        (mulAdstates, _) = Expression::extendArrExp(mulAdstates.clone(), false)?;
        expset = if (b.clone()) {Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: rang.clone() })] }), scalar: true, array: expcrset.clone() })} else {listHead(expcrset.clone())?};
        expderset = if (b.clone()) {Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: rang.clone() })] }), scalar: true, array: expcrdset.clone() })} else {listHead(expcrdset.clone())?};
        source = Arc::new(DAE::ElementSource { info: SourceInfo { fileName: (literal!("stateselection")).clone(), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) }, partOfLst: metamodelica::nil(), instance: Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE), connectEquationOptLst: metamodelica::nil(), typeLst: metamodelica::nil(), operations: metamodelica::nil(), comment: metamodelica::nil() });
        tp = ComponentReference::crefTypeFull(crA.clone())?;
        tp = DAEUtil::expTypeElementType(tp.clone());
        if DAEUtil::expTypeComplex(tp.clone()) {
            recordSize = Some(Expression::sizeOf(tp.clone())?);
        } else {
            recordSize = None;
        }
        eqn = if (b.clone()) {Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: list![rang.clone()], left: expset.clone(), right: mulAstates.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), recordSize: recordSize.clone() })} else {Arc::new(BackendDAE::Equation::EQUATION { exp: expset.clone(), scalar: mulAstates.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() })};
        deqn = if (b.clone()) {Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: list![rang.clone()], left: expderset.clone(), right: mulAdstates.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), recordSize: recordSize.clone() })} else {Arc::new(BackendDAE::Equation::EQUATION { exp: expderset.clone(), scalar: mulAdstates.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() })};
        expsetstart = Arc::new(DAE::Exp::BINARY { exp1: expcrA.clone(), operator: op.clone(), exp2: Arc::new(DAE::Exp::ARRAY { ty: tyExpCrStates.clone(), scalar: true, array: expcrstatesstart.clone() }) });
        (expsetstart, _) = Expression::extendArrExp(expsetstart.clone(), false)?;
        (setVars, _) = List::map2Fold(setVars.clone(), (std::sync::Arc::new(setStartExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<DAE::Exp>, i32, i32) -> Result<(BackendDAE::Var, i32)> + 'static>), expsetstart.clone(), rang.clone(), 1, metamodelica::nil());
        oVars = BackendVariable::addVars(setVars.clone(), oVars.clone());
        oEqns = BackendEquation::add(eqn.clone(), oEqns.clone())?;
        oEqns = BackendEquation::add(deqn.clone(), oEqns.clone())?;
        stateCandidates = List::map1(stateCandidates.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), crate::BackendDAE::VarKind::DUMMY_STATE);
        otherVars = List::map1(otherVars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), crate::BackendDAE::VarKind::DUMMY_STATE);
        oStateSets = cons(BackendDAE::StateSet { index: oSetIndex.clone(), rang: rang.clone(), state: crset.clone(), crA: crA.clone(), varA: aVars.clone(), statescandidates: stateCandidates.clone(), ovars: otherVars.clone(), eqns: cEqnsLst.clone(), oeqns: oEqnLst.clone(), crJ: crJ.clone(), varJ: varJ.clone(), jacobian: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) }, oStateSets.clone());
        oSetIndex = oSetIndex.clone() + 1;
    }
    if Flags::isSet(Flags::BLT_DUMP.clone())? {
        BackendDump::dumpStateSets(oStateSets.clone(), (literal!("Generated StateSets:")).clone());
    }
    Ok((oSetIndex, oVars, oEqns, oStateSets))
}

pub fn makeStartExp(mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Expression::crefExp(ComponentReference::crefPrefixStart(inCref.clone()))?;
    Ok(outExp)
}

fn setStartExp(mut inVar: BackendDAE::Var, mut startExp: Arc<DAE::Exp>, mut size: i32, mut iIndex: i32) -> Result<(BackendDAE::Var, i32)> {
    let mut outVar: BackendDAE::Var;
    let mut oIndex: i32 = iIndex.clone();
    let mut e: Arc<DAE::Exp>;
    e = if (intGt(size.clone(), 1)) {Expression::makeASUB(startExp.clone(), list![Arc::new(DAE::Exp::ICONST { integer: iIndex.clone() })])?} else {startExp.clone()};
    (e, _) = ExpressionSimplify::simplify(e.clone())?;
    outVar = BackendVariable::setVarStartValue(inVar.clone(), e.clone())?;
    oIndex = iIndex.clone() + 1;
    Ok((outVar, oIndex))
}

fn selectStates(mut nfreeStates: i32, mut nOrgEqns: i32, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut iSo: BackendDAE::StateOrder, mut orgEqnsLst: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, mut iMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMapIncRowEqn: metamodelica::Array<i32>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)), mut iSetIndex: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)), i32)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    let mut oSetIndex: i32 = iSetIndex.clone();
    (osyst, oshared, oHt, oSetIndex) = 'mc: {
        let __mc_input = inSystem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2, ass1, .. }, .. } => {
                    if !((intEq(nfreeStates.clone(), nOrgEqns.clone()))) { bail!("guard") }
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
                    let mut syst: Arc<BackendDAE::EqSystem>;
                    let mut funcs; // TODO: local with unresolved type
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut ne: i32 = 0;
                    let mut nv: i32 = 0;
                    let mut ass2 = (*ass2).clone();
                    let mut ass1 = (*ass1).clone();
                    eqnslst = List::flatten(Arc::new(orgEqnsLst.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
                    syst = BackendEquation::equationsAddDAE(eqnslst.clone(), inSystem.clone())?;
                    (syst, ht) = addAllDummyStates(syst.clone(), iSo.clone(), iHt.clone())?;
                    funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
                    (syst, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                    ass1 = Array::expand(nfreeStates.clone(), ass1.clone(), -1)?;
                    ass2 = Array::expand(nOrgEqns.clone(), ass2.clone(), -1)?;
                    nv = BackendVariable::varsSize(BackendVariable::daeVars(syst.clone()))?;
                    ne = BackendDAEUtil::systemSize(syst.clone());
                    let true = (BackendDAEEXT::setAssignment(ne.clone(), nv.clone(), ass2.clone(), ass1.clone())) else { bail!("pattern mismatch") };
                    Matching::matchingExternalsetAdjacencyMatrix(nv.clone(), ne.clone(), m.clone());
                    BackendDAEEXT::matching(nv.clone(), ne.clone(), 5, -1, metamodelica::OrderedFloat(0.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() }))?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        BackendDump::dumpEquationList(eqnslst.clone(), (literal!("No state selection needed for following equations:")).clone());
                    }
                    Ok((syst.clone(), inShared.clone(), ht.clone(), iSetIndex.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
                    let mut syst: Arc<BackendDAE::EqSystem>;
                    let mut funcs; // TODO: local with unresolved type
                    let mut setIndex: i32 = 0;
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut hov: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut so: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
                    ErrorExt::setCheckpoint((literal!("DynamicStateSelection")).clone());
                    (hov, so) = highestOrderDerivatives(BackendVariable::daeVars(inSystem.clone()), iSo.clone())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        BackendDump::dumpStateOrder(so.clone())?;
                    }
                    funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
                    syst = replaceHigherDerivatives(inSystem.clone())?;
                    (syst, _, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                    (syst, shared, ht, setIndex) = selectStatesWork(1, hov.clone(), syst.clone(), inShared.clone(), so.clone(), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), iHt.clone(), iSetIndex.clone())?;
                    ErrorExt::rollBack((literal!("DynamicStateSelection")).clone());
                    Ok((syst.clone(), shared.clone(), ht.clone(), setIndex.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::delCheckpoint((literal!("DynamicStateSelection")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, oHt, oSetIndex))
}

fn selectStatesWork(mut level: i32, mut iHov: Arc<metamodelica::List<BackendDAE::Var>>, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut so: BackendDAE::StateOrder, mut iOrgEqnsLst: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, mut iMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMapIncRowEqn: metamodelica::Array<i32>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)), mut iSetIndex: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)), i32)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    let mut oSetIndex: i32 = iSetIndex.clone();
    (osyst, oshared, oHt, oSetIndex) = (::match_deref::match_deref! { match &((inSystem.clone(), iOrgEqnsLst.clone())) {
        (_, _) if (Array::all(iOrgEqnsLst.clone(), std::sync::Arc::new(fnptr!(listEmpty, _)))) => {
            (inSystem.clone(), inShared.clone(), iHt.clone(), iSetIndex.clone())
        },
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2, ass1, .. }, orderedVars: vars, .. }, _) => {
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnslst1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut dummyVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut lov: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut hov: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut nfreeStates: i32 = 0;
            let mut neqns: i32 = 0;
            let mut setIndex: i32 = 0;
            let mut ne: i32 = 0;
            let mut ne1: i32 = 0;
            let mut nv: i32 = 0;
            let mut nv1: i32 = 0;
            let mut stateSets: StateSets = metamodelica::nil();
            let mut funcs; // TODO: local with unresolved type
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut orgEqnsLst: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
            let mut repl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut ass2 = (*ass2).clone();
            let mut ass1 = (*ass1).clone();
            (eqnslst1, orgEqnsLst) = removeFirstOrgEqns(iOrgEqnsLst.clone())?;
            (eqnslst, _) = BackendEquation::traverseExpsOfEquationList(eqnslst1.clone(), (std::sync::Arc::new(replaceFinalVarsEqn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements))> + 'static>), (BackendVariable::daeGlobalKnownVars(inShared.clone()), false, BackendVarTransform::emptyReplacements()))?;
            (eqnslst, _) = BackendEquation::traverseExpsOfEquationList(eqnslst.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDerStatesStatesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::StateOrder) -> Result<(Arc<DAE::Exp>, BackendDAE::StateOrder)> + 'static>), so.clone()))?;
            funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
            (eqnslst, _) = BackendEquation::traverseExpsOfEquationList(eqnslst.clone(), Arc::new(forceInlinEqn.clone()), funcs.clone())?;
            (eqnslst, _) = InlineArrayEquations::getScalarArrayEqns(eqnslst.clone())?;
            (hov, ht) = List::map1Fold(iHov.clone(), (std::sync::Arc::new(getLevelStates) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), level.clone(), HashTableCrIntToExp::emptyHashTable());
            (eqnslst, _) = BackendEquation::traverseExpsOfEquationList(eqnslst.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
            (eqnslst1, _) = BackendEquation::traverseExpsOfEquationList(eqnslst1.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
            varlst = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (hov.clone()).into_iter().cloned() {
            if !(BackendVariable::notVarStateSelectAlways(var.clone(), level.clone())) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            neqns = BackendEquation::equationLstSizeKeepAlgorithmAsOne(eqnslst.clone())?;
            nfreeStates = (varlst.clone().len() as i32);
            (dummyVars, stateSets) = selectStatesWork1(nfreeStates.clone(), varlst.clone(), neqns.clone(), eqnslst.clone(), level.clone(), inSystem.clone(), inShared.clone(), so.clone(), iMapEqnIncRow.clone(), iMapIncRowEqn.clone(), hov.clone(), metamodelica::nil(), metamodelica::nil())?;
            lov = List::fold3(iHov.clone(), (std::sync::Arc::new(getlowerOrderDerivatives) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32, BackendDAE::StateOrder, BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>), level.clone(), so.clone(), vars.clone(), metamodelica::nil());
            repl = HashTable2::emptyHashTable();
            (dummyVars, repl) = removeFirstOrderDerivatives(dummyVars.clone(), vars.clone(), so.clone(), repl.clone())?;
            nv = BackendVariable::varsSize(vars.clone())?;
            ne = BackendDAEUtil::systemSize(inSystem.clone());
            syst = BackendEquation::equationsAddDAE(eqnslst1.clone(), inSystem.clone())?;
            if Flags::getConfigString(Flags::INDEX_REDUCTION_METHOD.clone())? == literal!("dummyDerivatives") && neqns.clone() < nfreeStates.clone() {
                dummyVars = reduceStateSets(stateSets.clone(), dummyVars.clone())?;
                stateSets = metamodelica::nil();
            }
            (setIndex, syst) = addStateSets(stateSets.clone(), iSetIndex.clone(), syst.clone())?;
            (syst, ht) = addDummyStates(dummyVars.clone(), level.clone(), repl.clone(), syst.clone(), iHt.clone())?;
            let _ = List::fold1(iHov.clone(), (std::sync::Arc::new(fixDerivativeIndex) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32, BackendDAE::Variables) -> Result<BackendDAE::Variables> + 'static>), level.clone(), BackendVariable::daeVars(syst.clone()));
            (syst, m, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
            nv1 = BackendVariable::varsSize(BackendVariable::daeVars(syst.clone()))?;
            ne1 = BackendDAEUtil::systemSize(syst.clone());
            ass1 = Array::expand(nv1.clone() - nv.clone(), ass1.clone(), -1)?;
            ass2 = Array::expand(ne1.clone() - ne.clone(), ass2.clone(), -1)?;
            let true = (BackendDAEEXT::setAssignment(ne1.clone(), nv1.clone(), ass2.clone(), ass1.clone())) else { bail!("pattern mismatch") };
            Matching::matchingExternalsetAdjacencyMatrix(nv1.clone(), ne1.clone(), m.clone());
            BackendDAEEXT::matching(nv1.clone(), ne1.clone(), 5, -1, metamodelica::OrderedFloat(0.0_f64), 0);
            BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() }))?;
            (syst, shared, ht, setIndex) = selectStatesWork(level.clone() + 1, lov.clone(), syst.clone(), inShared.clone(), so.clone(), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), ht.clone(), setIndex.clone())?;
            (syst.clone(), shared.clone(), ht.clone(), setIndex.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((osyst, oshared, oHt, oSetIndex))
}

fn removeFirstOrderDerivatives(mut iDummyVars: Arc<metamodelica::List<BackendDAE::Var>>, mut iVars: BackendDAE::Variables, mut so: BackendDAE::StateOrder, mut iRepl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)))> {
    let mut oDummyVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oRepl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)) = iRepl.clone();
    for mut var in &*iDummyVars.clone() {
        let mut var = var.clone();
        (oDummyVars, oRepl) = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: 1, .. }, varName: dcr @ Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, ident: DAE::derivativeNamePrefix, .. }, .. } if (!(intEq(System::strncmp((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), (arcstr::literal!(DAE::derivativeNamePrefix)).clone(), 4), 0))) => {
            let mut exp: Arc<DAE::Exp>;
            exp = Expression::crefExp(cr.clone())?;
            exp = Expression::makePureBuiltinCall((literal!("der")).clone(), list![exp.clone()], Expression::r#typeof(exp.clone())?);
            oRepl = BaseHashTable::add((dcr.clone(), exp.clone()), oRepl.clone())?;
            (oDummyVars.clone(), oRepl.clone())
        },
        _ => {
            (cons(var.clone(), oDummyVars.clone()), oRepl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((oDummyVars, oRepl))
}

fn getlowerOrderDerivatives(mut inVar: BackendDAE::Var, mut level: i32, mut so: BackendDAE::StateOrder, mut vars: BackendDAE::Variables, mut iVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut oVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    oVars = 'mc: {
        let __mc_input = inVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: mut diffindx, .. }, varName: ref dcr, .. } = __mc_input.clone() else { bail!("nomatch") };
            if !((intEq(diffindx.clone(), 1))) { bail!("guard") }
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            crlst = getDerStateOrder(dcr.clone(), so.clone())?;
            vlst = List::map1(crlst.clone(), (std::sync::Arc::new(getVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), vars.clone());
            Ok(listAppend(vlst.clone(), iVars.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: mut diffindx, .. }, .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(List::consOnTrue(intGt(diffindx.clone(), level.clone()), inVar.clone(), iVars.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iVars.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oVars)
}

fn fixDerivativeIndex(mut inVar: BackendDAE::Var, mut level: i32, mut iVars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut oVars: BackendDAE::Variables;
    oVars = 'mc: {
        let __mc_input = (inVar.clone(), level.clone(), iVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { natural: mut natural, derName: mut derName, index: mut diffindx }, .. }, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut vars: BackendDAE::Variables;
            let mut v: BackendDAE::Var;
            let true = (intGt(diffindx.clone(), level.clone())) else { bail!("pattern mismatch") };
            diffindx = diffindx.clone() - level.clone();
            v = BackendVariable::setVarKind(inVar.clone(), BackendDAE::VarKind::STATE { index: diffindx.clone(), derName: derName.clone(), natural: natural.clone() })?;
            vars = BackendVariable::addVar(v.clone(), iVars.clone())?;
            Ok(vars.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iVars.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oVars)
}

fn selectStatesWork1(mut nfreeStates: i32, mut statecandidates: Arc<metamodelica::List<BackendDAE::Var>>, mut neqns: i32, mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut level: i32, mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut so: BackendDAE::StateOrder, mut iMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMapIncRowEqn: metamodelica::Array<i32>, mut iHov: Arc<metamodelica::List<BackendDAE::Var>>, mut inDummyVars: Arc<metamodelica::List<BackendDAE::Var>>, mut iStateSets: StateSets) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, StateSets)> {
    let mut outDummyVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oStateSets: StateSets = metamodelica::nil();
    (outDummyVars, oStateSets) = (::match_deref::match_deref! { match &(inSystem.clone()) {
        _ if (intEq(nfreeStates.clone(), neqns.clone())) => {
            (statecandidates.clone(), iStateSets.clone())
        },
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2, ass1, .. }, mT: Some(mT), m: Some(m), orderedEqs: eqns, orderedVars: vars, .. } if (intGt(nfreeStates.clone(), 1) && !(intGt(neqns.clone(), nfreeStates.clone()))) => {
            let mut dummyVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut stateVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut nv: i32 = 0;
            let mut nv1: i32 = 0;
            let mut ne: i32 = 0;
            let mut ne1: i32 = 0;
            let mut neqnarr: i32 = 0;
            let mut hovvars: BackendDAE::Variables;
            let mut eqns1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
            let mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
            let mut stateSets: StateSets = metamodelica::nil();
            let mut indexmap: metamodelica::Array<i32>;
            let mut invindexmap: metamodelica::Array<i32>;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mT1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut eqnslst1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut states: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
            let mut dstates: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
            let mut funcs; // TODO: local with unresolved type
            let mut mT = (*mT).clone();
            let mut m = (*m).clone();
            let mut eqns = (*eqns).clone();
            let mut vars = (*vars).clone();
            hovvars = BackendVariable::listVar1(statecandidates.clone());
            eqns1 = BackendEquation::listEquation(eqnslst.clone())?;
            syst = BackendDAEUtil::createEqSystem(hovvars.clone(), eqns1.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
            (me, meT, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst.clone(), inShared.clone(), false)?;
            m1 = adjacencyMatrixfromEnhancedStrict(me.clone(), hovvars.clone())?;
            mT1 = AdjacencyMatrix::transposeAdjacencyMatrix(m1.clone(), nfreeStates.clone())?;
            hovvars = sortStateCandidatesVars(hovvars.clone(), BackendVariable::daeVars(inSystem.clone()), Some(mT1.clone()))?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("########## Try static state selection ##########\n")); __mm_s.push_str(&*literal!("Try to select dummy vars with natural matching (newer)\n")); __mm_s.push_str(&*literal!("Select ")); __mm_s.push_str(&*intString((eqnslst.clone().len() as i32))); __mm_s.push_str(&*literal!(" dummy states from ")); __mm_s.push_str(&*intString(BackendVariable::varsSize(hovvars.clone())?)); __mm_s.push_str(&*literal!(" candidates.\n")); ArcStr::from(__mm_s) }).clone());
                BackendDump::dumpVariables(hovvars.clone(), (literal!("Highest order derivatives (state candidates):")).clone())?;
                BackendDump::dumpEquationList(eqnslst.clone(), (literal!("Constraint equations:")).clone());
            }
            nv = BackendVariable::varsSize(vars.clone())?;
            ne = BackendEquation::equationArraySize(eqns.clone())?;
            neqnarr = BackendEquation::getNumberOfEquations(eqns.clone());
            ne1 = ne.clone() + neqns.clone();
            indexmap = arrayCreate(nfreeStates.clone() + nv.clone(), -1);
            invindexmap = arrayCreate(nfreeStates.clone(), -1);
            nv1 = nv.clone() + nfreeStates.clone();
            let (__pa0, (__pa1, __pa2, _, _, _, _)) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(getStateIndexes) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, BackendDAE::Variables, Arc<metamodelica::List<i32>>)) -> Result<(BackendDAE::Var, (metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, BackendDAE::Variables, Arc<metamodelica::List<i32>>))> + 'static>), (indexmap.clone(), invindexmap.clone(), 1, nv.clone(), hovvars.clone(), metamodelica::nil()))?;
            vars = __pa0.clone();
            indexmap = __pa1.clone();
            invindexmap = __pa2.clone();
            m1 = arrayCreate(ne1.clone(), metamodelica::nil());
            mT1 = arrayCreate(nv1.clone(), metamodelica::nil());
            mapEqnIncRow = Array::expand(neqns.clone(), iMapEqnIncRow.clone(), metamodelica::nil())?;
            mapIncRowEqn = Array::expand(neqns.clone(), iMapIncRowEqn.clone(), -1)?;
            getAdjacencyMatrixSelectStates(ne.clone(), m1.clone(), mT1.clone(), m.clone(), indexmap.clone())?;
            funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
            getAdjacencyMatrixLevelEquations(eqnslst.clone(), vars.clone(), neqnarr.clone(), ne.clone(), m1.clone(), mT1.clone(), m.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), indexmap.clone(), funcs.clone(), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
            vec1 = Array::expand(nfreeStates.clone(), ass1.clone(), -1)?;
            vec2 = Array::expand(neqns.clone(), ass2.clone(), -1)?;
            let true = (BackendDAEEXT::setAssignment(nv1.clone(), ne1.clone(), vec1.clone(), vec2.clone())) else { bail!("pattern mismatch") };
            Matching::matchingExternalsetAdjacencyMatrix(ne1.clone(), nv1.clone(), mT1.clone());
            BackendDAEEXT::matching(ne1.clone(), nv1.clone(), 3, -1, metamodelica::OrderedFloat(0.0_f64), 0);
            BackendDAEEXT::getAssignment(vec1.clone(), vec2.clone())?;
            comps = Sorting::TarjanTransposed(mT1.clone(), vec2.clone())?;
            comps = List::select1(comps.clone(), (std::sync::Arc::new(selectBlock) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), ne.clone());
            ilst = List::fold1(comps.clone(), (std::sync::Arc::new(fnptr!(getCompsExtraEquations, Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), ne.clone(), metamodelica::nil());
            ilst = List::map1r(ilst.clone(), Arc::new(arrayGet.clone()), iMapIncRowEqn.clone());
            ilst = List::uniqueIntN(ilst.clone(), ne.clone())?;
            eqnslst1 = BackendEquation::getList(ilst.clone(), eqns.clone());
            ilst = List::fold2(comps.clone(), (std::sync::Arc::new(fnptr!(getCompsExtraVars, Arc<metamodelica::List<i32>>, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), nv.clone(), vec2.clone(), metamodelica::nil());
            vlst = List::map1r(ilst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
            eqns = BackendEquation::listEquation(eqnslst.clone())?;
            eqns = BackendEquation::addList(eqnslst1.clone(), eqns.clone())?;
            vars = BackendVariable::listVar1(vlst.clone());
            vars = BackendVariable::addVars(BackendVariable::varList(hovvars.clone())?, vars.clone());
            syst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
            (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst.clone(), inShared.clone(), false)?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
                println!("{}", (literal!("\n")).clone());
                BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
            }
            m = adjacencyMatrixfromEnhancedStrict(me.clone(), vars.clone())?;
            nv = BackendVariable::varsSize(vars.clone())?;
            ne = BackendEquation::equationArraySize(eqns.clone())?;
            mT = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), nv.clone())?;
            Matching::matchingExternalsetAdjacencyMatrix(ne.clone(), nv.clone(), mT.clone());
            BackendDAEEXT::matching(ne.clone(), nv.clone(), 3, -1, metamodelica::OrderedFloat(1.0_f64), 1);
            vec1 = arrayCreate(nv.clone(), -1);
            vec2 = arrayCreate(ne.clone(), -1);
            BackendDAEEXT::getAssignment(vec1.clone(), vec2.clone())?;
            (dstates, states, vec1, vec2) = forceStateSelectNever(vec1.clone(), vec2.clone(), vars.clone(), eqns.clone(), me.clone(), inShared.clone(), so.clone())?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                println!("{}", (literal!("\n")).clone());
                BackendDump::dumpMatchingVars(vec1.clone());
                println!("{}", (literal!("\n")).clone());
                BackendDump::dumpMatchingEqns(vec2.clone());
            }
            (dstates, _) = checkAssignment(1, nv.clone(), vec1.clone(), vars.clone())?;
            dummyVars = List::map1r(List::map(dstates.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _))), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
            stateVars = List::map1r(List::map(states.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _))), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
            dummyVars = List::select(dummyVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
            unassigned = Matching::getUnassigned(ne.clone(), vec2.clone(), metamodelica::nil());
            let _ = Matching::getAssigned(ne.clone(), vec2.clone(), metamodelica::nil());
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                if unassigned.clone().is_empty() {
                    println!("{}", (literal!("Perfect Matching, no dynamic index reduction needed! There are no unassigned equations.\n\n")).clone());
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        BackendDump::dumpVarList(dummyVars.clone(), (literal!("Selected dummy states:")).clone());
                        BackendDump::dumpVarList(stateVars.clone(), (literal!("Selected continuous states:")).clone());
                    }
                } else {
                    println!("{}", (literal!("No perfect matching possible, dynamic index reduction needed.\n")).clone());
                    unassigned = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut unassigned_eq in (unassigned.clone()).into_iter().cloned() {
            let __x = mapIncRowEqn.borrow()[(unassigned_eq.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    BackendDump::dumpEquationList(BackendEquation::getEquationArraySubsetLst(eqns.clone(), unassigned.clone())?, (literal!("Unassigned equations:")).clone());
                    BackendDump::dumpVarList(dummyVars.clone(), (literal!("Statically selected dummy states:")).clone());
                    println!("{}", (literal!("\n")).clone());
                }
            }
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec1.clone(), ass2: vec2.clone(), comps: metamodelica::nil() }))?;
            (syst, m, mT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
            comps = partitionSystem(m.clone(), mT.clone())?;
            (vlst, _, stateSets) = processComps4New(comps.clone(), nv.clone(), ne.clone(), vars.clone(), eqns.clone(), m.clone(), mT.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), vec2.clone(), vec1.clone(), level.clone(), inShared.clone(), iStateSets.clone())?;
            vlst = List::select(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
            (listAppend(dummyVars.clone(), vlst.clone()), stateSets.clone())
        },
        _ if (intGt(neqns.clone(), nfreeStates.clone())) => {
            let mut dummyVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut nv: i32 = 0;
            let mut hovvars: BackendDAE::Variables;
            let mut stateSets: StateSets = metamodelica::nil();
            let mut msg: ArcStr = arcstr::literal!("");
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                hovvars = BackendVariable::listVar1(statecandidates.clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("########## Try static state selection ##########\n")); __mm_s.push_str(&*literal!("Try to select dummy vars with natural matching (newer)\n")); __mm_s.push_str(&*literal!("Select ")); __mm_s.push_str(&*intString((eqnslst.clone().len() as i32))); __mm_s.push_str(&*literal!(" dummy states from ")); __mm_s.push_str(&*intString(BackendVariable::varsSize(hovvars.clone())?)); __mm_s.push_str(&*literal!(" andidates.\n")); ArcStr::from(__mm_s) }).clone());
                BackendDump::dumpVariables(hovvars.clone(), (literal!("Highest order derivatives (state candidates):")).clone())?;
                BackendDump::dumpEquationList(eqnslst.clone(), (literal!("Constraint equations:")).clone());
            }
            msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It is not possible to select continuous time states because Number of Equations ")); __mm_s.push_str(&*intString(neqns.clone())); __mm_s.push_str(&*literal!(" greater than number of States ")); __mm_s.push_str(&*intString(nfreeStates.clone())); __mm_s.push_str(&*literal!(" to select from.")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(msg.clone()).clone()])?;
            nv = (iHov.clone().len() as i32);
            if !(intGe(nv.clone(), neqns.clone())) {
                bail!("fail");
            }
            (dummyVars, stateSets) = selectStatesWork1(nv.clone(), iHov.clone(), neqns.clone(), eqnslst.clone(), level.clone(), inSystem.clone(), inShared.clone(), so.clone(), iMapEqnIncRow.clone(), iMapIncRowEqn.clone(), iHov.clone(), inDummyVars.clone(), iStateSets.clone())?;
            (dummyVars.clone(), stateSets.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outDummyVars, oStateSets))
}

fn forceStateSelectNever(mut vec_old1: metamodelica::Array<i32>, mut vec_old2: metamodelica::Array<i32>, mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut inShared: Arc<BackendDAE::Shared>, mut so: BackendDAE::StateOrder) -> Result<(Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut dummyStates: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut states: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut vec1: metamodelica::Array<i32> = vec_old1.clone();
    let mut vec2: metamodelica::Array<i32> = vec_old2.clone();
    let mut nv: i32 = 0;
    let mut nv2: i32 = 0;
    let mut ne: i32 = 0;
    let mut never_i: i32 = 0;
    let mut eq_i: i32 = 0;
    let mut old_i: i32 = 0;
    let mut var: BackendDAE::Var;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    let mut vec_res1: metamodelica::Array<i32>;
    let mut vec_res2: metamodelica::Array<i32>;
    let mut neverVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut neverVarsArray: BackendDAE::Variables;
    let mut neverIdx: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut syst2: Arc<BackendDAE::EqSystem>;
    let mut me2: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tplLst: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut msg: ArcStr = arcstr::literal!("");
    nv = BackendVariable::varsSize(vars.clone())?;
    ne = BackendEquation::equationArraySize(eqns.clone())?;
    let BackendDAE::STATEORDER { invHashTable: __pa0, .. } = (so.clone()) else { bail!("pattern mismatch") };
    ht = __pa0.clone();
    (dummyStates, states) = checkAssignment(1, nv.clone(), vec_old1.clone(), vars.clone())?;
    for mut state in &*states.clone() {
        let mut state = state.clone();
        var = BackendVariable::getVarAt(vars.clone(), Util::tuple22(state.clone()))?;
        if BackendVariable::varStateSelectNever(var.clone()) && !(BaseHashTable::hasKey(BackendVariable::varCref(var.clone())?, ht.clone())) {
            neverVars = cons(var.clone(), neverVars.clone());
            neverIdx = cons(Util::tuple22(state.clone()), neverIdx.clone());
        }
    }
    if !(neverVars.clone().is_empty()) {
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            BackendDump::dumpVarList(neverVars.clone(), (literal!("StateSelect.never variables that will tried to be forced as dummys")).clone());
        }
        if Matching::anyUnassigned(ne.clone(), vec2.clone()) {
            m = adjacencyMatrixfromEnhanced(me.clone(), vars.clone(), so.clone())?;
            mT = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), nv.clone())?;
            BackendDAEEXT::setAssignment(ne.clone(), nv.clone(), vec2.clone(), vec1.clone());
            Matching::matchingExternalsetAdjacencyMatrix(ne.clone(), nv.clone(), mT.clone());
            BackendDAEEXT::matching(ne.clone(), nv.clone(), 3, -1, metamodelica::OrderedFloat(1.0_f64), 1);
            BackendDAEEXT::getAssignment(vec1.clone(), vec2.clone())?;
            (dummyStates, states) = checkAssignment(1, nv.clone(), vec1.clone(), vars.clone())?;
            neverVars = metamodelica::nil();
            neverIdx = metamodelica::nil();
            for mut state in &*states.clone() {
                let mut state = state.clone();
                var = BackendVariable::getVarAt(vars.clone(), Util::tuple22(state.clone()))?;
                if BackendVariable::varStateSelectNever(var.clone()) && !(BaseHashTable::hasKey(BackendVariable::varCref(var.clone())?, ht.clone())) {
                    neverVars = cons(var.clone(), neverVars.clone());
                    neverIdx = cons(Util::tuple22(state.clone()), neverIdx.clone());
                }
            }
        }
        if !(neverVars.clone().is_empty()) {
            neverVarsArray = BackendVariable::listVar1(neverVars.clone());
            nv2 = BackendVariable::varsSize(neverVarsArray.clone())?;
            syst2 = BackendDAEUtil::createEqSystem(neverVarsArray.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
            (me2, _, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst2.clone(), inShared.clone(), false)?;
            m = adjacencyMatrixfromEnhancedPartial(me2.clone(), vars.clone(), neverVarsArray.clone(), vec2.clone(), so.clone())?;
            if !(AdjacencyMatrix::isEmpty(m.clone())) {
                mT = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), nv2.clone())?;
                vec_res1 = arrayCreate(nv2.clone(), -1);
                vec_res2 = arrayCreate(ne.clone(), -1);
                BackendDAEEXT::setAssignment(ne.clone(), nv2.clone(), vec_res2.clone(), vec_res1.clone());
                Matching::matchingExternalsetAdjacencyMatrix(ne.clone(), nv2.clone(), mT.clone());
                BackendDAEEXT::matching(ne.clone(), nv2.clone(), 3, -1, metamodelica::OrderedFloat(1.0_f64), 1);
                BackendDAEEXT::getAssignment(vec_res1.clone(), vec_res2.clone())?;
                tplLst = List::zip(neverIdx.clone(), List::intRange((neverIdx.clone().len() as i32)));
                for mut tpl in &*tplLst.clone() {
                    let mut tpl = tpl.clone();
                    (never_i, eq_i) = tpl.clone();
                    {
                        let __cell1 = vec_res1.borrow()[(eq_i.clone()-1) as usize].clone();
                        vec1.clone().borrow_mut()[(never_i.clone()-1) as usize] = __cell1;
                    }
                    old_i = vec2.borrow()[(vec_res1.borrow()[(eq_i.clone()-1) as usize].clone()-1) as usize].clone();
                    {
                        let __cell2 = never_i.clone();
                        vec2.clone().borrow_mut()[(vec_res1.borrow()[(eq_i.clone()-1) as usize].clone()-1) as usize] = __cell2;
                    }
                    if !(intEq(old_i.clone(), -1)) {
                        {
                            let __cell3 = -1;
                            vec1.clone().borrow_mut()[(old_i.clone()-1) as usize] = __cell3;
                        }
                    }
                }
            }
            neverVars = metamodelica::nil();
            neverIdx = metamodelica::nil();
            for mut state in &*states.clone() {
                let mut state = state.clone();
                var = BackendVariable::getVarAt(vars.clone(), Util::tuple22(state.clone()))?;
                if BackendVariable::varStateSelectNever(var.clone()) && BackendVariable::isNaturalState(var.clone()) && !(BaseHashTable::hasKey(BackendVariable::varCref(var.clone())?, ht.clone())) {
                    neverVars = cons(var.clone(), neverVars.clone());
                    neverIdx = cons(Util::tuple22(state.clone()), neverIdx.clone());
                }
            }
            if !(neverVars.clone().is_empty()) {
                msg = (System::gettext(({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::varListStringShort(neverVars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("They could not be forced to be statically selected as dummys, this could lead to errors during simulation, please use -d=bltdump for more information.\n")); ArcStr::from(__mm_s) }).clone())).clone();
                Error::addMessage(Error::STATE_STATESELECT_NEVER_FORCED.clone(), list![(msg.clone()).clone()])?;
            }
        }
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n###################################\n")); __mm_s.push_str(&*literal!("INCLUDES FORCED STATESELECT.NEVER()\n")); __mm_s.push_str(&*literal!("###################################\n")); ArcStr::from(__mm_s) }).clone());
        }
        (dummyStates, states) = checkAssignment(1, nv.clone(), vec1.clone(), vars.clone())?;
    }
    Ok((dummyStates, states, vec1, vec2))
}

fn selectBlock(mut comp: Arc<metamodelica::List<i32>>, mut ne: i32) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((comp.clone(), ne.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            false
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _) => {
            b = if (intLe(c.clone(), ne.clone())) {selectBlock(rest.clone(), ne.clone())?} else {true};
            b.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

fn getCompsExtraEquations(mut comp: Arc<metamodelica::List<i32>>, mut neqns: i32, mut iAcc: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    eqns = List::select1(comp.clone(), (std::sync::Arc::new(fnptr!(intLe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), neqns.clone());
    oAcc = listAppend(eqns.clone(), iAcc.clone());
    oAcc
}

fn getCompsExtraVars(mut comp: Arc<metamodelica::List<i32>>, mut nvars: i32, mut ass2: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    vars = List::map1r(comp.clone(), Arc::new(arrayGet.clone()), ass2.clone());
    vars = List::select1(vars.clone(), (std::sync::Arc::new(fnptr!(intLe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), nvars.clone());
    vars = List::select1(vars.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
    oAcc = listAppend(vars.clone(), iAcc.clone());
    oAcc
}

fn dumpBlock(mut comp: Arc<metamodelica::List<i32>>, mut iMapIncRowEqn: metamodelica::Array<i32>, mut nvars: i32, mut syst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ilst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ass2: metamodelica::Array<i32>;
    let mut invindexmap: metamodelica::Array<i32>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2: __pa0, ass1: __pa1, .. }, m: Some(__pa2), .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ass2 = __pa0.clone();
    invindexmap = __pa1.clone();
    m = __pa2.clone();
    eqns = List::map1r(comp.clone(), Arc::new(arrayGet.clone()), iMapIncRowEqn.clone());
    eqns = List::uniqueIntN(eqns.clone(), BackendDAEUtil::equationArraySizeDAE(syst.clone()))?;
    ilst = List::map1r(comp.clone(), Arc::new(arrayGet.clone()), ass2.clone());
    (ilst1, ilst) = List::split1OnTrue(ilst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), nvars.clone());
    ilst1 = List::map1(ilst1.clone(), (std::sync::Arc::new(fnptr!(intSub, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), nvars.clone());
    ilst1 = List::map1r(ilst1.clone(), Arc::new(arrayGet.clone()), invindexmap.clone());
    ilst1 = listAppend(ilst.clone(), ilst1.clone());
    println!("{}", (literal!("##########################\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::dumpMarkedVars(syst.clone(), ilst1.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (BackendDump::dumpMarkedEqns(syst.clone(), eqns.clone())?).clone());
    Ok(())
}

fn getStateIndexes(mut inVar: BackendDAE::Var, mut inTpl: (metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, BackendDAE::Variables, Arc<metamodelica::List<i32>>)) -> Result<(BackendDAE::Var, (metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, BackendDAE::Variables, Arc<metamodelica::List<i32>>))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, BackendDAE::Variables, Arc<metamodelica::List<i32>>);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, varName: cr, .. }, (stateindexs, invmap, indx, nv, hov, derstatesindexs)) => {
                    let mut s: i32 = 0;
                    let mut newindx: i32 = 0;
                    (_, s) = BackendVariable::getVarSingle(cr.clone(), hov.clone())?;
                    newindx = nv.clone() + s.clone();
                    {let _arr = stateindexs.clone(); _arr.borrow_mut()[(indx.clone()-1) as usize] = newindx.clone(); _arr};
                    {let _arr = invmap.clone(); _arr.borrow_mut()[(s.clone()-1) as usize] = indx.clone(); _arr};
                    Ok((inVar.clone(), (stateindexs.clone(), invmap.clone(), indx.clone() + 1, nv.clone(), hov.clone(), cons(indx.clone(), derstatesindexs.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (stateindexs, invmap, indx, nv, hov, derstatesindexs)) => {
                    Ok((inVar.clone(), (stateindexs.clone(), invmap.clone(), indx.clone() + 1, nv.clone(), hov.clone(), derstatesindexs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn getAdjacencyMatrixSelectStates(mut nEqns: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mo: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut stateindexs: metamodelica::Array<i32>) -> Result<()> {
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut negrow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in (1..=nEqns.clone()).rev() {
        row = mo.borrow()[(i.clone()-1) as usize].clone();
        row = List::map1(row.clone(), (std::sync::Arc::new(fnptr!(replaceStateIndex, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<i32> + 'static>), stateindexs.clone());
        {let _arr = m.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = row.clone(); _arr};
        (row, negrow) = List::split1OnTrue(row.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
        let _ = List::fold1(row.clone(), (std::sync::Arc::new(Array::consToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), i.clone(), mT.clone());
        row = List::map(negrow.clone(), Arc::new(intAbs.clone()));
        let _ = List::fold1(row.clone(), (std::sync::Arc::new(Array::consToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), -(i.clone()), mT.clone());
    }
    Ok(())
}

fn replaceStateIndex(mut iR: i32, mut stateindexs: metamodelica::Array<i32>) -> i32 {
    let mut oR: i32 = 0;
    let mut s: i32 = 0;
    let mut r: i32 = 0;
    oR = iR.clone();
    if !(intGt(iR.clone(), 0)) {
        r = intAbs(iR.clone());
        s = stateindexs.borrow()[(r.clone()-1) as usize].clone();
        if intGt(s.clone(), 0) {
            oR = s.clone();
        }
    }
    oR
}

fn partitionSystem(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut systs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut rowmarkarr: metamodelica::Array<i32>;
    let mut collmarkarr: metamodelica::Array<i32>;
    let mut nsystems: i32 = 0;
    let mut neqns: i32 = 0;
    let mut systsarr: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    neqns = (m.clone().borrow().len() as i32);
    rowmarkarr = arrayCreate(neqns.clone(), 0);
    collmarkarr = arrayCreate((mT.clone().borrow().len() as i32), 0);
    nsystems = partitionSystem1(neqns.clone(), m.clone(), mT.clone(), rowmarkarr.clone(), collmarkarr.clone(), 1)?;
    systsarr = arrayCreate(nsystems.clone(), metamodelica::nil());
    systsarr = partitionSystemSplitt(neqns.clone(), rowmarkarr.clone(), systsarr.clone())?;
    systs = Arc::new(systsarr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    Ok(systs)
}

fn partitionSystem1(mut index: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarkarr: metamodelica::Array<i32>, mut collmarkarr: metamodelica::Array<i32>, mut iNSystems: i32) -> Result<i32> {
    let mut oNSystems: i32 = 0;
    oNSystems = (match index.clone() {
        0 => {
            iNSystems.clone() - 1
        },
        _ if (!(intGt(rowmarkarr.borrow()[(index.clone()-1) as usize].clone(), 0))) => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut nsystems: i32 = 0;
            {let _arr = rowmarkarr.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = iNSystems.clone(); _arr};
            rows = List::select(m.borrow()[(index.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
            nsystems = partitionSystemstraverseRows(rows.clone(), metamodelica::nil(), m.clone(), mT.clone(), rowmarkarr.clone(), collmarkarr.clone(), iNSystems.clone())?;
            partitionSystem1(index.clone() - 1, m.clone(), mT.clone(), rowmarkarr.clone(), collmarkarr.clone(), nsystems.clone())?
        },
        _ => {
            partitionSystem1(index.clone() - 1, m.clone(), mT.clone(), rowmarkarr.clone(), collmarkarr.clone(), iNSystems.clone())?
        },
    });
    Ok(oNSystems)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn partitionSystemstraverseRows(mut iRows: Arc<metamodelica::List<i32>>, mut iQueue: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarkarr: metamodelica::Array<i32>, mut collmarkarr: metamodelica::Array<i32>, mut iNSystems: i32) -> Result<i32> {
    let mut oNSystems: i32 = 0;
    oNSystems = (::match_deref::match_deref! { match &((iRows.clone(), iQueue.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            iNSystems.clone() + 1
        },
        (Deref @ metamodelica::List::Nil, _) => {
            partitionSystemstraverseRows(iQueue.clone(), metamodelica::nil(), m.clone(), mT.clone(), rowmarkarr.clone(), collmarkarr.clone(), iNSystems.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _) if (!(intGt(collmarkarr.borrow()[(r.clone()-1) as usize].clone(), 0))) => {
            let mut colls: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            {let _arr = collmarkarr.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = iNSystems.clone(); _arr};
            colls = List::select(mT.borrow()[(r.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
            colls = List::select1r(colls.clone(), (std::sync::Arc::new(fnptr!(Matching::isUnAssigned, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarkarr.clone());
            let _ = List::fold1(colls.clone(), (std::sync::Arc::new(markTrue) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), iNSystems.clone(), rowmarkarr.clone());
            rows = List::flatten(List::map1r(colls.clone(), Arc::new(arrayGet.clone()), m.clone()));
            rows = listAppend(List::select1r(rows.clone(), (std::sync::Arc::new(fnptr!(Matching::isUnAssigned, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), collmarkarr.clone()), iQueue.clone());
            partitionSystemstraverseRows(rest.clone(), rows.clone(), m.clone(), mT.clone(), rowmarkarr.clone(), collmarkarr.clone(), iNSystems.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            partitionSystemstraverseRows(rest.clone(), iQueue.clone(), m.clone(), mT.clone(), rowmarkarr.clone(), collmarkarr.clone(), iNSystems.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oNSystems)
}

fn partitionSystemSplitt(mut index: i32, mut rowmarkarr: metamodelica::Array<i32>, mut systsarr: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut osystsarr: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    osystsarr = (match index.clone() {
        0 => {
            systsarr.clone()
        },
        _ => {
            let mut i: i32 = 0;
            let mut arr: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            i = rowmarkarr.borrow()[(index.clone()-1) as usize].clone();
            arr = Array::consToElement(i.clone(), index.clone(), systsarr.clone())?;
            partitionSystemSplitt(index.clone() - 1, rowmarkarr.clone(), arr.clone())?
        },
    });
    Ok(osystsarr)
}

fn processComps4New(mut iSets: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inVarSize: i32, mut inEqnsSize: i32, mut iVars: BackendDAE::Variables, mut iEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inMT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inMapIncRowEqn: metamodelica::Array<i32>, mut vec1: metamodelica::Array<i32>, mut vec2: metamodelica::Array<i32>, mut level: i32, mut iShared: Arc<BackendDAE::Shared>, mut iStateSets: StateSets) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, StateSets)> {
    let mut outDummyVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outDummyStates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut oStateSets: StateSets = iStateSets.clone();
    let mut mapEqnIncRow1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn1: metamodelica::Array<i32>;
    let mut ass1arr: metamodelica::Array<i32>;
    let mut dummyStates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqns1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = iEqns.clone();
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut seteqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut assigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut set: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut statevars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dstatevars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ass1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ass2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut assigend1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut range: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut flag: metamodelica::Array<bool>;
    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut states1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut dstates1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut nstatevars: i32 = 0;
    let mut nassigned: i32 = 0;
    let mut nunassigned: i32 = 0;
    let mut nass1arr: i32 = 0;
    let mut n: i32 = 0;
    let mut nv: i32 = 0;
    let mut ne: i32 = 0;
    let mut stateSets: StateSets = metamodelica::nil();
    match '__try0: {
        for mut seteqns in &*iSets.clone() {
            let mut seteqns = seteqns.clone();
            if !(List::select1r(seteqns.clone(), (std::sync::Arc::new(fnptr!(Matching::isUnAssigned, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), vec1.clone()).is_empty()) {
                unassigned = List::select1r(seteqns.clone(), (std::sync::Arc::new(fnptr!(Matching::isUnAssigned, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), vec1.clone());
                n = (inM.clone().borrow().len() as i32);
                set = unwrap_break_err!(getEqnsforDynamicStateSelection(unassigned.clone(), n.clone(), inM.clone(), inMT.clone(), vec1.clone(), vec2.clone(), inMapEqnIncRow.clone(), inMapIncRowEqn.clone()), '__try0);
                assigned = List::select1r(set.clone(), (std::sync::Arc::new(fnptr!(Matching::isAssigned, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), vec1.clone());
                flag = arrayCreate(inVarSize.clone(), true);
                (statevars, _) = List::fold3(set.clone(), (std::sync::Arc::new(fnptr!(getSetStates, i32, metamodelica::Array<bool>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>))) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<bool>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), flag.clone(), inM.clone(), vec2.clone(), (metamodelica::nil(), metamodelica::nil()));
                nstatevars = (statevars.clone().len() as i32);
                ass1 = List::consN(nstatevars.clone(), -1, metamodelica::nil());
                nunassigned = (unassigned.clone().len() as i32);
                ass2 = List::consN(nunassigned.clone(), -1, metamodelica::nil());
                varlst = List::map1r(statevars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iVars.clone());
                assigend1 = List::map1r(unassigned.clone(), Arc::new(arrayGet.clone()), inMapIncRowEqn.clone());
                n = (inMapIncRowEqn.clone().borrow().len() as i32);
                assigend1 = unwrap_break_err!(List::uniqueIntN(assigend1.clone(), n.clone()), '__try0);
                eqnlst = BackendEquation::getList(assigend1.clone(), eqns1.clone());
                eqns1 = List::fold(assigend1.clone(), (std::sync::Arc::new(BackendEquation::delete) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), eqns1.clone());
                nassigned = (assigned.clone().len() as i32);
                flag = arrayCreate(inEqnsSize.clone(), true);
                (eqnlst, varlst, ass1, ass2, eqns1) = unwrap_break_err!(getSetSystem(assigned.clone(), inMapEqnIncRow.clone(), inMapIncRowEqn.clone(), vec1.clone(), iVars.clone(), eqns1.clone(), flag.clone(), nassigned.clone(), eqnlst.clone(), varlst.clone(), ass1.clone(), ass2.clone()), '__try0);
                eqns = unwrap_break_err!(BackendEquation::listEquation(eqnlst.clone()), '__try0);
                vars = BackendVariable::listVar1(varlst.clone());
                syst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                (_, _, _, mapIncRowEqn1) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst.clone(), iShared.clone(), false), '__try0);
                ass1arr = metamodelica::arrayFromVec(ass1.clone().into_iter().cloned().collect());
                nass1arr = (ass1arr.clone().borrow().len() as i32);
                (dstates1, states1) = unwrap_break_err!(checkAssignment(1, nass1arr.clone(), ass1arr.clone(), vars.clone()), '__try0);
                assigend1 = if (!(assigned.clone().is_empty())) {List::intRange2(1, nassigned.clone())} else {metamodelica::nil()};
                nunassigned = nassigned.clone() + nunassigned.clone();
                nassigned = nassigned.clone() + 1;
                range = List::intRange2(nassigned.clone(), nunassigned.clone());
                nv = unwrap_break_err!(BackendVariable::varsSize(vars.clone()), '__try0);
                ne = unwrap_break_err!(BackendEquation::equationArraySize(eqns.clone()), '__try0);
                (varlst, oStateSets) = unwrap_break_err!(selectDummyDerivatives2new(dstates1.clone(), states1.clone(), range.clone(), assigend1.clone(), vars.clone(), nv.clone(), eqns.clone(), ne.clone(), mapIncRowEqn1.clone(), level.clone(), oStateSets.clone()), '__try0);
                dummyStates = List::map(varlst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                outDummyStates = List::append_reverse(dummyStates.clone(), outDummyStates.clone());
                outDummyVars = listAppend(varlst.clone(), outDummyVars.clone());
            }
        }
        outDummyStates = metamodelica::Dangerous::listReverseInPlace(outDummyStates.clone());
        Ok::<_, anyhow::Error>((outDummyStates.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outDummyStates = __try0_o0;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("- IndexReduction.processComps4New failed!")).clone()])?;
            bail!("fail");
        }
    }
    Ok((outDummyVars, outDummyStates, oStateSets))
}

fn getSetSystem(mut iEqns: Arc<metamodelica::List<i32>>, mut inMapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inMapIncRowEqn: metamodelica::Array<i32>, mut vec1: metamodelica::Array<i32>, mut iVars: BackendDAE::Variables, mut iEqnsArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut flag: metamodelica::Array<bool>, mut n: i32, mut iEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut iAss1: Arc<metamodelica::List<i32>>, mut iAss2: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut oEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut oVarsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oAss1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oAss2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oEqnsArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    (oEqnsLst, oVarsLst, oAss1, oAss2, oEqnsArr) = (::match_deref::match_deref! { match &(iEqns.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iEqnsLst.clone(), iVarsLst.clone(), iAss1.clone(), iAss2.clone(), iEqnsArr.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: rest } if (flag.borrow()[(e.clone()-1) as usize].clone() && intGt(vec1.borrow()[(e.clone()-1) as usize].clone(), 0)) => {
            let mut e1: i32 = 0;
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vindx: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ass: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ass1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ass2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqnarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            e1 = inMapIncRowEqn.borrow()[(e.clone()-1) as usize].clone();
            eqn = BackendEquation::get(iEqnsArr.clone(), e1.clone());
            eqnarr = BackendEquation::delete(e1.clone(), iEqnsArr.clone())?;
            eqns = inMapEqnIncRow.borrow()[(e1.clone()-1) as usize].clone();
            let _ = List::fold1r(eqns.clone(), Arc::new(arrayUpdate.clone()), false, flag.clone());
            vindx = List::map1r(eqns.clone(), Arc::new(arrayGet.clone()), vec1.clone());
            varlst = listAppend(List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iVars.clone()), iVarsLst.clone());
            ass = List::intRange2(n.clone() - (eqns.clone().len() as i32) + 1, n.clone());
            ass1 = listAppend(ass.clone(), iAss1.clone());
            ass2 = listAppend(ass.clone(), iAss2.clone());
            (oEqnsLst, oVarsLst, ass1, ass2, eqnarr) = getSetSystem(rest.clone(), inMapEqnIncRow.clone(), inMapIncRowEqn.clone(), vec1.clone(), iVars.clone(), eqnarr.clone(), flag.clone(), n.clone() - (eqns.clone().len() as i32), cons(eqn.clone(), iEqnsLst.clone()), varlst.clone(), ass1.clone(), ass2.clone())?;
            (oEqnsLst.clone(), oVarsLst.clone(), ass1.clone(), ass2.clone(), eqnarr.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut ass1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ass2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqnarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            (oEqnsLst, oVarsLst, ass1, ass2, eqnarr) = getSetSystem(rest.clone(), inMapEqnIncRow.clone(), inMapIncRowEqn.clone(), vec1.clone(), iVars.clone(), iEqnsArr.clone(), flag.clone(), n.clone(), iEqnsLst.clone(), iVarsLst.clone(), iAss1.clone(), iAss2.clone())?;
            (oEqnsLst.clone(), oVarsLst.clone(), ass1.clone(), ass2.clone(), eqnarr.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oEqnsLst, oVarsLst, oAss1, oAss2, oEqnsArr))
}

fn getSetStates(mut e: i32, mut flag: metamodelica::Array<bool>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vec2: metamodelica::Array<i32>, mut iStates: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut oStates: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    oStates = List::fold3(inM.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(getSetEqnStates) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<bool>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), flag.clone(), inM.clone(), vec2.clone(), iStates.clone());
    oStates
}

fn getSetEqnStates(mut v: i32, mut flag: metamodelica::Array<bool>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vec2: metamodelica::Array<i32>, mut iStates: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut oStates: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut states: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dstates: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (states, dstates) = iStates.clone();
    states = List::consOnTrue(intLt(vec2.borrow()[(v.clone()-1) as usize].clone(), 1) && flag.borrow()[(v.clone()-1) as usize].clone(), v.clone(), states.clone());
    dstates = List::consOnTrue(intGt(vec2.borrow()[(v.clone()-1) as usize].clone(), 0) && flag.borrow()[(v.clone()-1) as usize].clone(), v.clone(), dstates.clone());
    let _ = {let _arr = flag.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = false; _arr};
    oStates = (states.clone(), dstates.clone());
    Ok(oStates)
}

fn getEqnsforDynamicStateSelection(mut U: Arc<metamodelica::List<i32>>, mut neqns: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    eqns = (::match_deref::match_deref! { match &((U.clone(), neqns.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
            metamodelica::nil()
        },
        (_, _, _, _, _, _, _, _) => {
            let mut colummarks: metamodelica::Array<i32>;
            colummarks = arrayCreate(neqns.clone(), 0);
            getEqnsforDynamicStateSelection1(U.clone(), m.clone(), mT.clone(), 1, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), metamodelica::nil())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqns)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getEqnsforDynamicStateSelection1(mut U: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut inSubset: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outSubset: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outSubset = (::match_deref::match_deref! { match &((U.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _) => {
            inSubset.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _, _, _, _, _, _, _) if (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), 0)) => {
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut set: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut e1: i32 = 0;
            e1 = mapIncRowEqn.borrow()[(e.clone()-1) as usize].clone();
            eqns = mapEqnIncRow.borrow()[(e1.clone()-1) as usize].clone();
            let _ = List::fold1r(eqns.clone(), Arc::new(arrayUpdate.clone()), mark.clone(), colummarks.clone());
            (set, _) = getEqnsforDynamicStateSelectionPhase(eqns.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone(), false)?;
            getEqnsforDynamicStateSelection1(rest.clone(), m.clone(), mT.clone(), mark.clone() + 1, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), set.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _) => {
            getEqnsforDynamicStateSelection1(rest.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubset)
}

fn getEqnsforDynamicStateSelectionPhase(mut elst: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut inSubset: Arc<metamodelica::List<i32>>, mut iFound: bool) -> Result<(Arc<metamodelica::List<i32>>, bool)> {
    let mut outSubset: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oFound: bool = false;
    (outSubset, oFound) = (::match_deref::match_deref! { match &((elst.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone(), iFound.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _) => {
            (inSubset.clone(), iFound.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut set: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut found: bool = false;
            rows = List::select(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
            rows = List::removeOnTrue(ass1.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rows.clone());
            (set, found) = getEqnsforDynamicStateSelectionRows(rows.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone(), false)?;
            set = List::consOnTrue(found.clone(), e.clone(), set.clone());
            {let _arr = colummarks.clone(); let _val = if (found.clone()) {mark.clone()} else {colummarks.borrow()[(e.clone()-1) as usize].clone()}; _arr.borrow_mut()[(e.clone()-1) as usize] = _val; _arr};
            (set, found) = getEqnsforDynamicStateSelectionPhase(rest.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), set.clone(), found.clone() || iFound.clone())?;
            (set.clone(), found.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSubset, oFound))
}

fn getEqnsforDynamicStateSelectionRows(mut rows: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut inSubset: Arc<metamodelica::List<i32>>, mut iFound: bool) -> Result<(Arc<metamodelica::List<i32>>, bool)> {
    let mut outSubset: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oFound: bool = false;
    (outSubset, oFound) = (::match_deref::match_deref! { match &((rows.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone(), iFound.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _) => {
            (inSubset.clone(), iFound.clone())
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _, _) if (!(intGt(ass2.borrow()[(r.clone()-1) as usize].clone(), 0))) => {
            let mut set: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b: bool = false;
            (set, b) = getEqnsforDynamicStateSelectionRows(rest.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone(), true)?;
            (set.clone(), b.clone())
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _, _) if (intGt(ass2.borrow()[(r.clone()-1) as usize].clone(), 0) && intEq(colummarks.borrow()[(ass2.borrow()[(r.clone()-1) as usize].clone()-1) as usize].clone(), 0)) => {
            let mut set: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rc: i32 = 0;
            let mut e: i32 = 0;
            let mut b: bool = false;
            rc = ass2.borrow()[(r.clone()-1) as usize].clone();
            e = mapIncRowEqn.borrow()[(rc.clone()-1) as usize].clone();
            eqns = mapEqnIncRow.borrow()[(e.clone()-1) as usize].clone();
            List::fold1r(eqns.clone(), Arc::new(arrayUpdate.clone()), if (iFound.clone()) {mark.clone()} else {-(mark.clone())}, colummarks.clone());
            (set, b) = getEqnsforDynamicStateSelectionPhase(eqns.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone(), false)?;
            eqns = if (b.clone() && !(iFound.clone())) {eqns.clone()} else {metamodelica::nil()};
            let _ = List::fold1r(eqns.clone(), Arc::new(arrayUpdate.clone()), mark.clone(), colummarks.clone());
            (set, b) = getEqnsforDynamicStateSelectionRows(rest.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), set.clone(), b.clone() || iFound.clone())?;
            (set.clone(), b.clone())
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _, _) if (intGt(ass2.borrow()[(r.clone()-1) as usize].clone(), 0)) => {
            let mut set: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rc: i32 = 0;
            let mut b: bool = false;
            rc = ass2.borrow()[(r.clone()-1) as usize].clone();
            b = intGt(colummarks.borrow()[(rc.clone()-1) as usize].clone(), 0);
            (set, b) = getEqnsforDynamicStateSelectionRows(rest.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset.clone(), b.clone() || iFound.clone())?;
            (set.clone(), b.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSubset, oFound))
}

fn removeFirstOrgEqns(mut inOrgEqns: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>)> {
    let mut outEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outOrgEqns: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut orgEqn: (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    let mut orgeqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut e: i32 = 0;
    let mut numEqs: i32 = 0;
    outOrgEqns = inOrgEqns.clone();
    numEqs = (inOrgEqns.clone().borrow().len() as i32);
    for mut e in 1..=numEqs.clone() {
        orgeqns = outOrgEqns.clone().borrow()[(e.clone()-1) as usize].clone();
        if !(orgeqns.clone().is_empty()) {
            (outEqnsLst, orgeqns) = (::match_deref::match_deref! { match &(orgeqns.clone()) {
        Deref @ metamodelica::List::Cons { head: eqn, tail: Deref @ metamodelica::List::Nil } => {
            (cons(eqn.clone(), outEqnsLst.clone()), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: eqn, tail: eqns } => {
            (cons(eqn.clone(), outEqnsLst.clone()), eqns.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
            {let _arr = outOrgEqns.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = orgeqns.clone(); _arr};
        }
    }
    Ok((outEqnsLst, outOrgEqns))
}

fn sortStateCandidatesVars(mut inVars: BackendDAE::Variables, mut allVars: BackendDAE::Variables, mut m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<BackendDAE::Variables> {
    let mut outStates: BackendDAE::Variables;
    let mut varsize: i32 = 0;
    let mut varIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut prioTuples: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    let mut v: BackendDAE::Var;
    let mut varCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut prio1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut prio2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut prio: metamodelica::Array<metamodelica::Real>;
    let mut index: metamodelica::Array<i32>;
    let mut idx: i32 = 0;
    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut prio_lst: Arc<metamodelica::List<(metamodelica::Real, i32)>> = metamodelica::nil();
    varsize = BackendVariable::varsSize(inVars.clone())?;
    index = arrayCreate(varsize.clone(), -1);
    prio = arrayCreate(varsize.clone(), metamodelica::OrderedFloat(-1.0_f64));
    for mut idx in 1..=varsize.clone() {
        v = BackendVariable::getVarAt(inVars.clone(), idx.clone())?;
        (prio1, prio2) = varStateSelectPrio(v.clone(), allVars.clone(), idx.clone(), m.clone())?;
        {
            let __cell0 = prio1.clone() + prio2.clone();
            prio.clone().borrow_mut()[(idx.clone()-1) as usize] = __cell0;
        }
        {
            let __cell1 = idx.clone();
            index.clone().borrow_mut()[(idx.clone()-1) as usize] = __cell1;
        }
        if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
            varCref = BackendVariable::varCref(v.clone())?;
            BackendDump::debugStrCrefStrRealStrRealStrRealStr((literal!("Calc Prio for ")).clone(), varCref.clone(), (literal!("\n Prio StateSelect : ")).clone(), prio1.clone(), (literal!("\n Prio Heuristik : ")).clone(), prio2.clone(), (literal!("\n ### Prio Result : ")).clone(), prio.borrow()[(idx.clone()-1) as usize].clone(), (literal!("\n")).clone())?;
        }
    }
    prioTuples = {
        let mut __acc: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
        for mut idx in ((1..=varsize.clone()).rev()).into_iter() {
            let __x = (index.borrow()[(idx.clone()-1) as usize].clone(), prio.borrow()[(idx.clone()-1) as usize].clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    prioTuples = List::sort(prioTuples.clone(), (std::sync::Arc::new(fnptr!(sortprioTuples, (i32, metamodelica::Real), (i32, metamodelica::Real))) as std::sync::Arc<dyn ::std::ops::Fn((i32, metamodelica::Real), (i32, metamodelica::Real)) -> Result<bool> + 'static>))?;
    varIndices = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut elem in (prioTuples.clone()).into_iter().cloned() {
            let __x = Util::tuple21(elem.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    vlst = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut idx in (varIndices.clone()).into_iter().cloned() {
            let __x = BackendVariable::getVarAt(inVars.clone(), idx.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outStates = BackendVariable::listVar1(vlst.clone());
    Ok(outStates)
}

fn sortprioTuples(mut inTpl1: (i32, metamodelica::Real), mut inTpl2: (i32, metamodelica::Real)) -> bool {
    let mut b: bool = false;
    b = Util::tuple22(inTpl1.clone()) > Util::tuple22(inTpl2.clone());
    b
}

fn varStateSelectPrio(mut v: BackendDAE::Var, mut vars: BackendDAE::Variables, mut index: i32, mut m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<(metamodelica::Real, metamodelica::Real)> {
    let mut prio_att: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut prio_heu: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    prio_att = varStateSelectPrioAttribute(v.clone())?;
    prio_heu = varStateSelectHeuristicPrio(v.clone(), vars.clone(), index.clone(), m.clone())?;
    Ok((prio_att, prio_heu))
}

fn varStateSelectHeuristicPrio(mut v: BackendDAE::Var, mut vars: BackendDAE::Variables, mut index: i32, mut m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<metamodelica::Real> {
    let mut prio: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut prio1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut prio2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut prio3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut prio4: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut prio5: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut bstart: bool = false;
    let mut bfixed: bool = false;
    bstart = isSome(BackendVariable::varStartValueOption(v.clone())?);
    bfixed = BackendVariable::varFixed(v.clone());
    if bstart.clone() && bfixed.clone() {
        prio1 = metamodelica::OrderedFloat(0.5_f64);
        prio2 = metamodelica::OrderedFloat(0.5_f64);
    } else if bfixed.clone() {
        prio1 = metamodelica::OrderedFloat(0.1_f64);
        prio2 = metamodelica::OrderedFloat(0.5_f64);
    } else if bstart.clone() {
        prio1 = metamodelica::OrderedFloat(0.1_f64);
        prio2 = metamodelica::OrderedFloat(0.0_f64);
    } else {
        prio1 = metamodelica::OrderedFloat(0.0_f64);
        prio2 = metamodelica::OrderedFloat(0.0_f64);
    }
    prio3 = varStateSelectHeuristicPrio3(v.clone())?;
    prio4 = varStateSelectHeuristicPrio4(v.clone(), vars.clone())?;
    prio5 = varStateSelectHeuristicPrio5(v.clone(), index.clone(), m.clone());
    prio = prio1.clone() + prio2.clone() + prio3.clone() + prio4.clone() + prio5.clone();
    printVarListtateSelectHeuristicPrio(prio1.clone(), prio2.clone(), prio3.clone(), prio4.clone(), prio5.clone())?;
    Ok(prio)
}

fn printVarListtateSelectHeuristicPrio(mut Prio1: metamodelica::Real, mut Prio2: metamodelica::Real, mut Prio3: metamodelica::Real, mut Prio4: metamodelica::Real, mut Prio5: metamodelica::Real) -> Result<()> {
    if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Prio 1 : ")); __mm_s.push_str(&*realString(Prio1.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Prio 2 : ")); __mm_s.push_str(&*realString(Prio2.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Prio 3 : ")); __mm_s.push_str(&*realString(Prio3.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Prio 4 : ")); __mm_s.push_str(&*realString(Prio4.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Prio 5 : ")); __mm_s.push_str(&*realString(Prio5.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn varStateSelectHeuristicPrio5(mut v: BackendDAE::Var, mut index: i32, mut om: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> metamodelica::Real {
    let mut prio: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    prio = (match om.clone() {
        None => {
            metamodelica::OrderedFloat(0.0_f64)
        },
        Some(mut m) => {
            let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut n: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            row = m.borrow()[(index.clone()-1) as usize].clone();
            n = intReal((m.clone().borrow().len() as i32)) + metamodelica::OrderedFloat(1.0_f64);
            n = intReal((row.clone().len() as i32)) / n.clone();
            metamodelica::OrderedFloat(0.3_f64) * n.clone()
        },
    });
    prio
}

fn varStateSelectHeuristicPrio4(mut inVar: BackendDAE::Var, mut vars: BackendDAE::Variables) -> Result<metamodelica::Real> {
    let mut prio: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    prio = 'mc: {
        let __mc_input = (inVar.clone(), vars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(cr), .. }, .. }, _) => {
                    let mut v: BackendDAE::Var;
                    let mut b: bool = false;
                    let mut prio: metamodelica::Real = prio.clone();
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    b = BackendVariable::isDummyStateVar(v.clone());
                    prio = if (b.clone()) {metamodelica::OrderedFloat(0.0_f64)} else {metamodelica::OrderedFloat(0.55_f64)};
                    Ok(prio.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::OrderedFloat(0.0_f64))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(prio)
}

fn varStateSelectHeuristicPrio3(mut v: BackendDAE::Var) -> Result<metamodelica::Real> {
    let mut prio: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    prio = 'mc: {
        let __mc_input = v.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Var { varName: ref cr, .. } = __mc_input.clone() else { bail!("nomatch") };
            if !((stringEq((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), (arcstr::literal!(DAE::derivativeNamePrefix)).clone()))) { bail!("guard") }
            Ok(metamodelica::OrderedFloat(-5.0_f64))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::OrderedFloat(0.0_f64))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(prio)
}

fn varStateSelectPrioAttribute(mut v: BackendDAE::Var) -> Result<metamodelica::Real> {
    let mut prio: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut ss: DAE::StateSelect = DAE::StateSelect::ALWAYS;
    ss = BackendVariable::varStateSelect(v.clone());
    prio = (match ss.clone() {
        DAE::StateSelect::NEVER => if (BackendVariable::isArtificialState(v.clone())) {metamodelica::OrderedFloat(-15.0_f64)} else {metamodelica::OrderedFloat(-20.0_f64)},
        DAE::StateSelect::AVOID => metamodelica::OrderedFloat(-1.5_f64),
        DAE::StateSelect::DEFAULT => metamodelica::OrderedFloat(0.0_f64),
        DAE::StateSelect::PREFER => metamodelica::OrderedFloat(1.5_f64),
        DAE::StateSelect::ALWAYS => metamodelica::OrderedFloat(20.0_f64),
        _ => bail!("match: no arm matched"),
    });
    Ok(prio)
}

fn selectDummyDerivatives2new(mut dstates: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>, mut states: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>, mut unassignedEqns: Arc<metamodelica::List<i32>>, mut assignedEqns: Arc<metamodelica::List<i32>>, mut vars: BackendDAE::Variables, mut varSize: i32, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut eqnsSize: i32, mut mapIncRowEqn: metamodelica::Array<i32>, mut level: i32, mut iStateSets: StateSets) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, StateSets)> {
    let mut outDummyVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oStateSets: StateSets = metamodelica::nil();
    (outDummyVars, oStateSets) = 'mc: {
        let __mc_input = dstates.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((intEq((dstates.clone().len() as i32), eqnsSize.clone()))) { bail!("guard") }
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", (literal!("Select as States(1):\n")).clone());
                        BackendDump::debuglst(states.clone(), (std::sync::Arc::new(dumpStates) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!("\n")).clone());
                        println!("{}", (literal!("Select as dummyStates(1):\n")).clone());
                        BackendDump::debuglst(dstates.clone(), (std::sync::Arc::new(dumpStates) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!("\n")).clone());
                    }
                    Ok((metamodelica::nil(), iStateSets.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut statecandidates: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut ovarlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut unassignedEqnsSize: i32 = 0;
                    let mut size: i32 = 0;
                    let mut rang: i32 = 0;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut oeqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut unassignedEqns1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut assignedEqns1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    unassignedEqnsSize = (unassignedEqns.clone().len() as i32);
                    size = (states.clone().len() as i32);
                    rang = size.clone() - unassignedEqnsSize.clone();
                    let true = (intGt(rang.clone(), 0)) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        BackendDump::debugStrIntStrIntStr((literal!("Select ")).clone(), rang.clone(), (literal!(" from ")).clone(), size.clone(), (literal!(" States\n")).clone());
                        BackendDump::debuglst(states.clone(), (std::sync::Arc::new(dumpStates) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!("\n")).clone());
                        println!("{}", (literal!("Select as dummyStates(2):\n")).clone());
                        BackendDump::debuglst(dstates.clone(), (std::sync::Arc::new(dumpStates) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!("\n")).clone());
                    }
                    statecandidates = List::map1r(List::map(states.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _))), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    unassignedEqns1 = List::uniqueIntN(List::map1r(unassignedEqns.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone()), eqnsSize.clone())?;
                    eqnlst = BackendEquation::getList(unassignedEqns1.clone(), eqns.clone());
                    ovarlst = List::map1r(List::map(dstates.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _))), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    assignedEqns1 = List::uniqueIntN(List::map1r(assignedEqns.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone()), eqnsSize.clone())?;
                    oeqnlst = BackendEquation::getList(assignedEqns1.clone(), eqns.clone());
                    varlst = List::map1r(List::map(states.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _))), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    Ok((varlst.clone(), cons((level.clone(), rang.clone(), size.clone(), unassignedEqnsSize.clone(), statecandidates.clone(), eqnlst.clone(), ovarlst.clone(), oeqnlst.clone()), iStateSets.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut unassignedEqnsSize: i32 = 0;
                    let mut size: i32 = 0;
                    let mut rang: i32 = 0;
                    unassignedEqnsSize = (unassignedEqns.clone().len() as i32);
                    size = (states.clone().len() as i32);
                    rang = size.clone() - unassignedEqnsSize.clone();
                    if intLt(rang.clone(), 0) {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Selection of DummyDerivatives failed due to negative system rank of ")); __mm_s.push_str(&*intString(rang.clone())); __mm_s.push_str(&*literal!("!\n           There are ")); __mm_s.push_str(&*intString(unassignedEqnsSize.clone())); __mm_s.push_str(&*literal!(" unassigned equations and ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!(" potential states.\n")); ArcStr::from(__mm_s) }).clone()])?;
                    }
                    let true = (intEq(rang.clone(), 0)) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        println!("{}", (literal!("Select as dummyStates(3):\n")).clone());
                        BackendDump::debuglst(states.clone(), (std::sync::Arc::new(dumpStates) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!("\n")).clone());
                    }
                    varlst = List::map1r(List::map(states.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _))), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    Ok((varlst.clone(), iStateSets.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("- IndexReduction.selectDummyDerivatives2new failed!")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDummyVars, oStateSets))
}

pub fn makeder(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    tp = Expression::r#typeof(inExp.clone())?;
    outExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![inExp.clone()], attr: Arc::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
    Ok(outExp)
}

fn adjacencyMatrixfromEnhancedStrict(mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut vars: BackendDAE::Variables) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    m = Array::map1(me.clone(), (std::sync::Arc::new(fnptr!(adjacencyMatrixElementfromEnhancedStrict, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<i32>>> + 'static>), vars.clone())?;
    Ok(m)
}

fn adjacencyMatrixElementfromEnhancedStrict(mut iRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut vars: BackendDAE::Variables) -> Arc<metamodelica::List<i32>> {
    let mut oRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oRow = List::fold1(iRow.clone(), (std::sync::Arc::new(fnptr!(adjacencyMatrixElementElementfromEnhancedStrict, (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), BackendDAE::Variables, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn((i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), BackendDAE::Variables, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), vars.clone(), metamodelica::nil());
    oRow = List::map(oRow.clone(), Arc::new(intAbs.clone()));
    oRow = oRow.clone().reverse();
    oRow
}

fn adjacencyMatrixElementElementfromEnhancedStrict(mut inTpl: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), mut vars: BackendDAE::Variables, mut iRow: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut oRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oRow = (::match_deref::match_deref! { match &((inTpl.clone(), vars.clone(), iRow.clone())) {
        ((i, BackendDAE::Solvability::SOLVABILITY_SOLVED, _), _, _) => {
            cons(i.clone(), iRow.clone())
        },
        ((i, BackendDAE::Solvability::SOLVABILITY_CONSTONE, _), _, _) => {
            cons(i.clone(), iRow.clone())
        },
        ((i, BackendDAE::Solvability::SOLVABILITY_CONST { .. }, _), _, _) => {
            cons(i.clone(), iRow.clone())
        },
        ((i, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true }, _), _, _) => {
            cons(i.clone(), iRow.clone())
        },
        _ => {
            iRow.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oRow
}

fn adjacencyMatrixfromEnhanced(mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut vars: BackendDAE::Variables, mut so: BackendDAE::StateOrder) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    m = Array::map1(me.clone(), (std::sync::Arc::new(fnptr!(adjacencyMatrixElementfromEnhanced, Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, (BackendDAE::Variables, BackendDAE::StateOrder))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, (BackendDAE::Variables, BackendDAE::StateOrder)) -> Result<Arc<metamodelica::List<i32>>> + 'static>), (vars.clone(), so.clone()))?;
    Ok(m)
}

fn adjacencyMatrixElementfromEnhanced(mut iRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut tpl: (BackendDAE::Variables, BackendDAE::StateOrder)) -> Arc<metamodelica::List<i32>> {
    let mut oRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oRow = List::fold1(iRow.clone(), (std::sync::Arc::new(adjacencyMatrixElementElementfromEnhanced) as std::sync::Arc<dyn ::std::ops::Fn((i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), (BackendDAE::Variables, BackendDAE::StateOrder), Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), tpl.clone(), metamodelica::nil());
    oRow = List::map(oRow.clone(), Arc::new(intAbs.clone()));
    oRow = oRow.clone().reverse();
    oRow
}

fn adjacencyMatrixfromEnhancedPartial(mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut vars: BackendDAE::Variables, mut neverVars: BackendDAE::Variables, mut ass: metamodelica::Array<i32>, mut so: BackendDAE::StateOrder) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    m = Array::map1Ind(me.clone(), (std::sync::Arc::new(adjacencyMatrixElementfromEnhancedPartial) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, i32, (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Array<i32>, BackendDAE::StateOrder)) -> Result<Arc<metamodelica::List<i32>>> + 'static>), (vars.clone(), neverVars.clone(), ass.clone(), so.clone()))?;
    Ok(m)
}

fn adjacencyMatrixElementfromEnhancedPartial(mut iRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut index: i32, mut varsAssTpl: (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Array<i32>, BackendDAE::StateOrder)) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut neverVars: BackendDAE::Variables;
    let mut ass: metamodelica::Array<i32>;
    let mut so: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    (vars, neverVars, ass, so) = varsAssTpl.clone();
    if intEq(ass.borrow()[(index.clone()-1) as usize].clone(), -1) || !(BackendVariable::varStateSelectNever(BackendVariable::getVarAt(vars.clone(), ass.borrow()[(index.clone()-1) as usize].clone())?)) {
        oRow = List::fold1(iRow.clone(), (std::sync::Arc::new(adjacencyMatrixElementElementfromEnhanced) as std::sync::Arc<dyn ::std::ops::Fn((i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), (BackendDAE::Variables, BackendDAE::StateOrder), Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), (neverVars.clone(), so.clone()), metamodelica::nil());
        oRow = List::map(oRow.clone(), Arc::new(intAbs.clone()));
        oRow = oRow.clone().reverse();
    } else {
        oRow = metamodelica::nil();
    }
    Ok(oRow)
}

fn adjacencyMatrixElementElementfromEnhanced(mut inTpl: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>), mut tpl: (BackendDAE::Variables, BackendDAE::StateOrder), mut iRow: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oRow = (::match_deref::match_deref! { match &(inTpl.clone()) {
        (i, BackendDAE::Solvability::SOLVABILITY_SOLVED, _) => {
            cons(i.clone(), iRow.clone())
        },
        (i, BackendDAE::Solvability::SOLVABILITY_CONSTONE, _) => {
            cons(i.clone(), iRow.clone())
        },
        (i, BackendDAE::Solvability::SOLVABILITY_CONST { .. }, _) => {
            cons(i.clone(), iRow.clone())
        },
        (i, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true }, _) => {
            cons(i.clone(), iRow.clone())
        },
        (i, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false }, _) => {
            adjacencyMatrixElementElementfromEnhanced_1(i.clone(), tpl.clone(), iRow.clone())?
        },
        (i, BackendDAE::Solvability::SOLVABILITY_LINEAR { b: true }, _) => {
            adjacencyMatrixElementElementfromEnhanced_1(i.clone(), tpl.clone(), iRow.clone())?
        },
        (i, BackendDAE::Solvability::SOLVABILITY_NONLINEAR, _) => {
            adjacencyMatrixElementElementfromEnhanced_1(i.clone(), tpl.clone(), iRow.clone())?
        },
        _ => {
            iRow.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oRow)
}

fn adjacencyMatrixElementElementfromEnhanced_1(mut i: i32, mut tpl: (BackendDAE::Variables, BackendDAE::StateOrder), mut iRow: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut so: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    let mut v: BackendDAE::Var;
    let mut b: bool = false;
    let (__pa0, ref __pa2 @ BackendDAE::STATEORDER { invHashTable: ref __pa1, .. }) = (tpl.clone()) else { bail!("pattern mismatch") };
    vars = __pa0.clone();
    ht = __pa1.clone();
    so = __pa2.clone();
    v = BackendVariable::getVarAt(vars.clone(), intAbs(i.clone()))?;
    b = BackendVariable::varStateSelectNever(v.clone()) && !(BaseHashTable::hasKey(BackendVariable::varCref(v.clone())?, ht.clone()));
    oRow = List::consOnTrue(b.clone(), i.clone(), iRow.clone());
    Ok(oRow)
}

fn checkAssignment(mut index: i32, mut len: i32, mut ass: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>)> {
    let mut outAssigned: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut outUnassigned: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    for mut indx in index.clone()..=len.clone() {
        let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(vars.clone(), indx.clone())?) else { bail!("pattern mismatch") };
        cr = __pa0.clone();
        if intGt(ass.borrow()[(indx.clone()-1) as usize].clone(), 0) {
            outAssigned = cons((cr.clone(), indx.clone()), outAssigned.clone());
        } else {
            outUnassigned = cons((cr.clone(), indx.clone()), outUnassigned.clone());
        }
    }
    Ok((outAssigned, outUnassigned))
}

fn getLevelStates(mut inVar: BackendDAE::Var, mut level: i32, mut iHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> {
    let mut outVar: BackendDAE::Var;
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    (outVar, oHt) = 'mc: {
        let __mc_input = (inVar.clone(), level.clone(), iHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { encrypted, innerOuter: io, connectorType: ct, comment, hideResult, tearingSelectOption: ts, source, arryDim: dim, varType: tp, varParallelism: prl, varDirection: dir, varKind: BackendDAE::VarKind::STATE { natural, derName: None, index: diffcount }, varName: name, .. }, _, _) => {
                    if !((intGt(diffcount.clone(), 1))) { bail!("guard") }
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut odattr: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut dattr: Arc<DAE::VariableAttributes>;
                    let mut var: BackendDAE::Var;
                    let mut e: Arc<DAE::Exp>;
                    let mut n: i32 = 0;
                    n = diffcount.clone() - level.clone();
                    let true = (intGt(n.clone(), 0)) else { bail!("pattern mismatch") };
                    cr = Util::foldcallN(n.clone(), (std::sync::Arc::new(fnptr!(ComponentReference::crefPrefixDer, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), name.clone());
                    e = Expression::crefExp(cr.clone())?;
                    ht = BaseHashTable::add(((name.clone(), n.clone()), e.clone()), iHt.clone())?;
                    dattr = BackendVariable::getVariableAttributefromType(tp.clone())?;
                    odattr = DAEUtil::setFixedAttr(Some(dattr.clone()), Some(Arc::new(DAE::Exp::BCONST { bool: false })))?;
                    var = BackendDAE::Var { varName: cr.clone(), varKind: BackendDAE::VarKind::STATE { index: 1, derName: None, natural: natural.clone() }, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() };
                    Ok((var.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { natural, derName, index: diffcount }, .. }, _, _) => {
                    if !((intGt(diffcount.clone(), 1))) { bail!("guard") }
                    let mut var: BackendDAE::Var;
                    var = BackendVariable::setVarKind(inVar.clone(), BackendDAE::VarKind::STATE { index: 1, derName: derName.clone(), natural: natural.clone() })?;
                    Ok((var.clone(), iHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), iHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, oHt))
}

fn replaceHigherDerivatives(mut inSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem> = inSystem.clone();
    let mut vars: BackendDAE::Variables;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    let mut dummyvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut addassign: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut nv1: i32 = 0;
    let mut nv: i32 = 0;
    ht = HashTableCrIntToExp::emptyHashTable();
    nv = BackendVariable::varsSize(osyst.orderedVars.clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(osyst.matching.clone()) {
        Deref @ BackendDAE::Matching::MATCHING { ass2: __pa0, ass1: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ass2 = __pa0.clone();
    ass1 = __pa1.clone();
    let (__pa2, (_, _, __pa3, __pa4, __pa5, __pa6)) = BackendVariable::traverseBackendDAEVarsWithUpdate(osyst.orderedVars.clone(), (std::sync::Arc::new(makeHigherStatesRepl) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, i32, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))) -> Result<(BackendDAE::Var, (BackendDAE::Variables, i32, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))))> + 'static>), (osyst.orderedVars.clone(), 1, nv.clone(), metamodelica::nil(), metamodelica::nil(), ht.clone()))?;
    vars = __pa2.clone();
    nv1 = __pa3.clone();
    addassign = __pa4.clone();
    dummyvars = __pa5.clone();
    ht = __pa6.clone();
    dummyvars = dummyvars.clone().reverse();
    vars = BackendVariable::addVars(dummyvars.clone(), vars.clone());
    let (osyst.orderedVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(replaceDummyDerivativesVar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone())?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(osyst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
    ass1 = Array::expand(nv1.clone() - nv.clone(), ass1.clone(), -1)?;
    List::map2_0(addassign.clone(), (std::sync::Arc::new(setHigerDerivativeAssignment) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), metamodelica::Array<i32>, metamodelica::Array<i32>) -> Result<()> + 'static>), ass1.clone(), ass2.clone());
    assign_field!(osyst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() }));
    Ok(osyst)
}

fn setHigerDerivativeAssignment(mut inTpl: (i32, i32), mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut e: i32 = 0;
    (i, j) = inTpl.clone();
    e = ass1.borrow()[(i.clone()-1) as usize].clone();
    let _ = {let _arr = ass1.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = -1; _arr};
    let _ = {let _arr = ass1.clone(); _arr.borrow_mut()[(j.clone()-1) as usize] = e.clone(); _arr};
    let _ = {let _arr = ass2.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = j.clone(); _arr};
    Ok(())
}

fn makeHigherStatesRepl(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, i32, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))) -> Result<(BackendDAE::Var, (BackendDAE::Variables, i32, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))))> {
    let mut outVar: BackendDAE::Var;
    let mut oTpl: (BackendDAE::Variables, i32, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)));
    (outVar, oTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: None, index: diffcount, .. }, varName: name, .. }, (vars, i, j, addassign, varlst, ht)) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut j = (*j).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    let true = (intGt(diffcount.clone(), 1)) else { bail!("pattern mismatch") };
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    (varlst, ht, j) = makeHigherStatesRepl1(diffcount.clone() - 2, 2, name.clone(), cr.clone(), var.clone(), vars.clone(), varlst.clone(), ht.clone(), j.clone())?;
                    Ok((var.clone(), (vars.clone(), i.clone() + 1, j.clone(), cons((i.clone(), j.clone()), addassign.clone()), varlst.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var, (vars, i, j, addassign, varlst, ht)) => {
                    Ok((var.clone(), (vars.clone(), i.clone() + 1, j.clone(), addassign.clone(), varlst.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, oTpl))
}

fn makeHigherStatesRepl1(mut diffCount: i32, mut diffedCount: i32, mut iOrigName: Arc<DAE::ComponentRef>, mut iName: Arc<DAE::ComponentRef>, mut inVar: BackendDAE::Var, mut vars: BackendDAE::Variables, mut iVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)), mut iN: i32) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)), i32)> {
    let mut oVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    let mut oN: i32 = 0;
    (oVarLst, oHt, oN) = (::match_deref::match_deref! { match &((diffCount.clone(), diffedCount.clone(), iOrigName.clone(), iName.clone(), inVar.clone(), vars.clone(), iVarLst.clone(), iHt.clone(), iN.clone())) {
        (_, _, _, _, BackendDAE::Var { encrypted, innerOuter: io, connectorType: ct, comment, hideResult, tearingSelectOption: ts, source, arryDim: dim, varType: tp, varParallelism: prl, varDirection: dir, varName: name, .. }, _, _, _, _) if (intGt(diffCount.clone(), -1)) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
            let mut kind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
            let mut odattr: Option<Arc<DAE::VariableAttributes>> = None;
            let mut dattr: Arc<DAE::VariableAttributes>;
            let mut var: BackendDAE::Var;
            let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut e: Arc<DAE::Exp>;
            let mut n: i32 = 0;
            let mut name = (*name).clone();
            name = ComponentReference::crefPrefixDer(iName.clone());
            e = Expression::crefExp(name.clone())?;
            ht = BaseHashTable::add(((iOrigName.clone(), diffedCount.clone()), e.clone()), iHt.clone())?;
            dattr = BackendVariable::getVariableAttributefromType(tp.clone())?;
            odattr = DAEUtil::setFixedAttr(Some(dattr.clone()), Some(Arc::new(DAE::Exp::BCONST { bool: false })))?;
            odattr = DAEUtil::setProtectedAttr(odattr.clone(), DAEUtil::getProtectedAttr(inVar.values.clone()))?;
            kind = if (intGt(diffCount.clone(), 0)) {BackendDAE::VarKind::STATE { index: diffCount.clone(), derName: None, natural: true }} else {crate::BackendDAE::VarKind::DUMMY_DER};
            var = BackendDAE::Var { varName: name.clone(), varKind: kind.clone(), varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() };
            (vlst, ht, n) = makeHigherStatesRepl1(diffCount.clone() - 1, diffedCount.clone() + 1, iOrigName.clone(), name.clone(), inVar.clone(), vars.clone(), cons(var.clone(), iVarLst.clone()), ht.clone(), iN.clone() + 1)?;
            (vlst.clone(), ht.clone(), n.clone())
        },
        (_, _, _, _, _, _, _, _, _) => {
            (iVarLst.clone(), iHt.clone(), iN.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVarLst, oHt, oN))
}

fn addAllDummyStates(mut inSystem: Arc<BackendDAE::EqSystem>, mut so: BackendDAE::StateOrder, mut iHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> {
    let mut osyst: Arc<BackendDAE::EqSystem> = inSystem.clone();
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    let mut vars: BackendDAE::Variables;
    let mut dummvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let (__pa0, (_, _, __pa1, __pa2)) = BackendVariable::traverseBackendDAEVarsWithUpdate(osyst.orderedVars.clone(), (std::sync::Arc::new(makeAllDummyVarandDummyDerivativeRepl) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::StateOrder, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::StateOrder, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))))> + 'static>), (osyst.orderedVars.clone(), so.clone(), metamodelica::nil(), iHt.clone()))?;
    vars = __pa0.clone();
    dummvars = __pa1.clone();
    oHt = __pa2.clone();
    vars = BackendVariable::addVars(dummvars.clone(), vars.clone());
    let (osyst.orderedVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(replaceDummyDerivativesVar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), oHt.clone())?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(osyst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), oHt.clone()))?;
    Ok((osyst, oHt))
}

fn makeAllDummyVarandDummyDerivativeRepl(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, BackendDAE::StateOrder, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::StateOrder, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))))> {
    let mut outVar: BackendDAE::Var;
    let mut oTpl: (BackendDAE::Variables, BackendDAE::StateOrder, Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)));
    (outVar, oTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS), .. }), varKind: BackendDAE::VarKind::STATE { index: diffcount, .. }, .. }, _) => {
                    if !((intEq(diffcount.clone(), 1))) { bail!("guard") }
                    Ok((var.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS), .. }), varKind: BackendDAE::VarKind::STATE { natural, derName: Some(cr), .. }, .. }, _) => {
                    let mut var = (*var).clone();
                    var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: 1, derName: Some(cr.clone()), natural: natural.clone() })?;
                    Ok((var.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS), .. }), varKind: BackendDAE::VarKind::STATE { natural, derName: None, index: diffcount }, varName: name, .. }, (vars, so, varlst, ht)) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut var = (*var).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    (varlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(diffcount.clone() - 1, 2, name.clone(), cr.clone(), var.clone(), vars.clone(), so.clone(), varlst.clone(), ht.clone())?;
                    var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: natural.clone() })?;
                    Ok((var.clone(), (vars.clone(), so.clone(), varlst.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, varDirection: dir, varParallelism: prl, varType: tp, bindExp: bind, tplExp, arryDim: dim, source, values: attr, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, .. }, (vars, so, varlst, ht)) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut source = (*source).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    (varlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(1, 1, name.clone(), name.clone(), var.clone(), vars.clone(), so.clone(), varlst.clone(), ht.clone())?;
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    source = ElementSource::addSymbolicTransformation(source.clone(), Arc::new(DAE::SymbolicOperation::NEW_DUMMY_DER { chosen: cr.clone(), candidates: metamodelica::nil() }))?;
                    Ok((BackendDAE::Var { varName: name.clone(), varKind: crate::BackendDAE::VarKind::DUMMY_STATE, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bind.clone(), tplExp: tplExp.clone(), arryDim: dim.clone(), source: source.clone(), values: attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: var.encrypted.clone() }, (vars.clone(), so.clone(), varlst.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { derName: None, index: diffcount, .. }, varDirection: dir, varParallelism: prl, varType: tp, bindExp: bind, tplExp, arryDim: dim, source, values: attr, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, .. }, (vars, so, varlst, ht)) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut source = (*source).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    (varlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(diffcount.clone(), 1, name.clone(), name.clone(), var.clone(), vars.clone(), so.clone(), varlst.clone(), ht.clone())?;
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    source = ElementSource::addSymbolicTransformation(source.clone(), Arc::new(DAE::SymbolicOperation::NEW_DUMMY_DER { chosen: cr.clone(), candidates: metamodelica::nil() }))?;
                    Ok((BackendDAE::Var { varName: name.clone(), varKind: crate::BackendDAE::VarKind::DUMMY_STATE, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bind.clone(), tplExp: tplExp.clone(), arryDim: dim.clone(), source: source.clone(), values: attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: var.encrypted.clone() }, (vars.clone(), so.clone(), varlst.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::VARIABLE, varDirection: dir, varParallelism: prl, varType: tp, bindExp: bind, tplExp, arryDim: dim, source, values: attr, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, .. }, (vars, so, varlst, ht)) => {
                    if !((BackendVariable::varStateSelectPrefer(var.clone()))) { bail!("guard") }
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut source = (*source).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    (varlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(1, 1, name.clone(), name.clone(), var.clone(), vars.clone(), so.clone(), varlst.clone(), ht.clone())?;
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    source = ElementSource::addSymbolicTransformation(source.clone(), Arc::new(DAE::SymbolicOperation::NEW_DUMMY_DER { chosen: cr.clone(), candidates: metamodelica::nil() }))?;
                    Ok((BackendDAE::Var { varName: name.clone(), varKind: crate::BackendDAE::VarKind::DUMMY_STATE, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bind.clone(), tplExp: tplExp.clone(), arryDim: dim.clone(), source: source.clone(), values: attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: var.encrypted.clone() }, (vars.clone(), so.clone(), varlst.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, oTpl))
}

fn makeAllDummyVarandDummyDerivativeRepl1(mut diffCount: i32, mut diffedCount: i32, mut iOrigName: Arc<DAE::ComponentRef>, mut iName: Arc<DAE::ComponentRef>, mut inVar: BackendDAE::Var, mut vars: BackendDAE::Variables, mut so: BackendDAE::StateOrder, mut iVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> {
    let mut oVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    (oVarLst, oHt) = (::match_deref::match_deref! { match &((diffCount.clone(), diffedCount.clone(), iOrigName.clone(), iName.clone(), inVar.clone(), vars.clone(), so.clone(), iVarLst.clone(), iHt.clone())) {
        (0, _, _, _, _, _, _, _, _) => {
            (iVarLst.clone(), iHt.clone())
        },
        (_, _, _, _, BackendDAE::Var { encrypted, innerOuter: io, connectorType: ct, comment, hideResult, tearingSelectOption: ts, source, arryDim: dim, varType: tp, varParallelism: prl, varDirection: dir, varName: name, .. }, _, _, _, _) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
            let mut odattr: Option<Arc<DAE::VariableAttributes>> = None;
            let mut dattr: Arc<DAE::VariableAttributes>;
            let mut var: BackendDAE::Var;
            let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut e: Arc<DAE::Exp>;
            let mut name = (*name).clone();
            name = ComponentReference::crefPrefixDer(iName.clone());
            e = Expression::crefExp(name.clone())?;
            ht = BaseHashTable::add(((iOrigName.clone(), diffedCount.clone()), e.clone()), iHt.clone())?;
            dattr = BackendVariable::getVariableAttributefromType(tp.clone())?;
            odattr = DAEUtil::setFixedAttr(Some(dattr.clone()), Some(Arc::new(DAE::Exp::BCONST { bool: false })))?;
            odattr = DAEUtil::setProtectedAttr(odattr.clone(), DAEUtil::getProtectedAttr(inVar.values.clone()))?;
            var = BackendDAE::Var { varName: name.clone(), varKind: crate::BackendDAE::VarKind::DUMMY_DER, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() };
            (vlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(diffCount.clone() - 1, diffedCount.clone() + 1, iOrigName.clone(), name.clone(), inVar.clone(), vars.clone(), so.clone(), cons(var.clone(), iVarLst.clone()), ht.clone())?;
            (vlst.clone(), ht.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("IndexReduction.makeAllDummyVarandDummyDerivativeRepl1 failed!")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVarLst, oHt))
}

fn addDummyStates(mut dummyStates: Arc<metamodelica::List<BackendDAE::Var>>, mut level: i32, mut repl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), mut inSystem: Arc<BackendDAE::EqSystem>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    (osyst, oHt) = (::match_deref::match_deref! { match &((dummyStates.clone(), inSystem.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (inSystem.clone(), iHt.clone())
        },
        (_, syst) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
            let mut vars: BackendDAE::Variables;
            let mut syst = (*syst).clone();
            (vars, ht) = List::fold1(dummyStates.clone(), (std::sync::Arc::new(makeDummyVarandDummyDerivative) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32, (BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))) -> Result<(BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), level.clone(), (syst.orderedVars.clone(), iHt.clone()));
            let (syst.orderedVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(replaceDummyDerivativesVar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone())?;
            let _ = BackendDAEUtil::traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
            let _ = BackendDAEUtil::traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceFirstOrderDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)))> + 'static>), repl.clone()))?;
            (syst.clone(), ht.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oHt))
}

fn makeDummyVarandDummyDerivative(mut inVar: BackendDAE::Var, mut level: i32, mut inTpl: (BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))) -> Result<(BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> {
    let mut oTpl: (BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)));
    oTpl = 'mc: {
        let __mc_input = (inVar.clone(), level.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (BackendDAE::Var { encrypted: mut e, innerOuter: mut io, connectorType: ref ct, comment: mut comment, hideResult: mut hideResult, tearingSelectOption: mut ts, source: mut source, arryDim: ref dim, varType: ref tp, varParallelism: mut prl, varDirection: mut dir, varKind: BackendDAE::VarKind::STATE { index: mut diffindex, .. }, varName: ref name, .. }, _, (mut vars, mut ht)) = __mc_input.clone() else { bail!("nomatch") };
            let mut dummyderName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut odattr: Option<Arc<DAE::VariableAttributes>> = None;
            let mut dattr: Arc<DAE::VariableAttributes>;
            let mut dummy_state: BackendDAE::Var;
            let mut dummy_derstate: BackendDAE::Var;
            let mut dn: i32 = 0;
            let mut kind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
            let mut name = name.clone();
            dn = intMax(diffindex.clone() - level.clone(), 0);
            (name, dummyderName) = crefPrefixDerN(dn.clone(), name.clone());
            let _ = ElementSource::addSymbolicTransformation(source.clone(), Arc::new(DAE::SymbolicOperation::NEW_DUMMY_DER { chosen: dummyderName.clone(), candidates: metamodelica::nil() }))?;
            dattr = BackendVariable::getVariableAttributefromType(tp.clone())?;
            odattr = DAEUtil::setFixedAttr(Some(dattr.clone()), Some(Arc::new(DAE::Exp::BCONST { bool: false })))?;
            odattr = DAEUtil::setProtectedAttr(odattr.clone(), DAEUtil::getProtectedAttr(inVar.values.clone()))?;
            dummy_derstate = BackendDAE::Var { varName: dummyderName.clone(), varKind: crate::BackendDAE::VarKind::DUMMY_DER, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: e.clone() };
            kind = if (intEq(dn.clone(), 0)) {crate::BackendDAE::VarKind::DUMMY_STATE} else {crate::BackendDAE::VarKind::DUMMY_DER};
            dummy_state = BackendDAE::Var { varName: name.clone(), varKind: kind.clone(), varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: e.clone() };
            dummy_state = if (intEq(dn.clone(), 0)) {inVar.clone()} else {dummy_state.clone()};
            dummy_state = BackendVariable::setVarKind(dummy_state.clone(), kind.clone())?;
            vars = BackendVariable::addVar(dummy_derstate.clone(), vars.clone())?;
            vars = BackendVariable::addVar(dummy_state.clone(), vars.clone())?;
            diffindex = dn.clone() + 1;
            (ht, _, _, _) = BaseHashTable::add(((name.clone(), diffindex.clone()), Expression::crefExp(dummyderName.clone())?), ht.clone())?;
            Ok((vars.clone(), ht.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut msg: ArcStr = arcstr::literal!("");
            msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IndexReduction.makeDummyVarandDummyDerivative failed ")); __mm_s.push_str(&*BackendDump::varString(inVar.clone())?); __mm_s.push_str(&*literal!("!")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(msg.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oTpl)
}

fn crefPrefixDerN(mut n: i32, mut iName: Arc<DAE::ComponentRef>) -> (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) {
    let mut oName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut oDerName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (oName, oDerName) = (::match_deref::match_deref! { match &((n.clone(), iName.clone())) {
        (0, _) if (!(intGt(n.clone(), 0))) => {
            let mut dername: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            dername = ComponentReference::crefPrefixDer(iName.clone());
            (iName.clone(), dername.clone())
        },
        _ => {
            let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dername: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            dername = ComponentReference::crefPrefixDer(iName.clone());
            (name, dername) = crefPrefixDerN(n.clone() - 1, dername.clone());
            (name.clone(), dername.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oName, oDerName)
}

fn replaceFirstOrderDerivativesExp(mut inExp: Arc<DAE::Exp>, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    (outExp, ht) = 'mc: {
        let __mc_input = (inExp.clone(), iht.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, ht) => {
                    let mut e: Arc<DAE::Exp>;
                    e = BaseHashTable::get(cr.clone(), ht.clone())?;
                    Ok((e.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), iht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, ht))
}

fn replaceDummyDerivativesExp(mut inExp: Arc<DAE::Exp>, mut iht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    (outExp, ht) = 'mc: {
        let __mc_input = (inExp.clone(), iht.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, ht) => {
                    let mut e: Arc<DAE::Exp>;
                    e = BaseHashTable::get((cr.clone(), i.clone()), ht.clone())?;
                    Ok((e.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, ht) => {
                    let mut e: Arc<DAE::Exp>;
                    e = BaseHashTable::get((cr.clone(), 1), ht.clone())?;
                    Ok((e.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, ht) => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IndexReduction.replaceDummyDerivativesExp failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("!")); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![(msg.clone()).clone()])?;
                    Ok((e.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), iht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, ht))
}

fn replaceDummyDerivatives(mut inSyst: Arc<BackendDAE::EqSystem>, mut ht: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)), mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = inSyst.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut eventInfo: BackendDAE::EventInfo;
    BackendVariable::traverseBackendDAEVarsWithUpdate(outShared.aliasVars.clone(), (std::sync::Arc::new(replaceDummyDerivativesVar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone())?;
    BackendVariable::traverseBackendDAEVarsWithUpdate(outShared.globalKnownVars.clone(), (std::sync::Arc::new(replaceDummyDerivativesVar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone())?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(outShared.initialEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(outSyst.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(outShared.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
    Ok((outSyst, outShared))
}

fn replaceDummyDerivativesVar(mut inVar: BackendDAE::Var, mut inHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> {
    let mut outVar: BackendDAE::Var;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
    (outVar, outHt) = 'mc: {
        let __mc_input = (inVar.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values: attr, bindExp: Some(e), .. }, ht) => {
                    let mut v1: BackendDAE::Var;
                    let mut e1: Arc<DAE::Exp>;
                    let mut attr = (*attr).clone();
                    (e1, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone())?;
                    v1 = BackendVariable::setBindExp(v.clone(), Some(e1.clone()));
                    (attr, _) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
                    v1 = BackendVariable::setVarAttributes(v1.clone(), attr.clone());
                    Ok((v1.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values: attr, .. }, ht) => {
                    let mut v1: BackendDAE::Var;
                    let mut attr = (*attr).clone();
                    (attr, _) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr)))> + 'static>), ht.clone()))?;
                    v1 = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                    Ok((v1.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outHt))
}

pub fn splitEqnsinConstraintAndOther(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outCEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outOEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ne: i32 = 0;
    let mut nv: i32 = 0;
    let mut vec1: metamodelica::Array<i32>;
    let mut vec2: metamodelica::Array<i32>;
    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut assigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
    vars = BackendVariable::listVar1(inVarLst.clone());
    (eqnslst, _) = InlineArrayEquations::getScalarArrayEqns(inEqnsLst.clone())?;
    eqns = BackendEquation::listEquation(eqnslst.clone())?;
    syst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (me, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst.clone(), shared.clone(), false)?;
    m = adjacencyMatrixfromEnhancedStrict(me.clone(), vars.clone())?;
    nv = BackendVariable::varsSize(vars.clone())?;
    ne = BackendEquation::equationArraySize(eqns.clone())?;
    vec1 = arrayCreate(nv.clone(), -1);
    vec2 = arrayCreate(ne.clone(), -1);
    Matching::matchingExternalsetAdjacencyMatrix(nv.clone(), ne.clone(), m.clone());
    BackendDAEEXT::matching(nv.clone(), ne.clone(), 5, -1, metamodelica::OrderedFloat(1.0_f64), 1);
    BackendDAEEXT::getAssignment(vec2.clone(), vec1.clone())?;
    unassigned = Matching::getUnassigned(ne.clone(), vec2.clone(), metamodelica::nil());
    assigned = Matching::getAssigned(ne.clone(), vec2.clone(), metamodelica::nil());
    unassigned = List::map1r(unassigned.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
    unassigned = List::uniqueIntN(unassigned.clone(), ne.clone())?;
    outCEqnsLst = BackendEquation::getList(unassigned.clone(), eqns.clone());
    assigned = List::map1r(assigned.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone());
    assigned = List::uniqueIntN(assigned.clone(), ne.clone())?;
    outOEqnsLst = BackendEquation::getList(assigned.clone(), eqns.clone());
    Ok((outCEqnsLst, outOEqnsLst))
}

fn changeDerVariablesToStatesFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, eqns, ilst, eindx, mapIncRowEqn, mt)) => {
            let mut changedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut vars = (*vars).clone();
            let mut ilst = (*ilst).clone();
            (varlst, changedVars) = BackendVariable::getVar(cr.clone(), vars.clone())?;
            (vars, ilst) = algebraicState(varlst.clone(), changedVars.clone(), vars.clone(), ilst.clone())?;
            (e.clone(), (vars.clone(), eqns.clone(), ilst.clone(), eindx.clone(), mapIncRowEqn.clone(), mt.clone()))
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, eqns, ilst, eindx, mapIncRowEqn, mt)) => {
            let mut changedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut vars = (*vars).clone();
            let mut ilst = (*ilst).clone();
            (varlst, changedVars) = BackendVariable::getVar(cr.clone(), vars.clone())?;
            (vars, ilst) = increaseDifferentiation(varlst.clone(), changedVars.clone(), 2, vars.clone(), ilst.clone())?;
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![e.clone(), Arc::new(DAE::Exp::ICONST { integer: 2 })], attr: DAE::callAttrBuiltinReal().clone() }), (vars.clone(), eqns.clone(), ilst.clone(), eindx.clone(), mapIncRowEqn.clone(), mt.clone()))
        },
        (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: index }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, eqns, ilst, eindx, mapIncRowEqn, mt)) => {
            let mut changedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut vars = (*vars).clone();
            let mut ilst = (*ilst).clone();
            (varlst, changedVars) = BackendVariable::getVar(cr.clone(), vars.clone())?;
            (vars, ilst) = increaseDifferentiation(varlst.clone(), changedVars.clone(), index.clone(), vars.clone(), ilst.clone())?;
            (e.clone(), (vars.clone(), eqns.clone(), ilst.clone(), eindx.clone(), mapIncRowEqn.clone(), mt.clone()))
        },
        _ => {
            (inExp.clone(), inTpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn algebraicState(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inIndxLst: Arc<metamodelica::List<i32>>, mut inVars: BackendDAE::Variables, mut iChangedVars: Arc<metamodelica::List<i32>>) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<i32>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut oChangedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (oVars, oChangedVars) = (::match_deref::match_deref! { match &((inVarLst.clone(), inIndxLst.clone(), inVars.clone(), iChangedVars.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            (inVars.clone(), iChangedVars.clone())
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: _, tail: ilst }, _, _) => {
            let mut changedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vars: BackendDAE::Variables;
            (vars, changedVars) = algebraicState(vlst.clone(), ilst.clone(), inVars.clone(), iChangedVars.clone())?;
            (vars.clone(), changedVars.clone())
        },
        (Deref @ metamodelica::List::Cons { head: v, tail: vlst }, Deref @ metamodelica::List::Cons { head: index, tail: ilst }, _, _) => {
            let mut changedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vars: BackendDAE::Variables;
            let mut v = (*v).clone();
            v = BackendVariable::setVarKind(v.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false })?;
            if !(BackendVariable::varHasStateSelect(v.clone())) {
                v = BackendVariable::setVarStateSelect(v.clone(), openmodelica_frontend_types::DAE::StateSelect::NEVER)?;
            }
            vars = BackendVariable::addVar(v.clone(), inVars.clone())?;
            (vars, changedVars) = algebraicState(vlst.clone(), ilst.clone(), vars.clone(), cons(index.clone(), iChangedVars.clone()))?;
            (vars.clone(), changedVars.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oVars, oChangedVars))
}

fn increaseDifferentiation(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut iVarIndxs: Arc<metamodelica::List<i32>>, mut counter: i32, mut inVars: BackendDAE::Variables, mut iChangedVars: Arc<metamodelica::List<i32>>) -> Result<(BackendDAE::Variables, Arc<metamodelica::List<i32>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut oChangedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (oVars, oChangedVars) = (::match_deref::match_deref! { match &((inVarLst.clone(), iVarIndxs.clone(), counter.clone(), inVars.clone(), iChangedVars.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            (inVars.clone(), iChangedVars.clone())
        },
        (Deref @ metamodelica::List::Cons { head: var @ BackendDAE::Var { .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: i, tail: ilst }, _, _, _) => {
            let mut dcr: Option<Arc<DAE::ComponentRef>> = None;
            let mut vars: BackendDAE::Variables;
            let mut diffcounter: i32 = 0;
            let mut b: bool = false;
            let mut natural: bool = false;
            let mut changedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut var = (*var).clone();
            let mut ilst = (*ilst).clone();
            if BackendVariable::isStateVar(var.clone()) {
                let BackendDAE::STATE { index: __pa0, derName: __pa1, natural: __pa2 } = (var.varKind.clone()) else { bail!("pattern mismatch") };
                diffcounter = __pa0.clone();
                dcr = __pa1.clone();
                natural = __pa2.clone();
            } else {
                (diffcounter, dcr, natural) = (0, None, false);
            }
            b = intGt(counter.clone(), diffcounter.clone());
            diffcounter = if (b.clone()) {counter.clone()} else {diffcounter.clone()};
            var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: diffcounter.clone(), derName: dcr.clone(), natural: natural.clone() })?;
            vars = if (b.clone()) {BackendVariable::addVar(var.clone(), inVars.clone())?} else {inVars.clone()};
            changedVars = List::consOnTrue(b.clone(), i.clone(), iChangedVars.clone());
            (vars, ilst) = increaseDifferentiation(vlst.clone(), ilst.clone(), counter.clone(), vars.clone(), changedVars.clone())?;
            (vars.clone(), ilst.clone())
        },
        _ => {
            println!("{}", (literal!("IndexReduction.increaseDifferentiation failt because of wrong input:\n")).clone());
            BackendDump::printVar(listHead(inVarLst.clone())?)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVars, oChangedVars))
}

fn debugdifferentiateEqns(mut inTpl: (Arc<BackendDAE::Equation>, Arc<BackendDAE::Equation>, i32)) -> Result<()> {
    let mut a: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut b: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut idx: i32 = 0;
    (a, b, idx) = inTpl.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("------------------")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!("------------------\n")); __mm_s.push_str(&*literal!("Constraint equation to be differentiated:\n")); __mm_s.push_str(&*BackendDump::equationString(a.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Differentiated equation:\n")); __mm_s.push_str(&*BackendDump::equationString(b.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn getSetVars(mut index: i32, mut setsize: i32, mut nCandidates: i32, mut nCEqns: i32, mut level: i32) -> Result<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<DAE::Type>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut crstates: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crset: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut oSetVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut ocrA: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut oAVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut realtp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ocrJ: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut oJVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut set: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    set = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$STATESET")); __mm_s.push_str(&*intString(index.clone())); ArcStr::from(__mm_s) }).clone(), DAE::T_COMPLEX_DEFAULT().clone(), metamodelica::nil());
    tp = if (intGt(setsize.clone(), 1)) {Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: setsize.clone() })] })} else {DAE::T_REAL_DEFAULT().clone()};
    crstates = ComponentReference::joinCrefs(set.clone(), ComponentReferenceBasics::makeCrefIdent((literal!("x")).clone(), tp.clone(), metamodelica::nil()))?;
    oSetVars = BackendVariable::generateArrayVar(crstates.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false }, tp.clone(), None)?;
    oSetVars = List::map1(oSetVars.clone(), (std::sync::Arc::new(BackendVariable::setVarFixed) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), false);
    crset = List::map(oSetVars.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
    tp = if (intGt(setsize.clone(), 1)) {Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: setsize.clone() }), Arc::new(DAE::Dimension::DIM_INTEGER { integer: nCandidates.clone() })] })} else {Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nCandidates.clone() })] })};
    realtp = if (intGt(setsize.clone(), 1)) {Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: setsize.clone() }), Arc::new(DAE::Dimension::DIM_INTEGER { integer: nCandidates.clone() })] })} else {Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nCandidates.clone() })] })};
    ocrA = ComponentReference::joinCrefs(set.clone(), ComponentReferenceBasics::makeCrefIdent((literal!("A")).clone(), tp.clone(), metamodelica::nil()))?;
    oAVars = BackendVariable::generateArrayVar(ocrA.clone(), crate::BackendDAE::VarKind::VARIABLE, tp.clone(), None)?;
    oAVars = List::map1(oAVars.clone(), (std::sync::Arc::new(BackendVariable::setVarFixed) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), true);
    oAVars = List::map1(oAVars.clone(), (std::sync::Arc::new(BackendVariable::setVarStartValue) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<DAE::Exp>) -> Result<BackendDAE::Var> + 'static>), Arc::new(DAE::Exp::ICONST { integer: 0 }));
    oAVars = setSetAStart(oAVars.clone(), 1, 1, nCandidates.clone(), metamodelica::nil())?;
    tp = if (intGt(nCEqns.clone(), 1)) {Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nCEqns.clone() })] })} else {DAE::T_REAL_DEFAULT().clone()};
    ocrJ = ComponentReference::joinCrefs(set.clone(), ComponentReferenceBasics::makeCrefIdent((literal!("J")).clone(), tp.clone(), metamodelica::nil()))?;
    oJVars = BackendVariable::generateArrayVar(ocrJ.clone(), crate::BackendDAE::VarKind::VARIABLE, tp.clone(), None)?;
    oJVars = List::map1(oJVars.clone(), (std::sync::Arc::new(BackendVariable::setVarFixed) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), false);
    Ok((crstates, crset, oSetVars, ocrA, oAVars, realtp, ocrJ, oJVars))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn setSetAStart(mut iVars: Arc<metamodelica::List<BackendDAE::Var>>, mut n: i32, mut r: i32, mut nCandidates: i32, mut iAcc: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut oAcc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    oAcc = (::match_deref::match_deref! { match &((iVars.clone(), n.clone(), r.clone(), nCandidates.clone(), iAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            iAcc.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: v, tail: rest }, _, _, _, _) => {
            let mut n1: i32 = 0;
            let mut r1: i32 = 0;
            let mut start: i32 = 0;
            let mut v = (*v).clone();
            start = if (intEq(n.clone(), r.clone())) {1} else {0};
            v = BackendVariable::setVarStartValue(v.clone(), Arc::new(DAE::Exp::ICONST { integer: start.clone() }))?;
            n1 = if (intEq(n.clone(), nCandidates.clone())) {1} else {n.clone() + 1};
            r1 = if (intEq(n.clone(), nCandidates.clone())) {r.clone() + 1} else {r.clone()};
            setSetAStart(rest.clone(), n1.clone(), r1.clone(), nCandidates.clone(), cons(v.clone(), iAcc.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oAcc)
}

// =============================================================================
// set the derivative information to the states
// use equations der(s) = v and set s:STATE(derivativeName=v)
// =============================================================================
pub fn findStateOrder(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    systs = List::map(systs.clone(), (std::sync::Arc::new(findStateOrderWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>));
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

fn findStateOrderWork(mut inSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = inSystem.clone();
    assign_field!(outSystem.orderedVars = BackendEquation::traverseEquationArray(inSystem.orderedEqs.clone(), (std::sync::Arc::new(traverseFindStateOrder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::Variables)> + 'static>), inSystem.orderedVars.clone())?);
    Ok(outSystem)
}

fn traverseFindStateOrder(mut inEq: Arc<BackendDAE::Equation>, mut inVars: BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::Variables)> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outVars: BackendDAE::Variables;
    (outEq, outVars) = 'mc: {
        let __mc_input = (inEq.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, v) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut dvlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut v = (*v).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::derivativeEquation(e.clone())?) {
                        (__pa0, __pa1, _, _, false) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    dcr = __pa1.clone();
                    (vlst, _) = BackendVariable::getVar(cr.clone(), v.clone())?;
                    (dvlst, _) = BackendVariable::getVar(dcr.clone(), v.clone())?;
                    v = addStateOrderFinder(vlst.clone(), dvlst.clone(), v.clone())?;
                    Ok((e.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEq.clone(), inVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEq, outVars))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addStateOrderFinder(mut iVlst: Arc<metamodelica::List<BackendDAE::Var>>, mut iDerVlst: Arc<metamodelica::List<BackendDAE::Var>>, mut inVars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut oVars: BackendDAE::Variables;
    oVars = (::match_deref::match_deref! { match &((iVlst.clone(), iDerVlst.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            inVars.clone()
        },
        (Deref @ metamodelica::List::Cons { head: var @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: dcr, .. }, tail: dvlst }) => {
            let mut vars: BackendDAE::Variables;
            let mut var = (*var).clone();
            var = BackendVariable::setStateDerivative(var.clone(), Some(dcr.clone()))?;
            vars = BackendVariable::addVar(var.clone(), inVars.clone())?;
            addStateOrderFinder(vlst.clone(), dvlst.clone(), vars.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: var, tail: _ }, Deref @ metamodelica::List::Cons { head: dvar, tail: _ }) => {
            let mut msg: ArcStr = arcstr::literal!("");
            msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IndexReduction.addStateOrderFinder failed for ")); __mm_s.push_str(&*BackendDump::varString(var.clone())?); __mm_s.push_str(&*literal!(" with derivative ")); __mm_s.push_str(&*BackendDump::varString(dvar.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(msg.clone()).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("IndexReduction.addStateOrderFinder failed!")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oVars)
}

fn dumpStates(mut state: (Arc<DAE::ComponentRef>, i32)) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(Util::tuple22(state.clone()))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple21(state.clone()))?); ArcStr::from(__mm_s) }).clone();
    Ok(outStr)
}

/* *****************************************
 DAEHandler stuff
 *****************************************/
fn addStateOrder(mut cr: Arc<DAE::ComponentRef>, mut dcr: Arc<DAE::ComponentRef>, mut inStateOrder: BackendDAE::StateOrder) -> Result<BackendDAE::StateOrder> {
    let mut outStateOrder: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    outStateOrder = 'mc: {
        let __mc_input = (cr.clone(), dcr.clone(), inStateOrder.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, BackendDAE::StateOrder::STATEORDER { hashTable: ht, invHashTable: dht }) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut dht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
                    ht1 = BaseHashTable::add((cr.clone(), dcr.clone()), ht.clone())?;
                    if '__try0: {
                        let _ = unwrap_break_err!(getDerStateOrder(dcr.clone(), inStateOrder.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    dht1 = BaseHashTable::add((dcr.clone(), list![cr.clone()]), dht.clone())?;
                    Ok(BackendDAE::StateOrder::STATEORDER { hashTable: ht1.clone(), invHashTable: dht1.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, BackendDAE::StateOrder::STATEORDER { hashTable: ht, invHashTable: dht }) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut dht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    ht1 = BaseHashTable::add((cr.clone(), dcr.clone()), ht.clone())?;
                    crlst = getDerStateOrder(dcr.clone(), inStateOrder.clone())?;
                    dht1 = BaseHashTable::add((dcr.clone(), cons(cr.clone(), crlst.clone())), dht.clone())?;
                    Ok(BackendDAE::StateOrder::STATEORDER { hashTable: ht1.clone(), invHashTable: dht1.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStateOrder)
}

fn getStateOrder(mut cr: Arc<DAE::ComponentRef>, mut inStateOrder: BackendDAE::StateOrder) -> Result<Arc<DAE::ComponentRef>> {
    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let BackendDAE::STATEORDER { hashTable: __pa0, .. } = (inStateOrder.clone()) else { bail!("pattern mismatch") };
    ht = __pa0.clone();
    dcr = BaseHashTable::get(cr.clone(), ht.clone())?;
    Ok(dcr)
}

fn getDerStateOrder(mut dcr: Arc<DAE::ComponentRef>, mut inStateOrder: BackendDAE::StateOrder) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    let BackendDAE::STATEORDER { invHashTable: __pa0, .. } = (inStateOrder.clone()) else { bail!("pattern mismatch") };
    dht = __pa0.clone();
    crlst = BaseHashTable::get(dcr.clone(), dht.clone())?;
    Ok(crlst)
}

fn addOrgEqn(mut e: i32, mut inEqn: Arc<BackendDAE::Equation>, mut inOrgEqns: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> {
    let mut outOrgEqns: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outOrgEqns = inOrgEqns.clone();
    eqs = inOrgEqns.clone().borrow()[(e.clone()-1) as usize].clone();
    eqs = cons(inEqn.clone(), eqs.clone());
    {let _arr = outOrgEqns.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = eqs.clone(); _arr};
    Ok(outOrgEqns)
}

