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

use crate::HpcOmBenchmarkExt;
use openmodelica_util::System;

pub fn benchSystem() -> Result<((i32, i32), (i32, i32))> {
    let mut oTime: ((i32, i32), (i32, i32));
    let mut comCostM: i32 = 0;
    let mut comCostN: i32 = 0;
    let mut opCostM: i32 = 0;
    let mut opCostN: i32 = 0;
    let mut opCosts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut comCosts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    opCosts = HpcOmBenchmarkExt::requiredTimeForOp()?;
    let true = ((opCosts.clone().len() as i32) == 2) else { bail!("pattern mismatch") };
    opCostM = (opCosts.clone()).get(1)?;
    opCostN = (opCosts.clone()).get(2)?;
    s1 = (intString(opCostM.clone())).clone();
    s2 = (intString(opCostN.clone())).clone();
    comCosts = HpcOmBenchmarkExt::requiredTimeForComm()?;
    comCostM = (comCosts.clone()).get(1)?;
    comCostN = (comCosts.clone()).get(2)?;
    s1 = (intString(comCostM.clone())).clone();
    s2 = (intString(comCostN.clone())).clone();
    oTime = ((opCostM.clone(), opCostN.clone()), (comCostM.clone(), comCostN.clone()));
    Ok(oTime)
}

pub fn readCalcTimesFromFile(mut iFileNamePrefix: ArcStr) -> Result<Arc<metamodelica::List<(i32, i32, metamodelica::Real)>>> {
    let mut calcTimes: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
    let mut fullFileName: ArcStr = arcstr::literal!("");
    let mut tmpCalcTimes: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
    calcTimes = 'mc: {
        let __mc_input = iFileNamePrefix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut tmpCalcTimes: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = tmpCalcTimes.clone();
            let mut fullFileName: ArcStr = fullFileName.clone();
            fullFileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iFileNamePrefix.clone()); __mm_s.push_str(&*literal!(".json")); ArcStr::from(__mm_s) }).clone();
            let Some(_) = (System::getFileModificationTime((fullFileName.clone()).clone())) else { bail!("pattern mismatch") };
            println!("{}", (literal!("Using json-file\n")).clone());
            tmpCalcTimes = readCalcTimesFromJson((fullFileName.clone()).clone())?;
            Ok(tmpCalcTimes.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut fullFileName: ArcStr = fullFileName.clone();
            let mut tmpCalcTimes: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = tmpCalcTimes.clone();
            fullFileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iFileNamePrefix.clone()); __mm_s.push_str(&*literal!(".xml")); ArcStr::from(__mm_s) }).clone();
            let Some(_) = (System::getFileModificationTime((fullFileName.clone()).clone())) else { bail!("pattern mismatch") };
            tmpCalcTimes = readCalcTimesFromXml((fullFileName.clone()).clone())?;
            Ok(tmpCalcTimes.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("readCalcTimesFromFile: No valid profiling-file found.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(calcTimes)
}

fn readCalcTimesFromXml(mut fileName: ArcStr) -> Result<Arc<metamodelica::List<(i32, i32, metamodelica::Real)>>> {
    let mut calcTimes: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
    let mut tmpResult: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    tmpResult = HpcOmBenchmarkExt::readCalcTimesFromXml((fileName.clone()).clone())?;
    calcTimes = expandCalcTimes(tmpResult.clone(), metamodelica::nil())?;
    Ok(calcTimes)
}

fn readCalcTimesFromJson(mut fileName: ArcStr) -> Result<Arc<metamodelica::List<(i32, i32, metamodelica::Real)>>> {
    let mut calcTimes: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
    let mut tmpResult: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    tmpResult = HpcOmBenchmarkExt::readCalcTimesFromJson((fileName.clone()).clone())?;
    calcTimes = expandCalcTimes(tmpResult.clone(), metamodelica::nil())?;
    Ok(calcTimes)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn expandCalcTimes(mut iList: Arc<metamodelica::List<metamodelica::Real>>, mut iTuples: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>>) -> Result<Arc<metamodelica::List<(i32, i32, metamodelica::Real)>>> {
    let mut oTuples: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
    let mut eqIdx: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut numOfCalcs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut calcTimeSum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut intNumOfCalcs: i32 = 0;
    let mut intEqIdx: i32 = 0;
    let mut rest: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut tmpTuples: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
    oTuples = 'mc: {
        let __mc_input = (iList.clone(), iTuples.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: numOfCalcs, tail: Deref @ metamodelica::List::Cons { head: calcTimeSum, tail: Deref @ metamodelica::List::Cons { head: eqIdx, tail: rest } } }, _) => {
                    let mut tmpTuples: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = tmpTuples.clone();
                    let mut intNumOfCalcs: i32 = intNumOfCalcs.clone();
                    let mut intEqIdx: i32 = intEqIdx.clone();
                    intNumOfCalcs = ((numOfCalcs.clone()).0 as i32);
                    intEqIdx = ((eqIdx.clone()).0 as i32);
                    tmpTuples = expandCalcTimes(rest.clone(), cons((intEqIdx.clone(), intNumOfCalcs.clone(), calcTimeSum.clone()), iTuples.clone()))?;
                    Ok(tmpTuples.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(iTuples.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("expandCalcTimes: Invalid number of list-entries\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oTuples)
}

