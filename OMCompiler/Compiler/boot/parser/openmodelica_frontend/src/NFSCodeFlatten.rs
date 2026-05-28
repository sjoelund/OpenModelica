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

use crate::NFEnvExtends;
use crate::NFSCodeDependency;
use crate::NFSCodeEnv;
use crate::NFSCodeFlattenImports;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub type Env = Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>;

pub fn flattenProgram(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cls_path: Arc<Absyn::Path>;
    cls_path = getLastClassNameInProgram(inProgram.clone())?;
    (outProgram, _) = flattenClassInProgram(cls_path.clone(), inProgram.clone())?;
    Ok(outProgram)
}

fn getLastClassNameInProgram(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<Absyn::Path>> {
    let mut outClassName: Arc<Absyn::Path>;
    let mut prog: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    prog = inProgram.clone().reverse();
    let __pa0 = ::match_deref::match_deref! { match &(List::find(prog.clone(), (std::sync::Arc::new(fnptr!(isClass, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>))?) {
        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    outClassName = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
    Ok(outClassName)
}

fn isClass(mut inClass: Arc<SCode::Element>) -> bool {
    let mut outIsClass: bool = false;
    outIsClass = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClass
}

pub fn flattenClass(mut inClass: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outClass: Arc<SCode::Element>;
    let __pa0 = ::match_deref::match_deref! { match &(flattenProgram(list![inClass.clone()])?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outClass = __pa0.clone();
    Ok(outClass)
}

pub fn flattenClassInProgram(mut inClassName: Arc<Absyn::Path>, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Env)> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outEnv: Env = metamodelica::nil();
    (outProgram, outEnv) = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                prog => {
                    let mut env: Env = metamodelica::nil();
                    let mut prog = (*prog).clone();
                    System::tmpTickResetIndex(0, NFSCodeEnv::tmpTickIndex.clone());
                    System::tmpTickResetIndex(1, NFSCodeEnv::extendsTickIndex.clone());
                    System::setUsesCardinality(false);
                    env = NFSCodeEnv::buildInitialEnv()?;
                    env = NFSCodeEnv::extendEnvWithClasses(prog.clone(), env.clone());
                    env = NFEnvExtends::update(env.clone())?;
                    (prog, env) = NFSCodeDependency::analyse(inClassName.clone(), env.clone(), prog.clone())?;
                    if !(Flags::isSet(Flags::SCODE_INST.clone())?) {
                        (prog, env) = NFSCodeFlattenImports::flattenProgram(prog.clone(), env.clone());
                    }
                    Ok((prog.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSCodeFlatten.flattenClassInProgram failed on ")); __mm_s.push_str(&*AbsynUtil::pathString(inClassName.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outProgram, outEnv))
}

pub fn flattenCompleteProgram(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outProgram = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                prog => {
                    let mut env: Env = metamodelica::nil();
                    let mut prog = (*prog).clone();
                    env = NFSCodeEnv::buildInitialEnv()?;
                    env = NFSCodeEnv::extendEnvWithClasses(prog.clone(), env.clone());
                    env = NFEnvExtends::update(env.clone())?;
                    (prog, env) = NFSCodeFlattenImports::flattenProgram(prog.clone(), env.clone());
                    Ok(prog.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("NFSCodeFlatten.flattenCompleteProgram failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProgram)
}

