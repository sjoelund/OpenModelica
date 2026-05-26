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
use crate::ConnectUtil;
use crate::ConnectionGraph;
use crate::DAEUtil;
use crate::Expression;
use crate::FCore;
use crate::FGraph;
use crate::HashSet;
use crate::InnerOuter;
use crate::Inst;
use crate::InstBinding;
use crate::InstDAE;
use crate::InstFunction;
use crate::InstSection;
use crate::InstTypes;
use crate::InstUtil;
use crate::Lookup;
use crate::Mod;
use crate::PrefixUtil;
use crate::Types;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
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
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

pub type InstDims = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>;

pub fn instVar(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inState: ClassInf::State, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inIdent: ArcStr, mut inClass: Arc<SCode::Element>, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inIntegerLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImpl: bool, mut inComment: Arc<SCode::Comment>, mut info: SourceInfo, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets, mut componentDefinitionParentEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    if (::match_deref::match_deref! { match &(inIdent.clone()) {
        Deref @ "Integer" => true,
        Deref @ "Real" => true,
        Deref @ "Boolean" => true,
        Deref @ "String" => true,
        Deref @ "time" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
        Error::addSourceMessage(Error::RESERVED_IDENTIFIER.clone(), list![(inIdent.clone()).clone()], info.clone())?;
        bail!("fail");
    }
    io = SCodeUtil::prefixesInnerOuter(inPrefixes.clone())?;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inState.clone(), inMod.clone(), inPrefix.clone(), inIdent.clone(), inClass.clone(), inAttributes.clone(), inPrefixes.clone(), inDimensionLst.clone(), inIntegerLst.clone(), inInstDims.clone(), inImpl.clone(), inComment.clone(), info.clone(), inGraph.clone(), inSets.clone(), componentDefinitionParentEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl @ Deref @ SCode::Element::CLASS { name: typeName, .. }, attr, pf, dims, idxs, inst_dims, r#impl, comment, _, graph, csets, _) => {
                    let mut innerCompEnv: FCore::Graph;
                    let mut outerCompEnv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut outerDAE: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut fullName: ArcStr = arcstr::literal!("");
                    let mut typePath: Arc<Absyn::Path>;
                    let mut innerScope: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (AbsynUtil::isOnlyInner(io.clone())) else { bail!("pattern mismatch") };
                    (cache, innerCompEnv, ih, store, dae, csets, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    fullName = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), (typeName.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    outerCompEnv = InnerOuter::switchInnerToOuterInGraph(innerCompEnv.clone(), cref.clone())?;
                    outerDAE = DAE::emptyDae.clone();
                    innerScope = (FGraph::printGraphPathStr(componentDefinitionParentEnv.clone())?).clone();
                    ih = InnerOuter::updateInstHierarchy(ih.clone(), pre.clone(), io.clone(), InnerOuter::InstInner { innerPrefix: pre.clone(), name: (n.clone()).clone(), io: io.clone(), fullName: (fullName.clone()).clone(), typePath: typePath.clone(), scope: (innerScope.clone()).clone(), instResult: Some(InnerOuter::InstResult { outCache: cache.clone(), outEnv: outerCompEnv.clone(), outStore: store.clone(), outDae: outerDAE.clone(), outSets: csets.clone(), outType: ty.clone(), outGraph: graph.clone() }), outers: metamodelica::nil(), innerElement: None })?;
                    Ok((cache.clone(), innerCompEnv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, dims, idxs, inst_dims, r#impl, comment, _, graph, csets, _) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (AbsynUtil::isOnlyOuter(io.clone())) else { bail!("pattern mismatch") };
                    let false = (Mod::modEqual(r#mod.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))?) else { bail!("pattern mismatch") };
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    s1 = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
                    s2 = (Mod::prettyPrintMod(r#mod.clone(), 0)?).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::OUTER_MODIFICATION.clone(), list![(s.clone()).clone()], info.clone())?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone(), componentDefinitionParentEnv.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr @ SCode::Attributes { direction: Absyn::Direction::OUTPUT, .. }, pf, dims, idxs, inst_dims, r#impl, comment, _, graph, csets, _) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut topInstance: InnerOuter::TopInstance;
                    let mut sm: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (AbsynUtil::isOnlyOuter(io.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::modEqual(r#mod.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))?) else { bail!("pattern mismatch") };
                    let InnerOuter::INST_INNER { innerPrefix: _, name: _, io: _, fullName: _, typePath: _, scope: _, instResult: Some(InnerOuter::INST_RESULT { outCache: __pa0, outEnv: __pa1, outStore: __pa2, outDae: _, outSets: _, outType: __pa3, outGraph: __pa4 }), outers: _, innerElement: _ } = (InnerOuter::lookupInnerVar(cache.clone(), env.clone(), ih.clone(), pre.clone(), (n.clone()).clone(), io.clone())?) else { bail!("pattern mismatch") };
                    cache = __pa0.clone();
                    compenv = __pa1.clone();
                    store = __pa2.clone();
                    ty = __pa3.clone();
                    graph = __pa4.clone();
                    topInstance = listHead(ih.clone())?;
                    let InnerOuter::TOP_INSTANCE { sm: __pa5, .. } = (topInstance.clone()) else { bail!("pattern mismatch") };
                    sm = __pa5.clone();
                    let true = (BaseHashSet::currentSize(sm.clone()) > 0) else { bail!("pattern mismatch") };
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), sm.clone())?) else { bail!("pattern mismatch") };
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((inCache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, _, r#mod, pre, n, _, _, _, _, _, _, _, _, _, graph, csets, _) => {
                    let mut compenv: FCore::Graph;
                    let mut outerDAE: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut innerPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
                    let mut crefOuter: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefInner: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut outers: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut nInner: ArcStr = arcstr::literal!("");
                    let mut fullName: ArcStr = arcstr::literal!("");
                    let mut typePath: Arc<Absyn::Path>;
                    let mut innerScope: ArcStr = arcstr::literal!("");
                    let mut ioInner: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut instResult: Option<InnerOuter::InstResult> = None;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isOnlyOuter(io.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::modEqual(r#mod.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))?) else { bail!("pattern mismatch") };
                    let InnerOuter::INST_INNER { innerPrefix: __pa0, name: __pa1, io: __pa2, fullName: __pa3, typePath: __pa4, scope: __pa5, instResult: ref __pa12 @ Some(InnerOuter::INST_RESULT { outCache: ref __pa6, outEnv: ref __pa7, outStore: ref __pa8, outDae: ref __pa9, outSets: _, outType: ref __pa10, outGraph: ref __pa11 }), outers: __pa13, innerElement: _ } = (InnerOuter::lookupInnerVar(cache.clone(), env.clone(), ih.clone(), pre.clone(), (n.clone()).clone(), io.clone())?) else { bail!("pattern mismatch") };
                    innerPrefix = __pa0.clone();
                    nInner = __pa1.clone();
                    ioInner = __pa2.clone();
                    fullName = __pa3.clone();
                    typePath = __pa4.clone();
                    innerScope = __pa5.clone();
                    cache = __pa6.clone();
                    compenv = __pa7.clone();
                    store = __pa8.clone();
                    outerDAE = __pa9.clone();
                    ty = __pa10.clone();
                    graph = __pa11.clone();
                    instResult = __pa12.clone();
                    outers = __pa13.clone();
                    (cache, crefOuter) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    (cache, crefInner) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), innerPrefix.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    ih = InnerOuter::addOuterPrefixToIH(ih.clone(), crefOuter.clone(), crefInner.clone())?;
                    outers = List::unionElt(crefOuter.clone(), outers.clone());
                    ih = InnerOuter::updateInstHierarchy(ih.clone(), innerPrefix.clone(), ioInner.clone(), InnerOuter::InstInner { innerPrefix: innerPrefix.clone(), name: (nInner.clone()).clone(), io: ioInner.clone(), fullName: (fullName.clone()).clone(), typePath: typePath.clone(), scope: (innerScope.clone()).clone(), instResult: instResult.clone(), outers: outers.clone(), innerElement: None })?;
                    outerDAE = DAE::emptyDae.clone();
                    Ok((inCache.clone(), compenv.clone(), ih.clone(), store.clone(), outerDAE.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, dims, idxs, inst_dims, r#impl, comment, _, graph, csets, _) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut crefOuter: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut typeName: ArcStr = arcstr::literal!("");
                    let mut typePath: Arc<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isOnlyOuter(io.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::modEqual(r#mod.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))?) else { bail!("pattern mismatch") };
                    let InnerOuter::INST_INNER { innerPrefix: _, name: _, io: _, fullName: _, typePath: __pa0, scope: _, instResult: None, outers: _, innerElement: _ } = (InnerOuter::lookupInnerVar(cache.clone(), env.clone(), ih.clone(), pre.clone(), (n.clone()).clone(), io.clone())?) else { bail!("pattern mismatch") };
                    typePath = __pa0.clone();
                    (cache, crefOuter) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    typeName = (SCodeUtil::className(cl.clone())?).clone();
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), (typeName.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    if !(r#impl.clone() && listMember(pre.clone(), list![openmodelica_frontend_types::DAE::Prefix::NOPRE])) && !(Config::getGraphicsExpMode()?) {
                        s1 = (ComponentReferenceBasics::printComponentRefStr(crefOuter.clone())?).clone();
                        s2 = (AbsynUtil::innerOuterStr(io.clone())?).clone();
                        s3 = (InnerOuter::getExistingInnerDeclarations(ih.clone(), componentDefinitionParentEnv.clone())?).clone();
                        s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(typePath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s1.clone()); ArcStr::from(__mm_s) }).clone();
                        Error::addSourceMessage(Error::MISSING_INNER_PREFIX.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone()], info.clone())?;
                    }
                    (cache, compenv, ih, store, dae, _, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, dims, idxs, inst_dims, r#impl, comment, _, graph, csets, _) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut crefOuter: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut typeName: ArcStr = arcstr::literal!("");
                    let mut typePath: Arc<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isOnlyOuter(io.clone())) else { bail!("pattern mismatch") };
                    let true = (Mod::modEqual(r#mod.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))?) else { bail!("pattern mismatch") };
                    if '__try0: {
                        let _ = unwrap_break_err!(InnerOuter::lookupInnerVar(cache.clone(), env.clone(), ih.clone(), pre.clone(), (n.clone()).clone(), io.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (cache, crefOuter) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    typeName = (SCodeUtil::className(cl.clone())?).clone();
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), (typeName.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    if !(r#impl.clone() && listMember(pre.clone(), list![openmodelica_frontend_types::DAE::Prefix::NOPRE])) && !(Config::getGraphicsExpMode()?) {
                        s1 = (ComponentReferenceBasics::printComponentRefStr(crefOuter.clone())?).clone();
                        s2 = (AbsynUtil::innerOuterStr(io.clone())?).clone();
                        s3 = (InnerOuter::getExistingInnerDeclarations(ih.clone(), componentDefinitionParentEnv.clone())?).clone();
                        s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(typePath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s1.clone()); ArcStr::from(__mm_s) }).clone();
                        Error::addSourceMessage(Error::MISSING_INNER_PREFIX.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone()], info.clone())?;
                    }
                    (cache, compenv, ih, store, dae, _, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl @ Deref @ SCode::Element::CLASS { name: typeName, .. }, attr @ SCode::Attributes { direction: Absyn::Direction::OUTPUT, .. }, pf, dims, idxs, inst_dims, r#impl, comment, _, graph, csets, _) => {
                    let mut compenv: FCore::Graph;
                    let mut innerCompEnv: FCore::Graph;
                    let mut outerCompEnv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut innerDAE: DAE::DAElist;
                    let mut csetsInner: DAE::Connect::Sets;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut fullName: ArcStr = arcstr::literal!("");
                    let mut typePath: Arc<Absyn::Path>;
                    let mut innerScope: ArcStr = arcstr::literal!("");
                    let mut topInstance: InnerOuter::TopInstance;
                    let mut sm: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isInnerOuter(io.clone())) else { bail!("pattern mismatch") };
                    topInstance = listHead(ih.clone())?;
                    let InnerOuter::TOP_INSTANCE { sm: __pa0, .. } = (topInstance.clone()) else { bail!("pattern mismatch") };
                    sm = __pa0.clone();
                    let true = (BaseHashSet::currentSize(sm.clone()) > 0) else { bail!("pattern mismatch") };
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), sm.clone())?) else { bail!("pattern mismatch") };
                    (cache, innerCompEnv, ih, store, dae, csetsInner, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    fullName = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), (typeName.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    outerCompEnv = InnerOuter::switchInnerToOuterInGraph(innerCompEnv.clone(), cref.clone())?;
                    innerDAE = dae.clone();
                    innerScope = (FGraph::printGraphPathStr(componentDefinitionParentEnv.clone())?).clone();
                    ih = InnerOuter::updateInstHierarchy(ih.clone(), pre.clone(), io.clone(), InnerOuter::InstInner { innerPrefix: pre.clone(), name: (n.clone()).clone(), io: io.clone(), fullName: (fullName.clone()).clone(), typePath: typePath.clone(), scope: (innerScope.clone()).clone(), instResult: Some(InnerOuter::InstResult { outCache: cache.clone(), outEnv: outerCompEnv.clone(), outStore: store.clone(), outDae: innerDAE.clone(), outSets: csetsInner.clone(), outType: ty.clone(), outGraph: graph.clone() }), outers: metamodelica::nil(), innerElement: None })?;
                    (cache, compenv, ih, store, dae, _, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csetsInner.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl @ Deref @ SCode::Element::CLASS { name: typeName, .. }, attr, pf, dims, idxs, inst_dims, r#impl, comment, _, graph, csets, _) => {
                    let mut compenv: FCore::Graph;
                    let mut innerCompEnv: FCore::Graph;
                    let mut outerCompEnv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut outerDAE: DAE::DAElist;
                    let mut innerDAE: DAE::DAElist;
                    let mut csetsInner: DAE::Connect::Sets;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut fullName: ArcStr = arcstr::literal!("");
                    let mut typePath: Arc<Absyn::Path>;
                    let mut innerScope: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut pf = (*pf).clone();
                    let mut graph = (*graph).clone();
                    let true = (AbsynUtil::isInnerOuter(io.clone())) else { bail!("pattern mismatch") };
                    (cache, innerCompEnv, ih, store, dae, csetsInner, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    fullName = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
                    (cache, typePath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), (typeName.clone()).clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
                    outerCompEnv = InnerOuter::switchInnerToOuterInGraph(innerCompEnv.clone(), cref.clone())?;
                    innerDAE = dae.clone();
                    innerScope = (FGraph::printGraphPathStr(componentDefinitionParentEnv.clone())?).clone();
                    ih = InnerOuter::updateInstHierarchy(ih.clone(), pre.clone(), io.clone(), InnerOuter::InstInner { innerPrefix: pre.clone(), name: (n.clone()).clone(), io: io.clone(), fullName: (fullName.clone()).clone(), typePath: typePath.clone(), scope: (innerScope.clone()).clone(), instResult: Some(InnerOuter::InstResult { outCache: cache.clone(), outEnv: outerCompEnv.clone(), outStore: store.clone(), outDae: innerDAE.clone(), outSets: csetsInner.clone(), outType: ty.clone(), outGraph: graph.clone() }), outers: metamodelica::nil(), innerElement: None })?;
                    pf = SCodeUtil::prefixesSetInnerOuter(pf.clone(), openmodelica_ast::Absyn::InnerOuter::OUTER);
                    (cache, compenv, ih, store, dae, _, ty, graph) = instVar(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone(), componentDefinitionParentEnv.clone())?;
                    outerDAE = dae.clone();
                    dae = DAEUtil::joinDaes(outerDAE.clone(), innerDAE.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csetsInner.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, dims, idxs, inst_dims, r#impl, comment, _, graph, csets, _) => {
                    let mut compenv: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (AbsynUtil::isNotInnerOuter(io.clone())) else { bail!("pattern mismatch") };
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar_dispatch(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, r#mod, pre, n, cl, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache = (*cache).clone();
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    (cache, cref) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstVar.instVar failed while instatiating variable: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Mod::prettyPrintMod(r#mod.clone(), 0)?); __mm_s.push_str(&*literal!("\nin scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); __mm_s.push_str(&*literal!(" class:\n")); __mm_s.push_str(&*SCodeDump::unparseElementStr(cl.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn instVar_dispatch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inState: ClassInf::State, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inName: ArcStr, mut inClass: Arc<SCode::Element>, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inIndices: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImpl: bool, mut inComment: Arc<SCode::Comment>, mut inInfo: SourceInfo, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut comp_name: ArcStr = arcstr::literal!("");
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut cls: Arc<SCode::Element>;
    let mut type_mods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut attr: SCode::Attributes;
    let mut source: Arc<DAE::ElementSource>;
    match '__try0: {
        unwrap_break_err!(Error::updateCurrentComponent((inName.clone()).clone(), inInfo.clone(), Arc::new({ let __pe_b1 = inPrefix.clone(); move |__pe_a0| PrefixUtil::identAndPrefixToPath(__pe_a0, __pe_b1.clone()) })), '__try0);
        (outCache, dims, cls, type_mods) = unwrap_break_err!(InstUtil::getUsertypeDimensions(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone(), inImpl.clone()), '__try0);
        if dims.clone().is_empty() {
            dims = inDimensions.clone();
            cls = inClass.clone();
            r#mod = inMod.clone();
            attr = inAttributes.clone();
        } else {
            type_mods = unwrap_break_err!(liftUserTypeMod(type_mods.clone(), inDimensions.clone()), '__try0);
            dims = listAppend(inDimensions.clone(), dims.clone());
            r#mod = unwrap_break_err!(Mod::merge(inMod.clone(), type_mods.clone(), (literal!("")).clone(), true), '__try0);
            attr = InstUtil::propagateClassPrefix(inAttributes.clone(), inPrefix.clone());
        }
        (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = unwrap_break_err!(instVar2(outCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inState.clone(), r#mod.clone(), inPrefix.clone(), (inName.clone()).clone(), cls.clone(), attr.clone(), inPrefixes.clone(), dims.clone(), inIndices.clone(), inInstDims.clone(), inImpl.clone(), inComment.clone(), inInfo.clone(), inGraph.clone(), inSets.clone()), '__try0);
        source = unwrap_break_err!(ElementSource::createElementSource(inInfo.clone(), FGraph::getScopePath(inEnv.clone())?, inPrefix.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone())), '__try0);
        (outCache, outDae) = unwrap_break_err!(addArrayVarEquation(outCache.clone(), inEnv.clone(), outIH.clone(), inState.clone(), outDae.clone(), outType.clone(), r#mod.clone(), Types::variabilityToConst(SCodeUtil::attrVariability(attr.clone())?)?, inPrefix.clone(), (inName.clone()).clone(), source.clone()), '__try0);
        outCache = unwrap_break_err!(InstFunction::addRecordConstructorFunction(outCache.clone(), inEnv.clone(), Types::arrayElementType(outType.clone()), SCodeUtil::elementInfo(inClass.clone())), '__try0);
        unwrap_break_err!(Error::clearCurrentComponent(), '__try0);
        Ok::<_, anyhow::Error>((attr.clone(), cls.clone(), dims.clone(), r#mod.clone(), outCache.clone(), outDae.clone(), outEnv.clone(), outGraph.clone(), outIH.clone(), outSets.clone(), outStore.clone(), outType.clone(), source.clone(), type_mods.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10, __try0_o11, __try0_o12, __try0_o13)) => {
            attr = __try0_o0;
            cls = __try0_o1;
            dims = __try0_o2;
            r#mod = __try0_o3;
            outCache = __try0_o4;
            outDae = __try0_o5;
            outEnv = __try0_o6;
            outGraph = __try0_o7;
            outIH = __try0_o8;
            outSets = __try0_o9;
            outStore = __try0_o10;
            outType = __try0_o11;
            source = __try0_o12;
            type_mods = __try0_o13;
        }
        Err(_) => {
            Error::clearCurrentComponent()?;
            bail!("fail");
        }
    }
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn liftUserTypeMod(mut inMod: Arc<DAE::Mod>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<DAE::Mod>> {
    let mut outMod: Arc<DAE::Mod> = inMod.clone();
    if inDims.clone().is_empty() {
        return Ok(outMod);
    }
    outMod = 'mc: {
        let __mc_input = outMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { .. } => {
                    let mut outMod: Arc<DAE::Mod> = outMod.clone();
                    if !(SCodeUtil::eachBool(var_field!((*outMod).eachPrefix, DAE::Mod::MOD).clone())?) {
                        assign_variant_field!(outMod => DAE::Mod::MOD;
                            binding = liftUserTypeEqMod(var_field!((*outMod).binding, DAE::Mod::MOD).clone(), inDims.clone())?,
                            subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
        for mut s in (var_field!((*outMod).subModLst, DAE::Mod::MOD).clone()).into_iter().cloned() {
                    let __x = liftUserTypeSubMod(s.clone(), inDims.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
                        );
                    }
                    Ok(outMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(outMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn liftUserTypeSubMod(mut inSubMod: Arc<DAE::SubMod>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<DAE::SubMod>> {
    let mut outSubMod: Arc<DAE::SubMod> = inSubMod.clone();
    outSubMod = (::match_deref::match_deref! { match &(outSubMod.clone()) {
        Deref @ DAE::SubMod { .. } => {
            assign_field!(outSubMod.r#mod = liftUserTypeMod(outSubMod.r#mod.clone(), inDims.clone())?);
            outSubMod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubMod)
}

fn liftUserTypeEqMod(mut inEqMod: Option<DAE::EqMod>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Option<DAE::EqMod>> {
    let mut outEqMod: Option<DAE::EqMod> = None;
    let mut eq: DAE::EqMod;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if isNone(inEqMod.clone()) {
        outEqMod = inEqMod.clone();
        return Ok(outEqMod);
    }
    let Some(__pa0) = (inEqMod.clone()) else { bail!("pattern mismatch") };
    eq = __pa0.clone();
    eq = (match eq.clone() {
        DAE::EqMod::TYPED { .. } => {
            let __owned_variant_modifierAsExp_0 = Expression::liftExpList(var_field!(eq.modifierAsExp, DAE::EqMod::TYPED).clone(), inDims.clone())?;
            let __owned_variant_modifierAsValue_1 = Util::applyOption1(var_field!(eq.modifierAsValue, DAE::EqMod::TYPED).clone(), Arc::new(ValuesUtil::liftValueList), inDims.clone());
            if let DAE::EqMod::TYPED { modifierAsExp, modifierAsValue, .. } = &mut eq {
                *modifierAsExp = __owned_variant_modifierAsExp_0;
                *modifierAsValue = __owned_variant_modifierAsValue_1;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::EqMod::TYPED"); }
            ty = Types::getPropType(var_field!(eq.properties, DAE::EqMod::TYPED).clone())?;
            let __owned_variant_properties_0 = Types::setPropType(var_field!(eq.properties, DAE::EqMod::TYPED).clone(), Types::liftArrayListDims(ty.clone(), inDims.clone()))?;
            if let DAE::EqMod::TYPED { properties, .. } = &mut eq {
                *properties = __owned_variant_properties_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::EqMod::TYPED"); }
            eq.clone()
        },
        _ => eq.clone(),
    });
    outEqMod = Some(eq.clone());
    Ok(outEqMod)
}

fn addArrayVarEquation(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inState: ClassInf::State, mut inDae: DAE::DAElist, mut inType: Arc<DAE::Type>, mut r#mod: Arc<DAE::Mod>, mut r#const: DAE::Const, mut pre: DAE::Prefix, mut n: ArcStr, mut source: Arc<DAE::ElementSource>) -> Result<(FCore::Cache, DAE::DAElist)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDae: DAE::DAElist;
    (outCache, outDae) = 'mc: {
        let __mc_input = (inDae.clone(), r#const.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (Config::scalarizeBindings()?) else { bail!("pattern mismatch") };
            Ok((inCache.clone(), inDae.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::DAElist { elementLst: ref dae }, DAE::Const::C_VAR) = __mc_input.clone() else { bail!("nomatch") };
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut exp: Arc<DAE::Exp>;
            let mut eq: Arc<DAE::Element>;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let false = (ClassInfUtil::isFunctionOrRecord(inState.clone())) else { bail!("pattern mismatch") };
            ty = Types::simplifyType(inType.clone())?;
            let false = (Types::isExternalObject(Types::arrayElementType(ty.clone()))) else { bail!("pattern mismatch") };
            let false = (Types::isComplexType(Types::arrayElementType(ty.clone()))) else { bail!("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &(TypesDump::getDimensions(ty.clone())) {
                __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            dims = __pa0.clone();
            let __pa1 = ::match_deref::match_deref! { match &(InstBinding::makeVariableBinding(ty.clone(), r#mod.clone(), r#const.clone(), pre.clone(), (n.clone()).clone())?) {
                Some(__pa1) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            exp = __pa1.clone();
            cr = ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), ty.clone(), metamodelica::nil());
            (cache, cr) = PrefixUtil::prefixCref(inCache.clone(), inEnv.clone(), inIH.clone(), pre.clone(), cr.clone())?;
            eq = Arc::new(DAE::Element::ARRAY_EQUATION { dimension: dims.clone(), exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), array: exp.clone(), source: source.clone() });
            Ok((cache.clone(), DAE::DAElist { elementLst: cons(eq.clone(), dae.clone()) }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inCache.clone(), inDae.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDae))
}

fn instVar2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inState: ClassInf::State, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inName: ArcStr, mut inClass: Arc<SCode::Element>, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImpl: bool, mut inComment: Arc<SCode::Comment>, mut inInfo: SourceInfo, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inState.clone(), inMod.clone(), inPrefix.clone(), inName.clone(), inClass.clone(), inAttributes.clone(), inPrefixes.clone(), inDimensions.clone(), inSubscripts.clone(), inInstDims.clone(), inImpl.clone(), inComment.clone(), inInfo.clone(), inGraph.clone(), inSets.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod @ Deref @ DAE::Mod::MOD { binding: None, .. }, pre, n, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, attr, pf, dims, _, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut env_1: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ty_2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut dae_var_attr: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut fin: SCode::Final = SCode::Final::FINAL;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (ClassInfUtil::isFunction(ci_state.clone())) else { bail!("pattern mismatch") };
                    InstUtil::checkFunctionVar((n.clone()).clone(), attr.clone(), pf.clone(), info.clone())?;
                    (cache, env_1, ih, store, _, csets, ty, _, _, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), cl.clone(), inst_dims.clone(), r#impl.clone(), crate::InstTypes::CallingScope::INNER_CALL, graph.clone(), csets.clone())?;
                    ty_1 = InstUtil::makeArrayType(dims.clone(), ty.clone())?;
                    InstUtil::checkFunctionVarType(ty_1.clone(), ci_state.clone(), (n.clone()).clone(), info.clone())?;
                    (cache, dae_var_attr) = InstBinding::instDaeVariableAttributes(cache.clone(), env.clone(), r#mod.clone(), ty.clone(), metamodelica::nil())?;
                    ty_2 = Types::simplifyType(ty_1.clone())?;
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), ty_2.clone(), metamodelica::nil()))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(InstBinding::makeBinding(cache.clone(), env.clone(), attr.clone(), r#mod.clone(), ty_2.clone(), pre.clone(), (n.clone()).clone(), info.clone())?) {
                        (__pa0, Deref @ DAE::Binding::EQBOUND { exp: __pa1, evaluatedExp: _, constant_: _, source: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    e = __pa1.clone();
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(pf.clone()) {
                        Deref @ SCode::Prefixes { innerOuter: __pa2, finalPrefix: __pa3, visibility: __pa4, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    io = __pa2.clone();
                    fin = __pa3.clone();
                    vis = __pa4.clone();
                    dae = InstDAE::daeDeclare(cache.clone(), env.clone(), env_1.clone(), cr.clone(), ci_state.clone(), ty.clone(), attr.clone(), vis.clone(), Some(e.clone()), list![dims.clone()], None, dae_var_attr.clone(), Some(comment.clone()), io.clone(), fin.clone(), source.clone(), true)?;
                    store = UnitAbsynBuilder::instAddStore(store.clone(), ty.clone(), cr.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod @ Deref @ DAE::Mod::MOD { binding: Some(_), .. }, pre, n, cl, attr, pf, dims, _, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut e_1: Arc<DAE::Exp>;
                    let mut p: DAE::Properties;
                    let mut env_1: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ty_2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut dae_var_attr: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut fin: SCode::Final = SCode::Final::FINAL;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (ClassInfUtil::isFunction(ci_state.clone())) else { bail!("pattern mismatch") };
                    InstUtil::checkFunctionVar((n.clone()).clone(), attr.clone(), pf.clone(), info.clone())?;
                    let Some(DAE::TYPED { modifierAsExp: __pa0, modifierAsValue: _, properties: __pa1, modifierAsAbsynExp: _, .. }) = (Mod::modEquation(r#mod.clone())?) else { bail!("pattern mismatch") };
                    e = __pa0.clone();
                    p = __pa1.clone();
                    (cache, env_1, ih, store, _, csets, ty, _, _, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), cl.clone(), inst_dims.clone(), r#impl.clone(), crate::InstTypes::CallingScope::INNER_CALL, graph.clone(), csets.clone())?;
                    ty_1 = InstUtil::makeArrayType(dims.clone(), ty.clone())?;
                    InstUtil::checkFunctionVarType(ty_1.clone(), ci_state.clone(), (n.clone()).clone(), info.clone())?;
                    (cache, dae_var_attr) = InstBinding::instDaeVariableAttributes(cache.clone(), env.clone(), r#mod.clone(), ty.clone(), metamodelica::nil())?;
                    (e_1, _) = Types::matchProp(e.clone(), p.clone(), DAE::Properties::PROP { type_: ty_1.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, true)?;
                    ty_2 = Types::simplifyType(ty_1.clone())?;
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), ty_2.clone(), metamodelica::nil()))?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(pf.clone()) {
                        Deref @ SCode::Prefixes { innerOuter: __pa2, finalPrefix: __pa3, visibility: __pa4, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    io = __pa2.clone();
                    fin = __pa3.clone();
                    vis = __pa4.clone();
                    dae = InstDAE::daeDeclare(cache.clone(), env.clone(), env_1.clone(), cr.clone(), ci_state.clone(), ty.clone(), attr.clone(), vis.clone(), Some(e_1.clone()), list![dims.clone()], None, dae_var_attr.clone(), Some(comment.clone()), io.clone(), fin.clone(), source.clone(), true)?;
                    store = UnitAbsynBuilder::instAddStore(store.clone(), ty.clone(), cr.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl @ Deref @ SCode::Element::CLASS { .. }, attr, pf, dims, _, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut env_1: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut arrty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut dae: DAE::DAElist;
                    let mut dae_var_attr: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut fin: SCode::Final = SCode::Final::FINAL;
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut csets = (*csets).clone();
                    let true = (ClassInfUtil::isFunction(ci_state.clone())) else { bail!("pattern mismatch") };
                    InstUtil::checkFunctionVar((n.clone()).clone(), attr.clone(), pf.clone(), info.clone())?;
                    (cache, env_1, ih, store, _, csets, ty, _, _, _) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), r#mod.clone(), pre.clone(), cl.clone(), inst_dims.clone(), r#impl.clone(), crate::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY.clone(), csets.clone())?;
                    arrty = InstUtil::makeArrayType(dims.clone(), ty.clone())?;
                    InstUtil::checkFunctionVarType(arrty.clone(), ci_state.clone(), (n.clone()).clone(), info.clone())?;
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), arrty.clone(), metamodelica::nil()))?;
                    (cache, dae_var_attr) = InstBinding::instDaeVariableAttributes(cache.clone(), env.clone(), r#mod.clone(), ty.clone(), metamodelica::nil())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(pf.clone()) {
                        Deref @ SCode::Prefixes { innerOuter: __pa0, finalPrefix: __pa1, visibility: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    io = __pa0.clone();
                    fin = __pa1.clone();
                    vis = __pa2.clone();
                    dae = InstDAE::daeDeclare(cache.clone(), env.clone(), env_1.clone(), cr.clone(), ci_state.clone(), ty.clone(), attr.clone(), vis.clone(), None, list![dims.clone()], None, dae_var_attr.clone(), Some(comment.clone()), io.clone(), fin.clone(), source.clone(), true)?;
                    store = UnitAbsynBuilder::instAddStore(store.clone(), ty.clone(), cr.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), arrty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    let mut env: FCore::Graph;
                    let mut csets: DAE::Connect::Sets;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut ih: InstanceHierarchy = metamodelica::nil();
                    let mut store: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
                    let false = (ClassInfUtil::isFunction(inState.clone())) else { bail!("pattern mismatch") };
                    (cache, env, ih, store, dae, csets, ty, graph) = instScalar(inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inState.clone(), inMod.clone(), inPrefix.clone(), (inName.clone()).clone(), inClass.clone(), inAttributes.clone(), inPrefixes.clone(), inSubscripts.clone(), inInstDims.clone(), inImpl.clone(), Some(inComment.clone()), inInfo.clone(), inGraph.clone(), inSets.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod @ Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }, pre, n, cl, attr, pf, Deref @ metamodelica::List::Cons { head: dim @ Deref @ DAE::Dimension::DIM_UNKNOWN, tail: dims }, idxs, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut inst_dims_1: InstDims = metamodelica::nil();
                    let mut compenv: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let false = (ClassInfUtil::isFunction(ci_state.clone())) else { bail!("pattern mismatch") };
                    dim2 = InstUtil::instWholeDimFromMod(dim.clone(), r#mod.clone(), (n.clone()).clone(), info.clone())?;
                    inst_dims_1 = List::appendLastList(inst_dims.clone(), list![dim2.clone()])?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instArray(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), (cl.clone(), attr.clone()), pf.clone(), 1, dim2.clone(), dims.clone(), idxs.clone(), inst_dims_1.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    ty_1 = InstUtil::liftNonBasicTypes(ty.clone(), dim2.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod @ Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }, pre, n, cl, attr, pf, Deref @ metamodelica::List::Cons { head: dim @ Deref @ DAE::Dimension::DIM_UNKNOWN, tail: dims }, idxs, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut inst_dims_1: InstDims = metamodelica::nil();
                    let mut compenv: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut dime2: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let false = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let false = (ClassInfUtil::isFunction(ci_state.clone())) else { bail!("pattern mismatch") };
                    dim2 = InstUtil::instWholeDimFromMod(dim.clone(), r#mod.clone(), (n.clone()).clone(), info.clone())?;
                    inst_dims_1 = List::appendLastList(inst_dims.clone(), list![dim2.clone()])?;
                    dime2 = Expression::dimensionSubscript(dim2.clone())?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), cons(dime2.clone(), idxs.clone()), inst_dims_1.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    ty_1 = InstUtil::liftNonBasicTypes(ty.clone(), dim2.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, Deref @ metamodelica::List::Cons { head: dim, tail: dims }, idxs, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut inst_dims_1: InstDims = metamodelica::nil();
                    let mut compenv: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let false = (ClassInfUtil::isFunction(ci_state.clone())) else { bail!("pattern mismatch") };
                    inst_dims_1 = List::appendLastList(inst_dims.clone(), list![dim.clone()])?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instArray(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), (cl.clone(), attr.clone()), pf.clone(), 1, dim.clone(), dims.clone(), idxs.clone(), inst_dims_1.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    ty_1 = InstUtil::liftNonBasicTypes(ty.clone(), dim.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty_1.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, cl, attr, pf, Deref @ metamodelica::List::Cons { head: dim, tail: dims }, idxs, inst_dims, r#impl, comment, info, graph, csets) => {
                    let mut inst_dims_1: InstDims = metamodelica::nil();
                    let mut compenv: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut dime: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let false = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    let false = (ClassInfUtil::isFunction(ci_state.clone())) else { bail!("pattern mismatch") };
                    inst_dims_1 = List::appendLastList(inst_dims.clone(), list![dim.clone()])?;
                    dime = Expression::dimensionSubscript(dim.clone())?;
                    (cache, compenv, ih, store, dae, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), cons(dime.clone(), idxs.clone()), inst_dims_1.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, Deref @ DAE::Mod::NOMOD, _, n, _, _, _, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: _ }, _, _, _, _, info, _, _) => {
                    Error::addSourceMessage(Error::FAILURE_TO_DEDUCE_DIMS_NO_MOD.clone(), list![ArcStr::from(::std::format!("{}", (inSubscripts.clone().len() as i32) + 1)), (n.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, r#mod, pre, n, _, _, _, _, _, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstVar.instVar2 failed: ")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(pre.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*Mod::prettyPrintMod(r#mod.clone(), 0)?); __mm_s.push_str(&*literal!(")\n  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

pub fn instScalar(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inState: ClassInf::State, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inName: ArcStr, mut inClass: Arc<SCode::Element>, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImpl: bool, mut inComment: Option<Arc<SCode::Comment>>, mut inInfo: SourceInfo, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inState.clone(), inMod.clone(), inPrefix.clone(), inName.clone(), inClass.clone(), inAttributes.clone(), inPrefixes.clone(), inSubscripts.clone(), inInstDims.clone(), inImpl.clone(), inComment.clone(), inInfo.clone(), inGraph.clone(), inSets.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, _, r#mod, _, _, Deref @ SCode::Element::CLASS { restriction: res, name: cls_name, .. }, SCode::Attributes { variability: vt, .. }, Deref @ SCode::Prefixes { innerOuter: io, finalPrefix: fin, visibility: vis, .. }, idxs, _, _, _, _, _, _) => {
                    let mut implicitInstantiation: bool = false;
                    let mut inStateAndClassNameIsEqual: bool = false;
                    let mut ci_state: ClassInf::State;
                    let mut csets: DAE::Connect::Sets;
                    let mut graph: ConnectionGraph::ConnectionGraph;
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut dae: DAE::DAElist;
                    let mut dae1: DAE::DAElist;
                    let mut dae2: DAE::DAElist;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut pre: DAE::Prefix = DAE::Prefix::NOPRE;
                    let mut start: Option<Arc<DAE::Exp>> = None;
                    let mut ident_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut env_1: FCore::Graph;
                    let mut opt_binding: Option<Arc<DAE::Exp>> = None;
                    let mut dae_var_attr: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut opt_attr: Option<SCode::Attributes> = None;
                    let mut attr: SCode::Attributes;
                    let mut classWithElementsRemoved: Arc<SCode::Element>;
                    let mut stateName: ArcStr = arcstr::literal!("");
                    let mut predims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut idxs = (*idxs).clone();
                    idxs = idxs.clone().reverse();
                    ci_state = ClassInfUtil::start(res.clone(), Arc::new(Absyn::Path::IDENT { name: (cls_name.clone()).clone() }))?;
                    predims = List::lastListOrEmpty(inInstDims.clone());
                    pre = PrefixUtil::prefixAdd((inName.clone()).clone(), predims.clone(), idxs.clone(), inPrefix.clone(), vt.clone(), ci_state.clone(), inInfo.clone())?;
                    if Config::acceptMetaModelicaGrammar()? {
                        stateName = (AbsynUtil::pathString(ClassInfUtil::getStateName(inState.clone()), (literal!("")).clone(), true, false)?).clone();
                        inStateAndClassNameIsEqual = stringEqual((stateName.clone()).clone(), (cls_name.clone()).clone());
                        implicitInstantiation = SCodeUtil::isUniontype(inClass.clone()) && SCodeUtil::isConstant(inAttributes.variability.clone()) && inStateAndClassNameIsEqual.clone();
                        if implicitInstantiation.clone() {
                            classWithElementsRemoved = SCodeUtil::setClassDef(Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), inClass.clone())?;
                            (_, env_1, ih, store, dae1, csets, ty, _, opt_attr, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), inMod.clone(), pre.clone(), classWithElementsRemoved.clone(), inInstDims.clone(), inImpl.clone(), crate::InstTypes::CallingScope::INNER_CALL, inGraph.clone(), inSets.clone())?;
                        } else {
                            (cache, env_1, ih, store, dae1, csets, ty, _, opt_attr, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), inMod.clone(), pre.clone(), inClass.clone(), inInstDims.clone(), inImpl.clone(), crate::InstTypes::CallingScope::INNER_CALL, inGraph.clone(), inSets.clone())?;
                        }
                    } else {
                        (cache, env_1, ih, store, dae1, csets, ty, _, opt_attr, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), inMod.clone(), pre.clone(), inClass.clone(), inInstDims.clone(), inImpl.clone(), crate::InstTypes::CallingScope::INNER_CALL, inGraph.clone(), inSets.clone())?;
                    }
                    (cache, dae_var_attr) = InstBinding::instDaeVariableAttributes(cache.clone(), env_1.clone(), inMod.clone(), ty.clone(), metamodelica::nil())?;
                    attr = InstUtil::propagateAbSCDirection(vt.clone(), inAttributes.clone(), opt_attr.clone(), inInfo.clone())?;
                    attr = SCodeUtil::removeAttributeDimensions(attr.clone());
                    ident_ty = InstUtil::makeCrefBaseType(ty.clone(), inInstDims.clone())?;
                    cr = ComponentReferenceBasics::makeCrefIdent((inName.clone()).clone(), ident_ty.clone(), idxs.clone());
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), inPrefix.clone(), cr.clone())?;
                    InstUtil::checkModificationOnOuter(cache.clone(), env_1.clone(), ih.clone(), inPrefix.clone(), (inName.clone()).clone(), cr.clone(), inMod.clone(), vt.clone(), io.clone(), inImpl.clone(), inInfo.clone())?;
                    source = ElementSource::createElementSource(inInfo.clone(), FGraph::getScopePath(env_1.clone())?, inPrefix.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    r#mod = if (!(inSubscripts.clone().is_empty()) && !(SCodeUtil::isParameterOrConst(vt.clone())) && !(ClassInfUtil::isFunctionOrRecord(inState.clone())) && !(Types::isComplexType(Types::arrayElementType(ty.clone()))) && !(Types::isExternalObject(Types::arrayElementType(ty.clone()))) && !(Config::scalarizeBindings()?)) {Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)} else {inMod.clone()};
                    opt_binding = InstBinding::makeVariableBinding(ty.clone(), r#mod.clone(), Types::variabilityToConst(vt.clone())?, inPrefix.clone(), (inName.clone()).clone())?;
                    start = InstBinding::instStartBindingExp(inMod.clone(), ty.clone(), vt.clone())?;
                    if !(Flags::getConfigBool(Flags::USE_LOCAL_DIRECTION.clone())?) {
                        attr = stripVarAttrDirection(cr.clone(), ih.clone(), inState.clone(), inPrefix.clone(), attr.clone())?;
                    }
                    dae1 = InstUtil::propagateAttributes(dae1.clone(), attr.clone(), inPrefixes.clone(), inInfo.clone())?;
                    dae2 = InstDAE::daeDeclare(cache.clone(), env.clone(), env_1.clone(), cr.clone(), inState.clone(), ty.clone(), attr.clone(), vis.clone(), opt_binding.clone(), inInstDims.clone(), start.clone(), dae_var_attr.clone(), inComment.clone(), io.clone(), fin.clone(), source.clone(), false)?;
                    store = UnitAbsynBuilder::instAddStore(store.clone(), ty.clone(), cr.clone())?;
                    dae = instScalar2(cr.clone(), ty.clone(), vt.clone(), inMod.clone(), dae2.clone(), dae1.clone(), source.clone(), inImpl.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut implicitInstantiation: bool = false;
                    let mut inStateAndClassNameIsEqual: bool = false;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instScalar failed on ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!(" in scope ")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(inPrefix.clone())?); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn stripVarAttrDirection(mut inCref: Arc<DAE::ComponentRef>, mut ih: InstanceHierarchy, mut inState: ClassInf::State, mut inPrefix: DAE::Prefix, mut inAttributes: SCode::Attributes) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes;
    outAttributes = 'mc: {
        let __mc_input = (inCref.clone(), inState.clone(), inAttributes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, SCode::Attributes { direction: Absyn::Direction::BIDIR, .. }) => {
                    Ok(inAttributes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _, _) => {
                    Ok(inAttributes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::CONNECTOR { .. }, _) => {
                    if !((ConnectUtil::faceEqual(ConnectUtil::componentFaceType(inCref.clone())?, openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE))) { bail!("guard") }
                    Ok(inAttributes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut topInstance: InnerOuter::TopInstance;
                    let mut sm: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    topInstance = listHead(ih.clone())?;
                    let InnerOuter::TOP_INSTANCE { sm: __pa0, .. } = (topInstance.clone()) else { bail!("pattern mismatch") };
                    sm = __pa0.clone();
                    let true = (BaseHashSet::currentSize(sm.clone()) > 0) else { bail!("pattern mismatch") };
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), sm.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAttributes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(SCodeUtil::setAttributesDirection(inAttributes.clone(), openmodelica_ast::Absyn::Direction::BIDIR))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAttributes)
}

fn instScalar2(mut inCref: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inVariability: SCode::Variability, mut inMod: Arc<DAE::Mod>, mut inDae: DAE::DAElist, mut inClassDae: DAE::DAElist, mut inSource: Arc<DAE::ElementSource>, mut inImpl: bool) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((inCref.clone(), inType.clone(), inVariability.clone(), inMod.clone(), inDae.clone(), inClassDae.clone(), inSource.clone(), inImpl.clone())) {
        (_, _, SCode::Variability::CONST, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }, _, _, _, _) => {
            let mut dae: DAE::DAElist;
            dae = DAEUtil::joinDaes(inClassDae.clone(), inDae.clone())?;
            dae.clone()
        },
        (_, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, .. }), .. }, _, _, _, _) => {
            let mut dae: DAE::DAElist;
            dae = InstBinding::instModEquation(inCref.clone(), inType.clone(), inMod.clone(), inSource.clone(), inImpl.clone())?;
            dae = InstUtil::moveBindings(dae.clone(), inClassDae.clone())?;
            dae = DAEUtil::joinDaes(dae.clone(), inDae.clone())?;
            dae.clone()
        },
        (_, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, _, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: Deref @ DAE::Exp::CAST { exp: Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, .. }, .. }), .. }, _, _, _, _) => {
            let mut dae: DAE::DAElist;
            dae = InstBinding::instModEquation(inCref.clone(), inType.clone(), inMod.clone(), inSource.clone(), inImpl.clone())?;
            dae = InstUtil::moveBindings(dae.clone(), inClassDae.clone())?;
            dae = DAEUtil::joinDaes(dae.clone(), inDae.clone())?;
            dae.clone()
        },
        (_, _, SCode::Variability::PARAM, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { .. }), .. }, _, _, _, _) => {
            let mut dae: DAE::DAElist;
            dae = InstBinding::instModEquation(inCref.clone(), inType.clone(), inMod.clone(), inSource.clone(), inImpl.clone())?;
            dae = InstUtil::propagateBinding(inClassDae.clone(), dae.clone())?;
            dae = DAEUtil::joinDaes(dae.clone(), inDae.clone())?;
            dae.clone()
        },
        _ => {
            let mut dae: DAE::DAElist;
            let mut cls_dae: DAE::DAElist;
            dae = if (Types::isComplexType(inType.clone())) {InstBinding::instModEquation(inCref.clone(), inType.clone(), inMod.clone(), inSource.clone(), inImpl.clone())?} else {DAE::emptyDae.clone()};
            cls_dae = stripRecordDefaultBindingsFromDAE(inClassDae.clone(), inType.clone(), dae.clone());
            dae = DAEUtil::joinDaes(dae.clone(), inDae.clone())?;
            dae = DAEUtil::joinDaes(cls_dae.clone(), dae.clone())?;
            dae.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDae)
}

fn stripRecordDefaultBindingsFromDAE(mut inClassDAE: DAE::DAElist, mut inType: Arc<DAE::Type>, mut inEqDAE: DAE::DAElist) -> DAE::DAElist {
    let mut outClassDAE: DAE::DAElist;
    outClassDAE = (::match_deref::match_deref! { match &((inClassDAE.clone(), inType.clone(), inEqDAE.clone())) {
        (DAE::DAElist { elementLst: els }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, DAE::DAElist { elementLst: eqs @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }) => {
            let mut els = (*els).clone();
            (els, _) = List::mapFold(els.clone(), Arc::new(stripRecordDefaultBindingsFromElement), eqs.clone());
            DAE::DAElist { elementLst: els.clone() }
        },
        _ => {
            inClassDAE.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outClassDAE
}

fn stripRecordDefaultBindingsFromElement(mut inVar: Arc<DAE::Element>, mut inEqs: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<DAE::Element>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outVar: Arc<DAE::Element>;
    let mut outEqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    (outVar, outEqs) = (::match_deref::match_deref! { match &((inVar.clone(), inEqs.clone())) {
        (Deref @ DAE::Element::VAR { componentRef: var_cr, .. }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: eq_cr, .. }, .. }, tail: rest_eqs }) if (ComponentReferenceBasics::crefEqual(var_cr.clone(), eq_cr.clone())?) => {
            (DAEUtil::setElementVarBinding(inVar.clone(), None), rest_eqs.clone())
        },
        (Deref @ DAE::Element::VAR { componentRef: var_cr, .. }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: eq_cr, .. }, .. }, tail: _ }) if (ComponentReferenceBasics::crefPrefixOf(eq_cr.clone(), var_cr.clone())?) => {
            (DAEUtil::setElementVarBinding(inVar.clone(), None), inEqs.clone())
        },
        _ => {
            (inVar.clone(), inEqs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outEqs))
}

fn checkDimensionGreaterThanZero(mut inDim: Arc<DAE::Dimension>, mut inPrefix: DAE::Prefix, mut inIdent: ArcStr, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inDim.clone()) {
        Deref @ DAE::Dimension::DIM_INTEGER { .. } => {
            let mut dim_str: ArcStr = arcstr::literal!("");
            let mut cr_str: ArcStr = arcstr::literal!("");
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            if var_field!((*inDim).integer, DAE::Dimension::DIM_INTEGER).clone() < 0 {
                dim_str = (ExpressionBasics::dimensionString(inDim.clone())?).clone();
                cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (inIdent.clone()).clone(), identType: DAE::T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() });
                cr_str = (ComponentReferenceBasics::printComponentRefStr(PrefixUtil::prefixCrefNoContext(inPrefix.clone(), cr.clone())?)?).clone();
                Error::addSourceMessageAndFail(Error::NEGATIVE_DIMENSION_INDEX.clone(), list![(dim_str.clone()).clone(), (cr_str.clone()).clone()], info.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkArrayModDimSize(mut r#mod: Arc<DAE::Mod>, mut inDimension: Arc<DAE::Dimension>, mut inPrefix: DAE::Prefix, mut inIdent: ArcStr, mut inInfo: SourceInfo) -> () {
    let _ = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ DAE::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH, .. } => {
            List::map4_0(var_field!((*r#mod).subModLst, DAE::Mod::MOD).clone(), Arc::new(checkArraySubModDimSize), inDimension.clone(), inPrefix.clone(), (inIdent.clone()).clone(), inInfo.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn checkArraySubModDimSize(mut inSubMod: Arc<DAE::SubMod>, mut inDimension: Arc<DAE::Dimension>, mut inPrefix: DAE::Prefix, mut inIdent: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { ident: Deref @ "quantity", .. } => {
            ()
        },
        Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: eqmod, eachPrefix: SCode::Each::NOT_EACH, .. }, ident: name } => {
            let mut name = (*name).clone();
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIdent.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            let true = (checkArrayModBindingDimSize(eqmod.clone(), inDimension.clone(), inPrefix.clone(), (name.clone()).clone(), inInfo.clone())?) else { bail!("pattern mismatch") };
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkArrayModBindingDimSize(mut inBinding: Option<DAE::EqMod>, mut inDimension: Arc<DAE::Dimension>, mut inPrefix: DAE::Prefix, mut inIdent: ArcStr, mut inInfo: SourceInfo) -> Result<bool> {
    let mut outIsCorrect: bool = false;
    outIsCorrect = 'mc: {
        let __mc_input = inBinding.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let Some(DAE::EqMod::TYPED { info: mut info, properties: DAE::Properties::PROP { type_: ref ty, .. }, modifierAsExp: ref exp, .. }) = __mc_input.clone() else { bail!("nomatch") };
            let mut ty_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut dim_size1: i32 = 0;
            let mut dim_size2: i32 = 0;
            let mut exp_str: ArcStr = arcstr::literal!("");
            let mut exp_ty_str: ArcStr = arcstr::literal!("");
            let mut dims_str: ArcStr = arcstr::literal!("");
            let mut ty_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            ty_dim = Types::getDimensionNth(ty.clone(), 1)?;
            dim_size1 = Expression::dimensionSize(inDimension.clone())?;
            dim_size2 = Expression::dimensionSize(ty_dim.clone())?;
            let true = (dim_size1.clone() != dim_size2.clone()) else { bail!("pattern mismatch") };
            exp_str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            exp_ty_str = (TypesDump::unparseType(ty.clone())?).clone();
            let __pa0 = ::match_deref::match_deref! { match &(TypesDump::getDimensions(ty.clone())) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty_dims = __pa0.clone();
            dims_str = (ExpressionBasics::dimensionsString(cons(inDimension.clone(), ty_dims.clone()))).clone();
            Error::addMultiSourceMessage(Error::ARRAY_DIMENSION_MISMATCH.clone(), list![(exp_str.clone()).clone(), (exp_ty_str.clone()).clone(), (dims_str.clone()).clone()], cons(info.clone(), cons(inInfo.clone(), metamodelica::nil())))?;
            Ok(false)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIsCorrect)
}

fn instArray(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inState: ClassInf::State, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inIdent: ArcStr, mut inElement: (Arc<SCode::Element>, SCode::Attributes), mut inPrefixes: Arc<SCode::Prefixes>, mut inInteger: i32, mut inDimension: Arc<DAE::Dimension>, mut inDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inIntegerLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inBoolean: bool, mut inComment: Arc<SCode::Comment>, mut info: SourceInfo, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    checkDimensionGreaterThanZero(inDimension.clone(), inPrefix.clone(), (inIdent.clone()).clone(), info.clone())?;
    checkArrayModDimSize(inMod.clone(), inDimension.clone(), inPrefix.clone(), (inIdent.clone()).clone(), info.clone());
    (outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inStore.clone(), inState.clone(), inMod.clone(), inPrefix.clone(), inIdent.clone(), inElement.clone(), inPrefixes.clone(), inInteger.clone(), inDimension.clone(), inDimensionLst.clone(), inIntegerLst.clone(), inInstDims.clone(), inBoolean.clone(), inComment.clone(), info.clone(), inGraph.clone(), inSets.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ClassInf::State::FUNCTION { .. }, r#mod, pre, n, (cl, _), _, _, dim, _, _, inst_dims, _, _, _, graph, csets) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut p: DAE::Properties;
                    let mut env_1: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ty_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let true = (Expression::dimensionUnknownOrExp(dim.clone())) else { bail!("pattern mismatch") };
                    let Some(DAE::TYPED { properties: __pa0, modifierAsExp: __pa1, .. }) = (Mod::modEquation(r#mod.clone())?) else { bail!("pattern mismatch") };
                    p = __pa0.clone();
                    e = __pa1.clone();
                    (cache, env_1, ih, store, _, _, ty, _, _, graph) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), store.clone(), r#mod.clone(), pre.clone(), cl.clone(), inst_dims.clone(), true, crate::InstTypes::CallingScope::INNER_CALL, graph.clone(), csets.clone())?;
                    ty_1 = Types::simplifyType(ty.clone())?;
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), env.clone(), ih.clone(), pre.clone(), ComponentReferenceBasics::makeCrefIdent((n.clone()).clone(), ty_1.clone(), metamodelica::nil()))?;
                    (rhs, _) = Types::matchProp(e.clone(), p.clone(), DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, true)?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(env.clone())?, pre.clone(), (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
                    lhs = Expression::makeCrefExp(cr.clone(), ty_1.clone())?;
                    dae = InstSection::makeDaeEquation(lhs.clone(), rhs.clone(), source.clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL)?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), dae.clone(), inSets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, (cl, attr), pf, i, _, dims, idxs, inst_dims, r#impl, comment, _, graph, csets) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut compenv: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut daeLst: DAE::DAElist;
                    let mut s: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    let false = (Expression::dimensionKnown(inDimension.clone())) else { bail!("pattern mismatch") };
                    e = Arc::new(DAE::Exp::ICONST { integer: i.clone() });
                    s = Arc::new(DAE::Subscript::INDEX { exp: e.clone() });
                    r#mod = Mod::lookupIdxModification(r#mod.clone(), e.clone())?;
                    (cache, compenv, ih, store, daeLst, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), cons(s.clone(), idxs.clone()), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), daeLst.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, _, pre, n, (cl, attr), pf, _, Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, dims, idxs, inst_dims, r#impl, comment, _, graph, csets) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut compenv: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut s: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    ErrorExt::setCheckpoint((literal!("instArray Real[0]")).clone());
                    e = Arc::new(DAE::Exp::ICONST { integer: 0 });
                    s = Arc::new(DAE::Subscript::INDEX { exp: e.clone() });
                    r#mod = Mod::filterRedeclares(inMod.clone())?;
                    r#mod = Mod::lookupIdxModification(r#mod.clone(), e.clone())?;
                    (cache, compenv, ih, store, _, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), cons(s.clone(), idxs.clone()), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    ErrorExt::rollBack((literal!("instArray Real[0]")).clone());
                    Ok((cache.clone(), compenv.clone(), ih.clone(), store.clone(), DAE::emptyDae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, _, _, _, _, _, _, _, _) => {
                    ErrorExt::delCheckpoint((literal!("instArray Real[0]")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, _, _, _, (_, _), _, _, Deref @ DAE::Dimension::DIM_INTEGER { integer: stop }, _, _, _, _, _, _, graph, csets) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    (cache, env, ih, store, dae, csets, ty, graph) = instArrayDimInteger(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), inMod.clone(), inPrefix.clone(), (inIdent.clone()).clone(), inElement.clone(), inPrefixes.clone(), stop.clone(), inDimensionLst.clone(), inIntegerLst.clone(), inInstDims.clone(), inBoolean.clone(), inComment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), store.clone(), dae.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, (cl, attr), pf, _, Deref @ DAE::Dimension::DIM_ENUM { .. }, dims, idxs, inst_dims, r#impl, comment, _, graph, csets) => {
                    Ok(instArrayDimEnum(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), r#mod.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), inDimension.clone(), dims.clone(), idxs.clone(), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, store, ci_state, r#mod, pre, n, (cl, attr), pf, _, Deref @ DAE::Dimension::DIM_BOOLEAN, dims, idxs, inst_dims, r#impl, comment, _, graph, csets) => {
                    let mut env_1: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mod_2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut dae1: DAE::DAElist;
                    let mut dae2: DAE::DAElist;
                    let mut daeLst: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut store = (*store).clone();
                    let mut graph = (*graph).clone();
                    let mut csets = (*csets).clone();
                    mod_1 = Mod::lookupIdxModification(r#mod.clone(), Arc::new(DAE::Exp::BCONST { bool: false }))?;
                    mod_2 = Mod::lookupIdxModification(r#mod.clone(), Arc::new(DAE::Exp::BCONST { bool: true }))?;
                    (cache, env_1, ih, store, dae1, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), mod_1.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), cons(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::BCONST { bool: false }) }), idxs.clone()), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    (cache, _, ih, store, dae2, csets, ty, graph) = instVar2(cache.clone(), env.clone(), ih.clone(), store.clone(), ci_state.clone(), mod_2.clone(), pre.clone(), (n.clone()).clone(), cl.clone(), attr.clone(), pf.clone(), dims.clone(), cons(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::BCONST { bool: true }) }), idxs.clone()), inst_dims.clone(), r#impl.clone(), comment.clone(), info.clone(), graph.clone(), csets.clone())?;
                    daeLst = DAEUtil::joinDaes(dae1.clone(), dae2.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), store.clone(), daeLst.clone(), csets.clone(), ty.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, ci_state, r#mod, pre, n, (_, _), _, i, _, _, idxs, _, _, _, _, _, _) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut str4: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        let _ = unwrap_break_err!(Mod::lookupIdxModification(r#mod.clone(), Arc::new(DAE::Exp::ICONST { integer: i.clone() })), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    str1 = (PrefixUtil::printPrefixStrIgnoreNoPre(PrefixUtil::prefixAdd((n.clone()).clone(), metamodelica::nil(), metamodelica::nil(), pre.clone(), openmodelica_frontend_types::SCode::Variability::VAR, ci_state.clone(), info.clone())?)?).clone();
                    str2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(List::map(idxs.clone(), Arc::new(ExpressionBasics::printSubscriptStr)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    str3 = (Mod::prettyPrintMod(r#mod.clone(), 1)?).clone();
                    str4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(pre.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*str2.clone()); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*str3.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    str2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*str2.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::MODIFICATION_INDEX_NOT_FOUND.clone(), list![(str1.clone()).clone(), (str4.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instArray failed: ")); __mm_s.push_str(&*inIdent.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn instArrayDimInteger(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inState: ClassInf::State, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inName: ArcStr, mut inElement: (Arc<SCode::Element>, SCode::Attributes), mut inPrefixes: Arc<SCode::Prefixes>, mut inDimensionSize: i32, mut inRestDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImpl: bool, mut inComment: Arc<SCode::Comment>, mut inInfo: SourceInfo, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outStore: UnitAbsyn::InstStore = inStore.clone();
    let mut outDae: DAE::DAElist = DAE::emptyDae.clone();
    let mut outSets: DAE::Connect::Sets = inSets.clone();
    let mut outType: Arc<DAE::Type> = DAE::T_UNKNOWN_DEFAULT.clone();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph.clone();
    let mut c: Arc<SCode::Element>;
    let mut cls: Arc<SCode::Element>;
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut imod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut cls_path: Arc<Absyn::Path>;
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut attr: SCode::Attributes;
    let mut e: Arc<DAE::Exp>;
    let mut s: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    let mut inst_dims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>> = metamodelica::nil();
    let mut dae: DAE::DAElist;
    (cls, r#mod, attr, inst_dims) = (::match_deref::match_deref! { match &(inElement.clone()) {
        (c @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: smod, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cls_path, arrayDim: Some(_) }, .. }, .. }, attr) => {
            let mut smod = (*smod).clone();
            (_, cls, _) = Lookup::lookupClass(outCache.clone(), outEnv.clone(), cls_path.clone(), Some(c.info.clone()))?;
            smod = InstUtil::chainRedeclares(inMod.clone(), smod.clone())?;
            (_, r#mod) = Mod::elabMod(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), smod.clone(), inImpl.clone(), Mod::ModScope::DERIVED { path: cls_path.clone() }, inInfo.clone())?;
            r#mod = Mod::merge(inMod.clone(), r#mod.clone(), (literal!("")).clone(), true)?;
            (cls.clone(), r#mod.clone(), attr.clone(), metamodelica::nil())
        },
        (cls, attr) => (cls.clone(), inMod.clone(), attr.clone(), inInstDims.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut i in (1..=inDimensionSize.clone()).rev() {
        e = Arc::new(DAE::Exp::ICONST { integer: i.clone() });
        imod = Mod::lookupIdxModification(r#mod.clone(), e.clone())?;
        s = Arc::new(DAE::Subscript::INDEX { exp: e.clone() });
        (outCache, outEnv, outIH, outStore, dae, outSets, outType, outGraph) = instVar2(outCache.clone(), inEnv.clone(), outIH.clone(), outStore.clone(), inState.clone(), imod.clone(), inPrefix.clone(), (inName.clone()).clone(), cls.clone(), attr.clone(), inPrefixes.clone(), inRestDimensions.clone(), cons(s.clone(), inSubscripts.clone()), inst_dims.clone(), inImpl.clone(), inComment.clone(), inInfo.clone(), outGraph.clone(), outSets.clone())?;
        outDae = DAEUtil::joinDaes(dae.clone(), outDae.clone())?;
    }
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

fn instArrayDimEnum(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inStore: UnitAbsyn::InstStore, mut inState: ClassInf::State, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inName: ArcStr, mut inClass: Arc<SCode::Element>, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inDimension: Arc<DAE::Dimension>, mut inRestDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inImpl: bool, mut inComment: Arc<SCode::Comment>, mut inInfo: SourceInfo, mut inGraph: ConnectionGraph::ConnectionGraph, mut inSets: DAE::Connect::Sets) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, UnitAbsyn::InstStore, DAE::DAElist, DAE::Connect::Sets, Arc<DAE::Type>, ConnectionGraph::ConnectionGraph)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outStore: UnitAbsyn::InstStore = inStore.clone();
    let mut outDae: DAE::DAElist = DAE::emptyDae.clone();
    let mut outSets: DAE::Connect::Sets = inSets.clone();
    let mut outType: Arc<DAE::Type> = DAE::T_UNKNOWN_DEFAULT.clone();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph.clone();
    let mut enum_path: Arc<Absyn::Path>;
    let mut enum_lit_path: Arc<Absyn::Path>;
    let mut literals: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut i: i32 = 1;
    let mut e: Arc<DAE::Exp>;
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut dae: DAE::DAElist;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDimension.clone()) {
        Deref @ DAE::Dimension::DIM_ENUM { literals: __pa0, enumTypeName: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    literals = __pa0.clone();
    enum_path = __pa1.clone();
    for mut lit in &*literals.clone() {
        let mut lit = lit.clone();
        enum_lit_path = AbsynUtil::joinPaths(enum_path.clone(), Arc::new(Absyn::Path::IDENT { name: (lit.clone()).clone() }))?;
        e = Arc::new(DAE::Exp::ENUM_LITERAL { name: enum_lit_path.clone(), index: i.clone() });
        r#mod = Mod::lookupIdxModification(inMod.clone(), e.clone())?;
        i = i.clone() + 1;
        (outCache, outEnv, outIH, outStore, dae, outSets, outType, outGraph) = instVar2(outCache.clone(), inEnv.clone(), outIH.clone(), outStore.clone(), inState.clone(), r#mod.clone(), inPrefix.clone(), (inName.clone()).clone(), inClass.clone(), inAttributes.clone(), inPrefixes.clone(), inRestDimensions.clone(), cons(Arc::new(DAE::Subscript::INDEX { exp: e.clone() }), inSubscripts.clone()), inInstDims.clone(), inImpl.clone(), inComment.clone(), inInfo.clone(), outGraph.clone(), outSets.clone())?;
        outDae = DAEUtil::joinDaes(outDae.clone(), dae.clone())?;
    }
    Ok((outCache, outEnv, outIH, outStore, outDae, outSets, outType, outGraph))
}

