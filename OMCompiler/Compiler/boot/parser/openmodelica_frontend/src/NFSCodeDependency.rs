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

use crate::NFInstPrefix;
use crate::NFSCodeCheck;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeEnv;
use crate::NFSCodeFlattenRedeclare;
use crate::NFSCodeLookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub type Env = Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>;

pub type Item = Arc<NFSCodeEnv::Item>;

pub type Extends = Arc<NFSCodeEnv::Extends>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

pub fn analyse(mut inClassName: Arc<Absyn::Path>, mut inEnv: Env, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Env)> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outEnv: Env = metamodelica::nil();
    analyseClass(inClassName.clone(), inEnv.clone(), Absyn::dummyInfo.clone())?;
    analyseClassExtends(inEnv.clone())?;
    (outEnv, outProgram) = collectUsedProgram(inEnv.clone(), inProgram.clone(), inClassName.clone())?;
    Ok((outProgram, outEnv))
}

fn analyseClass(mut inClassName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inClassName.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    (item, env) = lookupClass(inClassName.clone(), inEnv.clone(), true, inInfo.clone(), Some(Error::LOOKUP_ERROR.clone()))?;
                    checkItemIsClass(item.clone())?;
                    analyseItem(item.clone(), env.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeDependency.analyseClass failed for ")); __mm_s.push_str(&*AbsynUtil::pathString(inClassName.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn lookupClass(mut inPath: Arc<Absyn::Path>, mut inEnv: Env, mut inBuiltinPossible: bool, mut inInfo: SourceInfo, mut inErrorType: Option<ErrorTypes::Message>) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outEnv) = 'mc: {
        let __mc_input = (inPath.clone(), inEnv.clone(), inBuiltinPossible.clone(), inInfo.clone(), inErrorType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    (item, env) = lookupClass2(inPath.clone(), inEnv.clone(), inBuiltinPossible.clone(), inInfo.clone(), inErrorType.clone())?;
                    (item, env, _) = NFSCodeEnv::resolveRedeclaredItem(item.clone(), env.clone());
                    Ok((item.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Some(error_id)) => {
                    let mut name_str: ArcStr = arcstr::literal!("");
                    let mut env_str: ArcStr = arcstr::literal!("");
                    name_str = (AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    env_str = (NFSCodeEnv::getEnvName(inEnv.clone())?).clone();
                    Error::addSourceMessage(error_id.clone(), list![(name_str.clone()).clone(), (env_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outEnv))
}

fn lookupClass2(mut inPath: Arc<Absyn::Path>, mut inEnv: Env, mut inBuiltinPossible: bool, mut inInfo: SourceInfo, mut inErrorType: Option<ErrorTypes::Message>) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outEnv) = (::match_deref::match_deref! { match &((inPath.clone(), inEnv.clone(), inBuiltinPossible.clone(), inInfo.clone(), inErrorType.clone())) {
        (Deref @ Absyn::Path::IDENT { .. }, _, true, _, _) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            (item, _, env) = NFSCodeLookup::lookupNameSilent(inPath.clone(), inEnv.clone(), inInfo.clone())?;
            (item.clone(), env.clone())
        },
        (Deref @ Absyn::Path::IDENT { .. }, _, false, _, _) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            (item, _, env) = NFSCodeLookup::lookupNameSilentNoBuiltin(inPath.clone(), inEnv.clone(), inInfo.clone())?;
            (item.clone(), env.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: id }, name: Deref @ "$ce" }, Deref @ metamodelica::List::Cons { head: _, tail: env }, _, _, _) => {
            let mut item: Item;
            let mut env = (*env).clone();
            (item, env) = NFSCodeLookup::lookupInheritedName((id.clone()).clone(), env.clone())?;
            (item.clone(), env.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { path: rest_path, name: id }, _, _, _, _) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            (item, _, env) = NFSCodeLookup::lookupNameSilent(Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }), inEnv.clone(), inInfo.clone())?;
            (item, env, _) = NFSCodeEnv::resolveRedeclaredItem(item.clone(), env.clone());
            analyseItem(item.clone(), env.clone())?;
            (item, env) = lookupNameInItem(rest_path.clone(), item.clone(), env.clone(), inErrorType.clone())?;
            (item.clone(), env.clone())
        },
        (Deref @ Absyn::Path::FULLYQUALIFIED { path: rest_path }, _, _, _, _) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
            (item, env) = lookupClass2(rest_path.clone(), env.clone(), false, inInfo.clone(), inErrorType.clone())?;
            (item.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outEnv))
}

fn lookupNameInItem(mut inName: Arc<Absyn::Path>, mut inItem: Item, mut inEnv: Env, mut inErrorType: Option<ErrorTypes::Message>) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inItem.clone(), inEnv.clone(), inErrorType.clone())) {
        (_, _, Deref @ metamodelica::List::Nil, _) => {
            (inItem.clone(), inEnv.clone())
        },
        (_, Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { info, modifications: mods, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: type_path, .. }, .. }, .. }, _, _) => {
            let mut env: Env = metamodelica::nil();
            let mut type_env: Env = metamodelica::nil();
            let mut redeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
            let mut item: Item;
            (item, type_env) = lookupClass(type_path.clone(), inEnv.clone(), true, info.clone(), inErrorType.clone())?;
            let true = (NFSCodeEnv::isClassItem(item.clone())) else { bail!("pattern mismatch") };
            redeclares = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(mods.clone());
            (item, type_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redeclares.clone(), item.clone(), type_env.clone(), inEnv.clone(), NFInstPrefix::emptyPrefix.clone())?;
            (item, env) = lookupNameInItem(inName.clone(), item.clone(), type_env.clone(), inErrorType.clone())?;
            (item.clone(), env.clone())
        },
        (_, Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::List::Cons { head: class_env, tail: Deref @ metamodelica::List::Nil }, cls: Deref @ SCode::Element::CLASS { info, .. }, .. }, _, _) => {
            let mut env: Env = metamodelica::nil();
            let mut item: Item;
            env = NFSCodeEnv::enterFrame(class_env.clone(), inEnv.clone());
            (item, env) = lookupClass(inName.clone(), env.clone(), false, info.clone(), inErrorType.clone())?;
            (item.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outEnv))
}

fn checkItemIsClass(mut inItem: Item) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ NFSCodeEnv::Item::CLASS { .. } => {
            ()
        },
        Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { info, name, .. }, .. } => {
            Error::addSourceMessage(Error::LOOKUP_TYPE_FOUND_COMP.clone(), list![(name.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseItem(mut inItem: Item, mut inEnv: Env) -> Result<()> {
    if NFSCodeEnv::isItemUsed(inItem.clone()) {
        return Ok(());
    }
    let _ = (::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
        (Deref @ NFSCodeEnv::Item::VAR { .. }, env) => {
            markItemAsUsed(inItem.clone(), env.clone())?;
            ()
        },
        (Deref @ NFSCodeEnv::Item::CLASS { classType: NFSCodeEnv::ClassType::BASIC_TYPE, .. }, _) => {
            ()
        },
        (Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::List::Cons { head: cls_env, tail: Deref @ metamodelica::List::Nil }, cls: cls @ Deref @ SCode::Element::CLASS { cmt, info, restriction: res, classDef: cdef, .. }, .. }, env) => {
            let mut env = (*env).clone();
            markItemAsUsed(inItem.clone(), env.clone())?;
            env = NFSCodeEnv::enterFrame(cls_env.clone(), env.clone());
            if if (var_field!((**cls).name, SCode::Element::CLASS).clone() == literal!("cardinality")) {(::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { name: None, .. }, tail: Deref @ metamodelica::List::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })} else {false} {
                System::setUsesCardinality(true);
            }
            analyseClassDef(cdef.clone(), res.clone(), env.clone(), false, info.clone())?;
            analyseMetaType(res.clone(), env.clone(), info.clone())?;
            analyseComment(cmt.clone(), env.clone(), info.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            env = __pa0.clone();
            analyseRedeclaredClass(cls.clone(), env.clone())?;
            ()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeDependency.analyseItem failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseItemIfRedeclares(mut inRepls: Arc<metamodelica::List<NFSCodeFlattenRedeclare::Replacement>>, mut inItem: Item, mut inEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inRepls.clone(), inItem.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut env: Env = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    env = __pa0.clone();
                    analyseItemNoStopOnUsed(inItem.clone(), env.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn analyseItemNoStopOnUsed(mut inItem: Item, mut inEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inItem.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::VAR { .. }, env) => {
                    markItemAsUsed(inItem.clone(), env.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { classType: NFSCodeEnv::ClassType::BASIC_TYPE, .. }, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::List::Cons { head: cls_env, tail: Deref @ metamodelica::List::Nil }, cls: cls @ Deref @ SCode::Element::CLASS { cmt, info, restriction: res, classDef: cdef, .. }, .. }, env) => {
                    let mut env = (*env).clone();
                    markItemAsUsed(inItem.clone(), env.clone())?;
                    env = NFSCodeEnv::enterFrame(cls_env.clone(), env.clone());
                    analyseClassDef(cdef.clone(), res.clone(), env.clone(), false, info.clone())?;
                    analyseMetaType(res.clone(), env.clone(), info.clone())?;
                    analyseComment(cmt.clone(), env.clone(), info.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    env = __pa0.clone();
                    analyseRedeclaredClass(cls.clone(), env.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeDependency.analyseItemNoStopOnUsed failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn markItemAsUsed(mut inItem: Item, mut inEnv: Env) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
        (Deref @ NFSCodeEnv::Item::VAR { isUsed: Some(is_used), .. }, _) => {
            Mutable::update(is_used.clone(), true);
            markEnvAsUsed(inEnv.clone())?;
            ()
        },
        (Deref @ NFSCodeEnv::Item::VAR { isUsed: None, .. }, _) => {
            ()
        },
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { .. }, env: Deref @ metamodelica::List::Cons { head: cls_env, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            markFrameAsUsed(cls_env.clone());
            markEnvAsUsed(inEnv.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn markFrameAsUsed(mut inFrame: Arc<NFSCodeEnv::Frame>) -> () {
    let _ = (::match_deref::match_deref! { match &(inFrame.clone()) {
        Deref @ NFSCodeEnv::Frame { isUsed: Some(is_used), .. } => {
            Mutable::update(is_used.clone(), true);
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn markEnvAsUsed(mut inEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: f @ Deref @ NFSCodeEnv::Frame { isUsed: Some(is_used), .. }, tail: rest_env } => {
                    let false = (Mutable::access(is_used.clone())) else { bail!("pattern mismatch") };
                    markEnvAsUsed2(f.clone(), rest_env.clone())?;
                    Mutable::update(is_used.clone(), true);
                    markEnvAsUsed(rest_env.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn markEnvAsUsed2(mut inFrame: Arc<NFSCodeEnv::Frame>, mut inEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inFrame.clone(), inEnv.clone())) {
        (Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, _) => {
            ()
        },
        (Deref @ NFSCodeEnv::Frame { name: Some(name), .. }, _) => {
            analyseClass(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inEnv.clone(), Absyn::dummyInfo.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseClassDef(mut inClassDef: Arc<SCode::ClassDef>, mut inRestriction: SCode::Restriction, mut inEnv: Env, mut inInModifierScope: bool, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inClassDef.clone(), inRestriction.clone(), inEnv.clone(), inInModifierScope.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { externalDecl: ext_decl, initialAlgorithmLst: ial, normalAlgorithmLst: nal, initialEquationLst: iel, normalEquationLst: nel, elementLst: el, .. }, _, _, _, _) => {
                    analyseElements(el.clone(), inEnv.clone(), inRestriction.clone())?;
                    List::map1_0(nel.clone(), Arc::new(fnptr!(analyseEquation, Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)), inEnv.clone());
                    List::map1_0(iel.clone(), Arc::new(fnptr!(analyseEquation, Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)), inEnv.clone());
                    List::map1_0(nal.clone(), Arc::new(analyseAlgorithm), inEnv.clone());
                    List::map1_0(ial.clone(), Arc::new(analyseAlgorithm), inEnv.clone());
                    analyseExternalDecl(ext_decl.clone(), inEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: el, .. }, _, _, _, _) => {
                    isExternalObject(el.clone(), inEnv.clone(), inInfo.clone())?;
                    analyseClass(Arc::new(Absyn::Path::IDENT { name: (literal!("constructor")).clone() }), inEnv.clone(), inInfo.clone())?;
                    analyseClass(Arc::new(Absyn::Path::IDENT { name: (literal!("destructor")).clone() }), inEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, _, _, _, _) => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("NFSCodeDependency.analyseClassDef failed on CLASS_EXTENDS")).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::DERIVED { modifications: mods, typeSpec: ty, .. }, _, Deref @ metamodelica::List::Cons { head: _, tail: env }, _, _) => {
                    let mut ty_env: Env = metamodelica::nil();
                    let mut nore_env: Env = metamodelica::nil();
                    let mut ty_item: Item;
                    let mut redecls: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
                    let mut repls: Arc<metamodelica::List<NFSCodeFlattenRedeclare::Replacement>> = metamodelica::nil();
                    let mut env = (*env).clone();
                    env = if (inInModifierScope.clone()) {inEnv.clone()} else {env.clone()};
                    nore_env = NFSCodeEnv::removeRedeclaresFromLocalScope(env.clone())?;
                    analyseTypeSpec(ty.clone(), nore_env.clone(), inInfo.clone())?;
                    (ty_item, _, ty_env) = NFSCodeLookup::lookupTypeSpec(ty.clone(), env.clone(), inInfo.clone())?;
                    (ty_item, ty_env, _) = NFSCodeEnv::resolveRedeclaredItem(ty_item.clone(), ty_env.clone());
                    ty_env = NFSCodeEnv::mergeItemEnv(ty_item.clone(), ty_env.clone());
                    redecls = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(mods.clone());
                    (ty_item, ty_env, repls) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redecls.clone(), ty_item.clone(), ty_env.clone(), inEnv.clone(), NFInstPrefix::emptyPrefix.clone())?;
                    analyseItemIfRedeclares(repls.clone(), ty_item.clone(), ty_env.clone())?;
                    analyseModifier(mods.clone(), inEnv.clone(), ty_env.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::ENUMERATION { .. }, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::OVERLOAD { pathLst: paths }, _, _, _, _) => {
                    if !(Config::synchronousFeaturesAllowed()) && AbsynUtil::pathFirstIdent(listHead(paths.clone())?)? == literal!("OMC_NO_CLOCK") {
                        List::map2_0(list![listHead(paths.clone())?], Arc::new(analyseClass), inEnv.clone(), inInfo.clone());
                    } else {
                        List::map2_0(paths.clone(), Arc::new(analyseClass), inEnv.clone(), inInfo.clone());
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PDER { .. }, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn isExternalObject(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let mut el: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut el_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    el = List::filterOnTrue(inElements.clone(), Arc::new(fnptr!(isNotExternalObject, Arc<SCode::Element>)));
    let false = ((el.clone().len() as i32) == (inElements.clone().len() as i32)) else { bail!("pattern mismatch") };
    el_names = List::filterMap(el.clone(), Arc::new(elementName));
    checkExternalObject(el_names.clone(), inEnv.clone(), inInfo.clone())?;
    Ok(())
}

fn elementName(mut inElement: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { name, .. } => {
            name.clone()
        },
        Deref @ SCode::Element::CLASS { name, .. } => {
            name.clone()
        },
        Deref @ SCode::Element::DEFINEUNIT { name, .. } => {
            name.clone()
        },
        Deref @ SCode::Element::EXTENDS { baseClassPath: bc, .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            name = (AbsynUtil::pathString(bc.clone(), (literal!(".")).clone(), true, false)?).clone();
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("extends ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            name.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn isNotExternalObject(mut inElement: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: Deref @ "ExternalObject" }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn checkExternalObject(mut inElements: Arc<metamodelica::List<ArcStr>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inElements.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ "constructor", tail: Deref @ metamodelica::List::Cons { head: Deref @ "destructor", tail: Deref @ metamodelica::List::Nil } }, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ "destructor", tail: Deref @ metamodelica::List::Cons { head: Deref @ "constructor", tail: Deref @ metamodelica::List::Nil } }, _, _) => {
            ()
        },
        _ => {
            let mut env_str: ArcStr = arcstr::literal!("");
            let mut has_con: bool = false;
            let mut has_des: bool = false;
            has_con = List::isMemberOnTrue((literal!("constructor")).clone(), inElements.clone(), Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)));
            has_des = List::isMemberOnTrue((literal!("destructor")).clone(), inElements.clone(), Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)));
            env_str = (NFSCodeEnv::getEnvName(inEnv.clone())?).clone();
            checkExternalObject2(inElements.clone(), has_con.clone(), has_des.clone(), (env_str.clone()).clone(), inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkExternalObject2(mut inElements: Arc<metamodelica::List<ArcStr>>, mut inHasConstructor: bool, mut inHasDestructor: bool, mut inObjectName: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inElements.clone(), inHasConstructor.clone(), inHasDestructor.clone(), inObjectName.clone(), inInfo.clone())) {
        (el, true, true, _, _) => {
            let mut el_str: ArcStr = arcstr::literal!("");
            let mut el = (*el).clone();
            (el, _) = List::deleteMemberOnTrue((literal!("constructor")).clone(), el.clone(), Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)))?;
            (el, _) = List::deleteMemberOnTrue((literal!("destructor")).clone(), el.clone(), Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)))?;
            el_str = stringDelimitList(el.clone(), (literal!(", ")).clone());
            el_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("contains invalid elements: ")); __mm_s.push_str(&*el_str.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::INVALID_EXTERNAL_OBJECT.clone(), list![(inObjectName.clone()).clone(), (el_str.clone()).clone()], inInfo.clone())?;
            ()
        },
        (_, false, true, _, _) => {
            Error::addSourceMessage(Error::INVALID_EXTERNAL_OBJECT.clone(), list![(inObjectName.clone()).clone(), (literal!("missing constructor")).clone()], inInfo.clone())?;
            ()
        },
        (_, true, false, _, _) => {
            Error::addSourceMessage(Error::INVALID_EXTERNAL_OBJECT.clone(), list![(inObjectName.clone()).clone(), (literal!("missing destructor")).clone()], inInfo.clone())?;
            ()
        },
        (_, false, false, _, _) => {
            Error::addSourceMessage(Error::INVALID_EXTERNAL_OBJECT.clone(), list![(inObjectName.clone()).clone(), (literal!("missing both constructor and destructor")).clone()], inInfo.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseMetaType(mut inRestriction: SCode::Restriction, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inRestriction.clone(), inEnv.clone(), inInfo.clone())) {
        (SCode::Restriction::R_METARECORD { name: union_name, .. }, _, _) => {
            analyseClass(union_name.clone(), inEnv.clone(), inInfo.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseRedeclaredClass(mut inClass: Arc<SCode::Element>, mut inEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inClass.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, _) => {
                    let false = (SCodeUtil::isElementRedeclare(inClass.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, _) => {
                    let mut item: Item;
                    item = Arc::new(NFSCodeEnv::Item::CLASS { cls: inClass.clone(), env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::USERDEFINED });
                    analyseRedeclaredClass2(item.clone(), inEnv.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn analyseRedeclaredClass2(mut inItem: Item, mut inEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inItem.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { info, .. }, .. }, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    (item, env) = NFSCodeLookup::lookupRedeclaredClassByItem(inItem.clone(), inEnv.clone(), info.clone())?;
                    analyseItem(item.clone(), env.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeDependency.analyseRedeclaredClass2 failed for ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn analyseElements(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnv: Env, mut inClassRestriction: SCode::Restriction) -> Result<()> {
    let mut exts: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    exts = NFSCodeEnv::getEnvExtendsFromTable(inEnv.clone())?;
    analyseElements2(inElements.clone(), inEnv.clone(), exts.clone(), inClassRestriction.clone())?;
    Ok(())
}

fn analyseElements2(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnv: Env, mut inExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>, mut inClassRestriction: SCode::Restriction) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inElements.clone(), inEnv.clone(), inExtends.clone(), inClassRestriction.clone())) {
        (Deref @ metamodelica::List::Cons { head: el, tail: rest_el }, _, _, _) => {
            let mut exts: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
            exts = analyseElement(el.clone(), inEnv.clone(), inExtends.clone(), inClassRestriction.clone())?;
            analyseElements2(rest_el.clone(), inEnv.clone(), exts.clone(), inClassRestriction.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseElement(mut inElement: Arc<SCode::Element>, mut inEnv: Env, mut inExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>, mut inClassRestriction: SCode::Restriction) -> Result<Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>> {
    let mut outExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    outExtends = (::match_deref::match_deref! { match &((inElement.clone(), inEnv.clone(), inExtends.clone(), inClassRestriction.clone())) {
        (Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: Deref @ "ExternalObject" }, .. }, _, _, _) => {
            bail!("fail")
        },
        (Deref @ SCode::Element::EXTENDS { info, modifications: mods, .. }, _, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Extends { baseClass: bc, .. }, tail: exts }, _) => {
            let mut ty_item: Item;
            let mut ty_env: Env = metamodelica::nil();
            (ty_item, _, ty_env) = NFSCodeLookup::lookupBaseClassName(bc.clone(), inEnv.clone(), info.clone())?;
            analyseExtends(bc.clone(), inEnv.clone(), info.clone())?;
            ty_env = NFSCodeEnv::mergeItemEnv(ty_item.clone(), ty_env.clone());
            analyseModifier(mods.clone(), inEnv.clone(), ty_env.clone(), info.clone())?;
            exts.clone()
        },
        (Deref @ SCode::Element::COMPONENT { info, prefixes, condition: cond_exp, modifications: mods, typeSpec: ty, attributes: attr, name, .. }, _, _, _) => {
            let mut ty_item: Item;
            let mut ty_env: Env = metamodelica::nil();
            let mut redecls: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
            markAsUsedOnRestriction((name.clone()).clone(), inClassRestriction.clone(), inEnv.clone(), info.clone())?;
            analyseAttributes(attr.clone(), inEnv.clone(), info.clone())?;
            analyseTypeSpec(ty.clone(), inEnv.clone(), info.clone())?;
            (ty_item, _, ty_env) = NFSCodeLookup::lookupTypeSpec(ty.clone(), inEnv.clone(), info.clone())?;
            (ty_item, ty_env, _) = NFSCodeEnv::resolveRedeclaredItem(ty_item.clone(), ty_env.clone());
            ty_env = NFSCodeEnv::mergeItemEnv(ty_item.clone(), ty_env.clone());
            NFSCodeCheck::checkRecursiveComponentDeclaration((name.clone()).clone(), info.clone(), ty_env.clone(), ty_item.clone(), inEnv.clone())?;
            redecls = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(mods.clone());
            (ty_item, ty_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redecls.clone(), ty_item.clone(), ty_env.clone(), inEnv.clone(), NFInstPrefix::emptyPrefix.clone())?;
            analyseModifier(mods.clone(), inEnv.clone(), ty_env.clone(), info.clone())?;
            analyseOptExp(cond_exp.clone(), inEnv.clone(), info.clone())?;
            analyseConstrainClass(SCodeUtil::replaceableOptConstraint(SCodeUtil::prefixesReplaceable(prefixes.clone())?)?, inEnv.clone(), info.clone())?;
            inExtends.clone()
        },
        (Deref @ SCode::Element::CLASS { info, restriction: SCode::Restriction::R_OPERATOR, name, .. }, _, _, SCode::Restriction::R_RECORD { isOperator: true }) => {
            analyseClass(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inEnv.clone(), info.clone())?;
            inExtends.clone()
        },
        (Deref @ SCode::Element::CLASS { info, restriction: SCode::Restriction::R_OPERATOR, name, .. }, _, _, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (SCodeDump::restrString(inClassRestriction.clone())?).clone();
            Error::addSourceMessage(Error::OPERATOR_FUNCTION_NOT_EXPECTED.clone(), list![(name.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ SCode::Element::CLASS { info, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION }, name, .. }, _, _, SCode::Restriction::R_RECORD { isOperator: true }) => {
            analyseClass(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inEnv.clone(), info.clone())?;
            inExtends.clone()
        },
        (Deref @ SCode::Element::CLASS { info, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION }, name, .. }, _, _, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (SCodeDump::restrString(inClassRestriction.clone())?).clone();
            Error::addSourceMessage(Error::OPERATOR_FUNCTION_NOT_EXPECTED.clone(), list![(name.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ SCode::Element::CLASS { info, restriction: res, name, .. }, _, _, SCode::Restriction::R_OPERATOR) => {
            let true = (SCodeUtil::isFunctionOrExtFunctionRestriction(res.clone())) else { bail!("pattern mismatch") };
            analyseClass(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inEnv.clone(), info.clone())?;
            inExtends.clone()
        },
        (Deref @ SCode::Element::CLASS { info, restriction: res, name, .. }, _, _, SCode::Restriction::R_OPERATOR) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let false = (SCodeUtil::isFunctionOrExtFunctionRestriction(res.clone())) else { bail!("pattern mismatch") };
            r#str = (SCodeDump::restrString(res.clone())?).clone();
            Error::addSourceMessage(Error::OPERATOR_FUNCTION_EXPECTED.clone(), list![(name.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ SCode::Element::CLASS { info, name: name @ Deref @ "equalityConstraint", .. }, _, _, _) => {
            analyseClass(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inEnv.clone(), info.clone())?;
            inExtends.clone()
        },
        (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, info, name, .. }, _, _, _) => {
            analyseClass(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inEnv.clone(), info.clone())?;
            inExtends.clone()
        },
        (Deref @ SCode::Element::CLASS { info, prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER, .. }, name, .. }, _, _, _) => {
            analyseClass(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inEnv.clone(), info.clone())?;
            inExtends.clone()
        },
        (Deref @ SCode::Element::CLASS { info, prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER_OUTER, .. }, name, .. }, _, _, _) => {
            analyseClass(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), inEnv.clone(), info.clone())?;
            inExtends.clone()
        },
        _ => {
            inExtends.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExtends)
}

fn markAsUsedOnConstant(mut inName: ArcStr, mut inAttr: SCode::Attributes, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inName.clone(), inAttr.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Attributes { variability: var, .. }, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: cls_and_vars, .. }, tail: _ }, _) => {
                    let mut is_used: Mutable::Mutable<bool>;
                    let true = (SCodeUtil::isParameterOrConst(var.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(cls_and_vars.clone(), (inName.clone()).clone())?) {
                        Deref @ NFSCodeEnv::Item::VAR { isUsed: Some(__pa0), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    is_used = __pa0.clone();
                    Mutable::update(is_used.clone(), true);
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn markAsUsedOnRestriction(mut inName: ArcStr, mut inRestriction: SCode::Restriction, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inName.clone(), inRestriction.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: cls_and_vars, .. }, tail: _ }, _) => {
                    let mut is_used: Mutable::Mutable<bool>;
                    let true = (markAsUsedOnRestriction2(inRestriction.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(cls_and_vars.clone(), (inName.clone()).clone())?) {
                        Deref @ NFSCodeEnv::Item::VAR { isUsed: Some(__pa0), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    is_used = __pa0.clone();
                    Mutable::update(is_used.clone(), true);
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn markAsUsedOnRestriction2(mut inRestriction: SCode::Restriction) -> bool {
    let mut isRestricted: bool = false;
    isRestricted = (match inRestriction.clone() {
        SCode::Restriction::R_CONNECTOR { .. } => true,
        SCode::Restriction::R_RECORD { isOperator: _ } => true,
        _ => false,
    });
    isRestricted
}

fn analyseExtends(mut inClassName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let mut item: Item;
    let mut env: Env = metamodelica::nil();
    (item, env) = lookupClass(inClassName.clone(), inEnv.clone(), true, inInfo.clone(), None)?;
    analyseItem(item.clone(), env.clone())?;
    Ok(())
}

fn analyseAttributes(mut inAttributes: SCode::Attributes, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let SCode::ATTR { arrayDims: __pa0, .. } = (inAttributes.clone()) else { bail!("pattern mismatch") };
    ad = __pa0.clone();
    List::map2_0(ad.clone(), Arc::new(analyseSubscript), inEnv.clone(), inInfo.clone());
    Ok(())
}

fn analyseModifier(mut inModifier: Arc<SCode::Mod>, mut inEnv: Env, mut inTypeEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inModifier.clone(), inEnv.clone(), inTypeEnv.clone(), inInfo.clone())) {
        (Deref @ SCode::Mod::NOMOD, _, _, _) => {
            ()
        },
        (Deref @ SCode::Mod::MOD { binding: bind_exp, subModLst: sub_mods, .. }, _, _, _) => {
            List::map2_0(sub_mods.clone(), Arc::new(analyseSubMod), (inEnv.clone(), inTypeEnv.clone()), inInfo.clone());
            analyseModBinding(bind_exp.clone(), inEnv.clone(), inInfo.clone())?;
            ()
        },
        (Deref @ SCode::Mod::REDECL { element: el, .. }, _, _, _) => {
            analyseRedeclareModifier(el.clone(), inEnv.clone(), inTypeEnv.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseRedeclareModifier(mut inElement: Arc<SCode::Element>, mut inEnv: Env, mut inTypeEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inElement.clone(), inEnv.clone(), inTypeEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { info, restriction: restr, classDef: cdef, prefixes, .. }, _, _) => {
                    analyseClassDef(cdef.clone(), restr.clone(), inEnv.clone(), true, info.clone())?;
                    analyseConstrainClass(SCodeUtil::replaceableOptConstraint(SCodeUtil::prefixesReplaceable(prefixes.clone())?)?, inEnv.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let _ = analyseElement(inElement.clone(), inEnv.clone(), metamodelica::nil(), openmodelica_frontend_types::SCode::Restriction::R_CLASS)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn analyseConstrainClass(mut inCC: Option<Arc<SCode::ConstrainClass>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inCC.clone(), inEnv.clone(), inInfo.clone())) {
        (Some(Deref @ SCode::ConstrainClass { modifier: r#mod, constrainingClass: path, .. }), _, _) => {
            let mut env: Env = metamodelica::nil();
            analyseClass(path.clone(), inEnv.clone(), inInfo.clone())?;
            (_, env) = lookupClass(path.clone(), inEnv.clone(), true, inInfo.clone(), Some(Error::LOOKUP_ERROR.clone()))?;
            analyseModifier(r#mod.clone(), inEnv.clone(), env.clone(), inInfo.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseSubMod(mut inSubMod: Arc<SCode::SubMod>, mut inEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>), mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inSubMod.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ SCode::SubMod { r#mod: m, ident }, (env, ty_env), _) => {
            analyseNameMod((ident.clone()).clone(), env.clone(), ty_env.clone(), m.clone(), inInfo.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseNameMod(mut inIdent: ArcStr, mut inEnv: Env, mut inTypeEnv: Env, mut inMod: Arc<SCode::Mod>, mut inInfo: SourceInfo) -> Result<()> {
    let mut item: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (item, env) = lookupNameMod(Arc::new(Absyn::Path::IDENT { name: (inIdent.clone()).clone() }), inTypeEnv.clone(), inInfo.clone())?;
    analyseNameMod2((inIdent.clone()).clone(), item.clone(), env.clone(), inEnv.clone(), inTypeEnv.clone(), inMod.clone(), inInfo.clone())?;
    Ok(())
}

fn analyseNameMod2(mut inIdent: ArcStr, mut inItem: Option<Arc<NFSCodeEnv::Item>>, mut inItemEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>, mut inEnv: Env, mut inTypeEnv: Env, mut inModifier: Arc<SCode::Mod>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inIdent.clone(), inItem.clone(), inItemEnv.clone(), inEnv.clone(), inTypeEnv.clone(), inModifier.clone(), inInfo.clone())) {
        (_, Some(item), Some(env), _, _, _, _) => {
            let mut env = (*env).clone();
            NFSCodeCheck::checkModifierIfRedeclare(item.clone(), inModifier.clone(), inInfo.clone())?;
            analyseItem(item.clone(), env.clone())?;
            env = NFSCodeEnv::mergeItemEnv(item.clone(), env.clone());
            analyseModifier(inModifier.clone(), inEnv.clone(), env.clone(), inInfo.clone())?;
            ()
        },
        _ => {
            analyseModifier(inModifier.clone(), inEnv.clone(), inTypeEnv.clone(), inInfo.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn lookupNameMod(mut inPath: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outEnv) = 'mc: {
        let __mc_input = (inPath.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    (item, _, env) = NFSCodeLookup::lookupNameSilent(inPath.clone(), inEnv.clone(), inInfo.clone())?;
                    (item, env, _) = NFSCodeEnv::resolveRedeclaredItem(item.clone(), env.clone());
                    Ok((Some(item.clone()), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outEnv))
}

fn analyseSubscript(mut inSubscript: Arc<Absyn::Subscript>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inSubscript.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ Absyn::Subscript::NOSUB, _, _) => {
            ()
        },
        (Deref @ Absyn::Subscript::SUBSCRIPT { subscript: sub_exp }, _, _) => {
            analyseExp(sub_exp.clone(), inEnv.clone(), inInfo.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseModBinding(mut inBinding: Option<Arc<Absyn::Exp>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inBinding.clone()) {
        None => {
            ()
        },
        Some(bind_exp) => {
            analyseExp(bind_exp.clone(), inEnv.clone(), inInfo.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseTypeSpec(mut inTypeSpec: Arc<Absyn::TypeSpec>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inTypeSpec.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ Absyn::TypeSpec::TPATH { arrayDim: ad, path: type_path }, _, _) => {
            analyseClass(type_path.clone(), inEnv.clone(), inInfo.clone())?;
            analyseTypeSpecDims(ad.clone(), inEnv.clone(), inInfo.clone());
            ()
        },
        (Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. }, _, _) => {
            ()
        },
        (Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: tys, .. }, _, _) => {
            List::map2_0(tys.clone(), Arc::new(analyseTypeSpec), inEnv.clone(), inInfo.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseTypeSpecDims(mut inDims: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut inEnv: Env, mut inInfo: SourceInfo) -> () {
    let _ = (::match_deref::match_deref! { match &((inDims.clone(), inEnv.clone(), inInfo.clone())) {
        (Some(dims), _, _) => {
            List::map2_0(dims.clone(), Arc::new(analyseTypeSpecDim), inEnv.clone(), inInfo.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn analyseTypeSpecDim(mut inDim: Arc<Absyn::Subscript>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inDim.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ Absyn::Subscript::NOSUB, _, _) => {
            ()
        },
        (Deref @ Absyn::Subscript::SUBSCRIPT { subscript: dim }, _, _) => {
            analyseExp(dim.clone(), inEnv.clone(), inInfo.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseExternalDecl(mut inExtDecl: Option<Arc<SCode::ExternalDecl>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inExtDecl.clone(), inEnv.clone(), inInfo.clone())) {
        (Some(Deref @ SCode::ExternalDecl { annotation_: None, args, .. }), _, _) => {
            List::map2_0(args.clone(), Arc::new(analyseExp), inEnv.clone(), inInfo.clone());
            ()
        },
        (Some(Deref @ SCode::ExternalDecl { annotation_: Some(ann), args, .. }), _, _) => {
            List::map2_0(args.clone(), Arc::new(analyseExp), inEnv.clone(), inInfo.clone());
            analyseAnnotation(ann.clone(), inEnv.clone(), inInfo.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseComment(mut inComment: Arc<SCode::Comment>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inComment.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ SCode::Comment { annotation_: Some(ann), .. }, _, _) => {
            analyseAnnotation(ann.clone(), inEnv.clone(), inInfo.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseAnnotation(mut inAnnotation: Arc<SCode::Annotation>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inAnnotation.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: sub_mods, .. } }, _, _) => {
            List::map2_0(sub_mods.clone(), Arc::new(analyseAnnotationMod), inEnv.clone(), inInfo.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn analyseAnnotationMod(mut inMod: Arc<SCode::SubMod>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inMod.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::SubMod { r#mod: mods, ident: Deref @ "derivative" }, _, _) => {
                    analyseModifier(mods.clone(), inEnv.clone(), NFSCodeEnv::emptyEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::SubMod { r#mod: mods, ident: Deref @ "inverse" }, _, _) => {
                    analyseModifier(mods.clone(), inEnv.clone(), NFSCodeEnv::emptyEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::SubMod { r#mod: mods, ident: id }, _, _) => {
                    analyseAnnotationName((id.clone()).clone(), inEnv.clone(), inInfo.clone())?;
                    analyseModifier(mods.clone(), inEnv.clone(), NFSCodeEnv::emptyEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn analyseAnnotationName(mut inName: ArcStr, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let mut item: Item;
    let mut env: Env = metamodelica::nil();
    (item, _, env) = NFSCodeLookup::lookupNameSilent(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), inEnv.clone(), inInfo.clone())?;
    (item, env, _) = NFSCodeEnv::resolveRedeclaredItem(item.clone(), env.clone());
    analyseItem(item.clone(), env.clone())?;
    Ok(())
}

fn analyseExp(mut inExp: Arc<Absyn::Exp>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    (_, _) = AbsynUtil::traverseExpBidir(inExp.clone(), Arc::new(analyseExpTraverserEnter), Arc::new(fnptr!(analyseExpTraverserExit, Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))), (inEnv.clone(), inInfo.clone()))?;
    Ok(())
}

fn analyseOptExp(mut inExp: Option<Arc<Absyn::Exp>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inExp.clone(), inEnv.clone(), inInfo.clone())) {
        (Some(exp), _, _) => {
            analyseExp(exp.clone(), inEnv.clone(), inInfo.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseExpTraverserEnter(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo);
    let mut env: Env = metamodelica::nil();
    let mut info: SourceInfo;
    (env, info) = inTuple.clone();
    env = analyseExp2(inExp.clone(), env.clone(), info.clone())?;
    outExp = inExp.clone();
    outTuple = (env.clone(), info.clone());
    Ok((outExp, outTuple))
}

fn analyseExp2(mut inExp: Arc<Absyn::Exp>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inExp.clone(), inEnv.clone(), inInfo.clone())) {
        (Deref @ Absyn::Exp::CREF { componentRef: cref }, _, _) => {
            analyseCref(cref.clone(), inEnv.clone(), inInfo.clone())?;
            inEnv.clone()
        },
        (Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterators: iters, .. }, function_: cref, .. }, _, _) => {
            let mut env: Env = metamodelica::nil();
            analyseCref(cref.clone(), inEnv.clone(), inInfo.clone())?;
            env = NFSCodeEnv::extendEnvWithIterators(iters.clone(), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), inEnv.clone());
            env.clone()
        },
        (Deref @ Absyn::Exp::CALL { function_: cref, .. }, _, _) => {
            analyseCref(cref.clone(), inEnv.clone(), inInfo.clone())?;
            inEnv.clone()
        },
        (Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, .. }, _, _) => {
            analyseCref(cref.clone(), inEnv.clone(), inInfo.clone())?;
            inEnv.clone()
        },
        (Deref @ Absyn::Exp::MATCHEXP { .. }, _, _) => {
            let mut env: Env = metamodelica::nil();
            env = NFSCodeEnv::extendEnvWithMatch(inExp.clone(), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), inEnv.clone())?;
            env.clone()
        },
        _ => {
            inEnv.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn analyseCref(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inCref.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::WILD, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    path = AbsynUtil::crefToPathIgnoreSubs(inCref.clone())?;
                    (item, env) = lookupClass(path.clone(), inEnv.clone(), true, inInfo.clone(), None)?;
                    analyseItem(item.clone(), env.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn analyseExpTraverserExit(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> (Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. }, .. }, (Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: env }, info)) => {
            (inExp.clone(), (env.clone(), info.clone()))
        },
        (Deref @ Absyn::Exp::MATCHEXP { .. }, (Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: env }, info)) => {
            (inExp.clone(), (env.clone(), info.clone()))
        },
        _ => {
            (inExp.clone(), inTuple.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outTuple)
}

fn analyseEquation(mut inEquation: Arc<SCode::Equation>, mut inEnv: Env) -> () {
    (_, _) = SCodeUtil::mapFoldEquations(inEquation.clone(), Arc::new(analyseEquationTraverser), inEnv.clone());
    ()
}

fn analyseEquationTraverser(mut eq: Arc<SCode::Equation>, mut env: Env) -> Result<(Arc<SCode::Equation>, Env)> {
    let mut eq: Arc<SCode::Equation> = eq;
    let mut env: Env = env;
    (eq, env) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_FOR { info, index: iter_name, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone());
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq.clone(), Arc::new(traverseExp), (env.clone(), info.clone()))?;
            (eq.clone(), env.clone())
        },
        Deref @ SCode::Equation::EQ_REINIT { info, cref: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, .. } => {
            analyseCref(cref1.clone(), env.clone(), info.clone())?;
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq.clone(), Arc::new(traverseExp), (env.clone(), info.clone()))?;
            (eq.clone(), env.clone())
        },
        _ => {
            let mut info: SourceInfo;
            info = SCodeUtil::getEquationInfo(eq.clone())?;
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq.clone(), Arc::new(traverseExp), (env.clone(), info.clone()))?;
            (eq.clone(), env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, env))
}

fn traverseExp(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo);
    (outExp, outTuple) = AbsynUtil::traverseExpBidir(inExp.clone(), Arc::new(analyseExpTraverserEnter), Arc::new(fnptr!(analyseExpTraverserExit, Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))), inTuple.clone())?;
    Ok((outExp, outTuple))
}

fn analyseAlgorithm(mut inAlgorithm: Arc<SCode::AlgorithmSection>, mut inEnv: Env) -> Result<()> {
    let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inAlgorithm.clone()) {
        Deref @ SCode::AlgorithmSection { statements: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stmts = __pa0.clone();
    List::map1_0(stmts.clone(), Arc::new(fnptr!(analyseStatement, Arc<SCode::Statement>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)), inEnv.clone());
    Ok(())
}

fn analyseStatement(mut inStatement: Arc<SCode::Statement>, mut inEnv: Env) -> () {
    (_, _) = SCodeUtil::mapFoldStatements(inStatement.clone(), Arc::new(analyseStatementTraverser), inEnv.clone());
    ()
}

fn analyseStatementTraverser(mut stmt: Arc<SCode::Statement>, mut env: Env) -> Result<(Arc<SCode::Statement>, Env)> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    let mut env: Env = env;
    (stmt, env) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_FOR { info, index: iter_name, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone());
            (_, _) = SCodeUtil::mapFoldStatementExps(stmt.clone(), Arc::new(traverseExp), (env.clone(), info.clone()));
            (stmt.clone(), env.clone())
        },
        Deref @ SCode::Statement::ALG_PARFOR { info, index: iter_name, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone());
            (_, _) = SCodeUtil::mapFoldStatementExps(stmt.clone(), Arc::new(traverseExp), (env.clone(), info.clone()));
            (stmt.clone(), env.clone())
        },
        _ => {
            let mut info: SourceInfo;
            info = SCodeUtil::getStatementInfo(stmt.clone())?;
            (_, _) = SCodeUtil::mapFoldStatementExps(stmt.clone(), Arc::new(traverseExp), (env.clone(), info.clone()));
            (stmt.clone(), env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((stmt, env))
}

fn analyseClassExtends(mut inEnv: Env) -> Result<()> {
    let mut tree: Arc<NFSCodeEnv::EnvTree::Tree> = Arc::new(NFSCodeEnv::EnvTree::Tree::EMPTY);
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tree = __pa0.clone();
    let _ = NFSCodeEnv::EnvTree::foldCond(tree.clone(), Arc::new(analyseAvlValue), inEnv.clone());
    Ok(())
}

fn analyseAvlValue(mut key: ArcStr, mut value: Item, mut env: Env) -> Result<(Env, bool)> {
    let mut env: Env = env;
    let mut cont: bool = false;
    cont = 'mc: {
        let __mc_input = (value.clone(), env.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { isUsed: Some(is_used), name: Some(_), .. }, tail: _ }) => {
                    let false = (Mutable::access(is_used.clone())) else { bail!("pattern mismatch") };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { classType: cls_ty, env: Deref @ metamodelica::List::Cons { head: cls_env, tail: Deref @ metamodelica::List::Nil }, cls }, _) => {
                    let mut env2: Env = metamodelica::nil();
                    env2 = NFSCodeEnv::enterFrame(cls_env.clone(), env.clone());
                    analyseClassExtendsDef(cls.clone(), cls_ty.clone(), env2.clone())?;
                    analyseClassExtends(env2.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((env, cont))
}

fn analyseClassExtendsDef(mut inClass: Arc<SCode::Element>, mut inClassType: NFSCodeEnv::ClassType, mut inEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inClass.clone(), inClassType.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { info, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: bc, .. }, tail: _ }, .. }, name: cls_name, .. }, NFSCodeEnv::ClassType::CLASS_EXTENDS, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    (item, _, env) = NFSCodeLookup::lookupBaseClassName(bc.clone(), inEnv.clone(), info.clone())?;
                    let true = (NFSCodeEnv::isItemUsed(item.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    env = __pa0.clone();
                    analyseClass(Arc::new(Absyn::Path::IDENT { name: (cls_name.clone()).clone() }), env.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { info, name: cls_name, .. }, NFSCodeEnv::ClassType::USERDEFINED, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let true = (SCodeUtil::isElementRedeclare(inClass.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    env = __pa0.clone();
                    item = Arc::new(NFSCodeEnv::Item::CLASS { cls: inClass.clone(), env: NFSCodeEnv::emptyEnv.clone(), classType: inClassType.clone() });
                    (item, _) = NFSCodeLookup::lookupRedeclaredClassByItem(item.clone(), env.clone(), info.clone())?;
                    let true = (NFSCodeEnv::isItemUsed(item.clone())) else { bail!("pattern mismatch") };
                    analyseClass(Arc::new(Absyn::Path::IDENT { name: (cls_name.clone()).clone() }), env.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn collectUsedProgram(mut inEnv: Env, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inClassName: Arc<Absyn::Path>) -> Result<(Env, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut outEnv: Env = metamodelica::nil();
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut env: Env = metamodelica::nil();
    let mut cls_and_vars: Arc<NFSCodeEnv::EnvTree::Tree> = Arc::new(NFSCodeEnv::EnvTree::Tree::EMPTY);
    env = NFSCodeEnv::buildInitialEnv()?;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cls_and_vars = __pa0.clone();
    (outProgram, outEnv) = collectUsedProgram2(cls_and_vars.clone(), inEnv.clone(), inProgram.clone(), inClassName.clone(), env.clone())?;
    Ok((outEnv, outProgram))
}

fn collectUsedProgram2(mut clsAndVars: Arc<NFSCodeEnv::EnvTree::Tree>, mut inEnv: Env, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inClassName: Arc<Absyn::Path>, mut inAccumEnv: Env) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Env)> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outAccumEnv: Env = metamodelica::nil();
    (outProgram, outAccumEnv) = 'mc: {
        let __mc_input = (clsAndVars.clone(), inEnv.clone(), inProgram.clone(), inClassName.clone(), inAccumEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((inProgram.clone(), inAccumEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: cls @ Deref @ SCode::Element::CLASS { name, .. }, tail: rest_prog }, _, env) => {
                    let mut cls_el: Arc<SCode::Element>;
                    let mut rest_prog = (*rest_prog).clone();
                    let mut env = (*env).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(collectUsedClass(cls.clone(), inEnv.clone(), clsAndVars.clone(), inClassName.clone(), env.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?) {
                        (__pa0 @ Deref @ SCode::Element::CLASS { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls_el = __pa0.clone();
                    env = __pa1.clone();
                    (rest_prog, env) = collectUsedProgram2(clsAndVars.clone(), inEnv.clone(), rest_prog.clone(), inClassName.clone(), env.clone())?;
                    Ok((cons(cls_el.clone(), rest_prog.clone()), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { .. }, tail: rest_prog }, _, env) => {
                    let mut rest_prog = (*rest_prog).clone();
                    let mut env = (*env).clone();
                    (rest_prog, env) = collectUsedProgram2(clsAndVars.clone(), inEnv.clone(), rest_prog.clone(), inClassName.clone(), env.clone())?;
                    Ok((rest_prog.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outProgram, outAccumEnv))
}

fn collectUsedClass(mut inClass: Arc<SCode::Element>, mut inEnv: Env, mut inClsAndVars: Arc<NFSCodeEnv::EnvTree::Tree>, mut inClassName: Arc<Absyn::Path>, mut inAccumEnv: Env, mut inAccumPath: Arc<Absyn::Path>) -> Result<(Arc<SCode::Element>, Env)> {
    let mut outClass: Arc<SCode::Element>;
    let mut outAccumEnv: Env = metamodelica::nil();
    (outClass, outAccumEnv) = (::match_deref::match_deref! { match &((inClass.clone(), inEnv.clone(), inClsAndVars.clone(), inClassName.clone(), inAccumEnv.clone(), inAccumPath.clone())) {
        (Deref @ SCode::Element::CLASS { name, prefixes: prefixes @ Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, encapsulatedPrefix: ep, partialPrefix: pp, restriction: res, classDef: cdef, cmt, info }, _, _, _, _, _) => {
            let mut basename: ArcStr = arcstr::literal!("");
            let mut item: Item;
            let mut resolved_item: Item;
            let mut class_frame: Arc<NFSCodeEnv::Frame>;
            let mut class_env: Env = metamodelica::nil();
            let mut env: Env = metamodelica::nil();
            let mut enclosing_env: Env = metamodelica::nil();
            let mut cls: Arc<SCode::Element>;
            let mut cdef = (*cdef).clone();
            item = NFSCodeEnv::EnvTree::get(inClsAndVars.clone(), (name.clone()).clone())?;
            (resolved_item, _) = NFSCodeLookup::resolveAlias(item.clone(), inEnv.clone())?;
            let true = (checkClassUsed(resolved_item.clone(), cdef.clone())) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::getItemEnv(resolved_item.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            class_frame = __pa0.clone();
            enclosing_env = NFSCodeEnv::enterScope(inEnv.clone(), (name.clone()).clone())?;
            (cdef, class_env) = collectUsedClassDef(cdef.clone(), enclosing_env.clone(), class_frame.clone(), inClassName.clone(), inAccumPath.clone())?;
            cls = Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef.clone(), cmt: cmt.clone(), info: info.clone() });
            resolved_item = updateItemEnv(resolved_item.clone(), cls.clone(), class_env.clone())?;
            basename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*arcstr::literal!(NFSCodeEnv::BASE_CLASS_SUFFIX)); ArcStr::from(__mm_s) }).clone();
            env = NFSCodeEnv::extendEnvWithItem(resolved_item.clone(), inAccumEnv.clone(), (basename.clone()).clone())?;
            env = NFSCodeEnv::extendEnvWithItem(item.clone(), env.clone(), (name.clone()).clone())?;
            (cls.clone(), env.clone())
        },
        (Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: ep, partialPrefix: pp, restriction: res, classDef: cdef, cmt, info }, _, _, _, _, _) => {
            let mut item: Item;
            let mut class_frame: Arc<NFSCodeEnv::Frame>;
            let mut class_env: Env = metamodelica::nil();
            let mut env: Env = metamodelica::nil();
            let mut enclosing_env: Env = metamodelica::nil();
            let mut cls: Arc<SCode::Element>;
            let mut cdef = (*cdef).clone();
            let _ = SCodeUtil::replaceableOptConstraint(SCodeUtil::prefixesReplaceable(prefixes.clone())?)?;
            item = NFSCodeEnv::EnvTree::get(inClsAndVars.clone(), (name.clone()).clone())?;
            let true = (checkClassUsed(item.clone(), cdef.clone())) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::getItemEnv(item.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            class_frame = __pa0.clone();
            enclosing_env = NFSCodeEnv::enterScope(inEnv.clone(), (name.clone()).clone())?;
            (cdef, class_env) = collectUsedClassDef(cdef.clone(), enclosing_env.clone(), class_frame.clone(), inClassName.clone(), inAccumPath.clone())?;
            cls = Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef.clone(), cmt: cmt.clone(), info: info.clone() });
            item = updateItemEnv(item.clone(), cls.clone(), class_env.clone())?;
            env = NFSCodeEnv::extendEnvWithItem(item.clone(), inAccumEnv.clone(), (name.clone()).clone())?;
            (cls.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outClass, outAccumEnv))
}

fn checkClassUsed(mut inItem: Item, mut inClassDef: Arc<SCode::ClassDef>) -> bool {
    let mut isUsed: bool = false;
    isUsed = (::match_deref::match_deref! { match &((inItem.clone(), inClassDef.clone())) {
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name: Deref @ "GraphicalAnnotationsProgram____", .. }, .. }, _) => true,
        _ => NFSCodeEnv::isItemUsed(inItem.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isUsed
}

fn updateItemEnv(mut inItem: Item, mut inClass: Arc<SCode::Element>, mut inEnv: Env) -> Result<Item> {
    let mut outItem: Item;
    outItem = (::match_deref::match_deref! { match &((inItem.clone(), inClass.clone(), inEnv.clone())) {
        (Deref @ NFSCodeEnv::Item::CLASS { classType: cls_ty, .. }, _, _) => {
            Arc::new(NFSCodeEnv::Item::CLASS { cls: inClass.clone(), env: inEnv.clone(), classType: cls_ty.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outItem)
}

fn collectUsedClassDef(mut classDef: Arc<SCode::ClassDef>, mut env: Env, mut inClassEnv: Arc<NFSCodeEnv::Frame>, mut inClassName: Arc<Absyn::Path>, mut inAccumPath: Arc<Absyn::Path>) -> Result<(Arc<SCode::ClassDef>, Env)> {
    let mut classDef: Arc<SCode::ClassDef> = classDef;
    let mut env: Env = env;
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::ClassDef::PARTS { elementLst: el, .. } => {
            let mut el = (*el).clone();
            (el, env) = collectUsedElements(el.clone(), env.clone(), inClassEnv.clone(), inClassName.clone(), inAccumPath.clone())?;
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = el.clone());
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: cdef, .. } => {
            let mut cdef = (*cdef).clone();
            (cdef, env) = collectUsedClassDef(cdef.clone(), env.clone(), inClassEnv.clone(), inClassName.clone(), inAccumPath.clone())?;
            assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; composition = cdef.clone());
            ()
        },
        _ => {
            env = list![inClassEnv.clone()];
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((classDef, env))
}

fn collectUsedElements(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnv: Env, mut inClassEnv: Arc<NFSCodeEnv::Frame>, mut inClassName: Arc<Absyn::Path>, mut inAccumPath: Arc<Absyn::Path>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Env)> {
    let mut outUsedElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outNewEnv: Env = metamodelica::nil();
    let mut empty_class_env: Arc<NFSCodeEnv::Frame>;
    let mut cls_and_vars: Arc<NFSCodeEnv::EnvTree::Tree> = Arc::new(NFSCodeEnv::EnvTree::Tree::EMPTY);
    let mut collect_constants: bool = false;
    (empty_class_env, cls_and_vars) = NFSCodeEnv::removeClsAndVarsFromFrame(inClassEnv.clone())?;
    collect_constants = AbsynUtil::pathEqual(inClassName.clone(), inAccumPath.clone());
    (outUsedElements, outNewEnv) = collectUsedElements2(inElements.clone(), inEnv.clone(), cls_and_vars.clone(), metamodelica::nil(), list![empty_class_env.clone()], inClassName.clone(), inAccumPath.clone(), collect_constants.clone());
    outNewEnv = removeUnusedRedeclares(outNewEnv.clone(), inEnv.clone())?;
    Ok((outUsedElements, outNewEnv))
}

fn collectUsedElements2(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnclosingEnv: Env, mut inClsAndVars: Arc<NFSCodeEnv::EnvTree::Tree>, mut inAccumElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inAccumEnv: Env, mut inClassName: Arc<Absyn::Path>, mut inAccumPath: Arc<Absyn::Path>, mut inCollectConstants: bool) -> (Arc<metamodelica::List<Arc<SCode::Element>>>, Env) {
    let mut outAccumElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut accum_env: Env = inAccumEnv.clone();
    let mut accum_el: Arc<SCode::Element>;
    for mut el in &*inElements.clone() {
        let mut el = el.clone();
        if '__try0: {
            (accum_el, accum_env) = unwrap_break_err!(collectUsedElement(el.clone(), inEnclosingEnv.clone(), inClsAndVars.clone(), accum_env.clone(), inClassName.clone(), inAccumPath.clone(), inCollectConstants.clone()), '__try0);
            outAccumElements = cons(accum_el.clone(), outAccumElements.clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    outAccumElements = outAccumElements.clone().reverse();
    (outAccumElements, accum_env)
}

fn collectUsedElement(mut inElement: Arc<SCode::Element>, mut inEnclosingEnv: Env, mut inClsAndVars: Arc<NFSCodeEnv::EnvTree::Tree>, mut inAccumEnv: Env, mut inClassName: Arc<Absyn::Path>, mut inAccumPath: Arc<Absyn::Path>, mut inCollectConstants: bool) -> Result<(Arc<SCode::Element>, Env)> {
    let mut outElement: Arc<SCode::Element>;
    let mut outAccumEnv: Env = metamodelica::nil();
    (outElement, outAccumEnv) = (::match_deref::match_deref! { match &((inElement.clone(), inEnclosingEnv.clone(), inClsAndVars.clone(), inAccumEnv.clone(), inClassName.clone(), inAccumPath.clone(), inCollectConstants.clone())) {
        (Deref @ SCode::Element::CLASS { name, .. }, _, _, env, _, _, _) => {
            let mut cls: Arc<SCode::Element>;
            let mut cls_path: Arc<Absyn::Path>;
            let mut env = (*env).clone();
            cls_path = AbsynUtil::joinPaths(inAccumPath.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
            (cls, env) = collectUsedClass(inElement.clone(), inEnclosingEnv.clone(), inClsAndVars.clone(), inClassName.clone(), env.clone(), cls_path.clone())?;
            (cls.clone(), env.clone())
        },
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::CONST, .. }, name, .. }, _, _, _, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            let mut item: Item;
            item = NFSCodeEnv::EnvTree::get(inClsAndVars.clone(), (name.clone()).clone())?;
            let true = (inCollectConstants.clone() || NFSCodeEnv::isItemUsed(item.clone())) else { bail!("pattern mismatch") };
            env = NFSCodeEnv::extendEnvWithItem(item.clone(), inAccumEnv.clone(), (name.clone()).clone())?;
            (inElement.clone(), env.clone())
        },
        (Deref @ SCode::Element::COMPONENT { name, .. }, _, _, _, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            let mut item: Item;
            item = NFSCodeEnv::newVarItem(inElement.clone(), true);
            env = NFSCodeEnv::extendEnvWithItem(item.clone(), inAccumEnv.clone(), (name.clone()).clone())?;
            (inElement.clone(), env.clone())
        },
        _ => {
            (inElement.clone(), inAccumEnv.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outElement, outAccumEnv))
}

fn removeUnusedRedeclares(mut inEnv: Env, mut inTotalEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut ty: NFSCodeEnv::FrameType = NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE;
    let mut cls_and_vars: Arc<NFSCodeEnv::EnvTree::Tree> = Arc::new(NFSCodeEnv::EnvTree::Tree::EMPTY);
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let mut imps: NFSCodeEnv::ImportTable;
    let mut is_used: Option<Mutable::Mutable<bool>> = None;
    let mut env: Env = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { name: __pa0, frameType: __pa1, clsAndVars: __pa2, extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa3, redeclaredElements: __pa4, classExtendsInfo: __pa5 }, importTable: __pa6, isUsed: __pa7 }, tail: Deref @ metamodelica::List::Nil } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    cls_and_vars = __pa2.clone();
    bcl = __pa3.clone();
    re = __pa4.clone();
    cei = __pa5.clone();
    imps = __pa6.clone();
    is_used = __pa7.clone();
    env = NFSCodeEnv::removeRedeclaresFromLocalScope(inTotalEnv.clone())?;
    bcl = List::map1(bcl.clone(), Arc::new(removeUnusedRedeclares2), env.clone());
    outEnv = list![Arc::new(NFSCodeEnv::Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: cls_and_vars.clone(), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: bcl.clone(), redeclaredElements: re.clone(), classExtendsInfo: cei.clone() }), importTable: imps.clone(), isUsed: is_used.clone() })];
    Ok(outEnv)
}

fn removeUnusedRedeclares2(mut inExtends: Arc<NFSCodeEnv::Extends>, mut inEnv: Env) -> Result<Arc<NFSCodeEnv::Extends>> {
    let mut outExtends: Arc<NFSCodeEnv::Extends>;
    let mut bc: Arc<Absyn::Path>;
    let mut redeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut info: SourceInfo;
    let mut env: Env = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inExtends.clone()) {
        Deref @ NFSCodeEnv::Extends { baseClass: __pa0, redeclareModifiers: __pa1, index: __pa2, info: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    bc = __pa0.clone();
    redeclares = __pa1.clone();
    index = __pa2.clone();
    info = __pa3.clone();
    redeclares = List::filter1(redeclares.clone(), Arc::new(removeUnusedRedeclares3), inEnv.clone());
    outExtends = Arc::new(NFSCodeEnv::Extends { baseClass: bc.clone(), redeclareModifiers: redeclares.clone(), index: index.clone(), info: info.clone() });
    Ok(outExtends)
}

fn removeUnusedRedeclares3(mut inRedeclare: Arc<NFSCodeEnv::Redeclaration>, mut inEnv: Env) -> Result<()> {
    let mut name: ArcStr = arcstr::literal!("");
    let mut item: Item;
    (name, _) = NFSCodeEnv::getRedeclarationNameInfo(inRedeclare.clone())?;
    (item, _, _) = NFSCodeLookup::lookupSimpleName((name.clone()).clone(), inEnv.clone())?;
    let true = (NFSCodeEnv::isItemUsed(item.clone())) else { bail!("pattern mismatch") };
    Ok(())
}

