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

use crate::NFInstDump;
use crate::NFInstTypes;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeEnv;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;

pub fn checkRecursiveShortDefinition(mut inTypeSpec: Arc<Absyn::TypeSpec>, mut inTypeName: ArcStr, mut inTypeEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, mut inInfo: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inTypeEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
                    let mut ts_path: Arc<Absyn::Path>;
                    let mut ty_path: Arc<Absyn::Path>;
                    ts_path = AbsynUtil::typeSpecPath(inTypeSpec.clone())?;
                    ty_path = NFSCodeEnv::getEnvPath(inTypeEnv.clone())?;
                    let false = (isSelfReference((inTypeName.clone()).clone(), ty_path.clone(), ts_path.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty: ArcStr = arcstr::literal!("");
                    ty = (Dump::unparseTypeSpec(inTypeSpec.clone())?).clone();
                    Error::addSourceMessage(Error::RECURSIVE_SHORT_CLASS_DEFINITION.clone(), list![(inTypeName.clone()).clone(), (ty.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn isSelfReference(mut inTypeName: ArcStr, mut inTypePath: Arc<Absyn::Path>, mut inReferencedName: Arc<Absyn::Path>) -> Result<bool> {
    let mut selfRef: bool = false;
    selfRef = (::match_deref::match_deref! { match &((inTypePath.clone(), inReferencedName.clone())) {
        (p1, Deref @ Absyn::Path::FULLYQUALIFIED { path: p2 }) => {
            AbsynUtil::pathEqual(AbsynUtil::joinPaths(p1.clone(), Arc::new(Absyn::Path::IDENT { name: (inTypeName.clone()).clone() }))?, p2.clone())
        },
        (_, p2) => {
            stringEqual((AbsynUtil::pathLastIdent(inTypePath.clone())?).clone(), (AbsynUtil::pathFirstIdent(p2.clone())?).clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(selfRef)
}

pub fn checkClassExtendsReplaceability(mut inBaseClass: Arc<NFSCodeEnv::Item>, mut inOriginInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inBaseClass.clone()) {
        Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { .. }, .. }, .. }, .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn checkRedeclareModifier(mut inModifier: Arc<NFSCodeEnv::Redeclaration>, mut inBaseClass: Arc<Absyn::Path>, mut inEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inModifier.clone()) {
        Deref @ NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: e @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { .. }, .. } } => {
            checkRedeclareModifier2(e.clone(), inBaseClass.clone(), inEnv.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkRedeclareModifier2(mut inModifier: Arc<SCode::Element>, mut inBaseClass: Arc<Absyn::Path>, mut inEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inModifier.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, .. }, name, .. } => {
                    let mut ty_path: Arc<Absyn::Path>;
                    ty_path = AbsynUtil::typeSpecPath(ty.clone())?;
                    let false = (isSelfReference((name.clone()).clone(), inBaseClass.clone(), ty_path.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { info, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, .. }, name, .. } => {
                    let mut ty_str: ArcStr = arcstr::literal!("");
                    ty_str = (Dump::unparseTypeSpec(ty.clone())?).clone();
                    Error::addSourceMessage(Error::RECURSIVE_SHORT_CLASS_DEFINITION.clone(), list![(name.clone()).clone(), (ty_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn checkModifierIfRedeclare(mut inItem: Arc<NFSCodeEnv::Item>, mut inModifier: Arc<SCode::Mod>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inModifier.clone()) {
        Deref @ SCode::Mod::REDECL { element: el, .. } => {
            checkRedeclaredElementPrefix(inItem.clone(), el.clone(), inInfo.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkRedeclaredElementPrefix(mut inItem: Arc<NFSCodeEnv::Item>, mut inReplacement: Arc<SCode::Element>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inItem.clone(), inReplacement.clone())) {
        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { info, typeSpec: ty1, attributes: SCode::Attributes { variability: var, .. }, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: repl, finalPrefix: fin, .. }, name, .. }, .. }, Deref @ SCode::Element::COMPONENT { typeSpec: ty2, prefixes: Deref @ SCode::Prefixes { .. }, .. }) => {
            let mut ty: ArcStr = arcstr::literal!("");
            let mut ok: bool = false;
            ty = (literal!("component")).clone();
            ok = checkCompRedeclarationReplaceable((name.clone()).clone(), repl.clone(), ty1.clone(), ty2.clone(), inInfo.clone(), info.clone())?;
            ok = checkRedeclarationFinal((name.clone()).clone(), (ty.clone()).clone(), fin.clone(), inInfo.clone(), info.clone())? && ok.clone();
            ok = checkRedeclarationVariability((name.clone()).clone(), (ty.clone()).clone(), var.clone(), inInfo.clone(), info.clone())? && ok.clone();
            let true = (ok.clone()) else { bail!("pattern mismatch") };
            ()
        },
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { info, restriction: res, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: repl, finalPrefix: fin, .. }, name, .. }, .. }, Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { .. }, .. }) => {
            let mut ty: ArcStr = arcstr::literal!("");
            let mut ok: bool = false;
            ty = (SCodeDump::restrictionStringPP(res.clone())?).clone();
            ok = checkClassRedeclarationReplaceable((name.clone()).clone(), repl.clone(), inInfo.clone(), info.clone())?;
            ok = checkRedeclarationFinal((name.clone()).clone(), (ty.clone()).clone(), fin.clone(), inInfo.clone(), info.clone())? && ok.clone();
            let true = (ok.clone()) else { bail!("pattern mismatch") };
            ()
        },
        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { info, name, .. }, .. }, Deref @ SCode::Element::CLASS { restriction: res, .. }) => {
            let mut ty: ArcStr = arcstr::literal!("");
            ty = (SCodeDump::restrictionStringPP(res.clone())?).clone();
            ty = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("a ")); __mm_s.push_str(&*ty.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(literal!("component")).clone(), (name.clone()).clone(), (ty.clone()).clone()], list![inInfo.clone(), info.clone()])?;
            bail!("fail")
        },
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { info, restriction: res, .. }, .. }, Deref @ SCode::Element::COMPONENT { name, .. }) => {
            let mut ty: ArcStr = arcstr::literal!("");
            ty = (SCodeDump::restrictionStringPP(res.clone())?).clone();
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(ty.clone()).clone(), (name.clone()).clone(), (literal!("a component")).clone()], list![inInfo.clone(), info.clone()])?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkClassRedeclarationReplaceable(mut inName: ArcStr, mut inReplaceable: Arc<SCode::Replaceable>, mut inOriginInfo: SourceInfo, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = (::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::Replaceable::NOT_REPLACEABLE if (!(Flags::getConfigBool(Flags::IGNORE_REPLACEABLE.clone())?)) => {
            Error::addMultiSourceMessage(Error::REDECLARE_NON_REPLACEABLE.clone(), list![(inName.clone()).clone()], list![inOriginInfo.clone(), inInfo.clone()])?;
            false
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isValid)
}

fn checkCompRedeclarationReplaceable(mut inName: ArcStr, mut inReplaceable: Arc<SCode::Replaceable>, mut inType1: Arc<Absyn::TypeSpec>, mut inType2: Arc<Absyn::TypeSpec>, mut inOriginInfo: SourceInfo, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = (::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::Replaceable::NOT_REPLACEABLE if (AbsynUtil::pathEqual(AbsynUtil::typeSpecPath(inType1.clone())?, AbsynUtil::typeSpecPath(inType2.clone())?)) => true,
        Deref @ SCode::Replaceable::NOT_REPLACEABLE if (!(Flags::getConfigBool(Flags::IGNORE_REPLACEABLE.clone())?)) => {
            Error::addMultiSourceMessage(Error::REDECLARE_NON_REPLACEABLE.clone(), list![(inName.clone()).clone()], list![inOriginInfo.clone(), inInfo.clone()])?;
            bail!("fail")
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isValid)
}

fn checkRedeclarationFinal(mut inName: ArcStr, mut inType: ArcStr, mut inFinal: SCode::Final, mut inOriginInfo: SourceInfo, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = (match inFinal.clone() {
        SCode::Final::NOT_FINAL => true,
        SCode::Final::FINAL => {
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE.clone(), list![(literal!("final")).clone(), (inType.clone()).clone(), (inName.clone()).clone()], list![inOriginInfo.clone(), inInfo.clone()])?;
            false
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(isValid)
}

fn checkRedeclarationVariability(mut inName: ArcStr, mut inType: ArcStr, mut inVariability: SCode::Variability, mut inOriginInfo: SourceInfo, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = (match inVariability.clone() {
        SCode::Variability::CONST => {
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE.clone(), list![(literal!("constant")).clone(), (inType.clone()).clone(), (inName.clone()).clone()], list![inOriginInfo.clone(), inInfo.clone()])?;
            false
        },
        _ => true,
    });
    Ok(isValid)
}

fn checkRedeclarationVisibility(mut inName: ArcStr, mut inType: ArcStr, mut inOriginalVisibility: SCode::Visibility, mut inNewVisibility: SCode::Visibility, mut inOriginInfo: SourceInfo, mut inNewInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = (match (inOriginalVisibility.clone(), inNewVisibility.clone()) {
        (SCode::Visibility::PUBLIC, SCode::Visibility::PROTECTED) => {
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(literal!("public element")).clone(), (inName.clone()).clone(), (literal!("protected")).clone()], list![inNewInfo.clone(), inOriginInfo.clone()])?;
            false
        },
        (SCode::Visibility::PROTECTED, SCode::Visibility::PUBLIC) => {
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(literal!("protected element")).clone(), (inName.clone()).clone(), (literal!("public")).clone()], list![inNewInfo.clone(), inOriginInfo.clone()])?;
            false
        },
        _ => true,
    });
    Ok(isValid)
}

pub fn checkDuplicateRedeclarations(mut inRedeclare: Arc<NFSCodeEnv::Redeclaration>, mut inRedeclarations: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>) -> Result<()> {
    let mut el_name: ArcStr = arcstr::literal!("");
    let mut el_info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    (el_name, el_info) = NFSCodeEnv::getRedeclarationNameInfo(inRedeclare.clone())?;
    let false = (checkDuplicateRedeclarations2((el_name.clone()).clone(), el_info.clone(), inRedeclarations.clone())?) else { bail!("pattern mismatch") };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn checkDuplicateRedeclarations2(mut inRedeclareName: ArcStr, mut inRedeclareInfo: SourceInfo, mut inRedeclarations: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>) -> Result<bool> {
    let mut outIsDuplicate: bool = false;
    outIsDuplicate = 'mc: {
        let __mc_input = inRedeclarations.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: redecl, tail: _ } => {
                    let mut el_name: ArcStr = arcstr::literal!("");
                    let mut el_info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    (el_name, el_info) = NFSCodeEnv::getRedeclarationNameInfo(redecl.clone())?;
                    let true = (stringEqual((inRedeclareName.clone()).clone(), (el_name.clone()).clone())) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), el_info.clone())?;
                    Error::addSourceMessage(Error::DUPLICATE_REDECLARATION.clone(), list![(inRedeclareName.clone()).clone()], inRedeclareInfo.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_redecls } => {
                    Ok(checkDuplicateRedeclarations2((inRedeclareName.clone()).clone(), inRedeclareInfo.clone(), rest_redecls.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIsDuplicate)
}

pub fn checkRecursiveComponentDeclaration(mut inComponentName: ArcStr, mut inComponentInfo: SourceInfo, mut inTypeEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, mut inTypeItem: Arc<NFSCodeEnv::Item>, mut inComponentEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inTypeEnv.clone(), inComponentEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let false = (NFSCodeEnv::envPrefixOf(inTypeEnv.clone(), inComponentEnv.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(cls_name), .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ } }) => {
                    let mut el: Arc<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(tree.clone(), (cls_name.clone()).clone())?) {
                        Deref @ NFSCodeEnv::Item::CLASS { cls: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    el = __pa0.clone();
                    let true = (SCodeUtil::isFunction(el.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(cls_name), .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ } }) => {
                    let mut el: Arc<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(tree.clone(), (cls_name.clone()).clone())?) {
                        Deref @ NFSCodeEnv::Item::CLASS { cls: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    el = __pa0.clone();
                    let true = (SCodeUtil::isUniontype(el.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty_name: ArcStr = arcstr::literal!("");
                    ty_name = (NFSCodeEnv::getItemName(inTypeItem.clone())?).clone();
                    Error::addSourceMessage(Error::RECURSIVE_DEFINITION.clone(), list![(inComponentName.clone()).clone(), (ty_name.clone()).clone()], inComponentInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn checkIdentNotEqTypeName(mut inIdent: ArcStr, mut inTypeName: Arc<Absyn::TypeSpec>, mut inInfo: SourceInfo) -> Result<bool> {
    let mut outIsNotEq: bool = false;
    outIsNotEq = 'mc: {
        let __mc_input = (inIdent.clone(), inTypeName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id, Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: ty }, .. }) => {
                    let true = (stringEq((id.clone()).clone(), (ty.clone()).clone())) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::LOOKUP_TYPE_FOUND_COMP.clone(), list![(id.clone()).clone()], inInfo.clone())?;
                    Ok(false)
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
    Ok(outIsNotEq)
}

pub fn checkComponentsEqual(mut inComponent1: Arc<NFInstTypes::Component>, mut inComponent2: Arc<NFInstTypes::Component>) -> () {
    let () = (::match_deref::match_deref! { match &(inComponent2.clone()) {
        _ => {
            println!("{}", (literal!("Found duplicate component\n")).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn checkInstanceRestriction(mut inItem: Arc<NFSCodeEnv::Item>, mut inPrefix: Arc<NFInstPrefix::Prefix>, mut inInfo: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inItem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { restriction: res, .. }, .. } => {
                    let true = (SCodeUtil::isInstantiableClassRestriction(res.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { restriction: res, .. }, .. } => {
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    let mut res_str: ArcStr = arcstr::literal!("");
                    res_str = (SCodeDump::restrictionStringPP(res.clone())?).clone();
                    pre_str = (NFInstDump::prefixStr(inPrefix.clone())?).clone();
                    Error::addSourceMessage(Error::INVALID_CLASS_RESTRICTION.clone(), list![(res_str.clone()).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- NFSCodeCheck.checkInstanceRestriction failed on unknown item.")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn checkPartialInstance(mut inItem: Arc<NFSCodeEnv::Item>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::PARTIAL, name, .. }, .. } => {
            Error::addSourceMessage(Error::INST_PARTIAL_CLASS.clone(), list![(name.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

