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

use crate::NFInstTypes;
use crate::NFSCodeCheck;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeEnv;
use crate::NFSCodeLookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type Env = Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>;

pub type Item = Arc<NFSCodeEnv::Item>;

pub type Extends = Arc<NFSCodeEnv::Extends>;

pub type Prefix = Arc<NFInstPrefix::Prefix>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Replacement {
    /// an item got replaced
    REPLACED {
        name: ArcStr,
        old: Item,
        new: Item,
        env: Env,
    },
    /// the redeclares got pushed into the extends of the base classes
    PUSHED {
        name: ArcStr,
        redeclaredItem: Item,
        baseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>,
        old: Arc<NFSCodeEnv::ExtendsTable>,
        new: Arc<NFSCodeEnv::ExtendsTable>,
        env: Env,
    },
}
pub use self::Replacement::{REPLACED,PUSHED};

pub type Replacements = Arc<metamodelica::List<Replacement>>;

pub static emptyReplacements: std::sync::LazyLock<Arc<metamodelica::List<Replacement>>> = std::sync::LazyLock::new(|| { metamodelica::nil() });

pub fn addElementRedeclarationsToEnv(mut inRedeclares: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnv: Env) -> Env {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = List::fold(inRedeclares.clone(), (std::sync::Arc::new(addElementRedeclarationsToEnv2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> + 'static>), inEnv.clone());
    outEnv
}

fn addElementRedeclarationsToEnv2(mut inRedeclare: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = 'mc: {
        let __mc_input = (inRedeclare.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut env_path: Arc<Absyn::Path>;
                    let mut ext_pathl: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut env: Env = metamodelica::nil();
                    let mut item: Item;
                    name = (SCodeUtil::elementName(inRedeclare.clone())?).clone();
                    info = SCodeUtil::elementInfo(inRedeclare.clone());
                    ext_pathl = lookupElementRedeclaration((name.clone()).clone(), inEnv.clone(), info.clone())?;
                    env_path = NFSCodeEnv::getEnvPath(inEnv.clone())?;
                    item = Arc::new(NFSCodeEnv::Item::ALIAS { name: (name.clone()).clone(), path: Some(env_path.clone()), info: info.clone() });
                    env = addRedeclareToEnvExtendsTable(item.clone(), ext_pathl.clone(), inEnv.clone(), info.clone())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeFlattenRedeclare.addElementRedeclarationsToEnv failed for ")); __mm_s.push_str(&*SCodeUtil::elementName(inRedeclare.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

fn lookupElementRedeclaration(mut inName: ArcStr, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outPaths = 'mc: {
        let __mc_input = (inName.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    paths = NFSCodeLookup::lookupBaseClasses((inName.clone()).clone(), inEnv.clone())?;
                    Ok(paths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::REDECLARE_NONEXISTING_ELEMENT.clone(), list![(inName.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPaths)
}

fn addRedeclareToEnvExtendsTable(mut inRedeclaredElement: Item, mut inBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(NFSCodeEnv::getEnvExtendsTable(inEnv.clone())?) {
        Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0, redeclaredElements: __pa1, classExtendsInfo: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    bcl = __pa0.clone();
    re = __pa1.clone();
    cei = __pa2.clone();
    bcl = addRedeclareToEnvExtendsTable2(inRedeclaredElement.clone(), inBaseClasses.clone(), bcl.clone())?;
    outEnv = NFSCodeEnv::setEnvExtendsTable(Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: bcl.clone(), redeclaredElements: re.clone(), classExtendsInfo: cei.clone() }), inEnv.clone())?;
    Ok(outEnv)
}

fn addRedeclareToEnvExtendsTable2(mut inRedeclaredElement: Item, mut inBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>) -> Result<Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>> {
    let mut outExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    outExtends = 'mc: {
        let __mc_input = (inRedeclaredElement.clone(), inBaseClasses.clone(), inExtends.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: bc1, tail: rest_bc }, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Extends { baseClass: bc2, redeclareModifiers: el, index, info }, tail: exl }) => {
                    let mut ex: Extends = Arc::new(<NFSCodeEnv::Extends as ::std::default::Default>::default());
                    let mut redecl: Arc<NFSCodeEnv::Redeclaration>;
                    let mut exl = (*exl).clone();
                    let true = (AbsynUtil::pathEqual(bc1.clone(), bc2.clone())) else { bail!("pattern mismatch") };
                    redecl = Arc::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: inRedeclaredElement.clone() });
                    NFSCodeCheck::checkDuplicateRedeclarations(redecl.clone(), el.clone())?;
                    ex = Arc::new(NFSCodeEnv::Extends { baseClass: bc2.clone(), redeclareModifiers: cons(redecl.clone(), el.clone()), index: index.clone(), info: info.clone() });
                    exl = addRedeclareToEnvExtendsTable2(inRedeclaredElement.clone(), rest_bc.clone(), exl.clone())?;
                    Ok(cons(ex.clone(), exl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(inExtends.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: ex, tail: exl }) => {
                    let mut exl = (*exl).clone();
                    exl = addRedeclareToEnvExtendsTable2(inRedeclaredElement.clone(), inBaseClasses.clone(), exl.clone())?;
                    Ok(cons(ex.clone(), exl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExtends)
}

pub fn processRedeclare(mut inRedeclare: Arc<NFSCodeEnv::Redeclaration>, mut inEnv: Env, mut inPrefix: Arc<NFInstPrefix::Prefix>) -> Result<Arc<NFSCodeEnv::Redeclaration>> {
    let mut outRedeclare: Arc<NFSCodeEnv::Redeclaration>;
    outRedeclare = 'mc: {
        let __mc_input = (inRedeclare.clone(), inEnv.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: el @ Deref @ SCode::Element::CLASS { .. } }, _, _) => {
                    let mut el_item: Item;
                    let mut redecl_item: Item;
                    let mut cls_env: Env = metamodelica::nil();
                    cls_env = NFSCodeEnv::makeClassEnvironment(el.clone(), true)?;
                    el_item = NFSCodeEnv::newClassItem(el.clone(), cls_env.clone(), crate::NFSCodeEnv::ClassType::USERDEFINED);
                    redecl_item = Arc::new(NFSCodeEnv::Item::REDECLARED_ITEM { item: el_item.clone(), declaredEnv: inEnv.clone() });
                    Ok(Arc::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: redecl_item.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: el @ Deref @ SCode::Element::COMPONENT { .. } }, _, _) => {
                    let mut el_item: Item;
                    let mut redecl_item: Item;
                    el_item = NFSCodeEnv::newVarItem(el.clone(), true);
                    redecl_item = Arc::new(NFSCodeEnv::Item::REDECLARED_ITEM { item: el_item.clone(), declaredEnv: inEnv.clone() });
                    Ok(Arc::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: redecl_item.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { .. }, _, _) => {
                    Ok(inRedeclare.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeFlattenRedeclare.processRedeclare failed on ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(NFSCodeEnv::getRedeclarationElement(inRedeclare.clone())?, SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*AbsynUtil::pathString(NFSCodeEnv::getEnvPath(inEnv.clone())?, (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRedeclare)
}

pub fn replaceRedeclares(mut inRedeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>, mut inClassItem: Item, mut inClassEnv: Env, mut inElementEnv: Env, mut inReplaceRedeclares: NFSCodeLookup::RedeclareReplaceStrategy) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outEnv) = 'mc: {
        let __mc_input = (inRedeclares.clone(), inClassItem.clone(), inClassEnv.clone(), inElementEnv.clone(), inReplaceRedeclares.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES) => {
                    Ok((Some(inClassItem.clone()), Some(inClassEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, NFSCodeLookup::RedeclareReplaceStrategy::INSERT_REDECLARES) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    (item, env, _) = replaceRedeclaredElementsInEnv(inRedeclares.clone(), inClassItem.clone(), inClassEnv.clone(), inElementEnv.clone(), NFInstPrefix::emptyPrefix().clone())?;
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

pub fn replaceRedeclaredElementsInEnv(mut inRedeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>, mut inItem: Item, mut inTypeEnv: Env, mut inElementEnv: Env, mut inPrefix: Arc<NFInstPrefix::Prefix>) -> Result<(Item, Env, Replacements)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    let mut outReplacements: Replacements = metamodelica::nil();
    (outItem, outEnv, outReplacements) = 'mc: {
        let __mc_input = (inRedeclares.clone(), inItem.clone(), inTypeEnv.clone(), inElementEnv.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok((inItem.clone(), inTypeEnv.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ NFSCodeEnv::Item::CLASS { classType: cls_ty, env: Deref @ metamodelica::List::Cons { head: item_env, tail: Deref @ metamodelica::List::Nil }, cls }, _, _, _) => {
                    let mut env: Env = metamodelica::nil();
                    let mut redecls: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
                    let mut repl: Replacements = metamodelica::nil();
                    let mut item_env = (*item_env).clone();
                    env = NFSCodeEnv::enterFrame(item_env.clone(), inTypeEnv.clone());
                    redecls = List::map2(inRedeclares.clone(), (std::sync::Arc::new(processRedeclare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Redeclaration>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<NFInstPrefix::Prefix>) -> Result<Arc<NFSCodeEnv::Redeclaration>> + 'static>), inElementEnv.clone(), inPrefix.clone());
                    (env, repl) = List::fold(redecls.clone(), (std::sync::Arc::new(replaceRedeclaredElementInEnv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Redeclaration>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)> + 'static>), (env.clone(), emptyReplacements.clone()));
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(env.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item_env = __pa0.clone();
                    env = __pa1.clone();
                    Ok((Arc::new(NFSCodeEnv::Item::CLASS { cls: cls.clone(), env: list![item_env.clone()], classType: cls_ty.clone() }), env.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- NFSCodeFlattenRedeclare.replaceRedeclaredElementsInEnv failed for:\n\t")).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("redeclares: ")); __mm_s.push_str(&*stringDelimitList(List::map(inRedeclares.clone(), (std::sync::Arc::new(NFSCodeEnv::printRedeclarationStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Redeclaration>) -> Result<ArcStr> + 'static>)), (literal!("\n---------\n")).clone())); __mm_s.push_str(&*literal!("\n\titem: ")); __mm_s.push_str(&*NFSCodeEnv::itemStr(inItem.clone())?); __mm_s.push_str(&*literal!("\n\tin scope:")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inElementEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outEnv, outReplacements))
}

pub fn extractRedeclaresFromModifier(mut inMod: Arc<SCode::Mod>) -> Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> {
    let mut outRedeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
    outRedeclares = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { subModLst: sub_mods, .. } => {
            let mut redeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
            redeclares = List::fold(sub_mods.clone(), (std::sync::Arc::new(extractRedeclareFromSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>, Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>) -> Result<Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>> + 'static>), metamodelica::nil());
            redeclares.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outRedeclares
}

fn extractRedeclareFromSubMod(mut inMod: Arc<SCode::SubMod>, mut inRedeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>) -> Result<Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>> {
    let mut outRedeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
    outRedeclares = (::match_deref::match_deref! { match &((inMod.clone(), inRedeclares.clone())) {
        (Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::REDECL { element: el, .. }, .. }, _) => {
            let mut redecl: Arc<NFSCodeEnv::Redeclaration>;
            redecl = Arc::new(NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: el.clone() });
            NFSCodeCheck::checkDuplicateRedeclarations(redecl.clone(), inRedeclares.clone())?;
            cons(redecl.clone(), inRedeclares.clone())
        },
        _ => {
            inRedeclares.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRedeclares)
}

fn replaceRedeclaredElementInEnv(mut inRedeclare: Arc<NFSCodeEnv::Redeclaration>, mut inEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)> {
    let mut outEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>);
    outEnv = 'mc: {
        let __mc_input = (inRedeclare.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: item }, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut envRpl: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>);
                    name = (NFSCodeEnv::getItemName(item.clone())?).clone();
                    envRpl = pushRedeclareIntoExtendsNoFail((name.clone()).clone(), item.clone(), inEnv.clone())?;
                    Ok(replaceElementInScope((name.clone()).clone(), item.clone(), envRpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: item }, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut bcl: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    name = (NFSCodeEnv::getItemName(item.clone())?).clone();
                    bcl = NFSCodeLookup::lookupBaseClasses((name.clone()).clone(), Util::tuple21(inEnv.clone()))?;
                    Ok(pushRedeclareIntoExtends((name.clone()).clone(), item.clone(), bcl.clone(), inEnv.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: item }, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut scope_name: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    scope_name = (NFSCodeEnv::getScopeName(Util::tuple21(inEnv.clone()))?).clone();
                    name = (NFSCodeEnv::getItemName(item.clone())?).clone();
                    info = NFSCodeEnv::getItemInfo(item.clone())?;
                    Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(name.clone()).clone(), (scope_name.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

fn pushRedeclareIntoExtendsNoFail(mut inName: ArcStr, mut inRedeclare: Item, mut inEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)> {
    let mut outEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>);
    outEnv = 'mc: {
        let __mc_input = (inName.clone(), inRedeclare.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut bcl: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut envRpl: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>);
                    bcl = NFSCodeLookup::lookupBaseClasses((inName.clone()).clone(), Util::tuple21(inEnv.clone()))?;
                    envRpl = pushRedeclareIntoExtends((inName.clone()).clone(), inRedeclare.clone(), bcl.clone(), inEnv.clone())?;
                    Ok(envRpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inEnv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

fn pushRedeclareIntoExtends(mut inName: ArcStr, mut inRedeclare: Item, mut inBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)> {
    let mut outEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>);
    let mut exts: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let mut etNew: Arc<NFSCodeEnv::ExtendsTable> = Arc::new(<NFSCodeEnv::ExtendsTable as ::std::default::Default>::default());
    let mut etOld: Arc<NFSCodeEnv::ExtendsTable> = Arc::new(<NFSCodeEnv::ExtendsTable as ::std::default::Default>::default());
    let mut name: ArcStr = arcstr::literal!("");
    let mut env: Env = metamodelica::nil();
    let mut repl: Replacements = metamodelica::nil();
    (env, repl) = inEnv.clone();
    let (__pa3, __pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(env.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: __pa3 @ Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0, redeclaredElements: __pa1, classExtendsInfo: __pa2 }, .. }, tail: _ } => (__pa3.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exts = __pa0.clone();
    re = __pa1.clone();
    cei = __pa2.clone();
    etOld = __pa3.clone();
    exts = pushRedeclareIntoExtends2((inName.clone()).clone(), inRedeclare.clone(), inBaseClasses.clone(), exts.clone())?;
    etNew = Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: exts.clone(), redeclaredElements: re.clone(), classExtendsInfo: cei.clone() });
    env = NFSCodeEnv::setEnvExtendsTable(etNew.clone(), env.clone())?;
    repl = cons(Replacement::PUSHED { name: (inName.clone()).clone(), redeclaredItem: inRedeclare.clone(), baseClasses: inBaseClasses.clone(), old: etOld.clone(), new: etNew.clone(), env: env.clone() }, repl.clone());
    outEnv = (env.clone(), repl.clone());
    Ok(outEnv)
}

fn pushRedeclareIntoExtends2(mut inName: ArcStr, mut inRedeclare: Item, mut inBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>) -> Result<Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>> {
    let mut outExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    outExtends = (::match_deref::match_deref! { match &((inName.clone(), inRedeclare.clone(), inBaseClasses.clone(), inExtends.clone())) {
        (_, _, Deref @ metamodelica::List::Cons { head: bc1, tail: rest_bc }, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Extends { baseClass: bc2, redeclareModifiers: redecls, index, info }, tail: rest_exts }) if (AbsynUtil::pathEqual(bc1.clone(), bc2.clone())) => {
            let mut redecls = (*redecls).clone();
            let mut rest_exts = (*rest_exts).clone();
            redecls = pushRedeclareIntoExtends3(inRedeclare.clone(), (inName.clone()).clone(), redecls.clone(), metamodelica::nil())?;
            rest_exts = pushRedeclareIntoExtends2((inName.clone()).clone(), inRedeclare.clone(), rest_bc.clone(), rest_exts.clone())?;
            cons(Arc::new(NFSCodeEnv::Extends { baseClass: bc2.clone(), redeclareModifiers: redecls.clone(), index: index.clone(), info: info.clone() }), rest_exts.clone())
        },
        (_, _, rest_bc, Deref @ metamodelica::List::Cons { head: ext, tail: rest_exts }) => {
            let mut rest_exts = (*rest_exts).clone();
            rest_exts = pushRedeclareIntoExtends2((inName.clone()).clone(), inRedeclare.clone(), rest_bc.clone(), rest_exts.clone())?;
            cons(ext.clone(), rest_exts.clone())
        },
        (_, _, Deref @ metamodelica::List::Nil, _) => {
            inExtends.clone()
        },
        (_, _, _, Deref @ metamodelica::List::Nil) => {
            let mut bc_strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut bcl_str: ArcStr = arcstr::literal!("");
            let mut err_msg: ArcStr = arcstr::literal!("");
            bc_strl = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (inBaseClasses.clone()).into_iter().cloned() {
            let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            bcl_str = stringDelimitList(bc_strl.clone(), (literal!(", ")).clone());
            err_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSCodeFlattenRedeclare.pushRedeclareIntoExtends2 couldn't find the base classes {")); __mm_s.push_str(&*bcl_str.clone()); __mm_s.push_str(&*literal!("} for ")); __mm_s.push_str(&*inName.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(err_msg.clone()).clone()])?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExtends)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn pushRedeclareIntoExtends3(mut inRedeclare: Item, mut inName: ArcStr, mut inRedeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>, mut inOutRedeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>) -> Result<Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>> {
    let mut outRedeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>> = metamodelica::nil();
    outRedeclares = (::match_deref::match_deref! { match &((inRedeclare.clone(), inName.clone(), inRedeclares.clone())) {
        (_, _, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: item }, tail: rest_redecls }) if (stringEqual((NFSCodeEnv::getItemName(item.clone())?).clone(), (inName.clone()).clone())) => {
            List::append_reverse(inOutRedeclares.clone(), cons(Arc::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: inRedeclare.clone() }), rest_redecls.clone()))
        },
        (_, _, Deref @ metamodelica::List::Cons { head: redecl, tail: rest_redecls }) => {
            pushRedeclareIntoExtends3(inRedeclare.clone(), (inName.clone()).clone(), rest_redecls.clone(), cons(redecl.clone(), inOutRedeclares.clone()))?
        },
        (_, _, Deref @ metamodelica::List::Nil) => {
            cons(Arc::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: inRedeclare.clone() }), inOutRedeclares.clone()).reverse()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRedeclares)
}

pub fn replaceElementInScope(mut inElementName: ArcStr, mut inElement: Item, mut inEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>)> {
    let mut outEnv: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Replacement>>);
    outEnv = (::match_deref::match_deref! { match &((inElementName.clone(), inElement.clone(), inEnv.clone())) {
        (_, _, (env @ Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ }, repl)) => {
            let mut old_item: Item;
            let mut new_item: Item;
            let mut env = (*env).clone();
            let mut tree = (*tree).clone();
            let mut repl = (*repl).clone();
            old_item = NFSCodeEnv::EnvTree::get(tree.clone(), (inElementName.clone()).clone())?;
            new_item = propagateItemPrefixes(old_item.clone(), inElement.clone())?;
            new_item = NFSCodeEnv::linkItemUsage(old_item.clone(), new_item.clone());
            tree = NFSCodeEnv::EnvTree::add(tree.clone(), (inElementName.clone()).clone(), new_item.clone(), (std::sync::Arc::new(fnptr!(EnvTree::addConflictReplace, Arc<NFSCodeEnv::Item>, Arc<NFSCodeEnv::Item>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Item>, Arc<NFSCodeEnv::Item>, ArcStr) -> Result<Arc<NFSCodeEnv::Item>> + 'static>))?;
            env = NFSCodeEnv::setEnvClsAndVars(tree.clone(), env.clone())?;
            repl = cons(Replacement::REPLACED { name: (inElementName.clone()).clone(), old: old_item.clone(), new: new_item.clone(), env: env.clone() }, repl.clone());
            (env.clone(), repl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn propagateItemPrefixes(mut inOriginalItem: Item, mut inNewItem: Item) -> Result<Item> {
    let mut outNewItem: Item;
    outNewItem = (::match_deref::match_deref! { match &((inOriginalItem.clone(), inNewItem.clone())) {
        (Deref @ NFSCodeEnv::Item::VAR { var: el1, .. }, Deref @ NFSCodeEnv::Item::VAR { isUsed: iu2, var: el2 }) => {
            let mut el2 = (*el2).clone();
            el2 = propagateAttributesVar(el1.clone(), el2.clone())?;
            Arc::new(NFSCodeEnv::Item::VAR { var: el2.clone(), isUsed: iu2.clone() })
        },
        (Deref @ NFSCodeEnv::Item::CLASS { cls: el1, .. }, Deref @ NFSCodeEnv::Item::CLASS { classType: ty2, env: env2, cls: el2 }) => {
            let mut el2 = (*el2).clone();
            el2 = propagateAttributesClass(el1.clone(), el2.clone())?;
            Arc::new(NFSCodeEnv::Item::CLASS { cls: el2.clone(), env: env2.clone(), classType: ty2.clone() })
        },
        (Deref @ NFSCodeEnv::Item::ALIAS { .. }, _) => {
            inNewItem.clone()
        },
        (_, Deref @ NFSCodeEnv::Item::ALIAS { .. }) => {
            inNewItem.clone()
        },
        (Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, .. }, _) => {
            propagateItemPrefixes(item.clone(), inNewItem.clone())?
        },
        (_, Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { declaredEnv: env1, item }) => {
            let mut item = (*item).clone();
            item = propagateItemPrefixes(inOriginalItem.clone(), item.clone())?;
            Arc::new(NFSCodeEnv::Item::REDECLARED_ITEM { item: item.clone(), declaredEnv: env1.clone() })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("NFSCodeFlattenRedeclare.propagateAttributes failed on unknown item.")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNewItem)
}

pub fn propagateAttributesVar(mut inOriginalVar: Arc<SCode::Element>, mut inNewVar: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outNewVar: Arc<SCode::Element>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut pref1: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
    let mut pref2: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
    let mut attr1: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    let mut attr2: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let mut cond: Option<Arc<Absyn::Exp>> = None;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inOriginalVar.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: __pa0, prefixes: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    attr1 = __pa0.clone();
    pref1 = __pa1.clone();
    let (__pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(inNewVar.clone()) {
        Deref @ SCode::Element::COMPONENT { name: __pa2, prefixes: __pa3, attributes: __pa4, typeSpec: __pa5, modifications: __pa6, comment: __pa7, condition: __pa8, info: __pa9 } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa2.clone();
    pref2 = __pa3.clone();
    attr2 = __pa4.clone();
    ty = __pa5.clone();
    r#mod = __pa6.clone();
    cmt = __pa7.clone();
    cond = __pa8.clone();
    info = __pa9.clone();
    pref2 = propagatePrefixes(pref1.clone(), pref2.clone())?;
    attr2 = propagateAttributes(attr1.clone(), attr2.clone())?;
    outNewVar = Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: pref2.clone(), attributes: attr2.clone(), typeSpec: ty.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() });
    Ok(outNewVar)
}

pub fn propagateAttributesClass(mut inOriginalClass: Arc<SCode::Element>, mut inNewClass: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outNewClass: Arc<SCode::Element>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut pref1: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
    let mut pref2: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
    let mut ep: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
    let mut pp: SCode::Partial = SCode::Partial::NOT_PARTIAL;
    let mut res: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let mut cdef: Arc<SCode::ClassDef>;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let __pa0 = ::match_deref::match_deref! { match &(inOriginalClass.clone()) {
        Deref @ SCode::Element::CLASS { prefixes: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    pref1 = __pa0.clone();
    let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(inNewClass.clone()) {
        Deref @ SCode::Element::CLASS { name: __pa1, prefixes: __pa2, encapsulatedPrefix: __pa3, partialPrefix: __pa4, restriction: __pa5, classDef: __pa6, cmt: __pa7, info: __pa8 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa1.clone();
    pref2 = __pa2.clone();
    ep = __pa3.clone();
    pp = __pa4.clone();
    res = __pa5.clone();
    cdef = __pa6.clone();
    cmt = __pa7.clone();
    info = __pa8.clone();
    pref2 = propagatePrefixes(pref1.clone(), pref2.clone())?;
    outNewClass = Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: pref2.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef.clone(), cmt: cmt.clone(), info: info.clone() });
    Ok(outNewClass)
}

fn propagatePrefixes(mut inOriginalPrefixes: Arc<SCode::Prefixes>, mut inNewPrefixes: Arc<SCode::Prefixes>) -> Result<Arc<SCode::Prefixes>> {
    let mut outNewPrefixes: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
    let mut vis1: SCode::Visibility = SCode::Visibility::PROTECTED;
    let mut vis2: SCode::Visibility = SCode::Visibility::PROTECTED;
    let mut io1: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut io2: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut rdp: SCode::Redeclare = SCode::Redeclare::NOT_REDECLARE;
    let mut fp: SCode::Final = SCode::Final::FINAL;
    let mut rpp: Arc<SCode::Replaceable> = Arc::new(SCode::Replaceable::NOT_REPLACEABLE);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inOriginalPrefixes.clone()) {
        Deref @ SCode::Prefixes { innerOuter: __pa0, visibility: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    io1 = __pa0.clone();
    vis1 = __pa1.clone();
    let (__pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(inNewPrefixes.clone()) {
        Deref @ SCode::Prefixes { visibility: __pa2, redeclarePrefix: __pa3, finalPrefix: __pa4, innerOuter: __pa5, replaceablePrefix: __pa6 } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vis2 = __pa2.clone();
    rdp = __pa3.clone();
    fp = __pa4.clone();
    io2 = __pa5.clone();
    rpp = __pa6.clone();
    io2 = propagatePrefixInnerOuter(io1.clone(), io2.clone());
    outNewPrefixes = Arc::new(SCode::Prefixes { visibility: vis2.clone(), redeclarePrefix: rdp.clone(), finalPrefix: fp.clone(), innerOuter: io2.clone(), replaceablePrefix: rpp.clone() });
    Ok(outNewPrefixes)
}

fn propagatePrefixInnerOuter(mut inOriginalIO: Absyn::InnerOuter, mut inIO: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut outIO: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    outIO = (match (inOriginalIO.clone(), inIO.clone()) {
        (_, Absyn::InnerOuter::NOT_INNER_OUTER) => inOriginalIO.clone(),
        _ => inIO.clone(),
    });
    outIO
}

fn propagateAttributes(mut inOriginalAttributes: SCode::Attributes, mut inNewAttributes: SCode::Attributes) -> Result<SCode::Attributes> {
    let mut outNewAttributes: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    let mut dims1: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut dims2: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut ct1: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    let mut ct2: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    let mut prl1: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    let mut prl2: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    let mut var1: SCode::Variability = SCode::Variability::CONST;
    let mut var2: SCode::Variability = SCode::Variability::CONST;
    let mut dir1: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut dir2: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut isf1: Absyn::IsField = Absyn::IsField::FIELD;
    let mut isf2: Absyn::IsField = Absyn::IsField::FIELD;
    let SCode::ATTR { arrayDims: __pa0, connectorType: __pa1, parallelism: __pa2, variability: __pa3, direction: __pa4, isField: __pa5 } = (inOriginalAttributes.clone()) else { bail!("pattern mismatch") };
    dims1 = __pa0.clone();
    ct1 = __pa1.clone();
    prl1 = __pa2.clone();
    var1 = __pa3.clone();
    dir1 = __pa4.clone();
    isf1 = __pa5.clone();
    let SCode::ATTR { arrayDims: __pa6, connectorType: __pa7, parallelism: __pa8, variability: __pa9, direction: __pa10, isField: __pa11 } = (inNewAttributes.clone()) else { bail!("pattern mismatch") };
    dims2 = __pa6.clone();
    ct2 = __pa7.clone();
    prl2 = __pa8.clone();
    var2 = __pa9.clone();
    dir2 = __pa10.clone();
    isf2 = __pa11.clone();
    dims2 = propagateArrayDimensions(dims1.clone(), dims2.clone());
    ct2 = propagateConnectorType(ct1.clone(), ct2.clone());
    prl2 = propagateParallelism(prl1.clone(), prl2.clone());
    var2 = propagateVariability(var1.clone(), var2.clone());
    dir2 = propagateDirection(dir1.clone(), dir2.clone());
    isf2 = propagateIsField(isf1.clone(), isf2.clone());
    outNewAttributes = SCode::Attributes { arrayDims: dims2.clone(), connectorType: ct2.clone(), parallelism: prl2.clone(), variability: var2.clone(), direction: dir2.clone(), isField: isf2.clone() };
    Ok(outNewAttributes)
}

fn propagateArrayDimensions(mut inOriginalDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inNewDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outNewDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    outNewDims = (::match_deref::match_deref! { match &((inOriginalDims.clone(), inNewDims.clone())) {
        (_, Deref @ metamodelica::List::Nil) => inOriginalDims.clone(),
        _ => inNewDims.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outNewDims
}

fn propagateConnectorType(mut inOriginalConnectorType: SCode::ConnectorType, mut inNewConnectorType: SCode::ConnectorType) -> SCode::ConnectorType {
    let mut outNewConnectorType: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    outNewConnectorType = (match (inOriginalConnectorType.clone(), inNewConnectorType.clone()) {
        (_, SCode::ConnectorType::POTENTIAL) => inOriginalConnectorType.clone(),
        _ => inNewConnectorType.clone(),
    });
    outNewConnectorType
}

fn propagateParallelism(mut inOriginalParallelism: SCode::Parallelism, mut inNewParallelism: SCode::Parallelism) -> SCode::Parallelism {
    let mut outNewParallelism: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    outNewParallelism = (match (inOriginalParallelism.clone(), inNewParallelism.clone()) {
        (_, SCode::Parallelism::NON_PARALLEL) => inOriginalParallelism.clone(),
        _ => inNewParallelism.clone(),
    });
    outNewParallelism
}

fn propagateVariability(mut inOriginalVariability: SCode::Variability, mut inNewVariability: SCode::Variability) -> SCode::Variability {
    let mut outNewVariability: SCode::Variability = SCode::Variability::CONST;
    outNewVariability = (match (inOriginalVariability.clone(), inNewVariability.clone()) {
        (_, SCode::Variability::VAR) => inOriginalVariability.clone(),
        _ => inNewVariability.clone(),
    });
    outNewVariability
}

fn propagateDirection(mut inOriginalDirection: Absyn::Direction, mut inNewDirection: Absyn::Direction) -> Absyn::Direction {
    let mut outNewDirection: Absyn::Direction = Absyn::Direction::BIDIR;
    outNewDirection = (match (inOriginalDirection.clone(), inNewDirection.clone()) {
        (_, Absyn::Direction::BIDIR) => inOriginalDirection.clone(),
        _ => inNewDirection.clone(),
    });
    outNewDirection
}

fn propagateIsField(mut inOriginalIsField: Absyn::IsField, mut inNewIsField: Absyn::IsField) -> Absyn::IsField {
    let mut outNewIsField: Absyn::IsField = Absyn::IsField::FIELD;
    outNewIsField = (match (inOriginalIsField.clone(), inNewIsField.clone()) {
        (_, Absyn::IsField::NONFIELD) => inOriginalIsField.clone(),
        _ => inNewIsField.clone(),
    });
    outNewIsField
}

fn traceReplaceElementInScope(mut inElementName: ArcStr, mut inOldItem: Item, mut inNewItem: Item, mut inEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inElementName.clone(), inOldItem.clone(), inNewItem.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("replacing element: ")); __mm_s.push_str(&*inElementName.clone()); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); __mm_s.push_str(&*literal!("\n\t")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Old Element:")); __mm_s.push_str(&*NFSCodeEnv::itemStr(inOldItem.clone())?); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(NFSCodeEnv::getItemEnvNoFail(inOldItem.clone())?)?); __mm_s.push_str(&*literal!("\n\t")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("New Element:")); __mm_s.push_str(&*NFSCodeEnv::itemStr(inNewItem.clone())?); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(NFSCodeEnv::getItemEnvNoFail(inNewItem.clone())?)?); __mm_s.push_str(&*literal!("\n===============\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("traceReplaceElementInScope failed on element: ")); __mm_s.push_str(&*inElementName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn tracePushRedeclareIntoExtends(mut inName: ArcStr, mut inRedeclare: Arc<NFSCodeEnv::Item>, mut inBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inEnv: Env, mut inEtNew: Arc<NFSCodeEnv::ExtendsTable>, mut inEtOld: Arc<NFSCodeEnv::ExtendsTable>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inName.clone(), inRedeclare.clone(), inBaseClasses.clone(), inEnv.clone(), inEtNew.clone(), inEtOld.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pushing: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!(" redeclare: ")); __mm_s.push_str(&*NFSCodeEnv::itemStr(inRedeclare.clone())?); __mm_s.push_str(&*literal!("\n\t")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("into baseclases: ")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (inBaseClasses.clone()).into_iter().cloned() {
                    let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n\t")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("called from env: ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", (literal!("-----------------\n")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tracePushRedeclareIntoExtends failed on element: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

