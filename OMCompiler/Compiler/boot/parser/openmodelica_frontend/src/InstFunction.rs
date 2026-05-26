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

use crate::ComponentReference;
use crate::ConnectionGraph;
use crate::DAEUtil;
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use crate::InnerOuter;
use crate::Inst;
use crate::InstTypes;
use crate::InstUtil;
use crate::Lookup;
use crate::Mod;
use crate::PrefixUtil;
use crate::Types;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::InstBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

pub type InstDims = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>;

pub fn instantiateExternalObject(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut els: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inMod: Arc<DAE::Mod>, mut r#impl: bool, mut comment: Arc<SCode::Comment>, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, ClassInf::State)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut dae: DAE::DAElist;
    let mut ciState: ClassInf::State;
    (outCache, outEnv, outIH, dae, ciState) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), els.clone(), inMod.clone(), r#impl.clone(), comment.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, false, _, _) => {
                    let mut destr: Arc<SCode::Element>;
                    let mut constr: Arc<SCode::Element>;
                    let mut className: Ident = arcstr::literal!("");
                    let mut classNameFQ: Arc<Absyn::Path>;
                    let mut functp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    className = (FNode::refName(FGraph::lastScopeRef(env.clone())?)?).clone();
                    checkExternalObjectMod(inMod.clone(), (className.clone()).clone())?;
                    destr = SCodeUtil::getExternalObjectDestructor(els.clone())?;
                    constr = SCodeUtil::getExternalObjectConstructor(els.clone())?;
                    env = FGraph::mkClassNode(env.clone(), destr.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, inMod.clone(), false)?;
                    env = FGraph::mkClassNode(env.clone(), constr.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, inMod.clone(), false)?;
                    (cache, ih) = instantiateExternalObjectDestructor(cache.clone(), env.clone(), ih.clone(), destr.clone())?;
                    (cache, ih, functp) = instantiateExternalObjectConstructor(cache.clone(), env.clone(), ih.clone(), constr.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(FGraph::getScopePath(env.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    classNameFQ = __pa0.clone();
                    (env, r) = FGraph::stripLastScopeRef(env.clone())?;
                    env = FGraph::mkTypeNode(env.clone(), (className.clone()).clone(), functp.clone())?;
                    env = FGraph::pushScopeRef(env.clone(), r.clone())?;
                    source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource.clone(), FGraph::getScopePath(env.clone())?)?;
                    source = ElementSource::addCommentToSource(source.clone(), Some(comment.clone()));
                    source = ElementSource::addElementSourceFileInfo(source.clone(), info.clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::DAElist { elementLst: list![Arc::new(DAE::Element::EXTOBJECTCLASS { path: classNameFQ.clone(), source: source.clone() })] }, ClassInf::State::EXTERNAL_OBJ { path: classNameFQ.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ih, _, _, true, _, _) => {
                    let mut classNameFQ: Arc<Absyn::Path>;
                    let __pa0 = ::match_deref::match_deref! { match &(FGraph::getScopePath(inEnv.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    classNameFQ = __pa0.clone();
                    Ok((cache.clone(), inEnv.clone(), ih.clone(), DAE::emptyDae.clone(), ClassInf::State::EXTERNAL_OBJ { path: classNameFQ.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstFunction.instantiateExternalObject failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, dae, ciState))
}

fn checkExternalObjectMod(mut inMod: Arc<DAE::Mod>, mut inClassName: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inMod.clone(), inClassName.clone())) {
        (Deref @ DAE::Mod::NOMOD, _) => {
            ()
        },
        (Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, .. }, _) => {
            ()
        },
        (Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { r#mod, ident: id }, tail: _ }, .. }, _) => {
            let mut info: SourceInfo;
            info = Mod::getModInfo(r#mod.clone());
            Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(id.clone()).clone(), (inClassName.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn instantiateExternalObjectDestructor(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut cl: Arc<SCode::Element>) -> Result<(FCore::Cache, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    (outCache, outIH) = 'mc: {
        let __mc_input = (inCache.clone(), env.clone(), inIH.clone(), cl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ih, _) => {
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, _, ih) = implicitFunctionInstantiation(cache.clone(), env.clone(), ih.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil())?;
                    Ok((cache.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstFunction.instantiateExternalObjectDestructor failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outIH))
}

fn instantiateExternalObjectConstructor(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut cl: Arc<SCode::Element>) -> Result<(FCore::Cache, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outCache, outIH, outType) = 'mc: {
        let __mc_input = (inCache.clone(), env.clone(), inIH.clone(), cl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ih, _) => {
                    let mut env1: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env1, ih) = implicitFunctionInstantiation(cache.clone(), env.clone(), ih.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil())?;
                    (cache, ty, _) = Lookup::lookupType(cache.clone(), env1.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("constructor")).clone() }), None)?;
                    Ok((cache.clone(), ih.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstFunction.instantiateExternalObjectConstructor failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outIH, outType))
}

pub fn implicitFunctionInstantiation(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inClass: Arc<SCode::Element>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    (outCache, outEnv, outIH) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone())) {
        (cache, env, ih, r#mod, pre, c @ Deref @ SCode::Element::CLASS { partialPrefix: pPrefix, restriction: SCode::Restriction::R_RECORD { isOperator: _ }, name: n, .. }, inst_dims) => {
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cenv: FCore::Graph;
            let mut fpath: Arc<Absyn::Path>;
            let mut source: Arc<DAE::ElementSource>;
            let mut fun: DAE::Function;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut ih = (*ih).clone();
            let mut c = (*c).clone();
            (cache, c, cenv) = Lookup::lookupRecordConstructorClass(cache.clone(), env.clone(), Arc::new(Absyn::Path::IDENT { name: (n.clone()).clone() }))?;
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(implicitFunctionInstantiation2(cache.clone(), cenv.clone(), ih.clone(), r#mod.clone(), pre.clone(), c.clone(), inst_dims.clone(), true)?) {
                (__pa0, __pa1, __pa2, Deref @ metamodelica::List::Cons { head: DAE::Function::FUNCTION { source: __pa3, type_: __pa4, path: __pa5, .. }, tail: Deref @ metamodelica::List::Nil }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            env = __pa1.clone();
            ih = __pa2.clone();
            source = __pa3.clone();
            ty1 = __pa4.clone();
            fpath = __pa5.clone();
            fun = DAE::Function::RECORD_CONSTRUCTOR { path: fpath.clone(), type_: ty1.clone(), source: source.clone() };
            cache = InstUtil::addFunctionsToDAE(cache.clone(), list![fun.clone()], pPrefix.clone())?;
            (cache.clone(), env.clone(), ih.clone())
        },
        (cache, env, ih, r#mod, pre, c @ Deref @ SCode::Element::CLASS { partialPrefix: pPrefix, restriction: r, .. }, inst_dims) => {
            let mut funs: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut ih = (*ih).clone();
            if '__try0: {
                let SCode::R_RECORD { isOperator: _ } = (r.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            (cache, env, ih, funs) = implicitFunctionInstantiation2(cache.clone(), env.clone(), ih.clone(), r#mod.clone(), pre.clone(), c.clone(), inst_dims.clone(), false)?;
            cache = InstUtil::addFunctionsToDAE(cache.clone(), funs.clone(), pPrefix.clone())?;
            (cache.clone(), env.clone(), ih.clone())
        },
        (_, env, _, _, _, Deref @ SCode::Element::CLASS { name: n, .. }, _) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.implicitFunctionInstantiation failed ")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone())?;
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH))
}

fn implicitFunctionInstantiation2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inClass: Arc<SCode::Element>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut instFunctionTypeOnly: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<metamodelica::List<DAE::Function>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut funcs: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
    (outCache, outEnv, outIH, funcs) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone(), instFunctionTypeOnly.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, r#mod, pre, Deref @ SCode::Element::CLASS { info, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: funcRest }, name: n, partialPrefix, prefixes: Deref @ SCode::Prefixes { visibility, .. }, classDef: cd, .. }, inst_dims, _) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_1: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut fpath: Arc<Absyn::Path>;
                    let mut c: Arc<SCode::Element>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut daeElts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut derFuncs: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
                    let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
                    let mut partialPrefixBool: bool = false;
                    let mut isImpure: bool = false;
                    let mut cmt: Arc<SCode::Comment>;
                    let mut cs: InstTypes::CallingScope = InstTypes::CallingScope::INNER_CALL;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let false = (SCodeUtil::isExternalFunctionRestriction(funcRest.clone())) else { bail!("pattern mismatch") };
                    isImpure = SCodeUtil::isImpureFunctionRestriction(funcRest.clone());
                    c = if (Config::acceptMetaModelicaGrammar()?) {inClass.clone()} else {SCodeUtil::setClassPartialPrefix(openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, inClass.clone())?};
                    cs = if (instFunctionTypeOnly.clone()) {crate::InstTypes::CallingScope::TYPE_CALL} else {crate::InstTypes::CallingScope::INNER_CALL};
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Inst::instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), r#mod.clone(), pre.clone(), c.clone(), inst_dims.clone(), true, cs.clone(), ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?) {
                        (__pa0, __pa1, __pa2, _, DAE::DAElist { elementLst: __pa3 }, _, __pa4, _, _, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    cenv = __pa1.clone();
                    ih = __pa2.clone();
                    daeElts = __pa3.clone();
                    ty = __pa4.clone();
                    List::map2_0(daeElts.clone(), Arc::new(InstUtil::checkFunctionElement), false, info.clone());
                    env_1 = env.clone();
                    (cache, fpath) = Inst::makeFullyQualifiedIdent(cache.clone(), env_1.clone(), (n.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    cmt = InstUtil::extractComment(daeElts.clone())?;
                    derFuncs = InstUtil::getDeriveAnnotation(cd.clone(), cmt.clone(), fpath.clone(), cache.clone(), cenv.clone(), ih.clone(), pre.clone(), info.clone())?;
                    cache = instantiateDerivativeFuncs(cache.clone(), env.clone(), ih.clone(), derFuncs.clone(), fpath.clone(), info.clone())?;
                    ty1 = InstUtil::setFullyQualifiedTypename(ty.clone(), fpath.clone());
                    checkExtObjOutput(ty1.clone(), info.clone())?;
                    env_1 = FGraph::mkTypeNode(env_1.clone(), (n.clone()).clone(), ty1.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    inlineType = InstBasics::commentIsInlineFunc(cmt.clone())?;
                    partialPrefixBool = SCodeUtil::partialBool(partialPrefix.clone())?;
                    daeElts = InstUtil::optimizeFunctionCheckForLocals(fpath.clone(), daeElts.clone(), None, metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                    InstUtil::checkFunctionDefUse(daeElts.clone(), info.clone())?;
                    if false && Config::acceptMetaModelicaGrammar()? && !(instFunctionTypeOnly.clone()) {
                        InstUtil::checkFunctionInputUsed(daeElts.clone(), None, (AbsynUtil::pathString(fpath.clone(), (literal!(".")).clone(), true, false)?).clone())?;
                    }
                    Ok((cache.clone(), env_1.clone(), ih.clone(), list![DAE::Function::FUNCTION { path: fpath.clone(), functions: cons(DAE::FunctionDefinition::FUNCTION_DEF { body: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut e in (daeElts.clone()).into_iter().cloned() {
            if !(!(DAEUtil::isComment(e.clone()))) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } }, derFuncs.clone()), type_: ty1.clone(), visibility: visibility.clone(), partialPrefix: partialPrefixBool.clone(), isImpure: isImpure.clone(), inlineType: inlineType.clone(), unusedInputs: metamodelica::nil(), source: source.clone(), comment: Some(cmt.clone()) }]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, r#mod, pre, c @ Deref @ SCode::Element::CLASS { encapsulatedPrefix, info, classDef: cd @ parts @ Deref @ SCode::ClassDef::PARTS { externalDecl: Some(scExtdecl), .. }, restriction: restr @ SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity } }, name: n, prefixes: Deref @ SCode::Prefixes { visibility, .. }, partialPrefix, .. }, inst_dims, _) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_1: FCore::Graph;
                    let mut tempenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut fpath: Arc<Absyn::Path>;
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut extdecl: DAE::ExternalDecl;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut daeElts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut derFuncs: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
                    let mut partialPrefixBool: bool = false;
                    let mut isImpure: bool = false;
                    let mut cmt: Arc<SCode::Comment>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Inst::instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), r#mod.clone(), pre.clone(), c.clone(), inst_dims.clone(), true, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?) {
                        (__pa0, __pa1, __pa2, _, DAE::DAElist { elementLst: __pa3 }, _, __pa4, _, _, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    cenv = __pa1.clone();
                    ih = __pa2.clone();
                    daeElts = __pa3.clone();
                    ty = __pa4.clone();
                    List::map2_0(daeElts.clone(), Arc::new(InstUtil::checkFunctionElement), true, info.clone());
                    (cache, fpath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), (n.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    cmt = InstUtil::extractComment(daeElts.clone())?;
                    derFuncs = InstUtil::getDeriveAnnotation(cd.clone(), cmt.clone(), fpath.clone(), cache.clone(), env.clone(), ih.clone(), pre.clone(), info.clone())?;
                    cache = instantiateDerivativeFuncs(cache.clone(), env.clone(), ih.clone(), derFuncs.clone(), fpath.clone(), info.clone())?;
                    ty1 = InstUtil::setFullyQualifiedTypename(ty.clone(), fpath.clone());
                    checkExtObjOutput(ty1.clone(), info.clone())?;
                    env_1 = FGraph::mkTypeNode(cenv.clone(), (n.clone()).clone(), ty1.clone())?;
                    vis = openmodelica_frontend_types::SCode::Visibility::PUBLIC;
                    isImpure = AbsynUtil::isImpure(purity.clone(), false);
                    (cache, tempenv, ih, _, _, _, _, _, _, _, _, _) = Inst::instClassdef(cache.clone(), env_1.clone(), ih.clone(), UnitAbsyn::noStore().clone(), r#mod.clone(), pre.clone(), ClassInf::State::FUNCTION { path: fpath.clone(), isImpure: isImpure.clone() }, (n.clone()).clone(), parts.clone(), restr.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), true, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone(), None, cmt.clone(), info.clone())?;
                    (cache, ih, extdecl) = instExtDecl(cache.clone(), tempenv.clone(), ih.clone(), (n.clone()).clone(), scExtdecl.clone(), daeElts.clone(), ty1.clone(), true, pre.clone(), info.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    partialPrefixBool = SCodeUtil::partialBool(partialPrefix.clone())?;
                    InstUtil::checkExternalFunction(daeElts.clone(), extdecl.clone(), (AbsynUtil::pathString(fpath.clone(), (literal!(".")).clone(), true, false)?).clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), list![DAE::Function::FUNCTION { path: fpath.clone(), functions: cons(DAE::FunctionDefinition::FUNCTION_EXT { body: daeElts.clone(), externalDecl: extdecl.clone() }, derFuncs.clone()), type_: ty1.clone(), visibility: visibility.clone(), partialPrefix: partialPrefixBool.clone(), isImpure: isImpure.clone(), inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, unusedInputs: metamodelica::nil(), source: source.clone(), comment: Some(cmt.clone()) }]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, pre, Deref @ SCode::Element::CLASS { cmt, classDef: Deref @ SCode::ClassDef::OVERLOAD { pathLst: funcnames }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity } }, prefixes: Deref @ SCode::Prefixes { visibility, .. }, name: n, .. }, _, _) => {
                    let mut fpath: Arc<Absyn::Path>;
                    let mut resfns: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    let mut isImpure: bool = false;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    (cache, env, ih, resfns) = instOverloadedFunctions(cache.clone(), env.clone(), ih.clone(), pre.clone(), funcnames.clone(), var_field!((*inClass).info, SCode::Element::CLASS).clone())?;
                    (cache, fpath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), (n.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    isImpure = AbsynUtil::isImpure(purity.clone(), false);
                    resfns = cons(DAE::Function::FUNCTION { path: fpath.clone(), functions: list![DAE::FunctionDefinition::FUNCTION_DEF { body: metamodelica::nil() }], type_: DAE::T_UNKNOWN_DEFAULT.clone(), visibility: visibility.clone(), partialPrefix: true, isImpure: isImpure.clone(), inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, unusedInputs: metamodelica::nil(), source: DAE::emptyElementSource.clone(), comment: Some(cmt.clone()) }, resfns.clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), resfns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, Deref @ SCode::Element::CLASS { name: n, .. }, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.implicitFunctionInstantiation2 failed ")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, funcs))
}

fn instantiateDerivativeFuncs(mut cache: FCore::Cache, mut env: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut funcs: Arc<metamodelica::List<DAE::FunctionDefinition>>, mut path: Arc<Absyn::Path>, mut info: SourceInfo) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    outCache = instantiateDerivativeFuncs2(cache.clone(), env.clone(), ih.clone(), DAEUtil::getDerivativePaths(funcs.clone())?, path.clone(), info.clone())?;
    Ok(outCache)
}

fn instantiateDerivativeFuncs2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPaths: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut path: Arc<Absyn::Path>, mut info: SourceInfo) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    outCache = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPaths.clone(), path.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(cache.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::List::Cons { head: p, tail: paths }, _, _) => {
                    let mut funcs: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    let mut cenv: FCore::Graph;
                    let mut cdef: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut p = (*p).clone();
                    (cache, cdef, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), p.clone(), Some(info.clone()))?;
                    (cache, p) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), p.clone())?;
                    let _ = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
                    let () = __mc_input.clone() else { bail!("nomatch") };
                    FCore::checkCachedInstFuncGuard(cache.clone(), p.clone())?;
                    Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    let mut cache: FCore::Cache = cache.clone();
                    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>;
                    let mut funcs: Arc<metamodelica::List<DAE::Function>>;
                    cache = FCore::addCachedInstFuncGuard(cache.clone(), p.clone())?;
                    (cache, _, ih, funcs) = implicitFunctionInstantiation2(cache.clone(), cenv.clone(), ih.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef.clone(), metamodelica::nil(), false)?;
                    funcs = InstUtil::addNameToDerivativeMapping(funcs.clone(), path.clone());
                    cache = FCore::addDaeFunction(cache.clone(), funcs.clone())?;
                    Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    Ok(instantiateDerivativeFuncs2(cache.clone(), env.clone(), ih.clone(), paths.clone(), path.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut p: Arc<Absyn::Path>;
                    let mut fun: ArcStr = arcstr::literal!("");
                    let mut scope: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(inPaths.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    fun = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    scope = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_FUNCTION_ERROR.clone(), list![(fun.clone()).clone(), (scope.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCache)
}

pub fn implicitFunctionTypeInstantiation(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inClass: Arc<SCode::Element>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    (outCache, outEnv, outIH) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: _ } }, .. }) => {
                    let mut env_1: FCore::Graph;
                    let mut funs: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env_1, ih, funs) = implicitFunctionInstantiation2(cache.clone(), env.clone(), ih.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, inClass.clone(), metamodelica::nil(), true)?;
                    cache = FCore::addDaeExtFunction(cache.clone(), funs.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ SCode::Element::CLASS { info, cmt, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: extDecl, elementLst: elts, .. }, restriction: r, partialPrefix: p, encapsulatedPrefix: e, prefixes, name: id }) => {
                    let mut stripped_class: Arc<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut elts = (*elts).clone();
                    elts = List::select(elts.clone(), Arc::new(fnptr!(isElementImportantForFunction, Arc<SCode::Element>)));
                    stripped_class = Arc::new(SCode::Element::CLASS { name: (id.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: e.clone(), partialPrefix: p.clone(), restriction: r.clone(), classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: elts.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: extDecl.clone() }), cmt: cmt.clone(), info: info.clone() });
                    (cache, env_1, ih, _) = implicitFunctionInstantiation2(cache.clone(), env.clone(), ih.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, stripped_class.clone(), metamodelica::nil(), true)?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ SCode::Element::CLASS { info, classDef: Deref @ SCode::ClassDef::DERIVED { modifications: mod1, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, .. }, .. }, name: id, .. }) => {
                    let mut env_1: FCore::Graph;
                    let mut fpath: Arc<Absyn::Path>;
                    let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cenv: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), None)?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    c = __pa1.clone();
                    cenv = __pa2.clone();
                    (cache, mod2) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, mod1.clone(), false, Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    (cache, _, ih, _, _, _, ty, _, _, _) = Inst::instClass(cache.clone(), cenv.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), mod2.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), true, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?;
                    env_1 = env.clone();
                    (cache, fpath) = Inst::makeFullyQualifiedIdent(cache.clone(), env_1.clone(), (id.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    ty1 = InstUtil::setFullyQualifiedTypename(ty.clone(), fpath.clone());
                    env_1 = FGraph::mkTypeNode(env_1.clone(), (id.clone()).clone(), ty1.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::OVERLOAD { .. }, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    (cache, env, ih, _) = implicitFunctionInstantiation2(cache.clone(), env.clone(), ih.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, inClass.clone(), metamodelica::nil(), true)?;
                    Ok((cache.clone(), env.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ SCode::Element::CLASS { name: id, .. }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.implicitFunctionTypeInstantiation failed ")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!("\nenv: ")); __mm_s.push_str(&*FGraph::getGraphNameStr(inEnv.clone())?); __mm_s.push_str(&*literal!("\nelelement: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(inClass.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH))
}

fn instOverloadedFunctions(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut pre: DAE::Prefix, mut inAbsynPathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inInfo: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<metamodelica::List<DAE::Function>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outFns: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
    (outCache, outEnv, outIH, outFns) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), pre.clone(), inAbsynPathLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ih, _, Deref @ metamodelica::List::Nil) => {
                    Ok((cache.clone(), inEnv.clone(), ih.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, Deref @ metamodelica::List::Cons { head: r#fn, tail: fns }) => {
                    let mut cenv: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut resfns1: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    let mut resfns2: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    let mut rest: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), r#fn.clone(), Some(inInfo.clone()))?) {
                        (__pa0, __pa2 @ Deref @ SCode::Element::CLASS { restriction: __pa1, .. }, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    rest = __pa1.clone();
                    c = __pa2.clone();
                    cenv = __pa3.clone();
                    let true = (SCodeUtil::isFunctionRestriction(rest.clone())) else { bail!("pattern mismatch") };
                    (cache, env, ih, resfns1) = implicitFunctionInstantiation2(inCache.clone(), cenv.clone(), inIH.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), c.clone(), metamodelica::nil(), false)?;
                    (cache, env, ih, resfns2) = instOverloadedFunctions(cache.clone(), env.clone(), ih.clone(), pre.clone(), fns.clone(), inInfo.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), listAppend(resfns1.clone(), resfns2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: r#fn, tail: _ }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instOverloaded_functions failed ")); __mm_s.push_str(&*AbsynUtil::pathString(r#fn.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outFns))
}

fn instExtDecl(mut cache: FCore::Cache, mut env: FCore::Graph, mut iH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut name: ArcStr, mut inScExtDecl: Arc<SCode::ExternalDecl>, mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut funcType: Arc<DAE::Type>, mut r#impl: bool, mut pre: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::ExternalDecl)> {
    let mut cache: FCore::Cache = cache;
    let mut iH: Arc<metamodelica::List<InnerOuter::TopInstance>> = iH;
    let mut daeextdecl: DAE::ExternalDecl;
    let mut fname: ArcStr = arcstr::literal!("");
    let mut lang: ArcStr = arcstr::literal!("");
    let mut fargs: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
    let mut rettype: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
    let mut ann: Option<Arc<SCode::Annotation>> = None;
    let mut extdecl: Arc<SCode::ExternalDecl> = inScExtDecl.clone();
    ann = InstUtil::instExtGetAnnotation(extdecl.clone())?;
    lang = (InstUtil::instExtGetLang(extdecl.clone())?).clone();
    fname = (InstUtil::instExtGetFname(extdecl.clone(), (name.clone()).clone())?).clone();
    if !(InstUtil::isExtExplicitCall(extdecl.clone())) {
        (fargs, rettype) = instExtMakeDefaultExternalCall(inElements.clone(), funcType.clone(), (lang.clone()).clone(), info.clone())?;
    } else {
        (cache, fargs) = InstUtil::instExtGetFargs(cache.clone(), env.clone(), extdecl.clone(), r#impl.clone(), pre.clone(), info.clone())?;
        (cache, rettype) = InstUtil::instExtGetRettype(cache.clone(), env.clone(), extdecl.clone(), r#impl.clone(), pre.clone(), info.clone())?;
    }
    daeextdecl = DAE::ExternalDecl { name: (fname.clone()).clone(), args: fargs.clone(), returnArg: rettype.clone(), language: (lang.clone()).clone(), ann: ann.clone() };
    Ok((cache, iH, daeextdecl))
}

fn instExtMakeDefaultExternalCall(mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut funcType: Arc<DAE::Type>, mut lang: ArcStr, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<DAE::ExtArg>>, DAE::ExtArg)> {
    let mut fargs: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
    let mut rettype: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut singleOutput: bool = false;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Element>;
    fargs = metamodelica::nil();
    if lang.clone() == literal!("builtin") {
        rettype = openmodelica_frontend_types::DAE::ExtArg::NOEXTARG;
        return Ok((fargs, rettype));
    }
    (rettype, singleOutput) = (::match_deref::match_deref! { match &(funcType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            if lang.clone() != literal!("builtin") {
                Error::addSourceMessage(Error::EXT_FN_SINGLE_RETURN_ARRAY.clone(), list![(lang.clone()).clone()], info.clone())?;
            }
            (openmodelica_frontend_types::DAE::ExtArg::NOEXTARG, false)
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_TUPLE { .. }, .. } => (openmodelica_frontend_types::DAE::ExtArg::NOEXTARG, false),
        Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_NORETCALL, .. } => (openmodelica_frontend_types::DAE::ExtArg::NOEXTARG, false),
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => (DAE::ExtArg::EXTARG { componentRef: DAEUtil::varCref(List::find(elements.clone(), Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)))?)?, direction: openmodelica_ast::Absyn::Direction::OUTPUT, type_: ty.clone() }, true),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("instExtMakeDefaultExternalCall failed for ")); __mm_s.push_str(&*TypesDump::unparseType(funcType.clone())?); ArcStr::from(__mm_s) }).clone(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut elt in &*elements.clone() {
        let mut elt = elt.clone();
        fargs = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT, .. } if (!(singleOutput.clone())) => addExtVarToCall(var_field!((*elt).componentRef, DAE::Element::VAR).clone(), openmodelica_ast::Absyn::Direction::OUTPUT, var_field!((*elt).dims, DAE::Element::VAR).clone(), fargs.clone())?,
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, .. } => addExtVarToCall(var_field!((*elt).componentRef, DAE::Element::VAR).clone(), openmodelica_ast::Absyn::Direction::INPUT, var_field!((*elt).dims, DAE::Element::VAR).clone(), fargs.clone())?,
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::BIDIR, .. } => addExtVarToCall(var_field!((*elt).componentRef, DAE::Element::VAR).clone(), openmodelica_ast::Absyn::Direction::OUTPUT, var_field!((*elt).dims, DAE::Element::VAR).clone(), fargs.clone())?,
        _ => fargs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    fargs = fargs.clone().reverse();
    Ok((fargs, rettype))
}

fn addExtVarToCall(mut cr: Arc<DAE::ComponentRef>, mut dir: Absyn::Direction, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut fargs: Arc<metamodelica::List<DAE::ExtArg>>) -> Result<Arc<metamodelica::List<DAE::ExtArg>>> {
    let mut fargs: Arc<metamodelica::List<DAE::ExtArg>> = fargs;
    fargs = cons(DAE::ExtArg::EXTARG { componentRef: cr.clone(), direction: dir.clone(), type_: ComponentReference::crefTypeFull(cr.clone())? }, fargs.clone());
    for mut dim in 1..=(dims.clone().len() as i32) {
        fargs = cons(DAE::ExtArg::EXTARGSIZE { componentRef: cr.clone(), type_: ComponentReference::crefTypeFull(cr.clone())?, exp: Arc::new(DAE::Exp::ICONST { integer: dim.clone() }) }, fargs.clone());
    }
    Ok(fargs)
}

pub fn getRecordConstructorFunction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, DAE::Function)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outFunc: DAE::Function;
    (outCache, outFunc) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut func: DAE::Function;
                    path = AbsynUtil::makeFullyQualified(inPath.clone());
                    func = FCore::getCachedInstFunc(inCache.clone(), path.clone())?;
                    Ok((inCache.clone(), func.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut recordCl: Arc<SCode::Element>;
                    let mut recordEnv: FCore::Graph;
                    let mut func: DAE::Function;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut recType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut fixedTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut funcTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut inputs: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut locals: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut eqCo: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut newName: ArcStr = arcstr::literal!("");
                    let mut extConvert: bool = false;
                    (_, recordCl, recordEnv) = Lookup::lookupClass(inCache.clone(), inEnv.clone(), inPath.clone(), None)?;
                    let true = (SCodeUtil::isRecord(recordCl.clone())) else { bail!("pattern mismatch") };
                    name = (SCodeUtil::getElementName(recordCl.clone())?).clone();
                    newName = (FGraph::getInstanceOriginalName(recordEnv.clone(), (name.clone()).clone())?).clone();
                    recordCl = SCodeUtil::setClassName((newName.clone()).clone(), recordCl.clone())?;
                    (cache, _, _, _, _, _, recType, _, _, _) = Inst::instClass(inCache.clone(), recordEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsynBuilder::emptyInstStore(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, recordCl.clone(), metamodelica::nil(), true, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(recType.clone()) {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, varLst: __pa1, equalityConstraint: __pa2, usedExternally: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    vars = __pa1.clone();
                    eqCo = __pa2.clone();
                    extConvert = __pa3.clone();
                    vars = Types::filterRecordComponents(vars.clone(), SCodeUtil::elementInfo(recordCl.clone()))?;
                    (inputs, locals) = List::extractOnTrue(vars.clone(), Arc::new(Types::isModifiableTypesVar));
                    inputs = List::map(inputs.clone(), Arc::new(fnptr!(Types::setVarDefaultInput, Arc<DAE::Var>)));
                    locals = List::map(locals.clone(), Arc::new(fnptr!(Types::setVarProtected, Arc<DAE::Var>)));
                    vars = listAppend(inputs.clone(), locals.clone());
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    fixedTy = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: vars.clone(), equalityConstraint: eqCo.clone(), usedExternally: extConvert.clone() });
                    fargs = Types::makeFargsList(inputs.clone());
                    funcTy = Arc::new(DAE::Type::T_FUNCTION { funcArg: fargs.clone(), funcResultType: fixedTy.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: path.clone() });
                    func = DAE::Function::RECORD_CONSTRUCTOR { path: path.clone(), type_: funcTy.clone(), source: DAE::emptyElementSource.clone() };
                    cache = InstUtil::addFunctionsToDAE(cache.clone(), list![func.clone()], openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    path = AbsynUtil::pathSetLastIdent(path.clone(), (name.clone()).clone())?;
                    fixedTy = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: vars.clone(), equalityConstraint: eqCo.clone(), usedExternally: extConvert.clone() });
                    fargs = Types::makeFargsList(inputs.clone());
                    funcTy = Arc::new(DAE::Type::T_FUNCTION { funcArg: fargs.clone(), funcResultType: fixedTy.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: path.clone() });
                    func = DAE::Function::RECORD_CONSTRUCTOR { path: path.clone(), type_: funcTy.clone(), source: DAE::emptyElementSource.clone() };
                    cache = InstUtil::addFunctionsToDAE(cache.clone(), list![func.clone()], openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    Ok((cache.clone(), func.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstFunction.getRecordConstructorFunction failed for ")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outFunc))
}

pub fn addRecordConstructorFunction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    outCache = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inType.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, .. }, _) => {
                    let mut cache = (*cache).clone();
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    (cache, _) = getRecordConstructorFunction(cache.clone(), inEnv.clone(), path.clone())?;
                    Ok(cache.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: vars, equalityConstraint: eqCo, usedExternally: extConvert }, _) => {
                    let mut inputs: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut locals: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut fixedTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut funcTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut func: DAE::Function;
                    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut path = (*path).clone();
                    let mut vars = (*vars).clone();
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    vars = Types::filterRecordComponents(vars.clone(), inInfo.clone())?;
                    (inputs, locals) = List::extractOnTrue(vars.clone(), Arc::new(Types::isModifiableTypesVar));
                    inputs = List::map(inputs.clone(), Arc::new(fnptr!(Types::setVarDefaultInput, Arc<DAE::Var>)));
                    locals = List::map(locals.clone(), Arc::new(fnptr!(Types::setVarProtected, Arc<DAE::Var>)));
                    vars = listAppend(inputs.clone(), locals.clone());
                    fixedTy = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: vars.clone(), equalityConstraint: eqCo.clone(), usedExternally: extConvert.clone() });
                    fargs = Types::makeFargsList(inputs.clone());
                    funcTy = Arc::new(DAE::Type::T_FUNCTION { funcArg: fargs.clone(), funcResultType: fixedTy.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: path.clone() });
                    func = DAE::Function::RECORD_CONSTRUCTOR { path: path.clone(), type_: funcTy.clone(), source: DAE::emptyElementSource.clone() };
                    cache = InstUtil::addFunctionsToDAE(cache.clone(), list![func.clone()], openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    Ok(cache.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCache.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCache)
}

fn isElementImportantForFunction(mut elt: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::VAR, direction: Absyn::Direction::BIDIR, .. }, prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED, .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn checkExtObjOutput(mut inType: Arc<DAE::Type>, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inType.clone(), info.clone())) {
        (Deref @ DAE::Type::T_FUNCTION { path, funcResultType: ty, .. }, _) => {
            ::match_deref::match_deref! { match &(Types::traverseType(ty.clone(), (path.clone(), info.clone(), true), Arc::new(checkExtObjOutputWork))?) {
                (_, (_, _, true)) => (),
                _ => bail!("pattern mismatch"),
            } };
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn checkExtObjOutputWork(mut ty: Arc<DAE::Type>, mut inTpl: (Arc<Absyn::Path>, SourceInfo, bool)) -> Result<(Arc<DAE::Type>, (Arc<Absyn::Path>, SourceInfo, bool))> {
    let mut oty: Arc<DAE::Type> = ty.clone();
    let mut outTpl: (Arc<Absyn::Path>, SourceInfo, bool);
    outTpl = (::match_deref::match_deref! { match &((ty.clone(), inTpl.clone())) {
        (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: path1 }, .. }, (path2, info, true)) => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            let mut b: bool = false;
            let mut path1 = (*path1).clone();
            path1 = AbsynUtil::joinPaths(path1.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("constructor")).clone() }))?;
            str1 = AbsynUtil::pathStringNoQual(path2.clone(), (literal!(".")).clone(), true, false)?;
            str2 = AbsynUtil::pathStringNoQual(path1.clone(), (literal!(".")).clone(), true, false)?;
            b = AbsynUtil::pathEqual(path1.clone(), path2.clone());
            Error::assertionOrAddSourceMessage(b.clone(), Error::FUNCTION_RETURN_EXT_OBJ.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
            outTpl = if (b.clone()) {inTpl.clone()} else {(path2.clone(), info.clone(), false)};
            outTpl.clone()
        },
        _ => {
            inTpl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oty, outTpl))
}

