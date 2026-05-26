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

use crate::Autoconf;
use crate::Flags;
use crate::System;

pub fn isRunning() -> Result<bool> {
    let mut runningTestsuite: bool = false;
    runningTestsuite = !(stringEq((Flags::getConfigString(Flags::RUNNING_TESTSUITE.clone())?).clone(), (literal!("")).clone()));
    Ok(runningTestsuite)
}

pub fn getTempFilesFile() -> Result<ArcStr> {
    let mut tempFile: ArcStr = arcstr::literal!("");
    tempFile = (Flags::getConfigString(Flags::RUNNING_TESTSUITE.clone())?).clone();
    Ok(tempFile)
}

pub fn friendly(mut name: ArcStr) -> Result<ArcStr> {
    let mut friendly: ArcStr = arcstr::literal!("");
    friendly = (friendly2(isRunning()?, (name.clone()).clone())?).clone();
    Ok(friendly)
}

fn friendly2(mut cond: bool, mut name: ArcStr) -> Result<ArcStr> {
    let mut friendly: ArcStr = arcstr::literal!("");
    friendly = ((match cond.clone() {
        true => {
            let mut i: i32 = 0;
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut newName: ArcStr = arcstr::literal!("");
            newName = (if (arcstr::literal!(Autoconf::os) == literal!("Windows_NT")) {System::stringReplace((name.clone()).clone(), (literal!("\\")).clone(), (literal!("/")).clone())?} else {name.clone()}).clone();
            (i, strs) = System::regex((newName.clone()).clone(), (literal!("^(.*/Compiler/)?(.*/testsuite/)?(.*/.openmodelica/libraries/)?(.*/lib/omlibrary/)?(.*/build/(install_cmake/)?)?(.*)$")).clone(), 8, true, false);
            friendly = ((strs.clone()).get(i.clone())?).clone();
            (i, strs) = System::regex((friendly.clone()).clone(), (literal!("^(.*)(/[_[:alnum:]]*\\.mos?_temp[0-9]*)(.*)$")).clone(), 4, true, false);
            if i.clone() == 4 {
                friendly = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*(strs.clone()).get(2)?); __mm_s.push_str(&*(strs.clone()).get(4)?); ArcStr::from(__mm_s) }).clone();
            }
            friendly.clone()
        },
        _ => {
            name.clone()
        },
    })).clone();
    Ok(friendly)
}

pub fn friendlyPath(mut inPath: ArcStr) -> Result<ArcStr> {
    let mut outPath: ArcStr = arcstr::literal!("");
    outPath = ('mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut path: ArcStr = arcstr::literal!("");
            let true = (isRunning()?) else { bail!("pattern mismatch") };
            let false = (System::directoryExists((inPath.clone()).clone())) else { bail!("pattern mismatch") };
            let false = (System::regularFileExists((inPath.clone()).clone())) else { bail!("pattern mismatch") };
            path = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("../")); __mm_s.push_str(&*inPath.clone()); ArcStr::from(__mm_s) }).clone();
            let true = (System::directoryExists((path.clone()).clone()) || System::regularFileExists((path.clone()).clone())) else { bail!("pattern mismatch") };
            Ok(path.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inPath.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outPath)
}

