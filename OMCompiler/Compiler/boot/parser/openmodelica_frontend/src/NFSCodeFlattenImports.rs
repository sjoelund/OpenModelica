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

use crate::NFSCodeEnv;
use crate::NFSCodeLookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub type Env = Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>;

pub type Item = Arc<NFSCodeEnv::Item>;

pub type Extends = Arc<NFSCodeEnv::Extends>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

pub fn flattenProgram(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnv: Env) -> (Arc<metamodelica::List<Arc<SCode::Element>>>, Env) {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outEnv: Env = metamodelica::nil();
    (outProgram, outEnv) = List::mapFold(inProgram.clone(), (std::sync::Arc::new(flattenClass) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)> + 'static>), inEnv.clone());
    (outProgram, outEnv)
}

pub fn flattenClass(mut inClass: Arc<SCode::Element>, mut inEnv: Env) -> Result<(Arc<SCode::Element>, Env)> {
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: Env = metamodelica::nil();
    (outClass, outEnv) = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { info, classDef: cdef, name, .. } => {
                    let mut item: Item;
                    let mut env: Env = metamodelica::nil();
                    let mut cls_env: Arc<NFSCodeEnv::Frame> = Arc::new(<NFSCodeEnv::Frame as ::std::default::Default>::default());
                    let mut cls: Arc<SCode::Element>;
                    let mut cls_ty: NFSCodeEnv::ClassType = NFSCodeEnv::ClassType::BASIC_TYPE;
                    let mut cdef = (*cdef).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupInClass((name.clone()).clone(), inEnv.clone())?) {
                        (Deref @ NFSCodeEnv::Item::CLASS { classType: __pa0, env: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls_ty = __pa0.clone();
                    cls_env = __pa1.clone();
                    env = NFSCodeEnv::enterFrame(cls_env.clone(), inEnv.clone());
                    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(flattenClassDef(cdef.clone(), env.clone(), info.clone())?) {
                        (__pa3, Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 }) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cdef = __pa3.clone();
                    cls_env = __pa4.clone();
                    env = __pa5.clone();
                    cls = SCodeUtil::setClassDef(cdef.clone(), inClass.clone())?;
                    item = NFSCodeEnv::newClassItem(cls.clone(), list![cls_env.clone()], cls_ty.clone());
                    env = NFSCodeEnv::updateItemInEnv(item.clone(), env.clone(), (name.clone()).clone())?;
                    Ok((cls.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeFlattenImports.flattenClass failed on ")); __mm_s.push_str(&*SCodeUtil::elementName(inClass.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outClass, outEnv))
}

fn flattenClassDef(mut inClassDef: Arc<SCode::ClassDef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Arc<SCode::ClassDef>, Env)> {
    let mut outClassDef: Arc<SCode::ClassDef>;
    let mut outEnv: Env = metamodelica::nil();
    (outClassDef, outEnv) = (::match_deref::match_deref! { match &((inClassDef.clone(), inEnv.clone())) {
        (Deref @ SCode::ClassDef::PARTS { elementLst: el, normalEquationLst: neql, initialEquationLst: ieql, normalAlgorithmLst: nal, initialAlgorithmLst: ial, constraintLst: nco, clsattrs: clats, externalDecl: extdecl }, _) => {
            let mut env: Env = metamodelica::nil();
            let mut el = (*el).clone();
            let mut neql = (*neql).clone();
            let mut ieql = (*ieql).clone();
            let mut nal = (*nal).clone();
            let mut ial = (*ial).clone();
            let mut nco = (*nco).clone();
            el = List::filterOnTrue(el.clone(), (std::sync::Arc::new(fnptr!(isNotImport, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>));
            (el, env) = List::mapFold(el.clone(), (std::sync::Arc::new(flattenElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)> + 'static>), inEnv.clone());
            neql = List::map1(neql.clone(), (std::sync::Arc::new(fnptr!(flattenEquation, Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::Equation>> + 'static>), env.clone());
            ieql = List::map1(ieql.clone(), (std::sync::Arc::new(fnptr!(flattenEquation, Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::Equation>> + 'static>), env.clone());
            nal = List::map1(nal.clone(), (std::sync::Arc::new(flattenAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>), env.clone());
            ial = List::map1(ial.clone(), (std::sync::Arc::new(flattenAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>), env.clone());
            nco = List::map2(nco.clone(), (std::sync::Arc::new(flattenConstraints) as std::sync::Arc<dyn ::std::ops::Fn(SCode::ConstraintSection, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<SCode::ConstraintSection> + 'static>), env.clone(), inInfo.clone());
            (Arc::new(SCode::ClassDef::PARTS { elementLst: el.clone(), normalEquationLst: neql.clone(), initialEquationLst: ieql.clone(), normalAlgorithmLst: nal.clone(), initialAlgorithmLst: ial.clone(), constraintLst: nco.clone(), clsattrs: clats.clone(), externalDecl: extdecl.clone() }), env.clone())
        },
        (Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: mods, composition: cdef }, _) => {
            let mut env: Env = metamodelica::nil();
            let mut mods = (*mods).clone();
            let mut cdef = (*cdef).clone();
            (cdef, env) = flattenClassDef(cdef.clone(), inEnv.clone(), inInfo.clone())?;
            mods = flattenModifier(mods.clone(), env.clone(), inInfo.clone())?;
            (Arc::new(SCode::ClassDef::CLASS_EXTENDS { modifications: mods.clone(), composition: cdef.clone() }), env.clone())
        },
        (Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, modifications: mods, attributes: attr }, env) => {
            let mut ty = (*ty).clone();
            let mut mods = (*mods).clone();
            let mut env = (*env).clone();
            mods = flattenModifier(mods.clone(), env.clone(), inInfo.clone())?;
            env = NFSCodeEnv::removeExtendsFromLocalScope(env.clone())?;
            ty = flattenTypeSpec(ty.clone(), env.clone(), inInfo.clone())?;
            (Arc::new(SCode::ClassDef::DERIVED { typeSpec: ty.clone(), modifications: mods.clone(), attributes: attr.clone() }), inEnv.clone())
        },
        _ => {
            (inClassDef.clone(), inEnv.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDef, outEnv))
}

fn flattenDerivedClassDef(mut inClassDef: Arc<SCode::ClassDef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<SCode::ClassDef>> {
    let mut outClassDef: Arc<SCode::ClassDef>;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut mods: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ SCode::ClassDef::DERIVED { typeSpec: __pa0, modifications: __pa1, attributes: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    mods = __pa1.clone();
    attr = __pa2.clone();
    ty = flattenTypeSpec(ty.clone(), inEnv.clone(), inInfo.clone())?;
    mods = flattenModifier(mods.clone(), inEnv.clone(), inInfo.clone())?;
    outClassDef = Arc::new(SCode::ClassDef::DERIVED { typeSpec: ty.clone(), modifications: mods.clone(), attributes: attr.clone() });
    Ok(outClassDef)
}

fn isNotImport(mut inElement: Arc<SCode::Element>) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::IMPORT { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn flattenElement(mut inElement: Arc<SCode::Element>, mut inEnv: Env) -> Result<(Arc<SCode::Element>, Env)> {
    let mut outElement: Arc<SCode::Element>;
    let mut outEnv: Env = metamodelica::nil();
    (outElement, outEnv) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { name, .. } => {
            let mut env: Env = metamodelica::nil();
            let mut elem: Arc<SCode::Element>;
            let mut item: Item;
            elem = flattenComponent(inElement.clone(), inEnv.clone())?;
            item = NFSCodeEnv::newVarItem(elem.clone(), true);
            env = NFSCodeEnv::updateItemInEnv(item.clone(), inEnv.clone(), (name.clone()).clone())?;
            (elem.clone(), env.clone())
        },
        Deref @ SCode::Element::CLASS { .. } => {
            let mut env: Env = metamodelica::nil();
            let mut elem: Arc<SCode::Element>;
            (elem, env) = flattenClass(inElement.clone(), inEnv.clone())?;
            (elem.clone(), env.clone())
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            (flattenExtends(inElement.clone(), inEnv.clone())?, inEnv.clone())
        },
        _ => {
            (inElement.clone(), inEnv.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outElement, outEnv))
}

fn flattenComponent(mut inComponent: Arc<SCode::Element>, mut inEnv: Env) -> Result<Arc<SCode::Element>> {
    let mut outComponent: Arc<SCode::Element>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut prefixes: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    let mut type_spec: Arc<Absyn::TypeSpec>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let mut cond: Option<Arc<Absyn::Exp>> = None;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(inComponent.clone()) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, prefixes: __pa1, attributes: __pa2, typeSpec: __pa3, modifications: __pa4, comment: __pa5, condition: __pa6, info: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    prefixes = __pa1.clone();
    attr = __pa2.clone();
    type_spec = __pa3.clone();
    r#mod = __pa4.clone();
    cmt = __pa5.clone();
    cond = __pa6.clone();
    info = __pa7.clone();
    attr = flattenAttributes(attr.clone(), inEnv.clone(), info.clone())?;
    type_spec = flattenTypeSpec(type_spec.clone(), inEnv.clone(), info.clone())?;
    r#mod = flattenModifier(r#mod.clone(), inEnv.clone(), info.clone())?;
    cond = flattenOptExp(cond.clone(), inEnv.clone(), info.clone())?;
    outComponent = Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: type_spec.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() });
    Ok(outComponent)
}

fn flattenAttributes(mut inAttributes: SCode::Attributes, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut ct: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    let mut prl: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    let mut var: SCode::Variability = SCode::Variability::CONST;
    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut isf: Absyn::IsField = Absyn::IsField::FIELD;
    let SCode::ATTR { arrayDims: __pa0, connectorType: __pa1, parallelism: __pa2, variability: __pa3, direction: __pa4, isField: __pa5 } = (inAttributes.clone()) else { bail!("pattern mismatch") };
    ad = __pa0.clone();
    ct = __pa1.clone();
    prl = __pa2.clone();
    var = __pa3.clone();
    dir = __pa4.clone();
    isf = __pa5.clone();
    ad = List::map2(ad.clone(), (std::sync::Arc::new(flattenSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::Subscript>> + 'static>), inEnv.clone(), inInfo.clone());
    outAttributes = SCode::Attributes { arrayDims: ad.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var.clone(), direction: dir.clone(), isField: isf.clone() };
    Ok(outAttributes)
}

fn flattenTypeSpec(mut inTypeSpec: Arc<Absyn::TypeSpec>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::TypeSpec>> {
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    outTypeSpec = (::match_deref::match_deref! { match &(inTypeSpec.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { arrayDim: ad, path } => {
            let mut path = (*path).clone();
            (_, path, _) = NFSCodeLookup::lookupClassName(path.clone(), inEnv.clone(), inInfo.clone())?;
            Arc::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: ad.clone() })
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. } => {
            inTypeSpec.clone()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: ad, typeSpecs: tys, path } => {
            let mut tys = (*tys).clone();
            tys = List::map2(tys.clone(), (std::sync::Arc::new(flattenTypeSpec) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::TypeSpec>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::TypeSpec>> + 'static>), inEnv.clone(), inInfo.clone());
            Arc::new(Absyn::TypeSpec::TCOMPLEX { path: path.clone(), typeSpecs: tys.clone(), arrayDim: ad.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTypeSpec)
}

fn flattenExtends(mut inExtends: Arc<SCode::Element>, mut inEnv: Env) -> Result<Arc<SCode::Element>> {
    let mut outExtends: Arc<SCode::Element>;
    let mut path: Arc<Absyn::Path>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut ann: Option<Arc<SCode::Annotation>> = None;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut env: Env = metamodelica::nil();
    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inExtends.clone()) {
        Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, visibility: __pa1, modifications: __pa2, ann: __pa3, info: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    vis = __pa1.clone();
    r#mod = __pa2.clone();
    ann = __pa3.clone();
    info = __pa4.clone();
    env = NFSCodeEnv::removeExtendsFromLocalScope(inEnv.clone())?;
    (_, path, _) = NFSCodeLookup::lookupBaseClassName(path.clone(), env.clone(), info.clone())?;
    r#mod = flattenModifier(r#mod.clone(), inEnv.clone(), info.clone())?;
    outExtends = Arc::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: vis.clone(), modifications: r#mod.clone(), ann: ann.clone(), info: info.clone() });
    Ok(outExtends)
}

fn flattenEquation(mut inEquation: Arc<SCode::Equation>, mut inEnv: Env) -> Arc<SCode::Equation> {
    let mut outEquation: Arc<SCode::Equation>;
    (outEquation, _) = SCodeUtil::mapFoldEquations(inEquation.clone(), (std::sync::Arc::new(flattenEquationTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<(Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)> + 'static>), inEnv.clone());
    outEquation
}

fn flattenEquationTraverser(mut eq: Arc<SCode::Equation>, mut env: Env) -> Result<(Arc<SCode::Equation>, Env)> {
    let mut eq: Arc<SCode::Equation> = eq;
    let mut env: Env = env;
    (eq, env) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_FOR { info, index: iter_name, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone());
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq.clone(), (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (eq.clone(), env.clone())
        },
        Deref @ SCode::Equation::EQ_REINIT { info, comment: cmt, expReinit: exp, cref: crefExp @ Deref @ Absyn::Exp::CREF { componentRef: cref } } => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone())?;
            eq = Arc::new(SCode::Equation::EQ_REINIT { cref: crefExp.clone(), expReinit: exp.clone(), comment: cmt.clone(), info: info.clone() });
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq.clone(), (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (eq.clone(), env.clone())
        },
        _ => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            info = SCodeUtil::getEquationInfo(eq.clone())?;
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq.clone(), (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (eq.clone(), env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, env))
}

fn traverseExp(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo);
    (outExp, outTuple) = AbsynUtil::traverseExpBidir(inExp.clone(), (std::sync::Arc::new(flattenExpTraverserEnter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (std::sync::Arc::new(fnptr!(flattenExpTraverserExit, Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), inTuple.clone())?;
    Ok((outExp, outTuple))
}

fn flattenConstraints(mut inConstraints: SCode::ConstraintSection, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<SCode::ConstraintSection> {
    let mut outConstraints: SCode::ConstraintSection = <SCode::ConstraintSection as ::std::default::Default>::default();
    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let SCode::CONSTRAINTS { constraints: __pa0 } = (inConstraints.clone()) else { bail!("pattern mismatch") };
    exps = __pa0.clone();
    exps = List::map2(exps.clone(), (std::sync::Arc::new(flattenExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::Exp>> + 'static>), inEnv.clone(), inInfo.clone());
    outConstraints = SCode::ConstraintSection { constraints: exps.clone() };
    Ok(outConstraints)
}

fn flattenAlgorithm(mut inAlgorithm: Arc<SCode::AlgorithmSection>, mut inEnv: Env) -> Result<Arc<SCode::AlgorithmSection>> {
    let mut outAlgorithm: Arc<SCode::AlgorithmSection> = Arc::new(<SCode::AlgorithmSection as ::std::default::Default>::default());
    let mut statements: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inAlgorithm.clone()) {
        Deref @ SCode::AlgorithmSection { statements: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    statements = __pa0.clone();
    statements = List::map1(statements.clone(), (std::sync::Arc::new(fnptr!(flattenStatement, Arc<SCode::Statement>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::Statement>> + 'static>), inEnv.clone());
    outAlgorithm = Arc::new(SCode::AlgorithmSection { statements: statements.clone() });
    Ok(outAlgorithm)
}

fn flattenStatement(mut inStatement: Arc<SCode::Statement>, mut inEnv: Env) -> Arc<SCode::Statement> {
    let mut outStatement: Arc<SCode::Statement>;
    (outStatement, _) = SCodeUtil::mapFoldStatements(inStatement.clone(), (std::sync::Arc::new(flattenStatementTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<(Arc<SCode::Statement>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)> + 'static>), inEnv.clone());
    outStatement
}

fn flattenStatementTraverser(mut stmt: Arc<SCode::Statement>, mut env: Env) -> Result<(Arc<SCode::Statement>, Env)> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    let mut env: Env = env;
    (stmt, env) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_FOR { info, index: iter_name, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone());
            (stmt, _) = SCodeUtil::mapFoldStatementExps(stmt.clone(), (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()));
            (stmt.clone(), env.clone())
        },
        Deref @ SCode::Statement::ALG_PARFOR { info, index: iter_name, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone());
            (stmt, _) = SCodeUtil::mapFoldStatementExps(stmt.clone(), (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()));
            (stmt.clone(), env.clone())
        },
        _ => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            info = SCodeUtil::getStatementInfo(stmt.clone())?;
            (stmt, _) = SCodeUtil::mapFoldStatementExps(stmt.clone(), (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()));
            (stmt.clone(), env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((stmt, env))
}

fn flattenModifier(mut inMod: Arc<SCode::Mod>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { finalPrefix: fp, eachPrefix: ep, subModLst: sub_mods, binding: opt_exp, comment: cmt, info } => {
            let mut sub_mods = (*sub_mods).clone();
            let mut opt_exp = (*opt_exp).clone();
            opt_exp = flattenModOptExp(opt_exp.clone(), inEnv.clone(), inInfo.clone())?;
            sub_mods = List::map2(sub_mods.clone(), (std::sync::Arc::new(flattenSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<SCode::SubMod>> + 'static>), inEnv.clone(), inInfo.clone());
            Arc::new(SCode::Mod::MOD { finalPrefix: fp.clone(), eachPrefix: ep.clone(), subModLst: sub_mods.clone(), binding: opt_exp.clone(), comment: cmt.clone(), info: info.clone() })
        },
        Deref @ SCode::Mod::REDECL { finalPrefix: fp, eachPrefix: ep, element: el } => {
            let mut el = (*el).clone();
            el = flattenRedeclare(el.clone(), inEnv.clone())?;
            Arc::new(SCode::Mod::REDECL { finalPrefix: fp.clone(), eachPrefix: ep.clone(), element: el.clone() })
        },
        Deref @ SCode::Mod::NOMOD => {
            inMod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn flattenModOptExp(mut inOptExp: Option<Arc<Absyn::Exp>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outOptExp: Option<Arc<Absyn::Exp>> = None;
    outOptExp = (::match_deref::match_deref! { match &(inOptExp.clone()) {
        Some(exp) => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv.clone(), inInfo.clone())?;
            Some(exp.clone())
        },
        _ => {
            inOptExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOptExp)
}

fn flattenSubMod(mut inSubMod: Arc<SCode::SubMod>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<SCode::SubMod>> {
    let mut outSubMod: Arc<SCode::SubMod> = Arc::new(<SCode::SubMod as ::std::default::Default>::default());
    outSubMod = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::SubMod { r#mod, ident } => {
            let mut r#mod = (*r#mod).clone();
            r#mod = flattenModifier(r#mod.clone(), inEnv.clone(), inInfo.clone())?;
            Arc::new(SCode::SubMod { ident: (ident.clone()).clone(), r#mod: r#mod.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubMod)
}

fn flattenRedeclare(mut inElement: Arc<SCode::Element>, mut inEnv: Env) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: ep, partialPrefix: pp, restriction: res, classDef: cdef @ Deref @ SCode::ClassDef::DERIVED { .. }, cmt, info } => {
            let mut cdef2: Arc<SCode::ClassDef>;
            cdef2 = flattenDerivedClassDef(cdef.clone(), inEnv.clone(), info.clone())?;
            Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef2.clone(), cmt: cmt.clone(), info: info.clone() })
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => {
            inElement.clone()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            let mut element: Arc<SCode::Element>;
            element = flattenComponent(inElement.clone(), inEnv.clone())?;
            element.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Unknown redeclare in NFSCodeFlattenImports.flattenRedeclare")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

fn flattenSubscript(mut inSub: Arc<Absyn::Subscript>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::Subscript>> {
    let mut outSub: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    outSub = (::match_deref::match_deref! { match &(inSub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { subscript: exp } => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv.clone(), inInfo.clone())?;
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: exp.clone() })
        },
        Deref @ Absyn::Subscript::NOSUB => {
            inSub.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSub)
}

fn flattenExp(mut inExp: Arc<Absyn::Exp>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (outExp, _) = AbsynUtil::traverseExpBidir(inExp.clone(), (std::sync::Arc::new(flattenExpTraverserEnter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (std::sync::Arc::new(fnptr!(flattenExpTraverserExit, Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (inEnv.clone(), inInfo.clone()))?;
    Ok(outExp)
}

fn flattenOptExp(mut inExp: Option<Arc<Absyn::Exp>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outExp: Option<Arc<Absyn::Exp>> = None;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Some(exp) => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv.clone(), inInfo.clone())?;
            Some(exp.clone())
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn flattenExpTraverserEnter(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ Absyn::Exp::CREF { componentRef: cref }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone())?;
            (Arc::new(Absyn::Exp::CREF { componentRef: cref.clone() }), tup.clone())
        },
        (Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterators: iters, iterType, exp }, function_: cref, .. }, (env, info)) => {
            let mut exp = (*exp).clone();
            let mut cref = (*cref).clone();
            let mut env = (*env).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone())?;
            env = NFSCodeEnv::extendEnvWithIterators(iters.clone(), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone());
            exp = flattenExp(exp.clone(), env.clone(), info.clone())?;
            (Arc::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: exp.clone(), iterType: iterType.clone(), iterators: iters.clone() }), typeVars: var_field!((*inExp).typeVars, Absyn::Exp::CALL).clone() }), (env.clone(), info.clone()))
        },
        (Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "SOME", .. }, .. }, _) => {
            (inExp.clone(), inTuple.clone())
        },
        (Deref @ Absyn::Exp::CALL { functionArgs: args, function_: cref, .. }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone())?;
            (Arc::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: args.clone(), typeVars: var_field!((*inExp).typeVars, Absyn::Exp::CALL).clone() }), tup.clone())
        },
        (Deref @ Absyn::Exp::PARTEVALFUNCTION { functionArgs: args, function_: cref }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone())?;
            (Arc::new(Absyn::Exp::PARTEVALFUNCTION { function_: cref.clone(), functionArgs: args.clone() }), tup.clone())
        },
        (exp @ Deref @ Absyn::Exp::MATCHEXP { .. }, (env, info)) => {
            let mut env = (*env).clone();
            env = NFSCodeEnv::extendEnvWithMatch(exp.clone(), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone())?;
            (exp.clone(), (env.clone(), info.clone()))
        },
        _ => {
            (inExp.clone(), inTuple.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

fn flattenExpTraverserExit(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> (Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) {
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

pub fn flattenComponentRefSubs(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs } => {
            let mut subs = (*subs).clone();
            subs = List::map2(subs.clone(), (std::sync::Arc::new(flattenSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::Subscript>> + 'static>), inEnv.clone(), inInfo.clone());
            Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subs.clone() })
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts: subs, componentRef: cref } => {
            let mut subs = (*subs).clone();
            let mut cref = (*cref).clone();
            subs = List::map2(subs.clone(), (std::sync::Arc::new(flattenSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::Subscript>> + 'static>), inEnv.clone(), inInfo.clone());
            cref = flattenComponentRefSubs(cref.clone(), inEnv.clone(), inInfo.clone())?;
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (name.clone()).clone(), subscripts: subs.clone(), componentRef: cref.clone() })
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut cref = (*cref).clone();
            cref = flattenComponentRefSubs(cref.clone(), inEnv.clone(), inInfo.clone())?;
            AbsynUtil::crefMakeFullyQualified(cref.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

