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
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub type Env = Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>;

pub type ClassType = NFSCodeEnv::ClassType;

pub type Extends = Arc<NFSCodeEnv::Extends>;

pub type Frame = Arc<NFSCodeEnv::Frame>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

pub type Item = Arc<NFSCodeEnv::Item>;

pub type ExtendsTableArray = metamodelica::Array<ExtendsWrapper>;

pub const BASECLASS_NOT_FOUND_ERROR: &'static str = "$1";

pub const BASECLASS_INHERITED_ERROR: &'static str = "$2";

pub const BASECLASS_REPLACEABLE_ERROR: &'static str = "$3";

pub const BASECLASS_IS_VAR_ERROR: &'static str = "$4";

pub const BASECLASS_UNKNOWN_ERROR: &'static str = "$5";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExtendsWrapper {
    UNQUALIFIED_EXTENDS {
        ext: Extends,
    },
    QUALIFIED_EXTENDS {
        ext: Extends,
    },
    NO_EXTENDS,
}
pub use self::ExtendsWrapper::{UNQUALIFIED_EXTENDS,QUALIFIED_EXTENDS,NO_EXTENDS};

pub fn update(mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut env: Env = metamodelica::nil();
    env = qualify(inEnv.clone())?;
    outEnv = update2(env.clone())?;
    Ok(outEnv)
}

pub fn qualify(mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ext_count: i32 = 0;
                    let mut ext_table: ExtendsTableArray;
                    ext_count = System::tmpTickIndex(NFSCodeEnv::extendsTickIndex.clone());
                    ext_table = createExtendsTable(ext_count.clone());
                    Ok(qualify2(inEnv.clone(), crate::NFSCodeEnv::ClassType::USERDEFINED, ext_table.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- NFEnvExtends.qualify failed.")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

fn qualify2(mut inEnv: Env, mut inClassType: ClassType, mut inExtendsTable: ExtendsTableArray) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut exts: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let mut env: Env = metamodelica::nil();
    let mut tree: Arc<NFSCodeEnv::EnvTree::Tree> = Arc::new(NFSCodeEnv::EnvTree::Tree::EMPTY);
    env = qualifyLocalScope(inEnv.clone(), inClassType.clone(), inExtendsTable.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tree = __pa0.clone();
    tree = NFSCodeEnv::EnvTree::map(tree.clone(), Arc::new({ let __pe_b2 = env.clone(); let __pe_b3 = inExtendsTable.clone(); move |__pe_a0, __pe_a1| qualify3(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }));
    outEnv = NFSCodeEnv::setEnvClsAndVars(tree.clone(), env.clone())?;
    Ok(outEnv)
}

fn qualify3(mut name: ArcStr, mut item: Item, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<Item> {
    let mut item: Item = item;
    item = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ NFSCodeEnv::Item::CLASS { cls, env: Deref @ metamodelica::List::Cons { head: cls_env, tail: Deref @ metamodelica::List::Nil }, classType: cls_ty } => {
            let mut env: Env = metamodelica::nil();
            let mut cls_env = (*cls_env).clone();
            env = NFSCodeEnv::enterFrame(cls_env.clone(), inEnv.clone());
            let __pa0 = ::match_deref::match_deref! { match &(qualify2(env.clone(), cls_ty.clone(), inExtendsTable.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cls_env = __pa0.clone();
            Arc::new(NFSCodeEnv::Item::CLASS { cls: cls.clone(), env: list![cls_env.clone()], classType: cls_ty.clone() })
        },
        _ => {
            item.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

fn qualifyLocalScope(mut inEnv: Env, mut inClassType: ClassType, mut inExtendsTable: ExtendsTableArray) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut exts: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cei: Option<Arc<SCode::Element>> = None;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(NFSCodeEnv::getEnvExtendsTable(inEnv.clone())?) {
        Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0, redeclaredElements: __pa1, classExtendsInfo: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exts = __pa0.clone();
    re = __pa1.clone();
    cei = __pa2.clone();
    exts = qualifyExtendsList(exts.clone(), inClassType.clone(), inEnv.clone(), inExtendsTable.clone());
    outEnv = NFSCodeEnv::setEnvExtendsTable(Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: exts.clone(), redeclaredElements: re.clone(), classExtendsInfo: cei.clone() }), inEnv.clone())?;
    Ok(outEnv)
}

fn qualifyExtendsList(mut inExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>, mut inClassType: ClassType, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> {
    let mut outExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    outExtends = (::match_deref::match_deref! { match &((inExtends.clone(), inClassType.clone(), inEnv.clone(), inExtendsTable.clone())) {
        (Deref @ metamodelica::List::Cons { head: ext, tail: extl }, NFSCodeEnv::ClassType::CLASS_EXTENDS, _, _) => {
            let mut extl = (*extl).clone();
            extl = List::map2Reverse(extl.clone(), (std::sync::Arc::new(qualifyExtends) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Extends>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, metamodelica::Array<ExtendsWrapper>) -> Result<Arc<NFSCodeEnv::Extends>> + 'static>), inEnv.clone(), inExtendsTable.clone());
            cons(ext.clone(), extl.clone())
        },
        _ => {
            let mut extl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
            extl = List::map2Reverse(inExtends.clone(), (std::sync::Arc::new(qualifyExtends) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Extends>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, metamodelica::Array<ExtendsWrapper>) -> Result<Arc<NFSCodeEnv::Extends>> + 'static>), inEnv.clone(), inExtendsTable.clone());
            extl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExtends
}

fn qualifyExtends(mut inExtends: Extends, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<Extends> {
    let mut outExtends: Extends;
    outExtends = 'mc: {
        let __mc_input = (inExtends.clone(), inEnv.clone(), inExtendsTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Extends { baseClass: Deref @ Absyn::Path::IDENT { name: id }, .. }, _, _) => {
                    let _ = NFSCodeLookup::lookupBuiltinType((id.clone()).clone())?;
                    Ok(inExtends.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut ext: Extends;
                    let __pa0 = ::match_deref::match_deref! { match &(qualifyExtends2(inExtends.clone(), inEnv.clone(), inExtendsTable.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ext = __pa0.clone();
                    Ok(ext.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Extends { baseClass: bc, .. }, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFEnvExtends.qualifyExtends failed on ")); __mm_s.push_str(&*AbsynUtil::pathString(bc.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExtends)
}

fn qualifyExtends2(mut inExtends: Extends, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<Option<Arc<NFSCodeEnv::Extends>>> {
    let mut outExtends: Option<Arc<NFSCodeEnv::Extends>> = None;
    outExtends = 'mc: {
        let __mc_input = (inExtends.clone(), inEnv.clone(), inExtendsTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Extends { index, .. }, _, _) => {
                    Ok(lookupQualifiedExtends(index.clone(), inExtendsTable.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Extends { baseClass: bc, redeclareModifiers: rl, index, info }, _, _) => {
                    let mut ext: Extends;
                    let mut env: Env = metamodelica::nil();
                    let mut bc = (*bc).clone();
                    addUnqualifiedToTable(inExtends.clone(), index.clone(), inExtendsTable.clone())?;
                    env = NFSCodeEnv::removeExtendFromLocalScope(bc.clone(), inEnv.clone())?;
                    bc = qualifyExtends3(bc.clone(), env.clone(), inExtendsTable.clone(), true, bc.clone(), info.clone(), None)?;
                    List::map2_0(rl.clone(), (std::sync::Arc::new(NFSCodeCheck::checkRedeclareModifier) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Redeclaration>, Arc<Absyn::Path>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<()> + 'static>), bc.clone(), inEnv.clone());
                    ext = Arc::new(NFSCodeEnv::Extends { baseClass: bc.clone(), redeclareModifiers: rl.clone(), index: index.clone(), info: info.clone() });
                    updateQualifiedInTable(ext.clone(), index.clone(), inExtendsTable.clone())?;
                    Ok(Some(ext.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExtends)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn qualifyExtends3(mut inBaseClass: Arc<Absyn::Path>, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray, mut inIsFirst: bool, mut inFullPath: Arc<Absyn::Path>, mut inInfo: SourceInfo, mut inErrorPath: Option<Arc<Absyn::Path>>) -> Result<Arc<Absyn::Path>> {
    let mut outBaseClass: Arc<Absyn::Path>;
    outBaseClass = (::match_deref::match_deref! { match &((inBaseClass.clone(), inEnv.clone(), inExtendsTable.clone(), inIsFirst.clone(), inFullPath.clone(), inInfo.clone(), inErrorPath.clone())) {
        (_, _, _, _, _, _, Some(bc)) => {
            bc.clone()
        },
        (Deref @ Absyn::Path::IDENT { name }, _, _, _, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            let mut ep: Option<Arc<Absyn::Path>> = None;
            let mut opath: Option<Arc<Absyn::Path>> = None;
            (opath, env, ep) = qualifyExtendsPart((name.clone()).clone(), inEnv.clone(), inExtendsTable.clone(), inIsFirst.clone(), inFullPath.clone(), inInfo.clone())?;
            makeExtendsPath(opath.clone(), None, env.clone(), ep.clone(), inIsFirst.clone())?
        },
        (Deref @ Absyn::Path::QUALIFIED { path: rest_path, name }, _, _, _, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            let mut ep: Option<Arc<Absyn::Path>> = None;
            let mut opath: Option<Arc<Absyn::Path>> = None;
            let mut rest_path = (*rest_path).clone();
            (opath, env, ep) = qualifyExtendsPart((name.clone()).clone(), inEnv.clone(), inExtendsTable.clone(), inIsFirst.clone(), inFullPath.clone(), inInfo.clone())?;
            rest_path = qualifyExtends3(rest_path.clone(), env.clone(), inExtendsTable.clone(), false, inFullPath.clone(), inInfo.clone(), ep.clone())?;
            makeExtendsPath(opath.clone(), Some(rest_path.clone()), env.clone(), ep.clone(), inIsFirst.clone())?
        },
        (Deref @ Absyn::Path::FULLYQUALIFIED { path: rest_path }, _, _, _, _, _, _) => {
            let mut env: Env = metamodelica::nil();
            env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
            qualifyExtends3(rest_path.clone(), env.clone(), inExtendsTable.clone(), inIsFirst.clone(), rest_path.clone(), inInfo.clone(), None)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBaseClass)
}

fn makeExtendsPath(mut inFirstPath: Option<Arc<Absyn::Path>>, mut inRestPath: Option<Arc<Absyn::Path>>, mut inEnv: Env, mut inErrorPath: Option<Arc<Absyn::Path>>, mut inIsFirst: bool) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inFirstPath.clone(), inRestPath.clone(), inEnv.clone(), inErrorPath.clone(), inIsFirst.clone())) {
        (_, _, _, Some(path), _) => {
            path.clone()
        },
        (_, Some(path @ Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", .. }), _, _, _) => {
            path.clone()
        },
        (_, Some(path @ Deref @ Absyn::Path::FULLYQUALIFIED { .. }), _, _, _) => {
            path.clone()
        },
        (_, _, _, _, true) => {
            let mut path: Arc<Absyn::Path>;
            path = NFSCodeEnv::getEnvPath(inEnv.clone())?;
            path = AbsynUtil::joinPathsOptSuffix(path.clone(), inRestPath.clone())?;
            path = AbsynUtil::makeFullyQualified(path.clone());
            path.clone()
        },
        (Some(path), _, _, _, _) => {
            AbsynUtil::joinPathsOptSuffix(path.clone(), inRestPath.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

fn qualifyExtendsPart(mut inName: ArcStr, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray, mut inIsFirst: bool, mut inFullPath: Arc<Absyn::Path>, mut inInfo: SourceInfo) -> Result<(Option<Arc<Absyn::Path>>, Env, Option<Arc<Absyn::Path>>)> {
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Env = metamodelica::nil();
    let mut outErrorPath: Option<Arc<Absyn::Path>> = None;
    let mut oitem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut oenv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    let mut fe: bool = false;
    (oitem, outPath, oenv, fe) = lookupSimpleName((inName.clone()).clone(), inEnv.clone(), inExtendsTable.clone())?;
    (outEnv, outErrorPath) = qualifyExtendsPart2(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), oitem.clone(), oenv.clone(), inEnv.clone(), inIsFirst.clone(), fe.clone(), inFullPath.clone())?;
    Ok((outPath, outEnv, outErrorPath))
}

fn qualifyExtendsPart2(mut inPartName: Arc<Absyn::Path>, mut inItem: Option<Arc<NFSCodeEnv::Item>>, mut inFoundEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>, mut inOriginEnv: Env, mut inIsFirst: bool, mut inFromExtends: bool, mut inFullPath: Arc<Absyn::Path>) -> Result<(Env, Option<Arc<Absyn::Path>>)> {
    let mut outEnv: Env = metamodelica::nil();
    let mut outErrorPath: Option<Arc<Absyn::Path>> = None;
    (outEnv, outErrorPath) = (::match_deref::match_deref! { match &((inPartName.clone(), inItem.clone(), inFoundEnv.clone(), inOriginEnv.clone(), inIsFirst.clone(), inFromExtends.clone(), inFullPath.clone())) {
        (_, Some(item), Some(env), _, _, _, _) => {
            let mut ep: Option<Arc<Absyn::Path>> = None;
            let mut env = (*env).clone();
            ep = checkExtendsPart(inIsFirst.clone(), inFromExtends.clone(), inPartName.clone(), item.clone(), inFullPath.clone(), env.clone(), inOriginEnv.clone())?;
            env = NFSCodeEnv::mergeItemEnv(item.clone(), env.clone());
            (env.clone(), ep.clone())
        },
        _ => {
            (NFSCodeEnv::emptyEnv.clone(), makeExtendsError(inFullPath.clone(), inPartName.clone(), (arcstr::literal!(BASECLASS_NOT_FOUND_ERROR)).clone())?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEnv, outErrorPath))
}

fn makeExtendsError(mut inBaseClass: Arc<Absyn::Path>, mut inPart: Arc<Absyn::Path>, mut inError: ArcStr) -> Result<Option<Arc<Absyn::Path>>> {
    let mut outError: Option<Arc<Absyn::Path>> = None;
    outError = (::match_deref::match_deref! { match &((inBaseClass.clone(), inPart.clone(), inError.clone())) {
        (_, _, _) => {
            let mut path: Arc<Absyn::Path>;
            path = AbsynUtil::joinPaths(inPart.clone(), Arc::new(Absyn::Path::QUALIFIED { name: (literal!("$bc")).clone(), path: inBaseClass.clone() }))?;
            path = Arc::new(Absyn::Path::QUALIFIED { name: (literal!("$E")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (inError.clone()).clone(), path: path.clone() }) });
            Some(path.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outError)
}

fn checkExtendsPart(mut inIsFirst: bool, mut inFromExtends: bool, mut inPartName: Arc<Absyn::Path>, mut inItem: Item, mut inBaseClass: Arc<Absyn::Path>, mut inFoundEnv: Env, mut inOriginEnv: Env) -> Result<Option<Arc<Absyn::Path>>> {
    let mut outErrorPath: Option<Arc<Absyn::Path>> = None;
    outErrorPath = 'mc: {
        let __mc_input = (inIsFirst.clone(), inFromExtends.clone(), inPartName.clone(), inItem.clone(), inBaseClass.clone(), inFoundEnv.clone(), inOriginEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, true, _, _, _, _, _) => {
                    Ok(makeExtendsError(inBaseClass.clone(), inPartName.clone(), (arcstr::literal!(BASECLASS_INHERITED_ERROR)).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ NFSCodeEnv::Item::CLASS { .. }, _, _, _) => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ NFSCodeEnv::Item::VAR { .. }, _, _, _) => {
                    let mut part: Arc<Absyn::Path>;
                    part = NFSCodeEnv::mergePathWithEnvPath(inPartName.clone(), inFoundEnv.clone())?;
                    Ok(makeExtendsError(inBaseClass.clone(), part.clone(), (arcstr::literal!(BASECLASS_IS_VAR_ERROR)).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(makeExtendsError(inBaseClass.clone(), inPartName.clone(), (arcstr::literal!(BASECLASS_UNKNOWN_ERROR)).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outErrorPath)
}

fn splitExtendsErrorPath(mut inPath: Arc<Absyn::Path>) -> Result<(Arc<Absyn::Path>, Arc<Absyn::Path>)> {
    let mut outBaseClass: Arc<Absyn::Path>;
    let mut outPartPath: Arc<Absyn::Path>;
    (outBaseClass, outPartPath) = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::QUALIFIED { name: part_str, path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$bc", path: bc } } => {
            (bc.clone(), Arc::new(Absyn::Path::IDENT { name: (part_str.clone()).clone() }))
        },
        Deref @ Absyn::Path::QUALIFIED { name: part_str, path: part } => {
            let mut bc: Arc<Absyn::Path>;
            let mut part = (*part).clone();
            (bc, part) = splitExtendsErrorPath(part.clone())?;
            (bc.clone(), Arc::new(Absyn::Path::QUALIFIED { name: (part_str.clone()).clone(), path: part.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outBaseClass, outPartPath))
}

pub fn printExtendsError(mut inErrorPath: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inErrorPath.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { path: bc, name: err_str }, name: Deref @ "$E" }, _, _) => {
                    let mut part: Arc<Absyn::Path>;
                    let mut env: Env = metamodelica::nil();
                    let mut bc = (*bc).clone();
                    (bc, part) = splitExtendsErrorPath(bc.clone())?;
                    env = NFSCodeEnv::removeExtendFromLocalScope(inErrorPath.clone(), inEnv.clone())?;
                    printExtendsError2((err_str.clone()).clone(), bc.clone(), part.clone(), env.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFEnvExtends.printExtendsError failed to print error ")); __mm_s.push_str(&*AbsynUtil::pathString(inErrorPath.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn printExtendsError2(mut inError: ArcStr, mut inBaseClass: Arc<Absyn::Path>, mut inPartPath: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inError.clone(), inBaseClass.clone(), inPartPath.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut bc_str: ArcStr = arcstr::literal!("");
                    let mut env_str: ArcStr = arcstr::literal!("");
                    let true = (stringEq((inError.clone()).clone(), (arcstr::literal!(BASECLASS_NOT_FOUND_ERROR)).clone())) else { bail!("pattern mismatch") };
                    bc_str = (AbsynUtil::pathString(inBaseClass.clone(), (literal!(".")).clone(), true, false)?).clone();
                    env_str = (NFSCodeEnv::getEnvName(inEnv.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_BASECLASS_ERROR.clone(), list![(bc_str.clone()).clone(), (env_str.clone()).clone()], inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::Path::IDENT { name: part }, _, _) => {
                    let mut bc_str: ArcStr = arcstr::literal!("");
                    let mut exts: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
                    let true = (stringEq((inError.clone()).clone(), (arcstr::literal!(BASECLASS_INHERITED_ERROR)).clone())) else { bail!("pattern mismatch") };
                    bc_str = (AbsynUtil::pathString(inBaseClass.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::INHERITED_EXTENDS.clone(), list![(bc_str.clone()).clone()], inInfo.clone())?;
                    exts = NFSCodeEnv::getEnvExtendsFromTable(inEnv.clone())?;
                    printInheritedExtendsError((part.clone()).clone(), exts.clone(), inEnv.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut bc_str: ArcStr = arcstr::literal!("");
                    let mut part: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let true = (stringEq((inError.clone()).clone(), (arcstr::literal!(BASECLASS_REPLACEABLE_ERROR)).clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupFullyQualified(inPartPath.clone(), inEnv.clone())?) {
                        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { info: __pa0, name: __pa1, .. }, .. }, _, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa0.clone();
                    part = __pa1.clone();
                    bc_str = (AbsynUtil::pathString(inBaseClass.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), inInfo.clone())?;
                    Error::addSourceMessage(Error::REPLACEABLE_BASE_CLASS.clone(), list![(part.clone()).clone(), (bc_str.clone()).clone()], info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut bc_str: ArcStr = arcstr::literal!("");
                    let mut part: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let true = (stringEq((inError.clone()).clone(), (arcstr::literal!(BASECLASS_IS_VAR_ERROR)).clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupFullyQualified(inPartPath.clone(), inEnv.clone())?) {
                        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { info: __pa0, name: __pa1, .. }, .. }, _, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa0.clone();
                    part = __pa1.clone();
                    bc_str = (AbsynUtil::pathString(inBaseClass.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), info.clone())?;
                    Error::addSourceMessage(Error::EXTEND_THROUGH_COMPONENT.clone(), list![(part.clone()).clone(), (bc_str.clone()).clone()], inInfo.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printInheritedExtendsError(mut inName: ArcStr, mut inExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>, mut inEnv: Env) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inName.clone(), inExtends.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: ext @ Deref @ NFSCodeEnv::Extends { info: info2, baseClass: bc, .. }, tail: rest_ext }, _) => {
                    let mut item: Item;
                    let mut info1: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut bc_str: ArcStr = arcstr::literal!("");
                    let mut info2 = (*info2).clone();
                    let mut bc = (*bc).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupInBaseClasses3((inName.clone()).clone(), ext.clone(), inEnv.clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES, metamodelica::nil())?) {
                        (Some(__pa0), _, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    info1 = NFSCodeEnv::getItemInfo(item.clone())?;
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(ext.clone()) {
                        Deref @ NFSCodeEnv::Extends { info: __pa1, baseClass: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    info2 = __pa1.clone();
                    bc = __pa2.clone();
                    bc = AbsynUtil::makeNotFullyQualified(bc.clone());
                    bc_str = (AbsynUtil::pathString(bc.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), info1.clone())?;
                    Error::addSourceMessage(Error::EXTENDS_INHERITED_FROM_LOCAL_EXTENDS.clone(), list![(inName.clone()).clone(), (bc_str.clone()).clone()], info2.clone())?;
                    printInheritedExtendsError((inName.clone()).clone(), rest_ext.clone(), inEnv.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest_ext }, _) => {
                    printInheritedExtendsError((inName.clone()).clone(), rest_ext.clone(), inEnv.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn lookupSimpleName(mut inName: ArcStr, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>, bool)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    let mut outFromExtends: bool = false;
    (outItem, outPath, outEnv, outFromExtends) = 'mc: {
        let __mc_input = (inName.clone(), inEnv.clone(), inExtendsTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut fe: bool = false;
                    (opt_item, opt_path, opt_env, fe) = lookupInLocalScope((inName.clone()).clone(), inEnv.clone(), inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone(), fe.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: frame_type, .. }, tail: env }, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    NFSCodeLookup::frameNotEncapsulated(frame_type.clone())?;
                    (opt_item, opt_path, opt_env, _) = lookupSimpleName((inName.clone()).clone(), env.clone(), inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, None, None, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv, outFromExtends))
}

fn lookupInLocalScope(mut inName: ArcStr, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>, bool)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    let mut outFromExtends: bool = false;
    (outItem, outPath, outEnv, outFromExtends) = 'mc: {
        let __mc_input = (inName.clone(), inEnv.clone(), inExtendsTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    (item, env) = NFSCodeLookup::lookupInClass((inName.clone()).clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() })), Some(env.clone()), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: bcl @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, tail: _ }, _) => {
                    let mut oitem: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut oenv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (oitem, oenv) = lookupInBaseClasses((inName.clone()).clone(), bcl.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    Ok((oitem.clone(), Some(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() })), oenv.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { qualifiedImports: imps, hidden: false, .. }, .. }, tail: _ }, _) => {
                    let mut oitem: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opath: Option<Arc<Absyn::Path>> = None;
                    let mut oenv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (oitem, opath, oenv) = lookupInQualifiedImports((inName.clone()).clone(), imps.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    Ok((oitem.clone(), opath.clone(), oenv.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { unqualifiedImports: imps, hidden: false, .. }, .. }, tail: _ }, _) => {
                    let mut oitem: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opath: Option<Arc<Absyn::Path>> = None;
                    let mut oenv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (oitem, opath, oenv) = lookupInUnqualifiedImports((inName.clone()).clone(), imps.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    Ok((oitem.clone(), opath.clone(), oenv.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv, outFromExtends))
}

fn lookupInBaseClasses(mut inName: ArcStr, mut inExtends: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inExtends.clone(), inEnv.clone(), inExtendsTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: ext, tail: _ }, _, _) => {
                    let mut opt_ext: Option<Arc<NFSCodeEnv::Extends>> = None;
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    let mut env: Env = metamodelica::nil();
                    env = NFSCodeEnv::setImportTableHidden(inEnv.clone(), false)?;
                    opt_ext = qualifyExtends2(ext.clone(), env.clone(), inExtendsTable.clone())?;
                    (opt_item, opt_env) = lookupInBaseClasses2((inName.clone()).clone(), opt_ext.clone(), env.clone(), inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest_ext }, _, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (opt_item, opt_env) = lookupInBaseClasses((inName.clone()).clone(), rest_ext.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outEnv))
}

fn lookupInBaseClasses2(mut inName: ArcStr, mut inExtends: Option<Arc<NFSCodeEnv::Extends>>, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inExtends.clone(), inEnv.clone(), inExtendsTable.clone())) {
        (_, Some(Deref @ NFSCodeEnv::Extends { baseClass: Deref @ Absyn::Path::FULLYQUALIFIED { path: bc }, .. }), _, _) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
            let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
            (item, env) = lookupFullyQualified(bc.clone(), inEnv.clone(), inExtendsTable.clone())?;
            env = NFSCodeEnv::mergeItemEnv(item.clone(), env.clone());
            env = NFSCodeEnv::setImportTableHidden(env.clone(), true)?;
            (opt_item, _, opt_env, _) = lookupInLocalScope((inName.clone()).clone(), env.clone(), inExtendsTable.clone())?;
            (opt_item.clone(), opt_env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outEnv))
}

fn lookupInQualifiedImports(mut inName: ArcStr, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inImports.clone(), inEnv.clone(), inExtendsTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: rest_imps }, _, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    let false = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (opt_item, opt_path, opt_env) = lookupInQualifiedImports((inName.clone()).clone(), rest_imps.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { path, name }, tail: _ }, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut path = (*path).clone();
                    let true = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (item, env) = lookupFullyQualified(path.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    path = NFSCodeEnv::prefixIdentWithEnv((inName.clone()).clone(), env.clone())?;
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: _ }, _, _) => {
                    let true = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((None, None, None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

fn lookupInUnqualifiedImports(mut inName: ArcStr, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>> = None;
    let mut outPath: Option<Arc<Absyn::Path>> = None;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inImports.clone(), inEnv.clone(), inExtendsTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ }, _, _) => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut path = (*path).clone();
                    (item, env) = lookupFullyQualified(path.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    env = NFSCodeEnv::mergeItemEnv(item.clone(), env.clone());
                    (item, env) = lookupFullyQualified2(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), env.clone(), inExtendsTable.clone())?;
                    path = NFSCodeEnv::prefixIdentWithEnv((inName.clone()).clone(), env.clone())?;
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest_imps }, _, _) => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>> = None;
                    let mut opt_path: Option<Arc<Absyn::Path>> = None;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = None;
                    (opt_item, opt_path, opt_env) = lookupInUnqualifiedImports((inName.clone()).clone(), rest_imps.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

fn lookupFullyQualified(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    let mut env: Env = metamodelica::nil();
    env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
    (outItem, outEnv) = lookupFullyQualified2(inName.clone(), env.clone(), inExtendsTable.clone())?;
    Ok((outItem, outEnv))
}

fn lookupFullyQualified2(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env = metamodelica::nil();
    (outItem, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inEnv.clone(), inExtendsTable.clone())) {
        (Deref @ Absyn::Path::IDENT { name }, _, _) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), inExtendsTable.clone())?) {
                (Some(__pa0), _, Some(__pa1), _) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            item = __pa0.clone();
            env = __pa1.clone();
            (item.clone(), env.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { path: rest_path, name }, _, _) => {
            let mut item: Item;
            let mut env: Env = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), inExtendsTable.clone())?) {
                (Some(__pa0), _, Some(__pa1), _) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            item = __pa0.clone();
            env = __pa1.clone();
            env = NFSCodeEnv::mergeItemEnv(item.clone(), env.clone());
            (item, env) = lookupFullyQualified2(rest_path.clone(), env.clone(), inExtendsTable.clone())?;
            (item.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outEnv))
}

fn createExtendsTable(mut inSize: i32) -> ExtendsTableArray {
    let mut outTable: ExtendsTableArray;
    outTable = arrayCreate(inSize.clone(), crate::NFEnvExtends::ExtendsWrapper::NO_EXTENDS);
    outTable
}

fn lookupQualifiedExtends(mut inIndex: i32, mut inExtendsTable: ExtendsTableArray) -> Result<Option<Arc<NFSCodeEnv::Extends>>> {
    let mut outExtends: Option<Arc<NFSCodeEnv::Extends>> = None;
    let mut ext: ExtendsWrapper = ExtendsWrapper::NO_EXTENDS;
    ext = inExtendsTable.clone().borrow()[(inIndex.clone()-1) as usize].clone();
    outExtends = lookupQualifiedExtends2(ext.clone(), inExtendsTable.clone())?;
    Ok(outExtends)
}

fn lookupQualifiedExtends2(mut inExtends: ExtendsWrapper, mut inExtendsTable: ExtendsTableArray) -> Result<Option<Arc<NFSCodeEnv::Extends>>> {
    let mut outExtends: Option<Arc<NFSCodeEnv::Extends>> = None;
    outExtends = (::match_deref::match_deref! { match &((inExtends.clone(), inExtendsTable.clone())) {
        (ExtendsWrapper::QUALIFIED_EXTENDS { ext }, _) => {
            Some(ext.clone())
        },
        (ExtendsWrapper::UNQUALIFIED_EXTENDS { ext: Deref @ NFSCodeEnv::Extends { .. } }, _) => {
            None
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExtends)
}

fn addUnqualifiedToTable(mut inExtends: Extends, mut inIndex: i32, mut inExtendsTable: ExtendsTableArray) -> Result<()> {
    let _ = {let _arr = inExtendsTable.clone(); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = ExtendsWrapper::UNQUALIFIED_EXTENDS { ext: inExtends.clone() }; _arr};
    Ok(())
}

fn updateQualifiedInTable(mut inExtends: Extends, mut inIndex: i32, mut inExtendsTable: ExtendsTableArray) -> Result<()> {
    let _ = {let _arr = inExtendsTable.clone(); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = ExtendsWrapper::QUALIFIED_EXTENDS { ext: inExtends.clone() }; _arr};
    Ok(())
}

fn update2(mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    let mut env: Env = metamodelica::nil();
    let mut rest_env: Env = metamodelica::nil();
    let mut name: Option<ArcStr> = None;
    let mut ty: FrameType = NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE;
    let mut tree: Arc<NFSCodeEnv::EnvTree::Tree> = Arc::new(NFSCodeEnv::EnvTree::Tree::EMPTY);
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut imps: NFSCodeEnv::ImportTable;
    let mut iu: Option<Mutable::Mutable<bool>> = None;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { name: __pa0, frameType: __pa1, clsAndVars: __pa2, extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa3, redeclaredElements: __pa4, classExtendsInfo: _ }, importTable: __pa5, isUsed: __pa6 }, tail: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    tree = __pa2.clone();
    bcl = __pa3.clone();
    re = __pa4.clone();
    imps = __pa5.clone();
    iu = __pa6.clone();
    rest_env = __pa7.clone();
    tree = NFSCodeEnv::EnvTree::map(tree.clone(), Arc::new({ let __pe_b2 = inEnv.clone(); move |__pe_a0, __pe_a1| update3(__pe_a0, __pe_a1, __pe_b2.clone()) }));
    env = cons(Arc::new(NFSCodeEnv::Frame { name: name.clone(), frameType: ty.clone(), clsAndVars: tree.clone(), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: bcl.clone(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: imps.clone(), isUsed: iu.clone() }), rest_env.clone());
    outEnv = NFSCodeFlattenRedeclare::addElementRedeclarationsToEnv(re.clone(), env.clone());
    Ok(outEnv)
}

fn update3(mut name: ArcStr, mut item: Item, mut inEnv: Env) -> Result<Item> {
    let mut item: Item = item;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ NFSCodeEnv::Item::CLASS { cls, env: Deref @ metamodelica::List::Cons { head: cls_env, tail: Deref @ metamodelica::List::Nil }, classType: cls_ty } => {
            let mut env: Env = metamodelica::nil();
            let mut cls = (*cls).clone();
            let mut cls_env = (*cls_env).clone();
            env = NFSCodeEnv::enterFrame(cls_env.clone(), inEnv.clone());
            (cls, env) = updateClassExtends(cls.clone(), env.clone(), cls_ty.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(update2(env.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cls_env = __pa0.clone();
            item = Arc::new(NFSCodeEnv::Item::CLASS { cls: cls.clone(), env: list![cls_env.clone()], classType: cls_ty.clone() });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

fn updateClassExtends(mut inClass: Arc<SCode::Element>, mut inEnv: Env, mut inClassType: ClassType) -> Result<(Arc<SCode::Element>, Env)> {
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: Env = metamodelica::nil();
    (outClass, outEnv) = (::match_deref::match_deref! { match &((inClass.clone(), inEnv.clone(), inClassType.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { classExtendsInfo: Some(ext), .. }, name: Some(name), .. }, tail: _ }, NFSCodeEnv::ClassType::CLASS_EXTENDS) => {
            let mut env: Env = metamodelica::nil();
            let mut mods: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut cls: Arc<SCode::Element>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ext.clone()) {
                Deref @ SCode::Element::EXTENDS { info: __pa0, modifications: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            info = __pa0.clone();
            mods = __pa1.clone();
            (cls, env) = updateClassExtends2(inClass.clone(), (name.clone()).clone(), mods.clone(), info.clone(), inEnv.clone())?;
            (cls.clone(), env.clone())
        },
        _ => {
            (inClass.clone(), inEnv.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClass, outEnv))
}

fn updateClassExtends2(mut inClass: Arc<SCode::Element>, mut inName: ArcStr, mut inMods: Arc<SCode::Mod>, mut inInfo: SourceInfo, mut inEnv: Env) -> Result<(Arc<SCode::Element>, Env)> {
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: Env = metamodelica::nil();
    (outClass, outEnv) = 'mc: {
        let __mc_input = (inClass.clone(), inName.clone(), inMods.clone(), inInfo.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: cls_frame, tail: env }) => {
                    let mut ext: Arc<SCode::Element>;
                    let mut cls: Arc<SCode::Element>;
                    let mut path: Arc<Absyn::Path>;
                    let mut cls_frame = (*cls_frame).clone();
                    (path, _) = lookupClassExtendsBaseClass((inName.clone()).clone(), env.clone(), inInfo.clone())?;
                    ext = Arc::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, modifications: inMods.clone(), ann: None, info: inInfo.clone() });
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::extendEnvWithExtends(ext.clone(), list![cls_frame.clone()])?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls_frame = __pa0.clone();
                    cls = SCodeUtil::addElementToClass(ext.clone(), inClass.clone())?;
                    Ok((cls.clone(), cons(cls_frame.clone(), env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inClass.clone(), inEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outClass, outEnv))
}

fn lookupClassExtendsBaseClass(mut inName: ArcStr, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Arc<Absyn::Path>, Item)> {
    let mut outPath: Arc<Absyn::Path>;
    let mut outItem: Item;
    (outPath, outItem) = 'mc: {
        let __mc_input = (inName.clone(), inEnv.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut item: Item;
                    let mut basename: ArcStr = arcstr::literal!("");
                    basename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*arcstr::literal!(NFSCodeEnv::BASE_CLASS_SUFFIX)); ArcStr::from(__mm_s) }).clone();
                    (item, _) = NFSCodeLookup::lookupInheritedName((basename.clone()).clone(), inEnv.clone())?;
                    path = Arc::new(Absyn::Path::QUALIFIED { name: (literal!("$ce")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (basename.clone()).clone() }) });
                    Ok((path.clone(), item.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut item: Item;
                    (item, _) = NFSCodeLookup::lookupInheritedName((inName.clone()).clone(), inEnv.clone())?;
                    path = Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() });
                    Ok((path.clone(), item.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INVALID_REDECLARATION_OF_CLASS.clone(), list![(inName.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outPath, outItem))
}

pub fn extendEnvWithClassExtends(mut inClassExtends: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = (::match_deref::match_deref! { match &((inClassExtends.clone(), inEnv.clone())) {
        (Deref @ SCode::Element::CLASS { info, cmt, classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: cdef, modifications: mods }, restriction: res, partialPrefix: pp, encapsulatedPrefix: ep, prefixes, name }, _) => {
            let mut env: Env = metamodelica::nil();
            let mut cls_env: Env = metamodelica::nil();
            let mut cls: Arc<SCode::Element>;
            let mut ext: Arc<SCode::Element>;
            cls = Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef.clone(), cmt: cmt.clone(), info: info.clone() });
            cls_env = NFSCodeEnv::makeClassEnvironment(cls.clone(), false)?;
            ext = Arc::new(SCode::Element::EXTENDS { baseClassPath: Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, modifications: mods.clone(), ann: None, info: info.clone() });
            cls_env = addClassExtendsInfoToEnv(ext.clone(), cls_env.clone())?;
            env = NFSCodeEnv::extendEnvWithItem(NFSCodeEnv::newClassItem(cls.clone(), cls_env.clone(), crate::NFSCodeEnv::ClassType::CLASS_EXTENDS), inEnv.clone(), (name.clone()).clone())?;
            env.clone()
        },
        (_, _) => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut el_str: ArcStr = arcstr::literal!("");
            let mut env_str: ArcStr = arcstr::literal!("");
            let mut err_msg: ArcStr = arcstr::literal!("");
            info = SCodeUtil::elementInfo(inClassExtends.clone());
            el_str = (SCodeDump::unparseElementStr(inClassExtends.clone(), SCodeDump::defaultOptions.clone())?).clone();
            env_str = (NFSCodeEnv::getEnvName(inEnv.clone())?).clone();
            err_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSCodeFlattenRedeclare.extendEnvWithClassExtends failed on unknown element ")); __mm_s.push_str(&*el_str.clone()); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*env_str.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(err_msg.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn addClassExtendsInfoToEnv(mut inClassExtends: Arc<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env = metamodelica::nil();
    outEnv = 'mc: {
        let __mc_input = (inClassExtends.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>> = metamodelica::nil();
                    let mut re: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut ext: Arc<NFSCodeEnv::ExtendsTable>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeEnv::getEnvExtendsTable(inEnv.clone())?) {
                        Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0, redeclaredElements: __pa1, classExtendsInfo: None } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    bcl = __pa0.clone();
                    re = __pa1.clone();
                    ext = Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: bcl.clone(), redeclaredElements: re.clone(), classExtendsInfo: Some(inClassExtends.clone()) });
                    Ok(NFSCodeEnv::setEnvExtendsTable(ext.clone(), inEnv.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut estr: ArcStr = arcstr::literal!("");
                    estr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFEnvExtends.addClassExtendsInfoToEnv: Trying to overwrite ")); __mm_s.push_str(&*literal!("existing class extends information, this should not happen!.")); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(estr.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

