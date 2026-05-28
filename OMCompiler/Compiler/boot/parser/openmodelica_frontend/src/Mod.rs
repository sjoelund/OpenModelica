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

use crate::Ceval;
use crate::ComponentReference;
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use crate::InnerOuter;
use crate::Inst;
use crate::InstUtil;
use crate::Lookup;
use crate::PrefixUtil;
use crate::Static;
use crate::Types;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::SCodeInstUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

/// Used to know where a modifier came from, for error reporting.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModScope {
    COMPONENT {
        name: ArcStr,
    },
    EXTENDS {
        path: Arc<Absyn::Path>,
    },
    DERIVED {
        path: Arc<Absyn::Path>,
    },
}
pub use self::ModScope::{COMPONENT,EXTENDS,DERIVED};

/// used for error reporting
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FullMod {
    /// the fully qualified cref and the mod, only used for redeclare
    MOD {
        cref: Arc<DAE::ComponentRef>,
        r#mod: Arc<DAE::Mod>,
    },
    /// the fully qualified cref and the sub mod for all other mods
    SUB_MOD {
        cref: Arc<DAE::ComponentRef>,
        subMod: Arc<DAE::SubMod>,
    },
}
pub use self::FullMod::{MOD,SUB_MOD};

pub type SubMod = Arc<DAE::SubMod>;

pub type EqMod = DAE::EqMod;

pub fn elabMod(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inMod: Arc<SCode::Mod>, mut inBoolean: bool, mut inModScope: ModScope, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Mod>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    r#mod = SCodeInstUtil::expandEnumerationMod(inMod.clone())?;
    (outCache, outMod) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#mod.clone(), inBoolean.clone(), inModScope.clone(), inInfo.clone())) {
        (cache, _, _, _, Deref @ SCode::Mod::NOMOD, _, _, _) => {
            (cache.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
        },
        (cache, env, ih, pre, Deref @ SCode::Mod::MOD { info, binding: None, subModLst: subs, eachPrefix: each_, finalPrefix, .. }, r#impl, _, _) => {
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, subs_1) = elabSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), inModScope.clone(), info.clone())?;
            (cache.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: None, info: info.clone() }))
        },
        (cache, env, ih, pre, Deref @ SCode::Mod::MOD { info, binding: Some(e), subModLst: subs, eachPrefix: each_, finalPrefix, .. }, r#impl, _, _) => {
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            let mut e_1: Arc<DAE::Exp>;
            let mut e_2: Arc<DAE::Exp>;
            let mut prop: DAE::Properties;
            let mut e_val: Option<Arc<Values::Value>> = None;
            let mut cache = (*cache).clone();
            (cache, subs_1) = elabSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), inModScope.clone(), info.clone())?;
            (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), Config::splitArrays()?, pre.clone(), info.clone())?;
            (e_1, prop) = Expression::tupleHead(e_1.clone(), prop.clone())?;
            (cache, e_1, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
            (e_val, cache) = elabModValue(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
            (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), env.clone(), ih.clone(), e_1.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_2.clone(), modifierAsValue: e_val.clone(), properties: prop.clone(), modifierAsAbsynExp: e.clone(), info: info.clone() }), info: info.clone() }))
        },
        (cache, env, ih, pre, Deref @ SCode::Mod::MOD { info, binding: Some(e), subModLst: subs, eachPrefix: each_, finalPrefix, .. }, r#impl, _, _) => {
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, subs_1) = elabSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), inModScope.clone(), info.clone())?;
            (cache.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::UNTYPED { exp: e.clone() }), info: info.clone() }))
        },
        (cache, env, ih, pre, Deref @ SCode::Mod::REDECL { element: elem, eachPrefix: each_, finalPrefix }, r#impl, _, info) => {
            let mut dm: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut elem = (*elem).clone();
            (elem, dm) = elabModRedeclareElement(cache.clone(), env.clone(), ih.clone(), pre.clone(), finalPrefix.clone(), elem.clone(), r#impl.clone(), inModScope.clone(), info.clone())?;
            (cache.clone(), Arc::new(DAE::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), element: elem.clone(), r#mod: dm.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outMod))
}

pub fn isInvariantMod(mut r#mod: Arc<SCode::Mod>) -> Result<bool> {
    let mut b: bool = false;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut mods: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    b = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::NOMOD => true,
        Deref @ SCode::Mod::MOD { binding: None, .. } => {
            b = (::match_deref::match_deref! { match &(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
        Some(e) => {
            (_, b) = AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isInvariantExpNoTraverse, Arc<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, bool) -> Result<(Arc<Absyn::Exp>, bool)> + 'static>), true)?;
            b.clone()
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(b.clone()) {
                return Ok(b);
            }
            for mut sm in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut sm = sm.clone();
                if !(isInvariantMod(sm.r#mod.clone())?) {
                    b = false;
                    return Ok(b);
                }
            }
            true
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: None, path: Deref @ Absyn::Path::FULLYQUALIFIED { .. } }, modifications: mods, .. }, .. } => isInvariantMod(mods.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isInvariantDAEMod(mut r#mod: Arc<DAE::Mod>) -> Result<bool> {
    let mut b: bool = false;
    let mut e: Arc<DAE::Exp>;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut mods: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    b = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ DAE::Mod::NOMOD => true,
        Deref @ DAE::Mod::MOD { binding: None, .. } => {
            b = (match var_field!((*r#mod).binding, DAE::Mod::MOD).clone() {
        Some(DAE::EqMod::TYPED { modifierAsExp: ref e, .. }) => {
            (_, b) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(fnptr!(Expression::isInvariantExpNoTraverse, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), true)?;
            b.clone()
        },
        Some(DAE::EqMod::UNTYPED { exp: mut exp }) => {
            (_, b) = AbsynUtil::traverseExp(exp.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isInvariantExpNoTraverse, Arc<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, bool) -> Result<(Arc<Absyn::Exp>, bool)> + 'static>), true)?;
            b.clone()
        },
        _ => true,
    });
            if !(b.clone()) {
                return Ok(b);
            }
            for mut sm in &*var_field!((*r#mod).subModLst, DAE::Mod::MOD).clone() {
                let mut sm = sm.clone();
                if !(isInvariantDAEMod(sm.r#mod.clone())?) {
                    b = false;
                    return Ok(b);
                }
            }
            true
        },
        Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: None, path: Deref @ Absyn::Path::FULLYQUALIFIED { .. } }, modifications: mods, .. }, .. } => isInvariantMod(mods.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn elabModForBasicType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inMod: Arc<SCode::Mod>, mut inBoolean: bool, mut inModScope: ModScope, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Mod>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    checkIfModsAreBasicTypeMods(inMod.clone())?;
    (outCache, outMod) = elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), inBoolean.clone(), inModScope.clone(), info.clone())?;
    Ok((outCache, outMod))
}

fn checkIfModsAreBasicTypeMods(mut r#mod: Arc<SCode::Mod>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::NOMOD => {
            ()
        },
        Deref @ SCode::Mod::MOD { subModLst: subs, .. } => {
            checkIfSubmodsAreBasicTypeMods(subs.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn checkIfSubmodsAreBasicTypeMods(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inSubs.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident, .. }, tail: subs } => {
            let true = (ClassInfUtil::isBasicTypeComponentName((ident.clone()).clone())) else { bail!("pattern mismatch") };
            checkIfSubmodsAreBasicTypeMods(subs.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn elabModRedeclareElement(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut finalPrefix: SCode::Final, mut inElt: Arc<SCode::Element>, mut r#impl: bool, mut inModScope: ModScope, mut info: SourceInfo) -> Result<(Arc<SCode::Element>, Arc<DAE::Mod>)> {
    let mut outElement: Arc<SCode::Element>;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    (outElement, outMod) = 'mc: {
        let __mc_input = inElt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: cn, prefixes: prefixes @ Deref @ SCode::Prefixes { visibility: vis, redeclarePrefix: redecl, finalPrefix: fi, innerOuter: io, replaceablePrefix: repl }, encapsulatedPrefix: enc, partialPrefix: p, restriction: restr, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: tp, modifications: r#mod, attributes: attr1 }, cmt, info: i } => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut tp1: Arc<Absyn::TypeSpec>;
                    let mut emod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut r#mod = (*r#mod).clone();
                    r#mod = SCodeUtil::mergeModifiers(r#mod.clone(), SCodeUtil::getConstrainedByModifiers(prefixes.clone()))?;
                    (cache, emod) = elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#mod.clone(), r#impl.clone(), inModScope.clone(), info.clone())?;
                    (_, tp1) = elabModQualifyTypespec(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#impl.clone(), info.clone(), (cn.clone()).clone(), tp.clone())?;
                    r#mod = unelabMod(emod.clone())?;
                    Ok((Arc::new(SCode::Element::CLASS { name: (cn.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: fi.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), encapsulatedPrefix: enc.clone(), partialPrefix: p.clone(), restriction: restr.clone(), classDef: Arc::new(SCode::ClassDef::DERIVED { typeSpec: tp1.clone(), modifications: r#mod.clone(), attributes: attr1.clone() }), cmt: cmt.clone(), info: i.clone() }), emod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION, .. } => {
                    Ok((inElt.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => {
                    Ok((inElt.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { name: compname, prefixes: prefixes @ Deref @ SCode::Prefixes { visibility: vis, redeclarePrefix: redecl, finalPrefix: fi, innerOuter: io, replaceablePrefix: repl }, attributes: attr, typeSpec: tp, modifications: r#mod, comment: cmt, condition: cond, info: i } => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut tp1: Arc<Absyn::TypeSpec>;
                    let mut emod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut r#mod = (*r#mod).clone();
                    r#mod = SCodeUtil::mergeModifiers(r#mod.clone(), SCodeUtil::getConstrainedByModifiers(prefixes.clone()))?;
                    (cache, emod) = elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#mod.clone(), r#impl.clone(), inModScope.clone(), info.clone())?;
                    (_, tp1) = elabModQualifyTypespec(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#impl.clone(), info.clone(), (compname.clone()).clone(), tp.clone())?;
                    r#mod = unelabMod(emod.clone())?;
                    Ok((Arc::new(SCode::Element::COMPONENT { name: (compname.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: fi.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: attr.clone(), typeSpec: tp1.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: i.clone() }), emod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                element => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unhandled element redeclare (we keep it as it is!): ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(element.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok((element.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElement, outMod))
}

fn elabModQualifyTypespec(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut r#impl: bool, mut info: SourceInfo, mut name: ArcStr, mut tp: Arc<Absyn::TypeSpec>) -> Result<(FCore::Cache, Arc<Absyn::TypeSpec>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outTp: Arc<Absyn::TypeSpec>;
    (outCache, outTp) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), r#impl.clone(), info.clone(), name.clone(), tp.clone())) {
        (cache, env, _, _, _, _, _, Deref @ Absyn::TypeSpec::TPATH { path: p, arrayDim: None }) => {
            let mut p1: Arc<Absyn::Path>;
            let mut cache = (*cache).clone();
            (cache, p1) = Inst::makeFullyQualified(cache.clone(), env.clone(), p.clone())?;
            (cache.clone(), Arc::new(Absyn::TypeSpec::TPATH { path: p1.clone(), arrayDim: None }))
        },
        (cache, env, ih, pre, _, _, _, Deref @ Absyn::TypeSpec::TPATH { path: p, arrayDim: Some(dims) }) => {
            let mut p1: Arc<Absyn::Path>;
            let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut edims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut dims = (*dims).clone();
            cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() });
            (cache, edims) = InstUtil::elabArraydim(cache.clone(), env.clone(), cref.clone(), p.clone(), dims.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), metamodelica::nil())?;
            (cache, edims) = PrefixUtil::prefixDimensions(cache.clone(), env.clone(), ih.clone(), pre.clone(), edims.clone())?;
            dims = List::map(edims.clone(), (std::sync::Arc::new(Expression::unelabDimension) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<Arc<Absyn::Subscript>> + 'static>));
            (cache, p1) = Inst::makeFullyQualified(cache.clone(), env.clone(), p.clone())?;
            (cache.clone(), Arc::new(Absyn::TypeSpec::TPATH { path: p1.clone(), arrayDim: Some(dims.clone()) }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outTp))
}

fn elabModValue(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inProp: DAE::Properties, mut inImpl: bool, mut inInfo: SourceInfo) -> Result<(Option<Arc<Values::Value>>, FCore::Cache)> {
    let mut outValue: Option<Arc<Values::Value>> = None;
    let mut outCache: FCore::Cache = inCache.clone();
    let mut err_count: i32 = 0;
    let mut msg: Absyn::Msg = Absyn::Msg::NO_MSG;
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    c = Types::propAllConst(inProp.clone())?;
    if !(Types::constIsVariable(c.clone())) {
        msg = AbsynUtil::optMsg(Types::constIsConst(c.clone()) && !(inImpl.clone()), inInfo.clone());
        err_count = Error::getNumErrorMessages();
        if '__try0: {
            (_, v) = unwrap_break_err!(Ceval::ceval(inCache.clone(), inEnv.clone(), inExp.clone(), false, msg.clone(), 0), '__try0);
            if ValuesUtil::isRecord(v.clone()) {
                v = unwrap_break_err!(ValuesUtil::typeConvertRecord(v.clone(), Expression::r#typeof(inExp.clone())?), '__try0);
            }
            outValue = Some(v.clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            if err_count.clone() != Error::getNumErrorMessages() && !(Expression::containsAnyCall(inExp.clone())?) {
                bail!("fail");
            }
        }
    }
    Ok((outValue, outCache))
}

pub fn unelabMod(mut inMod: Arc<DAE::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD => {
                    Ok(Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { info, binding: None, subModLst: subs, eachPrefix: each_, finalPrefix } => {
                    let mut subs_1: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    subs_1 = unelabSubmods(subs.clone())?;
                    Ok(Arc::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: None, comment: None, info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { info, binding: Some(DAE::EqMod::UNTYPED { exp: e }), subModLst: subs, eachPrefix: each_, finalPrefix } => {
                    let mut subs_1: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    subs_1 = unelabSubmods(subs.clone())?;
                    Ok(Arc::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(e.clone()), comment: None, info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { info, binding: Some(DAE::EqMod::TYPED { modifierAsValue: Some(v), .. }), subModLst: subs, eachPrefix: each_, finalPrefix } => {
                    let mut subs_1: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    let mut e_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    subs_1 = unelabSubmods(subs.clone())?;
                    e_1 = Expression::unelabExp(ValuesUtil::valueExp(v.clone(), None)?)?;
                    Ok(Arc::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(e_1.clone()), comment: None, info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { info, binding: Some(DAE::EqMod::TYPED { modifierAsExp: _, modifierAsValue: _, properties: _, modifierAsAbsynExp: absynExp, .. }), subModLst: subs, eachPrefix: each_, finalPrefix } => {
                    let mut subs_1: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    let mut e_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    subs_1 = unelabSubmods(subs.clone())?;
                    e_1 = absynExp.clone();
                    Ok(Arc::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(e_1.clone()), comment: None, info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { element: elem, eachPrefix: each_, finalPrefix, .. } => {
                    Ok(Arc::new(SCode::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), element: elem.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                r#mod => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Mod.elabUntypedMod failed: ")); __mm_s.push_str(&*printModStr(r#mod.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn unelabSubmods(mut inTypesSubModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSCodeSubModLst: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    outSCodeSubModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut x in (inTypesSubModLst.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(x.clone()) {
        Deref @ DAE::SubMod { r#mod: m, ident: i } => {
            let mut m_1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            m_1 = unelabMod(m.clone())?;
            Arc::new(SCode::SubMod { ident: (i.clone()).clone(), r#mod: m_1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outSCodeSubModLst)
}

fn unelabSubscript(mut inIntegerLst: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outSCodeSubscriptLst: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    outSCodeSubscriptLst = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut i in (inIntegerLst.clone()).into_iter().cloned() {
            let __x = Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() }) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outSCodeSubscriptLst
}

pub fn updateMod(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inBoolean: bool, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Mod>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    (outCache, outMod) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), inBoolean.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, Deref @ DAE::Mod::NOMOD, _, _) => {
                    Ok((cache.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, m @ Deref @ DAE::Mod::REDECL { .. }, _, _) => {
                    Ok((cache.clone(), m.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ DAE::Mod::MOD { info, binding: Some(DAE::EqMod::UNTYPED { exp: e }), subModLst: subs, eachPrefix: each_, finalPrefix: f }, r#impl, _) => {
                    let mut subs_1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut e_val: Option<Arc<Values::Value>> = None;
                    let mut cache = (*cache).clone();
                    (cache, subs_1) = updateSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), info.clone())?;
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, e_1, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (e_val, cache) = elabModValue(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), env.clone(), ih.clone(), e_1.clone(), pre.clone())?;
                    if Flags::isSet(Flags::UPDMOD.clone())? {
                        Debug::trace((literal!("Updated mod: ")).clone())?;
                        Debug::traceln((printModStr(Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_2.clone(), modifierAsValue: None, properties: prop.clone(), modifierAsAbsynExp: e.clone(), info: info.clone() }), info: info.clone() }))?).clone())?;
                    }
                    Ok((cache.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_2.clone(), modifierAsValue: e_val.clone(), properties: prop.clone(), modifierAsAbsynExp: e.clone(), info: info.clone() }), info: info.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ DAE::Mod::MOD { info, binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_1, modifierAsValue: e_val, properties: p, modifierAsAbsynExp: e, .. }), subModLst: subs, eachPrefix: each_, finalPrefix: f }, r#impl, _) => {
                    let mut subs_1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, subs_1) = updateSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::TYPED { modifierAsExp: e_1.clone(), modifierAsValue: e_val.clone(), properties: p.clone(), modifierAsAbsynExp: e.clone(), info: info.clone() }), info: info.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ DAE::Mod::MOD { info, binding: None, subModLst: subs, eachPrefix: each_, finalPrefix: f }, r#impl, _) => {
                    let mut subs_1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, subs_1) = updateSubmods(cache.clone(), env.clone(), ih.clone(), pre.clone(), subs.clone(), r#impl.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: None, info: info.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, m, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = (printModStr(m.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Mod.updateMod failed mod: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outMod))
}

fn updateSubmods(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inTypesSubModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inBoolean: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::SubMod>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outTypesSubModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    outTypesSubModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
        for mut x in (inTypesSubModLst.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(x.clone()) {
        Deref @ DAE::SubMod { r#mod: m, ident: i } => {
            let mut m_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            (outCache, m_1) = updateMod(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), m.clone(), inBoolean.clone(), info.clone())?;
            Arc::new(DAE::SubMod { ident: (i.clone()).clone(), r#mod: m_1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outCache, outTypesSubModLst))
}

pub fn elabUntypedMod(mut inMod: Arc<SCode::Mod>, mut inModScope: ModScope) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::NOMOD => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { info, binding: None, subModLst: subs, eachPrefix: each_, finalPrefix, .. } => {
                    let mut subs_1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
                    subs_1 = elabUntypedSubmods(subs.clone(), inModScope.clone())?;
                    Ok(Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: None, info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { info, binding: Some(e), subModLst: subs, eachPrefix: each_, finalPrefix, .. } => {
                    let mut subs_1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
                    subs_1 = elabUntypedSubmods(subs.clone(), inModScope.clone())?;
                    Ok(Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), subModLst: subs_1.clone(), binding: Some(DAE::EqMod::UNTYPED { exp: e.clone() }), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { element: elem, eachPrefix: each_, finalPrefix } => {
                    Ok(Arc::new(DAE::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: each_.clone(), element: elem.clone(), r#mod: Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s: ArcStr = arcstr::literal!("");
                    println!("{}", (literal!("- elab_untyped_mod ")).clone());
                    s = (SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    println!("{}", (s.clone()).clone());
                    println!("{}", (literal!(" failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn elabSubmods(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSCodeSubModLst: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inBoolean: bool, mut inModScope: ModScope, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::SubMod>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outTypesSubModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    submods = compactSubMods(inSCodeSubModLst.clone(), inModScope.clone());
    (outCache, outTypesSubModLst) = elabSubmods2(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), submods.clone(), inBoolean.clone(), info.clone(), metamodelica::nil())?;
    Ok((outCache, outTypesSubModLst))
}

fn elabSubmods2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inImpl: bool, mut inInfo: SourceInfo, mut inAccumMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::SubMod>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    (outCache, outSubMods) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSubMods.clone(), inImpl.clone(), inInfo.clone(), inAccumMods.clone())) {
        (cache, _, _, _, Deref @ metamodelica::List::Cons { head: smod, tail: rest_smods }, _, _, _) => {
            let mut dmod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
            let mut accum_mods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, dmod) = elabSubmod(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), smod.clone(), inImpl.clone(), inInfo.clone())?;
            (cache, accum_mods) = elabSubmods2(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), rest_smods.clone(), inImpl.clone(), inInfo.clone(), cons(dmod.clone(), inAccumMods.clone()))?;
            (cache.clone(), accum_mods.clone())
        },
        _ => {
            (inCache.clone(), inAccumMods.clone().reverse())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outSubMods))
}

fn compactSubMods(mut inSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inModScope: ModScope) -> Arc<metamodelica::List<Arc<SCode::SubMod>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    submods = List::fold2(inSubMods.clone(), (std::sync::Arc::new(compactSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>, ModScope, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> + 'static>), inModScope.clone(), metamodelica::nil(), metamodelica::nil());
    outSubMods = submods.clone().reverse();
    outSubMods
}

fn compactSubMod(mut inSubMod: Arc<SCode::SubMod>, mut inModScope: ModScope, mut inName: Arc<metamodelica::List<ArcStr>>, mut inAccumMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut found: bool = false;
    (submods, found) = List::findMap(inAccumMods.clone(), Arc::new({ let __pe_b1 = inSubMod.clone(); let __pe_b2 = inModScope.clone(); let __pe_b3 = inName.clone(); move |__pe_a0| compactSubMod2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    outSubMods = List::consOnTrue(!(found.clone()), inSubMod.clone(), submods.clone());
    Ok(outSubMods)
}

fn compactSubMod2(mut inExistingMod: Arc<SCode::SubMod>, mut inNewMod: Arc<SCode::SubMod>, mut inModScope: ModScope, mut inName: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<SCode::SubMod>, bool)> {
    let mut outMod: Arc<SCode::SubMod> = Arc::new(<SCode::SubMod as ::std::default::Default>::default());
    let mut outFound: bool = false;
    (outMod, outFound) = (::match_deref::match_deref! { match &((inExistingMod.clone(), inNewMod.clone(), inModScope.clone(), inName.clone())) {
        (Deref @ SCode::SubMod { ident: name1, .. }, Deref @ SCode::SubMod { ident: name2, .. }, _, _) if (!(stringEqual((name1.clone()).clone(), (name2.clone()).clone()))) => {
            (inExistingMod.clone(), false)
        },
        (Deref @ SCode::SubMod { ident: name1, .. }, _, _, _) => {
            let mut submod: Arc<SCode::SubMod> = Arc::new(<SCode::SubMod as ::std::default::Default>::default());
            submod = mergeSubModsInSameScope(inExistingMod.clone(), inNewMod.clone(), cons((name1.clone()).clone(), inName.clone()), inModScope.clone())?;
            (submod.clone(), true)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outMod, outFound))
}

fn mergeSubModsInSameScope(mut inMod1: Arc<SCode::SubMod>, mut inMod2: Arc<SCode::SubMod>, mut inElementName: Arc<metamodelica::List<ArcStr>>, mut inModScope: ModScope) -> Result<Arc<SCode::SubMod>> {
    let mut outMod: Arc<SCode::SubMod> = Arc::new(<SCode::SubMod as ::std::default::Default>::default());
    let mut scope: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut info1: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut info2: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut mod1: Arc<SCode::Mod> = inMod1.r#mod.clone();
    let mut mod2: Arc<SCode::Mod> = inMod2.r#mod.clone();
    outMod = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ SCode::Mod::MOD { .. }, Deref @ SCode::Mod::MOD { binding: None, .. }) => {
            submods = List::fold2(var_field!((*mod1).subModLst, SCode::Mod::MOD).clone(), (std::sync::Arc::new(compactSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>, ModScope, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> + 'static>), inModScope.clone(), inElementName.clone(), var_field!((*mod2).subModLst, SCode::Mod::MOD).clone());
            Arc::new(SCode::SubMod { ident: (inMod1.ident.clone()).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: var_field!((*mod1).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*mod1).eachPrefix, SCode::Mod::MOD).clone(), subModLst: submods.clone(), binding: var_field!((*mod1).binding, SCode::Mod::MOD).clone(), comment: var_field!((*mod1).comment, SCode::Mod::MOD).clone(), info: var_field!((*mod1).info, SCode::Mod::MOD).clone() }) })
        },
        (Deref @ SCode::Mod::MOD { binding: None, .. }, Deref @ SCode::Mod::MOD { .. }) => {
            submods = List::fold2(var_field!((*mod1).subModLst, SCode::Mod::MOD).clone(), (std::sync::Arc::new(compactSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>, ModScope, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> + 'static>), inModScope.clone(), inElementName.clone(), var_field!((*mod2).subModLst, SCode::Mod::MOD).clone());
            Arc::new(SCode::SubMod { ident: (inMod2.ident.clone()).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: var_field!((*mod2).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*mod2).eachPrefix, SCode::Mod::MOD).clone(), subModLst: submods.clone(), binding: var_field!((*mod2).binding, SCode::Mod::MOD).clone(), comment: var_field!((*mod2).comment, SCode::Mod::MOD).clone(), info: var_field!((*mod2).info, SCode::Mod::MOD).clone() }) })
        },
        _ => {
            info1 = SCodeUtil::getModifierInfo(mod1.clone());
            info2 = SCodeUtil::getModifierInfo(mod2.clone());
            scope = (printModScope(inModScope.clone())?).clone();
            name = stringDelimitList(inElementName.clone().reverse(), (literal!(".")).clone());
            Error::addMultiSourceMessage(Error::DUPLICATE_MODIFICATIONS.clone(), list![(name.clone()).clone(), (scope.clone()).clone()], list![info2.clone(), info1.clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn printModScope(mut inModScope: ModScope) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inModScope.clone() {
        ModScope::COMPONENT { name: mut name } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("component ")).clone())); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }
        },
        ModScope::EXTENDS { path: mut path } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("extends ")).clone())); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }
        },
        ModScope::DERIVED { path: mut path } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("inherited class ")).clone())); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn elabSubmod(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSubMod: Arc<SCode::SubMod>, mut inBoolean: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::SubMod>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSubMod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut dmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut i: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::SubMod { r#mod: __pa0, ident: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    smod = __pa0.clone();
    i = __pa1.clone();
    (outCache, dmod) = elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), smod.clone(), inBoolean.clone(), ModScope::COMPONENT { name: (i.clone()).clone() }, info.clone())?;
    outSubMod = Arc::new(DAE::SubMod { ident: (i.clone()).clone(), r#mod: dmod.clone() });
    Ok((outCache, outSubMod))
}

fn elabUntypedSubmods(mut inSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inModScope: ModScope) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    submods = compactSubMods(inSubMods.clone(), inModScope.clone());
    outSubMods = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
        for mut m in (submods.clone().reverse()).into_iter().cloned() {
            let __x = elabUntypedSubmod(m.clone())?;
            __acc = __x.append(&__acc);
        }
        __acc
    };
    Ok(outSubMods)
}

fn elabUntypedSubmod(mut inSubMod: Arc<SCode::SubMod>) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut outTypesSubModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    outTypesSubModLst = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::SubMod { r#mod: m, ident: i } => {
            let mut m_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            m_1 = elabUntypedMod(m.clone(), ModScope::COMPONENT { name: (literal!("")).clone() })?;
            list![Arc::new(DAE::SubMod { ident: (i.clone()).clone(), r#mod: m_1.clone() })]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTypesSubModLst)
}

// - Lookup
// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn lookupModificationP(mut inMod: Arc<DAE::Mod>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inMod.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ Absyn::Path::IDENT { name: n }) => {
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    r#mod = lookupCompModification(m.clone(), (n.clone()).clone())?;
                    Ok(r#mod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ Absyn::Path::FULLYQUALIFIED { path: p }) => {
                    Ok(lookupModificationP(m.clone(), p.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ Absyn::Path::QUALIFIED { path: p, name: n }) => {
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    r#mod = lookupCompModification(m.clone(), (n.clone()).clone())?;
                    mod_1 = lookupModificationP(r#mod.clone(), p.clone())?;
                    Ok(mod_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf((literal!("- Mod.lookupModificationP failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

pub fn lookupCompModification(mut inMod: Arc<DAE::Mod>, mut inIdent: ArcStr) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &((inMod.clone(), inIdent.clone())) {
        (Deref @ DAE::Mod::MOD { info, binding: eqMod, subModLst: subs, eachPrefix: e, finalPrefix: f }, n) => {
            let mut mod1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            mod1 = lookupCompModification2(subs.clone(), (n.clone()).clone())?;
            mod2 = lookupComplexCompModification(eqMod.clone(), (n.clone()).clone(), f.clone(), e.clone(), info.clone());
            checkDuplicateModifications(mod1.clone(), mod2.clone(), (n.clone()).clone())?
        },
        _ => {
            Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

pub fn getModifs(mut inMods: Arc<DAE::Mod>, mut inName: ArcStr, mut inSMod: Arc<SCode::Mod>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inMods.clone(), inName.clone(), inSMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    m = lookupCompModification(inMods.clone(), (inName.clone()).clone())?;
                    m = mergeModifiers(inMods.clone(), m.clone(), inSMod.clone())?;
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    m = mergeModifiers(inMods.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), inSMod.clone())?;
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn mergeModifiers(mut inMods: Arc<DAE::Mod>, mut inMod: Arc<DAE::Mod>, mut inSMod: Arc<SCode::Mod>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inMods.clone(), inMod.clone(), inSMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: sl, binding: _, comment: _, .. }) => {
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    m = mergeSubMods(inMods.clone(), inMod.clone(), f.clone(), e.clone(), sl.clone())?;
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn mergeSubMods(mut inMods: Arc<DAE::Mod>, mut inMod: Arc<DAE::Mod>, mut f: SCode::Final, mut e: SCode::Each, mut inSMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = inSMods.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod: Deref @ SCode::Mod::MOD { info, binding: Some(Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: _ } }), .. } }, tail: rest } => {
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    m = lookupCompModification(inMods.clone(), (id.clone()).clone())?;
                    m = Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: list![Arc::new(DAE::SubMod { ident: (n.clone()).clone(), r#mod: m.clone() })], binding: None, info: info.clone() });
                    m = merge(inMod.clone(), m.clone(), (literal!("")).clone(), true)?;
                    m = mergeSubMods(inMods.clone(), m.clone(), f.clone(), e.clone(), rest.clone())?;
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    m = mergeSubMods(inMods.clone(), inMod.clone(), f.clone(), e.clone(), rest.clone())?;
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

pub fn lookupCompModificationFromEqu(mut inMod: Arc<DAE::Mod>, mut inIdent: ArcStr) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &((inMod.clone(), inIdent.clone())) {
        (Deref @ DAE::Mod::NOMOD, _) => {
            Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)
        },
        (Deref @ DAE::Mod::REDECL { .. }, _) => {
            Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)
        },
        (Deref @ DAE::Mod::MOD { info, binding: eqMod, subModLst: subs, eachPrefix: e, finalPrefix: f }, n) => {
            let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut mod1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            mod1 = lookupCompModification2(subs.clone(), (n.clone()).clone())?;
            mod2 = lookupComplexCompModification(eqMod.clone(), (n.clone()).clone(), f.clone(), e.clone(), info.clone());
            r#mod = selectEqMod(mod1.clone(), mod2.clone(), (n.clone()).clone())?;
            r#mod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn selectEqMod(mut subMod: Arc<DAE::Mod>, mut eqMod: Arc<DAE::Mod>, mut n: ArcStr) -> Result<Arc<DAE::Mod>> {
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    r#mod = (::match_deref::match_deref! { match &((subMod.clone(), eqMod.clone(), n.clone())) {
        (_, Deref @ DAE::Mod::NOMOD, _) => subMod.clone(),
        (_, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }, _) => eqMod.clone(),
        _ => {
            r#mod = checkDuplicateModifications(subMod.clone(), eqMod.clone(), (n.clone()).clone())?;
            r#mod.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

fn lookupComplexCompModification(mut inEqMod: Option<DAE::EqMod>, mut inName: ArcStr, mut inFinal: SCode::Final, mut inEach: SCode::Each, mut inInfo: SourceInfo) -> Arc<DAE::Mod> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD);
    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut name: ArcStr = arcstr::literal!("");
    let mut e: Arc<DAE::Exp>;
    let mut ae: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut eq_mod: DAE::EqMod;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    if '__try0: {
        let Some(DAE::TYPED { info: __pa1, modifierAsValue: Some(Values::RECORD { index: (-1), comp: __pa2, orderd: __pa3, .. }), .. }) = (inEqMod.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        info = __pa1.clone();
        names = __pa2.clone();
        values = __pa3.clone();
        for mut name in &*names.clone() {
            let mut name = name.clone();
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(values.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            v = __pa4.clone();
            values = __pa5.clone();
            if name.clone() == inName.clone() {
                e = unwrap_break_err!(ValuesUtil::valueExp(v.clone(), None), '__try0);
                ae = unwrap_break_err!(Expression::unelabExp(e.clone()), '__try0);
                ty = unwrap_break_err!(Types::complicateType(Expression::r#typeof(e.clone()).unwrap()), '__try0);
                eq_mod = DAE::EqMod::TYPED { modifierAsExp: e.clone(), modifierAsValue: Some(v.clone()), properties: DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }, modifierAsAbsynExp: ae.clone(), info: info.clone() };
                outMod = Arc::new(DAE::Mod::MOD { finalPrefix: inFinal.clone(), eachPrefix: inEach.clone(), subModLst: metamodelica::nil(), binding: Some(eq_mod.clone()), info: inInfo.clone() });
                break;
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    outMod
}

fn checkDuplicateModifications(mut mod1: Arc<DAE::Mod>, mut mod2: Arc<DAE::Mod>, mut n: ArcStr) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ DAE::Mod::NOMOD, _) => {
            mod2.clone()
        },
        (_, Deref @ DAE::Mod::NOMOD) => {
            mod1.clone()
        },
        (Deref @ DAE::Mod::REDECL { .. }, Deref @ DAE::Mod::MOD { .. }) => {
            mergeRedeclareWithBinding(mod1.clone(), mod2.clone())?
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::REDECL { .. }) => {
            mergeRedeclareWithBinding(mod2.clone(), mod1.clone())?
        },
        (Deref @ DAE::Mod::MOD { binding: None, .. }, Deref @ DAE::Mod::MOD { .. }) => {
            let mut submods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            submods = checkDuplicateModifications2(var_field!((*mod1).subModLst, DAE::Mod::MOD).clone(), var_field!((*mod2).subModLst, DAE::Mod::MOD).clone(), (n.clone()).clone())?;
            Arc::new(DAE::Mod::MOD { finalPrefix: var_field!((*mod2).finalPrefix, DAE::Mod::MOD).clone(), eachPrefix: var_field!((*mod2).eachPrefix, DAE::Mod::MOD).clone(), subModLst: submods.clone(), binding: var_field!((*mod2).binding, DAE::Mod::MOD).clone(), info: var_field!((*mod2).info, DAE::Mod::MOD).clone() })
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { binding: None, .. }) => {
            let mut submods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            submods = checkDuplicateModifications2(var_field!((*mod1).subModLst, DAE::Mod::MOD).clone(), var_field!((*mod2).subModLst, DAE::Mod::MOD).clone(), (n.clone()).clone())?;
            Arc::new(DAE::Mod::MOD { finalPrefix: var_field!((*mod1).finalPrefix, DAE::Mod::MOD).clone(), eachPrefix: var_field!((*mod1).eachPrefix, DAE::Mod::MOD).clone(), subModLst: submods.clone(), binding: var_field!((*mod1).binding, DAE::Mod::MOD).clone(), info: var_field!((*mod1).info, DAE::Mod::MOD).clone() })
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { .. }) => {
            Error::addMultiSourceMessage(Error::DUPLICATE_MODIFICATIONS.clone(), list![(n.clone()).clone(), (literal!("")).clone()], list![getModInfo(mod1.clone()), getModInfo(mod2.clone())])?;
            mod2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn checkDuplicateModifications2(mut inSubMods1: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inSubMods2: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inName: ArcStr) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut submods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = inSubMods2.clone();
    let mut osubmod: Option<Arc<DAE::SubMod>> = None;
    let mut submod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
    let mut info1: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut info2: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    for mut s in &*inSubMods1.clone() {
        let mut s = s.clone();
        (submods, osubmod) = List::deleteMemberOnTrue((subModName(s.clone())?).clone(), submods.clone(), (std::sync::Arc::new(isSubModNamed) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::SubMod>) -> Result<bool> + 'static>))?;
        if isSome(osubmod.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(osubmod.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            submod = __pa0.clone();
            info1 = subModInfo(s.clone())?;
            info2 = subModInfo(submod.clone())?;
            Error::addMultiSourceMessage(Error::MULTIPLE_MODIFIER.clone(), list![(inName.clone()).clone()], list![info1.clone(), info2.clone()])?;
        }
    }
    outSubMods = listAppend(inSubMods1.clone(), inSubMods2.clone());
    Ok(outSubMods)
}

fn mergeRedeclareWithBinding(mut inRedeclare: Arc<DAE::Mod>, mut inBinding: Arc<DAE::Mod>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = inRedeclare.clone();
    outMod = (::match_deref::match_deref! { match &((outMod.clone(), inBinding.clone())) {
        (Deref @ DAE::Mod::REDECL { .. }, Deref @ DAE::Mod::MOD { binding: Some(_), subModLst: Deref @ metamodelica::List::Nil, .. }) => {
            assign_variant_field!(outMod => DAE::Mod::REDECL; r#mod = merge(inBinding.clone(), var_field!((*outMod).r#mod, DAE::Mod::REDECL).clone(), (literal!("")).clone(), true)?);
            outMod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn modEqualNoPrefix(mut mod1: Arc<DAE::Mod>, mut mod2: Arc<DAE::Mod>) -> Result<(Arc<DAE::Mod>, bool)> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut equal: bool = false;
    (outMod, equal) = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { .. }) => {
            let true = (subModsEqual(var_field!((*mod1).subModLst, DAE::Mod::MOD).clone(), var_field!((*mod2).subModLst, DAE::Mod::MOD).clone())?) else { bail!("pattern mismatch") };
            let true = (eqModEqual(var_field!((*mod1).binding, DAE::Mod::MOD).clone(), var_field!((*mod2).binding, DAE::Mod::MOD).clone())?) else { bail!("pattern mismatch") };
            (mod2.clone(), true)
        },
        (Deref @ DAE::Mod::REDECL { .. }, Deref @ DAE::Mod::REDECL { .. }) => {
            let true = (SCodeUtil::elementEqual(var_field!((*mod1).element, DAE::Mod::REDECL).clone(), var_field!((*mod2).element, DAE::Mod::REDECL).clone())?) else { bail!("pattern mismatch") };
            (mod2.clone(), true)
        },
        (Deref @ DAE::Mod::NOMOD, Deref @ DAE::Mod::NOMOD) => (Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), true),
        _ => (mod2.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outMod, equal))
}

fn lookupNamedSubMod(mut inSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inIdent: ArcStr) -> Result<Arc<DAE::SubMod>> {
    let mut outSubMod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
    outSubMod = List::getMemberOnTrue((inIdent.clone()).clone(), inSubMods.clone(), (std::sync::Arc::new(isSubModNamed) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::SubMod>) -> Result<bool> + 'static>))?;
    Ok(outSubMod)
}

fn isSubModNamed(mut inIdent: ArcStr, mut inSubMod: Arc<DAE::SubMod>) -> Result<bool> {
    let mut outIsNamed: bool = false;
    let mut ident: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { ident: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ident = __pa0.clone();
    outIsNamed = stringEq((inIdent.clone()).clone(), (ident.clone()).clone());
    Ok(outIsNamed)
}

pub fn printSubsStr(mut inSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut addParan: bool) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = stringDelimitList(List::map(inSubMods.clone(), (std::sync::Arc::new(prettyPrintSubmod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone());
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (addParan.clone()) {literal!("(")} else {literal!("")}); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*if (addParan.clone()) {literal!(")")} else {literal!("")}); ArcStr::from(__mm_s) }).clone();
    s
}

fn lookupCompModification2(mut inSubModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inIdent: ArcStr) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inSubModLst.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let __pa0 = ::match_deref::match_deref! { match &(lookupNamedSubMod(inSubModLst.clone(), (inIdent.clone()).clone())?) {
                        Deref @ DAE::SubMod { r#mod: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#mod = __pa0.clone();
                    Ok(r#mod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

pub fn lookupIdxModification(mut inMod: Arc<DAE::Mod>, mut inIndex: Arc<DAE::Exp>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { .. } => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { .. } => {
                    let mut mod1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut subs: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
                    let mut eq: Option<DAE::EqMod> = None;
                    (mod1, subs) = lookupIdxModification2(var_field!((*inMod).subModLst, DAE::Mod::MOD).clone(), inIndex.clone())?;
                    mod2 = Arc::new(DAE::Mod::MOD { finalPrefix: var_field!((*inMod).finalPrefix, DAE::Mod::MOD).clone(), eachPrefix: var_field!((*inMod).eachPrefix, DAE::Mod::MOD).clone(), subModLst: subs.clone(), binding: None, info: var_field!((*inMod).info, DAE::Mod::MOD).clone() });
                    mod2 = merge(mod2.clone(), mod1.clone(), (literal!("")).clone(), true)?;
                    eq = indexEqmod(var_field!((*inMod).binding, DAE::Mod::MOD).clone(), list![inIndex.clone()], var_field!((*inMod).info, DAE::Mod::MOD).clone())?;
                    mod1 = Arc::new(DAE::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: var_field!((*inMod).eachPrefix, DAE::Mod::MOD).clone(), subModLst: metamodelica::nil(), binding: eq.clone(), info: var_field!((*inMod).info, DAE::Mod::MOD).clone() });
                    mod2 = merge(mod2.clone(), mod1.clone(), (literal!("")).clone(), true)?;
                    Ok(mod2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Mod.lookupIdxModification(")).clone())?;
                    Debug::trace((printModStr(inMod.clone())?).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inIndex.clone())?); __mm_s.push_str(&*literal!(") failed")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn lookupIdxModification2(mut inSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inIndex: Arc<DAE::Exp>) -> Result<(Arc<DAE::Mod>, Arc<metamodelica::List<Arc<DAE::SubMod>>>)> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD);
    let mut outSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut name: ArcStr = arcstr::literal!("");
    for mut submod in &*inSubMods.clone() {
        let mut submod = submod.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(submod.clone()) {
            Deref @ DAE::SubMod { ident: __pa0, r#mod: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa0.clone();
        r#mod = __pa1.clone();
        r#mod = lookupIdxModification3(r#mod.clone(), inIndex.clone())?;
        if !(isNoMod(r#mod.clone())) {
            outSubMods = cons(Arc::new(DAE::SubMod { ident: (name.clone()).clone(), r#mod: r#mod.clone() }), outSubMods.clone());
        }
    }
    outSubMods = outSubMods.clone().reverse();
    Ok((outMod, outSubMods))
}

fn lookupIdxModification3(mut inMod: Arc<DAE::Mod>, mut inIndex: Arc<DAE::Exp>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::NOMOD => {
            Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)
        },
        Deref @ DAE::Mod::REDECL { .. } => {
            inMod.clone()
        },
        Deref @ DAE::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH, .. } => {
            let mut subs: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            let mut eq: Option<DAE::EqMod> = None;
            (_, subs) = lookupIdxModification2(var_field!((*inMod).subModLst, DAE::Mod::MOD).clone(), inIndex.clone())?;
            eq = indexEqmod(var_field!((*inMod).binding, DAE::Mod::MOD).clone(), list![inIndex.clone()], var_field!((*inMod).info, DAE::Mod::MOD).clone())?;
            Arc::new(DAE::Mod::MOD { finalPrefix: var_field!((*inMod).finalPrefix, DAE::Mod::MOD).clone(), eachPrefix: var_field!((*inMod).eachPrefix, DAE::Mod::MOD).clone(), subModLst: subs.clone(), binding: eq.clone(), info: var_field!((*inMod).info, DAE::Mod::MOD).clone() })
        },
        Deref @ DAE::Mod::MOD { eachPrefix: SCode::Each::EACH, .. } => {
            inMod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn indexEqmod(mut inBinding: Option<DAE::EqMod>, mut inIndices: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inInfo: SourceInfo) -> Result<Option<DAE::EqMod>> {
    let mut outBinding: Option<DAE::EqMod> = inBinding.clone();
    let mut exp: Arc<DAE::Exp>;
    let mut oval: Option<Arc<Values::Value>> = None;
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut eq: DAE::EqMod;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    if isNone(inBinding.clone()) || inIndices.clone().is_empty() {
        return Ok(outBinding);
    }
    let Some(__pa0) = (inBinding.clone()) else { bail!("pattern mismatch") };
    eq = __pa0.clone();
    outBinding = 'mc: {
        let __mc_input = eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::EqMod::TYPED { modifierAsValue: Some(Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, .. }), .. } => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::EqMod::TYPED { modifierAsExp: exp, modifierAsValue: oval, properties: DAE::Properties::PROP { type_: ty, constFlag: c }, modifierAsAbsynExp: aexp, info } => {
                    let mut exp = (*exp).clone();
                    let mut oval = (*oval).clone();
                    let mut ty = (*ty).clone();
                    let mut val: Arc<Values::Value> = val.clone();
                    for mut i in &*inIndices.clone() {
                        let mut i = i.clone();
                        if !(Types::isArray(ty.clone())) {
                            Error::addSourceMessage(Error::MODIFIER_NON_ARRAY_TYPE_WARNING.clone(), list![(ExpressionBasics::printExpStr(exp.clone())?).clone()], inInfo.clone())?;
                            return Ok(outBinding);
                        }
                        ty = Types::unliftArray(ty.clone())?;
                        (exp, _) = ExpressionSimplify::simplify1(Expression::makeASUB(exp.clone(), list![i.clone()])?)?;
                    }
                    if isSome(oval.clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(oval.clone()) {
                            Some(__pa0) => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        val = __pa0.clone();
                        for mut i in &*inIndices.clone() {
                            let mut i = i.clone();
                            val = ValuesUtil::nthArrayelt(val.clone(), ExpressionBasics::expArrayIndex(i.clone())?)?;
                        }
                        oval = Some(val.clone());
                    }
                    Ok(Some(DAE::EqMod::TYPED { modifierAsExp: exp.clone(), modifierAsValue: oval.clone(), properties: DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() }, modifierAsAbsynExp: aexp.clone(), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Mod.indexEqmod failed for mod:\n ")); __mm_s.push_str(&*TypesDump::unparseEqMod(eq.clone())?); __mm_s.push_str(&*literal!("\n indices: ")); __mm_s.push_str(&*ExpressionDump::printExpListStr(inIndices.clone())); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBinding)
}

pub fn merge(mut inModOuter: Arc<DAE::Mod>, mut inModInner: Arc<DAE::Mod>, mut inElementName: ArcStr, mut inCheckFinal: bool) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut mod_str: ArcStr = arcstr::literal!("");
    if isEmptyMod(inModOuter.clone()) {
        outMod = inModInner.clone();
    } else if isEmptyMod(inModInner.clone()) {
        outMod = inModOuter.clone();
    } else if inCheckFinal.clone() && isFinalMod(inModInner.clone()) && !(merge_isEqual(inModOuter.clone(), inModInner.clone())) && !(isRedeclareMod(inModOuter.clone())) {
        mod_str = (unparseModStr(inModOuter.clone())?).clone();
        Error::addMultiSourceMessage(Error::FINAL_COMPONENT_OVERRIDE.clone(), list![(inElementName.clone()).clone(), (mod_str.clone()).clone()], list![getModInfo(inModInner.clone()), getModInfo(inModOuter.clone())])?;
        bail!("fail");
    } else {
        outMod = doMerge(inModOuter.clone(), inModInner.clone(), inCheckFinal.clone())?;
    }
    Ok(outMod)
}

fn merge_isEqual(mut inMod1: Arc<DAE::Mod>, mut inMod2: Arc<DAE::Mod>) -> bool {
    let mut outIsEqual: bool = false;
    let mut info1: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut info2: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    if referenceEq(&inMod1.clone(),&inMod2.clone()) {
        outIsEqual = true;
    } else {
        info1 = getModInfo(inMod1.clone());
        info2 = getModInfo(inMod2.clone());
        outIsEqual = !(Util::sourceInfoIsEmpty(info1.clone()) || Util::sourceInfoIsEmpty(info2.clone())) && Util::sourceInfoIsEqual(info1.clone(), info2.clone());
    }
    outIsEqual
}

pub fn isFinalMod(mut inMod1: Arc<DAE::Mod>) -> bool {
    let mut outMod: bool = false;
    outMod = (::match_deref::match_deref! { match &(inMod1.clone()) {
        Deref @ DAE::Mod::MOD { finalPrefix: SCode::Final::FINAL, .. } => true,
        Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { finalPrefix: SCode::Final::FINAL, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

fn doMerge(mut inModOuter: Arc<DAE::Mod>, mut inModInner: Arc<DAE::Mod>, mut inCheckFinal: bool) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = inModOuter.clone();
    outMod = (::match_deref::match_deref! { match &((outMod.clone(), inModInner.clone())) {
        (Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { .. }, .. }, Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: None }, .. }, .. }, .. }) => {
            inModOuter.clone()
        },
        (Deref @ DAE::Mod::REDECL { r#mod: emod1, element: el1 @ Deref @ SCode::Element::COMPONENT { .. }, .. }, Deref @ DAE::Mod::REDECL { r#mod: emod2, element: el2 @ Deref @ SCode::Element::COMPONENT { .. }, .. }) => {
            let mut smod1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut smod2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut emod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut dmod1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut dmod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut dmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut el1 = (*el1).clone();
            smod1 = SCodeUtil::getConstrainedByModifiers(var_field!((*el1).prefixes, SCode::Element::COMPONENT).clone());
            smod1 = SCodeUtil::mergeModifiers(var_field!((*el1).modifications, SCode::Element::COMPONENT).clone(), smod1.clone())?;
            dmod1 = elabUntypedMod(smod1.clone(), ModScope::COMPONENT { name: (var_field!((*el1).name, SCode::Element::COMPONENT).clone()).clone() })?;
            smod2 = SCodeUtil::getConstrainedByModifiers(var_field!((**el2).prefixes, SCode::Element::COMPONENT).clone());
            smod2 = SCodeUtil::mergeModifiers(var_field!((**el2).modifications, SCode::Element::COMPONENT).clone(), smod2.clone())?;
            dmod2 = elabUntypedMod(smod2.clone(), ModScope::COMPONENT { name: (var_field!((**el2).name, SCode::Element::COMPONENT).clone()).clone() })?;
            dmod = merge(dmod1.clone(), dmod2.clone(), (var_field!((*el1).name, SCode::Element::COMPONENT).clone()).clone(), inCheckFinal.clone())?;
            emod = merge(emod1.clone(), emod2.clone(), (var_field!((*el1).name, SCode::Element::COMPONENT).clone()).clone(), inCheckFinal.clone())?;
            assign_variant_field!(el1 => SCode::Element::COMPONENT;
                modifications = unelabMod(dmod.clone())?,
                prefixes = SCodeUtil::propagatePrefixes(var_field!((**el2).prefixes, SCode::Element::COMPONENT).clone(), var_field!((*el1).prefixes, SCode::Element::COMPONENT).clone())?,
                attributes = SCodeUtil::propagateAttributes(var_field!((**el2).attributes, SCode::Element::COMPONENT).clone(), var_field!((*el1).attributes, SCode::Element::COMPONENT).clone(), false)?
            );
            assign_variant_field!(outMod => DAE::Mod::REDECL;
                element = el1.clone(),
                r#mod = emod.clone()
            );
            outMod.clone()
        },
        (Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::CLASS { .. }, .. }, Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: None }, .. }, .. }, .. }) => {
            inModOuter.clone()
        },
        (Deref @ DAE::Mod::REDECL { r#mod: emod1, element: el1 @ Deref @ SCode::Element::CLASS { .. }, .. }, Deref @ DAE::Mod::REDECL { r#mod: emod2, element: el2 @ Deref @ SCode::Element::CLASS { .. }, .. }) => {
            let mut smod1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut smod2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut emod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut dmod1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut dmod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut res: SCode::Restriction = SCode::Restriction::R_BLOCK;
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut emod1 = (*emod1).clone();
            let mut el1 = (*el1).clone();
            let mut emod2 = (*emod2).clone();
            smod1 = SCodeUtil::getConstrainedByModifiers(var_field!((*el1).prefixes, SCode::Element::CLASS).clone());
            dmod1 = elabUntypedMod(smod1.clone(), ModScope::COMPONENT { name: (var_field!((*el1).name, SCode::Element::CLASS).clone()).clone() })?;
            emod1 = merge(emod1.clone(), dmod1.clone(), (var_field!((*el1).name, SCode::Element::CLASS).clone()).clone(), inCheckFinal.clone())?;
            smod2 = SCodeUtil::getConstrainedByModifiers(var_field!((**el2).prefixes, SCode::Element::CLASS).clone());
            dmod2 = elabUntypedMod(smod2.clone(), ModScope::COMPONENT { name: (var_field!((**el2).name, SCode::Element::CLASS).clone()).clone() })?;
            emod2 = merge(emod2.clone(), dmod2.clone(), (var_field!((*el1).name, SCode::Element::CLASS).clone()).clone(), inCheckFinal.clone())?;
            emod = merge(emod1.clone(), emod2.clone(), (var_field!((*el1).name, SCode::Element::CLASS).clone()).clone(), inCheckFinal.clone())?;
            assign_variant_field!(el1 => SCode::Element::CLASS; prefixes = SCodeUtil::propagatePrefixes(var_field!((**el2).prefixes, SCode::Element::CLASS).clone(), var_field!((**el2).prefixes, SCode::Element::CLASS).clone())?);
            (res, info) = SCodeUtil::checkSameRestriction(var_field!((*el1).restriction, SCode::Element::CLASS).clone(), var_field!((**el2).restriction, SCode::Element::CLASS).clone(), var_field!((*el1).info, SCode::Element::CLASS).clone(), var_field!((**el2).info, SCode::Element::CLASS).clone());
            assign_variant_field!(el1 => SCode::Element::CLASS;
                restriction = res.clone(),
                info = info.clone()
            );
            assign_variant_field!(outMod => DAE::Mod::REDECL;
                element = el1.clone(),
                r#mod = emod.clone()
            );
            outMod.clone()
        },
        (Deref @ DAE::Mod::REDECL { r#mod: emod, element: el1, .. }, Deref @ DAE::Mod::MOD { .. }) => {
            let mut emod = (*emod).clone();
            emod = merge(emod.clone(), inModInner.clone(), (literal!("")).clone(), inCheckFinal.clone())?;
            assign_variant_field!(outMod => DAE::Mod::REDECL;
                element = el1.clone(),
                r#mod = emod.clone()
            );
            outMod.clone()
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::REDECL { r#mod: emod, element: el2, .. }) => {
            let mut emod = (*emod).clone();
            emod = merge(inModOuter.clone(), emod.clone(), (literal!("")).clone(), inCheckFinal.clone())?;
            Arc::new(DAE::Mod::REDECL { finalPrefix: var_field!((*inModInner).finalPrefix, DAE::Mod::REDECL).clone(), eachPrefix: var_field!((*inModInner).eachPrefix, DAE::Mod::REDECL).clone(), element: el2.clone(), r#mod: emod.clone() })
        },
        (Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, binding: Some(eqmod @ DAE::EqMod::TYPED { modifierAsValue: Some(val @ Deref @ Values::Value::RECORD { .. }), .. }), .. }, Deref @ DAE::Mod::MOD { subModLst: submods @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, binding: None, .. }) => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut name: ArcStr = arcstr::literal!("");
            let mut submod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
            let mut eqmod = (*eqmod).clone();
            let mut val = (*val).clone();
            let mut submods = (*submods).clone();
            names = var_field!((*val).comp, Values::Value::RECORD).clone();
            vals = metamodelica::nil();
            for mut v in &*var_field!((*val).orderd, Values::Value::RECORD).clone() {
                let mut v = v.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(names.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                name = __pa0.clone();
                names = __pa1.clone();
                if ValuesUtil::isEmpty(v.clone()) {
                    if '__try2: {
                        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(List::deleteMemberOnTrue((name.clone()).clone(), submods.clone(), (std::sync::Arc::new(isSubModNamed) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::SubMod>) -> Result<bool> + 'static>)), '__try2)) {
                            (__pa3, Some(__pa4)) => (__pa3.clone(), __pa4.clone()),
                            _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        submods = __pa3.clone();
                        submod = __pa4.clone();
                        v = unwrap_break_err!(subModValue(submod.clone()), '__try2);
                        Ok::<(), anyhow::Error>(())
                    }.is_err() {
                    }
                }
                vals = cons(v.clone(), vals.clone());
            }
            assign_variant_field!(val => Values::Value::RECORD; orderd = vals.clone().reverse());
            let __owned_variant_modifierAsValue_0 = Some(val.clone());
            if let DAE::EqMod::TYPED { modifierAsValue, .. } = &mut eqmod {
                *modifierAsValue = __owned_variant_modifierAsValue_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::EqMod::TYPED"); }
            assign_variant_field!(outMod => DAE::Mod::MOD;
                binding = Some(eqmod.clone()),
                subModLst = stripSubModBindings(var_field!((*inModInner).subModLst, DAE::Mod::MOD).clone())?
            );
            outMod.clone()
        },
        (Deref @ DAE::Mod::MOD { subModLst: submods @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, binding: None, .. }, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, binding: Some(eqmod @ DAE::EqMod::TYPED { modifierAsValue: Some(val @ Deref @ Values::Value::RECORD { .. }), .. }), .. }) => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut name: ArcStr = arcstr::literal!("");
            let mut submod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
            let mut submods = (*submods).clone();
            let mut eqmod = (*eqmod).clone();
            let mut val = (*val).clone();
            names = var_field!((*val).comp, Values::Value::RECORD).clone();
            vals = metamodelica::nil();
            for mut v in &*var_field!((*val).orderd, Values::Value::RECORD).clone() {
                let mut v = v.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(names.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                name = __pa0.clone();
                names = __pa1.clone();
                if '__try2: {
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(List::deleteMemberOnTrue((name.clone()).clone(), submods.clone(), (std::sync::Arc::new(isSubModNamed) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::SubMod>) -> Result<bool> + 'static>)), '__try2)) {
                        (__pa3, Some(__pa4)) => (__pa3.clone(), __pa4.clone()),
                        _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    submods = __pa3.clone();
                    submod = __pa4.clone();
                    v = unwrap_break_err!(subModValue(submod.clone()), '__try2);
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
                vals = cons(v.clone(), vals.clone());
            }
            assign_variant_field!(val => Values::Value::RECORD; orderd = vals.clone().reverse());
            let __owned_variant_modifierAsValue_0 = Some(val.clone());
            if let DAE::EqMod::TYPED { modifierAsValue, .. } = &mut eqmod {
                *modifierAsValue = __owned_variant_modifierAsValue_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::EqMod::TYPED"); }
            assign_variant_field!(outMod => DAE::Mod::MOD;
                binding = Some(eqmod.clone()),
                subModLst = stripSubModBindings(var_field!((*outMod).subModLst, DAE::Mod::MOD).clone())?
            );
            outMod.clone()
        },
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { .. }) => {
            assign_variant_field!(outMod => DAE::Mod::MOD;
                subModLst = mergeSubs(var_field!((*outMod).subModLst, DAE::Mod::MOD).clone(), var_field!((*inModInner).subModLst, DAE::Mod::MOD).clone(), inCheckFinal.clone())?,
                binding = mergeEq(var_field!((*outMod).binding, DAE::Mod::MOD).clone(), var_field!((*inModInner).binding, DAE::Mod::MOD).clone())
            );
            outMod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn mergeSubs(mut inSubMods1: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inSubMods2: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inCheckFinal: bool) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut submods2: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut m1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut m2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut osm2: Option<Arc<DAE::SubMod>> = None;
    let mut sm2: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
    if inSubMods1.clone().is_empty() {
        outSubMods = inSubMods2.clone();
    } else if inSubMods2.clone().is_empty() {
        outSubMods = inSubMods1.clone();
    } else {
        submods2 = inSubMods2.clone();
        for mut sm1 in &*inSubMods1.clone() {
            let mut sm1 = sm1.clone();
            (submods2, osm2) = List::deleteMemberOnTrue((subModName(sm1.clone())?).clone(), submods2.clone(), (std::sync::Arc::new(subModIsNamed) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::SubMod>) -> Result<bool> + 'static>))?;
            if isSome(osm2.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(osm2.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                sm2 = __pa0.clone();
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(sm1.clone()) {
                    Deref @ DAE::SubMod { r#mod: __pa1, ident: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                m1 = __pa1.clone();
                name = __pa2.clone();
                let __pa3 = ::match_deref::match_deref! { match &(sm2.clone()) {
                    Deref @ DAE::SubMod { r#mod: __pa3, .. } => __pa3.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                m2 = __pa3.clone();
                m1 = merge(m1.clone(), m2.clone(), (name.clone()).clone(), inCheckFinal.clone())?;
                sm1 = Arc::new(DAE::SubMod { ident: (name.clone()).clone(), r#mod: m1.clone() });
            }
            outSubMods = cons(sm1.clone(), outSubMods.clone());
        }
        outSubMods = List::append_reverse(outSubMods.clone(), submods2.clone());
    }
    Ok(outSubMods)
}

fn mergeEq(mut inOuterEq: Option<DAE::EqMod>, mut inInnerEq: Option<DAE::EqMod>) -> Option<DAE::EqMod> {
    let mut outEqMod: Option<DAE::EqMod> = if (isSome(inOuterEq.clone())) {inOuterEq.clone()} else {inInnerEq.clone()};
    outEqMod
}

pub fn modEquation(mut inMod: Arc<DAE::Mod>) -> Result<Option<DAE::EqMod>> {
    let mut outEqMod: Option<DAE::EqMod> = None;
    outEqMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::NOMOD => None,
        Deref @ DAE::Mod::REDECL { .. } => None,
        Deref @ DAE::Mod::MOD { .. } => var_field!((*inMod).binding, DAE::Mod::MOD).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqMod)
}

fn modSubsetOrEqualOrNonOverlap(mut mod1: Arc<DAE::Mod>, mut mod2: Arc<DAE::Mod>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (mod1.clone(), mod2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { finalPrefix: f1, eachPrefix: _, subModLst: _, binding: None, info: _ }, Deref @ DAE::Mod::MOD { finalPrefix: f2, eachPrefix: SCode::Each::NOT_EACH, subModLst: Deref @ metamodelica::List::Nil, binding: Some(_), info: _ }) => {
                    let true = (SCodeUtil::finalEqual(f1.clone(), f2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { binding: eqmod1, .. }, Deref @ DAE::Mod::MOD { finalPrefix: _, eachPrefix: SCode::Each::NOT_EACH, subModLst: Deref @ metamodelica::List::Nil, binding: eqmod2, info: _ }) => {
                    let true = (eqModSubsetOrEqual(eqmod1.clone(), eqmod2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { finalPrefix: f1, eachPrefix: each1, subModLst: submods1, binding: eqmod1, info: _ }, Deref @ DAE::Mod::MOD { finalPrefix: f2, eachPrefix: each2, subModLst: submods2, binding: eqmod2, info: _ }) => {
                    let true = (SCodeUtil::finalEqual(f1.clone(), f2.clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::eachEqual(each1.clone(), each2.clone())) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(submods1.clone(), submods2.clone())?) else { bail!("pattern mismatch") };
                    let true = (eqModSubsetOrEqual(eqmod1.clone(), eqmod2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::REDECL { finalPrefix: f1, eachPrefix: each1, .. }, Deref @ DAE::Mod::REDECL { finalPrefix: f2, eachPrefix: each2, .. }) => {
                    let true = (SCodeUtil::finalEqual(f1.clone(), f2.clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::eachEqual(each1.clone(), each2.clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::elementEqual(var_field!((*mod1).element, DAE::Mod::REDECL).clone(), var_field!((*mod2).element, DAE::Mod::REDECL).clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::NOMOD, Deref @ DAE::Mod::NOMOD) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

fn eqModSubsetOrEqual(mut eqMod1: Option<DAE::EqMod>, mut eqMod2: Option<DAE::EqMod>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (eqMod1.clone(), eqMod2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (None, None) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (None, Some(_)) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::TYPED { .. }), Some(DAE::EqMod::TYPED { .. })) = __mc_input.clone() else { bail!("nomatch") };
            let true = (eqModEqual(eqMod1.clone(), eqMod2.clone())?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::TYPED { modifierAsAbsynExp: ref aexp1, .. }), Some(DAE::EqMod::UNTYPED { exp: ref aexp2 })) = __mc_input.clone() else { bail!("nomatch") };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::UNTYPED { exp: ref aexp1 }), Some(DAE::EqMod::TYPED { modifierAsAbsynExp: ref aexp2, .. })) = __mc_input.clone() else { bail!("nomatch") };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::UNTYPED { exp: ref aexp1 }), Some(DAE::EqMod::UNTYPED { exp: ref aexp2 })) = __mc_input.clone() else { bail!("nomatch") };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

fn subModsSubsetOrEqual(mut subModLst1: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut subModLst2: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (subModLst1.clone(), subModLst2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id1, r#mod: mod1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id2, r#mod: mod2 }, tail: rest2 }) => {
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (modEqual(mod1.clone(), mod2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(rest1.clone(), rest2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

pub fn modEqual(mut mod1: Arc<DAE::Mod>, mut mod2: Arc<DAE::Mod>) -> Result<bool> {
    let mut equal: bool = false;
    equal = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ DAE::Mod::MOD { .. }, Deref @ DAE::Mod::MOD { .. }) => SCodeUtil::finalEqual(var_field!((*mod1).finalPrefix, DAE::Mod::MOD).clone(), var_field!((*mod2).finalPrefix, DAE::Mod::MOD).clone()) && SCodeUtil::eachEqual(var_field!((*mod1).eachPrefix, DAE::Mod::MOD).clone(), var_field!((*mod2).eachPrefix, DAE::Mod::MOD).clone()) && List::isEqualOnTrue(var_field!((*mod1).subModLst, DAE::Mod::MOD).clone(), var_field!((*mod2).subModLst, DAE::Mod::MOD).clone(), (std::sync::Arc::new(subModEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>, Arc<DAE::SubMod>) -> Result<bool> + 'static>)) && eqModEqual(var_field!((*mod1).binding, DAE::Mod::MOD).clone(), var_field!((*mod2).binding, DAE::Mod::MOD).clone())?,
        (Deref @ DAE::Mod::REDECL { .. }, Deref @ DAE::Mod::REDECL { .. }) => SCodeUtil::finalEqual(var_field!((*mod1).finalPrefix, DAE::Mod::REDECL).clone(), var_field!((*mod2).finalPrefix, DAE::Mod::REDECL).clone()) && SCodeUtil::eachEqual(var_field!((*mod1).eachPrefix, DAE::Mod::REDECL).clone(), var_field!((*mod2).eachPrefix, DAE::Mod::REDECL).clone()) && SCodeUtil::elementEqual(var_field!((*mod1).element, DAE::Mod::REDECL).clone(), var_field!((*mod2).element, DAE::Mod::REDECL).clone())?,
        (Deref @ DAE::Mod::NOMOD, Deref @ DAE::Mod::NOMOD) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

fn subModsEqual(mut inSubModLst1: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inSubModLst2: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (inSubModLst1.clone(), inSubModLst2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id1, r#mod: mod1 }, tail: subModLst1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id2, r#mod: mod2 }, tail: subModLst2 }) => {
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (modEqual(mod1.clone(), mod2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(subModLst1.clone(), subModLst2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

pub fn subModEqual(mut subMod1: Arc<DAE::SubMod>, mut subMod2: Arc<DAE::SubMod>) -> Result<bool> {
    let mut equal: bool = false;
    equal = (::match_deref::match_deref! { match &((subMod1.clone(), subMod2.clone())) {
        (Deref @ DAE::SubMod { ident: id1, r#mod: mod1 }, Deref @ DAE::SubMod { ident: id2, r#mod: mod2 }) if (stringEq((id1.clone()).clone(), (id2.clone()).clone()) && modEqual(mod1.clone(), mod2.clone())?) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

fn valEqual(mut inV1: Option<Arc<Values::Value>>, mut inV2: Option<Arc<Values::Value>>, mut equal: bool) -> Result<bool> {
    let mut bEq: bool = false;
    bEq = (::match_deref::match_deref! { match &((inV1.clone(), inV2.clone(), equal.clone())) {
        (_, _, true) => {
            true
        },
        (None, None, _) => {
            equal.clone()
        },
        (Some(v1), Some(v2), false) => {
            bEq = ExpressionBasics::expEqual(ValuesUtil::valueExp(v1.clone(), None)?, ValuesUtil::valueExp(v2.clone(), None)?)?;
            bEq.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(bEq)
}

fn eqModEqual(mut eqMod1: Option<DAE::EqMod>, mut eqMod2: Option<DAE::EqMod>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (eqMod1.clone(), eqMod2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (None, None) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::TYPED { modifierAsValue: mut v1, modifierAsExp: ref exp1, .. }), Some(DAE::EqMod::TYPED { modifierAsValue: mut v2, modifierAsExp: ref exp2, .. })) = __mc_input.clone() else { bail!("nomatch") };
            let mut equal: bool = equal.clone();
            equal = ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?;
            let true = (valEqual(v1.clone(), v2.clone(), equal.clone())?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::TYPED { modifierAsAbsynExp: ref aexp1, .. }), Some(DAE::EqMod::UNTYPED { exp: ref aexp2 })) = __mc_input.clone() else { bail!("nomatch") };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::UNTYPED { exp: ref aexp1 }), Some(DAE::EqMod::TYPED { modifierAsAbsynExp: ref aexp2, .. })) = __mc_input.clone() else { bail!("nomatch") };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(DAE::EqMod::UNTYPED { exp: ref aexp1 }), Some(DAE::EqMod::UNTYPED { exp: ref aexp2 })) = __mc_input.clone() else { bail!("nomatch") };
            let true = (AbsynUtil::expEqual(aexp1.clone(), aexp2.clone())?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

pub fn printModStr(mut inMod: Arc<DAE::Mod>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD => {
                    Ok(literal!("()"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { eachPrefix, finalPrefix, .. } => {
                    let mut prefix: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    prefix = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*SCodeDump::finalStr(finalPrefix.clone())?); __mm_s.push_str(&*SCodeDump::eachStr(eachPrefix.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = (SCodeDump::unparseElementStr(var_field!((*inMod).element, DAE::Mod::REDECL).clone(), SCodeDump::defaultOptions.clone())?).clone();
                    res = stringAppendList(list![(literal!("(")).clone(), (prefix.clone()).clone(), (r#str.clone()).clone(), (literal!(")")).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { binding: eq, subModLst: subs, eachPrefix, finalPrefix, .. } => {
                    let mut prefix: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut s1_1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    prefix = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*SCodeDump::finalStr(finalPrefix.clone())?); __mm_s.push_str(&*SCodeDump::eachStr(eachPrefix.clone())?); ArcStr::from(__mm_s) }).clone();
                    s1 = printSubs1Str(subs.clone())?;
                    s1_1 = stringDelimitList(s1.clone(), (literal!(", ")).clone());
                    s1_1 = (if (!(subs.clone().is_empty())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" {")); __mm_s.push_str(&*s1_1.clone()); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }} else {s1_1.clone()}).clone();
                    s2 = (printEqmodStr(eq.clone())?).clone();
                    r#str = stringAppendList(list![(prefix.clone()).clone(), (s1_1.clone()).clone(), (s2.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!(" failure in printModStr \n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn printMod(mut m: Arc<DAE::Mod>) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (printModStr(m.clone())?).clone();
    Print::printBuf((r#str.clone()).clone())?;
    Ok(())
}

pub fn prettyPrintMod(mut m: Arc<DAE::Mod>, mut depth: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = (m.clone(), depth.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { binding: None, subModLst: subs, .. }, _) => {
                    Ok(prettyPrintSubs(subs.clone(), depth.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { binding: Some(eq), finalPrefix: fp, .. }, _) => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (SCodeUtil::finalBool(fp.clone())?) {literal!("final ")} else {literal!("")}); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*TypesDump::unparseEqMod(eq.clone())?); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::REDECL { .. }, _) => {
                    Ok(SCodeDump::unparseElementStr(var_field!((*m).element, DAE::Mod::REDECL).clone(), SCodeDump::defaultOptions.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::NOMOD, _) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!(" failed prettyPrintMod\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn prettyPrintSubs(mut inSubs: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut depth: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &((inSubs.clone(), depth.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            literal!("")
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod: Deref @ DAE::Mod::REDECL { .. } }, tail: _ }, _) => {
            let mut s2: ArcStr = arcstr::literal!("");
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" redeclare(")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!("), class or component ")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone();
            s2.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod: m }, tail: _ }, _) => {
            let mut s2: ArcStr = arcstr::literal!("");
            s2 = (prettyPrintMod(m.clone(), depth.clone() + 1)?).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("), class or component ")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone();
            s2.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn prettyPrintSubmod(mut inSub: Arc<DAE::SubMod>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(inSub.clone()) {
        Deref @ DAE::SubMod { ident: id, r#mod: m @ Deref @ DAE::Mod::REDECL { .. } } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (SCodeDump::unparseElementStr(var_field!((**m).element, DAE::Mod::REDECL).clone(), SCodeDump::defaultOptions.clone())?).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!("(redeclare ")); __mm_s.push_str(&*if (SCodeUtil::eachBool(var_field!((**m).eachPrefix, DAE::Mod::REDECL).clone())?) {literal!("each ")} else {literal!("")}); __mm_s.push_str(&*if (SCodeUtil::finalBool(var_field!((**m).finalPrefix, DAE::Mod::REDECL).clone())?) {literal!("final ")} else {literal!("")}); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            s2.clone()
        },
        Deref @ DAE::SubMod { ident: id, r#mod: m } => {
            let mut s2: ArcStr = arcstr::literal!("");
            s2 = (prettyPrintMod(m.clone(), 0)?).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
            s2.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn printSubs1Str(mut inTypesSubModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = (::match_deref::match_deref! { match &(inTypesSubModLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: xs } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            s1 = (printSubStr(x.clone())?).clone();
            res = printSubs1Str(xs.clone())?;
            cons((s1.clone()).clone(), res.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStringLst)
}

fn printSubStr(mut inSubMod: Arc<DAE::SubMod>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { r#mod, ident: n } => {
            let mut mod_str: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            mod_str = (printModStr(r#mod.clone())?).clone();
            res = (stringAppend(({ let mut __mm_s = String::new(); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone(), (mod_str.clone()).clone())).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn printEqmodStr(mut inTypesEqModOption: Option<DAE::EqMod>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inTypesEqModOption.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: Some(e_val), properties: prop, modifierAsAbsynExp: _, .. }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut e_val_str: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    str2 = (Types::printPropStr(prop.clone())?).clone();
                    e_val_str = (ValuesDump::valString(e_val.clone())?).clone();
                    res = stringAppendList(list![(literal!(" = (typed)")).clone(), (r#str.clone()).clone(), (literal!(" ")).clone(), (str2.clone()).clone(), (literal!(", value: ")).clone(), (e_val_str.clone()).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: None, properties: prop, modifierAsAbsynExp: _, .. }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    str2 = (Types::printPropStr(prop.clone())?).clone();
                    res = stringAppendList(list![(literal!(" = (typed)")).clone(), (r#str.clone()).clone(), (literal!(", type:\n")).clone(), (str2.clone()).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(DAE::EqMod::UNTYPED { exp: ae }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    r#str = (Dump::printExpStr(ae.clone())?).clone();
                    res = (stringAppend((literal!(" =(untyped) ")).clone(), (r#str.clone()).clone())).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: ArcStr = arcstr::literal!("");
                    res = (literal!("---Mod.printEqmodStr FAILED---")).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn renameTopLevelNamedSubMod(mut r#mod: Arc<DAE::Mod>, mut oldIdent: ArcStr, mut newIdent: ArcStr) -> Arc<DAE::Mod> {
    let mut outMod: Arc<DAE::Mod> = r#mod.clone();
    outMod = (::match_deref::match_deref! { match &(outMod.clone()) {
        Deref @ DAE::Mod::MOD { .. } => {
            assign_variant_field!(outMod => DAE::Mod::MOD; subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
        for mut s in (var_field!((*outMod).subModLst, DAE::Mod::MOD).clone()).into_iter().cloned() {
            let __x = renameNamedSubMod(s.clone(), (oldIdent.clone()).clone(), (newIdent.clone()).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outMod.clone()
        },
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn renameNamedSubMod(mut submod: Arc<DAE::SubMod>, mut oldIdent: ArcStr, mut newIdent: ArcStr) -> Arc<DAE::SubMod> {
    let mut outMod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
    outMod = (::match_deref::match_deref! { match &((submod.clone(), oldIdent.clone(), newIdent.clone())) {
        (Deref @ DAE::SubMod { ident: id, r#mod }, _, _) if (stringEq((id.clone()).clone(), (oldIdent.clone()).clone())) => {
            Arc::new(DAE::SubMod { ident: (newIdent.clone()).clone(), r#mod: r#mod.clone() })
        },
        _ => {
            submod.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn emptyModOrEquality(mut r#mod: Arc<DAE::Mod>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ DAE::Mod::NOMOD => true,
        Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn intStringDot(mut i: i32) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn isPrefixOf(mut indexSubMod: (ArcStr, Arc<DAE::SubMod>), mut idx: ArcStr) -> Result<bool> {
    let mut isPrefix: bool = false;
    isPrefix = 'mc: {
        let __mc_input = (indexSubMod.clone(), idx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((i, _), _) => {
                    let mut len1: i32 = 0;
                    let mut len2: i32 = 0;
                    len1 = ((i.clone()).clone().len() as i32);
                    len2 = ((idx.clone()).clone().len() as i32);
                    let true = (0 == System::strncmp((i.clone()).clone(), (idx.clone()).clone(), intMin(len1.clone(), len2.clone()))) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isPrefix)
}

fn getFullModsFromMod(mut inTopCref: Arc<DAE::ComponentRef>, mut inMod: Arc<DAE::Mod>) -> Result<Arc<metamodelica::List<FullMod>>> {
    let mut outFullMods: Arc<metamodelica::List<FullMod>> = metamodelica::nil();
    outFullMods = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::NOMOD => metamodelica::nil(),
        Deref @ DAE::Mod::MOD { .. } => getFullModsFromSubMods(inTopCref.clone(), var_field!((*inMod).subModLst, DAE::Mod::MOD).clone())?,
        Deref @ DAE::Mod::REDECL { .. } => list![getFullModFromModRedeclare(inTopCref.clone(), inMod.clone())?],
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFullMods)
}

fn getFullModFromModRedeclare(mut inTopCref: Arc<DAE::ComponentRef>, mut inRedeclare: Arc<DAE::Mod>) -> Result<FullMod> {
    let mut outFullMod: FullMod;
    let mut el: Arc<SCode::Element>;
    let mut id: ArcStr = arcstr::literal!("");
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let __pa0 = ::match_deref::match_deref! { match &(inRedeclare.clone()) {
        Deref @ DAE::Mod::REDECL { element: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    el = __pa0.clone();
    id = (SCodeUtil::elementName(el.clone())?).clone();
    cref = ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
    cref = ComponentReference::joinCrefs(inTopCref.clone(), cref.clone())?;
    outFullMod = FullMod::MOD { cref: cref.clone(), r#mod: inRedeclare.clone() };
    Ok(outFullMod)
}

fn getFullModsFromSubMods(mut inTopCref: Arc<DAE::ComponentRef>, mut inSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<Arc<metamodelica::List<FullMod>>> {
    let mut outFullMods: Arc<metamodelica::List<FullMod>> = metamodelica::nil();
    outFullMods = (::match_deref::match_deref! { match &((inTopCref.clone(), inSubMods.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (_, Deref @ metamodelica::List::Cons { head: subMod @ Deref @ DAE::SubMod { ident: id, r#mod }, tail: rest }) => {
            let mut fullMods1: Arc<metamodelica::List<FullMod>> = metamodelica::nil();
            let mut fullMods2: Arc<metamodelica::List<FullMod>> = metamodelica::nil();
            let mut fullMods: Arc<metamodelica::List<FullMod>> = metamodelica::nil();
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = ComponentReference::joinCrefs(inTopCref.clone(), ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
            fullMods1 = getFullModsFromMod(cref.clone(), r#mod.clone())?;
            fullMods2 = getFullModsFromSubMods(inTopCref.clone(), rest.clone())?;
            fullMods = listAppend(if (fullMods1.clone().is_empty()) {cons(FullMod::SUB_MOD { cref: cref.clone(), subMod: subMod.clone() }, fullMods1.clone())} else {fullMods1.clone()}, fullMods2.clone());
            fullMods.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFullMods)
}

fn fullModCrefsEqual(mut inFullMod1: FullMod, mut inFullMod2: FullMod) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = (match (inFullMod1.clone(), inFullMod2.clone()) {
        (FullMod::MOD { cref: ref cr1, r#mod: _ }, FullMod::MOD { cref: ref cr2, r#mod: _ }) => {
            ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?
        },
        (FullMod::SUB_MOD { cref: ref cr1, subMod: _ }, FullMod::SUB_MOD { cref: ref cr2, subMod: _ }) => {
            ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?
        },
        (FullMod::MOD { cref: ref cr1, r#mod: _ }, FullMod::SUB_MOD { cref: ref cr2, subMod: _ }) => {
            ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?
        },
        (FullMod::SUB_MOD { cref: ref cr1, subMod: _ }, FullMod::MOD { cref: ref cr2, r#mod: _ }) => {
            ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(isEqual)
}

fn prettyPrintFullMod(mut inFullMod: FullMod, mut inDepth: i32) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ((match (inFullMod.clone(), inDepth.clone()) {
        (FullMod::MOD { cref: ref cr, r#mod: mut r#mod }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*prettyPrintMod(r#mod.clone(), inDepth.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (FullMod::SUB_MOD { cref: ref cr, subMod: mut subMod }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*prettyPrintSubmod(subMod.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outStr)
}

pub fn getUnelabedSubMod(mut inMod: Arc<SCode::Mod>, mut inIdent: ArcStr) -> Result<Arc<SCode::Mod>> {
    let mut outSubMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { subModLst: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    submods = __pa0.clone();
    outSubMod = getUnelabedSubMod2(submods.clone(), (inIdent.clone()).clone())?;
    Ok(outSubMod)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getUnelabedSubMod2(mut inSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inIdent: ArcStr) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inSubMods.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { r#mod: m, ident: id }, tail: _ }, _) => {
                    let true = (stringEqual((id.clone()).clone(), (inIdent.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest_mods }, _) => {
                    Ok(getUnelabedSubMod2(rest_mods.clone(), (inIdent.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

pub fn isUntypedMod(mut inMod: Arc<DAE::Mod>) -> Result<bool> {
    let mut outIsUntyped: bool = false;
    outIsUntyped = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::UNTYPED { .. }), .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { subModLst: submods, .. } => {
                    let _ = List::find(submods.clone(), (std::sync::Arc::new(isUntypedSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>) -> Result<bool> + 'static>))?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIsUntyped)
}

fn isUntypedSubMod(mut inSubMod: Arc<DAE::SubMod>) -> Result<bool> {
    let mut outIsUntyped: bool = false;
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let __pa0 = ::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { r#mod: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#mod = __pa0.clone();
    outIsUntyped = isUntypedMod(r#mod.clone())?;
    Ok(outIsUntyped)
}

pub fn getUntypedCrefs(mut inMod: Arc<DAE::Mod>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outCrefs = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::UNTYPED { exp }), .. } => {
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    crefs = AbsynUtil::getCrefFromExp(exp.clone(), true, true)?;
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { subModLst: submods, .. } => {
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    crefs = List::fold(submods.clone(), (std::sync::Arc::new(getUntypedCrefFromSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> + 'static>), metamodelica::nil());
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefs)
}

fn getUntypedCrefFromSubMod(mut inSubMod: Arc<DAE::SubMod>, mut inCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outCrefs = (::match_deref::match_deref! { match &((inSubMod.clone(), inCrefs.clone())) {
        (Deref @ DAE::SubMod { r#mod, .. }, _) => {
            let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            crefs = getUntypedCrefs(r#mod.clone())?;
            listAppend(crefs.clone(), inCrefs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCrefs)
}

// moved from Types!
pub fn stripSubmod(mut inMod: Arc<DAE::Mod>) -> Arc<DAE::Mod> {
    let mut outMod: Arc<DAE::Mod> = inMod.clone();
    outMod = (::match_deref::match_deref! { match &(outMod.clone()) {
        Deref @ DAE::Mod::MOD { .. } => {
            assign_variant_field!(outMod => DAE::Mod::MOD; subModLst = metamodelica::nil());
            outMod.clone()
        },
        _ => outMod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn removeFirstSubsRedecl(mut inMod: Arc<DAE::Mod>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { info, binding: eq, subModLst: Deref @ metamodelica::List::Nil, eachPrefix: each_, finalPrefix: f } => {
                    Ok(Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: metamodelica::nil(), binding: eq.clone(), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { binding: None, subModLst: subs, .. } => {
                    ::match_deref::match_deref! { match &(removeRedecl(subs.clone())) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { info, binding: eq, subModLst: subs, eachPrefix: each_, finalPrefix: f } => {
                    let mut subs = (*subs).clone();
                    subs = removeRedecl(subs.clone());
                    Ok(Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: each_.clone(), subModLst: subs.clone(), binding: eq.clone(), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                m => {
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn removeRedecl(mut isubs: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Arc<metamodelica::List<Arc<DAE::SubMod>>> {
    let mut osubs: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    osubs = (::match_deref::match_deref! { match &(isubs.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: _, r#mod: Deref @ DAE::Mod::REDECL { .. } }, tail: subs } => {
            removeRedecl(subs.clone())
        },
        Deref @ metamodelica::List::Cons { head: sm, tail: subs } => {
            osubs = removeRedecl(subs.clone());
            cons(sm.clone(), osubs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    osubs
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn removeModList(mut inMod: Arc<DAE::Mod>, mut remStrings: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut s: ArcStr = arcstr::literal!("");
    outMod = (::match_deref::match_deref! { match &((inMod.clone(), remStrings.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            inMod.clone()
        },
        (_, Deref @ metamodelica::List::Cons { head: s, tail: _ }) => {
            removeModList(removeMod(inMod.clone(), (s.clone()).clone())?, remStrings.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

pub fn removeMod(mut inMod: Arc<DAE::Mod>, mut componentModified: ArcStr) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::NOMOD => {
            Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)
        },
        Deref @ DAE::Mod::REDECL { .. } => {
            if (SCodeUtil::elementName(var_field!((*inMod).element, DAE::Mod::REDECL).clone())? == componentModified.clone()) {Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)} else {inMod.clone()}
        },
        Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: subs, binding: oem, info } => {
            let mut subs = (*subs).clone();
            subs = removeModInSubs(subs.clone(), (componentModified.clone()).clone())?;
            outMod = Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: subs.clone(), binding: oem.clone(), info: info.clone() });
            outMod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn removeModInSubs(mut inSubs: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut componentName: ArcStr) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut outsubs: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    outsubs = (::match_deref::match_deref! { match &((inSubs.clone(), componentName.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: s1, r#mod: m1 }, tail: subs }, _) => {
            let mut subs1: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            let mut subs2: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            subs1 = if (stringEq((s1.clone()).clone(), (componentName.clone()).clone())) {metamodelica::nil()} else {list![Arc::new(DAE::SubMod { ident: (s1.clone()).clone(), r#mod: m1.clone() })]};
            subs2 = removeModInSubs(subs.clone(), (componentName.clone()).clone())?;
            outsubs = listAppend(subs1.clone(), subs2.clone());
            outsubs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outsubs)
}

pub fn addEachIfNeeded(mut inMod: Arc<DAE::Mod>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inMod.clone(), inDimensions.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(inMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::NOMOD, _) => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::REDECL { finalPrefix, eachPrefix: _, element: el, r#mod }, _) => {
                    Ok(Arc::new(DAE::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, element: el.clone(), r#mod: r#mod.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix: SCode::Each::EACH, subModLst: subs, binding: eq, info }, _) => {
                    Ok(Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, subModLst: subs.clone(), binding: eq.clone(), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix, subModLst: subs, binding: eq, info }, _) => {
                    let mut subs = (*subs).clone();
                    subs = addEachToSubsIfNeeded(subs.clone(), inDimensions.clone())?;
                    Ok(Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: eachPrefix.clone(), subModLst: subs.clone(), binding: eq.clone(), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Mod.addEachIfNeeded failed on: ")); __mm_s.push_str(&*printModStr(inMod.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

pub fn addEachOneLevel(mut inMod: Arc<DAE::Mod>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { finalPrefix, eachPrefix: _, element: el, r#mod } => {
                    Ok(Arc::new(DAE::Mod::REDECL { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, element: el.clone(), r#mod: r#mod.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix: _, subModLst: subs, binding: eq, info } => {
                    Ok(Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, subModLst: subs.clone(), binding: eq.clone(), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Mod.addEachOneLevel failed on: ")); __mm_s.push_str(&*printModStr(inMod.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

pub fn addEachToSubsIfNeeded(mut inSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    outSubMods = (::match_deref::match_deref! { match &((inSubMods.clone(), inDimensions.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            inSubMods.clone()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod: m }, tail: rest }, _) => {
            let mut m = (*m).clone();
            let mut rest = (*rest).clone();
            m = addEachOneLevel(m.clone())?;
            rest = addEachToSubsIfNeeded(rest.clone(), inDimensions.clone())?;
            cons(Arc::new(DAE::SubMod { ident: (id.clone()).clone(), r#mod: m.clone() }), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubMods)
}

pub fn isEmptyMod(mut inMod: Arc<DAE::Mod>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::NOMOD => true,
        Deref @ DAE::Mod::MOD { binding: None, subModLst: Deref @ metamodelica::List::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn isNoMod(mut inMod: Arc<DAE::Mod>) -> bool {
    let mut outIsNoMod: bool = false;
    outIsNoMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::NOMOD => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNoMod
}

pub fn getModInfo(mut inMod: Arc<DAE::Mod>) -> SourceInfo {
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outInfo = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::MOD { .. } => {
            var_field!((*inMod).info, DAE::Mod::MOD).clone()
        },
        Deref @ DAE::Mod::REDECL { .. } => {
            SCodeUtil::elementInfo(var_field!((*inMod).element, DAE::Mod::REDECL).clone())
        },
        _ => {
            Absyn::dummyInfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInfo
}

pub fn isRedeclareMod(mut inMod: Arc<DAE::Mod>) -> bool {
    let mut yes: bool = false;
    yes = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::REDECL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    yes
}

pub fn getClassModifier(mut inEnv: FCore::Graph, mut inName: ArcStr) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut n: FCore::Node = <FCore::Node as ::std::default::Default>::default();
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inEnv.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut n: FCore::Node = n.clone();
            let mut r#mod: Arc<DAE::Mod> = r#mod.clone();
            n = FNode::fromRef(FNode::child(FGraph::lastScopeRef(inEnv.clone())?, (inName.clone()).clone())?)?;
            if !(FNode::isInstance(FNode::fromRef(FGraph::lastScopeRef(inEnv.clone())?)?)) {
                let FCore::N { data: FCore::CL { r#mod: __pa0, .. }, .. } = (n.clone()) else { bail!("pattern mismatch") };
                r#mod = __pa0.clone();
                r#mod = removeMod(r#mod.clone(), (inName.clone()).clone())?;
            } else {
                r#mod = Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD);
            }
            Ok(r#mod.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn subModValue(mut inSubMod: Arc<DAE::SubMod>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let __pa0 = ::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsValue: Some(__pa0), .. }), .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    Ok(outValue)
}

fn subModName(mut inSubMod: Arc<DAE::SubMod>) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { ident: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    Ok(outName)
}

fn subModIsNamed(mut inName: ArcStr, mut inSubMod: Arc<DAE::SubMod>) -> Result<bool> {
    let mut outNameEq: bool = false;
    outNameEq = inName.clone() == subModName(inSubMod.clone())?;
    Ok(outNameEq)
}

fn subModInfo(mut inSubMod: Arc<DAE::SubMod>) -> Result<SourceInfo> {
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let __pa0 = ::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { r#mod: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#mod = __pa0.clone();
    outInfo = getModInfo(r#mod.clone());
    Ok(outInfo)
}

fn setEqMod(mut inEqMod: Option<DAE::EqMod>, mut inMod: Arc<DAE::Mod>) -> Arc<DAE::Mod> {
    let mut outMod: Arc<DAE::Mod> = inMod.clone();
    outMod = (::match_deref::match_deref! { match &(outMod.clone()) {
        Deref @ DAE::Mod::MOD { .. } => {
            assign_variant_field!(outMod => DAE::Mod::MOD; binding = inEqMod.clone());
            outMod.clone()
        },
        _ => outMod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

fn stripSubModBindings(mut inSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut id: ArcStr = arcstr::literal!("");
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    for mut submod in &*inSubMods.clone() {
        let mut submod = submod.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(submod.clone()) {
            Deref @ DAE::SubMod { ident: __pa0, r#mod: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        id = __pa0.clone();
        r#mod = __pa1.clone();
        r#mod = setEqMod(None, r#mod.clone());
        if !(isEmptyMod(r#mod.clone())) {
            outSubMods = cons(Arc::new(DAE::SubMod { ident: (id.clone()).clone(), r#mod: r#mod.clone() }), outSubMods.clone());
        }
    }
    outSubMods = outSubMods.clone().reverse();
    Ok(outSubMods)
}

pub fn filterRedeclares(mut inMod: Arc<DAE::Mod>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = inMod.clone();
    outMod = (::match_deref::match_deref! { match &(outMod.clone()) {
        Deref @ DAE::Mod::MOD { .. } => {
            assign_variant_field!(outMod => DAE::Mod::MOD;
                subModLst = filterRedeclaresSubMods(var_field!((*outMod).subModLst, DAE::Mod::MOD).clone())?,
                binding = None
            );
            if (var_field!((*outMod).subModLst, DAE::Mod::MOD).clone().is_empty()) {Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)} else {outMod.clone()}
        },
        _ => outMod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn filterRedeclaresSubMods(mut inSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut id: ArcStr = arcstr::literal!("");
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    for mut submod in &*inSubMods.clone() {
        let mut submod = submod.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(submod.clone()) {
            Deref @ DAE::SubMod { ident: __pa0, r#mod: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        id = __pa0.clone();
        r#mod = __pa1.clone();
        r#mod = filterRedeclares(r#mod.clone())?;
        if isRedeclareMod(r#mod.clone()) {
            outSubMods = cons(Arc::new(DAE::SubMod { ident: (id.clone()).clone(), r#mod: r#mod.clone() }), outSubMods.clone());
        }
    }
    outSubMods = outSubMods.clone().reverse();
    Ok(outSubMods)
}

pub fn unparseModStr(mut inMod: Arc<DAE::Mod>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::NOMOD => {
            literal!("")
        },
        Deref @ DAE::Mod::MOD { .. } => {
            let mut final_str: ArcStr = arcstr::literal!("");
            let mut each_str: ArcStr = arcstr::literal!("");
            let mut sub_str: ArcStr = arcstr::literal!("");
            let mut binding_str: ArcStr = arcstr::literal!("");
            final_str = (if (SCodeUtil::finalBool(var_field!((*inMod).finalPrefix, DAE::Mod::MOD).clone())?) {literal!("final ")} else {literal!("")}).clone();
            each_str = (if (SCodeUtil::eachBool(var_field!((*inMod).eachPrefix, DAE::Mod::MOD).clone())?) {literal!("each ")} else {literal!("")}).clone();
            sub_str = (List::toString(var_field!((*inMod).subModLst, DAE::Mod::MOD).clone(), (std::sync::Arc::new(unparseSubModStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0)?).clone();
            binding_str = (unparseBindingStr(var_field!((*inMod).binding, DAE::Mod::MOD).clone())?).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*final_str.clone()); __mm_s.push_str(&*each_str.clone()); __mm_s.push_str(&*sub_str.clone()); __mm_s.push_str(&*binding_str.clone()); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::Mod::REDECL { .. } => {
            let mut final_str: ArcStr = arcstr::literal!("");
            let mut each_str: ArcStr = arcstr::literal!("");
            let mut el_str: ArcStr = arcstr::literal!("");
            final_str = (if (SCodeUtil::finalBool(var_field!((*inMod).finalPrefix, DAE::Mod::REDECL).clone())?) {literal!("final ")} else {literal!("")}).clone();
            each_str = (if (SCodeUtil::eachBool(var_field!((*inMod).eachPrefix, DAE::Mod::REDECL).clone())?) {literal!("each ")} else {literal!("")}).clone();
            el_str = (SCodeDump::unparseElementStr(var_field!((*inMod).element, DAE::Mod::REDECL).clone(), SCodeDump::defaultOptions.clone())?).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*final_str.clone()); __mm_s.push_str(&*each_str.clone()); __mm_s.push_str(&*literal!("redeclare ")); __mm_s.push_str(&*el_str.clone()); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn unparseSubModStr(mut inSubMod: Arc<DAE::SubMod>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*inSubMod.ident.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*unparseModStr(inSubMod.r#mod.clone())?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn unparseBindingStr(mut inBinding: Option<DAE::EqMod>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inBinding.clone() {
        None => {
            literal!("")
        },
        Some(DAE::EqMod::TYPED { modifierAsAbsynExp: ref exp, .. }) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); ArcStr::from(__mm_s) }
        },
        Some(DAE::EqMod::UNTYPED { exp: mut exp }) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

