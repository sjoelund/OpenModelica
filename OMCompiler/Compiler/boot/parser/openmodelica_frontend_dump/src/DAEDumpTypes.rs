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

use crate::SCodeDump;
use crate::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::System;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct splitElements {
    pub v: Arc<metamodelica::List<Arc<DAE::Element>>>,
    pub ie: Arc<metamodelica::List<Arc<DAE::Element>>>,
    pub ia: Arc<metamodelica::List<Arc<DAE::Element>>>,
    pub e: Arc<metamodelica::List<Arc<DAE::Element>>>,
    pub a: Arc<metamodelica::List<Arc<DAE::Element>>>,
    pub co: Arc<metamodelica::List<Arc<DAE::Element>>>,
    pub o: Arc<metamodelica::List<Arc<DAE::Element>>>,
    pub ca: Arc<metamodelica::List<Arc<DAE::Element>>>,
    pub sm: Arc<metamodelica::List<Arc<compWithSplitElements>>>,
}

impl Default for splitElements {
    fn default() -> Self {
        Self {
            v: Default::default(),
            ie: Default::default(),
            ia: Default::default(),
            e: Default::default(),
            a: Default::default(),
            co: Default::default(),
            o: Default::default(),
            ca: Default::default(),
            sm: Default::default(),
        }
    }
}

pub type SPLIT_ELEMENTS = splitElements;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct compWithSplitElements {
    pub name: ArcStr,
    pub spltElems: Arc<splitElements>,
    pub comment: Option<Arc<SCode::Comment>>,
}

impl Default for compWithSplitElements {
    fn default() -> Self {
        Self {
            name: Default::default(),
            spltElems: Default::default(),
            comment: Default::default(),
        }
    }
}

pub type COMP_WITH_SPLIT = compWithSplitElements;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct functionList {
    pub funcs: Arc<metamodelica::List<DAE::Function>>,
}

impl Default for functionList {
    fn default() -> Self {
        Self {
            funcs: Default::default(),
        }
    }
}

pub type FUNCTION_LIST = functionList;


pub fn dumpCommentStr(mut inComment: Option<Arc<SCode::Comment>>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inComment.clone()) {
        Some(Deref @ SCode::Comment { comment: Some(cmt), .. }) => {
            let mut cmt = (*cmt).clone();
            cmt = (System::escapedString((cmt.clone()).clone(), false)).clone();
            stringAppendList(list![(literal!(" \"")).clone(), (cmt.clone()).clone(), (literal!("\"")).clone()])
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

pub fn dumpClassAnnotationStr(mut inComment: Option<Arc<SCode::Comment>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (dumpAnnotationStr(inComment.clone(), (literal!("  ")).clone(), (literal!(";\n")).clone())?).clone();
    Ok(outString)
}

pub fn dumpCommentAnnotationStr(mut inComment: Option<Arc<SCode::Comment>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inComment.clone()) {
        None => literal!(""),
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*dumpCommentStr(inComment.clone())); __mm_s.push_str(&*dumpCompAnnotationStr(inComment.clone())?); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn dumpCompAnnotationStr(mut inComment: Option<Arc<SCode::Comment>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (dumpAnnotationStr(inComment.clone(), (literal!(" ")).clone(), (literal!("")).clone())?).clone();
    Ok(outString)
}

fn dumpAnnotationStr(mut inComment: Option<Arc<SCode::Comment>>, mut inPrefix: ArcStr, mut inSuffix: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inComment.clone(), inPrefix.clone(), inSuffix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: ann_mod }), .. }), _, _) => {
                    let mut ann: ArcStr = arcstr::literal!("");
                    let mut ann_mod = (*ann_mod).clone();
                    if Config::showAnnotations()? {
                        ann = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*literal!("annotation")); __mm_s.push_str(&*SCodeDump::printModStr(ann_mod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*inSuffix.clone()); ArcStr::from(__mm_s) }).clone();
                    } else if Config::showStructuralAnnotations()? {
                        ann_mod = filterStructuralMods(ann_mod.clone());
                        if !(SCodeUtil::isEmptyMod(ann_mod.clone())) {
                            ann = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*literal!("annotation")); __mm_s.push_str(&*SCodeDump::printModStr(ann_mod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*inSuffix.clone()); ArcStr::from(__mm_s) }).clone();
                        } else {
                            ann = (literal!("")).clone();
                        }
                    } else {
                        ann = (literal!("")).clone();
                    }
                    Ok(ann.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn filterStructuralMods(mut r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    r#mod = SCodeUtil::filterSubMods(r#mod.clone(), (std::sync::Arc::new(fnptr!(filterStructuralMod, Arc<SCode::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>));
    r#mod
}

fn filterStructuralMod(mut r#mod: Arc<SCode::SubMod>) -> bool {
    let mut keep: bool = false;
    keep = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "Evaluate" => true,
        Deref @ "Inline" => true,
        Deref @ "LateInline" => true,
        Deref @ "derivative" => true,
        Deref @ "inverse" => true,
        Deref @ "smoothOrder" => true,
        Deref @ "InlineAfterIndexReduction" => true,
        Deref @ "GenerateEvents" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    keep
}

