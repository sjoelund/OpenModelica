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
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util_datatypes_basic::List;

fn constantBindingOrNone(mut inBinding: Option<Arc<Absyn::Exp>>) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outBinding: Option<Arc<Absyn::Exp>> = None;
    outBinding = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Some(e) => {
            if (AbsynUtil::getCrefFromExp(e.clone(), true, true)?.is_empty()) {inBinding.clone()} else {None}
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinding)
}

pub fn removeNonConstantBindingsKeepRedeclares(mut inMod: Arc<SCode::Mod>, mut onlyRedeclares: bool) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { finalPrefix: fp, eachPrefix: ep, subModLst: sl, binding, comment: cmt, info: i } => {
            let mut sl = (*sl).clone();
            let mut binding = (*binding).clone();
            binding = if (onlyRedeclares.clone()) {None} else {constantBindingOrNone(binding.clone())?};
            sl = removeNonConstantBindingsKeepRedeclaresFromSubMod(sl.clone(), onlyRedeclares.clone())?;
            Arc::new(SCode::Mod::MOD { finalPrefix: fp.clone(), eachPrefix: ep.clone(), subModLst: sl.clone(), binding: binding.clone(), comment: cmt.clone(), info: i.clone() })
        },
        Deref @ SCode::Mod::REDECL { .. } => {
            inMod.clone()
        },
        _ => {
            inMod.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn removeNonConstantBindingsKeepRedeclaresFromSubMod(mut inSl: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut onlyRedeclares: bool) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSl: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    outSl = (::match_deref::match_deref! { match &(inSl.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod: m }, tail: rest } => {
            let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            let mut m = (*m).clone();
            m = removeNonConstantBindingsKeepRedeclares(m.clone(), onlyRedeclares.clone())?;
            sl = removeNonConstantBindingsKeepRedeclaresFromSubMod(rest.clone(), onlyRedeclares.clone())?;
            cons(Arc::new(SCode::SubMod { ident: (n.clone()).clone(), r#mod: m.clone() }), sl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSl)
}

pub fn addRedeclareAsElementsToExtends(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut redeclareElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outExtendsElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outExtendsElements = (::match_deref::match_deref! { match &((inElements.clone(), redeclareElements.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            inElements.clone()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath, visibility, modifications: r#mod, ann, info }, tail: rest }, redecls) => {
            let mut out: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut redeclareMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            let mut r#mod = (*r#mod).clone();
            submods = makeElementsIntoSubMods(openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, redecls.clone())?;
            redeclareMod = Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: submods.clone(), binding: None, comment: None, info: info.clone() });
            r#mod = SCodeUtil::mergeSCodeMods(redeclareMod.clone(), r#mod.clone())?;
            out = addRedeclareAsElementsToExtends(rest.clone(), redecls.clone())?;
            cons(Arc::new(SCode::Element::EXTENDS { baseClassPath: baseClassPath.clone(), visibility: visibility.clone(), modifications: r#mod.clone(), ann: ann.clone(), info: info.clone() }), out.clone())
        },
        (Deref @ metamodelica::List::Cons { head: el, tail: rest }, redecls) => {
            let mut out: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            out = addRedeclareAsElementsToExtends(rest.clone(), redecls.clone())?;
            cons(el.clone(), out.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExtendsElements)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeElementsIntoSubMods(mut inFinal: SCode::Final, mut inEach: SCode::Each, mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    outSubMods = (::match_deref::match_deref! { match &((inFinal.clone(), inEach.clone(), inElements.clone())) {
        (_, _, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (f, e, Deref @ metamodelica::List::Cons { head: el @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: rest }) => {
            let mut newSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- AbsynToSCode.makeElementsIntoSubMods ignoring class-extends redeclare-as-element: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            newSubMods = makeElementsIntoSubMods(f.clone(), e.clone(), rest.clone())?;
            newSubMods.clone()
        },
        (f, e, Deref @ metamodelica::List::Cons { head: el @ Deref @ SCode::Element::COMPONENT { name: n, .. }, tail: rest }) => {
            let mut newSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            newSubMods = makeElementsIntoSubMods(f.clone(), e.clone(), rest.clone())?;
            cons(Arc::new(SCode::SubMod { ident: (n.clone()).clone(), r#mod: Arc::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: el.clone() }) }), newSubMods.clone())
        },
        (f, e, Deref @ metamodelica::List::Cons { head: el @ Deref @ SCode::Element::CLASS { name: n, .. }, tail: rest }) => {
            let mut newSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            newSubMods = makeElementsIntoSubMods(f.clone(), e.clone(), rest.clone())?;
            cons(Arc::new(SCode::SubMod { ident: (n.clone()).clone(), r#mod: Arc::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: el.clone() }) }), newSubMods.clone())
        },
        (f, e, Deref @ metamodelica::List::Cons { head: el, tail: rest }) => {
            let mut newSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- AbsynToSCode.makeElementsIntoSubMods ignoring redeclare-as-element redeclaration: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            newSubMods = makeElementsIntoSubMods(f.clone(), e.clone(), rest.clone())?;
            newSubMods.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubMods)
}

fn removeReferenceInBinding(mut inBinding: Option<Arc<Absyn::Exp>>, mut inCref: Arc<Absyn::ComponentRef>) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outBinding: Option<Arc<Absyn::Exp>> = None;
    outBinding = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Some(e) => {
            let mut crlst1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            let mut crlst2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            crlst1 = AbsynUtil::getCrefFromExp(e.clone(), true, true)?;
            crlst2 = AbsynUtil::removeCrefFromCrefs(crlst1.clone(), inCref.clone())?;
            if (intEq((crlst1.clone().len() as i32), (crlst2.clone().len() as i32))) {inBinding.clone()} else {None}
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinding)
}

pub fn removeSelfReferenceFromMod(mut inMod: Arc<SCode::Mod>, mut inCref: Arc<Absyn::ComponentRef>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { finalPrefix: fp, eachPrefix: ep, subModLst: sl, binding, comment: cmt, info: i } => {
            let mut sl = (*sl).clone();
            let mut binding = (*binding).clone();
            binding = removeReferenceInBinding(binding.clone(), inCref.clone())?;
            sl = removeSelfReferenceFromSubMod(sl.clone(), inCref.clone())?;
            Arc::new(SCode::Mod::MOD { finalPrefix: fp.clone(), eachPrefix: ep.clone(), subModLst: sl.clone(), binding: binding.clone(), comment: cmt.clone(), info: i.clone() })
        },
        Deref @ SCode::Mod::REDECL { .. } => {
            inMod.clone()
        },
        _ => {
            inMod.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn removeSelfReferenceFromSubMod(mut inSl: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inCref: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSl: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    outSl = (::match_deref::match_deref! { match &(inSl.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod: m }, tail: rest } => {
            let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            let mut m = (*m).clone();
            m = removeSelfReferenceFromMod(m.clone(), inCref.clone())?;
            sl = removeSelfReferenceFromSubMod(rest.clone(), inCref.clone())?;
            cons(Arc::new(SCode::SubMod { ident: (n.clone()).clone(), r#mod: m.clone() }), sl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSl)
}

fn expandEnumerationSubMod(mut inSubMod: Arc<SCode::SubMod>, mut inChanged: bool) -> Result<(Arc<SCode::SubMod>, bool)> {
    let mut outSubMod: Arc<SCode::SubMod> = Arc::new(<SCode::SubMod as ::std::default::Default>::default());
    let mut outChanged: bool = false;
    (outSubMod, outChanged) = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::SubMod { r#mod, ident } => {
            let mut mod1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            mod1 = expandEnumerationMod(r#mod.clone())?;
            if (referenceEq(&r#mod.clone(),&mod1.clone())) {(inSubMod.clone(), inChanged.clone())} else {(Arc::new(SCode::SubMod { ident: (ident.clone()).clone(), r#mod: mod1.clone() }), true)}
        },
        _ => {
            (inSubMod.clone(), inChanged.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSubMod, outChanged))
}

pub fn expandEnumerationMod(mut inMod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut f: SCode::Final = SCode::Final::FINAL;
    let mut e: SCode::Each = SCode::Each::EACH;
    let mut el: Arc<SCode::Element>;
    let mut el1: Arc<SCode::Element>;
    let mut submod: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut binding: Option<Arc<Absyn::Exp>> = None;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut changed: bool = false;
    let mut cmt: Option<ArcStr> = None;
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::REDECL { finalPrefix: f, eachPrefix: e, element: el } => {
            el1 = expandEnumerationClass(el.clone())?;
            if (referenceEq(&el.clone(),&el1.clone())) {inMod.clone()} else {Arc::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: el1.clone() })}
        },
        Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: submod, binding, comment: cmt, info } => {
            let mut submod = (*submod).clone();
            (submod, changed) = List::mapFold(submod.clone(), (std::sync::Arc::new(expandEnumerationSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>, bool) -> Result<(Arc<SCode::SubMod>, bool)> + 'static>), false);
            if (changed.clone()) {Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: submod.clone(), binding: binding.clone(), comment: cmt.clone(), info: info.clone() })} else {inMod.clone()}
        },
        _ => inMod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

pub fn expandEnumerationClass(mut inElement: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { info, cmt, classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst: l }, prefixes, restriction: SCode::Restriction::R_TYPE, name: n, .. } => {
            let mut c: Arc<SCode::Element>;
            c = expandEnumeration((n.clone()).clone(), l.clone(), prefixes.clone(), cmt.clone(), info.clone())?;
            c.clone()
        },
        Deref @ SCode::Element::EXTENDS { info, ann, modifications: m, visibility: v, baseClassPath: p } => {
            let mut m1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            m1 = expandEnumerationMod(m.clone())?;
            if (referenceEq(&m.clone(),&m1.clone())) {inElement.clone()} else {Arc::new(SCode::Element::EXTENDS { baseClassPath: p.clone(), visibility: v.clone(), modifications: m1.clone(), ann: ann.clone(), info: info.clone() })}
        },
        _ => {
            inElement.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

pub fn expandEnumeration(mut n: ArcStr, mut l: Arc<metamodelica::List<Arc<SCode::Enum>>>, mut prefixes: Arc<SCode::Prefixes>, mut cmt: Arc<SCode::Comment>, mut info: SourceInfo) -> Result<Arc<SCode::Element>> {
    let mut outClass: Arc<SCode::Element>;
    outClass = Arc::new(SCode::Element::CLASS { name: (n.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION, classDef: makeEnumParts(l.clone(), info.clone())?, cmt: cmt.clone(), info: info.clone() });
    Ok(outClass)
}

fn makeEnumParts(mut inEnumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>, mut info: SourceInfo) -> Result<Arc<SCode::ClassDef>> {
    let mut classDef: Arc<SCode::ClassDef>;
    classDef = Arc::new(SCode::ClassDef::PARTS { elementLst: makeEnumComponents(inEnumLst.clone(), info.clone())?, normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None });
    Ok(classDef)
}

fn makeEnumComponents(mut inEnumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outSCodeElementLst: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outSCodeElementLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (inEnumLst.clone()).into_iter().cloned() {
            let __x = SCodeUtil::makeEnumType(e.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outSCodeElementLst)
}

