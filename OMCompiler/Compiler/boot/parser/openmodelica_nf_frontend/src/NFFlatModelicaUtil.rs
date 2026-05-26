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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::Util;

// Used to indicate what type of element an annotation comes from, to allow
// filtering out specific annotations for dumping.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ElementType {
    ROOT_CLASS = 1,
    CLASS = 2,
    FUNCTION = 3,
    COMPONENT = 4,
    EQUATION = 5,
    ALGORITHM = 6,
    OTHER = 7,
}
impl PartialOrd for ElementType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ElementType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn appendElementSourceCommentString(mut source: Arc<DAE::ElementSource>, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut opt_cmt: Option<Arc<SCode::Comment>> = None;
    opt_cmt = ElementSource::getOptComment(source.clone())?;
    if isSome(opt_cmt.clone()) {
        s = appendCommentString(Util::getOption(opt_cmt.clone())?, s.clone())?;
    }
    Ok(s)
}

pub fn appendElementSourceCommentAnnotation(mut source: Arc<DAE::ElementSource>, mut elementType: ElementType, mut indent: ArcStr, mut ending: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut opt_cmt: Option<Arc<SCode::Comment>> = None;
    opt_cmt = ElementSource::getOptComment(source.clone())?;
    if isSome(opt_cmt.clone()) {
        s = appendCommentAnnotation(Util::getOption(opt_cmt.clone())?, elementType.clone(), (indent.clone()).clone(), (ending.clone()).clone(), s.clone())?;
    }
    Ok(s)
}

pub fn appendElementSourceComment(mut source: Arc<DAE::ElementSource>, mut elementType: ElementType, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    s = appendCommentOpt(ElementSource::getOptComment(source.clone())?, elementType.clone(), s.clone())?;
    Ok(s)
}

pub fn appendCommentOpt(mut comment: Option<Arc<SCode::Comment>>, mut elementType: ElementType, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    if isSome(comment.clone()) {
        s = appendComment(Util::getOption(comment.clone())?, elementType.clone(), s.clone())?;
    }
    Ok(s)
}

pub fn appendComment(mut comment: Arc<SCode::Comment>, mut elementType: ElementType, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    s = appendCommentString(comment.clone(), s.clone())?;
    s = appendCommentAnnotation(comment.clone(), elementType.clone(), (literal!(" ")).clone(), (literal!("")).clone(), s.clone())?;
    Ok(s)
}

pub fn appendCommentString(mut comment: Arc<SCode::Comment>, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut r#str: ArcStr = arcstr::literal!("");
    let () = (::match_deref::match_deref! { match &(comment.clone()) {
        Deref @ SCode::Comment { comment: Some(r#str), .. } => {
            s = IOStream::append(s.clone(), (literal!(" \"")).clone())?;
            s = IOStream::append(s.clone(), (System::escapedString((r#str.clone()).clone(), false)).clone())?;
            s = IOStream::append(s.clone(), (literal!("\"")).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn appendCommentAnnotation(mut comment: Arc<SCode::Comment>, mut elementType: ElementType, mut indent: ArcStr, mut ending: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let () = (::match_deref::match_deref! { match &(comment.clone()) {
        Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: r#mod }), .. } => {
            let mut r#mod = (*r#mod).clone();
            r#mod = (match elementType.clone() {
        ElementType::ROOT_CLASS { .. } => filterRootClassAnnotations(r#mod.clone()),
        _ => DAEDumpTypes::filterStructuralMods(r#mod.clone()),
    });
            if !(SCodeUtil::isEmptyMod(r#mod.clone())) {
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                s = IOStream::append(s.clone(), (literal!("annotation")).clone())?;
                s = appendAnnotationMod(r#mod.clone(), s.clone())?;
                s = IOStream::append(s.clone(), (ending.clone()).clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn filterRootClassAnnotations(mut r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    fn filter(mut smod: Arc<SCode::SubMod>) -> bool {
        let mut keep: bool = false;
        keep = (::match_deref::match_deref! { match &(smod.ident.clone()) {
        Deref @ "experiment" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        keep
    }

    let mut r#mod: Arc<SCode::Mod> = r#mod;
    r#mod = SCodeUtil::filterSubMods(r#mod.clone(), Arc::new(fnptr!(filter, Arc<SCode::SubMod>)));
    r#mod
}

pub fn appendAnnotationMod(mut r#mod: Arc<SCode::Mod>, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            if !(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone().is_empty()) {
                s = IOStream::append(s.clone(), (literal!("(")).clone())?;
                s = appendAnnotationSubMod(listHead(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone())?, s.clone())?;
                for mut m in &*listRest(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone())? {
                    let mut m = m.clone();
                    s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
                    s = appendAnnotationSubMod(m.clone(), s.clone())?;
                }
                s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            }
            if isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
                s = IOStream::append(s.clone(), (literal!(" = ")).clone())?;
                s = appendExp(Util::getOption(var_field!((*r#mod).binding, SCode::Mod::MOD).clone())?, s.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn appendAnnotationSubMod(mut r#mod: Arc<SCode::SubMod>, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut m: Arc<SCode::Mod> = r#mod.r#mod.clone();
    let () = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            if SCodeUtil::finalBool(var_field!((*m).finalPrefix, SCode::Mod::MOD).clone())? {
                s = IOStream::append(s.clone(), (literal!("final ")).clone())?;
            }
            if SCodeUtil::eachBool(var_field!((*m).eachPrefix, SCode::Mod::MOD).clone())? {
                s = IOStream::append(s.clone(), (literal!("each ")).clone())?;
            }
            s = IOStream::append(s.clone(), (r#mod.ident.clone()).clone())?;
            s = appendAnnotationMod(m.clone(), s.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn appendExp(mut exp: Arc<Absyn::Exp>, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (e, _) = AbsynUtil::traverseExp(exp.clone(), Arc::new(quoteCref), 0)?;
    s = IOStream::append(s.clone(), (Dump::printExpStr(e.clone())?).clone())?;
    Ok(s)
}

pub fn quoteCref(mut exp: Arc<Absyn::Exp>, mut dummy: i32) -> Result<(Arc<Absyn::Exp>, i32)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut dummy: i32 = dummy;
    let mut r#str: ArcStr = arcstr::literal!("");
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } if (!(AbsynUtil::crefIsWild(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone()))) => {
            r#str = (Dump::printComponentRefStr(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone())?).clone();
            if r#str.clone() != literal!("time") {
                r#str = (Util::makeQuotedIdentifier((r#str.clone()).clone())?).clone();
                assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (r#str.clone()).clone(), subscripts: metamodelica::nil() }));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, dummy))
}

