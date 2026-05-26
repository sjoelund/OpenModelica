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
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use crate::HashSet;
use crate::InstSection;
use crate::Lookup;
use crate::Mod;
use crate::PrefixUtil;
use crate::UnitAbsyn;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub type Cache = FCore::Cache;

#[derive(Clone, Debug, PartialEq)]
pub struct InstResult {
    pub outCache: Cache,
    pub outEnv: FCore::Graph,
    pub outStore: UnitAbsyn::InstStore,
    pub outDae: DAE::DAElist,
    pub outSets: DAE::Connect::Sets,
    pub outType: Arc<DAE::Type>,
    pub outGraph: ConnectionGraph::ConnectionGraph,
}

pub type INST_RESULT = InstResult;


#[derive(Clone, Debug, PartialEq)]
pub struct InstInner {
    /// the prefix of the inner. we need it to prefix the outer variables with it!
    pub innerPrefix: DAE::Prefix,
    pub name: ArcStr,
    pub io: Absyn::InnerOuter,
    /// full inner component name
    pub fullName: ArcStr,
    /// the type of the inner
    pub typePath: Arc<Absyn::Path>,
    /// the scope of the inner
    pub scope: ArcStr,
    pub instResult: Option<InstResult>,
    /// which outers are referencing this inner
    pub outers: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    /// class or component
    pub innerElement: Option<Arc<SCode::Element>>,
}

pub type INST_INNER = InstInner;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OuterPrefix {
    /// the prefix of this outer + component name
    pub outerComponentRef: Arc<DAE::ComponentRef>,
    /// the coresponding prefix for this outer + component name
    pub innerComponentRef: Arc<DAE::ComponentRef>,
}

pub type OUTER = OuterPrefix;


pub type OuterPrefixes = Arc<metamodelica::List<OuterPrefix>>;

pub static emptyOuterPrefixes: std::sync::LazyLock<Arc<metamodelica::List<OuterPrefix>>> = std::sync::LazyLock::new(|| { metamodelica::nil() });

/// the prefix + '.' + the component name
pub type Key = Arc<DAE::ComponentRef>;

/// the inputs of the instantiation function and the results
pub type Value = InstInner;

/// a top instance is an instance of a model thar resides at top level
#[derive(Clone, Debug, PartialEq)]
pub struct TopInstance {
    /// top model path
    pub path: Option<Arc<Absyn::Path>>,
    /// hash table with fully qualified components
    pub ht: InstHierarchyHashTable,
    /// the outer prefixes help us prefix the outer components with the correct prefix of inner component directly
    pub outerPrefixes: OuterPrefixes,
    /// Set of synchronous SM states (fully qualified components)
    pub sm: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)),
}

pub type TOP_INSTANCE = TopInstance;


pub type InstHierarchy = Arc<metamodelica::List<TopInstance>>;

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<metamodelica::List<TopInstance>>
// Expr: Array { elems: [], ty: List(Unknown) }
pub fn emptyInstHierarchy() -> Arc<metamodelica::List<TopInstance>> { todo!("non-Sync, non-const-emittable constant emptyInstHierarchy — extend codegen") }

pub fn handleInnerOuterEquations(mut io: Absyn::InnerOuter, mut inDae: DAE::DAElist, mut inIH: InstHierarchy, mut inGraphNew: ConnectionGraph::ConnectionGraph, mut inGraph: ConnectionGraph::ConnectionGraph) -> Result<(DAE::DAElist, InstHierarchy, ConnectionGraph::ConnectionGraph)> {
    let mut odae: DAE::DAElist;
    let mut outIH: InstHierarchy = metamodelica::nil();
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (odae, outIH, outGraph) = 'mc: {
        let __mc_input = (io.clone(), inDae.clone(), inIH.clone(), inGraphNew.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::InnerOuter::OUTER, dae, ih, _, graph) => {
                    let mut odae: DAE::DAElist;
                    (odae, _) = DAEUtil::splitDAEIntoVarsAndEquations(dae.clone())?;
                    Ok((odae.clone(), ih.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::InnerOuter::INNER_OUTER, dae, ih, _, graph) => {
                    let mut dae1: DAE::DAElist;
                    let mut dae2: DAE::DAElist;
                    let mut dae = (*dae).clone();
                    (dae1, dae2) = DAEUtil::splitDAEIntoVarsAndEquations(dae.clone())?;
                    dae2 = DAEUtil::nameUniqueOuterVars(dae2.clone());
                    dae = DAEUtil::joinDaes(dae1.clone(), dae2.clone())?;
                    Ok((dae.clone(), ih.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::InnerOuter::INNER, dae, ih, graphNew, _) => {
                    Ok((dae.clone(), ih.clone(), graphNew.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::InnerOuter::NOT_INNER_OUTER, dae, ih, graphNew, _) => {
                    Ok((dae.clone(), ih.clone(), graphNew.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("- InnerOuter.handleInnerOuterEquations failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((odae, outIH, outGraph))
}

pub fn changeInnerOuterInOuterConnect(mut sets: DAE::Connect::Sets) -> DAE::Connect::Sets {
    let mut sets: DAE::Connect::Sets = sets;
    sets.outerConnects = List::map(sets.outerConnects.clone(), Arc::new(changeInnerOuterInOuterConnect2));
    sets
}

fn changeInnerOuterInOuterConnect2(mut inOC: DAE::Connect::OuterConnect) -> Result<DAE::Connect::OuterConnect> {
    let mut outOC: DAE::Connect::OuterConnect;
    outOC = 'mc: {
        let __mc_input = inOC.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Connect::OuterConnect { scope: mut scope, cr1: mut cr1, io1: mut io1, f1: mut f1, cr2: mut cr2, io2: mut io2, f2: mut f2, source: mut source } = __mc_input.clone() else { bail!("nomatch") };
            let mut ncr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let (_, true) = (innerOuterBooleans(io1.clone())?) else { bail!("pattern mismatch") };
            ncr1 = PrefixUtil::prefixToCref(scope.clone())?;
            let false = (ComponentReferenceBasics::crefFirstCrefLastCrefEqual(ncr1.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
            Ok(DAE::Connect::OuterConnect { scope: scope.clone(), cr1: cr1.clone(), io1: openmodelica_ast::Absyn::InnerOuter::INNER, f1: f1.clone(), cr2: cr2.clone(), io2: io2.clone(), f2: f2.clone(), source: source.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Connect::OuterConnect { scope: mut scope, cr1: mut cr1, io1: mut io1, f1: mut f1, cr2: mut cr2, io2: mut io2, f2: mut f2, source: mut source } = __mc_input.clone() else { bail!("nomatch") };
            let mut ncr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let (_, true) = (innerOuterBooleans(io2.clone())?) else { bail!("pattern mismatch") };
            ncr2 = PrefixUtil::prefixToCref(scope.clone())?;
            let false = (ComponentReferenceBasics::crefFirstCrefLastCrefEqual(ncr2.clone(), cr2.clone())?) else { bail!("pattern mismatch") };
            Ok(DAE::Connect::OuterConnect { scope: scope.clone(), cr1: cr1.clone(), io1: io1.clone(), f1: f1.clone(), cr2: cr2.clone(), io2: openmodelica_ast::Absyn::InnerOuter::INNER, f2: f2.clone(), source: source.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inOC.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outOC)
}

pub fn retrieveOuterConnections(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstHierarchy, mut inPrefix: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut inTopCall: bool, mut inCGraph: ConnectionGraph::ConnectionGraph) -> Result<(DAE::Connect::Sets, Arc<metamodelica::List<DAE::Connect::OuterConnect>>, ConnectionGraph::ConnectionGraph)> {
    let mut outSets: DAE::Connect::Sets;
    let mut outInnerOuterConnects: Arc<metamodelica::List<DAE::Connect::OuterConnect>> = metamodelica::nil();
    let mut outCGraph: ConnectionGraph::ConnectionGraph;
    let mut oc: Arc<metamodelica::List<DAE::Connect::OuterConnect>> = metamodelica::nil();
    let DAE::Connect::Sets { outerConnects: __pa0, .. } = (inSets.clone()) else { bail!("pattern mismatch") };
    oc = __pa0.clone();
    (oc, outSets, outInnerOuterConnects, outCGraph) = retrieveOuterConnections2(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), oc.clone(), inSets.clone(), inTopCall.clone(), inCGraph.clone())?;
    outSets.outerConnects = oc.clone();
    Ok((outSets, outInnerOuterConnects, outCGraph))
}

fn removeInnerPrefixFromCref(mut inPrefix: DAE::Prefix, mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = 'mc: {
        let __mc_input = (inPrefix.clone(), inCref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Prefix::NOPRE, _) => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut crefPrefix: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crOuter: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    crefPrefix = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    crOuter = ComponentReference::crefStripPrefix(inCref.clone(), crefPrefix.clone())?;
                    Ok(crOuter.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

fn retrieveOuterConnections2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstHierarchy, mut inPrefix: DAE::Prefix, mut inOuterConnects: Arc<metamodelica::List<DAE::Connect::OuterConnect>>, mut inSets: DAE::Connect::Sets, mut inTopCall: bool, mut inCGraph: ConnectionGraph::ConnectionGraph) -> Result<(Arc<metamodelica::List<DAE::Connect::OuterConnect>>, DAE::Connect::Sets, Arc<metamodelica::List<DAE::Connect::OuterConnect>>, ConnectionGraph::ConnectionGraph)> {
    let mut outOuterConnects: Arc<metamodelica::List<DAE::Connect::OuterConnect>> = metamodelica::nil();
    let mut outSets: DAE::Connect::Sets;
    let mut outInnerOuterConnects: Arc<metamodelica::List<DAE::Connect::OuterConnect>> = metamodelica::nil();
    let mut outCGraph: ConnectionGraph::ConnectionGraph;
    (outOuterConnects, outSets, outInnerOuterConnects, outCGraph) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inOuterConnects.clone(), inSets.clone(), inTopCall.clone(), inCGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok((inOuterConnects.clone(), inSets.clone(), metamodelica::nil(), inCGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: DAE::Connect::OuterConnect { scope, cr1, io1, f1, cr2, io2, f2, source: source @ Deref @ DAE::ElementSource { info, .. } }, tail: rest_oc }, sets, _, graph) => {
                    let mut ioc: Arc<metamodelica::List<DAE::Connect::OuterConnect>> = metamodelica::nil();
                    let mut inner1: bool = false;
                    let mut outer1: bool = false;
                    let mut added: bool = false;
                    let mut cr1 = (*cr1).clone();
                    let mut cr2 = (*cr2).clone();
                    let mut rest_oc = (*rest_oc).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    (inner1, outer1) = lookupVarInnerOuterAttr(inCache.clone(), inEnv.clone(), inIH.clone(), cr1.clone(), cr2.clone())?;
                    let true = (inner1.clone()) else { bail!("pattern mismatch") };
                    let false = (outer1.clone()) else { bail!("pattern mismatch") };
                    cr1 = removeInnerPrefixFromCref(inPrefix.clone(), cr1.clone())?;
                    cr2 = removeInnerPrefixFromCref(inPrefix.clone(), cr2.clone())?;
                    (sets, added) = ConnectUtil::addOuterConnectToSets(cr1.clone(), cr2.clone(), io1.clone(), io2.clone(), f1.clone(), f2.clone(), sets.clone(), info.clone())?;
                    (sets, graph) = addOuterConnectIfEmpty(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), sets.clone(), added.clone(), cr1.clone(), io1.clone(), f1.clone(), cr2.clone(), io2.clone(), f2.clone(), info.clone(), graph.clone())?;
                    (rest_oc, sets, ioc, graph) = retrieveOuterConnections2(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), rest_oc.clone(), sets.clone(), inTopCall.clone(), graph.clone())?;
                    rest_oc = if (outer1.clone()) {cons(DAE::Connect::OuterConnect { scope: scope.clone(), cr1: cr1.clone(), io1: io1.clone(), f1: f1.clone(), cr2: cr2.clone(), io2: io2.clone(), f2: f2.clone(), source: source.clone() }, rest_oc.clone())} else {rest_oc.clone()};
                    Ok((rest_oc.clone(), sets.clone(), ioc.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: DAE::Connect::OuterConnect { scope: _, cr1, io1, f1, cr2, io2, f2, source: Deref @ DAE::ElementSource { info, .. } }, tail: rest_oc }, sets, true, graph) => {
                    let mut ioc: Arc<metamodelica::List<DAE::Connect::OuterConnect>> = metamodelica::nil();
                    let mut inner1: bool = false;
                    let mut inner2: bool = false;
                    let mut outer1: bool = false;
                    let mut outer2: bool = false;
                    let mut added: bool = false;
                    let mut io1 = (*io1).clone();
                    let mut io2 = (*io2).clone();
                    let mut rest_oc = (*rest_oc).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    (inner1, outer1) = innerOuterBooleans(io1.clone())?;
                    (inner2, outer2) = innerOuterBooleans(io2.clone())?;
                    let true = (boolOr(inner1.clone(), inner2.clone())) else { bail!("pattern mismatch") };
                    let false = (boolOr(outer1.clone(), outer2.clone())) else { bail!("pattern mismatch") };
                    io1 = convertInnerOuterInnerToOuter(io1.clone());
                    io2 = convertInnerOuterInnerToOuter(io2.clone());
                    (sets, added) = ConnectUtil::addOuterConnectToSets(cr1.clone(), cr2.clone(), io1.clone(), io2.clone(), f1.clone(), f2.clone(), sets.clone(), info.clone())?;
                    (sets, graph) = addOuterConnectIfEmpty(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), sets.clone(), added.clone(), cr1.clone(), io1.clone(), f1.clone(), cr2.clone(), io2.clone(), f2.clone(), info.clone(), graph.clone())?;
                    (rest_oc, sets, ioc, graph) = retrieveOuterConnections2(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), rest_oc.clone(), sets.clone(), true, graph.clone())?;
                    Ok((rest_oc.clone(), sets.clone(), ioc.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: oc, tail: rest_oc }, sets, _, graph) => {
                    let mut ioc: Arc<metamodelica::List<DAE::Connect::OuterConnect>> = metamodelica::nil();
                    let mut rest_oc = (*rest_oc).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    (rest_oc, sets, ioc, graph) = retrieveOuterConnections2(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), rest_oc.clone(), sets.clone(), inTopCall.clone(), graph.clone())?;
                    Ok((cons(oc.clone(), rest_oc.clone()), sets.clone(), ioc.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outOuterConnects, outSets, outInnerOuterConnects, outCGraph))
}

fn convertInnerOuterInnerToOuter(mut io: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut oio: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    oio = (match io.clone() {
        Absyn::InnerOuter::INNER => openmodelica_ast::Absyn::InnerOuter::OUTER,
        _ => io.clone(),
    });
    oio
}

fn addOuterConnectIfEmpty(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstHierarchy, mut pre: DAE::Prefix, mut inSets: DAE::Connect::Sets, mut added: bool, mut cr1: Arc<DAE::ComponentRef>, mut iio1: Absyn::InnerOuter, mut f1: DAE::Connect::Face, mut cr2: Arc<DAE::ComponentRef>, mut iio2: Absyn::InnerOuter, mut f2: DAE::Connect::Face, mut info: SourceInfo, mut inCGraph: ConnectionGraph::ConnectionGraph) -> Result<(DAE::Connect::Sets, ConnectionGraph::ConnectionGraph)> {
    let mut outSets: DAE::Connect::Sets;
    let mut outCGraph: ConnectionGraph::ConnectionGraph;
    (outSets, outCGraph) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), pre.clone(), inSets.clone(), added.clone(), cr1.clone(), iio1.clone(), f1.clone(), cr2.clone(), iio2.clone(), f2.clone(), info.clone(), inCGraph.clone())) {
        (_, _, _, _, _, true, _, _, _, _, _, _, _, _) => {
            (inSets.clone(), inCGraph.clone())
        },
        (cache, env, ih, _, DAE::Connect::Sets { sets, setCount: sc, connections: cl, outerConnects: oc }, false, _, io1, _, _, io2, _, _, graph) => {
            let mut vt1: SCode::Variability = SCode::Variability::CONST;
            let mut vt2: SCode::Variability = SCode::Variability::CONST;
            let mut t1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut t2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ct: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut ih = (*ih).clone();
            let mut sets = (*sets).clone();
            let mut sc = (*sc).clone();
            let mut cl = (*cl).clone();
            let mut io1 = (*io1).clone();
            let mut io2 = (*io2).clone();
            let mut graph = (*graph).clone();
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupVar(cache.clone(), env.clone(), cr1.clone())?) {
                (__pa0, Deref @ DAE::Attributes { variability: __pa1, connectorType: __pa2, .. }, __pa3, _, _, _, _, _, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            vt1 = __pa1.clone();
            ct = __pa2.clone();
            t1 = __pa3.clone();
            let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Lookup::lookupVar(cache.clone(), env.clone(), cr2.clone())?) {
                (__pa4, Deref @ DAE::Attributes { variability: __pa5, .. }, __pa6, _, _, _, _, _, _) => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa4.clone();
            vt2 = __pa5.clone();
            t2 = __pa6.clone();
            io1 = removeOuter(io1.clone())?;
            io2 = removeOuter(io2.clone())?;
            let (__pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(InstSection::connectComponents(cache.clone(), env.clone(), ih.clone(), DAE::Connect::Sets { sets: sets.clone(), setCount: sc.clone(), connections: cl.clone(), outerConnects: metamodelica::nil() }, pre.clone(), cr1.clone(), f1.clone(), t1.clone(), vt1.clone(), cr2.clone(), f2.clone(), t2.clone(), vt2.clone(), ct.clone(), io1.clone(), io2.clone(), graph.clone(), info.clone())?) {
                (__pa7, __pa8, __pa9, DAE::Connect::Sets { connections: __pa10, setCount: __pa11, sets: __pa12, .. }, _, __pa13) => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa7.clone();
            env = __pa8.clone();
            ih = __pa9.clone();
            cl = __pa10.clone();
            sc = __pa11.clone();
            sets = __pa12.clone();
            graph = __pa13.clone();
            (DAE::Connect::Sets { sets: sets.clone(), setCount: sc.clone(), connections: cl.clone(), outerConnects: oc.clone() }, graph.clone())
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSets, outCGraph))
}

fn removeOuter(mut io: Absyn::InnerOuter) -> Result<Absyn::InnerOuter> {
    let mut outIo: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    outIo = (match io.clone() {
        Absyn::InnerOuter::OUTER => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        Absyn::InnerOuter::INNER => openmodelica_ast::Absyn::InnerOuter::INNER,
        Absyn::InnerOuter::INNER_OUTER => openmodelica_ast::Absyn::InnerOuter::INNER,
        Absyn::InnerOuter::NOT_INNER_OUTER => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        _ => bail!("match: no arm matched"),
    });
    Ok(outIo)
}

fn lookupVarInnerOuterAttr(mut cache: FCore::Cache, mut env: FCore::Graph, mut inIH: InstHierarchy, mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<(bool, bool)> {
    let mut isInner: bool = false;
    let mut isOuter: bool = false;
    (isInner, isOuter) = 'mc: {
        let __mc_input = (cache.clone(), env.clone(), inIH.clone(), cr1.clone(), cr2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut io1: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut io2: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut isInner1: bool = false;
                    let mut isInner2: bool = false;
                    let mut isOuter1: bool = false;
                    let mut isOuter2: bool = false;
                    let mut isInner: bool = isInner.clone();
                    let mut isOuter: bool = isOuter.clone();
                    ErrorExt::setCheckpoint((literal!("lookupVarInnerOuterAttr")).clone());
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupVar(cache.clone(), env.clone(), cr1.clone())?) {
                        (_, Deref @ DAE::Attributes { innerOuter: __pa0, .. }, _, _, _, _, _, _, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    io1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(Lookup::lookupVar(cache.clone(), env.clone(), cr2.clone())?) {
                        (_, Deref @ DAE::Attributes { innerOuter: __pa1, .. }, _, _, _, _, _, _, _) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    io2 = __pa1.clone();
                    (isInner1, isOuter1) = innerOuterBooleans(io1.clone())?;
                    (isInner2, isOuter2) = innerOuterBooleans(io2.clone())?;
                    isInner = isInner1.clone() || isInner2.clone();
                    isOuter = isOuter1.clone() || isOuter2.clone();
                    ErrorExt::rollBack((literal!("lookupVarInnerOuterAttr")).clone());
                    Ok((isInner.clone(), isOuter.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut isOuter: bool = isOuter.clone();
                    let mut isInner: bool = isInner.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupVar(cache.clone(), env.clone(), cr1.clone())?) {
                        (_, Deref @ DAE::Attributes { innerOuter: __pa0, .. }, _, _, _, _, _, _, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    io = __pa0.clone();
                    (isInner, isOuter) = innerOuterBooleans(io.clone())?;
                    ErrorExt::rollBack((literal!("lookupVarInnerOuterAttr")).clone());
                    Ok((isInner.clone(), isOuter.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
                    let mut isInner: bool = isInner.clone();
                    let mut isOuter: bool = isOuter.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupVar(cache.clone(), env.clone(), cr2.clone())?) {
                        (_, Deref @ DAE::Attributes { innerOuter: __pa0, .. }, _, _, _, _, _, _, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    io = __pa0.clone();
                    (isInner, isOuter) = innerOuterBooleans(io.clone())?;
                    ErrorExt::rollBack((literal!("lookupVarInnerOuterAttr")).clone());
                    Ok((isInner.clone(), isOuter.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack((literal!("lookupVarInnerOuterAttr")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((isInner, isOuter))
}

fn innerOuterBooleans(mut io: Absyn::InnerOuter) -> Result<(bool, bool)> {
    let mut inner1: bool = false;
    let mut outer1: bool = false;
    (inner1, outer1) = (match io.clone() {
        Absyn::InnerOuter::INNER => (true, false),
        Absyn::InnerOuter::OUTER => (false, true),
        Absyn::InnerOuter::INNER_OUTER => (true, true),
        Absyn::InnerOuter::NOT_INNER_OUTER => (false, false),
        _ => bail!("match: no arm matched"),
    });
    Ok((inner1, outer1))
}

pub fn outerConnection(mut io1: Absyn::InnerOuter, mut io2: Absyn::InnerOuter) -> bool {
    let mut isOuter: bool = false;
    isOuter = (match (io1.clone(), io2.clone()) {
        (Absyn::InnerOuter::OUTER, _) => true,
        (_, Absyn::InnerOuter::OUTER) => true,
        (Absyn::InnerOuter::INNER_OUTER, _) => true,
        (_, Absyn::InnerOuter::INNER_OUTER) => true,
        _ => false,
    });
    isOuter
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lookupInnerInIH(mut inTIH: TopInstance, mut inPrefix: DAE::Prefix, mut inComponentIdent: ArcStr) -> Result<InstInner> {
    let mut outInstInner: InstInner;
    outInstInner = 'mc: {
        let __mc_input = (inTIH.clone(), inPrefix.clone(), inComponentIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { .. }, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, .. }, _) => {
                    Ok(lookupInnerInIH(inTIH.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, (inComponentIdent.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { .. }, DAE::Prefix::NOPRE, name) => {
                    Ok(emptyInstInner(openmodelica_frontend_types::DAE::Prefix::NOPRE, (name.clone()).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { path: _, ht, outerPrefixes: _, sm: _ }, _, name) => {
                    let mut prefix: DAE::Prefix = DAE::Prefix::NOPRE;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut instInner: InstInner;
                    prefix = PrefixUtil::prefixStripLast(inPrefix.clone())?;
                    (_, cref) = PrefixUtil::prefixCref(FCore::emptyCache(), FGraph::empty(), emptyInstHierarchy().clone(), prefix.clone(), ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    instInner = get(cref.clone(), ht.clone())?;
                    Ok(instInner.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { path: _, ht, outerPrefixes: _, sm: _ }, _, name) => {
                    let mut prefix: DAE::Prefix = DAE::Prefix::NOPRE;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut instInner: InstInner;
                    prefix = PrefixUtil::prefixStripLast(inPrefix.clone())?;
                    (_, cref) = PrefixUtil::prefixCref(FCore::emptyCache(), FGraph::empty(), emptyInstHierarchy().clone(), prefix.clone(), ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil()))?;
                    if '__try0: {
                        let _ = unwrap_break_err!(get(cref.clone(), ht.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    instInner = lookupInnerInIH(inTIH.clone(), prefix.clone(), (name.clone()).clone())?;
                    Ok(instInner.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TopInstance { .. }, prefix, name) => {
                    Ok(emptyInstInner(prefix.clone(), (name.clone()).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInstInner)
}

pub fn modificationOnOuter(mut cache: FCore::Cache, mut env: FCore::Graph, mut ih: InstHierarchy, mut prefix: DAE::Prefix, mut componentName: ArcStr, mut cr: Arc<DAE::ComponentRef>, mut inMod: Arc<DAE::Mod>, mut io: Absyn::InnerOuter, mut r#impl: bool, mut inInfo: SourceInfo) -> Result<bool> {
    let mut modd: bool = false;
    modd = 'mc: {
        let __mc_input = (cache.clone(), env.clone(), ih.clone(), prefix.clone(), componentName.clone(), cr.clone(), inMod.clone(), io.clone(), r#impl.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, Deref @ DAE::Mod::MOD { .. }, Absyn::InnerOuter::OUTER, _, _) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    s2 = (Mod::prettyPrintMod(inMod.clone(), 0)?).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::OUTER_MODIFICATION.clone(), list![(s.clone()).clone()], inInfo.clone())?;
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
    Ok(modd)
}

pub fn switchInnerToOuterInGraph(mut inEnv: FCore::Graph, mut inCr: Arc<DAE::ComponentRef>) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = (::match_deref::match_deref! { match &((inEnv.clone(), inCr.clone())) {
        (FCore::Graph::EG { name: _ }, _) => {
            inEnv.clone()
        },
        (FCore::Graph::G { scope: Deref @ metamodelica::List::Nil, .. }, _) => {
            inEnv.clone()
        },
        (_, cr) => {
            let mut r: metamodelica::Array<FCore::Node>;
            let mut n: FCore::Node;
            r = FGraph::lastScopeRef(inEnv.clone())?;
            n = FNode::fromRef(r.clone())?;
            n = switchInnerToOuterInNode(n.clone(), cr.clone());
            r = FNode::updateRef(r.clone(), n.clone())?;
            inEnv.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn switchInnerToOuterInNode(mut inNode: FCore::Node, mut inCr: Arc<DAE::ComponentRef>) -> FCore::Node {
    let mut outNode: FCore::Node = inNode.clone();
    let _ = (match outNode.clone() {
        FCore::Node { .. } => {
            outNode.children = FNode::RefTree::map(outNode.children.clone(), { let __pe_b1 = inCr.clone(); move |__pe_a0, __pe_a2| switchInnerToOuterInChild(__pe_a0, __pe_b1.clone(), __pe_a2) }).unwrap();
            ()
        },
        _ => (),
    });
    outNode
}

fn switchInnerToOuterInChild(mut name: ArcStr, mut cr: Arc<DAE::ComponentRef>, mut inRef: metamodelica::Array<FCore::Node>) -> Result<metamodelica::Array<FCore::Node>> {
    let mut r#ref: metamodelica::Array<FCore::Node>;
    let mut n: FCore::Node;
    n = FNode::fromRef(inRef.clone())?;
    n = switchInnerToOuterInChildrenValue(n.clone(), cr.clone())?;
    r#ref = FNode::updateRef(inRef.clone(), n.clone())?;
    Ok(r#ref)
}

fn switchInnerToOuterInChildrenValue(mut inNode: FCore::Node, mut inCr: Arc<DAE::ComponentRef>) -> Result<FCore::Node> {
    let mut outNode: FCore::Node;
    outNode = 'mc: {
        let __mc_input = (inNode.clone(), inCr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (node, _) => {
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut attributes: Arc<DAE::Attributes>;
                    let mut visibility: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut bndsrc: bool = false;
                    let mut ct: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
                    let mut parallelism: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
                    let mut variability: SCode::Variability = SCode::Variability::CONST;
                    let mut direction: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut cnstForRange: Option<DAE::Const> = None;
                    r = FNode::childFromNode(node.clone(), (arcstr::literal!(FNode::itNodeName)).clone())?;
                    let FCore::IT { i: __t6 } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*__t6).clone()) {
                        Deref @ DAE::Var { name: __pa0, attributes: __pa1, ty: __pa2, binding: __pa3, bind_from_outside: __pa4, constOfForIteratorRange: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    attributes = __pa1.clone();
                    ty = __pa2.clone();
                    binding = __pa3.clone();
                    bndsrc = __pa4.clone();
                    cnstForRange = __pa5.clone();
                    let (__pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(attributes.clone()) {
                        Deref @ DAE::Attributes { connectorType: __pa7, parallelism: __pa8, variability: __pa9, direction: __pa10, innerOuter: Absyn::InnerOuter::INNER, visibility: __pa11 } => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ct = __pa7.clone();
                    parallelism = __pa8.clone();
                    variability = __pa9.clone();
                    direction = __pa10.clone();
                    visibility = __pa11.clone();
                    attributes = Arc::new(DAE::Attributes { connectorType: ct.clone(), parallelism: parallelism.clone(), variability: variability.clone(), direction: direction.clone(), innerOuter: openmodelica_ast::Absyn::InnerOuter::OUTER, visibility: visibility.clone() });
                    r = FNode::updateRef(r.clone(), FNode::setData(FNode::fromRef(r.clone())?, FCore::Data::IT { i: Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: attributes.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: bndsrc.clone(), constOfForIteratorRange: cnstForRange.clone() }) })?)?;
                    Ok(node.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (node, _) => {
                    let mut r: metamodelica::Array<FCore::Node>;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut attributes: Arc<DAE::Attributes>;
                    let mut visibility: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut bndsrc: bool = false;
                    let mut ct: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
                    let mut parallelism: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
                    let mut variability: SCode::Variability = SCode::Variability::CONST;
                    let mut direction: Absyn::Direction = Absyn::Direction::BIDIR;
                    let mut cnstForRange: Option<DAE::Const> = None;
                    r = FNode::childFromNode(node.clone(), (arcstr::literal!(FNode::itNodeName)).clone())?;
                    let FCore::IT { i: __t6 } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*__t6).clone()) {
                        Deref @ DAE::Var { name: __pa0, attributes: __pa1, ty: __pa2, binding: __pa3, bind_from_outside: __pa4, constOfForIteratorRange: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    attributes = __pa1.clone();
                    ty = __pa2.clone();
                    binding = __pa3.clone();
                    bndsrc = __pa4.clone();
                    cnstForRange = __pa5.clone();
                    let (__pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(attributes.clone()) {
                        Deref @ DAE::Attributes { connectorType: __pa7, parallelism: __pa8, variability: __pa9, direction: __pa10, innerOuter: Absyn::InnerOuter::INNER_OUTER, visibility: __pa11 } => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ct = __pa7.clone();
                    parallelism = __pa8.clone();
                    variability = __pa9.clone();
                    direction = __pa10.clone();
                    visibility = __pa11.clone();
                    attributes = Arc::new(DAE::Attributes { connectorType: ct.clone(), parallelism: parallelism.clone(), variability: variability.clone(), direction: direction.clone(), innerOuter: openmodelica_ast::Absyn::InnerOuter::OUTER, visibility: visibility.clone() });
                    r = FNode::updateRef(r.clone(), FNode::setData(FNode::fromRef(r.clone())?, FCore::Data::IT { i: Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: attributes.clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: bndsrc.clone(), constOfForIteratorRange: cnstForRange.clone() }) })?)?;
                    Ok(node.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(inNode.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

// /////////////////////////////////////////////////
// / instance hieararchy for inner/outer
// / add furher functions before this
// /////////////////////////////////////////////////
fn emptyInstInner(mut innerPrefix: DAE::Prefix, mut name: ArcStr) -> InstInner {
    let mut outInstInner: InstInner;
    outInstInner = InstInner { innerPrefix: innerPrefix.clone(), name: (name.clone()).clone(), io: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, fullName: (literal!("")).clone(), typePath: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), scope: (literal!("")).clone(), instResult: None, outers: metamodelica::nil(), innerElement: None };
    outInstInner
}

pub fn lookupInnerVar(mut inCache: Cache, mut inEnv: FCore::Graph, mut inIH: InstHierarchy, mut inPrefix: DAE::Prefix, mut inIdent: ArcStr, mut io: Absyn::InnerOuter) -> Result<InstInner> {
    let mut outInstInner: InstInner;
    outInstInner = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inIdent.clone(), io.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: tih, tail: _ }, pre, n, _) => {
                    let mut instInner: InstInner;
                    instInner = lookupInnerInIH(tih.clone(), pre.clone(), (n.clone()).clone())?;
                    Ok(instInner.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, pre, n, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InnerOuter.lookupInnerVar failed on component: ")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(pre.clone())?); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInstInner)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn updateInstHierarchy(mut inIH: InstHierarchy, mut inPrefix: DAE::Prefix, mut inInnerOuter: Absyn::InnerOuter, mut inInstInner: InstInner) -> Result<InstHierarchy> {
    let mut outIH: InstHierarchy = metamodelica::nil();
    outIH = (::match_deref::match_deref! { match &((inIH.clone(), inPrefix.clone(), inInnerOuter.clone(), inInstInner.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, InstInner { .. }) => {
            let mut tih: TopInstance;
            let mut ih: InstHierarchy = metamodelica::nil();
            let mut ht: InstHierarchyHashTable;
            let mut sm: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            ht = emptyInstHierarchyHashTable();
            sm = HashSet::emptyHashSet();
            tih = TopInstance { path: None, ht: ht.clone(), outerPrefixes: emptyOuterPrefixes.clone(), sm: sm.clone() };
            ih = updateInstHierarchy(list![tih.clone()], inPrefix.clone(), inInnerOuter.clone(), inInstInner.clone())?;
            ih.clone()
        },
        (Deref @ metamodelica::List::Cons { head: TopInstance { path: pathOpt, ht, outerPrefixes, sm }, tail: restIH }, _, _, InstInner { name, .. }) => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut ht = (*ht).clone();
            cref_ = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil());
            (_, cref) = PrefixUtil::prefixCref(FCore::emptyCache(), FGraph::empty(), emptyInstHierarchy().clone(), inPrefix.clone(), cref_.clone())?;
            ht = add((cref.clone(), inInstInner.clone()), ht.clone())?;
            cons(TopInstance { path: pathOpt.clone(), ht: ht.clone(), outerPrefixes: outerPrefixes.clone(), sm: sm.clone() }, restIH.clone())
        },
        (_, _, _, InstInner { .. }) => {
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outIH)
}

pub fn updateSMHierarchy(mut smState: Arc<DAE::ComponentRef>, mut inIH: InstHierarchy) -> Result<InstHierarchy> {
    let mut outIH: InstHierarchy = metamodelica::nil();
    outIH = (::match_deref::match_deref! { match &((smState.clone(), inIH.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            let mut tih: TopInstance;
            let mut ih: InstHierarchy = metamodelica::nil();
            let mut ht: InstHierarchyHashTable;
            let mut sm: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut sm2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            ht = emptyInstHierarchyHashTable();
            sm = HashSet::emptyHashSet();
            sm2 = BaseHashSet::add(smState.clone(), sm.clone())?;
            tih = TopInstance { path: None, ht: ht.clone(), outerPrefixes: emptyOuterPrefixes.clone(), sm: sm2.clone() };
            ih = list![tih.clone()];
            ih.clone()
        },
        (cref, Deref @ metamodelica::List::Cons { head: TopInstance { path: pathOpt, ht, outerPrefixes, sm }, tail: restIH }) => {
            let mut sm = (*sm).clone();
            sm = BaseHashSet::add(cref.clone(), sm.clone())?;
            cons(TopInstance { path: pathOpt.clone(), ht: ht.clone(), outerPrefixes: outerPrefixes.clone(), sm: sm.clone() }, restIH.clone())
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, _) => {
            let true = (Flags::isSet(Flags::INSTANCE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InnerOuter.updateSMHierarchy failure for: ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outIH)
}

pub fn addClassIfInner(mut inClass: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inScope: FCore::Graph, mut inIH: InstHierarchy) -> Result<InstHierarchy> {
    let mut outIH: InstHierarchy = metamodelica::nil();
    outIH = 'mc: {
        let __mc_input = (inClass.clone(), inPrefix.clone(), inScope.clone(), inIH.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, name, .. }, _, _, _) => {
                    let mut scopeName: ArcStr = arcstr::literal!("");
                    let mut outIH: Arc<metamodelica::List<TopInstance>> = outIH.clone();
                    let true = (AbsynUtil::isInner(io.clone())) else { bail!("pattern mismatch") };
                    scopeName = (FGraph::getGraphNameStr(inScope.clone())?).clone();
                    outIH = updateInstHierarchy(inIH.clone(), inPrefix.clone(), io.clone(), InstInner { innerPrefix: inPrefix.clone(), name: (name.clone()).clone(), io: io.clone(), fullName: (name.clone()).clone(), typePath: Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), scope: (scopeName.clone()).clone(), instResult: None, outers: metamodelica::nil(), innerElement: Some(inClass.clone()) })?;
                    Ok(outIH.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inIH.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIH)
}

pub fn addOuterPrefixToIH(mut inIH: InstHierarchy, mut inOuterComponentRef: Arc<DAE::ComponentRef>, mut inInnerComponentRef: Arc<DAE::ComponentRef>) -> Result<InstHierarchy> {
    let mut outIH: InstHierarchy = metamodelica::nil();
    outIH = 'mc: {
        let __mc_input = (inIH.clone(), inOuterComponentRef.clone(), inInnerComponentRef.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    let mut tih: TopInstance;
                    let mut ih: InstHierarchy = metamodelica::nil();
                    let mut ht: InstHierarchyHashTable;
                    let mut sm: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = emptyInstHierarchyHashTable();
                    sm = HashSet::emptyHashSet();
                    tih = TopInstance { path: None, ht: ht.clone(), outerPrefixes: list![OuterPrefix { outerComponentRef: ComponentReference::crefStripSubs(inOuterComponentRef.clone())?, innerComponentRef: inInnerComponentRef.clone() }], sm: sm.clone() };
                    ih = list![tih.clone()];
                    Ok(ih.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: TopInstance { path: pathOpt, ht, outerPrefixes, sm }, tail: restIH }, _, _) => {
                    let mut outerPrefixes = (*outerPrefixes).clone();
                    outerPrefixes = List::unionElt(OuterPrefix { outerComponentRef: ComponentReference::crefStripSubs(inOuterComponentRef.clone())?, innerComponentRef: inInnerComponentRef.clone() }, outerPrefixes.clone());
                    Ok(cons(TopInstance { path: pathOpt.clone(), ht: ht.clone(), outerPrefixes: outerPrefixes.clone(), sm: sm.clone() }, restIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InnerOuter.addOuterPrefix failed to add: outer cref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inOuterComponentRef.clone())?); __mm_s.push_str(&*literal!(" refers to inner cref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inInnerComponentRef.clone())?); __mm_s.push_str(&*literal!(" to IH")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIH)
}

pub fn prefixOuterCrefWithTheInnerPrefix(mut inIH: InstHierarchy, mut inOuterComponentRef: Arc<DAE::ComponentRef>, mut inPrefix: DAE::Prefix) -> Result<Arc<DAE::ComponentRef>> {
    let mut outInnerComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outInnerComponentRef = (::match_deref::match_deref! { match &((inIH.clone(), inOuterComponentRef.clone(), inPrefix.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            bail!("fail")
        },
        (Deref @ metamodelica::List::Cons { head: TopInstance { path: _, ht: _, outerPrefixes: outerPrefixes @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, sm: _ }, tail: Deref @ metamodelica::List::Nil }, _, _) => {
            let mut outerCrefPrefix: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut fullCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut innerCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut innerCrefPrefix: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (_, fullCref) = PrefixUtil::prefixCref(FCore::emptyCache(), FGraph::empty(), emptyInstHierarchy().clone(), inPrefix.clone(), inOuterComponentRef.clone())?;
            (outerCrefPrefix, innerCrefPrefix) = searchForInnerPrefix(fullCref.clone(), inOuterComponentRef.clone(), outerPrefixes.clone())?;
            innerCref = changeOuterReferenceToInnerReference(fullCref.clone(), outerCrefPrefix.clone(), innerCrefPrefix.clone())?;
            innerCref.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInnerComponentRef)
}

fn changeOuterReferenceToInnerReference(mut inFullCref: Arc<DAE::ComponentRef>, mut inOuterCrefPrefix: Arc<DAE::ComponentRef>, mut inInnerCrefPrefix: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outInnerCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outInnerCref = (::match_deref::match_deref! { match &((inFullCref.clone(), inOuterCrefPrefix.clone(), inInnerCrefPrefix.clone())) {
        (ifull, ocp, icp) => {
            let mut ic: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut eifull: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut eocp: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut eicp: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut epre: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut erest: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut esuffix: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            eifull = ComponentReference::explode(ifull.clone())?;
            eicp = ComponentReference::explode(icp.clone())?;
            (eocp, esuffix) = List::split(eifull.clone(), ComponentReference::identifierCount(ocp.clone()))?;
            (epre, erest) = List::splitEqualPrefix(eocp.clone(), eicp.clone(), Arc::new(ComponentReferenceBasics::crefFirstIdentEqual), metamodelica::nil())?;
            (_, eicp) = List::splitEqualPrefix(eicp.clone(), epre.clone(), Arc::new(ComponentReferenceBasics::crefFirstIdentEqual), metamodelica::nil())?;
            (erest, _) = List::splitEqualPrefix(erest.clone().reverse(), eicp.clone().reverse(), Arc::new(ComponentReferenceBasics::crefFirstIdentEqual), metamodelica::nil())?;
            erest = List::append_reverse(erest.clone(), esuffix.clone());
            eifull = listAppend(epre.clone(), erest.clone());
            ic = ComponentReference::implode(eifull.clone())?;
            ic.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInnerCref)
}

fn searchForInnerPrefix(mut fullCref: Arc<DAE::ComponentRef>, mut inOuterCref: Arc<DAE::ComponentRef>, mut outerPrefixes: OuterPrefixes) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)> {
    let mut outerCrefPrefix: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut innerCrefPrefix: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut id: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut b1: bool = false;
    let mut b2: bool = false;
    for mut op in &*outerPrefixes.clone() {
        let mut op = op.clone();
        let OuterPrefix { outerComponentRef: __pa0, .. } = (op.clone()) else { bail!("pattern mismatch") };
        outerCrefPrefix = __pa0.clone();
        b1 = ComponentReferenceBasics::crefPrefixOfIgnoreSubscripts(outerCrefPrefix.clone(), fullCref.clone());
        if !(b1.clone()) {
            cr = ComponentReference::crefStripLastIdent(outerCrefPrefix.clone())?;
            b2 = ComponentReferenceBasics::crefLastIdent(outerCrefPrefix.clone())? == ComponentReferenceBasics::crefFirstIdent(inOuterCref.clone())? && ComponentReferenceBasics::crefPrefixOfIgnoreSubscripts(cr.clone(), fullCref.clone());
        }
        if b1.clone() || b2.clone() {
            let OuterPrefix { innerComponentRef: __pa1, .. } = (op.clone()) else { bail!("pattern mismatch") };
            innerCrefPrefix = __pa1.clone();
            return Ok((outerCrefPrefix, innerCrefPrefix));
        }
    }
    bail!("fail");
    Ok((outerCrefPrefix, innerCrefPrefix))
}

fn printInnerDefStr(mut inInstInner: InstInner) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ((match inInstInner.clone() {
        InstInner { innerPrefix: _, name: _, io: _, fullName: mut fullName, typePath: mut typePath, scope: mut scope, instResult: _, outers: mut outers, innerElement: _ } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut strOuters: ArcStr = arcstr::literal!("");
            let mut outers = outers.clone();
            outers = List::uniqueOnTrue(outers.clone(), Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare));
            strOuters = (if (outers.clone().is_empty()) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" Referenced by 'outer' components: {")); __mm_s.push_str(&*stringDelimitList(List::map(outers.clone(), Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }}).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(typePath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*fullName.clone()); __mm_s.push_str(&*literal!("; defined in scope: ")); __mm_s.push_str(&*scope.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*strOuters.clone()); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outStr)
}

pub fn getExistingInnerDeclarations(mut inIH: InstHierarchy, mut inEnv: FCore::Graph) -> Result<ArcStr> {
    let mut innerDeclarations: ArcStr = arcstr::literal!("");
    innerDeclarations = ((::match_deref::match_deref! { match &((inIH.clone(), inEnv.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("There are no 'inner' components defined in the model in any of the parent scopes of 'outer' component's scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }
        },
        (Deref @ metamodelica::List::Cons { head: TopInstance { path: _, ht, outerPrefixes: _, sm: _ }, tail: _ }, _) => {
            let mut inners: Arc<metamodelica::List<InstInner>> = metamodelica::nil();
            let mut r#str: ArcStr = arcstr::literal!("");
            inners = getInnersFromInstHierarchyHashTable(ht.clone())?;
            r#str = stringDelimitList(List::map(inners.clone(), Arc::new(printInnerDefStr)), (literal!("\n    ")).clone());
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(innerDeclarations)
}

fn getInnersFromInstHierarchyHashTable(mut t: InstHierarchyHashTable) -> Result<Arc<metamodelica::List<InstInner>>> {
    let mut inners: Arc<metamodelica::List<InstInner>> = metamodelica::nil();
    inners = List::map(hashTableList(t.clone())?, Arc::new(fnptr!(getValue, (Arc<DAE::ComponentRef>, InstInner))));
    Ok(inners)
}

fn getValue(mut tpl: (Arc<DAE::ComponentRef>, InstInner)) -> InstInner {
    let mut v: InstInner;
    v = (::match_deref::match_deref! { match &(tpl.clone()) {
        (_, v) => v.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    v
}

// ///////////////////////////////////////////////////////////////
// hash table implementation for InnerOuter instance hierarchy //
// ///////////////////////////////////////////////////////////////
fn hashFunc(mut k: Key) -> Result<i32> {
    let mut res: i32 = 0;
    res = stringHashDjb2((ComponentReferenceBasics::printComponentRefStr(k.clone())?).clone());
    Ok(res)
}

fn keyEqual(mut key1: Key, mut key2: Key) -> Result<bool> {
    let mut res: bool = false;
    res = ComponentReferenceBasics::crefEqualNoStringCompare(key1.clone(), key2.clone())?;
    Ok(res)
}

fn dumpInstHierarchyHashTable(mut t: InstHierarchyHashTable) -> Result<()> {
    println!("{}", (literal!("InstHierarchyHashTable:\n")).clone());
    println!("{}", stringDelimitList(List::map(hashTableList(t.clone())?, Arc::new(dumpTuple)), (literal!("\n")).clone()));
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn dumpTuple(mut tpl: (Arc<DAE::ComponentRef>, InstInner)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(tpl.clone()) {
        (k, _) => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*ComponentReference::crefStr(k.clone())?); __mm_s.push_str(&*literal!(" opaque InstInner for now, implement printing. ")); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

/* end of InstHierarchyHashTable instance specific code */
/* Generic hashtable code below!! */
#[derive(Clone, Debug, PartialEq)]
pub struct InstHierarchyHashTable {
    /// hashtable to translate Key to array indx
    pub hashTable: metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>,
    /// Array of values
    pub valueArr: ValueArray,
    /// bucket size
    pub bucketSize: i32,
    /// number of entries in hashtable
    pub numberOfEntries: i32,
}

pub type HASHTABLE = InstHierarchyHashTable;


/// array of values are expandable, to amortize the
/// cost of adding elements in a more efficient manner
#[derive(Clone, Debug, PartialEq)]
pub struct ValueArray {
    /// number of elements in hashtable
    pub numberOfElements: i32,
    /// array of values
    pub valueArray: metamodelica::Array<Option<(Arc<DAE::ComponentRef>, InstInner)>>,
}

pub type VALUE_ARRAY = ValueArray;


fn emptyInstHierarchyHashTable() -> InstHierarchyHashTable {
    let mut hashTable: InstHierarchyHashTable;
    let mut arr: metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>;
    let mut lst: Arc<metamodelica::List<Option<(Arc<DAE::ComponentRef>, InstInner)>>> = metamodelica::nil();
    let mut emptyarr: metamodelica::Array<Option<(Arc<DAE::ComponentRef>, InstInner)>>;
    arr = arrayCreate(1000, metamodelica::nil());
    emptyarr = arrayCreate(100, None);
    hashTable = InstHierarchyHashTable { hashTable: arr.clone(), valueArr: ValueArray { numberOfElements: 0, valueArray: emptyarr.clone() }, bucketSize: 1000, numberOfEntries: 0 };
    hashTable
}

fn add(mut entry: (Arc<DAE::ComponentRef>, InstInner), mut hashTable: InstHierarchyHashTable) -> Result<InstHierarchyHashTable> {
    let mut outHashTable: InstHierarchyHashTable;
    outHashTable = 'mc: {
        let __mc_input = (entry.clone(), hashTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ (key, _), InstHierarchyHashTable { hashTable: hashvec, valueArr: varr, bucketSize: bsize, numberOfEntries: _ }) => {
                    let mut hval: i32 = 0;
                    let mut indx: i32 = 0;
                    let mut newpos: i32 = 0;
                    let mut n_1: i32 = 0;
                    let mut varr_1: ValueArray;
                    let mut indexes: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
                    let mut hashvec_1: metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>;
                    if '__try0: {
                        let _ = unwrap_break_err!(get(key.clone(), hashTable.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    hval = hashFunc(key.clone())?;
                    indx = intMod(hval.clone(), bsize.clone());
                    newpos = valueArrayLength(varr.clone())?;
                    varr_1 = valueArrayAdd(varr.clone(), v.clone())?;
                    indexes = hashvec.borrow()[(indx.clone() + 1-1) as usize].clone();
                    hashvec_1 = {let _arr = hashvec.clone(); _arr.borrow_mut()[(indx.clone() + 1-1) as usize] = cons((key.clone(), newpos.clone()), indexes.clone()); _arr};
                    n_1 = valueArrayLength(varr_1.clone())?;
                    Ok(InstHierarchyHashTable { hashTable: hashvec_1.clone(), valueArr: varr_1.clone(), bucketSize: bsize.clone(), numberOfEntries: n_1.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (newv @ (key, _), InstHierarchyHashTable { hashTable: hashvec, valueArr: varr, bucketSize: bsize, numberOfEntries: n }) => {
                    let mut indx: i32 = 0;
                    let mut varr_1: ValueArray;
                    (_, indx) = get1(key.clone(), hashTable.clone())?;
                    varr_1 = valueArraySetnth(varr.clone(), indx.clone(), newv.clone())?;
                    Ok(InstHierarchyHashTable { hashTable: hashvec.clone(), valueArr: varr_1.clone(), bucketSize: bsize.clone(), numberOfEntries: n.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("- InnerOuter.add failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHashTable)
}

pub fn get(mut key: Key, mut hashTable: InstHierarchyHashTable) -> Result<Value> {
    let mut value: Value;
    (value, _) = get1(key.clone(), hashTable.clone())?;
    Ok(value)
}

fn get1(mut key: Key, mut hashTable: InstHierarchyHashTable) -> Result<(Value, i32)> {
    let mut value: Value;
    let mut indx: i32 = 0;
    (value, indx) = (::match_deref::match_deref! { match &((key.clone(), hashTable.clone())) {
        (_, InstHierarchyHashTable { hashTable: hashvec, valueArr: varr, bucketSize: bsize, numberOfEntries: _ }) => {
            let mut hval: i32 = 0;
            let mut hashindx: i32 = 0;
            let mut indexes: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>> = metamodelica::nil();
            let mut v: Value;
            let mut k: Key = Arc::new(DAE::ComponentRef::WILD);
            hval = hashFunc(key.clone())?;
            hashindx = intMod(hval.clone(), bsize.clone());
            indexes = hashvec.borrow()[(hashindx.clone() + 1-1) as usize].clone();
            indx = get2(key.clone(), indexes.clone())?;
            (k, v) = valueArrayNth(varr.clone(), indx.clone())?;
            let true = (keyEqual(k.clone(), key.clone())?) else { bail!("pattern mismatch") };
            (v.clone(), indx.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((value, indx))
}

fn get2(mut key: Key, mut keyIndices: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>) -> Result<i32> {
    let mut index: i32 = 0;
    index = 'mc: {
        let __mc_input = (key.clone(), keyIndices.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: (key2, index), tail: _ }) => {
                    let true = (keyEqual(key.clone(), key2.clone())?) else { bail!("pattern mismatch") };
                    Ok(index.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: xs }) => {
                    let mut index: i32 = index.clone();
                    index = get2(key.clone(), xs.clone())?;
                    Ok(index.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(index)
}

fn hashTableList(mut hashTable: InstHierarchyHashTable) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>>> {
    let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>> = metamodelica::nil();
    tplLst = (match hashTable.clone() {
        InstHierarchyHashTable { valueArr: mut varr, .. } => {
            tplLst = valueArrayList(varr.clone())?;
            tplLst.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(tplLst)
}

fn valueArrayList(mut valueArray: ValueArray) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>>> {
    let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>> = metamodelica::nil();
    tplLst = 'mc: {
        let __mc_input = valueArray.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray { numberOfElements: 0, .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::nil())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray { valueArray: mut arr, numberOfElements: 1 } = __mc_input.clone() else { bail!("nomatch") };
            let mut elt: (Arc<DAE::ComponentRef>, InstInner);
            let __pa0 = ::match_deref::match_deref! { match &(arr.borrow()[(0 + 1-1) as usize].clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elt = __pa0.clone();
            Ok(list![elt.clone()])
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let ValueArray { valueArray: mut arr, numberOfElements: mut n } = __mc_input.clone() else { bail!("nomatch") };
            let mut lastpos: i32 = 0;
            let mut lst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>> = metamodelica::nil();
            lastpos = n.clone() - 1;
            lst = valueArrayList2(arr.clone(), 0, lastpos.clone())?;
            Ok(lst.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplLst)
}

fn valueArrayList2(mut inVarOptionArray1: metamodelica::Array<Option<(Arc<DAE::ComponentRef>, InstInner)>>, mut inInteger2: i32, mut inInteger3: i32) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>>> {
    let mut outVarLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>> = metamodelica::nil();
    outVarLst = 'mc: {
        let __mc_input = (inVarOptionArray1.clone(), inInteger2.clone(), inInteger3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut arr, mut pos, mut lastpos) = __mc_input.clone() else { bail!("nomatch") };
            if !((pos.clone() == lastpos.clone())) { bail!("guard") }
            let mut v: (Arc<DAE::ComponentRef>, InstInner);
            let __pa0 = ::match_deref::match_deref! { match &(arr.borrow()[(pos.clone() + 1-1) as usize].clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v = __pa0.clone();
            Ok(list![v.clone()])
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut arr, mut pos, mut lastpos) = __mc_input.clone() else { bail!("nomatch") };
            let mut v: (Arc<DAE::ComponentRef>, InstInner);
            let mut pos_1: i32 = 0;
            let mut res: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>> = metamodelica::nil();
            pos_1 = pos.clone() + 1;
            let __pa0 = ::match_deref::match_deref! { match &(arr.borrow()[(pos.clone() + 1-1) as usize].clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v = __pa0.clone();
            res = valueArrayList2(arr.clone(), pos_1.clone(), lastpos.clone())?;
            Ok(cons(v.clone(), res.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut arr, mut pos, mut lastpos) = __mc_input.clone() else { bail!("nomatch") };
            let mut pos_1: i32 = 0;
            let mut res: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, InstInner)>> = metamodelica::nil();
            pos_1 = pos.clone() + 1;
            ::match_deref::match_deref! { match &(arr.borrow()[(pos.clone() + 1-1) as usize].clone()) {
                None => (),
                _ => bail!("pattern mismatch"),
            } };
            res = valueArrayList2(arr.clone(), pos_1.clone(), lastpos.clone())?;
            Ok(res.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

fn valueArrayLength(mut valueArray: ValueArray) -> Result<i32> {
    let mut size: i32 = 0;
    size = (match valueArray.clone() {
        ValueArray { numberOfElements: mut size, .. } => size.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(size)
}

fn valueArrayAdd(mut valueArray: ValueArray, mut entry: (Arc<DAE::ComponentRef>, InstInner)) -> Result<ValueArray> {
    let mut outValueArray: ValueArray;
    outValueArray = 'mc: {
        let __mc_input = (valueArray.clone(), entry.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ValueArray { valueArray: arr, numberOfElements: n }, _) => {
                    if !((n.clone() < (arr.clone().borrow().len() as i32))) { bail!("guard") }
                    let mut n_1: i32 = 0;
                    let mut arr_1: metamodelica::Array<Option<(Arc<DAE::ComponentRef>, InstInner)>>;
                    n_1 = n.clone() + 1;
                    arr_1 = {let _arr = arr.clone(); _arr.borrow_mut()[(n.clone() + 1-1) as usize] = Some(entry.clone()); _arr};
                    Ok(ValueArray { numberOfElements: n_1.clone(), valueArray: arr_1.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ValueArray { valueArray: arr, numberOfElements: n }, _) => {
                    if !((n.clone() < (arr.clone().borrow().len() as i32))) { bail!("guard") }
                    let mut n_1: i32 = 0;
                    let mut size: i32 = 0;
                    let mut expandsize: i32 = 0;
                    let mut expandsize_1: i32 = 0;
                    let mut arr_1: metamodelica::Array<Option<(Arc<DAE::ComponentRef>, InstInner)>>;
                    let mut arr_2: metamodelica::Array<Option<(Arc<DAE::ComponentRef>, InstInner)>>;
                    let mut rsize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rexpandsize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    size = (arr.clone().borrow().len() as i32);
                    rsize = intReal(size.clone());
                    rexpandsize = rsize.clone() * metamodelica::OrderedFloat(0.4_f64);
                    expandsize = ((rexpandsize.clone()).0 as i32);
                    expandsize_1 = intMax(expandsize.clone(), 1);
                    arr_1 = Array::expand(expandsize_1.clone(), arr.clone(), None)?;
                    n_1 = n.clone() + 1;
                    arr_2 = {let _arr = arr_1.clone(); _arr.borrow_mut()[(n.clone() + 1-1) as usize] = Some(entry.clone()); _arr};
                    Ok(ValueArray { numberOfElements: n_1.clone(), valueArray: arr_2.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-InstHierarchyHashTable.valueArrayAdd failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValueArray)
}

fn valueArraySetnth(mut valueArray: ValueArray, mut pos: i32, mut entry: (Arc<DAE::ComponentRef>, InstInner)) -> Result<ValueArray> {
    let mut outValueArray: ValueArray;
    outValueArray = 'mc: {
        let __mc_input = (valueArray.clone(), pos.clone(), entry.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ValueArray { numberOfElements: _, valueArray: arr }, _, _) => {
                    if !((pos.clone() < (arr.clone().borrow().len() as i32))) { bail!("guard") }
                    {let _arr = arr.clone(); _arr.borrow_mut()[(pos.clone() + 1-1) as usize] = Some(entry.clone()); _arr};
                    Ok(valueArray.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-InstHierarchyHashTable.valueArraySetnth failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValueArray)
}

fn valueArrayClearnth(mut valueArray: ValueArray, mut pos: i32) -> Result<ValueArray> {
    let mut outValueArray: ValueArray;
    outValueArray = 'mc: {
        let __mc_input = (valueArray.clone(), pos.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (ValueArray { numberOfElements: _, valueArray: mut arr }, _) = __mc_input.clone() else { bail!("nomatch") };
            if !((pos.clone() < (arr.clone().borrow().len() as i32))) { bail!("guard") }
            {let _arr = arr.clone(); _arr.borrow_mut()[(pos.clone() + 1-1) as usize] = None; _arr};
            Ok(valueArray.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("-InstHierarchyHashTable.valueArrayClearnth failed\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValueArray)
}

fn valueArrayNth(mut valueArray: ValueArray, mut pos: i32) -> Result<(Key, Value)> {
    let mut key: Key = Arc::new(DAE::ComponentRef::WILD);
    let mut value: Value;
    (key, value) = (match (valueArray.clone(), pos.clone()) {
        (ValueArray { valueArray: mut arr, numberOfElements: mut n }, _) => {
            let mut k: Key = Arc::new(DAE::ComponentRef::WILD);
            let mut v: Value;
            let true = (pos.clone() < n.clone()) else { bail!("pattern mismatch") };
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(arr.borrow()[(pos.clone() + 1-1) as usize].clone()) {
                Some((__pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            k = __pa0.clone();
            v = __pa1.clone();
            (k.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((key, value))
}

