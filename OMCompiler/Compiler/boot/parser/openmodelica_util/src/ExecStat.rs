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

use crate::ClockIndexes;
use crate::Error;
use crate::Flags;
use crate::Global;
use crate::StringUtil;
use crate::System;
use openmodelica_util_datatypes_basic::GCExt;

pub const timeFormat: &'static str = "%.4g";

// Why not "%.9f"?
pub const timeMaxLength: i32 = 20;

// Why not 21?
pub const memoryMaxSizeInUnit: i32 = 500;

// Why not 512?
pub const memorySignificantDigits: i32 = 4;

// Why not 3?
pub fn execStatReset() -> Result<()> {
    crate::Globals::gcProfilingIndex.with(|__root| *__root.borrow_mut() = GCExt::getProfStats());
    System::realtimeClear(ClockIndexes::RT_CLOCK_EXECSTAT.clone())?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_EXECSTAT.clone())?;
    Ok(())
}

pub fn execStat(mut name: ArcStr) -> Result<()> {
    fn snprintff(mut val: metamodelica::Real) -> ArcStr {
        let mut r#str: ArcStr = System::snprintff((arcstr::literal!(timeFormat)).clone(), timeMaxLength.clone(), val.clone()).unwrap();
        r#str
    }

    fn bytesToReadableUnit(mut bytes: metamodelica::Real) -> ArcStr {
        let mut r#str: ArcStr = StringUtil::bytesToReadableUnit(bytes.clone(), memorySignificantDigits.clone(), metamodelica::OrderedFloat((memoryMaxSizeInUnit.clone()) as f64));
        r#str
    }

    let mut t: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut total: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeStr: ArcStr = arcstr::literal!("");
    let mut totalTimeStr: ArcStr = arcstr::literal!("");
    let mut gcStr: ArcStr = arcstr::literal!("");
    let mut memory: i32 = 0;
    let mut oldMemory: i32 = 0;
    let mut heapsize_full: i32 = 0;
    let mut free_bytes_full: i32 = 0;
    let mut since: i32 = 0;
    let mut before: i32 = 0;
    let mut stats: GCExt::ProfStats = <GCExt::ProfStats as ::std::default::Default>::default();
    let mut oldStats: GCExt::ProfStats = <GCExt::ProfStats as ::std::default::Default>::default();
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        for mut i in &*if (Flags::isSet(Flags::EXEC_STAT_EXTRA_GC.clone())?) {list![1, 2]} else {list![1]} {
            let mut i = i.clone();
            if i.clone() == 2 {
                GCExt::gcollect();
            }
            t = System::realtimeAccumulate(ClockIndexes::RT_CLOCK_EXECSTAT.clone())?;
            total = System::realtimeAccumulated(ClockIndexes::RT_CLOCK_EXECSTAT.clone())?;
            let ref __pa4 @ GCExt::PROFSTATS { free_bytes_full: ref __pa0, heapsize_full: ref __pa1, allocd_bytes_before_gc: ref __pa2, bytes_allocd_since_gc: ref __pa3, .. } = (GCExt::getProfStats()) else { bail!("pattern mismatch") };
            free_bytes_full = __pa0.clone();
            heapsize_full = __pa1.clone();
            before = __pa2.clone();
            since = __pa3.clone();
            stats = __pa4.clone();
            memory = since.clone() + before.clone();
            oldStats = crate::Globals::gcProfilingIndex.with(|__root| __root.borrow().clone());
            let GCExt::PROFSTATS { allocd_bytes_before_gc: __pa5, bytes_allocd_since_gc: __pa6, .. } = (oldStats.clone()) else { bail!("pattern mismatch") };
            before = __pa5.clone();
            since = __pa6.clone();
            oldMemory = since.clone() + before.clone();
            timeStr = (snprintff(t.clone())).clone();
            totalTimeStr = (snprintff(total.clone())).clone();
            if Flags::isSet(Flags::GC_PROF.clone())? {
                gcStr = (GCExt::profStatsStr(stats.clone(), (literal!("")).clone(), (literal!(" / ")).clone())?).clone();
                Error::addMessage(Error::EXEC_STAT_GC.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*if (i.clone() == 2) {literal!(" GC")} else {literal!("")}); ArcStr::from(__mm_s) }).clone(), (timeStr.clone()).clone(), (totalTimeStr.clone()).clone(), (gcStr.clone()).clone()])?;
            } else {
                Error::addMessage(Error::EXEC_STAT.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*if (i.clone() == 2) {literal!(" GC")} else {literal!("")}); ArcStr::from(__mm_s) }).clone(), (timeStr.clone()).clone(), (totalTimeStr.clone()).clone(), (bytesToReadableUnit(metamodelica::OrderedFloat((memory.clone() - oldMemory.clone()) as f64))).clone(), (bytesToReadableUnit(metamodelica::OrderedFloat((memory.clone()) as f64))).clone(), (bytesToReadableUnit(metamodelica::OrderedFloat((free_bytes_full.clone()) as f64))).clone(), (bytesToReadableUnit(metamodelica::OrderedFloat((heapsize_full.clone()) as f64))).clone()])?;
            }
            crate::Globals::gcProfilingIndex.with(|__root| *__root.borrow_mut() = stats.clone());
            System::realtimeTick(ClockIndexes::RT_CLOCK_EXECSTAT.clone())?;
        }
    }
    Ok(())
}

