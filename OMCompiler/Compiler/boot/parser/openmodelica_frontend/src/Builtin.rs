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

use crate::FBuiltin;
use crate::FCore;
use crate::FGraph;
use crate::FGraphBuildEnv;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Util;

pub fn variableIsBuiltin(mut cref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. } => {
            variableNameIsBuiltin((id.clone()).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn variableNameIsBuiltin(mut name: ArcStr) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "time" => true,
        Deref @ "startTime" => Config::acceptOptimicaGrammar()?,
        Deref @ "finalTime" => Config::acceptOptimicaGrammar()?,
        Deref @ "objective" => Config::acceptOptimicaGrammar()?,
        Deref @ "objectiveIntegrand" => Config::acceptOptimicaGrammar()?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isDer(mut inPath: Arc<Absyn::Path>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "der" } => {
            ()
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path } => {
            isDer(path.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn initialGraph(mut inCache: FCore::Cache) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut graph: FCore::Graph;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let anyNonExpandableConnector2int: Arc<DAE::Type> = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: Arc::new(DAE::Type::T_ANYTYPE { anyClassType: Some(ClassInf::State::CONNECTOR { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$dummy$")).clone() }), isExpandable: false }) }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("cardinality")).clone() }) });
    let anyExpandableConnector2int: Arc<DAE::Type> = Arc::new(DAE::Type::T_FUNCTION { funcArg: list![Arc::new(DAE::FuncArg { name: (literal!("x")).clone(), ty: Arc::new(DAE::Type::T_ANYTYPE { anyClassType: Some(ClassInf::State::CONNECTOR { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$dummy$")).clone() }), isExpandable: true }) }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("cardinality")).clone() }) });
    (outCache, graph) = 'mc: {
        let __mc_input = inCache.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else { bail!("nomatch") };
            let mut graph: FCore::Graph;
            graph = FCore::getCachedInitialGraph(cache.clone())?;
            graph = FGraph::clone(graph.clone())?;
            Ok((cache.clone(), graph.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else { bail!("nomatch") };
            let mut graph: FCore::Graph;
            graph = getSetInitialGraph(None)?;
            Ok((cache.clone(), graph.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else { bail!("nomatch") };
            let mut initialProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut graph: FCore::Graph;
            graph = FGraph::new((literal!("graph")).clone(), FCore::dummyTopModel.clone())?;
            graph = FGraphBuildEnv::mkProgramGraph(FBuiltin::getBasicTypes()?, crate::FCore::Kind::BASIC_TYPE, graph.clone())?;
            graph = FBuiltin::initialGraphModelica(graph.clone(), (std::sync::Arc::new(FGraphBuildEnv::mkTypeNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, metamodelica::Array<FCore::Node>, ArcStr, FCore::Graph) -> Result<FCore::Graph> + 'static>), (std::sync::Arc::new(FGraphBuildEnv::mkCompNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>))?;
            (_, initialProgram) = FBuiltin::getInitialFunctions()?;
            graph = FGraphBuildEnv::mkProgramGraph(initialProgram.clone(), crate::FCore::Kind::BUILTIN, graph.clone())?;
            graph = FBuiltin::initialGraphOptimica(graph.clone(), (std::sync::Arc::new(FGraphBuildEnv::mkCompNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>))?;
            graph = FBuiltin::initialGraphMetaModelica(graph.clone(), (std::sync::Arc::new(FGraphBuildEnv::mkTypeNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, metamodelica::Array<FCore::Node>, ArcStr, FCore::Graph) -> Result<FCore::Graph> + 'static>))?;
            cache = FCore::setCachedInitialGraph(cache.clone(), graph.clone());
            let _ = getSetInitialGraph(Some(graph.clone()))?;
            graph = FGraph::clone(graph.clone())?;
            Ok((cache.clone(), graph.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, graph))
}

fn getSetInitialGraph(mut inEnvOpt: Option<FCore::Graph>) -> Result<FCore::Graph> {
    let mut initialEnv: FCore::Graph;
    initialEnv = 'mc: {
        let __mc_input = inEnvOpt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                let _ = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = metamodelica::nil());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let None = __mc_input.clone() else { bail!("nomatch") };
            let mut assocLst: Arc<metamodelica::List<(i32, FCore::Graph)>> = metamodelica::nil();
            let mut graph: FCore::Graph;
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            graph = FGraph::clone(Util::assoc(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, assocLst.clone())?)?;
            Ok(graph.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let Some(mut graph) = __mc_input.clone() else { bail!("nomatch") };
            let mut assocLst: Arc<metamodelica::List<(i32, FCore::Graph)>> = metamodelica::nil();
            let mut f: i32 = 0;
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            f = Flags::getConfigEnum(Flags::GRAMMAR.clone())?;
            assocLst = if (f.clone() == Flags::METAMODELICA.clone()) {cons((Flags::METAMODELICA.clone(), graph.clone()), assocLst.clone())} else {if (f.clone() == Flags::PARMODELICA.clone()) {cons((Flags::PARMODELICA.clone(), graph.clone()), assocLst.clone())} else {if (f.clone() == Flags::MODELICA.clone()) {cons((Flags::MODELICA.clone(), graph.clone()), assocLst.clone())} else {assocLst.clone()}}};
            crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = assocLst.clone());
            Ok(graph.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(initialEnv)
}

pub fn clearInitialGraph() -> () {
    crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = metamodelica::nil());
    ()
}

