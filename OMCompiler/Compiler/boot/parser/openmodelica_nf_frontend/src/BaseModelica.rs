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

use openmodelica_util::Flags;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ScalarizeMode {
    SCALARIZED = 1,
    PARTIALLY_SCALARIZED = 2,
    NOT_SCALARIZED = 3,
}
impl PartialOrd for ScalarizeMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ScalarizeMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for ScalarizeMode {
    fn default() -> Self { Self::SCALARIZED }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum RecordMode {
    WITH_RECORDS = 1,
    WITHOUT_RECORDS = 2,
}
impl PartialOrd for RecordMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for RecordMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for RecordMode {
    fn default() -> Self { Self::WITH_RECORDS }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputFormat {
    pub scalarizeMode: ScalarizeMode,
    pub recordMode: RecordMode,
    pub moveBindings: bool,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self {
            scalarizeMode: Default::default(),
            recordMode: Default::default(),
            moveBindings: Default::default(),
        }
    }
}

pub type OUTPUT_FORMAT = OutputFormat;


pub static defaultFormat: std::sync::LazyLock<OutputFormat> = std::sync::LazyLock::new(|| { OutputFormat { scalarizeMode: ScalarizeMode::PARTIALLY_SCALARIZED.clone(), recordMode: RecordMode::WITH_RECORDS.clone(), moveBindings: false } });

pub fn formatFromFlags() -> Result<OutputFormat> {
    let mut format: OutputFormat = defaultFormat.clone();
    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        format.scalarizeMode = ScalarizeMode::NOT_SCALARIZED.clone();
    } else if Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), (literal!("scalarize")).clone())? {
        format.scalarizeMode = ScalarizeMode::SCALARIZED.clone();
        format.recordMode = RecordMode::WITHOUT_RECORDS.clone();
    }
    for mut option in &*Flags::getConfigStringList(Flags::BASE_MODELICA_FORMAT.clone())? {
        let mut option = option.clone();
        let () = (::match_deref::match_deref! { match &(option.clone()) {
        Deref @ "scalarized" => {
            format.scalarizeMode = ScalarizeMode::SCALARIZED.clone();
            ()
        },
        Deref @ "partiallyScalarized" => {
            format.scalarizeMode = ScalarizeMode::PARTIALLY_SCALARIZED.clone();
            ()
        },
        Deref @ "nonScalarized" => {
            format.scalarizeMode = ScalarizeMode::NOT_SCALARIZED.clone();
            ()
        },
        Deref @ "withRecords" => {
            format.recordMode = RecordMode::WITH_RECORDS.clone();
            ()
        },
        Deref @ "withoutRecords" => {
            format.recordMode = RecordMode::WITHOUT_RECORDS.clone();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    format.moveBindings = Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), (literal!("moveBindings")).clone())?;
    Ok(format)
}

pub fn inlineFunctions() -> bool {
    let mut enabled: bool = Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), (literal!("inlineFunctions")).clone()).unwrap();
    enabled
}

