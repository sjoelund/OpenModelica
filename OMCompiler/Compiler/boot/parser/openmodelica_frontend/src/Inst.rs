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

use crate::Builtin;
use crate::Ceval;
use crate::ComponentReference;
use crate::ConnectUtil;
use crate::ConnectionGraph;
use crate::DAEDump;
use crate::DAEUtil;
use crate::Expression;
use crate::FCore;
use crate::FGraph;
use crate::FGraphBuildEnv;
use crate::FNode;
use crate::FUnitCheck as UnitCheck;
use crate::HashSet;
use crate::HashTable5;
use crate::HashTable;
use crate::HashTableCG;
use crate::InnerOuter;
use crate::InstBinding;
use crate::InstExtends;
use crate::InstFunction;
use crate::InstHashTable;
use crate::InstMeta;
use crate::InstSection;
use crate::InstStateMachineUtil;
use crate::InstTypes;
use crate::InstUtil;
use crate::InstVar;
use crate::Lookup;
use crate::Mod;
use crate::PrefixUtil;
use crate::SCodeInstUtil;
use crate::Static;
use crate::Types;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use crate::UnitParserExt;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// public imports
// **
// These type aliases are introduced to make the code a little more readable.
// **
/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

pub type InstDims = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>;

type BasicTypeAttrTyper = std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>;

// protected imports
// BTH
fn instantiateClass_dispatch(mut inCache: FCore::Cache, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>, mut doSCodeDep: bool, mut relaxedFrontEnd: bool, mut clearCache: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDAElist: DAE::DAElist;
    (outCache, outEnv, outIH, outDAElist) = (::match_deref::match_deref! { match &((inCache.clone(), inIH.clone(), inProgram.clone(), inPath.clone())) {
        (cache, ih, cdecls @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::IDENT { .. }) => {
            let mut env: FCore::Graph;
            let mut dae2: DAE::DAElist;
            let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
            let mut cache = (*cache).clone();
            let mut ih = (*ih).clone();
            let mut cdecls = (*cdecls).clone();
            cache = FCore::setCacheClassName(cache.clone(), path.clone());
            if doSCodeDep.clone() {
                cdecls = InstUtil::scodeFlatten(cdecls.clone(), inPath.clone())?;
                ExecStat::execStat((literal!("FrontEnd - scodeFlatten")).clone())?;
            }
            (cache, env) = Builtin::initialGraph(cache.clone())?;
            env = FGraphBuildEnv::mkProgramGraph(cdecls.clone(), crate::FCore::Kind::USERDEFINED, env.clone())?;
            source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(env.clone())?)?;
            if Flags::isSet(Flags::GC_PROF.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*GCExt::profStatsStr(GCExt::getProfStats(), (literal!("GC stats after pre-frontend work (building graphs):")).clone(), (literal!("\n  ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            ExecStat::execStat((literal!("FrontEnd - mkProgramGraph")).clone())?;
            (cache, env, ih, dae2) = instClassInProgram(cache.clone(), env.clone(), ih.clone(), cdecls.clone(), path.clone(), source.clone(), relaxedFrontEnd.clone())?;
            if clearCache.clone() {
                InstHashTable::release()?;
            }
            (cache.clone(), env.clone(), ih.clone(), dae2.clone())
        },
        (cache, ih, cdecls @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
            let mut env: FCore::Graph;
            let mut dae: DAE::DAElist;
            let mut n: ArcStr = arcstr::literal!("");
            let mut pathstr: ArcStr = arcstr::literal!("");
            let mut cdef: Arc<SCode::Element>;
            let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
            let mut daeElts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut cmt: Option<Arc<SCode::Comment>> = None;
            let mut cache = (*cache).clone();
            let mut ih = (*ih).clone();
            let mut cdecls = (*cdecls).clone();
            cache = FCore::setCacheClassName(cache.clone(), path.clone());
            if doSCodeDep.clone() {
                cdecls = InstUtil::scodeFlatten(cdecls.clone(), inPath.clone())?;
                ExecStat::execStat((literal!("FrontEnd - scodeFlatten")).clone())?;
            }
            pathstr = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            (cache, env) = Builtin::initialGraph(cache.clone())?;
            env = FGraphBuildEnv::mkProgramGraph(cdecls.clone(), crate::FCore::Kind::USERDEFINED, env.clone())?;
            let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), Some(Absyn::dummyInfo.clone()))?) {
                (__pa0, __pa2 @ Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            n = __pa1.clone();
            cdef = __pa2.clone();
            env = __pa3.clone();
            checkInstanceRestriction(cdef.clone(), path.clone(), relaxedFrontEnd.clone())?;
            if Flags::isSet(Flags::GC_PROF.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*GCExt::profStatsStr(GCExt::getProfStats(), (literal!("GC stats after pre-frontend work (building graphs):")).clone(), (literal!("\n  ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            ExecStat::execStat((literal!("FrontEnd - mkProgramGraph")).clone())?;
            (cache, env, ih, _, dae, _, _, _, _, _) = instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), makeTopComponentPrefix(env.clone(), (n.clone()).clone()), cdef.clone(), metamodelica::nil(), false, crate::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone())?;
            dae = InstUtil::reEvaluateInitialIfEqns(cache.clone(), env.clone(), dae.clone(), true)?;
            source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(env.clone())?)?;
            daeElts = DAEUtil::daeElements(dae.clone())?;
            cmt = SCodeUtil::getElementComment(cdef.clone());
            dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMP { ident: (pathstr.clone()).clone(), dAElist: daeElts.clone(), source: source.clone(), comment: cmt.clone() })] };
            if clearCache.clone() {
                InstHashTable::release()?;
            }
            (cache.clone(), env.clone(), ih.clone(), dae.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outDAElist))
}

pub fn instantiateClass(mut inCache: FCore::Cache, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>, mut doSCodeDep: bool, mut relaxedFrontEnd: bool, mut clearCache: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDAElist: DAE::DAElist;
    (outCache, outEnv, outIH, outDAElist) = 'mc: {
        let __mc_input = (inCache.clone(), inIH.clone(), inProgram.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _) => {
                    Error::addMessage(Error::NO_CLASSES_LOADED.clone(), metamodelica::nil())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, path) => {
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outEnv: FCore::Graph;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDAElist: DAE::DAElist;
                    (outCache, outEnv, outIH, outDAElist) = instantiateClass_dispatch(cache.clone(), ih.clone(), cdecls.clone(), path.clone(), doSCodeDep.clone(), relaxedFrontEnd.clone(), clearCache.clone())?;
                    outDAElist = UnitCheck::checkUnits(outDAElist.clone(), FCore::getFunctionTree(outCache.clone()))?;
                    Ok((outCache.clone(), outEnv.clone(), outIH.clone(), outDAElist.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: _, tail: _ }, path) => {
                    let mut cname_str: ArcStr = arcstr::literal!("");
                    let mut stackOverflow: bool = false;
                    stackOverflow = setStackOverflowSignal(false);
                    cname_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*if (stackOverflow.clone()) {literal!(". The compiler got into Stack Overflow!")} else {literal!("")}); ArcStr::from(__mm_s) }).clone();
                    if !(Config::getGraphicsExpMode()?) {
                        Error::addMessage(Error::ERROR_FLATTENING.clone(), list![(cname_str.clone()).clone()])?;
                    }
                    InstHashTable::release()?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDAElist))
}

pub fn instantiatePartialClass(mut inCache: FCore::Cache, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDAElist: DAE::DAElist;
    (outCache, outEnv, outIH, outDAElist) = 'mc: {
        let __mc_input = (inCache.clone(), inIH.clone(), inProgram.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _) => {
                    Error::addMessage(Error::NO_CLASSES_LOADED.clone(), metamodelica::nil())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::IDENT { .. }) => {
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut cdecls = (*cdecls).clone();
                    (cache, env) = Builtin::initialGraph(cache.clone())?;
                    env_1 = FGraphBuildEnv::mkProgramGraph(cdecls.clone(), crate::FCore::Kind::USERDEFINED, env.clone())?;
                    cdecls = List::map1(cdecls.clone(), (std::sync::Arc::new(SCodeUtil::classSetPartial) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, SCode::Partial) -> Result<Arc<SCode::Element>> + 'static>), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL);
                    source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(env.clone())?)?;
                    (cache, env_2, ih, dae) = instClassInProgram(cache.clone(), env_1.clone(), ih.clone(), cdecls.clone(), path.clone(), source.clone(), true)?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), dae.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut n: ArcStr = arcstr::literal!("");
                    let mut pathstr: ArcStr = arcstr::literal!("");
                    let mut cdef: Arc<SCode::Element>;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut daeElts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut cmt: Option<Arc<SCode::Comment>> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env) = Builtin::initialGraph(cache.clone())?;
                    env_1 = FGraphBuildEnv::mkProgramGraph(cdecls.clone(), crate::FCore::Kind::USERDEFINED, env.clone())?;
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env_1.clone(), path.clone(), Some(Absyn::dummyInfo.clone()))?) {
                        (__pa0, __pa2 @ Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    n = __pa1.clone();
                    cdef = __pa2.clone();
                    env_2 = __pa3.clone();
                    cdef = SCodeUtil::classSetPartial(cdef.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    (cache, env_2, ih, _, dae, _, _, _, _, _) = instClass(cache.clone(), env_2.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), makeTopComponentPrefix(env_2.clone(), (n.clone()).clone()), cdef.clone(), metamodelica::nil(), false, crate::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone())?;
                    pathstr = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(env.clone())?)?;
                    daeElts = DAEUtil::daeElements(dae.clone())?;
                    cmt = SCodeUtil::getElementComment(cdef.clone());
                    dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMP { ident: (pathstr.clone()).clone(), dAElist: daeElts.clone(), source: source.clone(), comment: cmt.clone() })] };
                    Ok((cache.clone(), env_2.clone(), ih.clone(), dae.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, path) => {
                    if !((!(Config::getGraphicsExpMode()?))) { bail!("guard") }
                    let mut cname_str: ArcStr = arcstr::literal!("");
                    cname_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::ERROR_FLATTENING.clone(), list![(cname_str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDAElist))
}

fn makeTopComponentPrefix(mut inGraph: FCore::Graph, mut inName: ArcStr) -> DAE::Prefix {
    let mut outPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
    let mut p: Arc<Absyn::Path>;
    outPrefix = openmodelica_frontend_types::DAE::Prefix::NOPRE;
    outPrefix
}

fn instClassInProgram(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>, mut inSource: Arc<DAE::ElementSource>, mut relaxedFrontEnd: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    (outCache, outEnv, outIH, outDae) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inProgram.clone(), inPath.clone(), inSource.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), DAE::emptyDae().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ Absyn::Path::IDENT { name: Deref @ "" }, _) => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), DAE::emptyDae().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ Absyn::Path::IDENT { name }, _) => {
                    let mut cls: Arc<SCode::Element>;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut ih: InstanceHierarchy = metamodelica::nil();
                    let mut dae: DAE::DAElist;
                    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut cmt: Option<Arc<SCode::Comment>> = None;
                    cls = InstUtil::lookupTopLevelClass((name.clone()).clone(), inProgram.clone(), true)?;
                    (cache, env, ih, _, dae, _, _, _, _, _) = instClass(inCache.clone(), inEnv.clone(), inIH.clone(), UnitAbsynBuilder::emptyInstStore(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), makeTopComponentPrefix(inEnv.clone(), (name.clone()).clone()), cls.clone(), metamodelica::nil(), false, crate::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone())?;
                    dae = InstUtil::reEvaluateInitialIfEqns(cache.clone(), env.clone(), dae.clone(), true)?;
                    elts = DAEUtil::daeElements(dae.clone())?;
                    cmt = SCodeUtil::getElementComment(cls.clone());
                    dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMP { ident: (name.clone()).clone(), dAElist: elts.clone(), source: inSource.clone(), comment: cmt.clone() })] };
                    Ok((cache.clone(), env.clone(), ih.clone(), dae.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Inst.instClassInProgram failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDae))
}

pub fn instClass(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inClass: Arc<SCode::Element>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImplicit: bool, mut inCallingScope: InstTypes::CallingScope, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, ClassInf::State, Option<SCode::Attributes>, ConnectionGraph::ConnectionGraph)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outState: ClassInf::State;
    let mut optDerAttr: Option<SCode::Attributes> = None;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (cache, outEnv, outIH, outStore, outDae, outSets, outType, outState, optDerAttr, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone(), inImplicit.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, store, _, _, Deref @ SCode::Element::CLASS { info, restriction: r, partialPrefix: SCode::Partial::PARTIAL, name: n, .. }, _, _, _, _, _) => {
                    let mut env: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut ci_state_1: ClassInf::State;
                    let mut dae: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Arc<SCode::Element>;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut store = (*store).clone();
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    let false = (SCodeUtil::isFunctionRestriction(r.clone())) else { bail!("pattern mismatch") };
                    c = SCodeUtil::setClassPartialPrefix(openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, inClass.clone())?;
                    if !(Config::getGraphicsExpMode()?) {
                        Error::addSourceMessage(Error::INST_PARTIAL_CLASS_CHECK_MODEL_WARNING.clone(), list![(n.clone()).clone()], info.clone())?;
                    }
                    (cache, env, ih, store, dae, csets, ty, ci_state_1, oDA, graph) = instClass(inCache.clone(), inEnv.clone(), inIH.clone(), store.clone(), inMod.clone(), inPrefix.clone(), c.clone(), inInstDims.clone(), inImplicit.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), ci_state_1.clone(), oDA.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, r#mod, pre, c @ Deref @ SCode::Element::CLASS { info, partialPrefix, restriction: r, encapsulatedPrefix: encflag, name: n, .. }, inst_dims, r#impl, callscope, graph, _) => {
                    let mut env_1: FCore::Graph;
                    let mut env_3: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let mut strDepth: ArcStr = arcstr::literal!("");
                    let mut callscope_1: bool = false;
                    let mut isFn: bool = false;
                    let mut notIsPartial: bool = false;
                    let mut isPartialFn: bool = false;
                    let mut recursionDepthReached: bool = false;
                    let mut ci_state: ClassInf::State;
                    let mut ci_state_1: ClassInf::State;
                    let mut dae1: DAE::DAElist;
                    let mut dae1_1: DAE::DAElist;
                    let mut dae: DAE::DAElist;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut bc_ty: Option<Arc<DAE::Type>> = None;
                    let mut fq_class: Arc<Absyn::Path>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut equalityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    recursionDepthReached = (FGraph::currentScope(env.clone())?.len() as i32) < Global::recursionDepthLimit.clone();
                    if !(recursionDepthReached.clone()) {
                        scopeName = (FGraph::printGraphPathStr(env.clone())?).clone();
                        strDepth = (intString(Global::recursionDepthLimit.clone())).clone();
                        Error::addSourceMessage(Error::RECURSION_DEPTH_REACHED.clone(), list![(strDepth.clone()).clone(), (scopeName.clone()).clone()], info.clone())?;
                        bail!("fail");
                    }
                    isFn = SCodeUtil::isFunctionRestriction(r.clone());
                    notIsPartial = !(SCodeUtil::partialBool(partialPrefix.clone())?);
                    isPartialFn = isFn.clone() && SCodeUtil::partialBool(partialPrefix.clone())?;
                    let true = (notIsPartial.clone() || isPartialFn.clone()) else { bail!("pattern mismatch") };
                    env_1 = FGraph::openScope(env.clone(), encflag.clone(), (n.clone()).clone(), FGraph::restrictionToScopeType(r.clone()))?;
                    ci_state = ClassInfUtil::start(r.clone(), FGraph::getGraphName(env_1.clone())?)?;
                    csets = ConnectUtil::newSet(pre.clone(), inSets.clone())?;
                    (cache, env_3, ih, store, dae1, csets, ci_state_1, tys, bc_ty, oDA, equalityConstraint, graph) = instClassIn(cache.clone(), env_1.clone(), ih.clone(), store.clone(), r#mod.clone(), pre.clone(), ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), csets.clone(), None)?;
                    csets = ConnectUtil::addSet(inSets.clone(), csets.clone())?;
                    (cache, fq_class) = makeFullyQualifiedIdent(cache.clone(), env.clone(), (n.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    callscope_1 = InstUtil::isTopCall(callscope.clone());
                    dae1_1 = DAEUtil::addComponentType(dae1.clone(), fq_class.clone())?;
                    (csets, _, graph) = InnerOuter::retrieveOuterConnections(cache.clone(), env_3.clone(), ih.clone(), pre.clone(), csets.clone(), callscope_1.clone(), graph.clone())?;
                    dae = ConnectUtil::equations(callscope_1.clone(), csets.clone(), dae1_1.clone(), graph.clone(), (AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(fq_class.clone()), (literal!(".")).clone(), true, false)?).clone())?;
                    ty = InstUtil::mktype(fq_class.clone(), ci_state_1.clone(), tys.clone(), bc_ty.clone(), equalityConstraint.clone(), c.clone(), InstUtil::extractComment(dae.elementLst.clone())?)?;
                    dae = InstUtil::updateDeducedUnits(callscope_1.clone(), store.clone(), dae.clone());
                    ty = markDerivedRecordOutsideBindings(ty.clone(), c.clone())?;
                    ty = markTypesVarsOutsideBindings(ty.clone(), r#mod.clone())?;
                    ty = InstUtil::fixInstClassType(ty.clone(), isPartialFn.clone())?;
                    Ok((cache.clone(), env_3.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), ci_state_1.clone(), oDA.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, _, _, Deref @ SCode::Element::CLASS { info, partialPrefix: SCode::Partial::PARTIAL, name: n, .. }, _, false, _, _, _) => {
                    if !(Config::getGraphicsExpMode()?) {
                        Error::addSourceMessage(Error::INST_PARTIAL_CLASS.clone(), list![(n.clone()).clone()], info.clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, _, Deref @ SCode::Element::CLASS { name: n, .. }, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instClass: ")); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, outEnv, outIH, outStore, outDae, outSets, outType, outState, optDerAttr, outGraph))
}

fn instClassBasictype(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inClass: Arc<SCode::Element>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImplicit: bool, mut inCallingScope: InstTypes::CallingScope, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Var>>>, ClassInf::State)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outTypeVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outState: ClassInf::State;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outTypeVars, outState) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone(), inImplicit.clone(), inCallingScope.clone(), inSets.clone())) {
        (cache, env, ih, store, r#mod, pre, c @ Deref @ SCode::Element::CLASS { restriction: r, encapsulatedPrefix: encflag, name: n, .. }, inst_dims, r#impl, _, _) => {
            let mut env_1: FCore::Graph;
            let mut env_3: FCore::Graph;
            let mut ci_state: ClassInf::State;
            let mut ci_state_1: ClassInf::State;
            let mut c_1: Arc<SCode::Element>;
            let mut dae1: DAE::DAElist;
            let mut dae1_1: DAE::DAElist;
            let mut dae: DAE::DAElist;
            let mut csets: DAE::Connect::Sets;
            let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut bc_ty: Option<Arc<DAE::Type>> = None;
            let mut fq_class: Arc<Absyn::Path>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            let mut ih = (*ih).clone();
            let mut store = (*store).clone();
            env_1 = FGraph::openScope(env.clone(), encflag.clone(), (n.clone()).clone(), FGraph::restrictionToScopeType(r.clone()))?;
            ci_state = ClassInfUtil::start(r.clone(), FGraph::getGraphName(env_1.clone())?)?;
            c_1 = SCodeUtil::classSetPartial(c.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
            (cache, env_3, ih, store, dae1, csets, ci_state_1, tys, bc_ty, _, _, _) = instClassIn(cache.clone(), env_1.clone(), ih.clone(), store.clone(), r#mod.clone(), pre.clone(), ci_state.clone(), c_1.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, inst_dims.clone(), r#impl.clone(), crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), inSets.clone(), None)?;
            (cache, fq_class) = makeFullyQualifiedIdent(cache.clone(), env_3.clone(), (n.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
            dae1_1 = DAEUtil::addComponentType(dae1.clone(), fq_class.clone())?;
            dae = dae1_1.clone();
            ty = InstUtil::mktypeWithArrays(fq_class.clone(), ci_state_1.clone(), tys.clone(), bc_ty.clone(), c.clone(), InstUtil::extractComment(dae.elementLst.clone())?)?;
            (cache.clone(), env_3.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), tys.clone(), ci_state_1.clone())
        },
        (_, _, _, _, _, _, Deref @ SCode::Element::CLASS { .. }, _, _, _, _) => {
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outTypeVars, outState))
}

pub fn instClassIn(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inClass: Arc<SCode::Element>, mut inVisibility: SCode::Visibility, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut implicitInstantiation: bool, mut inCallingScope: InstTypes::CallingScope, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets, mut instSingleCref: Option<Arc<DAE::ComponentRef>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, Option<Arc<DAE::Type>>, Option<SCode::Attributes>, Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outType: Option<Arc<DAE::Type>> = None;
    let mut optDerAttr: Option<SCode::Attributes> = None;
    let mut outEqualityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outState, outVars, outType, optDerAttr, outEqualityConstraint, outGraph) = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } => {
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut env: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut dae: DAE::DAElist;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut equalityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy = metamodelica::nil();
                    let mut store: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
                    let true = (boolOr(AbsynUtil::isNotInnerOuter(io.clone()), AbsynUtil::isOnlyInner(io.clone()))) else { bail!("pattern mismatch") };
                    (cache, env, ih, store, ci_state, graph, csets, dae, tys, bc, oDA, equalityConstraint) = instClassIn2(inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), inClass.clone(), inVisibility.clone(), inInstDims.clone(), implicitInstantiation.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone(), instSingleCref.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), oDA.clone(), equalityConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, encapsulatedPrefix: encflag, restriction: r, name: n, .. } => {
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut env: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut c: Arc<SCode::Element>;
                    let mut dae: DAE::DAElist;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut equalityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy = metamodelica::nil();
                    let mut store: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
                    let mut n = (*n).clone();
                    let true = (boolOr(AbsynUtil::isInnerOuter(io.clone()), AbsynUtil::isOnlyOuter(io.clone()))) else { bail!("pattern mismatch") };
                    let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: __pa0 }, .. } = (FNode::refData(FGraph::lastScopeRef(inEnv.clone())?)?) else { bail!("pattern mismatch") };
                    n = __pa0.clone();
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    env = FGraph::openScope(env.clone(), encflag.clone(), (n.clone()).clone(), FGraph::restrictionToScopeType(r.clone()))?;
                    ci_state = ClassInfUtil::start(r.clone(), FGraph::getGraphName(env.clone())?)?;
                    let InnerOuter::INST_INNER { innerElement: Some(__pa1), .. } = (InnerOuter::lookupInnerVar(inCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), (n.clone()).clone(), io.clone())?) else { bail!("pattern mismatch") };
                    c = __pa1.clone();
                    (cache, env, ih, store, ci_state, graph, csets, dae, tys, bc, oDA, equalityConstraint) = instClassIn2(inCache.clone(), env.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), ci_state.clone(), c.clone(), inVisibility.clone(), inInstDims.clone(), implicitInstantiation.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone(), instSingleCref.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), oDA.clone(), equalityConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, name: n, .. } => {
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut env: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut c: Arc<SCode::Element>;
                    let mut dae: DAE::DAElist;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut equalityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy = metamodelica::nil();
                    let mut store: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
                    let mut n = (*n).clone();
                    let true = (boolOr(AbsynUtil::isInnerOuter(io.clone()), AbsynUtil::isOnlyOuter(io.clone()))) else { bail!("pattern mismatch") };
                    n = (FGraph::getInstanceOriginalName(inEnv.clone(), (n.clone()).clone())?).clone();
                    let InnerOuter::INST_INNER { innerElement: Some(__pa0), .. } = (InnerOuter::lookupInnerVar(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), (n.clone()).clone(), io.clone())?) else { bail!("pattern mismatch") };
                    c = __pa0.clone();
                    (cache, env, ih, store, ci_state, graph, csets, dae, tys, bc, oDA, equalityConstraint) = instClassIn2(inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), c.clone(), inVisibility.clone(), inInstDims.clone(), implicitInstantiation.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone(), instSingleCref.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), oDA.clone(), equalityConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { info, prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, name: n, .. } => {
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut env: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut dae: DAE::DAElist;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut equalityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut store: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
                    let true = (boolOr(AbsynUtil::isInnerOuter(io.clone()), AbsynUtil::isOnlyOuter(io.clone()))) else { bail!("pattern mismatch") };
                    if !(Config::getGraphicsExpMode()?) {
                        s1 = (n.clone()).clone();
                        s2 = (AbsynUtil::innerOuterStr(io.clone())?).clone();
                        Error::addSourceMessage(Error::MISSING_INNER_CLASS.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (literal!("")).clone()], info.clone())?;
                    }
                    (cache, env, ih, store, ci_state, graph, csets, dae, tys, bc, oDA, equalityConstraint) = instClassIn2(inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), inClass.clone(), inVisibility.clone(), inInstDims.clone(), implicitInstantiation.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone(), instSingleCref.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), oDA.clone(), equalityConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outVars, outType, optDerAttr, outEqualityConstraint, outGraph))
}

pub fn instClassIn2(mut cache: FCore::Cache, mut env: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut store: UnitAbsyn::InstStore, mut r#mod: Arc<DAE::Mod>, mut prefix: DAE::Prefix, mut state: ClassInf::State, mut cls: Arc<SCode::Element>, mut visibility: SCode::Visibility, mut instDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut implicitInst: bool, mut callingScope: InstTypes::CallingScope, mut graph: ConnectionGraph::ConnectionGraph, mut sets: DAE::Connect::Sets, mut instSingleCref: Option<Arc<DAE::ComponentRef>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, ClassInf::State, ConnectionGraph::ConnectionGraph, DAE::Connect::Sets, DAE::DAElist, Arc<metamodelica::List<Arc<DAE::Var>>>, Option<Arc<DAE::Type>>, Option<SCode::Attributes>, Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>> = ih;
    let mut store: UnitAbsyn::InstStore = store;
    let mut state: ClassInf::State = state;
    let mut graph: ConnectionGraph::ConnectionGraph = graph;
    let mut sets: DAE::Connect::Sets = sets;
    let mut dae: DAE::DAElist;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut ty: Option<Arc<DAE::Type>> = None;
    let mut optDerAttr: Option<SCode::Attributes> = None;
    let mut equalityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut cache_path: Arc<Absyn::Path>;
    let mut inputs: (Arc<DAE::Mod>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Arc<SCode::Element>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, bool, Option<Arc<DAE::ComponentRef>>, InstTypes::CallingScope);
    let mut outputs: (FCore::Graph, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, Option<Arc<DAE::Type>>, Option<SCode::Attributes>, Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, ConnectionGraph::ConnectionGraph);
    let mut bbx: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, bool, Arc<DAE::Mod>, DAE::Connect::Sets, ClassInf::State, Arc<SCode::Element>, Option<Arc<DAE::ComponentRef>>);
    let mut bby: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, bool, Arc<DAE::Mod>, DAE::Connect::Sets, ClassInf::State, Arc<SCode::Element>, Option<Arc<DAE::ComponentRef>>);
    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut pre: DAE::Prefix = DAE::Prefix::NOPRE;
    let mut csets: DAE::Connect::Sets;
    let mut st: ClassInf::State;
    let mut e: Arc<SCode::Element>;
    let mut dims: InstDims = metamodelica::nil();
    let mut r#impl: bool = false;
    let mut scr: Option<Arc<DAE::ComponentRef>> = None;
    let mut cs: InstTypes::CallingScope = InstTypes::CallingScope::INNER_CALL;
    let mut cached_graph: ConnectionGraph::ConnectionGraph;
    if SCodeUtil::isPackage(cls.clone()) && SCodeUtil::isPartial(cls.clone()) {
        (cache, env, ih, state, _) = partialInstClassIn(cache.clone(), env.clone(), ih.clone(), r#mod.clone(), prefix.clone(), state.clone(), cls.clone(), visibility.clone(), instDims.clone(), 0)?;
        dae = DAE::emptyDae().clone();
        vars = metamodelica::nil();
        ty = None;
        optDerAttr = None;
        equalityConstraint = None;
        return Ok((cache, env, ih, store, state, graph, sets, dae, vars, ty, optDerAttr, equalityConstraint));
    }
    cache_path = generateCachePath(env.clone(), cls.clone(), prefix.clone(), callingScope.clone())?;
    if Flags::isSet(Flags::CACHE.clone())? {
        if '__try0: {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(InstHashTable::get(cache_path.clone()), '__try0)) {
                Deref @ metamodelica::List::Cons { head: Some(InstHashTable::CachedInstItem::FUNC_instClassIn { inputs: __pa1, outputs: __pa2 }), tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            inputs = __pa1.clone();
            outputs = __pa2.clone();
            let (__pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(inputs.clone()) {
                (__pa4, __pa5, __pa6, __pa7, __pa8 @ Deref @ SCode::Element::CLASS { .. }, __pa9, __pa10, __pa11, __pa12) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            m = __pa4.clone();
            pre = __pa5.clone();
            csets = __pa6.clone();
            st = __pa7.clone();
            e = __pa8.clone();
            dims = __pa9.clone();
            r#impl = __pa10.clone();
            scr = __pa11.clone();
            cs = __pa12.clone();
            unwrap_break_err!(InstUtil::prefixEqualUnlessBasicType(prefix.clone(), pre.clone(), cls.clone()), '__try0);
            if dims.clone() == instDims.clone() && r#impl.clone() == implicitInst.clone() && m.clone() == r#mod.clone() && csets.clone() == sets.clone() && st.clone() == state.clone() && e.clone() == cls.clone() && scr.clone() == instSingleCref.clone() && callingScopeCacheEq(cs.clone(), callingScope.clone()) {
                (env, dae, sets, state, vars, ty, optDerAttr, equalityConstraint, cached_graph) = outputs.clone();
                graph = unwrap_break_err!(ConnectionGraph::merge(graph.clone(), cached_graph.clone()), '__try0);
                unwrap_break_err!(showCacheInfo((literal!("Full Inst Hit: ")).clone(), cache_path.clone()), '__try0);
                return Ok((cache, env, ih, store, state, graph, sets, dae, vars, ty, optDerAttr, equalityConstraint));
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    match '__try13: {
        inputs = (r#mod.clone(), prefix.clone(), sets.clone(), state.clone(), cls.clone(), instDims.clone(), implicitInst.clone(), instSingleCref.clone(), callingScope.clone());
        (cache, env, ih, store, dae, sets, state, vars, ty, optDerAttr, equalityConstraint, graph) = unwrap_break_err!(instClassIn_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), r#mod.clone(), prefix.clone(), state.clone(), cls.clone(), visibility.clone(), instDims.clone(), implicitInst.clone(), callingScope.clone(), graph.clone(), sets.clone(), instSingleCref.clone()), '__try13);
        outputs = (env.clone(), dae.clone(), sets.clone(), state.clone(), vars.clone(), ty.clone(), optDerAttr.clone(), equalityConstraint.clone(), graph.clone());
        unwrap_break_err!(showCacheInfo((literal!("Full Inst Add: ")).clone(), cache_path.clone()), '__try13);
        unwrap_break_err!(InstHashTable::addToInstCache(cache_path.clone(), Some(InstHashTable::CachedInstItem::FUNC_instClassIn { inputs: inputs.clone(), outputs: outputs.clone() }), None), '__try13);
        Ok::<_, anyhow::Error>((cache.clone(), dae.clone(), env.clone(), equalityConstraint.clone(), graph.clone(), ih.clone(), inputs.clone(), optDerAttr.clone(), outputs.clone(), sets.clone(), state.clone(), store.clone(), ty.clone(), vars.clone()))
    } {
        Ok((__try13_o0, __try13_o1, __try13_o2, __try13_o3, __try13_o4, __try13_o5, __try13_o6, __try13_o7, __try13_o8, __try13_o9, __try13_o10, __try13_o11, __try13_o12, __try13_o13)) => {
            cache = __try13_o0;
            dae = __try13_o1;
            env = __try13_o2;
            equalityConstraint = __try13_o3;
            graph = __try13_o4;
            ih = __try13_o5;
            inputs = __try13_o6;
            optDerAttr = __try13_o7;
            outputs = __try13_o8;
            sets = __try13_o9;
            state = __try13_o10;
            store = __try13_o11;
            ty = __try13_o12;
            vars = __try13_o13;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instClassIn2 failed on class: ")); __mm_s.push_str(&*SCodeUtil::elementName(cls.clone())?); __mm_s.push_str(&*literal!(" in environment: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail");
        }
    }
    Ok((cache, env, ih, store, state, graph, sets, dae, vars, ty, optDerAttr, equalityConstraint))
}

fn markDerivedRecordOutsideBindings(mut inType: Arc<DAE::Type>, mut inClass: Arc<SCode::Element>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut derMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    if !(SCodeUtil::isRecord(inClass.clone())) || !(SCodeUtil::isDerivedClass(inClass.clone())) {
        outType = inType.clone();
        return Ok(outType);
    }
    derMod = SCodeUtil::getDerivedMod(inClass.clone())?;
    if SCodeUtil::isEmptyMod(derMod.clone()) {
        outType = inType.clone();
        return Ok(outType);
    }
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(derMod.clone()) {
            Deref @ SCode::Mod::MOD { subModLst: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        submods = __pa1.clone();
        Ok::<_, anyhow::Error>((submods.clone(),))
    } {
        Ok((__try0_o0,)) => {
            submods = __try0_o0;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Unexpected Mod structure in collectAndFixDerivedComplexOutsideBindings.")).clone()])?;
            bail!("fail");
        }
    }
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            let mut tvars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            tvars = metamodelica::nil();
            for mut var in &*var_field!((*inType).varLst, DAE::Type::T_COMPLEX).clone() {
                let mut var = var.clone();
                for mut submod in &*submods.clone() {
                    let mut submod = submod.clone();
                    if varIsModifiedInDerivedMod((var.name.clone()).clone(), submod.clone())? {
                        assign_field!(
                            var.bind_from_outside = true,
                            var.binding = markBindingFromDerivedRecordMods(var.binding.clone())
                        );
                        break;
                    }
                }
                tvars = cons(var.clone(), tvars.clone());
            }
            tvars = tvars.clone().reverse();
            Arc::new(DAE::Type::T_COMPLEX { complexClassType: var_field!((*inType).complexClassType, DAE::Type::T_COMPLEX).clone(), varLst: tvars.clone(), equalityConstraint: var_field!((*inType).equalityConstraint, DAE::Type::T_COMPLEX).clone(), usedExternally: var_field!((*inType).usedExternally, DAE::Type::T_COMPLEX).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

fn markBindingFromDerivedRecordMods(mut bind: Arc<DAE::Binding>) -> Arc<DAE::Binding> {
    let mut bind: Arc<DAE::Binding> = bind;
    let _ = (::match_deref::match_deref! { match &(bind.clone()) {
        Deref @ DAE::Binding::EQBOUND { .. } => {
            assign_variant_field!(bind => DAE::Binding::EQBOUND; source = openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DERIVED_RECORD_DECL);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    bind
}

fn varIsModifiedInDerivedMod(mut inName: ArcStr, mut inSubmod: Arc<SCode::SubMod>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inSubmod.clone()) {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::REDECL { .. }, .. } => false,
        Deref @ SCode::SubMod { .. } => stringEqual((inSubmod.ident.clone()).clone(), (inName.clone()).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

fn markTypesVarsOutsideBindings(mut inType: Arc<DAE::Type>, mut inMod: Arc<DAE::Mod>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = inType.clone();
    let mut submods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    if !(Types::isRecord(inType.clone())) {
        return Ok(outType);
    }
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(inMod.clone()) {
            Deref @ DAE::Mod::MOD { subModLst: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        submods = __pa1.clone();
        Ok::<_, anyhow::Error>((submods.clone(),))
    } {
        Ok((__try0_o0,)) => {
            submods = __try0_o0;
        }
        Err(_) => {
            return Ok(outType);
        }
    }
    if submods.clone().is_empty() {
        return Ok(outType);
    }
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            let mut tvars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            tvars = metamodelica::nil();
            for mut var in &*var_field!((*inType).varLst, DAE::Type::T_COMPLEX).clone() {
                let mut var = var.clone();
                for mut submod in &*submods.clone() {
                    let mut submod = submod.clone();
                    if varIsModifiedInMod((var.name.clone()).clone(), submod.clone())? {
                        assign_field!(var.bind_from_outside = true);
                        break;
                    }
                }
                tvars = cons(var.clone(), tvars.clone());
            }
            tvars = tvars.clone().reverse();
            Arc::new(DAE::Type::T_COMPLEX { complexClassType: var_field!((*inType).complexClassType, DAE::Type::T_COMPLEX).clone(), varLst: tvars.clone(), equalityConstraint: var_field!((*inType).equalityConstraint, DAE::Type::T_COMPLEX).clone(), usedExternally: var_field!((*inType).usedExternally, DAE::Type::T_COMPLEX).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

fn varIsModifiedInMod(mut inName: ArcStr, mut inSubmod: Arc<DAE::SubMod>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inSubmod.clone()) {
        Deref @ DAE::SubMod { .. } => stringEqual((inSubmod.ident.clone()).clone(), (inName.clone()).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

fn callingScopeCacheEq(mut inCallingScope1: InstTypes::CallingScope, mut inCallingScope2: InstTypes::CallingScope) -> bool {
    let mut outIsEq: bool = false;
    outIsEq = (match (inCallingScope1.clone(), inCallingScope2.clone()) {
        (InstTypes::CallingScope::TYPE_CALL, InstTypes::CallingScope::TYPE_CALL) => true,
        (InstTypes::CallingScope::TYPE_CALL, _) => false,
        (_, InstTypes::CallingScope::TYPE_CALL) => false,
        _ => true,
    });
    outIsEq
}

pub fn instClassIn_dispatch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inClass: Arc<SCode::Element>, mut inVisibility: SCode::Visibility, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut implicitInstantiation: bool, mut inCallingScope: InstTypes::CallingScope, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets, mut instSingleCref: Option<Arc<DAE::ComponentRef>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, Option<Arc<DAE::Type>>, Option<SCode::Attributes>, Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outTypesTypeOption: Option<Arc<DAE::Type>> = None;
    let mut optDerAttr: Option<SCode::Attributes> = None;
    let mut outEqualityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, outTypesTypeOption, optDerAttr, outEqualityConstraint, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), inClass.clone(), inVisibility.clone(), inInstDims.clone(), implicitInstantiation.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone(), instSingleCref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, Deref @ SCode::Element::CLASS { name: n, .. }, _, inst_dims, _, _, graph, _, _) => {
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut typer: BasicTypeAttrTyper;
                    ty = getBasicTypeType((n.clone()).clone())?;
                    typer = getBasicTypeAttrTyper((n.clone()).clone())?;
                    ty = liftNonExpType(ty.clone(), inst_dims.clone(), Config::splitArrays()?);
                    tys = instBasicTypeAttributes(cache.clone(), env.clone(), mods.clone(), ty.clone(), typer.clone(), pre.clone())?;
                    ty = Types::setTypeVars(ty.clone(), tys.clone())?;
                    bc = arrayBasictypeBaseclass(inst_dims.clone(), ty.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), inSets.clone(), ci_state.clone(), tys.clone(), bc.clone(), None, None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, c @ Deref @ SCode::Element::CLASS { info, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, restriction: SCode::Restriction::R_ENUMERATION, name: n, .. }, _, inst_dims, r#impl, callscope, graph, _, _) => {
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut env_1: FCore::Graph;
                    let mut ci_state_1: ClassInf::State;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut env_2: FCore::Graph;
                    let mut env_3: FCore::Graph;
                    let mut comp: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut fq_class: Arc<Absyn::Path>;
                    let mut tys1: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut tys2: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    names = SCodeUtil::componentNames(c.clone());
                    Types::checkEnumDuplicateLiterals(names.clone(), info.clone())?;
                    tys = instBasicTypeAttributes(cache.clone(), env.clone(), mods.clone(), DAE::T_ENUMERATION_DEFAULT().clone(), (std::sync::Arc::new(getEnumAttributeType) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>), pre.clone())?;
                    ci_state_1 = ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::NEWDEF)?;
                    comp = InstUtil::addNomod(els.clone());
                    (cache, env_1, ih) = InstUtil::addComponentsToEnv(cache.clone(), env.clone(), ih.clone(), mods.clone(), pre.clone(), ci_state_1.clone(), comp.clone(), r#impl.clone())?;
                    (cache, env_2, ih, store, _, csets, ci_state_1, tys1, graph, _) = instElementList(cache.clone(), env_1.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state_1.clone(), comp.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), true)?;
                    (cache, fq_class) = makeFullyQualifiedIdent(cache.clone(), env_2.clone(), (n.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    eqConstraint = InstUtil::equalityConstraint(env_2.clone(), els.clone(), info.clone());
                    ty2 = Arc::new(DAE::Type::T_ENUMERATION { index: None, path: fq_class.clone(), names: names.clone(), literalVarLst: tys1.clone(), attributeLst: tys.clone() });
                    bc = arrayBasictypeBaseclass(inst_dims.clone(), ty2.clone())?;
                    bc = if (isSome(bc.clone())) {bc.clone()} else {Some(ty2.clone())};
                    ty = InstUtil::mktype(fq_class.clone(), ci_state_1.clone(), tys1.clone(), bc.clone(), eqConstraint.clone(), c.clone(), SCode::noComment.clone())?;
                    (cache, env_3) = InstUtil::updateEnumerationEnvironment(cache.clone(), env_2.clone(), ty.clone(), c.clone(), ci_state_1.clone())?;
                    tys2 = listAppend(tys.clone(), tys1.clone());
                    Ok((cache.clone(), env_3.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state_1.clone(), tys2.clone(), bc.clone(), None, None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, c @ Deref @ SCode::Element::CLASS { encapsulatedPrefix, partialPrefix, info, cmt: comment, classDef: d, restriction: r, name: n, .. }, vis, inst_dims, r#impl, callscope, graph, _, _) => {
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut env_1: FCore::Graph;
                    let mut ci_state_1: ClassInf::State;
                    let mut csets: DAE::Connect::Sets;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut dae: DAE::DAElist;
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    ErrorExt::setCheckpoint((literal!("instClassParts")).clone());
                    let false = (InstUtil::isBuiltInClass((n.clone()).clone())) else { bail!("pattern mismatch") };
                    let _ = (match r.clone() {
        SCode::Restriction::R_ENUMERATION => bail!("fail"),
        _ => (),
    });
                    (cache, env_1, ih, store, dae, csets, ci_state_1, tys, bc, oDA, eqConstraint, graph) = instClassdef(cache.clone(), env.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state.clone(), (n.clone()).clone(), d.clone(), r.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), comment.clone(), info.clone())?;
                    dae = if (SCodeUtil::isFunction(c.clone()) && !(r#impl.clone())) {DAE::DAElist { elementLst: metamodelica::nil() }} else {dae.clone()};
                    ErrorExt::delCheckpoint((literal!("instClassParts")).clone());
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state_1.clone(), tys.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, _, _, ci_state, c @ Deref @ SCode::Element::CLASS { .. }, _, _, r#impl, _, graph, _, _) => {
                    let mut b: bool = false;
                    b = Flags::getConfigBool(Flags::CHECK_MODEL.clone())? && !(r#impl.clone()) && SCodeUtil::isFunction(c.clone());
                    if !(b.clone()) {
                        ErrorExt::delCheckpoint((literal!("instClassParts")).clone());
                        bail!("fail");
                    } else {
                        ErrorExt::rollBack((literal!("instClassParts")).clone());
                    }
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), inSets.clone(), ci_state.clone(), metamodelica::nil(), None, None, None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, outTypesTypeOption, optDerAttr, outEqualityConstraint, outGraph))
}

fn liftNonExpType(mut inType: Arc<DAE::Type>, mut inInstDims: InstDims, mut inSplitArrays: bool) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inType.clone(), inInstDims.clone(), inSplitArrays.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: dims, tail: _ }, false) => {
            Types::liftArrayListDims(inType.clone(), dims.clone())
        },
        _ => {
            inType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

fn getBasicTypeType(mut inName: ArcStr) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inName.clone()) {
        Deref @ "Real" => DAE::T_REAL_DEFAULT().clone(),
        Deref @ "Integer" => DAE::T_INTEGER_DEFAULT().clone(),
        Deref @ "String" => DAE::T_STRING_DEFAULT().clone(),
        Deref @ "Boolean" => DAE::T_BOOL_DEFAULT().clone(),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            DAE::T_CLOCK_DEFAULT().clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

fn getBasicTypeAttrTyper(mut inName: ArcStr) -> Result<BasicTypeAttrTyper> {
    let mut outTyper: BasicTypeAttrTyper;
    outTyper = (::match_deref::match_deref! { match &(inName.clone()) {
        Deref @ "Real" => (std::sync::Arc::new(getRealAttributeType) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>),
        Deref @ "Integer" => (std::sync::Arc::new(getIntAttributeType) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>),
        Deref @ "String" => (std::sync::Arc::new(getStringAttributeType) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>),
        Deref @ "Boolean" => (std::sync::Arc::new(getBoolAttributeType) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            (std::sync::Arc::new(getClockAttributeType) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTyper)
}

fn getRealAttributeType(mut inAttrName: ArcStr, mut inBaseType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inAttrName.clone(), inBaseType.clone(), inInfo.clone())) {
        (Deref @ "quantity", _, _) => DAE::T_STRING_DEFAULT().clone(),
        (Deref @ "unit", _, _) => DAE::T_STRING_DEFAULT().clone(),
        (Deref @ "displayUnit", _, _) => DAE::T_STRING_DEFAULT().clone(),
        (Deref @ "min", _, _) => inBaseType.clone(),
        (Deref @ "max", _, _) => inBaseType.clone(),
        (Deref @ "start", _, _) => inBaseType.clone(),
        (Deref @ "fixed", _, _) => DAE::T_BOOL_DEFAULT().clone(),
        (Deref @ "nominal", _, _) => inBaseType.clone(),
        (Deref @ "stateSelect", _, _) => InstBinding::stateSelectType().clone(),
        (Deref @ "uncertain", _, _) => InstBinding::uncertaintyType().clone(),
        (Deref @ "distribution", _, _) => InstBinding::distributionType().clone(),
        _ => {
            Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(inAttrName.clone()).clone(), (literal!("Real")).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getIntAttributeType(mut inAttrName: ArcStr, mut inBaseType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inAttrName.clone(), inBaseType.clone(), inInfo.clone())) {
        (Deref @ "quantity", _, _) => DAE::T_STRING_DEFAULT().clone(),
        (Deref @ "min", _, _) => inBaseType.clone(),
        (Deref @ "max", _, _) => inBaseType.clone(),
        (Deref @ "start", _, _) => inBaseType.clone(),
        (Deref @ "fixed", _, _) => DAE::T_BOOL_DEFAULT().clone(),
        (Deref @ "nominal", _, _) => inBaseType.clone(),
        (Deref @ "uncertain", _, _) => InstBinding::uncertaintyType().clone(),
        (Deref @ "distribution", _, _) => InstBinding::distributionType().clone(),
        _ => {
            Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(inAttrName.clone()).clone(), (literal!("Integer")).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getStringAttributeType(mut inAttrName: ArcStr, mut inBaseType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inAttrName.clone(), inBaseType.clone(), inInfo.clone())) {
        (Deref @ "quantity", _, _) => DAE::T_STRING_DEFAULT().clone(),
        (Deref @ "start", _, _) => inBaseType.clone(),
        _ => {
            Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(inAttrName.clone()).clone(), (literal!("String")).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getBoolAttributeType(mut inAttrName: ArcStr, mut inBaseType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inAttrName.clone(), inBaseType.clone(), inInfo.clone())) {
        (Deref @ "quantity", _, _) => DAE::T_STRING_DEFAULT().clone(),
        (Deref @ "start", _, _) => inBaseType.clone(),
        (Deref @ "fixed", _, _) => DAE::T_BOOL_DEFAULT().clone(),
        _ => {
            Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(inAttrName.clone()).clone(), (literal!("Boolean")).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getClockAttributeType(mut inAttrName: ArcStr, mut inBaseType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inAttrName.clone(), inBaseType.clone(), inInfo.clone())) {
        (_, _, _) => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getEnumAttributeType(mut inAttrName: ArcStr, mut inBaseType: Arc<DAE::Type>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inAttrName.clone(), inBaseType.clone(), inInfo.clone())) {
        (Deref @ "quantity", _, _) => DAE::T_STRING_DEFAULT().clone(),
        (Deref @ "min", _, _) => inBaseType.clone(),
        (Deref @ "max", _, _) => inBaseType.clone(),
        (Deref @ "start", _, _) => inBaseType.clone(),
        (Deref @ "fixed", _, _) => DAE::T_BOOL_DEFAULT().clone(),
        _ => {
            Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(inAttrName.clone()).clone(), (literal!("enumeration(:)")).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn instBasicTypeAttributes(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inMod: Arc<DAE::Mod>, mut inBaseType: Arc<DAE::Type>, mut inTypeFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>, mut inPrefix: DAE::Prefix) -> Result<Arc<metamodelica::List<Arc<DAE::Var>>>> {
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    outVars = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::MOD { subModLst: submods, .. } => {
            List::map4(submods.clone(), (std::sync::Arc::new(instBasicTypeAttributes2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>, FCore::Cache, FCore::Graph, Arc<DAE::Type>, BasicTypeAttrTyper) -> Result<Arc<DAE::Var>> + 'static>), inCache.clone(), inEnv.clone(), inBaseType.clone(), inTypeFunc.clone())
        },
        Deref @ DAE::Mod::NOMOD => {
            metamodelica::nil()
        },
        Deref @ DAE::Mod::REDECL { .. } => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVars)
}

fn instBasicTypeAttributes2(mut inSubMod: Arc<DAE::SubMod>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inBaseType: Arc<DAE::Type>, mut inTypeFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var>;
    outVar = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { info, binding: Some(DAE::EqMod::TYPED { properties: p, modifierAsValue: val, modifierAsExp: exp, .. }), .. }, ident: name } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ty = getRealAttributeType((name.clone()).clone(), inBaseType.clone(), info.clone())?;
            instBuiltinAttribute(inCache.clone(), inEnv.clone(), (name.clone()).clone(), val.clone(), exp.clone(), ty.clone(), p.clone())?
        },
        Deref @ DAE::SubMod { ident: name, .. } => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instBasicTypeAttributes2 failed on ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

fn instBuiltinAttribute(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut id: ArcStr, mut optVal: Option<Arc<Values::Value>>, mut bind: Arc<DAE::Exp>, mut inExpectedTp: Arc<DAE::Type>, mut bindProp: DAE::Properties) -> Result<Arc<DAE::Var>> {
    let mut var: Arc<DAE::Var>;
    var = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), id.clone(), optVal.clone(), bind.clone(), inExpectedTp.clone(), bindProp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(v), _, expectedTp, DAE::Properties::PROP { type_: bindTp, constFlag: c }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind1: Arc<DAE::Exp>;
                    let mut vbind: Arc<DAE::Exp>;
                    let mut v = (*v).clone();
                    let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { bail!("pattern mismatch") };
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    (vbind, _) = Types::matchType(ValuesUtil::valueExp(v.clone(), None)?, bindTp.clone(), expectedTp.clone(), true)?;
                    v = ValuesUtil::expValue(vbind.clone())?;
                    Ok(Arc::new(DAE::Var { name: (id.clone()).clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: Arc::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: Some(v.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(v), _, expectedTp, DAE::Properties::PROP { type_: bindTp @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil }, .. }, constFlag: c }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind1: Arc<DAE::Exp>;
                    let mut vbind: Arc<DAE::Exp>;
                    let mut v = (*v).clone();
                    let mut expectedTp = (*expectedTp).clone();
                    let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { bail!("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    expectedTp = Types::liftArray(expectedTp.clone(), d.clone());
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    (vbind, _) = Types::matchType(ValuesUtil::valueExp(v.clone(), None)?, bindTp.clone(), expectedTp.clone(), true)?;
                    v = ValuesUtil::expValue(vbind.clone())?;
                    Ok(Arc::new(DAE::Var { name: (id.clone()).clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: Arc::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: Some(v.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _, expectedTp, DAE::Properties::PROP { type_: bindTp, constFlag: c }) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind1: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { bail!("pattern mismatch") };
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    (cache, v) = Ceval::ceval(cache.clone(), env.clone(), bind1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    Ok(Arc::new(DAE::Var { name: (id.clone()).clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: Arc::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: Some(v.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _, expectedTp, DAE::Properties::PROP { type_: bindTp @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil }, .. }, constFlag: c }) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind1: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    let mut expectedTp = (*expectedTp).clone();
                    let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { bail!("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    expectedTp = Types::liftArray(expectedTp.clone(), d.clone());
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    (cache, v) = Ceval::ceval(cache.clone(), env.clone(), bind1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    Ok(Arc::new(DAE::Var { name: (id.clone()).clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: Arc::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: Some(v.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, expectedTp, DAE::Properties::PROP { type_: bindTp, constFlag: c }) => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut bind1: Arc<DAE::Exp>;
                    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
                        let true = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { bail!("pattern mismatch") };
                    } else {
                        let false = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { bail!("pattern mismatch") };
                    }
                    (bind1, t_1) = Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true)?;
                    Ok(Arc::new(DAE::Var { name: (id.clone()).clone(), attributes: DAE::dummyAttrParam().clone(), ty: t_1.clone(), binding: Arc::new(DAE::Binding::EQBOUND { exp: bind1.clone(), evaluatedExp: None, constant_: openmodelica_frontend_types::DAE::Const::C_PARAM, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, DAE::Properties::PROP { type_: _, constFlag: c }) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (c.clone() == openmodelica_frontend_types::DAE::Const::C_VAR) else { bail!("pattern mismatch") };
                    s = (ExpressionBasics::printExpStr(bind.clone())?).clone();
                    Error::addMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![(id.clone()).clone(), (literal!("PARAM")).clone(), (s.clone()).clone(), (literal!("VAR")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, expectedTp, DAE::Properties::PROP { type_: bindTp, constFlag: _ }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        (_, _) = unwrap_break_err!(Types::matchType(bind.clone(), bindTp.clone(), expectedTp.clone(), true), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("builtin attribute ")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" of type ")); __mm_s.push_str(&*TypesDump::unparseType(bindTp.clone())?); ArcStr::from(__mm_s) }).clone();
                    s2 = (TypesDump::unparseType(expectedTp.clone())?).clone();
                    Error::addMessage(Error::TYPE_ERROR.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(v), _, expectedTp, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("instBuiltinAttribute failed for: ")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" value binding: ")); __mm_s.push_str(&*ValuesDump::printValStr(v.clone())?); __mm_s.push_str(&*literal!(" binding: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(bind.clone())?); __mm_s.push_str(&*literal!(" expected type: ")); __mm_s.push_str(&*TypesDump::printTypeStr(expectedTp.clone())?); __mm_s.push_str(&*literal!(" type props: ")); __mm_s.push_str(&*Types::printPropStr(bindProp.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, expectedTp, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("instBuiltinAttribute failed for: ")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(" value binding: NONE()")); __mm_s.push_str(&*literal!(" binding: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(bind.clone())?); __mm_s.push_str(&*literal!(" expected type: ")); __mm_s.push_str(&*TypesDump::printTypeStr(expectedTp.clone())?); __mm_s.push_str(&*literal!(" type props: ")); __mm_s.push_str(&*Types::printPropStr(bindProp.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(var)
}

fn arrayBasictypeBaseclass(mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inType: Arc<DAE::Type>) -> Result<Option<Arc<DAE::Type>>> {
    let mut outOptType: Option<Arc<DAE::Type>> = None;
    outOptType = (::match_deref::match_deref! { match &((inInstDims.clone(), inType.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            None
        },
        _ => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            dims = List::last(inInstDims.clone())?;
            ty = Expression::liftArrayLeftList(inType.clone(), dims.clone());
            Some(ty.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOptType)
}

pub fn partialInstClassIn(mut cache: FCore::Cache, mut env: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut r#mod: Arc<DAE::Mod>, mut prefix: DAE::Prefix, mut state: ClassInf::State, mut cls: Arc<SCode::Element>, mut visibility: SCode::Visibility, mut instDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut numIter: i32) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>> = ih;
    let mut state: ClassInf::State = state;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut cache_path: Arc<Absyn::Path>;
    let mut inputs: (Arc<DAE::Mod>, DAE::Prefix, ClassInf::State, Arc<SCode::Element>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>);
    let mut outputs: (FCore::Graph, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>);
    let mut bbx: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, Arc<DAE::Mod>, ClassInf::State, Arc<SCode::Element>);
    let mut bby: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, Arc<DAE::Mod>, ClassInf::State, Arc<SCode::Element>);
    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut pre: DAE::Prefix = DAE::Prefix::NOPRE;
    let mut st: ClassInf::State;
    let mut e: Arc<SCode::Element>;
    let mut dims: InstDims = metamodelica::nil();
    let mut partial_inst: bool = false;
    cache_path = generateCachePath(env.clone(), cls.clone(), prefix.clone(), crate::InstTypes::CallingScope::INNER_CALL)?;
    if Flags::isSet(Flags::CACHE.clone())? {
        if '__try0: {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(InstHashTable::get(cache_path.clone()), '__try0)) {
                Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: Some(InstHashTable::CachedInstItem::FUNC_partialInstClassIn { inputs: __pa1, outputs: __pa2 }), tail: Deref @ metamodelica::List::Nil } } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            inputs = __pa1.clone();
            outputs = __pa2.clone();
            let (__pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(inputs.clone()) {
                (__pa4, __pa5, __pa6, __pa7 @ Deref @ SCode::Element::CLASS { .. }, __pa8) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            m = __pa4.clone();
            pre = __pa5.clone();
            st = __pa6.clone();
            e = __pa7.clone();
            dims = __pa8.clone();
            unwrap_break_err!(InstUtil::prefixEqualUnlessBasicType(pre.clone(), prefix.clone(), cls.clone()), '__try0);
            if dims.clone() == instDims.clone() && m.clone() == r#mod.clone() && st.clone() == state.clone() && e.clone() == cls.clone() {
                (env, state, vars) = outputs.clone();
                unwrap_break_err!(showCacheInfo((literal!("Partial Inst Hit: ")).clone(), cache_path.clone()), '__try0);
                return Ok((cache, env, ih, state, vars));
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    if numIter.clone() >= Global::recursionDepthLimit.clone() {
        Error::addSourceMessage(Error::RECURSION_DEPTH_REACHED.clone(), list![ArcStr::from(::std::format!("{}", Global::recursionDepthLimit.clone())), (FGraph::printGraphPathStr(env.clone())?).clone()], SCodeUtil::elementInfo(cls.clone()))?;
        bail!("fail");
    }
    match '__try9: {
        partial_inst = System::getPartialInstantiation();
        System::setPartialInstantiation(true);
        inputs = (r#mod.clone(), prefix.clone(), state.clone(), cls.clone(), instDims.clone());
        (cache, env, ih, state, vars) = unwrap_break_err!(partialInstClassIn_dispatch(cache.clone(), env.clone(), ih.clone(), r#mod.clone(), prefix.clone(), state.clone(), cls.clone(), visibility.clone(), instDims.clone(), partial_inst.clone(), numIter.clone() + 1), '__try9);
        outputs = (env.clone(), state.clone(), vars.clone());
        unwrap_break_err!(showCacheInfo((literal!("Partial Inst Add: ")).clone(), cache_path.clone()), '__try9);
        unwrap_break_err!(InstHashTable::addToInstCache(cache_path.clone(), None, Some(InstHashTable::CachedInstItem::FUNC_partialInstClassIn { inputs: inputs.clone(), outputs: outputs.clone() })), '__try9);
        Ok::<_, anyhow::Error>((cache.clone(), env.clone(), ih.clone(), inputs.clone(), outputs.clone(), partial_inst.clone(), state.clone(), vars.clone()))
    } {
        Ok((__try9_o0, __try9_o1, __try9_o2, __try9_o3, __try9_o4, __try9_o5, __try9_o6, __try9_o7)) => {
            cache = __try9_o0;
            env = __try9_o1;
            ih = __try9_o2;
            inputs = __try9_o3;
            outputs = __try9_o4;
            partial_inst = __try9_o5;
            state = __try9_o6;
            vars = __try9_o7;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.partialInstClassIn failed on class: ")); __mm_s.push_str(&*SCodeUtil::elementName(cls.clone())?); __mm_s.push_str(&*literal!(" in environment: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail");
        }
    }
    Ok((cache, env, ih, state, vars))
}

fn partialInstClassIn_dispatch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inClass: Arc<SCode::Element>, mut inVisibility: SCode::Visibility, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut partialInst: bool, mut numIter: i32) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outState: ClassInf::State = inState.clone();
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut success: bool = false;
    success = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Real", .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Integer", .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "String", .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Boolean", .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Clock", .. } => {
                    if !((Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())? == 33)) { bail!("guard") }
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = outVars.clone();
                    let mut outState: ClassInf::State = outState.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    (outCache, outEnv, outIH, outState, outVars) = partialInstClassdef(inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), inClass.clone(), var_field!((*inClass).classDef, SCode::Element::CLASS).clone(), inVisibility.clone(), inInstDims.clone(), numIter.clone())?;
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
    System::setPartialInstantiation(partialInst.clone());
    if !(success.clone()) {
        bail!("fail");
    }
    Ok((outCache, outEnv, outIH, outState, outVars))
}

pub fn instClassdef(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut store: UnitAbsyn::InstStore, mut inMod2: Arc<DAE::Mod>, mut inPrefix3: DAE::Prefix, mut inState5: ClassInf::State, mut className: ArcStr, mut inClassDef6: Arc<SCode::ClassDef>, mut inRestriction7: SCode::Restriction, mut inVisibility: SCode::Visibility, mut inPartialPrefix: SCode::Partial, mut inEncapsulatedPrefix: SCode::Encapsulated, mut inInstDims9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImplicit: bool, mut inCallingScope: InstTypes::CallingScope, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets, mut instSingleCref: Option<Arc<DAE::ComponentRef>>, mut comment: Arc<SCode::Comment>, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, Option<Arc<DAE::Type>>, Option<SCode::Attributes>, Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outTypesTypeOption: Option<Arc<DAE::Type>> = None;
    let mut optDerAttr: Option<SCode::Attributes> = None;
    let mut outEqualityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, outTypesTypeOption, optDerAttr, outEqualityConstraint, outGraph) = instClassdef2(inCache.clone(), inEnv.clone(), inIH.clone(), store.clone(), inMod2.clone(), inPrefix3.clone(), inState5.clone(), (className.clone()).clone(), inClassDef6.clone(), inRestriction7.clone(), inVisibility.clone(), inPartialPrefix.clone(), inEncapsulatedPrefix.clone(), inInstDims9.clone(), inImplicit.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone(), instSingleCref.clone(), comment.clone(), info.clone(), Mutable::create(false))?;
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, outTypesTypeOption, optDerAttr, outEqualityConstraint, outGraph))
}

fn instClassdefBasicType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inMod2: Arc<DAE::Mod>, mut inPrefix3: DAE::Prefix, mut inState5: ClassInf::State, mut className: ArcStr, mut inClassDef6: Arc<SCode::ClassDef>, mut inRestriction7: SCode::Restriction, mut inVisibility: SCode::Visibility, mut inInstDims9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImplicit: bool, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets, mut instSingleCref: Option<Arc<DAE::ComponentRef>>, mut info: SourceInfo, mut stopInst: Mutable::Mutable<bool>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, Option<Arc<DAE::Type>>, Option<SCode::Attributes>, Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outTypesTypeOption: Option<Arc<DAE::Type>> = None;
    let mut optDerAttr: Option<SCode::Attributes> = None;
    let mut outEqualityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, outTypesTypeOption, optDerAttr, outEqualityConstraint, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod2.clone(), inPrefix3.clone(), inState5.clone(), className.clone(), inClassDef6.clone(), inRestriction7.clone(), inVisibility.clone(), inInstDims9.clone(), inImplicit.clone(), inGraph.clone(), inSets.clone(), instSingleCref.clone(), info.clone(), stopInst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, _, Deref @ SCode::ClassDef::PARTS { initialAlgorithmLst: Deref @ metamodelica::List::Nil, normalAlgorithmLst: Deref @ metamodelica::List::Nil, initialEquationLst: Deref @ metamodelica::List::Nil, normalEquationLst: Deref @ metamodelica::List::Nil, elementLst: els, .. }, _, _, inst_dims, r#impl, graph, _, _, _, _) => {
                    let mut cdefelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut compelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extendselts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut cdefelts_1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut cdefelts_2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut csets: DAE::Connect::Sets;
                    let mut dae1: DAE::DAElist;
                    let mut dae2: DAE::DAElist;
                    let mut dae: DAE::DAElist;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut mods = (*mods).clone();
                    let mut graph = (*graph).clone();
                    ErrorExt::setCheckpoint((literal!("instClassdefBasicType1")).clone());
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(InstUtil::splitElts(els.clone())?) {
                        (__pa0, Deref @ metamodelica::List::Nil, __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cdefelts = __pa0.clone();
                    extendselts = __pa1.clone();
                    compelts = __pa2.clone();
                    (cache, env1, ih) = InstUtil::addClassdefsToEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), cdefelts.clone(), r#impl.clone(), Some(mods.clone()), false)?;
                    cdefelts_1 = InstUtil::addNomod(cdefelts.clone());
                    env2 = env1.clone();
                    cdefelts_2 = cdefelts_1.clone();
                    (cache, env3, ih, store, dae1, csets, _, tys, graph, _) = instElementList(cache.clone(), env2.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state.clone(), cdefelts_2.clone(), inst_dims.clone(), r#impl.clone(), crate::InstTypes::CallingScope::INNER_CALL, graph.clone(), inSets.clone(), true)?;
                    mods = Mod::removeFirstSubsRedecl(mods.clone())?;
                    ErrorExt::rollBack((literal!("instClassdefBasicType1")).clone());
                    (cache, ih, store, dae2, bc, tys) = instBasictypeBaseclass(cache.clone(), env3.clone(), ih.clone(), store.clone(), extendselts.clone(), compelts.clone(), mods.clone(), inst_dims.clone(), (className.clone()).clone(), info.clone(), stopInst.clone())?;
                    eqConstraint = InstUtil::equalityConstraint(env3.clone(), els.clone(), info.clone());
                    dae = DAEUtil::joinDaes(dae1.clone(), dae2.clone())?;
                    Ok((cache.clone(), env3.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), tys.clone(), bc.clone(), None, eqConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, Deref @ SCode::ClassDef::PARTS { initialAlgorithmLst: Deref @ metamodelica::List::Nil, normalAlgorithmLst: Deref @ metamodelica::List::Nil, initialEquationLst: Deref @ metamodelica::List::Nil, normalEquationLst: Deref @ metamodelica::List::Nil, .. }, _, _, _, _, _, _, _, _, _) => {
                    let true = (ErrorExt::isTopCheckpoint((literal!("instClassdefBasicType1")).clone())) else { bail!("pattern mismatch") };
                    ErrorExt::rollBack((literal!("instClassdefBasicType1")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, outTypesTypeOption, optDerAttr, outEqualityConstraint, outGraph))
}

fn instClassdef2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inMod2: Arc<DAE::Mod>, mut inPrefix3: DAE::Prefix, mut inState5: ClassInf::State, mut className: ArcStr, mut inClassDef6: Arc<SCode::ClassDef>, mut inRestriction7: SCode::Restriction, mut inVisibility: SCode::Visibility, mut inPartialPrefix: SCode::Partial, mut inEncapsulatedPrefix: SCode::Encapsulated, mut inInstDims9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImplicit: bool, mut inCallingScope: InstTypes::CallingScope, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets, mut instSingleCref: Option<Arc<DAE::ComponentRef>>, mut comment: Arc<SCode::Comment>, mut info: SourceInfo, mut stopInst: Mutable::Mutable<bool>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, Option<Arc<DAE::Type>>, Option<SCode::Attributes>, Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut oty: Option<Arc<DAE::Type>> = None;
    let mut optDerAttr: Option<SCode::Attributes> = None;
    let mut outEqualityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, oty, optDerAttr, outEqualityConstraint, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inMod2.clone(), inPrefix3.clone(), inState5.clone(), className.clone(), inClassDef6.clone(), inRestriction7.clone(), inVisibility.clone(), inPartialPrefix.clone(), inEncapsulatedPrefix.clone(), inInstDims9.clone(), inImplicit.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone(), instSingleCref.clone(), comment.clone(), info.clone(), stopInst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, _, Deref @ SCode::ClassDef::PARTS { initialAlgorithmLst: Deref @ metamodelica::List::Nil, normalAlgorithmLst: Deref @ metamodelica::List::Nil, initialEquationLst: Deref @ metamodelica::List::Nil, normalEquationLst: Deref @ metamodelica::List::Nil, elementLst: els, .. }, re, vis, _, _, inst_dims, r#impl, _, graph, _, _, _, _, _) => {
                    let mut cdefelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extendselts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extendsclasselts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut compelts_2_elem: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut env1: FCore::Graph;
                    let mut extcomps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut csets: DAE::Connect::Sets;
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut fdae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut ci_state = (*ci_state).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let false = (openmodelica_frontend_types::SCode::Restriction::R_MODEL == re.clone()) else { bail!("pattern mismatch") };
                    let false = (openmodelica_frontend_types::SCode::Restriction::R_PACKAGE == re.clone()) else { bail!("pattern mismatch") };
                    let false = (SCodeUtil::isFunctionRestriction(re.clone())) else { bail!("pattern mismatch") };
                    let false = (SCode::Restriction::R_RECORD { isOperator: true } == re.clone()) else { bail!("pattern mismatch") };
                    let false = (SCode::Restriction::R_RECORD { isOperator: false } == re.clone()) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(InstUtil::splitElts(els.clone())?) {
                        (__pa0, __pa1, __pa2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Nil) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cdefelts = __pa0.clone();
                    extendsclasselts = __pa1.clone();
                    extendselts = __pa2.clone();
                    extendselts = SCodeInstUtil::addRedeclareAsElementsToExtends(extendselts.clone(), List::select(els.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isRedeclareElement, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>)))?;
                    (cache, env1, ih) = InstUtil::addClassdefsToEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), cdefelts.clone(), r#impl.clone(), Some(mods.clone()), false)?;
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(InstExtends::instExtendsAndClassExtendsList(cache.clone(), env1.clone(), ih.clone(), mods.clone(), pre.clone(), extendselts.clone(), extendsclasselts.clone(), els.clone(), ci_state.clone(), (className.clone()).clone(), r#impl.clone(), false)?) {
                        (__pa3, _, _, _, __pa4, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    extcomps = __pa4.clone();
                    compelts_2_elem = List::map(extcomps.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    ::match_deref::match_deref! { match &(InstUtil::splitElts(compelts_2_elem.clone())?) {
                        (_, _, _, Deref @ metamodelica::List::Nil) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (cache, env, ih, store, fdae, csets, ci_state, vars, bc, oDA, eqConstraint, graph) = instClassdefBasicType(cache.clone(), env.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state.clone(), (className.clone()).clone(), inClassDef6.clone(), re.clone(), vis.clone(), inst_dims.clone(), r#impl.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), info.clone(), stopInst.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), fdae.clone(), csets.clone(), ci_state.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, _, ci_state, _, Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, _, _, _, _, _, r#impl, _, graph, _, _, _, _, _) => {
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut ci_state = (*ci_state).clone();
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::isExternalObject(els.clone())) else { bail!("pattern mismatch") };
                    (cache, env, ih, dae, ci_state) = InstFunction::instantiateExternalObject(cache.clone(), env.clone(), ih.clone(), els.clone(), mods.clone(), r#impl.clone(), comment.clone(), info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), inSets.clone(), ci_state.clone(), metamodelica::nil(), None, None, None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, _, Deref @ SCode::ClassDef::PARTS { externalDecl: ed, clsattrs, constraintLst: constrs, initialAlgorithmLst: initalg, normalAlgorithmLst: alg, initialEquationLst: initeqs, normalEquationLst: eqs, elementLst: els }, re, _, _, _, inst_dims, r#impl, callscope, graph, csets, _, _, _, _) => {
                    let mut cdefelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut compelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extendselts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extendsclasselts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut compelts_2_elem: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut env5: FCore::Graph;
                    let mut cdefelts_1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut extcomps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut compelts_1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut compelts_2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut comp_cond: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut csets1: DAE::Connect::Sets;
                    let mut csets2: DAE::Connect::Sets;
                    let mut csets3: DAE::Connect::Sets;
                    let mut csets4: DAE::Connect::Sets;
                    let mut csets5: DAE::Connect::Sets;
                    let mut dae1: DAE::DAElist;
                    let mut dae2: DAE::DAElist;
                    let mut dae3: DAE::DAElist;
                    let mut dae4: DAE::DAElist;
                    let mut dae5: DAE::DAElist;
                    let mut dae6: DAE::DAElist;
                    let mut dae7: DAE::DAElist;
                    let mut dae8: DAE::DAElist;
                    let mut dae: DAE::DAElist;
                    let mut ci_state1: ClassInf::State;
                    let mut ci_state2: ClassInf::State;
                    let mut ci_state3: ClassInf::State;
                    let mut ci_state4: ClassInf::State;
                    let mut ci_state5: ClassInf::State;
                    let mut ci_state6: ClassInf::State;
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut emods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut checkMods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut eqs2: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut initeqs2: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut eqs_1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut initeqs_1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut alg2: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut initalg2: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut alg_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut initalg_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut comments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut unrollForLoops: bool = false;
                    let mut zero_dims: bool = false;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut elementSource: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut smCompCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut smInitialCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut smCompToFlatSM: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut domainFieldsLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut mods = (*mods).clone();
                    let mut els = (*els).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let mut oty: Option<Arc<DAE::Type>> = oty.clone();
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let false = (SCodeUtil::isExternalObject(els.clone())) else { bail!("pattern mismatch") };
                    ci_state1 = ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::NEWDEF)?;
                    els = InstUtil::extractConstantPlusDeps(els.clone(), instSingleCref.clone(), metamodelica::nil(), (className.clone()).clone())?;
                    (cdefelts, extendsclasselts, extendselts, compelts) = InstUtil::splitElts(els.clone())?;
                    extendselts = SCodeInstUtil::addRedeclareAsElementsToExtends(extendselts.clone(), List::select(els.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isRedeclareElement, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>)))?;
                    (cache, env1, ih) = InstUtil::addClassdefsToEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), cdefelts.clone(), r#impl.clone(), Some(mods.clone()), FGraph::isEmptyScope(env.clone()))?;
                    (cache, env2, ih, emods, extcomps, eqs2, initeqs2, alg2, initalg2, comments) = InstExtends::instExtendsAndClassExtendsList(cache.clone(), env1.clone(), ih.clone(), mods.clone(), pre.clone(), extendselts.clone(), extendsclasselts.clone(), els.clone(), ci_state.clone(), (className.clone()).clone(), r#impl.clone(), false)?;
                    compelts_1 = InstUtil::addNomod(compelts.clone());
                    cdefelts_1 = InstUtil::addNomod(cdefelts.clone());
                    compelts_1 = List::flatten(list![extcomps.clone(), compelts_1.clone(), cdefelts_1.clone()]);
                    eqs_1 = joinExtEquations(eqs.clone(), eqs2.clone(), callscope.clone());
                    initeqs_1 = joinExtEquations(initeqs.clone(), initeqs2.clone(), callscope.clone());
                    alg_1 = joinExtAlgorithms(alg.clone(), alg2.clone(), callscope.clone());
                    initalg_1 = joinExtAlgorithms(initalg.clone(), initalg2.clone(), callscope.clone());
                    (compelts_1, eqs_1, initeqs_1, alg_1, initalg_1) = InstUtil::extractConstantPlusDepsTpl(compelts_1.clone(), instSingleCref.clone(), metamodelica::nil(), (className.clone()).clone(), eqs_1.clone(), initeqs_1.clone(), alg_1.clone(), initalg_1.clone())?;
                    if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                        compelts_1 = InstUtil::addGhostCells(compelts_1.clone(), eqs_1.clone());
                    }
                    checkMods = Mod::merge(mods.clone(), emods.clone(), (className.clone()).clone(), true)?;
                    mods = checkMods.clone();
                    (cache, env3, ih) = InstUtil::addComponentsToEnv(cache.clone(), env2.clone(), ih.clone(), mods.clone(), pre.clone(), ci_state.clone(), compelts_1.clone(), r#impl.clone())?;
                    compelts_2_elem = List::map(compelts_1.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    InstUtil::matchModificationToComponents(compelts_2_elem.clone(), checkMods.clone(), (FGraph::printGraphPathStr(env3.clone())?).clone())?;
                    (comp_cond, compelts_1) = List::splitOnTrue(compelts_1.clone(), (std::sync::Arc::new(fnptr!(InstUtil::componentHasCondition, (Arc<SCode::Element>, Arc<DAE::Mod>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>));
                    compelts_2 = listAppend(compelts_1.clone(), comp_cond.clone());
                    (smCompCrefs, smInitialCrefs) = InstStateMachineUtil::getSMStatesInContext(eqs_1.clone(), pre.clone());
                    ih = List::fold(smCompCrefs.clone(), (std::sync::Arc::new(InnerOuter::updateSMHierarchy) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<InnerOuter::TopInstance>>) -> Result<Arc<metamodelica::List<InnerOuter::TopInstance>>> + 'static>), ih.clone());
                    (cache, env5, ih, store, dae1, csets, ci_state2, vars, graph, domainFieldsLst) = instElementList(cache.clone(), env3.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state1.clone(), compelts_2.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), csets.clone(), true)?;
                    zero_dims = InstUtil::instDimsHasZeroDims(inst_dims.clone())?;
                    elementSource = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env3.clone())?, pre.clone(), (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?;
                    csets1 = ConnectUtil::addConnectorVariablesFromDAE(zero_dims.clone(), ci_state1.clone(), pre.clone(), vars.clone(), info.clone(), elementSource.clone(), csets.clone())?;
                    (cache, eqs_1) = InstUtil::reorderConnectEquationsExpandable(cache.clone(), env5.clone(), eqs_1.clone())?;
                    if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                        eqs_1 = List::fold1(eqs_1.clone(), (std::sync::Arc::new(InstUtil::discretizePDE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>>, Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> + 'static>), domainFieldsLst.clone(), metamodelica::nil());
                    }
                    (cache, env5, ih, dae2, csets2, ci_state3, graph) = instList(cache.clone(), env5.clone(), ih.clone(), pre.clone(), csets1.clone(), ci_state2.clone(), (std::sync::Arc::new(InstSection::instEquation) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Arc<SCode::Equation>, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>), eqs_1.clone(), r#impl.clone(), InstTypes::alwaysUnroll.clone(), graph.clone())?;
                    DAEUtil::verifyEquationsDAE(dae2.clone())?;
                    if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                        initeqs_1 = List::fold1(initeqs_1.clone(), (std::sync::Arc::new(InstUtil::discretizePDE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>>, Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> + 'static>), domainFieldsLst.clone(), metamodelica::nil());
                    }
                    (cache, env5, ih, dae3, csets3, ci_state4, graph) = instList(cache.clone(), env5.clone(), ih.clone(), pre.clone(), csets2.clone(), ci_state3.clone(), (std::sync::Arc::new(InstSection::instInitialEquation) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Arc<SCode::Equation>, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>), initeqs_1.clone(), r#impl.clone(), InstTypes::alwaysUnroll.clone(), graph.clone())?;
                    unrollForLoops = if (SCodeUtil::isFunctionRestriction(re.clone())) {InstTypes::neverUnroll.clone()} else {InstTypes::alwaysUnroll.clone()};
                    (cache, env5, ih, dae4, csets4, ci_state5, graph) = instList(cache.clone(), env5.clone(), ih.clone(), pre.clone(), csets3.clone(), ci_state4.clone(), (std::sync::Arc::new(InstSection::instAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Arc<SCode::AlgorithmSection>, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>), alg_1.clone(), r#impl.clone(), unrollForLoops.clone(), graph.clone())?;
                    (cache, env5, ih, dae5, csets5, ci_state6, graph) = instList(cache.clone(), env5.clone(), ih.clone(), pre.clone(), csets4.clone(), ci_state5.clone(), (std::sync::Arc::new(InstSection::instInitialAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Arc<SCode::AlgorithmSection>, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>), initalg_1.clone(), r#impl.clone(), unrollForLoops.clone(), graph.clone())?;
                    (cache, env5, dae6) = instClassAttributes(cache.clone(), env5.clone(), pre.clone(), clsattrs.clone(), r#impl.clone(), info.clone())?;
                    (cache, env5, dae7, _) = instConstraints(cache.clone(), env5.clone(), pre.clone(), ci_state6.clone(), constrs.clone(), r#impl.clone())?;
                    dae8 = instFunctionAnnotations(cons(comment.clone(), comments.clone()), ci_state6.clone())?;
                    smCompToFlatSM = InstStateMachineUtil::createSMNodeToFlatSMGroupTable(dae2.clone())?;
                    (dae1, dae2) = InstStateMachineUtil::wrapSMCompsInFlatSMs(ih.clone(), dae1.clone(), dae2.clone(), smCompToFlatSM.clone(), smInitialCrefs.clone())?;
                    dae = DAEUtil::joinDaeLst(list![dae1.clone(), dae2.clone(), dae3.clone(), dae4.clone(), dae5.clone(), dae6.clone(), dae7.clone(), dae8.clone()])?;
                    csets5 = InnerOuter::changeInnerOuterInOuterConnect(csets5.clone());
                    eqConstraint = InstUtil::equalityConstraint(env5.clone(), els.clone(), info.clone());
                    ci_state6 = if (isSome(ed.clone())) {ClassInfUtil::assertTrans(ci_state6.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_EXT_DECL, info.clone())?} else {ci_state6.clone()};
                    (cache, oty) = InstMeta::fixUniontype(cache.clone(), env5.clone(), ci_state6.clone(), inClassDef6.clone())?;
                    let _ = (::match_deref::match_deref! { match &(oty.clone()) {
        Some(ty @ Deref @ DAE::Type::T_METAUNIONTYPE { typeVars: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }) => {
                    Error::addSourceMessage(Error::UNIONTYPE_MISSING_TYPEVARS.clone(), list![(TypesDump::unparseType(ty.clone())?).clone()], info.clone())?;
                    bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    Ok((cache.clone(), env5.clone(), ih.clone(), store.clone(), dae.clone(), csets5.clone(), ci_state6.clone(), vars.clone(), oty.clone(), None, eqConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: ad, path: cn } }, re, vis, _, _, inst_dims, r#impl, callscope, graph, _, _, _, _, _) => {
                    let mut env3: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut cenv_2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut csets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut ci_state2: ClassInf::State;
                    let mut new_ci_state: ClassInf::State;
                    let mut ci_state_1: ClassInf::State;
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mods_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut r: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut enc2: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut inst_dims_1: InstDims = metamodelica::nil();
                    let mut cn2: ArcStr = arcstr::literal!("");
                    let mut c: Arc<SCode::Element>;
                    let mut eq: Option<DAE::EqMod> = None;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), Some(info.clone()))?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { restriction: __pa1 @ SCode::Restriction::R_ENUMERATION, encapsulatedPrefix: __pa2, name: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    r = __pa1.clone();
                    enc2 = __pa2.clone();
                    cn2 = __pa3.clone();
                    c = __pa4.clone();
                    cenv = __pa5.clone();
                    env3 = FGraph::openScope(cenv.clone(), enc2.clone(), (cn2.clone()).clone(), Some(crate::FCore::ScopeType::CLASS_SCOPE))?;
                    ci_state2 = ClassInfUtil::start(r.clone(), FGraph::getGraphName(env3.clone())?)?;
                    new_ci_state = ClassInfUtil::start(r.clone(), FGraph::getGraphName(env3.clone())?)?;
                    (cache, cenv_2, _, _, _, _, _, _, _, _, _, _) = instClassIn(cache.clone(), env3.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state2.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), false, callscope.clone(), ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), None)?;
                    (cache, mod_1) = Mod::elabMod(cache.clone(), cenv_2.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    mods_1 = Mod::merge(mods.clone(), mod_1.clone(), (className.clone()).clone(), true)?;
                    eq = Mod::modEquation(mods_1.clone())?;
                    (cache, dims) = InstUtil::elabArraydimOpt(cache.clone(), cenv_2.clone(), Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("")).clone(), subscripts: metamodelica::nil() }), cn.clone(), ad.clone(), eq.clone(), r#impl.clone(), true, pre.clone(), info.clone(), inst_dims.clone())?;
                    inst_dims_1 = List::appendLastList(inst_dims.clone(), dims.clone())?;
                    (cache, env_2, ih, store, dae, csets_1, ci_state_1, vars, bc, oDA, eqConstraint, graph) = instClassIn(cache.clone(), cenv_2.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), new_ci_state.clone(), c.clone(), vis.clone(), inst_dims_1.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone())?;
                    ClassInfUtil::assertValid(ci_state_1.clone(), re.clone(), info.clone())?;
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), store.clone(), dae.clone(), csets_1.clone(), ci_state_1.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: ad, path: cn } }, re, vis, _, _, inst_dims, r#impl, callscope, graph, _, _, _, _, _) => {
                    let mut cenv: FCore::Graph;
                    let mut cenv_2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut parentEnv: FCore::Graph;
                    let mut csets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut new_ci_state: ClassInf::State;
                    let mut ci_state_1: ClassInf::State;
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mods_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut r: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut valid_connector: bool = false;
                    let mut enc2: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut inst_dims_1: InstDims = metamodelica::nil();
                    let mut cn2: ArcStr = arcstr::literal!("");
                    let mut c: Arc<SCode::Element>;
                    let mut eq: Option<DAE::EqMod> = None;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), Some(info.clone()))?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { restriction: __pa1, encapsulatedPrefix: __pa2, name: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    r = __pa1.clone();
                    enc2 = __pa2.clone();
                    cn2 = __pa3.clone();
                    c = __pa4.clone();
                    cenv = __pa5.clone();
                    let true = (InstUtil::checkDerivedRestriction(re.clone(), r.clone(), (cn2.clone()).clone())) else { bail!("pattern mismatch") };
                    valid_connector = ConnectUtil::checkShortConnectorDef(ci_state.clone(), DA.clone(), info.clone())?;
                    Mutable::update(stopInst.clone(), !(valid_connector.clone()));
                    let true = (valid_connector.clone()) else { bail!("pattern mismatch") };
                    cenv_2 = FGraph::openScope(cenv.clone(), enc2.clone(), (cn2.clone()).clone(), FGraph::classInfToScopeType(ci_state.clone()))?;
                    new_ci_state = ClassInfUtil::start(r.clone(), FGraph::getGraphName(cenv_2.clone())?)?;
                    r#mod = InstUtil::chainRedeclares(mods.clone(), r#mod.clone())?;
                    (parentEnv, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, mod_1) = Mod::elabMod(cache.clone(), parentEnv.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    mods_1 = Mod::merge(mods.clone(), mod_1.clone(), (className.clone()).clone(), true)?;
                    eq = Mod::modEquation(mods_1.clone())?;
                    (cache, dims) = InstUtil::elabArraydimOpt(cache.clone(), parentEnv.clone(), Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("")).clone(), subscripts: metamodelica::nil() }), cn.clone(), ad.clone(), eq.clone(), r#impl.clone(), true, pre.clone(), info.clone(), inst_dims.clone())?;
                    inst_dims_1 = List::appendLastList(inst_dims.clone(), dims.clone())?;
                    let _ = AbsynUtil::getArrayDimOptAsList(ad.clone());
                    (cache, env_2, ih, store, dae, csets_1, ci_state_1, vars, bc, oDA, eqConstraint, graph) = instClassIn(cache.clone(), cenv_2.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), new_ci_state.clone(), c.clone(), vis.clone(), inst_dims_1.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone())?;
                    ClassInfUtil::assertValid(ci_state_1.clone(), re.clone(), info.clone())?;
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), store.clone(), dae.clone(), csets_1.clone(), ci_state_1.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: ad, path: cn } }, re, vis, partialPrefix, encapsulatedPrefix, inst_dims, r#impl, callscope, graph, _, _, _, _, _) => {
                    let mut parentEnv: FCore::Graph;
                    let mut parentClassEnv: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mods_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut r: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut cn2: ArcStr = arcstr::literal!("");
                    let mut classDefParent: Arc<SCode::ClassDef>;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut ci_state = (*ci_state).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let false = (re.clone() == openmodelica_frontend_types::SCode::Restriction::R_TYPE) else { bail!("pattern mismatch") };
                    let false = (re.clone() == openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION) else { bail!("pattern mismatch") };
                    let false = (re.clone() == openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION) else { bail!("pattern mismatch") };
                    let false = (SCodeUtil::isConnector(re.clone())) else { bail!("pattern mismatch") };
                    let true = (boolOr(ad.clone() == None, ad.clone() == Some(metamodelica::nil()))) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), Some(info.clone()))?) {
                        (__pa0, Deref @ SCode::Element::CLASS { classDef: __pa1, restriction: __pa2, name: __pa3, .. }, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    classDefParent = __pa1.clone();
                    r = __pa2.clone();
                    cn2 = __pa3.clone();
                    parentClassEnv = __pa4.clone();
                    let false = (InstUtil::checkDerivedRestriction(re.clone(), r.clone(), (cn2.clone()).clone())) else { bail!("pattern mismatch") };
                    if (match r.clone() {
        SCode::Restriction::R_PACKAGE { .. } => false,
        _ => if (SCodeUtil::restrictionEqual(r.clone(), re.clone())) {Mod::isInvariantMod(r#mod.clone())? && Mod::isInvariantDAEMod(mods.clone())?} else {false},
    }) {
                        r#mod = InstUtil::chainRedeclares(mods.clone(), r#mod.clone())?;
                        (parentEnv, _) = FGraph::stripLastScopeRef(env.clone())?;
                        (cache, mod_1) = Mod::elabMod(cache.clone(), parentEnv.clone(), ih.clone(), pre.clone(), r#mod.clone(), false, Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                        mods_1 = Mod::merge(mods.clone(), mod_1.clone(), (className.clone()).clone(), true)?;
                        (cache, env, ih, store, dae, csets, ci_state, vars, bc, oDA, eqConstraint, graph) = instClassdef2(cache.clone(), parentClassEnv.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), ci_state.clone(), (className.clone()).clone(), classDefParent.clone(), re.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), comment.clone(), info.clone(), stopInst.clone())?;
                        oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    } else {
                        r#mod = InstUtil::chainRedeclares(mods.clone(), r#mod.clone())?;
                        (parentEnv, _) = FGraph::stripLastScopeRef(env.clone())?;
                        (cache, mod_1) = Mod::elabMod(cache.clone(), parentEnv.clone(), ih.clone(), pre.clone(), r#mod.clone(), false, Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                        mods_1 = Mod::merge(mods.clone(), mod_1.clone(), (className.clone()).clone(), true)?;
                        (cache, env, ih, store, dae, csets, ci_state, vars, bc, oDA, eqConstraint, graph) = instClassdef2(cache.clone(), env.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), ci_state.clone(), (className.clone()).clone(), Arc::new(SCode::ClassDef::PARTS { elementLst: list![Arc::new(SCode::Element::EXTENDS { baseClassPath: cn.clone(), visibility: vis.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), ann: None, info: info.clone() })], normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), re.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), comment.clone(), info.clone(), stopInst.clone())?;
                        oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    }
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: ad, path: cn } }, re, vis, _, _, inst_dims, r#impl, callscope, graph, _, _, _, _, _) => {
                    let mut cenv: FCore::Graph;
                    let mut cenv_2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut parentEnv: FCore::Graph;
                    let mut csets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut new_ci_state: ClassInf::State;
                    let mut ci_state_1: ClassInf::State;
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mods_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut r: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut enc2: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut inst_dims_1: InstDims = metamodelica::nil();
                    let mut cn2: ArcStr = arcstr::literal!("");
                    let mut c: Arc<SCode::Element>;
                    let mut eq: Option<DAE::EqMod> = None;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut eqConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut graph = (*graph).clone();
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), Some(info.clone()))?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { restriction: __pa1, encapsulatedPrefix: __pa2, name: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    r = __pa1.clone();
                    enc2 = __pa2.clone();
                    cn2 = __pa3.clone();
                    c = __pa4.clone();
                    cenv = __pa5.clone();
                    let false = (InstUtil::checkDerivedRestriction(re.clone(), r.clone(), (cn2.clone()).clone())) else { bail!("pattern mismatch") };
                    cenv_2 = FGraph::openScope(cenv.clone(), enc2.clone(), (className.clone()).clone(), FGraph::classInfToScopeType(ci_state.clone()))?;
                    new_ci_state = ClassInfUtil::start(r.clone(), FGraph::getGraphName(cenv_2.clone())?)?;
                    c = SCodeUtil::setClassName((className.clone()).clone(), c.clone())?;
                    r#mod = InstUtil::chainRedeclares(mods.clone(), r#mod.clone())?;
                    (parentEnv, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, mod_1) = Mod::elabMod(cache.clone(), parentEnv.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    mods_1 = Mod::merge(mods.clone(), mod_1.clone(), (className.clone()).clone(), true)?;
                    eq = Mod::modEquation(mods_1.clone())?;
                    (cache, dims) = InstUtil::elabArraydimOpt(cache.clone(), parentEnv.clone(), Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("")).clone(), subscripts: metamodelica::nil() }), cn.clone(), ad.clone(), eq.clone(), r#impl.clone(), true, pre.clone(), info.clone(), inst_dims.clone())?;
                    inst_dims_1 = List::appendLastList(inst_dims.clone(), dims.clone())?;
                    (cache, env_2, ih, store, dae, csets_1, ci_state_1, vars, bc, oDA, eqConstraint, graph) = instClassIn(cache.clone(), cenv_2.clone(), ih.clone(), store.clone(), mods_1.clone(), pre.clone(), new_ci_state.clone(), c.clone(), vis.clone(), inst_dims_1.clone(), r#impl.clone(), callscope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone())?;
                    ClassInfUtil::assertValid(ci_state_1.clone(), re.clone(), info.clone())?;
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), store.clone(), dae.clone(), csets_1.clone(), ci_state_1.clone(), vars.clone(), bc.clone(), oDA.clone(), eqConstraint.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, mods, _, _, _, Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { .. }, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (Mod::emptyModOrEquality(mods.clone()) && SCodeUtil::emptyModOrEquality(r#mod.clone())) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::META_COMPLEX_TYPE_MOD.clone(), metamodelica::nil(), info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, typeSpecs: Deref @ metamodelica::List::Cons { head: tSpec, tail: Deref @ metamodelica::List::Nil }, arrayDim: None } }, _, _, _, _, inst_dims, r#impl, _, graph, _, _, _, _, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(mods.clone()) && SCodeUtil::emptyModOrEquality(r#mod.clone())) else { bail!("pattern mismatch") };
                    (cache, _, ih, tys, csets, oDA) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), list![tSpec.clone()], pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), inSets.clone(), info.clone())?;
                    ty = listHead(tys.clone())?;
                    ty = Types::boxIfUnboxedType(ty.clone())?;
                    bc = Some(Arc::new(DAE::Type::T_METALIST { ty: ty.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_LIST { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Option" }, typeSpecs: Deref @ metamodelica::List::Cons { head: tSpec, tail: Deref @ metamodelica::List::Nil }, arrayDim: None } }, _, _, _, _, inst_dims, r#impl, _, graph, _, _, _, _, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(mods.clone()) && SCodeUtil::emptyModOrEquality(r#mod.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(instClassDefHelper(cache.clone(), env.clone(), ih.clone(), list![tSpec.clone()], pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), inSets.clone(), info.clone())?) {
                        (__pa0, _, __pa1, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }, __pa3, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ih = __pa1.clone();
                    ty = __pa2.clone();
                    csets = __pa3.clone();
                    oDA = __pa4.clone();
                    ty = Types::boxIfUnboxedType(ty.clone())?;
                    bc = Some(Arc::new(DAE::Type::T_METAOPTION { ty: ty.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_OPTION { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tuple" }, typeSpecs: tSpecs, arrayDim: None } }, _, _, _, _, inst_dims, r#impl, _, graph, _, _, _, _, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(mods.clone()) && SCodeUtil::emptyModOrEquality(r#mod.clone())) else { bail!("pattern mismatch") };
                    (cache, _, ih, tys, csets, oDA) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), tSpecs.clone(), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), inSets.clone(), info.clone())?;
                    tys = List::map(tys.clone(), (std::sync::Arc::new(Types::boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    bc = Some(Arc::new(DAE::Type::T_METATUPLE { types: tys.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_TUPLE { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, typeSpecs: Deref @ metamodelica::List::Cons { head: tSpec, tail: Deref @ metamodelica::List::Nil }, arrayDim: None } }, _, _, _, _, inst_dims, r#impl, _, graph, _, _, _, _, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(mods.clone()) && SCodeUtil::emptyModOrEquality(r#mod.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(instClassDefHelper(cache.clone(), env.clone(), ih.clone(), list![tSpec.clone()], pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), inSets.clone(), info.clone())?) {
                        (__pa0, _, __pa1, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }, __pa3, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ih = __pa1.clone();
                    ty = __pa2.clone();
                    csets = __pa3.clone();
                    oDA = __pa4.clone();
                    ty = Types::boxIfUnboxedType(ty.clone())?;
                    bc = Some(Arc::new(DAE::Type::T_METAARRAY { ty: ty.clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_ARRAY { path: Arc::new(Absyn::Path::IDENT { name: (className.clone()).clone() }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, typeSpecs: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Any" }, arrayDim: None }, tail: Deref @ metamodelica::List::Nil }, arrayDim: None } }, _, _, _, _, inst_dims, r#impl, _, graph, _, _, _, _, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(mods.clone()) && SCodeUtil::emptyModOrEquality(r#mod.clone())) else { bail!("pattern mismatch") };
                    (cache, _, ih, _, csets, oDA) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), metamodelica::nil(), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), inSets.clone(), info.clone())?;
                    bc = Some(Arc::new(DAE::Type::T_METAPOLYMORPHIC { name: (className.clone()).clone() }));
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), ClassInf::State::META_POLYMORPHIC { path: Arc::new(Absyn::Path::IDENT { name: (className.clone()).clone() }) }, metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, mods, _, _, _, Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. }, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let true = (Mod::emptyModOrEquality(mods.clone()) && SCodeUtil::emptyModOrEquality(r#mod.clone())) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::META_POLYMORPHIC.clone(), list![(className.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: r#str }, typeSpecs: tSpecs, arrayDim: None } }, re, vis, partialPrefix, encapsulatedPrefix, inst_dims, r#impl, _, graph, _, _, _, _, _) => {
                    let mut r#str = (*r#str).clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph;
                    let mut outEqualityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = outEqualityConstraint.clone();
                    let mut outDae: DAE::DAElist;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = outTypesVarLst.clone();
                    let mut outSets: DAE::Connect::Sets;
                    let mut outStore: UnitAbsyn::InstStore = outStore.clone();
                    let mut outState: ClassInf::State;
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outEnv: FCore::Graph;
                    let mut oty: Option<Arc<DAE::Type>> = oty.clone();
                    let mut optDerAttr: Option<SCode::Attributes> = optDerAttr.clone();
                    r#str = (Util::assoc((r#str.clone()).clone(), list![(literal!("List"), literal!("list")), (literal!("Tuple"), literal!("tuple")), (literal!("Array"), literal!("array"))])?).clone();
                    (outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, oty, optDerAttr, outEqualityConstraint, outGraph) = instClassdef2(cache.clone(), env.clone(), ih.clone(), store.clone(), mods.clone(), pre.clone(), ci_state.clone(), (className.clone()).clone(), Arc::new(SCode::ClassDef::DERIVED { typeSpec: Arc::new(Absyn::TypeSpec::TCOMPLEX { path: Arc::new(Absyn::Path::IDENT { name: (r#str.clone()).clone() }), typeSpecs: tSpecs.clone(), arrayDim: None }), modifications: r#mod.clone(), attributes: DA.clone() }), re.clone(), vis.clone(), partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), r#impl.clone(), inCallingScope.clone(), graph.clone(), inSets.clone(), instSingleCref.clone(), comment.clone(), info.clone(), stopInst.clone())?;
                    Ok((outCache.clone(), outEnv.clone(), outIH.clone(), outStore.clone(), outDae.clone(), outSets.clone(), outState.clone(), outTypesVarLst.clone(), oty.clone(), optDerAttr.clone(), outEqualityConstraint.clone(), outGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, _, _, Deref @ SCode::ClassDef::DERIVED { attributes: DA, modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: cn, typeSpecs: tSpecs, arrayDim: None } }, _, _, _, _, inst_dims, r#impl, _, graph, _, _, _, _, _) => {
                    let mut cenv: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut new_ci_state: ClassInf::State;
                    let mut bc: Option<Arc<DAE::Type>> = None;
                    let mut cn2: ArcStr = arcstr::literal!("");
                    let mut classDef: Arc<SCode::ClassDef>;
                    let mut fq_class: Arc<Absyn::Path>;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut typeVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::emptyModOrEquality(mods.clone()) && SCodeUtil::emptyModOrEquality(r#mod.clone())) else { bail!("pattern mismatch") };
                    let false = (listMember((AbsynUtil::pathString(cn.clone(), (literal!(".")).clone(), true, false)?).clone(), list![(literal!("tuple")).clone(), (literal!("Tuple")).clone(), (literal!("array")).clone(), (literal!("Array")).clone(), (literal!("Option")).clone(), (literal!("list")).clone(), (literal!("List")).clone()])) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), Some(info.clone()))?) {
                        (__pa0, Deref @ SCode::Element::CLASS { classDef: __pa1, restriction: SCode::Restriction::R_UNIONTYPE { typeVars: __pa2 }, name: __pa3, .. }, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    classDef = __pa1.clone();
                    typeVars = __pa2.clone();
                    cn2 = __pa3.clone();
                    cenv = __pa4.clone();
                    (cache, fq_class) = makeFullyQualifiedIdent(cache.clone(), cenv.clone(), (cn2.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    new_ci_state = ClassInf::State::META_UNIONTYPE { path: fq_class.clone(), typeVars: typeVars.clone() };
                    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(InstMeta::fixUniontype(cache.clone(), env.clone(), new_ci_state.clone(), classDef.clone())?) {
                        (__pa5, Some(__pa6 @ Deref @ DAE::Type::T_METAUNIONTYPE { .. })) => (__pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa5.clone();
                    ty = __pa6.clone();
                    (cache, _, ih, tys, csets, oDA) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), tSpecs.clone(), pre.clone(), inst_dims.clone(), r#impl.clone(), metamodelica::nil(), inSets.clone(), info.clone())?;
                    tys = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut t in (tys.clone()).into_iter().cloned() {
                    let __x = Types::boxIfUnboxedType(t.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    if !((tys.clone().len() as i32) == (typeVars.clone().len() as i32)) {
                        Error::addSourceMessage(Error::UNIONTYPE_WRONG_NUM_TYPEVARS.clone(), list![(AbsynUtil::pathString(fq_class.clone(), (literal!(".")).clone(), true, false)?).clone(), ArcStr::from(::std::format!("{}", (typeVars.clone().len() as i32))), ArcStr::from(::std::format!("{}", (tys.clone().len() as i32)))], info.clone())?;
                        bail!("fail");
                    }
                    ty = Types::setTypeVariables(ty.clone(), tys.clone());
                    oDA = SCodeUtil::mergeAttributes(DA.clone(), oDA.clone())?;
                    bc = Some(ty.clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), DAE::emptyDae().clone(), csets.clone(), new_ci_state.clone(), metamodelica::nil(), bc.clone(), oDA.clone(), None, graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: tSpec @ Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: Some(_), .. }, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut cns: ArcStr = arcstr::literal!("");
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    cns = (Dump::unparseTypeSpec(tSpec.clone())?).clone();
                    Error::addSourceMessage(Error::META_INVALID_COMPLEX_TYPE.clone(), list![(cns.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: tSpec @ Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: tSpecs, path: cn, .. }, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut cns: ArcStr = arcstr::literal!("");
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (listMember((AbsynUtil::pathString(cn.clone(), (literal!(".")).clone(), true, false)?, (tSpecs.clone().len() as i32) == 1), list![(literal!("tuple"), false), (literal!("array"), true), (literal!("Option"), true), (literal!("list"), true)])) else { bail!("pattern mismatch") };
                    cns = (Dump::unparseTypeSpec(tSpec.clone())?).clone();
                    Error::addSourceMessage(Error::META_INVALID_COMPLEX_TYPE.clone(), list![(cns.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _, _, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, .. }, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut cns: ArcStr = arcstr::literal!("");
                    let mut scope_str: ArcStr = arcstr::literal!("");
                    let false = (Mutable::access(stopInst.clone())) else { bail!("pattern mismatch") };
                    if '__try0: {
                        (_, _, _) = unwrap_break_err!(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    cns = (AbsynUtil::pathString(cn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    scope_str = (FGraph::printGraphPathStr(env.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_ERROR.clone(), list![(cns.clone()).clone(), (scope_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _, _, _, _, Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, .. }, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    if '__try0: {
                        (_, _, _) = unwrap_break_err!(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Debug::trace((literal!("- Inst.instClassdef DERIVED( ")).clone())?;
                    Debug::trace((AbsynUtil::pathString(cn.clone(), (literal!(".")).clone(), true, false)?).clone())?;
                    Debug::trace((literal!(") lookup failed\n ENV:")).clone())?;
                    Debug::trace((FGraph::printGraphStr(env.clone())).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- Inst.instClassdef failed")).clone())?;
                    s = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  class :")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outTypesVarLst, oty, optDerAttr, outEqualityConstraint, outGraph))
}

fn joinExtEquations(mut inEq: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inExtEq: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inCallingScope: InstTypes::CallingScope) -> Arc<metamodelica::List<Arc<SCode::Equation>>> {
    let mut outEq: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    outEq = (::match_deref::match_deref! { match &((inEq.clone(), inExtEq.clone(), inCallingScope.clone())) {
        (_, _, InstTypes::CallingScope::TYPE_CALL) => metamodelica::nil(),
        _ => listAppend(inEq.clone(), inExtEq.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEq
}

fn joinExtAlgorithms(mut inAlg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut inExtAlg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut inCallingScope: InstTypes::CallingScope) -> Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> {
    let mut outAlg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    outAlg = (match inCallingScope.clone() {
        InstTypes::CallingScope::TYPE_CALL => metamodelica::nil(),
        _ => listAppend(inAlg.clone(), inExtAlg.clone()),
    });
    outAlg
}

fn instClassDefHelper(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inSpecs: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>, mut inPre: DAE::Prefix, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImpl: bool, mut accTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inSets: DAE::Connect::Sets, mut inInfo: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<metamodelica::List<Arc<DAE::Type>>>, DAE::Connect::Sets, Option<SCode::Attributes>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outType: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets;
    let mut outAttr: Option<SCode::Attributes> = None;
    (outCache, outEnv, outIH, outType, outSets, outAttr) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inSpecs.clone(), inPre.clone(), inInstDims.clone(), inImpl.clone(), accTypes.clone(), inSets.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::List::Nil, _, _, _, localAccTypes, _) => {
                    Ok((cache.clone(), env.clone(), ih.clone(), localAccTypes.clone().reverse(), inSets.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: _ }, tail: restTypeSpecs }, pre, dims, r#impl, localAccTypes, _) => {
                    let mut cenv: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Arc<SCode::Element>;
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut localAccTypes = (*localAccTypes).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), Some(inInfo.clone()))?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    c = __pa1.clone();
                    cenv = __pa2.clone();
                    let false = (SCodeUtil::isFunction(c.clone())) else { bail!("pattern mismatch") };
                    (cache, cenv, ih, _, _, csets, ty, _, oDA, _) = instClass(cache.clone(), cenv.clone(), ih.clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), c.clone(), dims.clone(), r#impl.clone(), crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), inSets.clone())?;
                    localAccTypes = cons(ty.clone(), localAccTypes.clone());
                    (cache, env, ih, localAccTypes, csets, _) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), restTypeSpecs.clone(), pre.clone(), dims.clone(), r#impl.clone(), localAccTypes.clone(), csets.clone(), inInfo.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), localAccTypes.clone(), csets.clone(), oDA.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: _ }, tail: restTypeSpecs }, pre, dims, r#impl, localAccTypes, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut localAccTypes = (*localAccTypes).clone();
                    (cache, ty, _) = Lookup::lookupType(cache.clone(), env.clone(), cn.clone(), None)?;
                    localAccTypes = cons(ty.clone(), localAccTypes.clone());
                    (cache, env, ih, localAccTypes, csets, _) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), restTypeSpecs.clone(), pre.clone(), dims.clone(), r#impl.clone(), localAccTypes.clone(), inSets.clone(), inInfo.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), localAccTypes.clone(), csets.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::List::Cons { head: tSpec @ Deref @ Absyn::TypeSpec::TCOMPLEX { path: p, typeSpecs: _, arrayDim: _ }, tail: restTypeSpecs }, pre, dims, r#impl, localAccTypes, _) => {
                    let mut csets: DAE::Connect::Sets;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Arc<SCode::Element>;
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut oDA: Option<SCode::Attributes> = None;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut localAccTypes = (*localAccTypes).clone();
                    id = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    c = Arc::new(SCode::Element::CLASS { name: (id.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::DERIVED { typeSpec: tSpec.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() });
                    (cache, _, ih, _, _, csets, ty, _, oDA, _) = instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), c.clone(), dims.clone(), r#impl.clone(), crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), inSets.clone())?;
                    localAccTypes = cons(ty.clone(), localAccTypes.clone());
                    (cache, env, ih, localAccTypes, csets, _) = instClassDefHelper(cache.clone(), env.clone(), ih.clone(), restTypeSpecs.clone(), pre.clone(), dims.clone(), r#impl.clone(), localAccTypes.clone(), csets.clone(), inInfo.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), localAccTypes.clone(), csets.clone(), oDA.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outType, outSets, outAttr))
}

fn instBasictypeBaseclass(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inSCodeElementLst2: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inSCodeElementLst3: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inMod4: Arc<DAE::Mod>, mut inInstDims5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut className: ArcStr, mut info: SourceInfo, mut stopInst: Mutable::Mutable<bool>) -> Result<(FCore::Cache, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, Option<Arc<DAE::Type>>, Arc<metamodelica::List<Arc<DAE::Var>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outTypesTypeOption: Option<Arc<DAE::Type>> = None;
    let mut outTypeVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    (outCache, outIH, outStore, outDae, outTypesTypeOption, outTypeVars) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inSCodeElementLst2.clone(), inSCodeElementLst3.clone(), inMod4.clone(), inInstDims5.clone(), className.clone(), info.clone(), stopInst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { modifications: r#mod, baseClassPath: path, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, mods, inst_dims, _, _, _) => {
                    let mut m_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut m_2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cdef: Arc<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut b3: bool = false;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint((literal!("instBasictypeBaseclass")).clone());
                    (cache, m_1) = Mod::elabModForBasicType(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::DERIVED { path: path.clone() }, info.clone())?;
                    m_2 = Mod::merge(mods.clone(), m_1.clone(), (className.clone()).clone(), true)?;
                    (cache, cdef, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), Some(info.clone()))?;
                    (cache, _, ih, store, dae, _, ty, tys, _) = instClassBasictype(cache.clone(), cenv.clone(), ih.clone(), store.clone(), m_2.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef.clone(), inst_dims.clone(), false, crate::InstTypes::CallingScope::INNER_CALL, Connect::emptySet().clone())?;
                    b1 = Types::basicType(ty.clone());
                    b2 = Types::arrayType(ty.clone());
                    b3 = Types::extendsBasicType(ty.clone());
                    let true = (boolOr(b1.clone(), boolOr(b2.clone(), b3.clone()))) else { bail!("pattern mismatch") };
                    ErrorExt::rollBack((literal!("instBasictypeBaseclass")).clone());
                    Ok((cache.clone(), ih.clone(), store.clone(), dae.clone(), Some(ty.clone()), tys.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: path, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    rollbackCheck(path.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { .. }, tail: Deref @ metamodelica::List::Nil }, _, mods, inst_dims, _, _, _) => {
                    let false = (inSCodeElementLst3.clone().is_empty()) else { bail!("pattern mismatch") };
                    ErrorExt::setCheckpoint((literal!("instBasictypeBaseclass2")).clone());
                    instBasictypeBaseclass2(cache.clone(), env.clone(), ih.clone(), store.clone(), inSCodeElementLst2.clone(), inSCodeElementLst3.clone(), mods.clone(), inst_dims.clone(), (className.clone()).clone(), info.clone(), stopInst.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outIH, outStore, outDae, outTypesTypeOption, outTypeVars))
}

fn rollbackCheck(mut p: Arc<Absyn::Path>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = p.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut n: ArcStr = arcstr::literal!("");
                    n = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    let true = (InstUtil::isBuiltInClass((n.clone()).clone())) else { bail!("pattern mismatch") };
                    ErrorExt::rollBack((literal!("instBasictypeBaseclass")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack((literal!("instBasictypeBaseclass")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn instBasictypeBaseclass2(mut inCache: FCore::Cache, mut inEnv1: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut store: UnitAbsyn::InstStore, mut inSCodeElementLst2: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inSCodeElementLst3: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inMod4: Arc<DAE::Mod>, mut inInstDims5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut className: ArcStr, mut inInfo: SourceInfo, mut stopInst: Mutable::Mutable<bool>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inCache.clone(), inEnv1.clone(), inIH.clone(), store.clone(), inSCodeElementLst2.clone(), inSCodeElementLst3.clone(), inMod4.clone(), inInstDims5.clone(), className.clone(), stopInst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { info, modifications: r#mod, baseClassPath: path, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _, inst_dims, _, _) => {
                    let mut m_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cdef: Arc<SCode::Element>;
                    let mut cdef_1: Arc<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut classname: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, m_1) = Mod::elabModForBasicType(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::DERIVED { path: path.clone() }, inInfo.clone())?;
                    (cache, cdef, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), Some(info.clone()))?;
                    cdef_1 = SCodeUtil::classSetPartial(cdef.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    (cache, _, ih, _, _, _, ty, _, _, _) = instClass(cache.clone(), cenv.clone(), ih.clone(), store.clone(), m_1.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef_1.clone(), inst_dims.clone(), false, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone())?;
                    b1 = Types::basicType(ty.clone());
                    b2 = Types::arrayType(ty.clone());
                    let true = (boolOr(b1.clone(), b2.clone())) else { bail!("pattern mismatch") };
                    classname = (FGraph::printGraphPathStr(env.clone())?).clone();
                    ErrorExt::rollBack((literal!("instBasictypeBaseclass2")).clone());
                    Error::addSourceMessage(Error::INHERIT_BASIC_WITH_COMPS.clone(), list![(classname.clone()).clone()], inInfo.clone())?;
                    Mutable::update(stopInst.clone(), true);
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack((literal!("instBasictypeBaseclass2")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn partialInstClassdef(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inClass: Arc<SCode::Element>, mut inClassDef: Arc<SCode::ClassDef>, mut inVisibility: SCode::Visibility, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut numIter: i32) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outState: ClassInf::State;
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    (outCache, outEnv, outIH, outState, outVars) = ({
        let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            let mut partial_prefix: SCode::Partial = SCode::Partial::NOT_PARTIAL;
            let mut class_name: ArcStr = arcstr::literal!("");
            let mut cdef_els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut class_ext_els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut extends_els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut emods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut ext_comps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
            let mut const_els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
            partial_prefix = SCodeUtil::getClassPartialPrefix(inClass.clone())?;
            partial_prefix = InstUtil::isPartial(partial_prefix.clone(), inMod.clone());
            class_name = (SCodeUtil::elementName(inClass.clone())?).clone();
            outState = ClassInfUtil::trans(inState.clone(), openmodelica_frontend_types::ClassInf::Event::NEWDEF)?;
            (cdef_els, class_ext_els, extends_els, _) = InstUtil::splitElts(var_field!((*inClassDef).elementLst, SCode::ClassDef::PARTS).clone())?;
            extends_els = SCodeInstUtil::addRedeclareAsElementsToExtends(extends_els.clone(), List::select(var_field!((*inClassDef).elementLst, SCode::ClassDef::PARTS).clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isRedeclareElement, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>)))?;
            (outCache, outEnv, outIH) = InstUtil::addClassdefsToEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), cdef_els.clone(), true, Some(inMod.clone()), FGraph::isEmptyScope(inEnv.clone()))?;
            (outCache, outEnv, outIH, emods, ext_comps, _, _, _, _, _) = InstExtends::instExtendsAndClassExtendsList(outCache.clone(), outEnv.clone(), outIH.clone(), inMod.clone(), inPrefix.clone(), extends_els.clone(), class_ext_els.clone(), var_field!((*inClassDef).elementLst, SCode::ClassDef::PARTS).clone(), inState.clone(), (class_name.clone()).clone(), true, true)?;
            const_els = listAppend(ext_comps.clone(), InstUtil::addNomod(InstUtil::constantEls(var_field!((*inClassDef).elementLst, SCode::ClassDef::PARTS).clone())?));
            r#mod = Mod::merge(inMod.clone(), emods.clone(), (class_name.clone()).clone(), true)?;
            (cdef_els, ext_comps) = InstUtil::classdefElts2(ext_comps.clone(), partial_prefix.clone())?;
            (outCache, outEnv, outIH) = InstUtil::addClassdefsToEnv(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), cdef_els.clone(), true, Some(r#mod.clone()), false)?;
            (outCache, outEnv, outIH) = InstUtil::addComponentsToEnv(outCache.clone(), outEnv.clone(), outIH.clone(), r#mod.clone(), inPrefix.clone(), inState.clone(), const_els.clone(), false)?;
            (outCache, outEnv, outIH, _, _, _, outState, outVars, _, _) = instElementList(outCache.clone(), outEnv.clone(), outIH.clone(), UnitAbsyn::noStore().clone(), r#mod.clone(), inPrefix.clone(), outState.clone(), const_els.clone(), inInstDims.clone(), true, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), false)?;
            (outCache.clone(), outEnv.clone(), outIH.clone(), outState.clone(), outVars.clone())
        },
        Deref @ SCode::ClassDef::DERIVED { modifications: class_mod, typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: class_dims, path: class_path }, .. } => {
            let mut class_name: ArcStr = arcstr::literal!("");
            let mut scope_str: ArcStr = arcstr::literal!("");
            let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut cls: Arc<SCode::Element>;
            let mut cdef: Arc<SCode::ClassDef>;
            let mut cenv: FCore::Graph;
            let mut parent_env: FCore::Graph;
            let mut der_re: SCode::Restriction = SCode::Restriction::R_BLOCK;
            let mut parent_re: SCode::Restriction = SCode::Restriction::R_BLOCK;
            let mut enc: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut eq: Option<DAE::EqMod> = None;
            let mut has_dims: bool = false;
            let mut is_basic_type: bool = false;
            let mut inst_dims: InstDims = metamodelica::nil();
            let mut scope_ty: Option<FCore::ScopeType> = None;
            info = SCodeUtil::elementInfo(inClass.clone());
            has_dims = !(isNone(class_dims.clone()) || class_dims.clone() == Some(metamodelica::nil()));
            match '__try0: {
                let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(unwrap_break_err!(Lookup::lookupClass(inCache.clone(), inEnv.clone(), class_path.clone(), Some(info.clone())), '__try0)) {
                    (__pa1, __pa2 @ Deref @ SCode::Element::CLASS { .. }, __pa3) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                outCache = __pa1.clone();
                cls = __pa2.clone();
                cenv = __pa3.clone();
                Ok::<_, anyhow::Error>((cenv.clone(), cls.clone(), outCache.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                    cenv = __try0_o0;
                    cls = __try0_o1;
                    outCache = __try0_o2;
                }
                Err(_) => {
                    class_name = (AbsynUtil::pathString(class_path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    scope_str = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                    Error::addSourceMessageAndFail(Error::LOOKUP_ERROR.clone(), list![(class_name.clone()).clone(), (scope_str.clone()).clone()], info.clone())?;
                    bail!("try/else: outputs not set in else branch");
                }
            }
            let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(cls.clone()) {
                Deref @ SCode::Element::CLASS { restriction: __pa4, encapsulatedPrefix: __pa5, name: __pa6, .. } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            der_re = __pa4.clone();
            enc = __pa5.clone();
            class_name = __pa6.clone();
            parent_re = SCodeUtil::getClassRestriction(inClass.clone())?;
            is_basic_type = InstUtil::checkDerivedRestriction(parent_re.clone(), der_re.clone(), (class_name.clone()).clone());
            smod = InstUtil::chainRedeclares(inMod.clone(), class_mod.clone())?;
            (parent_env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
            (outCache, r#mod) = Mod::elabMod(outCache.clone(), parent_env.clone(), inIH.clone(), inPrefix.clone(), smod.clone(), false, Mod::ModScope::DERIVED { path: class_path.clone() }, info.clone())?;
            r#mod = Mod::merge(inMod.clone(), r#mod.clone(), (class_name.clone()).clone(), true)?;
            if has_dims.clone() && !(is_basic_type.clone()) {
                cls = SCodeUtil::setClassName((class_name.clone()).clone(), cls.clone())?;
                eq = Mod::modEquation(r#mod.clone())?;
                (outCache, dims) = InstUtil::elabArraydimOpt(outCache.clone(), parent_env.clone(), Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("")).clone(), subscripts: metamodelica::nil() }), class_path.clone(), class_dims.clone(), eq.clone(), false, true, inPrefix.clone(), info.clone(), inInstDims.clone())?;
                inst_dims = List::appendLastList(inInstDims.clone(), dims.clone())?;
            } else {
                inst_dims = inInstDims.clone();
            }
            if is_basic_type.clone() || has_dims.clone() {
                scope_ty = if (is_basic_type.clone()) {FGraph::restrictionToScopeType(der_re.clone())} else {FGraph::classInfToScopeType(inState.clone())};
                cenv = FGraph::openScope(cenv.clone(), enc.clone(), (class_name.clone()).clone(), scope_ty.clone())?;
                outState = ClassInfUtil::start(der_re.clone(), FGraph::getGraphName(cenv.clone())?)?;
                (outCache, outEnv, outIH, outState, outVars) = partialInstClassIn(outCache.clone(), cenv.clone(), inIH.clone(), r#mod.clone(), inPrefix.clone(), outState.clone(), cls.clone(), inVisibility.clone(), inst_dims.clone(), numIter.clone())?;
            } else {
                cdef = Arc::new(SCode::ClassDef::PARTS { elementLst: list![Arc::new(SCode::Element::EXTENDS { baseClassPath: class_path.clone(), visibility: inVisibility.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), ann: None, info: info.clone() })], normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None });
                (outCache, outEnv, outIH, outState, outVars) = partialInstClassdef(outCache.clone(), inEnv.clone(), inIH.clone(), r#mod.clone(), inPrefix.clone(), inState.clone(), inClass.clone(), cdef.clone(), inVisibility.clone(), inInstDims.clone(), numIter.clone())?;
            }
            if SCodeUtil::isPartial(cls.clone()) {
                outEnv = FGraph::makeScopePartial(inEnv.clone());
            }
            (outCache.clone(), outEnv.clone(), outIH.clone(), outState.clone(), outVars.clone())
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok((outCache, outEnv, outIH, outState, outVars))
}

pub fn instElementList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImplInst: bool, mut inCallingScope: InstTypes::CallingScope, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets, mut inStopOnError: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, ConnectionGraph::ConnectionGraph, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outStore: UnitAbsyn::InstStore = inStore.clone();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets = inSets.clone();
    let mut outState: ClassInf::State = inState.clone();
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph.clone();
    let mut domainFieldsListOut: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>> = metamodelica::nil();
    let mut el: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut dae: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut varsl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Var>>>>> = metamodelica::nil();
    let mut dael: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
    let mut fieldDomOpt: Option<(Arc<Absyn::ComponentRef>, Arc<DAE::ComponentRef>)> = None;
    let mut element_order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut el_arr: metamodelica::Array<(Arc<SCode::Element>, Arc<DAE::Mod>)>;
    let mut var_arr: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Var>>>>;
    let mut dae_arr: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Element>>>>;
    let mut length: i32 = 0;
    cache = InstUtil::pushStructuralParameters(inCache.clone());
    el = InstUtil::sortElementList(inElements.clone(), inEnv.clone(), FGraph::inFunctionScope(inEnv.clone())?)?;
    el = InstUtil::sortInnerFirstTplLstElementMod(el.clone())?;
    if !(ClassInfUtil::isFunction(inState.clone())) {
        element_order = getSortedElementOrdering(inElements.clone(), el.clone())?;
        el_arr = metamodelica::arrayFromVec(inElements.clone().into_iter().cloned().collect());
        length = (el.clone().len() as i32);
        var_arr = arrayCreate(length.clone(), metamodelica::nil());
        dae_arr = arrayCreate(length.clone(), metamodelica::nil());
        for mut idx in &*element_order.clone() {
            let mut idx = idx.clone();
            (cache, outEnv, outIH, outStore, dae, outSets, outState, vars, outGraph, fieldDomOpt) = instElement2(cache.clone(), outEnv.clone(), outIH.clone(), outStore.clone(), inMod.clone(), inPrefix.clone(), outState.clone(), el_arr.borrow()[(idx.clone()-1) as usize].clone(), inInstDims.clone(), inImplInst.clone(), inCallingScope.clone(), outGraph.clone(), outSets.clone(), inStopOnError.clone())?;
            {let _arr = var_arr.clone(); _arr.borrow_mut()[(length.clone() - idx.clone() + 1-1) as usize] = vars.clone(); _arr};
            {let _arr = dae_arr.clone(); _arr.borrow_mut()[(length.clone() - idx.clone() + 1-1) as usize] = dae.clone(); _arr};
            if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                domainFieldsListOut = InstUtil::optAppendField(domainFieldsListOut.clone(), fieldDomOpt.clone());
            }
        }
        outVars = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut lst in (var_arr.clone()).borrow().iter() {
            let __x = lst.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    };
        outDae = DAE::DAElist { elementLst: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut lst in (dae_arr.clone()).borrow().iter() {
            let __x = lst.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    } };
        GCExt::free(var_arr.clone());
        GCExt::free(dae_arr.clone());
    } else {
        for mut e in &*el.clone() {
            let mut e = e.clone();
            (cache, outEnv, outIH, outStore, dae, outSets, outState, vars, outGraph, fieldDomOpt) = instElement2(cache.clone(), outEnv.clone(), outIH.clone(), outStore.clone(), inMod.clone(), inPrefix.clone(), outState.clone(), e.clone(), inInstDims.clone(), inImplInst.clone(), inCallingScope.clone(), outGraph.clone(), outSets.clone(), inStopOnError.clone())?;
            varsl = cons(vars.clone(), varsl.clone());
            dael = cons(dae.clone(), dael.clone());
        }
        outVars = List::flattenReverse(varsl.clone());
        outDae = DAE::DAElist { elementLst: List::flattenReverse(dael.clone()) };
    }
    outCache = InstUtil::popStructuralParameters(cache.clone(), inPrefix.clone())?;
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outVars, outGraph, domainFieldsListOut))
}

fn getSortedElementOrdering(mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inSortedElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut index_map: Arc<metamodelica::List<(Arc<SCode::Element>, i32)>> = metamodelica::nil();
    let mut sorted_el: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut i: i32 = 1;
    for mut e in &*inElements.clone() {
        let mut e = e.clone();
        index_map = cons((Util::tuple21(e.clone()), i.clone()), index_map.clone());
        i = i.clone() + 1;
    }
    index_map = index_map.clone().reverse();
    sorted_el = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (inSortedElements.clone()).into_iter().cloned() {
            let __x = Util::tuple21(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    for mut e in &*sorted_el.clone() {
        let mut e = e.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::deleteMemberOnTrue(e.clone(), index_map.clone(), (std::sync::Arc::new(fnptr!(getSortedElementOrdering_comp, Arc<SCode::Element>, (Arc<SCode::Element>, i32))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, (Arc<SCode::Element>, i32)) -> Result<bool> + 'static>))?) {
            (__pa0, Some((_, __pa1))) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        index_map = __pa0.clone();
        i = __pa1.clone();
        outIndices = cons(i.clone(), outIndices.clone());
    }
    outIndices = outIndices.clone().reverse();
    Ok(outIndices)
}

fn getSortedElementOrdering_comp(mut inElement1: Arc<SCode::Element>, mut inElement2: (Arc<SCode::Element>, i32)) -> bool {
    let mut outEqual: bool = SCodeUtil::elementNameEqual(inElement1.clone(), Util::tuple21(inElement2.clone()));
    outEqual
}

pub fn instElement2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>), mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImplicit: bool, mut inCallingScope: InstTypes::CallingScope, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets, mut inStopOnError: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, Arc<metamodelica::List<Arc<DAE::Element>>>, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, ConnectionGraph::ConnectionGraph, Option<(Arc<Absyn::ComponentRef>, Arc<DAE::ComponentRef>)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outStore: UnitAbsyn::InstStore = inStore.clone();
    let mut outDae: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets = inSets.clone();
    let mut outState: ClassInf::State = inState.clone();
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph.clone();
    let mut outFieldDomOpt: Option<(Arc<Absyn::ComponentRef>, Arc<DAE::ComponentRef>)> = None;
    let mut elt: (Arc<SCode::Element>, Arc<DAE::Mod>);
    let mut is_deleted: bool = false;
    (is_deleted, outEnv, outCache) = isDeletedComponent(inElement.clone(), inPrefix.clone(), inStopOnError.clone(), inEnv.clone(), inCache.clone())?;
    if is_deleted.clone() {
        return Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outVars, outGraph, outFieldDomOpt));
    }
    match '__try0: {
        ErrorExt::setCheckpoint((literal!("instElement2")).clone());
        let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(updateCompeltsMods(inCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), list![inElement.clone()], outState.clone(), inImplicit.clone()), '__try0)) {
            (__pa1, __pa2, __pa3, Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        outCache = __pa1.clone();
        outEnv = __pa2.clone();
        outIH = __pa3.clone();
        elt = __pa4.clone();
        let (__pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15) = ::match_deref::match_deref! { match &(unwrap_break_err!(instElement(outCache.clone(), outEnv.clone(), outIH.clone(), outStore.clone(), inMod.clone(), inPrefix.clone(), outState.clone(), elt.clone(), inInstDims.clone(), inImplicit.clone(), inCallingScope.clone(), outGraph.clone(), inSets.clone()), '__try0)) {
            (__pa6, __pa7, __pa8, __pa9, DAE::DAElist { elementLst: __pa10 }, __pa11, __pa12, __pa13, __pa14, __pa15) => (__pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        outCache = __pa6.clone();
        outEnv = __pa7.clone();
        outIH = __pa8.clone();
        outStore = __pa9.clone();
        outDae = __pa10.clone();
        outSets = __pa11.clone();
        outState = __pa12.clone();
        outVars = __pa13.clone();
        outGraph = __pa14.clone();
        outFieldDomOpt = __pa15.clone();
        unwrap_break_err!(Error::clearCurrentComponent(), '__try0);
        ErrorExt::delCheckpoint((literal!("instElement2")).clone());
        Ok::<_, anyhow::Error>((elt.clone(), outCache.clone(), outDae.clone(), outEnv.clone(), outFieldDomOpt.clone(), outGraph.clone(), outIH.clone(), outSets.clone(), outState.clone(), outStore.clone(), outVars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10)) => {
            elt = __try0_o0;
            outCache = __try0_o1;
            outDae = __try0_o2;
            outEnv = __try0_o3;
            outFieldDomOpt = __try0_o4;
            outGraph = __try0_o5;
            outIH = __try0_o6;
            outSets = __try0_o7;
            outState = __try0_o8;
            outStore = __try0_o9;
            outVars = __try0_o10;
        }
        Err(_) => {
            if inStopOnError.clone() {
                ErrorExt::delCheckpoint((literal!("instElement2")).clone());
                bail!("fail");
            } else {
                ErrorExt::rollBack((literal!("instElement2")).clone());
                outCache = inCache.clone();
                outEnv = inEnv.clone();
                outIH = inIH.clone();
                return Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outVars, outGraph, outFieldDomOpt));
            }
        }
    }
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outState, outVars, outGraph, outFieldDomOpt))
}

fn isDeletedComponent(mut element: (Arc<SCode::Element>, Arc<DAE::Mod>), mut prefix: DAE::Prefix, mut stopOnError: bool, mut env: FCore::Graph, mut cache: FCore::Cache) -> Result<(bool, FCore::Graph, FCore::Cache)> {
    let mut isDeleted: bool = false;
    let mut env: FCore::Graph = env;
    let mut cache: FCore::Cache = cache;
    let mut el: Arc<SCode::Element>;
    let mut el_name: ArcStr = arcstr::literal!("");
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut cond_val_opt: Option<bool> = None;
    let mut cond_val: bool = false;
    let mut var: Arc<DAE::Var>;
    if InstUtil::componentHasCondition(element.clone()) {
        (el, _) = element.clone();
        (el_name, info) = InstUtil::extractCurrentName(el.clone())?;
        if SCodeUtil::isElementRedeclare(el.clone())? {
            Error::addSourceMessage(Error::REDECLARE_CONDITION.clone(), list![(el_name.clone()).clone()], info.clone())?;
            bail!("fail");
        }
        (cond_val_opt, cache) = InstUtil::instElementCondExp(cache.clone(), env.clone(), el.clone(), prefix.clone(), info.clone())?;
        if isNone(cond_val_opt.clone()) {
            if stopOnError.clone() {
                bail!("fail");
            } else {
                isDeleted = false;
                return Ok((isDeleted, env, cache));
            }
        }
        let Some(__pa0) = (cond_val_opt.clone()) else { bail!("pattern mismatch") };
        cond_val = __pa0.clone();
        isDeleted = !(cond_val.clone());
        if isDeleted.clone() == true {
            var = Arc::new(DAE::Var { name: (el_name.clone()).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None });
            env = FGraph::updateComp(env.clone(), var.clone(), crate::FCore::Status::VAR_DELETED, FGraph::emptyGraph().clone())?;
        }
    } else {
        isDeleted = false;
    }
    Ok((isDeleted, env, cache))
}

pub fn instElement(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inUnitStore: UnitAbsyn::InstStore, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>), mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImplicit: bool, mut inCallingScope: InstTypes::CallingScope, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, ConnectionGraph::ConnectionGraph, Option<(Arc<Absyn::ComponentRef>, Arc<DAE::ComponentRef>)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outUnitStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut outFieldDomOpt: Option<(Arc<Absyn::ComponentRef>, Arc<DAE::ComponentRef>)> = None;
    (outCache, outEnv, outIH, outUnitStore, outDae, outSets, outState, outVars, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inUnitStore.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), inElement.clone(), inInstDims.clone(), inImplicit.clone(), inCallingScope.clone(), inGraph.clone(), inSets.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, (Deref @ SCode::Element::IMPORT { .. }, _), _, _, _, _, _) => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), inUnitStore.clone(), DAE::emptyDae().clone(), inSets.clone(), inState.clone(), metamodelica::nil(), inGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, (cls @ Deref @ SCode::Element::CLASS { .. }, cmod), _, _, _, _, _) => {
                    let mut env: FCore::Graph;
                    if !(Mod::isEmptyMod(cmod.clone())) {
                        env = FGraph::updateClass(inEnv.clone(), cls.clone(), inPrefix.clone(), cmod.clone(), crate::FCore::Status::CLS_UNTYPED, inEnv.clone())?;
                    } else {
                        env = inEnv.clone();
                    }
                    Ok((inCache.clone(), env.clone(), inIH.clone(), inUnitStore.clone(), DAE::emptyDae().clone(), inSets.clone(), inState.clone(), metamodelica::nil(), inGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, (el @ Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, name, .. }, cmod), inst_dims, r#impl, _, graph, csets) => {
                    let mut own_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut t: Arc<Absyn::Path>;
                    let mut ts: Arc<Absyn::TypeSpec>;
                    let mut already_declared: bool = false;
                    let mut is_function_input: bool = false;
                    let mut graph_new: ConnectionGraph::ConnectionGraph;
                    let mut dae_attr: Arc<DAE::Attributes>;
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cref2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut dae: DAE::DAElist;
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut class_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mm: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut var_class_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut m_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cls_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut new_var: Arc<DAE::Var>;
                    let mut env2: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut comp_env: FCore::Graph;
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs3: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut cond: Option<Arc<Absyn::Exp>> = None;
                    let mut eq: Option<DAE::EqMod> = None;
                    let mut comment: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
                    let mut attr: SCode::Attributes;
                    let mut cls: Arc<SCode::Element>;
                    let mut comp: Arc<SCode::Element>;
                    let mut final_prefix: SCode::Final = SCode::Final::FINAL;
                    let mut m: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut oldmod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut prefixes: Arc<SCode::Prefixes>;
                    let mut topInstance: InnerOuter::TopInstance;
                    let mut sm: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut isInSM: bool = false;
                    let mut elems: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut ci_state = (*ci_state).clone();
                    let mut name = (*name).clone();
                    let mut inst_dims = (*inst_dims).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let mut outFieldDomOpt: Option<(Arc<Absyn::ComponentRef>, Arc<DAE::ComponentRef>)> = outFieldDomOpt.clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa5, __pa4, __pa7, __pa6, __pa10, __pa8, __pa9, __pa11) = ::match_deref::match_deref! { match &(el.clone()) {
                        Deref @ SCode::Element::COMPONENT { info: __pa0, condition: __pa1, comment: __pa2, modifications: __pa3, typeSpec: __pa5 @ Deref @ Absyn::TypeSpec::TPATH { path: __pa4, .. }, attributes: __pa7 @ SCode::Attributes { arrayDims: __pa6, .. }, prefixes: __pa10 @ Deref @ SCode::Prefixes { innerOuter: __pa8, finalPrefix: __pa9, .. }, name: __pa11 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone(), __pa4.clone(), __pa7.clone(), __pa6.clone(), __pa10.clone(), __pa8.clone(), __pa9.clone(), __pa11.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa0.clone();
                    cond = __pa1.clone();
                    comment = __pa2.clone();
                    m = __pa3.clone();
                    t = __pa4.clone();
                    ts = __pa5.clone();
                    ad = __pa6.clone();
                    attr = __pa7.clone();
                    io = __pa8.clone();
                    final_prefix = __pa9.clone();
                    prefixes = __pa10.clone();
                    name = __pa11.clone();
                    let true = (if (Config::acceptParModelicaGrammar()?) {InstUtil::checkParallelismWRTEnv(env.clone(), (name.clone()).clone(), attr.clone(), info.clone())?} else {true}) else { bail!("pattern mismatch") };
                    m = SCodeUtil::mergeModifiers(m.clone(), SCodeUtil::getConstrainedByModifiers(prefixes.clone()))?;
                    if SCodeUtil::finalBool(final_prefix.clone())? {
                        m = InstUtil::traverseModAddFinal(m.clone())?;
                    }
                    comp = if (referenceEq(&el.modifications.clone(),&m.clone())) {el.clone()} else {Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: ts.clone(), modifications: m.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() })};
                    oldmod = m.clone();
                    already_declared = InstUtil::checkMultiplyDeclared(cache.clone(), env.clone(), mods.clone(), pre.clone(), ci_state.clone(), (comp.clone(), cmod.clone()), inst_dims.clone(), r#impl.clone())?;
                    m = InstUtil::chainRedeclares(mods.clone(), m.clone())?;
                    m = SCodeInstUtil::expandEnumerationMod(m.clone())?;
                    m = InstUtil::traverseModAddDims(cache.clone(), env.clone(), pre.clone(), m.clone(), inst_dims.clone())?;
                    comp = if (referenceEq(&oldmod.clone(),&m.clone())) {comp.clone()} else {Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: ts.clone(), modifications: m.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() })};
                    ci_state = ClassInfUtil::trans(ci_state.clone(), ClassInf::Event::FOUND_COMPONENT { name: (name.clone()).clone() })?;
                    cref = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    (cache, _) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), cref.clone())?;
                    class_mod = Mod::lookupModificationP(mods.clone(), t.clone())?;
                    mm = Mod::lookupCompModification(mods.clone(), (name.clone()).clone())?;
                    own_cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() });
                    crefs1 = InstUtil::getCrefFromMod(m.clone())?;
                    crefs2 = InstUtil::getCrefFromDim(ad.clone())?;
                    crefs3 = InstUtil::getCrefFromCond(cond.clone())?;
                    crefs = List::unionList(list![crefs1.clone(), crefs2.clone(), crefs3.clone()])?;
                    (cache, env, ih, store, crefs) = removeSelfReferenceAndUpdate(cache.clone(), env.clone(), ih.clone(), store.clone(), crefs.clone(), own_cref.clone(), t.clone(), ci_state.clone(), attr.clone(), prefixes.clone(), r#impl.clone(), inst_dims.clone(), pre.clone(), mods.clone(), m.clone(), info.clone())?;
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), mods.clone(), crefs.clone(), ci_state.clone(), r#impl.clone());
                    (cache, class_mod) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), class_mod.clone(), r#impl.clone(), info.clone())?;
                    (cache, mm) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), mm.clone(), r#impl.clone(), info.clone())?;
                    (var_class_mod, class_mod) = modifyInstantiateClass(class_mod.clone(), t.clone())?;
                    (cache, m_1) = Mod::elabMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), m.clone(), r#impl.clone(), Mod::ModScope::COMPONENT { name: (name.clone()).clone() }, info.clone())?;
                    r#mod = Mod::merge(mm.clone(), class_mod.clone(), (name.clone()).clone(), true)?;
                    r#mod = Mod::merge(r#mod.clone(), m_1.clone(), (name.clone()).clone(), !(ClassInfUtil::isRecord(ci_state.clone())))?;
                    r#mod = Mod::merge(cmod.clone(), r#mod.clone(), (name.clone()).clone(), true)?;
                    r#mod = Mod::merge(r#mod.clone(), var_class_mod.clone(), (name.clone()).clone(), true)?;
                    let (__pa14, __pa15, __pa16, __pa17, __pa19, __pa18, __pa22, __pa20, __pa21, __pa23, __pa24, __pa25) = ::match_deref::match_deref! { match &(redeclareType(cache.clone(), env2.clone(), ih.clone(), r#mod.clone(), comp.clone(), pre.clone(), ci_state.clone(), r#impl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))?) {
                        (__pa14, __pa15, __pa16, Deref @ SCode::Element::COMPONENT { name: __pa17, prefixes: __pa19 @ Deref @ SCode::Prefixes { innerOuter: __pa18, .. }, attributes: __pa22 @ SCode::Attributes { direction: __pa20, arrayDims: __pa21, .. }, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa23, arrayDim: _ }, modifications: _, comment: __pa24, condition: _, info: _ }, __pa25) => (__pa14.clone(), __pa15.clone(), __pa16.clone(), __pa17.clone(), __pa19.clone(), __pa18.clone(), __pa22.clone(), __pa20.clone(), __pa21.clone(), __pa23.clone(), __pa24.clone(), __pa25.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa14.clone();
                    env2 = __pa15.clone();
                    ih = __pa16.clone();
                    name = __pa17.clone();
                    io = __pa18.clone();
                    prefixes = __pa19.clone();
                    dir = __pa20.clone();
                    ad = __pa21.clone();
                    attr = __pa22.clone();
                    t = __pa23.clone();
                    comment = __pa24.clone();
                    mod_1 = __pa25.clone();
                    (cache, cls, cenv) = Lookup::lookupClass(cache.clone(), env2.clone(), t.clone(), Some(info.clone()))?;
                    cls_mod = Mod::getClassModifier(cenv.clone(), (SCodeUtil::className(cls.clone())?).clone())?;
                    if !(Mod::isEmptyMod(cls_mod.clone())) {
                        if !(ad.clone().is_empty()) {
                            cls_mod = Mod::addEachIfNeeded(cls_mod.clone(), list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })])?;
                        }
                        mod_1 = Mod::merge(mod_1.clone(), cls_mod.clone(), (name.clone()).clone(), true)?;
                    }
                    attr = SCodeUtil::mergeAttributesFromClass(attr.clone(), cls.clone())?;
                    inst_dims = List::appendElt(metamodelica::nil(), inst_dims.clone());
                    (cache, r#mod) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), info.clone())?;
                    (cache, mod_1) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), mod_1.clone(), r#impl.clone(), info.clone())?;
                    (r#mod, mod_1) = InstUtil::selectModifiers(r#mod.clone(), mod_1.clone(), t.clone())?;
                    eq = Mod::modEquation(r#mod.clone())?;
                    is_function_input = InstUtil::isFunctionInput(ci_state.clone(), dir.clone());
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), env2.clone(), own_cref.clone(), t.clone(), ad.clone(), eq.clone(), r#impl.clone(), true, is_function_input.clone(), pre.clone(), info.clone(), inst_dims.clone())?;
                    if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                        (dims, mod_1, outFieldDomOpt) = InstUtil::elabField(inCache.clone(), inEnv.clone(), (name.clone()).clone(), attr.clone(), dims.clone(), mod_1.clone(), info.clone())?;
                    }
                    (cenv, cls, ih) = FGraph::createVersionScope(env2.clone(), (name.clone()).clone(), pre.clone(), mod_1.clone(), cenv.clone(), cls.clone(), ih.clone())?;
                    (cache, cref2) = PrefixUtil::prefixCref(cache.clone(), cenv.clone(), ih.clone(), pre.clone(), cref.clone())?;
                    if !(ih.clone().is_empty()) {
                        topInstance = listHead(ih.clone())?;
                        let InnerOuter::TOP_INSTANCE { sm: __pa28, .. } = (topInstance.clone()) else { bail!("pattern mismatch") };
                        sm = __pa28.clone();
                        if BaseHashSet::has(cref2.clone(), sm.clone())? {
                            isInSM = true;
                        } else {
                            isInSM = false;
                        }
                    } else {
                        isInSM = false;
                    }
                    (cache, comp_env, ih, store, dae, csets, ty, graph_new) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), ci_state.clone(), mod_1.clone(), pre.clone(), (name.clone()).clone(), cls.clone(), attr.clone(), prefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone(), env2.clone())?;
                    if isInSM.clone() {
                        let DAE::DAE { elementLst: __pa29 } = (dae.clone()) else { bail!("pattern mismatch") };
                        elems = __pa29.clone();
                        dae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::SM_COMP { componentRef: cref2.clone(), dAElist: elems.clone() })] };
                    }
                    (cache, binding) = InstBinding::makeBinding(cache.clone(), env2.clone(), attr.clone(), r#mod.clone(), ty.clone(), pre.clone(), (name.clone()).clone(), info.clone())?;
                    dae_attr = DAEUtil::translateSCodeAttrToDAEAttr(attr.clone(), prefixes.clone())?;
                    (ty, _) = Types::traverseType(ty.clone(), 1, (std::sync::Arc::new(fnptr!(Types::setIsFunctionPointer, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?;
                    binding = removePrefixFromBinding(binding.clone(), pre.clone())?;
                    new_var = Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: dae_attr.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env2.clone(), new_var.clone(), crate::FCore::Status::VAR_DAE, comp_env.clone())?;
                    vars = if (already_declared.clone()) {metamodelica::nil()} else {list![new_var.clone()]};
                    dae = if (already_declared.clone()) {DAE::emptyDae().clone()} else {dae.clone()};
                    (_, ih, graph) = InnerOuter::handleInnerOuterEquations(io.clone(), DAE::emptyDae().clone(), ih.clone(), graph_new.clone(), graph.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), vars.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, mods, pre, ci_state, (comp @ Deref @ SCode::Element::COMPONENT { name, prefixes: prefixes @ Deref @ SCode::Prefixes { innerOuter: io, finalPrefix: final_prefix, .. }, attributes: attr @ SCode::Attributes { connectorType: ct, arrayDims: ad, .. }, typeSpec: ts @ Deref @ Absyn::TypeSpec::TCOMPLEX { path: type_name, .. }, modifications: m, comment, condition: cond, info }, cmod), inst_dims, r#impl, _, graph, csets) => {
                    let mut own_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut already_declared: bool = false;
                    let mut graph_new: ConnectionGraph::ConnectionGraph;
                    let mut dae_attr: Arc<DAE::Attributes>;
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut dae: DAE::DAElist;
                    let mut m_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut new_var: Arc<DAE::Var>;
                    let mut comp_env: FCore::Graph;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut cls: Arc<SCode::Element>;
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut comp = (*comp).clone();
                    let mut m = (*m).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    if SCodeUtil::finalBool(final_prefix.clone())? {
                        m = InstUtil::traverseModAddFinal(m.clone())?;
                        comp = Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: ts.clone(), modifications: m.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() });
                    }
                    already_declared = InstUtil::checkMultiplyDeclared(cache.clone(), env.clone(), mods.clone(), pre.clone(), ci_state.clone(), (comp.clone(), cmod.clone()), inst_dims.clone(), r#impl.clone())?;
                    cref = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    (cache, _) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), cref.clone())?;
                    own_cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() });
                    (cache, m_1) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), m.clone(), r#impl.clone(), Mod::ModScope::COMPONENT { name: (name.clone()).clone() }, info.clone())?;
                    id = (AbsynUtil::pathString(type_name.clone(), (literal!(".")).clone(), true, false)?).clone();
                    cls = Arc::new(SCode::Element::CLASS { name: (id.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::DERIVED { typeSpec: ts.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), attributes: SCode::Attributes { arrayDims: ad.clone(), connectorType: ct.clone(), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } }), cmt: SCode::noComment.clone(), info: info.clone() });
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), env.clone(), own_cref.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("Integer")).clone() }), ad.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    (cache, comp_env, ih, store, dae, csets, ty, graph_new) = InstVar::instVar(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), m_1.clone(), pre.clone(), (name.clone()).clone(), cls.clone(), attr.clone(), prefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone(), env.clone())?;
                    (cache, binding) = InstBinding::makeBinding(cache.clone(), env.clone(), attr.clone(), m_1.clone(), ty.clone(), pre.clone(), (name.clone()).clone(), info.clone())?;
                    dae_attr = DAEUtil::translateSCodeAttrToDAEAttr(attr.clone(), prefixes.clone())?;
                    (ty, _) = Types::traverseType(ty.clone(), 1, (std::sync::Arc::new(fnptr!(Types::setIsFunctionPointer, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?;
                    new_var = Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: dae_attr.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), crate::FCore::Status::VAR_DAE, comp_env.clone())?;
                    vars = if (already_declared.clone()) {metamodelica::nil()} else {list![new_var.clone()]};
                    dae = if (already_declared.clone()) {DAE::emptyDae().clone()} else {dae.clone()};
                    (_, ih, graph) = InnerOuter::handleInnerOuterEquations(io.clone(), DAE::emptyDae().clone(), ih.clone(), graph_new.clone(), graph.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ci_state.clone(), vars.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _, pre, ci_state, (Deref @ SCode::Element::COMPONENT { info, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: t, arrayDim: _ }, attributes: SCode::Attributes { variability: vt, .. }, name, .. }, _), _, _, _, _, _) => {
                    let mut ns: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut scope_str: ArcStr = arcstr::literal!("");
                    let mut pre = (*pre).clone();
                    if '__try0: {
                        (_, _, _) = unwrap_break_err!(Lookup::lookupClass(cache.clone(), env.clone(), t.clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    s = (AbsynUtil::pathString(t.clone(), (literal!(".")).clone(), true, false)?).clone();
                    scope_str = (FGraph::printGraphPathStr(env.clone())?).clone();
                    pre = PrefixUtil::prefixAdd((name.clone()).clone(), metamodelica::nil(), metamodelica::nil(), pre.clone(), vt.clone(), ci_state.clone(), info.clone())?;
                    ns = (PrefixUtil::printPrefixStrIgnoreNoPre(pre.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_ERROR_COMPNAME.clone(), list![(s.clone()).clone(), (scope_str.clone()).clone(), (ns.clone()).clone()], info.clone())?;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Lookup class failed:")); __mm_s.push_str(&*AbsynUtil::pathString(t.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, _, _, (comp, _), _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instElement failed: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(comp.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outUnitStore, outDae, outSets, outState, outVars, outGraph, outFieldDomOpt))
}

fn removePrefixFromBinding(mut inBind: Arc<DAE::Binding>, mut inPrefix: DAE::Prefix) -> Result<Arc<DAE::Binding>> {
    let mut outBind: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    outBind = (::match_deref::match_deref! { match &((inBind.clone(), inPrefix.clone())) {
        (bind @ Deref @ DAE::Binding::EQBOUND { .. }, pref @ DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { .. }, .. }) => {
            let mut bind = (*bind).clone();
            assign_variant_field!(bind => DAE::Binding::EQBOUND; exp = PrefixUtil::removeCompPrefixFromExps(var_field!((*bind).exp, DAE::Binding::EQBOUND).clone(), var_field!(pref.compPre, DAE::Prefix::PREFIX).clone())?);
            bind.clone()
        },
        _ => {
            inBind.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBind)
}

fn updateCompeltsMods(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inState: ClassInf::State, mut inImplicit: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    (outCache, outEnv, outIH, outComponents) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inComponents.clone(), inState.clone(), inImplicit.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outEnv: FCore::Graph;
                    let mut outComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = outComponents.clone();
                    ErrorExt::setCheckpoint((literal!("updateCompeltsMods")).clone());
                    (outCache, outEnv, outIH, outComponents) = updateCompeltsMods_dispatch(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inComponents.clone(), inState.clone(), inImplicit.clone())?;
                    ErrorExt::rollBack((literal!("updateCompeltsMods")).clone());
                    Ok((outCache.clone(), outEnv.clone(), outIH.clone(), outComponents.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack((literal!("updateCompeltsMods")).clone());
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), inComponents.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outComponents))
}

fn updateCompeltsMods_dispatch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inState: ClassInf::State, mut inImplicit: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    (outCache, outEnv, outIH, outComponents) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inComponents.clone(), inState.clone(), inImplicit.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((cache.clone(), env.clone(), ih.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::List::Cons { head: elMod @ (_, Deref @ DAE::Mod::NOMOD), tail: xs }, ci_state, r#impl) => {
                    let mut res: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    (cache, env, ih, res) = updateCompeltsMods_dispatch(cache.clone(), env.clone(), ih.clone(), pre.clone(), xs.clone(), ci_state.clone(), r#impl.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), cons(elMod.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::List::Cons { head: (comp, cmod @ Deref @ DAE::Mod::REDECL { element: redComp, .. }), tail: xs }, ci_state, r#impl) => {
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut umod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut cmod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cmod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut ltmod: Arc<metamodelica::List<Arc<DAE::Mod>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut fprefix: SCode::Final = SCode::Final::FINAL;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    info = SCodeUtil::elementInfo(redComp.clone());
                    umod = Mod::unelabMod(cmod.clone())?;
                    crefs = InstUtil::getCrefFromMod(umod.clone())?;
                    crefs_1 = InstUtil::getCrefFromCompDim(comp.clone())?;
                    crefs = List::unionOnTrue(crefs.clone(), crefs_1.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefEqual, Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>));
                    name = (SCodeUtil::elementName(comp.clone())?).clone();
                    cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() });
                    ltmod = List::map1(crefs.clone(), (std::sync::Arc::new(InstUtil::getModsForDep) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<DAE::Mod>> + 'static>), xs.clone());
                    cmod2 = List::fold2r(cons(cmod.clone(), ltmod.clone()), (std::sync::Arc::new(Mod::merge) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Mod>, Arc<DAE::Mod>, ArcStr, bool) -> Result<Arc<DAE::Mod>> + 'static>), (name.clone()).clone(), true, Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD));
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::elementPrefixes(comp.clone())?) {
                        Deref @ SCode::Prefixes { finalPrefix: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    fprefix = __pa0.clone();
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), cmod2.clone(), crefs.clone(), ci_state.clone(), r#impl.clone());
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env2.clone(), ih.clone(), pre.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: fprefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![Arc::new(DAE::SubMod { ident: (name.clone()).clone(), r#mod: cmod.clone() })], binding: None, info: info.clone() }), list![cref.clone()], ci_state.clone(), r#impl.clone());
                    (cache, cmod_1) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), cmod.clone(), r#impl.clone(), info.clone())?;
                    (cache, env3, ih, res) = updateCompeltsMods_dispatch(cache.clone(), env2.clone(), ih.clone(), pre.clone(), xs.clone(), ci_state.clone(), r#impl.clone())?;
                    Ok((cache.clone(), env3.clone(), ih.clone(), cons((comp.clone(), cmod_1.clone()), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::List::Cons { head: (comp, cmod @ Deref @ DAE::Mod::MOD { .. }), tail: xs }, ci_state, r#impl) => {
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut res: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut fprefix: SCode::Final = SCode::Final::FINAL;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let false = (Mod::isUntypedMod(cmod.clone())?) else { bail!("pattern mismatch") };
                    name = (SCodeUtil::elementName(comp.clone())?).clone();
                    cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() });
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::elementPrefixes(comp.clone())?) {
                        Deref @ SCode::Prefixes { finalPrefix: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    fprefix = __pa0.clone();
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: fprefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![Arc::new(DAE::SubMod { ident: (name.clone()).clone(), r#mod: cmod.clone() })], binding: None, info: cmod.info.clone() }), list![cref.clone()], ci_state.clone(), r#impl.clone());
                    (cache, env3, ih, res) = updateCompeltsMods_dispatch(cache.clone(), env2.clone(), ih.clone(), pre.clone(), xs.clone(), ci_state.clone(), r#impl.clone())?;
                    Ok((cache.clone(), env3.clone(), ih.clone(), cons((comp.clone(), cmod.clone()), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::List::Cons { head: (comp, cmod @ Deref @ DAE::Mod::MOD { .. }), tail: xs }, ci_state, r#impl) => {
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut umod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut cmod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cmod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut ltmod: Arc<metamodelica::List<Arc<DAE::Mod>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut fprefix: SCode::Final = SCode::Final::FINAL;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    info = SCodeUtil::elementInfo(comp.clone());
                    umod = Mod::unelabMod(cmod.clone())?;
                    crefs = InstUtil::getCrefFromMod(umod.clone())?;
                    crefs_1 = InstUtil::getCrefFromCompDim(comp.clone())?;
                    crefs = List::unionOnTrue(crefs.clone(), crefs_1.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefEqual, Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>));
                    name = (SCodeUtil::elementName(comp.clone())?).clone();
                    cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() });
                    ltmod = List::map1(crefs.clone(), (std::sync::Arc::new(InstUtil::getModsForDep) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<DAE::Mod>> + 'static>), xs.clone());
                    cmod2 = List::fold2r(ltmod.clone(), (std::sync::Arc::new(Mod::merge) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Mod>, Arc<DAE::Mod>, ArcStr, bool) -> Result<Arc<DAE::Mod>> + 'static>), (name.clone()).clone(), true, Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD));
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::elementPrefixes(comp.clone())?) {
                        Deref @ SCode::Prefixes { finalPrefix: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    fprefix = __pa0.clone();
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env.clone(), ih.clone(), pre.clone(), cmod2.clone(), crefs.clone(), ci_state.clone(), r#impl.clone());
                    (cache, env2, ih) = updateComponentsInEnv(cache.clone(), env2.clone(), ih.clone(), pre.clone(), Arc::new(DAE::Mod::MOD { finalPrefix: fprefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![Arc::new(DAE::SubMod { ident: (name.clone()).clone(), r#mod: cmod.clone() })], binding: None, info: cmod.info.clone() }), list![cref.clone()], ci_state.clone(), r#impl.clone());
                    (cache, cmod_1) = Mod::updateMod(cache.clone(), env2.clone(), ih.clone(), pre.clone(), cmod.clone(), r#impl.clone(), info.clone())?;
                    (cache, env3, ih, res) = updateCompeltsMods_dispatch(cache.clone(), env2.clone(), ih.clone(), pre.clone(), xs.clone(), ci_state.clone(), r#impl.clone())?;
                    Ok((cache.clone(), env3.clone(), ih.clone(), cons((comp.clone(), cmod_1.clone()), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outComponents))
}

pub fn redeclareType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inElement: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inImpl: bool, mut inCmod: Arc<DAE::Mod>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<SCode::Element>, Arc<DAE::Mod>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outElement: Arc<SCode::Element> = inElement.clone();
    let mut outMod: Arc<DAE::Mod> = Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD);
    let mut redecl_el: Arc<SCode::Element>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut redecl_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut old_m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut redecl_name: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut found: bool = false;
    let mut repl: Arc<SCode::Replaceable> = Arc::new(SCode::Replaceable::NOT_REPLACEABLE);
    let mut cc: Option<Arc<SCode::ConstrainClass>> = None;
    let mut cc_comps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    if !(Mod::isRedeclareMod(inMod.clone())) {
        outMod = Mod::merge(inMod.clone(), inCmod.clone(), (literal!("")).clone(), true)?;
        return Ok((outCache, outEnv, outIH, outElement, outMod));
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::REDECL { r#mod: __pa0, element: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    redecl_mod = __pa0.clone();
    redecl_el = __pa1.clone();
    redecl_name = (SCodeUtil::elementName(redecl_el.clone())?).clone();
    (outElement, outMod) = 'mc: {
        let __mc_input = (redecl_el.clone(), inElement.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: repl, .. }, .. }) => {
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = crefs.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut m: Arc<DAE::Mod> = m.clone();
                    let mut redecl_mod: Arc<DAE::Mod> = redecl_mod.clone();
                    let mut old_m: Arc<DAE::Mod> = old_m.clone();
                    let mut r#mod: Arc<SCode::Mod> = r#mod.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut cc_comps: Arc<metamodelica::List<Arc<SCode::Element>>> = cc_comps.clone();
                    let mut outElement: Arc<SCode::Element> = outElement.clone();
                    let true = (redecl_name.clone() == var_field!((*inElement).name, SCode::Element::COMPONENT).clone()) else { bail!("pattern mismatch") };
                    r#mod = InstUtil::chainRedeclares(inMod.clone(), var_field!((*redecl_el).modifications, SCode::Element::COMPONENT).clone())?;
                    crefs = InstUtil::getCrefFromMod(r#mod.clone())?;
                    (outCache, outEnv, outIH) = updateComponentsInEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crefs.clone(), inState.clone(), inImpl.clone());
                    (outCache, m) = Mod::elabMod(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), r#mod.clone(), inImpl.clone(), Mod::ModScope::COMPONENT { name: (redecl_name.clone()).clone() }, var_field!((*redecl_el).info, SCode::Element::COMPONENT).clone())?;
                    (outCache, old_m) = Mod::elabMod(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), var_field!((*inElement).modifications, SCode::Element::COMPONENT).clone(), inImpl.clone(), Mod::ModScope::COMPONENT { name: (var_field!((*inElement).name, SCode::Element::COMPONENT).clone()).clone() }, var_field!((*inElement).info, SCode::Element::COMPONENT).clone())?;
                    m = (::match_deref::match_deref! { match &(repl.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: cc @ Some(_) } => {
                    cc_comps = InstUtil::extractConstrainingComps(cc.clone(), inEnv.clone(), inPrefix.clone())?;
                    redecl_mod = InstUtil::keepConstrainingTypeModifersOnly(redecl_mod.clone(), cc_comps.clone())?;
                    old_m = InstUtil::keepConstrainingTypeModifersOnly(old_m.clone(), cc_comps.clone())?;
                    m = Mod::merge(m.clone(), redecl_mod.clone(), (redecl_name.clone()).clone(), true)?;
                    m = Mod::merge(m.clone(), old_m.clone(), (redecl_name.clone()).clone(), true)?;
                    m = Mod::merge(m.clone(), inCmod.clone(), (redecl_name.clone()).clone(), true)?;
                    m.clone()
        },
        _ => {
                    m = Mod::merge(redecl_mod.clone(), m.clone(), (redecl_name.clone()).clone(), true)?;
                    m = Mod::merge(m.clone(), old_m.clone(), (redecl_name.clone()).clone(), true)?;
                    m = Mod::merge(inCmod.clone(), m.clone(), (redecl_name.clone()).clone(), true)?;
                    m.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    (outCache, outElement) = propagateRedeclCompAttr(outCache.clone(), outEnv.clone(), inElement.clone(), redecl_el.clone())?;
                    outElement = SCodeUtil::setComponentMod(outElement.clone(), r#mod.clone())?;
                    Ok((outElement.clone(), m.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::CLASS { .. }) => {
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let true = (redecl_name.clone() == var_field!((*inElement).name, SCode::Element::CLASS).clone()) else { bail!("pattern mismatch") };
                    (outCache, outEnv, outIH) = updateComponentsInEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), list![Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (var_field!((*inElement).name, SCode::Element::CLASS).clone()).clone(), subscripts: metamodelica::nil() })], inState.clone(), inImpl.clone());
                    Ok((inElement.clone(), redecl_mod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
                    let mut name: ArcStr = name.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    name = (AbsynUtil::typeSpecPathString(var_field!((*inElement).typeSpec, SCode::Element::COMPONENT).clone())).clone();
                    let true = (redecl_name.clone() == name.clone()) else { bail!("pattern mismatch") };
                    (outCache, outEnv, outIH) = updateComponentsInEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), list![Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() })], inState.clone(), inImpl.clone());
                    Ok((inElement.clone(), redecl_mod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut name: ArcStr = name.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    name = (AbsynUtil::pathFirstIdent(AbsynUtil::typeSpecPath(var_field!((*inElement).typeSpec, SCode::Element::COMPONENT).clone())?)?).clone();
                    let true = (redecl_name.clone() == name.clone()) else { bail!("pattern mismatch") };
                    (outCache, outEnv, outIH) = updateComponentsInEnv(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), list![Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() })], inState.clone(), inImpl.clone());
                    Ok((inElement.clone(), redecl_mod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inElement.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outElement, outMod))
}

fn propagateRedeclCompAttr(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inOldComponent: Arc<SCode::Element>, mut inNewComponent: Arc<SCode::Element>) -> Result<(FCore::Cache, Arc<SCode::Element>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outComponent: Arc<SCode::Element>;
    let mut is_array: bool = false;
    if SCodeUtil::isArrayComponent(inOldComponent.clone()) && !(SCodeUtil::isArrayComponent(inNewComponent.clone())) {
        (outCache, is_array) = Lookup::isArrayType(outCache.clone(), inEnv.clone(), SCodeUtil::getElementTypePath(inNewComponent.clone())?);
    }
    outComponent = SCodeUtil::propagateAttributesVar(inOldComponent.clone(), inNewComponent.clone(), is_array.clone())?;
    Ok((outCache, outComponent))
}

fn updateComponentsInEnv(mut cache: FCore::Cache, mut env: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut pre: DAE::Prefix, mut r#mod: Arc<DAE::Mod>, mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut ci_state: ClassInf::State, mut r#impl: bool) -> (FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>) {
    let mut outCache: FCore::Cache = cache.clone();
    let mut outEnv: FCore::Graph = env.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    ErrorExt::setCheckpoint((literal!("updateComponentsInEnv__")).clone());
    if '__try0: {
        (outCache, outEnv, outIH, _) = unwrap_break_err!(updateComponentsInEnv2(cache.clone(), env.clone(), inIH.clone(), pre.clone(), r#mod.clone(), crefs.clone(), ci_state.clone(), r#impl.clone(), None, None), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    ErrorExt::rollBack((literal!("updateComponentsInEnv__")).clone());
    (outCache, outEnv, outIH)
}

fn getUpdatedCompsHashTable(mut optHT: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))>) -> (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr)) {
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr));
    ht = (match optHT.clone() {
        Some(mut ht) => ht.clone(),
        _ => HashTable5::emptyHashTableSized(BaseHashTable::lowBucketSize.clone()),
    });
    ht
}

fn updateComponentInEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut pre: DAE::Prefix, mut r#mod: Arc<DAE::Mod>, mut cref: Arc<Absyn::ComponentRef>, mut inCIState: ClassInf::State, mut r#impl: bool, mut inUpdatedComps: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))>, mut currentCref: Option<Arc<Absyn::ComponentRef>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outUpdatedComps: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))> = None;
    (outCache, outEnv, outIH, outUpdatedComps) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), pre.clone(), r#mod.clone(), cref.clone(), inCIState.clone(), r#impl.clone(), inUpdatedComps.clone(), currentCref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { info, modifications: smod, attributes, prefixes: prefixes @ Deref @ SCode::Prefixes { visibility, .. }, name, .. }, .. }, _, _, _, _, _) => {
                    let mut n: ArcStr = arcstr::literal!("");
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut ct: SCode::ConnectorType = SCode::ConnectorType::FLOW;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut attr: SCode::Attributes;
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut prl1: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
                    let mut var1: SCode::Variability = SCode::Variability::CONST;
                    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut t: Arc<Absyn::Path>;
                    let mut m: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut cmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cl: Arc<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs3: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs_2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut cond: Option<Arc<Absyn::Exp>> = None;
                    let mut pf: Arc<SCode::Prefixes>;
                    let mut daeMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut idENV: FCore::Graph;
                    let mut updatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr));
                    let mut ci_state: ClassInf::State;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut info = (*info).clone();
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    let true = (stringEq((id.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    let false = (smod.clone() == Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupIdentLocal(cache.clone(), env.clone(), (name.clone()).clone())?) {
                        (__pa0, Deref @ DAE::Var { name: _, attributes: _, ty: _, binding: _, bind_from_outside: _, .. }, _, _, _, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    (cache, daeMod) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), smod.clone(), r#impl.clone(), Mod::ModScope::COMPONENT { name: (name.clone()).clone() }, info.clone())?;
                    mods = daeMod.clone();
                    attr = attributes.clone();
                    m = smod.clone();
                    cmod = Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD);
                    pf = prefixes.clone();
                    io = SCodeUtil::prefixesInnerOuter(pf.clone())?;
                    let SCode::ATTR { arrayDims: __pa1, connectorType: __pa2, parallelism: __pa3, variability: __pa4, direction: __pa5, .. } = (attr.clone()) else { bail!("pattern mismatch") };
                    ad = __pa1.clone();
                    ct = __pa2.clone();
                    prl1 = __pa3.clone();
                    var1 = __pa4.clone();
                    dir = __pa5.clone();
                    let (__pa6, __pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?) {
                        (__pa6, _, Deref @ SCode::Element::COMPONENT { name: __pa7, prefixes: _, attributes: _, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa8, arrayDim: _ }, modifications: _, comment: _, condition: __pa9, info: __pa10 }, _, _, __pa11) => (__pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa6.clone();
                    n = __pa7.clone();
                    t = __pa8.clone();
                    cond = __pa9.clone();
                    info = __pa10.clone();
                    idENV = __pa11.clone();
                    ci_state = InstUtil::updateClassInfState(cache.clone(), idENV.clone(), env.clone(), inCIState.clone())?;
                    (cache, cl, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), t.clone(), None)?;
                    updatedComps = getUpdatedCompsHashTable(inUpdatedComps.clone());
                    (mods, cmod, m) = InstUtil::noModForUpdatedComponents(var1.clone(), updatedComps.clone(), cref.clone(), mods.clone(), cmod.clone(), m.clone())?;
                    crefs = InstUtil::getCrefFromMod(m.clone())?;
                    crefs2 = InstUtil::getCrefFromDim(ad.clone())?;
                    crefs3 = InstUtil::getCrefFromCond(cond.clone())?;
                    crefs_1 = listAppend(crefs.clone(), listAppend(crefs2.clone(), crefs3.clone()));
                    crefs_2 = InstUtil::removeCrefFromCrefs(crefs_1.clone(), cref.clone())?;
                    updatedComps = BaseHashTable::add((cref.clone(), 0), updatedComps.clone())?;
                    let (__pa13, __pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(updateComponentsInEnv2(cache.clone(), env.clone(), ih.clone(), pre.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crefs_2.clone(), ci_state.clone(), r#impl.clone(), Some(updatedComps.clone()), Some(cref.clone()))?) {
                        (__pa13, __pa14, __pa15, Some(__pa16)) => (__pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa13.clone();
                    env2 = __pa14.clone();
                    ih = __pa15.clone();
                    updatedComps = __pa16.clone();
                    (cache, env_1, ih, updatedComps) = updateComponentInEnv2(cache.clone(), env2.clone(), cenv.clone(), ih.clone(), pre.clone(), t.clone(), (n.clone()).clone(), ad.clone(), cl.clone(), attr.clone(), pf.clone(), Arc::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: visibility.clone() }), info.clone(), m.clone(), cmod.clone(), mods.clone(), cref.clone(), ci_state.clone(), r#impl.clone(), updatedComps.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), Some(updatedComps.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::CLASS { name, .. }, .. }, _, _, _, _, _) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut cl: Arc<SCode::Element>;
                    let mut updatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr));
                    let mut env = (*env).clone();
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    let true = (stringEq((name.clone()).clone(), (id.clone()).clone())) else { bail!("pattern mismatch") };
                    (cl, _) = Lookup::lookupClassLocal(env.clone(), (name.clone()).clone())?;
                    env = FGraph::updateClass(env.clone(), SCodeUtil::mergeWithOriginal(var_field!((*r#mod).element, DAE::Mod::REDECL).clone(), cl.clone())?, pre.clone(), r#mod.clone(), crate::FCore::Status::CLS_UNTYPED, env.clone())?;
                    updatedComps = getUpdatedCompsHashTable(inUpdatedComps.clone());
                    updatedComps = BaseHashTable::add((cref.clone(), 0), updatedComps.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), Some(updatedComps.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, _, _, _, _, _) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut is: FCore::Status = FCore::Status::CLS_FULL;
                    let mut cache = (*cache).clone();
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    (cache, _, _, _, is, _) = Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?;
                    let true = (FCore::isTyped(is.clone())) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), env.clone(), ih.clone(), inUpdatedComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, mods, _, _, _, _, _) => {
                    let mut n: ArcStr = arcstr::literal!("");
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut ct: SCode::ConnectorType = SCode::ConnectorType::FLOW;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut attr: SCode::Attributes;
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut prl1: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
                    let mut var1: SCode::Variability = SCode::Variability::CONST;
                    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut t: Arc<Absyn::Path>;
                    let mut m: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut cmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cl: Arc<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs_2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut cond: Option<Arc<Absyn::Exp>> = None;
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut pf: Arc<SCode::Prefixes>;
                    let mut visibility: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut idENV: FCore::Graph;
                    let mut updatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr));
                    let mut ci_state: ClassInf::State;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut mods = (*mods).clone();
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    let (__pa0, __pa1, __pa4, __pa2, __pa3, __pa10, __pa5, __pa6, __pa7, __pa8, __pa9, __pa11, __pa12, __pa13, __pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?) {
                        (__pa0, _, Deref @ SCode::Element::COMPONENT { name: __pa1, prefixes: __pa4 @ Deref @ SCode::Prefixes { visibility: __pa2, innerOuter: __pa3, .. }, attributes: __pa10 @ SCode::Attributes { arrayDims: __pa5, connectorType: __pa6, parallelism: __pa7, variability: __pa8, direction: __pa9, .. }, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa11, arrayDim: _ }, modifications: __pa12, comment: _, condition: __pa13, info: __pa14 }, __pa15, _, __pa16) => (__pa0.clone(), __pa1.clone(), __pa4.clone(), __pa2.clone(), __pa3.clone(), __pa10.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    n = __pa1.clone();
                    visibility = __pa2.clone();
                    io = __pa3.clone();
                    pf = __pa4.clone();
                    ad = __pa5.clone();
                    ct = __pa6.clone();
                    prl1 = __pa7.clone();
                    var1 = __pa8.clone();
                    dir = __pa9.clone();
                    attr = __pa10.clone();
                    t = __pa11.clone();
                    m = __pa12.clone();
                    cond = __pa13.clone();
                    info = __pa14.clone();
                    cmod = __pa15.clone();
                    idENV = __pa16.clone();
                    ci_state = InstUtil::updateClassInfState(cache.clone(), idENV.clone(), env.clone(), inCIState.clone())?;
                    (cache, cl, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), t.clone(), None)?;
                    updatedComps = getUpdatedCompsHashTable(inUpdatedComps.clone());
                    (mods, cmod, m) = InstUtil::noModForUpdatedComponents(var1.clone(), updatedComps.clone(), cref.clone(), mods.clone(), cmod.clone(), m.clone())?;
                    crefs = List::flatten(list![InstUtil::getCrefFromMod(m.clone())?, InstUtil::getCrefFromDim(ad.clone())?, InstUtil::getCrefFromCond(cond.clone())?, Mod::getUntypedCrefs(cmod.clone())?]);
                    crefs_2 = InstUtil::removeCrefFromCrefs(crefs.clone(), cref.clone())?;
                    crefs_2 = InstUtil::removeOptCrefFromCrefs(crefs_2.clone(), currentCref.clone())?;
                    updatedComps = BaseHashTable::add((cref.clone(), 0), updatedComps.clone())?;
                    let (__pa19, __pa20, __pa21, __pa22) = ::match_deref::match_deref! { match &(updateComponentsInEnv2(cache.clone(), env.clone(), ih.clone(), pre.clone(), mods.clone(), crefs_2.clone(), ci_state.clone(), r#impl.clone(), Some(updatedComps.clone()), Some(cref.clone()))?) {
                        (__pa19, __pa20, __pa21, Some(__pa22)) => (__pa19.clone(), __pa20.clone(), __pa21.clone(), __pa22.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa19.clone();
                    env2 = __pa20.clone();
                    ih = __pa21.clone();
                    updatedComps = __pa22.clone();
                    (cache, env_1, ih, updatedComps) = updateComponentInEnv2(cache.clone(), env2.clone(), cenv.clone(), ih.clone(), pre.clone(), t.clone(), (n.clone()).clone(), ad.clone(), cl.clone(), attr.clone(), pf.clone(), Arc::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: visibility.clone() }), info.clone(), m.clone(), cmod.clone(), mods.clone(), cref.clone(), ci_state.clone(), r#impl.clone(), updatedComps.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), Some(updatedComps.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, _, _, _, _, _) => {
                    Ok((cache.clone(), env.clone(), ih.clone(), inUpdatedComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.updateComponentInEnv failed, cref = ")); __mm_s.push_str(&*Dump::printComponentRefStr(cref.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" mods: ")); __mm_s.push_str(&*Mod::printModStr(r#mod.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" prefix: ")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(pre.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone(), inUpdatedComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outUpdatedComps))
}

fn updateComponentInEnv2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut cenv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut pre: DAE::Prefix, mut path: Arc<Absyn::Path>, mut name: ArcStr, mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut cl: Arc<SCode::Element>, mut attr: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut dattr: Arc<DAE::Attributes>, mut info: SourceInfo, mut m: Arc<SCode::Mod>, mut cmod: Arc<DAE::Mod>, mut r#mod: Arc<DAE::Mod>, mut cref: Arc<Absyn::ComponentRef>, mut ci_state: ClassInf::State, mut r#impl: bool, mut inUpdatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr)))> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outUpdatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr));
    match '__try0: {
        ErrorExt::setCheckpoint((literal!("Inst.updateComponentInEnv2")).clone());
        (outCache, outEnv, outIH, outUpdatedComps) = unwrap_break_err!(updateComponentInEnv2_dispatch(inCache.clone(), inEnv.clone(), cenv.clone(), inIH.clone(), pre.clone(), path.clone(), (name.clone()).clone(), ad.clone(), cl.clone(), attr.clone(), inPrefixes.clone(), dattr.clone(), info.clone(), m.clone(), cmod.clone(), r#mod.clone(), cref.clone(), ci_state.clone(), r#impl.clone(), inUpdatedComps.clone()), '__try0);
        ErrorExt::delCheckpoint((literal!("Inst.updateComponentInEnv2")).clone());
        Ok::<_, anyhow::Error>((outCache.clone(), outEnv.clone(), outIH.clone(), outUpdatedComps.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            outCache = __try0_o0;
            outEnv = __try0_o1;
            outIH = __try0_o2;
            outUpdatedComps = __try0_o3;
        }
        Err(_) => {
            ErrorExt::rollBack((literal!("Inst.updateComponentInEnv2")).clone());
            bail!("fail");
        }
    }
    Ok((outCache, outEnv, outIH, outUpdatedComps))
}

fn updateComponentInEnv2_dispatch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inClsEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inPath: Arc<Absyn::Path>, mut inName: ArcStr, mut inSubscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inClass: Arc<SCode::Element>, mut inAttr: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inDAttr: Arc<DAE::Attributes>, mut inInfo: SourceInfo, mut inSMod: Arc<SCode::Mod>, mut inClsMod: Arc<DAE::Mod>, mut inMod: Arc<DAE::Mod>, mut inCref: Arc<Absyn::ComponentRef>, mut inState: ClassInf::State, mut inImpl: bool, mut inUpdatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr)))> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outUpdatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr)) = inUpdatedComps.clone();
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut mod1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut class_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut comp_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut eq: Option<DAE::EqMod> = None;
    let mut own_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut cls_env: FCore::Graph;
    let mut comp_env: FCore::Graph;
    let mut cls: Arc<SCode::Element>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let mut var: Arc<DAE::Var>;
    if '__try0: {
        let 1 = (unwrap_break_err!(BaseHashTable::get(inCref.clone(), inUpdatedComps.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        smod = SCodeUtil::mergeModifiers(inSMod.clone(), SCodeUtil::getConstrainedByModifiers(inPrefixes.clone()))?;
        (outCache, mod1) = updateComponentInEnv3(outCache.clone(), outEnv.clone(), outIH.clone(), smod.clone(), inImpl.clone(), Mod::ModScope::COMPONENT { name: (inName.clone()).clone() }, inInfo.clone())?;
        class_mod = Mod::lookupModificationP(inMod.clone(), inPath.clone())?;
        mod2 = Mod::merge(class_mod.clone(), mod1.clone(), (inName.clone()).clone(), true)?;
        mod2 = Mod::merge(inClsMod.clone(), mod2.clone(), (inName.clone()).clone(), true)?;
        (outCache, mod2) = Mod::updateMod(outCache.clone(), outEnv.clone(), outIH.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, mod2.clone(), inImpl.clone(), inInfo.clone())?;
        r#mod = if (InstUtil::redeclareBasicType(inClsMod.clone())?) {mod1.clone()} else {mod2.clone()};
        eq = Mod::modEquation(r#mod.clone())?;
        own_cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (inName.clone()).clone(), subscripts: metamodelica::nil() });
        (outCache, dims) = InstUtil::elabArraydim(outCache.clone(), outEnv.clone(), own_cref.clone(), inPath.clone(), inSubscripts.clone(), eq.clone(), inImpl.clone(), true, false, inPrefix.clone(), inInfo.clone(), metamodelica::nil())?;
        (cls_env, cls, outIH) = FGraph::createVersionScope(outEnv.clone(), (inName.clone()).clone(), inPrefix.clone(), r#mod.clone(), inClsEnv.clone(), inClass.clone(), outIH.clone())?;
        (outCache, comp_env, outIH, _, _, _, ty, _) = InstVar::instVar(outCache.clone(), cls_env.clone(), outIH.clone(), UnitAbsyn::noStore().clone(), inState.clone(), r#mod.clone(), inPrefix.clone(), (inName.clone()).clone(), cls.clone(), inAttr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), metamodelica::nil(), inImpl.clone(), SCode::noComment.clone(), inInfo.clone(), ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), outEnv.clone())?;
        (outCache, binding) = InstBinding::makeBinding(outCache.clone(), outEnv.clone(), inAttr.clone(), r#mod.clone(), ty.clone(), inPrefix.clone(), (inName.clone()).clone(), inInfo.clone())?;
        var = Arc::new(DAE::Var { name: (inName.clone()).clone(), attributes: inDAttr.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None });
        outEnv = FGraph::updateComp(outEnv.clone(), var.clone(), crate::FCore::Status::VAR_TYPED, comp_env.clone())?;
        outUpdatedComps = BaseHashTable::add((inCref.clone(), 1), outUpdatedComps.clone())?;
    }
    Ok((outCache, outEnv, outIH, outUpdatedComps))
}

fn updateComponentInEnv3(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<SCode::Mod>, mut inImpl: bool, mut inModScope: Mod::ModScope, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Mod>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    (outCache, outMod) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inImpl.clone(), inModScope.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    ErrorExt::setCheckpoint((literal!("updateComponentInEnv3")).clone());
                    (cache, r#mod) = Mod::elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, inMod.clone(), inImpl.clone(), inModScope.clone(), inInfo.clone())?;
                    ErrorExt::rollBack((literal!("updateComponentInEnv3")).clone());
                    Ok((cache.clone(), r#mod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack((literal!("updateComponentInEnv3")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outMod))
}

pub fn makeEnvFromProgram(mut prog: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env_1: FCore::Graph;
    let mut env: FCore::Graph;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    (cache, env) = Builtin::initialGraph(FCore::emptyCache())?;
    env_1 = FGraphBuildEnv::mkProgramGraph(prog.clone(), crate::FCore::Kind::USERDEFINED, env.clone())?;
    outCache = cache.clone();
    Ok((outCache, env_1))
}

pub fn makeFullyQualified(mut cache: FCore::Cache, mut inEnv: FCore::Graph, mut path: Arc<Absyn::Path>) -> Result<(FCore::Cache, Arc<Absyn::Path>)> {
    let mut cache: FCore::Cache = cache;
    let mut path: Arc<Absyn::Path> = path;
    (cache, path) = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            (cache, path) = makeFullyQualifiedIdent(cache.clone(), inEnv.clone(), (var_field!((*path).name, Absyn::Path::IDENT).clone()).clone(), path.clone())?;
            (cache.clone(), path.clone())
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => (cache.clone(), path.clone()),
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            (cache, path) = makeFullyQualifiedFromQual(cache.clone(), inEnv.clone(), path.clone())?;
            (cache.clone(), path.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cache, path))
}

fn makeFullyQualifiedFromQual(mut cache: FCore::Cache, mut inEnv: FCore::Graph, mut path: Arc<Absyn::Path>) -> Result<(FCore::Cache, Arc<Absyn::Path>)> {
    let mut cache: FCore::Cache = cache;
    let mut path: Arc<Absyn::Path> = path;
    (cache, path) = 'mc: {
        let __mc_input = path.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env_1: FCore::Graph;
                    let mut path_2: Arc<Absyn::Path>;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut cache: FCore::Cache = cache.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), inEnv.clone(), path.clone(), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    name = __pa1.clone();
                    env_1 = __pa2.clone();
                    path_2 = makeFullyQualified2(env_1.clone(), (name.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    Ok((cache.clone(), AbsynUtil::makeFullyQualified(path_2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env: FCore::Graph;
                    let mut path3: Arc<Absyn::Path>;
                    let mut crPath: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut cache: FCore::Cache = cache.clone();
                    crPath = ComponentReference::pathToCref(path.clone())?;
                    (cache, _, _, _, _, _, env, _, name) = Lookup::lookupVarInternal(cache.clone(), inEnv.clone(), crPath.clone(), crate::InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN)?;
                    path3 = makeFullyQualified2(env.clone(), (name.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    Ok((cache.clone(), AbsynUtil::makeFullyQualified(path3.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env: FCore::Graph;
                    let mut path3: Arc<Absyn::Path>;
                    let mut crPath: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut cache: FCore::Cache = cache.clone();
                    crPath = ComponentReference::pathToCref(path.clone())?;
                    (cache, env, _, _, _, _, _, _, name) = Lookup::lookupVarInPackages(cache.clone(), inEnv.clone(), crPath.clone(), metamodelica::nil(), Mutable::create(false))?;
                    path3 = makeFullyQualified2(env.clone(), (name.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    Ok((cache.clone(), AbsynUtil::makeFullyQualified(path3.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((cache.clone(), path.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, path))
}

pub fn makeFullyQualifiedIdent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ident: ArcStr, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, Arc<Absyn::Path>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outPath: Arc<Absyn::Path>;
    let mut isKnownBuiltin: bool = false;
    (outPath, isKnownBuiltin) = makeFullyQualifiedIdentCheckBuiltin((ident.clone()).clone())?;
    if isKnownBuiltin.clone() {
        outCache = inCache.clone();
        return Ok((outCache, outPath));
    }
    (outCache, outPath) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), ident.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut env_1: FCore::Graph;
            let mut path_2: Arc<Absyn::Path>;
            let mut name: ArcStr = arcstr::literal!("");
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClassIdent(cache.clone(), env.clone(), (ident.clone()).clone(), None)?) {
                (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            name = __pa1.clone();
            env_1 = __pa2.clone();
            path_2 = makeFullyQualified2(env_1.clone(), (name.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path_2.clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, mut s) = __mc_input.clone() else { bail!("nomatch") };
            let mut path_2: Arc<Absyn::Path>;
            let mut name: ArcStr = arcstr::literal!("");
            let mut r: metamodelica::Array<FCore::Node>;
            r = FGraph::lastScopeRef(env.clone())?;
            let false = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
            name = (FNode::refName(r.clone())?).clone();
            let true = (name.clone() == s.clone()) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &(FGraph::getScopePath(env.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            path_2 = __pa0.clone();
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path_2.clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, mut s) = __mc_input.clone() else { bail!("nomatch") };
            let mut env_1: FCore::Graph;
            let mut path_2: Arc<Absyn::Path>;
            (cache, _, env_1) = Lookup::lookupTypeIdent(cache.clone(), env.clone(), (s.clone()).clone(), None)?;
            path_2 = makeFullyQualified2(env_1.clone(), (s.clone()).clone(), inPath.clone())?;
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path_2.clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut path3: Arc<Absyn::Path>;
            let mut name: ArcStr = arcstr::literal!("");
            (cache, _, _, _, _, _, env, _, name) = Lookup::lookupVarInternalIdent(cache.clone(), env.clone(), (ident.clone()).clone(), metamodelica::nil(), crate::InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN)?;
            path3 = makeFullyQualified2(env.clone(), (name.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path3.clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut path3: Arc<Absyn::Path>;
            let mut name: ArcStr = arcstr::literal!("");
            (cache, env, _, _, _, _, _, _, name) = Lookup::lookupVarInPackagesIdent(cache.clone(), env.clone(), (ident.clone()).clone(), metamodelica::nil(), metamodelica::nil(), Mutable::create(false))?;
            path3 = makeFullyQualified2(env.clone(), (name.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
            Ok((cache.clone(), AbsynUtil::makeFullyQualified(path3.clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inCache.clone(), (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "" } => Arc::new(Absyn::Path::IDENT { name: (ident.clone()).clone() }),
        _ => inPath.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outPath))
}

fn makeFullyQualifiedIdentCheckBuiltin(mut ident: ArcStr) -> Result<(Arc<Absyn::Path>, bool)> {
    let mut path: Arc<Absyn::Path>;
    let mut isKnownBuiltin: bool = true;
    path = (::match_deref::match_deref! { match &(ident.clone()) {
        Deref @ "Boolean" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Boolean")).clone() }) }),
        Deref @ "Integer" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Integer")).clone() }) }),
        Deref @ "Real" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() }) }),
        Deref @ "String" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("String")).clone() }) }),
        Deref @ "EnumType" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }) }),
        Deref @ "assert" => Arc::new(Absyn::Path::IDENT { name: (literal!("assert")).clone() }),
        Deref @ "reinit" => Arc::new(Absyn::Path::IDENT { name: (literal!("reinit")).clone() }),
        Deref @ "smooth" => Arc::new(Absyn::Path::IDENT { name: (literal!("smooth")).clone() }),
        Deref @ "list" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            Arc::new(Absyn::Path::IDENT { name: (literal!("list")).clone() })
        },
        Deref @ "Option" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            Arc::new(Absyn::Path::IDENT { name: (literal!("Option")).clone() })
        },
        Deref @ "tuple" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            Arc::new(Absyn::Path::IDENT { name: (literal!("tuple")).clone() })
        },
        Deref @ "polymorphic" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            Arc::new(Absyn::Path::IDENT { name: (literal!("polymorphic")).clone() })
        },
        Deref @ "array" => {
            isKnownBuiltin = Config::acceptMetaModelicaGrammar()?;
            Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() })
        },
        _ => {
            isKnownBuiltin = false;
            Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((path, isKnownBuiltin))
}

pub fn instList<Type_a: Clone + 'static>(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inState: ClassInf::State, mut instFunc: Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Type_a, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>, mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inImplicit: bool, mut unrollForLoops: bool, mut inGraph: ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> {
    pub type InstFunc<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Type_a, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>;

    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), instFunc.clone(), inTypeALst.clone(), inImplicit.clone(), unrollForLoops.clone(), inGraph.clone())) {
        (cache, env, ih, _, csets, ci_state, _, Deref @ metamodelica::List::Nil, _, _, graph) => {
            (cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state.clone(), graph.clone())
        },
        (cache, env, ih, pre, csets, ci_state, _, Deref @ metamodelica::List::Cons { head: e, tail: es }, r#impl, _, graph) => {
            let mut env_1: FCore::Graph;
            let mut env_2: FCore::Graph;
            let mut csets_1: DAE::Connect::Sets;
            let mut csets_2: DAE::Connect::Sets;
            let mut ci_state_1: ClassInf::State;
            let mut ci_state_2: ClassInf::State;
            let mut dae1: DAE::DAElist;
            let mut dae2: DAE::DAElist;
            let mut dae: DAE::DAElist;
            let mut cache = (*cache).clone();
            let mut ih = (*ih).clone();
            let mut graph = (*graph).clone();
            (cache, env_1, ih, dae1, csets_1, ci_state_1, graph) = instFunc(cache.clone(), env.clone(), ih.clone(), pre.clone(), csets.clone(), ci_state.clone(), e.clone(), r#impl.clone(), unrollForLoops.clone(), graph.clone())?;
            (cache, env_2, ih, dae2, csets_2, ci_state_2, graph) = instList(cache.clone(), env_1.clone(), ih.clone(), pre.clone(), csets_1.clone(), ci_state_1.clone(), instFunc.clone(), es.clone(), r#impl.clone(), unrollForLoops.clone(), graph.clone())?;
            dae = DAEUtil::joinDaes(dae1.clone(), dae2.clone())?;
            (cache.clone(), env_2.clone(), ih.clone(), dae.clone(), csets_2.clone(), ci_state_2.clone(), graph.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn instConstraints(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inConstraints: Arc<metamodelica::List<SCode::ConstraintSection>>, mut inImpl: bool) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist, ClassInf::State)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outDae: DAE::DAElist;
    let mut outState: ClassInf::State;
    (outCache, outEnv, outDae, outState) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inPrefix.clone(), inState.clone(), inConstraints.clone(), inImpl.clone())) {
        (_, _, _, _, Deref @ metamodelica::List::Nil, _) => {
            (inCache.clone(), inEnv.clone(), DAE::emptyDae().clone(), inState.clone())
        },
        (_, _, _, _, Deref @ metamodelica::List::Cons { head: constr, tail: rest }, _) => {
            let mut env1: FCore::Graph;
            let mut env2: FCore::Graph;
            let mut constraints_1: DAE::DAElist;
            let mut constraints_2: DAE::DAElist;
            let mut ci_state: ClassInf::State;
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut dae: DAE::DAElist;
            (cache, env1, constraints_1, ci_state) = InstSection::instConstraint(inCache.clone(), inEnv.clone(), inPrefix.clone(), inState.clone(), constr.clone(), inImpl.clone())?;
            (cache, env2, constraints_2, ci_state) = instConstraints(cache.clone(), env1.clone(), inPrefix.clone(), ci_state.clone(), rest.clone(), inImpl.clone())?;
            dae = DAEUtil::joinDaes(constraints_1.clone(), constraints_2.clone())?;
            (cache.clone(), env2.clone(), dae.clone(), ci_state.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- Inst.instConstraints failed\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outDae, outState))
}

fn instClassAttributes(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inInfo: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outDae: DAE::DAElist;
    (outCache, outEnv, outDae) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inPrefix.clone(), inAttrs.clone(), inImplicit.clone(), inInfo.clone())) {
        (cache, env, _, Deref @ metamodelica::List::Nil, _, _) => {
            (cache.clone(), env.clone(), DAE::emptyDae().clone())
        },
        (_, _, _, _, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut clsAttrs: DAE::DAElist;
            let mut dae: DAE::DAElist;
            clsAttrs = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: Arc::new(DAE::ClassAttributes { objetiveE: None, objectiveIntegrandE: None, startTimeE: None, finalTimeE: None }) })] };
            (cache, env, dae) = instClassAttributes2(inCache.clone(), inEnv.clone(), inPrefix.clone(), inAttrs.clone(), inImplicit.clone(), inInfo.clone(), clsAttrs.clone())?;
            (cache.clone(), env.clone(), dae.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- Inst.instClassAttributes failed\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outDae))
}

fn instClassAttributes2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplicit: bool, mut inInfo: SourceInfo, mut inClsAttrs: DAE::DAElist) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outDae: DAE::DAElist;
    (outCache, outEnv, outDae) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inPrefix.clone(), inAttrs.clone(), inImplicit.clone(), inInfo.clone(), inClsAttrs.clone())) {
        (cache, env, _, Deref @ metamodelica::List::Nil, _, _, clsAttrs) => {
            (cache.clone(), env.clone(), clsAttrs.clone())
        },
        (cache, env, pre, Deref @ metamodelica::List::Cons { head: na, tail: rest }, r#impl, _, clsAttrs) => {
            let mut env_2: FCore::Graph;
            let mut attrName: ArcStr = arcstr::literal!("");
            let mut attrExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut outExp: Arc<DAE::Exp>;
            let mut cache = (*cache).clone();
            let mut clsAttrs = (*clsAttrs).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(na.clone()) {
                Deref @ Absyn::NamedArg { argName: __pa0, argValue: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            attrName = __pa0.clone();
            attrExp = __pa1.clone();
            (cache, outExp, _) = Static::elabExp(cache.clone(), env.clone(), attrExp.clone(), r#impl.clone(), false, pre.clone(), inInfo.clone())?;
            clsAttrs = insertClassAttribute(clsAttrs.clone(), (attrName.clone()).clone(), outExp.clone())?;
            (cache, env_2, clsAttrs) = instClassAttributes2(cache.clone(), env.clone(), pre.clone(), rest.clone(), r#impl.clone(), inInfo.clone(), clsAttrs.clone())?;
            (cache.clone(), env_2.clone(), clsAttrs.clone())
        },
        _ => {
            Error::addMessage(Error::OPTIMICA_ERROR.clone(), list![(literal!("Class Attributes allowed only for Optimization classes.")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outDae))
}

fn insertClassAttribute(mut inAttrs: DAE::DAElist, mut attrName: ArcStr, mut inAttrExp: Arc<DAE::Exp>) -> Result<DAE::DAElist> {
    let mut outAttrs: DAE::DAElist;
    outAttrs = (::match_deref::match_deref! { match &((inAttrs.clone(), attrName.clone(), inAttrExp.clone())) {
        (attrs, Deref @ "objective", _) => {
            let mut startTimeE: Option<Arc<DAE::Exp>> = None;
            let mut finalTimeE: Option<Arc<DAE::Exp>> = None;
            let mut objectiveIntegrandE: Option<Arc<DAE::Exp>> = None;
            let mut attrs = (*attrs).clone();
            let DAE::DAE { elementLst: metamodelica::List::Cons { head: DAE::CLASS_ATTRIBUTES { classAttrs: __t3 }, tail: __t4 } } = (attrs.clone()) else { bail!("pattern mismatch") };
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*__t3).clone()) {
                Deref @ DAE::ClassAttributes { objetiveE: _, objectiveIntegrandE: __pa0, startTimeE: __pa1, finalTimeE: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ::match_deref::match_deref! { match &(__t4.clone()) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            objectiveIntegrandE = __pa0.clone();
            startTimeE = __pa1.clone();
            finalTimeE = __pa2.clone();
            attrs = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: Arc::new(DAE::ClassAttributes { objetiveE: Some(inAttrExp.clone()), objectiveIntegrandE: objectiveIntegrandE.clone(), startTimeE: startTimeE.clone(), finalTimeE: finalTimeE.clone() }) })] };
            attrs.clone()
        },
        (attrs, Deref @ "objectiveIntegrand", _) => {
            let mut objectiveE: Option<Arc<DAE::Exp>> = None;
            let mut startTimeE: Option<Arc<DAE::Exp>> = None;
            let mut finalTimeE: Option<Arc<DAE::Exp>> = None;
            let mut attrs = (*attrs).clone();
            let DAE::DAE { elementLst: metamodelica::List::Cons { head: DAE::CLASS_ATTRIBUTES { classAttrs: __t3 }, tail: __t4 } } = (attrs.clone()) else { bail!("pattern mismatch") };
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*__t3).clone()) {
                Deref @ DAE::ClassAttributes { objetiveE: __pa0, objectiveIntegrandE: _, startTimeE: __pa1, finalTimeE: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ::match_deref::match_deref! { match &(__t4.clone()) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            objectiveE = __pa0.clone();
            startTimeE = __pa1.clone();
            finalTimeE = __pa2.clone();
            attrs = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: Arc::new(DAE::ClassAttributes { objetiveE: objectiveE.clone(), objectiveIntegrandE: Some(inAttrExp.clone()), startTimeE: startTimeE.clone(), finalTimeE: finalTimeE.clone() }) })] };
            attrs.clone()
        },
        (attrs, Deref @ "startTime", _) => {
            let mut objectiveE: Option<Arc<DAE::Exp>> = None;
            let mut finalTimeE: Option<Arc<DAE::Exp>> = None;
            let mut objectiveIntegrandE: Option<Arc<DAE::Exp>> = None;
            let mut attrs = (*attrs).clone();
            let DAE::DAE { elementLst: metamodelica::List::Cons { head: DAE::CLASS_ATTRIBUTES { classAttrs: __t3 }, tail: __t4 } } = (attrs.clone()) else { bail!("pattern mismatch") };
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*__t3).clone()) {
                Deref @ DAE::ClassAttributes { objetiveE: __pa0, objectiveIntegrandE: __pa1, startTimeE: _, finalTimeE: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ::match_deref::match_deref! { match &(__t4.clone()) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            objectiveE = __pa0.clone();
            objectiveIntegrandE = __pa1.clone();
            finalTimeE = __pa2.clone();
            attrs = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: Arc::new(DAE::ClassAttributes { objetiveE: objectiveE.clone(), objectiveIntegrandE: objectiveIntegrandE.clone(), startTimeE: Some(inAttrExp.clone()), finalTimeE: finalTimeE.clone() }) })] };
            attrs.clone()
        },
        (attrs, Deref @ "finalTime", _) => {
            let mut objectiveE: Option<Arc<DAE::Exp>> = None;
            let mut startTimeE: Option<Arc<DAE::Exp>> = None;
            let mut objectiveIntegrandE: Option<Arc<DAE::Exp>> = None;
            let mut attrs = (*attrs).clone();
            let DAE::DAE { elementLst: metamodelica::List::Cons { head: DAE::CLASS_ATTRIBUTES { classAttrs: __t3 }, tail: __t4 } } = (attrs.clone()) else { bail!("pattern mismatch") };
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*__t3).clone()) {
                Deref @ DAE::ClassAttributes { objetiveE: __pa0, objectiveIntegrandE: __pa1, startTimeE: __pa2, finalTimeE: _ } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ::match_deref::match_deref! { match &(__t4.clone()) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            objectiveE = __pa0.clone();
            objectiveIntegrandE = __pa1.clone();
            startTimeE = __pa2.clone();
            attrs = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::CLASS_ATTRIBUTES { classAttrs: Arc::new(DAE::ClassAttributes { objetiveE: objectiveE.clone(), objectiveIntegrandE: objectiveIntegrandE.clone(), startTimeE: startTimeE.clone(), finalTimeE: Some(inAttrExp.clone()) }) })] };
            attrs.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- Inst.insertClassAttribute failed\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAttrs)
}

pub fn instantiateBoschClass(mut inCache: FCore::Cache, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDAElist: DAE::DAElist;
    (outCache, outEnv, outIH, outDAElist) = 'mc: {
        let __mc_input = (inCache.clone(), inIH.clone(), inProgram.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _) => {
                    Error::addMessage(Error::NO_CLASSES_LOADED.clone(), metamodelica::nil())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::IDENT { .. }) => {
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env) = Builtin::initialGraph(cache.clone())?;
                    env_1 = FGraphBuildEnv::mkProgramGraph(cdecls.clone(), crate::FCore::Kind::USERDEFINED, env.clone())?;
                    (cache, env_2, ih, dae) = instBoschClassInProgram(cache.clone(), env_1.clone(), ih.clone(), cdecls.clone(), path.clone())?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), dae.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih, cdecls @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, path @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut cdef: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env) = Builtin::initialGraph(cache.clone())?;
                    env_1 = FGraphBuildEnv::mkProgramGraph(cdecls.clone(), crate::FCore::Kind::USERDEFINED, env.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env_1.clone(), path.clone(), Some(Absyn::dummyInfo.clone()))?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    cdef = __pa1.clone();
                    env_2 = __pa2.clone();
                    (cache, env_2, ih, _, dae, _, _, _, _, _) = instClass(cache.clone(), env_2.clone(), ih.clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef.clone(), metamodelica::nil(), false, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone())?;
                    let _ = AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?;
                    Ok((cache.clone(), env_2.clone(), ih.clone(), dae.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, path) => {
                    let mut cname_str: ArcStr = arcstr::literal!("");
                    cname_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::ERROR_FLATTENING.clone(), list![(cname_str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDAElist))
}

fn instBoschClassInProgram(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outDae: DAE::DAElist;
    (outCache, outEnv, outIH, outDae) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inProgram.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::List::Cons { head: c @ Deref @ SCode::Element::CLASS { name: name1, .. }, tail: _ }, Deref @ Absyn::Path::IDENT { name: name2 }) => {
                    let mut dae: DAE::DAElist;
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let true = (stringEq((name1.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    (cache, env_1, ih, _, dae, _, _, _, _, _) = instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), false, crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), dae.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { name: name1, .. }, tail: cs }, path @ Deref @ Absyn::Path::IDENT { name: name2 }) => {
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let false = (stringEq((name1.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    (cache, env, ih, dae) = instBoschClassInProgram(cache.clone(), env.clone(), ih.clone(), cs.clone(), path.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), dae.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::List::Nil, _) => {
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outDae))
}

fn modifyInstantiateClass(mut inMod: Arc<DAE::Mod>, mut path: Arc<Absyn::Path>) -> Result<(Arc<DAE::Mod>, Arc<DAE::Mod>)> {
    let mut omod1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut omod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    (omod1, omod2) = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::CLASS { name: id, .. }, .. } => {
            if (id.clone() == AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?) {(inMod.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))} else {(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), inMod.clone())}
        },
        _ => {
            (Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), inMod.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((omod1, omod2))
}

fn removeSelfReferenceAndUpdate(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inRefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut inRef: Arc<Absyn::ComponentRef>, mut inPath: Arc<Absyn::Path>, mut inState: ClassInf::State, mut iattr: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut r#impl: bool, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut pre: DAE::Prefix, mut mods: Arc<DAE::Mod>, mut scodeMod: Arc<SCode::Mod>, mut info: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut o1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    (outCache, outEnv, outIH, outStore, o1) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inRefs.clone(), inRef.clone(), inPath.clone(), inState.clone(), iattr.clone(), inPrefixes.clone(), r#impl.clone(), inInstDims.clone(), pre.clone(), mods.clone(), scodeMod.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1, _, _, _, _, _, _, _, _, _, _) => {
                    let mut cl2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    i1 = (cl2.clone().len() as i32);
                    i2 = (cl1.clone().len() as i32);
                    let true = (i1.clone() == i2.clone()) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, sty, state, attr @ SCode::Attributes { direction: dir, variability: var1, parallelism: prl1, connectorType: ct, arrayDims: ad, .. }, _, _, inst_dims, _, _, _, _) => {
                    let mut cl2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut compenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut new_var: Arc<DAE::Var>;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    (cache, c, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), sty.clone(), Some(info.clone()))?;
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), cenv.clone(), c1.clone(), sty.clone(), ad.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    smod = SCodeInstUtil::removeSelfReferenceFromMod(scodeMod.clone(), c1.clone())?;
                    (cache, m) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), smod.clone(), r#impl.clone(), Mod::ModScope::COMPONENT { name: (n.clone()).clone() }, info.clone())?;
                    (cenv, c, ih) = FGraph::createVersionScope(env.clone(), (n.clone()).clone(), pre.clone(), m.clone(), cenv.clone(), c.clone(), ih.clone())?;
                    (cache, compenv, ih, store, _, _, ty, _) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), state.clone(), m.clone(), pre.clone(), (n.clone()).clone(), c.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), true, SCode::noComment.clone(), info.clone(), ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), env.clone())?;
                    io = SCodeUtil::prefixesInnerOuter(inPrefixes.clone())?;
                    vis = SCodeUtil::prefixesVisibility(inPrefixes.clone())?;
                    new_var = Arc::new(DAE::Var { name: (n.clone()).clone(), attributes: Arc::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: vis.clone() }), ty: ty.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), crate::FCore::Status::VAR_TYPED, compenv.clone())?;
                    ErrorExt::rollBack((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    ErrorExt::rollBack((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, sty, state, attr @ SCode::Attributes { direction: dir, variability: var1, parallelism: prl1, connectorType: ct, arrayDims: ad, .. }, _, _, inst_dims, _, _, _, _) => {
                    let mut cl2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut compenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut new_var: Arc<DAE::Var>;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    (cache, c, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), sty.clone(), Some(info.clone()))?;
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), cenv.clone(), c1.clone(), sty.clone(), ad.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    smod = SCodeInstUtil::removeNonConstantBindingsKeepRedeclares(scodeMod.clone(), false)?;
                    (cache, m) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), smod.clone(), r#impl.clone(), Mod::ModScope::COMPONENT { name: (n.clone()).clone() }, info.clone())?;
                    (cenv, c, ih) = FGraph::createVersionScope(env.clone(), (n.clone()).clone(), pre.clone(), m.clone(), cenv.clone(), c.clone(), ih.clone())?;
                    (cache, compenv, ih, store, _, _, ty, _) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), state.clone(), m.clone(), pre.clone(), (n.clone()).clone(), c.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), true, SCode::noComment.clone(), info.clone(), ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), env.clone())?;
                    io = SCodeUtil::prefixesInnerOuter(inPrefixes.clone())?;
                    vis = SCodeUtil::prefixesVisibility(inPrefixes.clone())?;
                    new_var = Arc::new(DAE::Var { name: (n.clone()).clone(), attributes: Arc::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: vis.clone() }), ty: ty.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), crate::FCore::Status::VAR_TYPED, compenv.clone())?;
                    ErrorExt::rollBack((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    ErrorExt::rollBack((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, sty, state, attr @ SCode::Attributes { direction: dir, variability: var1, parallelism: prl1, connectorType: ct, arrayDims: ad, .. }, _, _, inst_dims, _, _, _, _) => {
                    let mut cl2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut compenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut new_var: Arc<DAE::Var>;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    (cache, c, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), sty.clone(), Some(info.clone()))?;
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), cenv.clone(), c1.clone(), sty.clone(), ad.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    smod = SCodeInstUtil::removeNonConstantBindingsKeepRedeclares(scodeMod.clone(), true)?;
                    (cache, m) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), smod.clone(), r#impl.clone(), Mod::ModScope::COMPONENT { name: (n.clone()).clone() }, info.clone())?;
                    (cenv, c, ih) = FGraph::createVersionScope(env.clone(), (n.clone()).clone(), pre.clone(), m.clone(), cenv.clone(), c.clone(), ih.clone())?;
                    (cache, compenv, ih, store, _, _, ty, _) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), state.clone(), m.clone(), pre.clone(), (n.clone()).clone(), c.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), true, SCode::noComment.clone(), info.clone(), ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), env.clone())?;
                    io = SCodeUtil::prefixesInnerOuter(inPrefixes.clone())?;
                    vis = SCodeUtil::prefixesVisibility(inPrefixes.clone())?;
                    new_var = Arc::new(DAE::Var { name: (n.clone()).clone(), attributes: Arc::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: vis.clone() }), ty: ty.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), crate::FCore::Status::VAR_TYPED, compenv.clone())?;
                    ErrorExt::rollBack((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    ErrorExt::rollBack((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, sty, state, attr @ SCode::Attributes { direction: dir, variability: var1, parallelism: prl1, connectorType: ct, arrayDims: ad, .. }, _, _, inst_dims, _, _, _, _) => {
                    let mut cl2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut compenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut new_var: Arc<DAE::Var>;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut m: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    ErrorExt::setCheckpoint((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    (cache, c, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), sty.clone(), Some(info.clone()))?;
                    (cache, dims) = InstUtil::elabArraydim(cache.clone(), cenv.clone(), c1.clone(), sty.clone(), ad.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), inst_dims.clone())?;
                    m = Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD);
                    (cenv, c, ih) = FGraph::createVersionScope(env.clone(), (n.clone()).clone(), pre.clone(), m.clone(), cenv.clone(), c.clone(), ih.clone())?;
                    (cache, compenv, ih, store, _, _, ty, _) = InstVar::instVar(cache.clone(), cenv.clone(), ih.clone(), store.clone(), state.clone(), m.clone(), pre.clone(), (n.clone()).clone(), c.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), metamodelica::nil(), inst_dims.clone(), true, SCode::noComment.clone(), info.clone(), ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), env.clone())?;
                    io = SCodeUtil::prefixesInnerOuter(inPrefixes.clone())?;
                    vis = SCodeUtil::prefixesVisibility(inPrefixes.clone())?;
                    new_var = Arc::new(DAE::Var { name: (n.clone()).clone(), attributes: Arc::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl1.clone(), variability: var1.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: vis.clone() }), ty: ty.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None });
                    env = FGraph::updateComp(env.clone(), new_var.clone(), crate::FCore::Status::VAR_TYPED, compenv.clone())?;
                    ErrorExt::rollBack((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    ErrorExt::rollBack((literal!("Inst.removeSelfReferenceAndUpdate")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, cl1, c1, _, _, _, _, _, _, _, _, _, _) => {
                    let mut cl2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    cl2 = InstUtil::removeCrefFromCrefs(cl1.clone(), c1.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), cl2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, o1))
}

fn updateComponentsInEnv2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut pre: DAE::Prefix, mut r#mod: Arc<DAE::Mod>, mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut ci_state: ClassInf::State, mut r#impl: bool, mut inUpdatedComps: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))>, mut currentCref: Option<Arc<Absyn::ComponentRef>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outUpdatedComps: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr))> = inUpdatedComps.clone();
    let mut name: ArcStr = arcstr::literal!("");
    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    for mut cr in &*crefs.clone() {
        let mut cr = cr.clone();
        if '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(cr.clone()) {
                Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: __pa1 } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            name = __pa1.clone();
            let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(Lookup::lookupIdentLocal(outCache.clone(), outEnv.clone(), (name.clone()).clone()), '__try0)) {
                (_, Deref @ DAE::Var { binding: __pa2, .. }, _, _, _, _) => __pa2.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            binding = __pa2.clone();
            let true = (DAEUtil::isBound(binding.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            (outCache, outEnv, outIH, outUpdatedComps) = updateComponentInEnv(outCache.clone(), outEnv.clone(), outIH.clone(), pre.clone(), r#mod.clone(), cr.clone(), ci_state.clone(), r#impl.clone(), outUpdatedComps.clone(), currentCref.clone())?;
        }
    }
    Ok((outCache, outEnv, outIH, outUpdatedComps))
}

fn makeFullyQualified2(mut env: FCore::Graph, mut name: ArcStr, mut cachedPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    let mut scope: Arc<Absyn::Path>;
    let mut oscope: Option<Arc<Absyn::Path>> = None;
    oscope = FGraph::getScopePath(env.clone())?;
    if isNone(oscope.clone()) {
        path = makeFullyQualified2Builtin((name.clone()).clone(), cachedPath.clone());
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(oscope.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        scope = __pa0.clone();
        path = AbsynUtil::joinPaths(scope.clone(), (::match_deref::match_deref! { match &(cachedPath.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "" } => Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }),
        _ => cachedPath.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }))?;
    }
    Ok(path)
}

fn makeFullyQualified2Builtin(mut ident: ArcStr, mut cachedPath: Arc<Absyn::Path>) -> Arc<Absyn::Path> {
    let mut path: Arc<Absyn::Path>;
    path = (::match_deref::match_deref! { match &(ident.clone()) {
        Deref @ "abs" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("abs")).clone() }) }),
        Deref @ "acos" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("acos")).clone() }) }),
        Deref @ "activeState" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("activeState")).clone() }) }),
        Deref @ "actualStream" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("actualStream")).clone() }) }),
        Deref @ "asin" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("asin")).clone() }) }),
        Deref @ "atan" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("atan")).clone() }) }),
        Deref @ "atan2" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("atan2")).clone() }) }),
        Deref @ "backSample" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("backSample")).clone() }) }),
        Deref @ "cardinality" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("cardinality")).clone() }) }),
        Deref @ "cat" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("cat")).clone() }) }),
        Deref @ "ceil" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("ceil")).clone() }) }),
        Deref @ "change" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("change")).clone() }) }),
        Deref @ "classDirectory" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("classDirectory")).clone() }) }),
        Deref @ "cos" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("cos")).clone() }) }),
        Deref @ "cosh" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("cosh")).clone() }) }),
        Deref @ "cross" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("cross")).clone() }) }),
        Deref @ "delay" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("delay")).clone() }) }),
        Deref @ "der" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }) }),
        Deref @ "diagonal" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("diagonal")).clone() }) }),
        Deref @ "div" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("div")).clone() }) }),
        Deref @ "edge" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("edge")).clone() }) }),
        Deref @ "exp" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("exp")).clone() }) }),
        Deref @ "fill" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("fill")).clone() }) }),
        Deref @ "firstTick" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("firstTick")).clone() }) }),
        Deref @ "floor" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("floor")).clone() }) }),
        Deref @ "getInstanceName" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("getInstanceName")).clone() }) }),
        Deref @ "hold" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("hold")).clone() }) }),
        Deref @ "homotopy" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("homotopy")).clone() }) }),
        Deref @ "identity" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("identity")).clone() }) }),
        Deref @ "inStream" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("inStream")).clone() }) }),
        Deref @ "initial" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("initial")).clone() }) }),
        Deref @ "initialState" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("initialState")).clone() }) }),
        Deref @ "integer" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("integer")).clone() }) }),
        Deref @ "interval" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("interval")).clone() }) }),
        Deref @ "intAbs" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("intAbs")).clone() }) }),
        Deref @ "linspace" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("linspace")).clone() }) }),
        Deref @ "log" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("log")).clone() }) }),
        Deref @ "log10" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("log10")).clone() }) }),
        Deref @ "matrix" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("matrix")).clone() }) }),
        Deref @ "max" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }) }),
        Deref @ "min" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("min")).clone() }) }),
        Deref @ "mod" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("mod")).clone() }) }),
        Deref @ "ndims" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("ndims")).clone() }) }),
        Deref @ "noClock" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("noClock")).clone() }) }),
        Deref @ "noEvent" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("noEvent")).clone() }) }),
        Deref @ "ones" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("ones")).clone() }) }),
        Deref @ "outerProduct" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("outerProduct")).clone() }) }),
        Deref @ "pre" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("pre")).clone() }) }),
        Deref @ "previous" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }) }),
        Deref @ "print" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("print")).clone() }) }),
        Deref @ "product" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("product")).clone() }) }),
        Deref @ "realAbs" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("realAbs")).clone() }) }),
        Deref @ "rem" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("rem")).clone() }) }),
        Deref @ "rooted" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("rooted")).clone() }) }),
        Deref @ "sample" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }) }),
        Deref @ "scalar" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("scalar")).clone() }) }),
        Deref @ "semilinear" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("semilinear")).clone() }) }),
        Deref @ "shiftSample" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("shiftSample")).clone() }) }),
        Deref @ "sign" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sign")).clone() }) }),
        Deref @ "sin" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sin")).clone() }) }),
        Deref @ "sinh" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sinh")).clone() }) }),
        Deref @ "size" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("size")).clone() }) }),
        Deref @ "skew" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("skew")).clone() }) }),
        Deref @ "smooth" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("smooth")).clone() }) }),
        Deref @ "spatialDistribution" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("spatialDistribution")).clone() }) }),
        Deref @ "sqrt" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sqrt")).clone() }) }),
        Deref @ "subSample" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("subSample")).clone() }) }),
        Deref @ "symmetric" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("symmetric")).clone() }) }),
        Deref @ "tan" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("tan")).clone() }) }),
        Deref @ "tanh" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("tanh")).clone() }) }),
        Deref @ "terminal" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("terminal")).clone() }) }),
        Deref @ "ticksInState" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("ticksInState")).clone() }) }),
        Deref @ "timeInState" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("timeInState")).clone() }) }),
        Deref @ "transition" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("transition")).clone() }) }),
        Deref @ "transpose" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("transpose")).clone() }) }),
        Deref @ "vector" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("vector")).clone() }) }),
        Deref @ "zeros" => Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("zeros")).clone() }) }),
        _ => (::match_deref::match_deref! { match &(cachedPath.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "" } => Arc::new(Absyn::Path::IDENT { name: (ident.clone()).clone() }),
        _ => cachedPath.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    path
}

pub fn getCachedInstance(mut cache: FCore::Cache, mut env: FCore::Graph, mut name: ArcStr, mut r#ref: metamodelica::Array<FCore::Node>) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut cache_path: Arc<Absyn::Path>;
    let mut cls: Arc<SCode::Element>;
    let mut prefix: DAE::Prefix = DAE::Prefix::NOPRE;
    let mut prefix2: DAE::Prefix = DAE::Prefix::NOPRE;
    let mut env2: FCore::Graph;
    let mut enc: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
    let mut res: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let mut inputs: (Arc<DAE::Mod>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Arc<SCode::Element>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, bool, Option<Arc<DAE::ComponentRef>>, InstTypes::CallingScope);
    let true = (Flags::isSet(Flags::CACHE.clone())?) else { bail!("pattern mismatch") };
    let FCore::CL { e: __t4, pre: __pa3, .. } = (FNode::refData(r#ref.clone())?) else { bail!("pattern mismatch") };
    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &((*__t4).clone()) {
        __pa2 @ Deref @ SCode::Element::CLASS { restriction: __pa0, encapsulatedPrefix: __pa1, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    enc = __pa1.clone();
    cls = __pa2.clone();
    prefix = __pa3.clone();
    env2 = FGraph::openScope(env.clone(), enc.clone(), (name.clone()).clone(), FGraph::restrictionToScopeType(res.clone()))?;
    match '__try5: {
        cache_path = unwrap_break_err!(generateCachePath(env2.clone(), cls.clone(), prefix.clone(), crate::InstTypes::CallingScope::INNER_CALL), '__try5);
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(unwrap_break_err!(InstHashTable::get(cache_path.clone()), '__try5)) {
            Deref @ metamodelica::List::Cons { head: Some(InstHashTable::CachedInstItem::FUNC_instClassIn { inputs: __pa6, outputs: (__pa7, _, _, _, _, _, _, _, _) }), tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } => (__pa6.clone(), __pa7.clone()),
            _ => break '__try5 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        inputs = __pa6.clone();
        env = __pa7.clone();
        (_, prefix2, _, _, _, _, _, _, _) = inputs.clone();
        let true = (PrefixUtil::isPrefix(prefix.clone()) && PrefixUtil::isPrefix(prefix2.clone())) else { break '__try5 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        Ok::<_, anyhow::Error>((env.clone(),))
    } {
        Ok((__try5_o0,)) => {
            env = __try5_o0;
        }
        Err(_) => {
            env = FGraph::pushScopeRef(env.clone(), r#ref.clone())?;
        }
    }
    Ok((cache, env))
}

fn generateCachePath(mut env: FCore::Graph, mut cls: Arc<SCode::Element>, mut prefix: DAE::Prefix, mut callScope: InstTypes::CallingScope) -> Result<Arc<Absyn::Path>> {
    let mut cachePath: Arc<Absyn::Path>;
    let mut name: ArcStr = arcstr::literal!("");
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*InstTypes::callingScopeStr(callScope.clone())?); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*SCodeDump::restrString(SCodeUtil::getClassRestriction(cls.clone())?)?); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*generatePrefixStr(prefix.clone())); __mm_s.push_str(&*literal!("$")); ArcStr::from(__mm_s) }).clone();
    cachePath = AbsynUtil::joinPaths(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), FGraph::getGraphName(env.clone())?)?;
    Ok(cachePath)
}

pub fn generatePrefixStr(mut inPrefix: DAE::Prefix) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    match '__try0: {
        r#str = (unwrap_break_err!(AbsynUtil::pathString(PrefixUtil::prefixToPath(inPrefix.clone()).unwrap(), (literal!("$")).clone(), false, true), '__try0)).clone();
        Ok::<_, anyhow::Error>((r#str.clone(),))
    } {
        Ok((__try0_o0,)) => {
            r#str = __try0_o0;
        }
        Err(_) => {
            r#str = (literal!("")).clone();
        }
    }
    r#str
}

fn showCacheInfo(mut inMsg: ArcStr, mut inPath: Arc<Absyn::Path>) -> Result<()> {
    if Flags::isSet(Flags::SHOW_INST_CACHE_INFO.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inMsg.clone()); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn instFunctionAnnotations(mut comments: Arc<metamodelica::List<Arc<SCode::Comment>>>, mut state: ClassInf::State) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist = DAE::emptyDae().clone();
    let mut comment: Option<ArcStr> = None;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD);
    let mut mod2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    if !(ClassInfUtil::isFunction(state.clone())) {
        return Ok(dae);
    }
    for mut cmt in &*comments.clone() {
        let mut cmt = cmt.clone();
        if isNone(comment.clone()) {
            comment = cmt.comment.clone();
        }
        r#mod = (::match_deref::match_deref! { match &(cmt.clone()) {
        Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: mod2 }), .. } => SCodeUtil::mergeModifiers(mod2.clone(), r#mod.clone())?,
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    dae = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::NOMOD => if (isNone(comment.clone())) {dae.clone()} else {DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMMENT { cmt: Arc::new(SCode::Comment { annotation_: None, comment: comment.clone() }) })] }},
        _ => DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMMENT { cmt: Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: r#mod.clone() })), comment: comment.clone() }) })] },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dae)
}

pub fn instClassType(mut cache: FCore::Cache, mut env: FCore::Graph, mut classElem: Arc<SCode::Element>) -> Result<(FCore::Cache, FCore::Graph, Arc<DAE::Type>)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (cache, env, _, _, _, _, ty, _, _, _) = instClass(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, classElem.clone(), metamodelica::nil(), true, crate::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone())?;
    Ok((cache, env, ty))
}

fn checkInstanceRestriction(mut cdef: Arc<SCode::Element>, mut path: Arc<Absyn::Path>, mut relaxedFrontEnd: bool) -> Result<()> {
    if !(relaxedFrontEnd.clone()) && (SCodeUtil::isFunction(cdef.clone()) || SCodeUtil::isPackage(cdef.clone())) {
        Error::addSourceMessage(Error::INST_INVALID_RESTRICTION.clone(), list![(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), (SCodeDump::restrString(SCodeUtil::getClassRestriction(cdef.clone())?)?).clone()], SCodeUtil::elementInfo(cdef.clone()))?;
        bail!("fail");
    }
    Ok(())
}

