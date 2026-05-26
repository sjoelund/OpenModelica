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

pub const RT_NO_CLOCK: i32 = -1;

pub const RT_CLOCK_SIMULATE_TOTAL: i32 = 8;

pub const RT_CLOCK_SIMULATE_SIMULATION: i32 = 9;

pub const RT_CLOCK_BUILD_MODEL: i32 = 10;

pub const RT_CLOCK_EXECSTAT: i32 = 11;

pub const RT_CLOCK_FRONTEND: i32 = 13;

pub const RT_CLOCK_BACKEND: i32 = 14;

pub const RT_CLOCK_SIMCODE: i32 = 15;

pub const RT_CLOCK_LINEARIZE: i32 = 16;

pub const RT_CLOCK_TEMPLATES: i32 = 17;

pub const RT_CLOCK_UNCERTAINTIES: i32 = 18;

pub const RT_PROFILER0: i32 = 19;

pub const RT_PROFILER1: i32 = 20;

pub const RT_PROFILER2: i32 = 21;

pub const RT_CLOCK_EXECSTAT_JACOBIANS: i32 = 22;

pub const RT_CLOCK_USER_RESERVED: i32 = 23;

pub const RT_CLOCK_EXECSTAT_HPCOM_MODULES: i32 = 24;

pub const RT_CLOCK_SHOW_STATEMENT: i32 = 25;

pub const RT_CLOCK_FINST: i32 = 26;

pub const RT_CLOCK_NEW_BACKEND_MODULE: i32 = 29;

pub const RT_CLOCK_NEW_BACKEND_INITIALIZATION: i32 = 30;

pub static buildModelClocks: std::sync::LazyLock<Arc<metamodelica::List<i32>>> = std::sync::LazyLock::new(|| { list![RT_CLOCK_BUILD_MODEL.clone(), RT_CLOCK_SIMULATE_TOTAL.clone(), RT_CLOCK_TEMPLATES.clone(), RT_CLOCK_LINEARIZE.clone(), RT_CLOCK_SIMCODE.clone(), RT_CLOCK_BACKEND.clone(), RT_CLOCK_FRONTEND.clone()] });

pub fn toString(mut clockIndex: i32) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match clockIndex.clone() {
        RT_NO_CLOCK => literal!("NON"),
        RT_CLOCK_SIMULATE_TOTAL => literal!("STO"),
        RT_CLOCK_SIMULATE_SIMULATION => literal!("SSI"),
        RT_CLOCK_BUILD_MODEL => literal!("BLD"),
        RT_CLOCK_EXECSTAT => literal!("EXS"),
        RT_CLOCK_FRONTEND => literal!("FRT"),
        RT_CLOCK_BACKEND => literal!("BCK"),
        RT_CLOCK_SIMCODE => literal!("SCD"),
        RT_CLOCK_LINEARIZE => literal!("LIN"),
        RT_CLOCK_TEMPLATES => literal!("TMP"),
        RT_CLOCK_UNCERTAINTIES => literal!("UNC"),
        RT_PROFILER0 => literal!("PR0"),
        RT_PROFILER1 => literal!("PR1"),
        RT_PROFILER2 => literal!("PR2"),
        RT_CLOCK_EXECSTAT_JACOBIANS => literal!("JAC"),
        RT_CLOCK_USER_RESERVED => literal!("RES"),
        RT_CLOCK_EXECSTAT_HPCOM_MODULES => literal!("HPC"),
        RT_CLOCK_SHOW_STATEMENT => literal!("STM"),
        RT_CLOCK_FINST => literal!("FIN"),
        RT_CLOCK_NEW_BACKEND_MODULE => literal!("SIM"),
        RT_CLOCK_NEW_BACKEND_INITIALIZATION => literal!("INI"),
        _ => literal!("ERR"),
    })).clone();
    r#str
}

