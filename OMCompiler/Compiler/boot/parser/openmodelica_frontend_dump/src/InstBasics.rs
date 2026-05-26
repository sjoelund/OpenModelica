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

use crate::SCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;

pub fn commentIsInlineFunc(mut cmt: Arc<SCode::Comment>) -> Result<DAE::InlineType> {
    let mut outInlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    outInlineType = 'mc: {
        let __mc_input = cmt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: smlst, .. } }), .. } => {
                    Ok(isInlineFunc2(smlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInlineType)
}

fn isInlineFunc2(mut inSubModList: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> DAE::InlineType {
    let mut res: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    let mut stop: bool = false;
    res = openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE;
    for mut tp in &*inSubModList.clone() {
        let mut tp = tp.clone();
        stop = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ SCode::SubMod { ident: Deref @ "Inline", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => {
            res = openmodelica_frontend_types::DAE::InlineType::NORM_INLINE;
            false
        },
        Deref @ SCode::SubMod { ident: Deref @ "Inline", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: false }), .. } } => {
            res = openmodelica_frontend_types::DAE::InlineType::NO_INLINE;
            false
        },
        Deref @ SCode::SubMod { ident: Deref @ "LateInline", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => {
            res = openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE;
            true
        },
        Deref @ SCode::SubMod { ident: Deref @ "__MathCore_InlineAfterIndexReduction", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => {
            res = openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE;
            true
        },
        Deref @ SCode::SubMod { ident: Deref @ "__Dymola_InlineAfterIndexReduction", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => {
            res = openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE;
            true
        },
        Deref @ SCode::SubMod { ident: Deref @ "InlineAfterIndexReduction", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => {
            res = openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE;
            true
        },
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_EarlyInline", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => {
            res = openmodelica_frontend_types::DAE::InlineType::EARLY_INLINE;
            true
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if stop.clone() {
            break;
        }
    }
    res
}

pub fn commentGenerateEvents(mut cmt: Arc<SCode::Comment>) -> bool {
    fn commentGenerateEvents2(mut inSubModList: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> bool {
        let mut res: bool = false;
        let mut stop: bool = false;
        res = false;
        for mut tp in &*inSubModList.clone() {
            let mut tp = tp.clone();
            stop = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ SCode::SubMod { ident: Deref @ "GenerateEvents", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: res }), .. } } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if stop.clone() {
                break;
            }
        }
        res
    }

    let mut generateEvents: bool = false;
    generateEvents = (::match_deref::match_deref! { match &(cmt.clone()) {
        Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: smlst, .. } }), .. } => {
            commentGenerateEvents2(smlst.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    generateEvents
}

pub fn getFunctionRestrictionPurity(mut purity: Absyn::FunctionPurity, mut cmt: Arc<SCode::Comment>, mut newFrontend: bool) -> Result<DAE::Purity> {
    let mut outPurity: DAE::Purity = DAE::Purity::PURE;
    outPurity = (match purity.clone() {
        Absyn::FunctionPurity::PURE { .. } => DAE::Purity::PURE.clone(),
        Absyn::FunctionPurity::IMPURE { .. } => DAE::Purity::IMPURE.clone(),
        _ => DAE::Purity::UNDEFINED.clone(),
    });
    if outPurity.clone() == DAE::Purity::UNDEFINED.clone() {
        if SCodeUtil::commentHasBooleanNamedAnnotation(cmt.clone(), (literal!("__ModelicaAssociation_Impure")).clone())? {
            outPurity = DAE::Purity::IMPURE.clone();
        } else if !(newFrontend.clone()) && SCodeUtil::commentHasBooleanNamedAnnotation(cmt.clone(), (literal!("__OpenModelica_Impure")).clone())? {
            outPurity = DAE::Purity::OM_IMPURE.clone();
        }
    }
    Ok(outPurity)
}

