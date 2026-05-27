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

use openmodelica_frontend_types::DAE;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReplacePattern {
    /// from string (ie \".\"
    pub from: ArcStr,
    /// to string (ie \"$p\") ))
    pub to: ArcStr,
}

pub type REPLACEPATTERN = ReplacePattern;


pub static replaceStringPatterns: std::sync::LazyLock<Arc<metamodelica::List<ReplacePattern>>> = std::sync::LazyLock::new(|| { list![ReplacePattern { from: (literal!(".")).clone(), to: (arcstr::literal!(pointStr)).clone() }, ReplacePattern { from: (literal!("[")).clone(), to: (arcstr::literal!(leftBraketStr)).clone() }, ReplacePattern { from: (literal!("]")).clone(), to: (arcstr::literal!(rightBraketStr)).clone() }, ReplacePattern { from: (literal!("(")).clone(), to: (arcstr::literal!(leftParStr)).clone() }, ReplacePattern { from: (literal!(")")).clone(), to: (arcstr::literal!(rightParStr)).clone() }, ReplacePattern { from: (literal!(",")).clone(), to: (arcstr::literal!(commaStr)).clone() }, ReplacePattern { from: (literal!("'")).clone(), to: (arcstr::literal!(appostrophStr)).clone() }] });

pub const pointStr: &'static str = "$P";

pub const leftBraketStr: &'static str = "$lB";

pub const rightBraketStr: &'static str = "$rB";

pub const leftParStr: &'static str = "$lP";

pub const rightParStr: &'static str = "$rP";

pub const commaStr: &'static str = "$c";

pub const appostrophStr: &'static str = "$a";

pub fn modelicaStringToCStr(mut r#str: ArcStr, mut changeDerCall: bool) -> Result<ArcStr> {
    let mut res_str: ArcStr = arcstr::literal!("");
    res_str = ((match (r#str.clone(), changeDerCall.clone()) {
        (_, false) => {
            res_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*modelicaStringToCStr1((r#str.clone()).clone(), replaceStringPatterns.clone())?); ArcStr::from(__mm_s) }).clone();
            res_str.clone()
        },
        (mut s, true) => {
            s = (modelicaStringToCStr2((s.clone()).clone())?).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(res_str)
}

fn modelicaStringToCStr1(mut inString: ArcStr, mut inReplacePatternLst: Arc<metamodelica::List<ReplacePattern>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inString.clone(), inReplacePatternLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#str, Deref @ metamodelica::List::Nil) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#str, Deref @ metamodelica::List::Cons { head: ReplacePattern { to, from }, tail: res }) => {
                    let mut str_1: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    str_1 = (modelicaStringToCStr1((r#str.clone()).clone(), res.clone())?).clone();
                    res_str = (System::stringReplace((str_1.clone()).clone(), (from.clone()).clone(), (to.clone()).clone())?).clone();
                    Ok(res_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendUtil.modelicaStringToCStr1")); __mm_s.push_str(&*literal!(" failed for str:")); __mm_s.push_str(&*inString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn modelicaStringToCStr2(mut inDerName: ArcStr) -> Result<ArcStr> {
    let mut outDerName: ArcStr = arcstr::literal!("");
    outDerName = ('mc: {
        let __mc_input = inDerName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut derName = __mc_input.clone() else { bail!("nomatch") };
            let mut name: ArcStr = arcstr::literal!("");
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let 0 = (System::strncmp((derName.clone()).clone(), (literal!("der(")).clone(), 4)) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &(System::strtok((derName.clone()).clone(), (literal!("()")).clone())) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            names = __pa0.clone();
            names = List::map1(names.clone(), (std::sync::Arc::new(modelicaStringToCStr) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, bool) -> Result<ArcStr> + 'static>), false);
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(DAE::derivativeNamePrefix)); __mm_s.push_str(&*stringAppendList(names.clone())); ArcStr::from(__mm_s) }).clone();
            Ok(name.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut derName = __mc_input.clone() else { bail!("nomatch") };
            let mut name: ArcStr = arcstr::literal!("");
            let 0 = (System::strncmp((derName.clone()).clone(), (literal!("pre(")).clone(), 4)) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &(System::strtok((derName.clone()).clone(), (literal!("()")).clone())) {
                Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pre(")); __mm_s.push_str(&*modelicaStringToCStr((name.clone()).clone(), false)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            Ok(name.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut derName = __mc_input.clone() else { bail!("nomatch") };
            Ok(modelicaStringToCStr((derName.clone()).clone(), false)?)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outDerName)
}

