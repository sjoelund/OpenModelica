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

use crate::CevalScript;
use crate::RewriteRules;
use crate::StaticScript;
use crate::SymbolTable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::BackendCevalInterface;
use openmodelica_frontend::FCore;
use openmodelica_frontend::InstHashTable;
use openmodelica_frontend_dump::BackendInterface;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;

pub fn initializeBackendInterface() -> () {
    BackendInterface::initializeBackendInterface(BackendInterface::BackendInterfaceFunctions { noRewriteRulesFrontEnd: (std::sync::Arc::new(noRewriteRulesFrontEnd) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>), rewriteFrontEnd: (std::sync::Arc::new(rewriteFrontEnd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<(Arc<Absyn::Exp>, bool)> + 'static>), appendLibrary: (std::sync::Arc::new(appendLibrary) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, ArcStr) -> Result<(Absyn::Program, bool)> + 'static>), initInstHashTable: (std::sync::Arc::new(InstHashTable::init) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<()> + 'static>) });
    BackendCevalInterface::initializeBackendInterface(BackendCevalInterface::BackendInterfaceFunctions { cevalInteractiveFunctions: (std::sync::Arc::new(cevalInteractiveFunctions) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<DAE::Exp>, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>), cevalCallFunction: (std::sync::Arc::new(cevalCallFunction) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Values::Value>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>), elabCallInteractive: (std::sync::Arc::new(elabCallInteractive) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<Absyn::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> + 'static>) });
    ()
}

fn cevalInteractiveFunctions(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inMsg: Absyn::Msg, mut inNumIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = CevalScript::cevalInteractiveFunctions(inCache.clone(), inEnv.clone(), inExp.clone(), inMsg.clone(), inNumIter.clone())?;
    Ok((outCache, outValue))
}

fn cevalCallFunction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inImplInst: bool, mut inMsg: Absyn::Msg, mut inNumIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = CevalScript::cevalCallFunction(inCache.clone(), inEnv.clone(), inExp.clone(), inValues.clone(), inImplInst.clone(), inMsg.clone(), inNumIter.clone())?;
    Ok((outCache, outValue))
}

fn elabCallInteractive(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCref: Arc<Absyn::ComponentRef>, mut inExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplInst: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = StaticScript::elabCallInteractive(inCache.clone(), inEnv.clone(), inCref.clone(), inExps.clone(), inNamedArgs.clone(), inImplInst.clone(), inPrefix.clone(), inInfo.clone())?;
    Ok((outCache, outExp, outProperties))
}

fn noRewriteRulesFrontEnd() -> Result<bool> {
    let mut noRules: bool = false;
    noRules = RewriteRules::noRewriteRulesFrontEnd()?;
    Ok(noRules)
}

fn rewriteFrontEnd(mut inExp: Arc<Absyn::Exp>) -> Result<(Arc<Absyn::Exp>, bool)> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut isChanged: bool = false;
    (outExp, isChanged) = RewriteRules::rewriteFrontEnd(inExp.clone())?;
    Ok((outExp, isChanged))
}

fn appendLibrary(mut modelName: Arc<Absyn::Path>, mut modelicaPath: ArcStr) -> Result<(Absyn::Program, bool)> {
    let mut program: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut success: bool = false;
    program = SymbolTable::getAbsyn();
    (program, success) = CevalScript::loadModel(list![(modelName.clone(), literal!(""), list![(literal!("default")).clone()], false)], (modelicaPath.clone()).clone(), program.clone(), true, true, true, false, false, (literal!("")).clone())?;
    SymbolTable::setAbsyn(program.clone())?;
    Ok((program, success))
}

